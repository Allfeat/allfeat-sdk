use parity_scale_codec::Decode;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use std::{fmt::Debug, sync::Arc};
use subxt::{
    Config, SubstrateConfig,
    config::HashFor,
    tx::{Payload, Signer, TxStatus},
    utils::AccountId32,
};
use system::AllfeatTxSystem;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{Function, Reflect};

use crate::{
    Client,
    utils::sign::{
        AllfeatSubmittableTransaction, TransactionPayload, extension_signature_for_extrinsic,
    },
};

pub mod system;

#[wasm_bindgen(js_name = "Tx")]
#[derive(Debug, Clone)]
pub struct JsTx(pub(super) Arc<Client>);

#[wasm_bindgen(js_class = "Tx")]
impl JsTx {
    #[wasm_bindgen]
    pub fn system(&self) -> AllfeatTxSystem {
        AllfeatTxSystem(self.0.clone())
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen(js_name = "Signer")]
pub struct JsSigner {
    account: AccountId32,
    sign_payload_fn: Function,
    sign_raw_fn: Function,
}

#[wasm_bindgen(js_class = "Signer")]
impl JsSigner {
    #[wasm_bindgen(getter)]
    pub fn address(&self) -> String {
        self.account.to_string()
    }

    #[wasm_bindgen(getter, js_name = "signPayload")]
    pub fn sign_payload_fn(&self) -> Function {
        self.sign_payload_fn.clone()
    }

    #[wasm_bindgen(getter, js_name = "signRaw")]
    pub fn sign_raw_fn(&self) -> Function {
        self.sign_raw_fn.clone()
    }
}

impl JsSigner {
    pub fn account_ref(&self) -> &AccountId32 {
        &self.account
    }
}

#[wasm_bindgen(js_name = "Call")]
pub struct JsCall {
    client: Arc<Client>,
    call: Box<dyn Payload>,
}

#[wasm_bindgen(js_class = "Call")]
impl JsCall {
    #[wasm_bindgen(js_name = "withSigner")]
    pub fn with_signer(
        self,
        signer: JsValue,
        address: String,
    ) -> Result<JsCallWithSigner, JsError> {
        let account: AccountId32 = address.parse()?;

        let sign_payload = Reflect::get(&signer, &JsValue::from_str("signPayload"))
            .map_err(|e| JsError::new(&format!("Could not get `signPayload`: {e:?}")))?;
        let sign_payload_fn = sign_payload
            .dyn_into::<Function>()
            .map_err(|_| JsError::new("`signPayload` is not a function"))?;

        let sign_raw = Reflect::get(&signer, &JsValue::from_str("signRaw"))
            .map_err(|e| JsError::new(&format!("Could not get `signRaw`: {e:?}")))?;
        let sign_raw_fn = sign_raw
            .dyn_into::<Function>()
            .map_err(|_| JsError::new("`signRaw` is not a function"))?;

        let signer = JsSigner {
            account,
            sign_payload_fn,
            sign_raw_fn,
        };
        Ok(JsCallWithSigner { call: self, signer })
    }

    #[wasm_bindgen(js_name = "getEncodedCallData")]
    pub fn encoded_call_data(&self) -> Vec<u8> {
        self.call.encode_call_data(&self.client.metadata()).unwrap()
    }
}

#[wasm_bindgen(js_name = "CallWithSigner")]
pub struct JsCallWithSigner {
    call: JsCall,
    signer: JsSigner,
}

#[wasm_bindgen(js_class = "CallWithSigner")]
impl JsCallWithSigner {
    #[wasm_bindgen(getter)]
    pub fn signer(&self) -> JsSigner {
        self.signer.clone()
    }

    #[wasm_bindgen(js_name = "getEncodedCallData")]
    pub fn encoded_call_data(&self) -> Vec<u8> {
        self.call.encoded_call_data()
    }

    #[wasm_bindgen]
    pub async fn sign(&self) -> Result<JsSubmittableTransaction, JsError> {
        let signed = extension_signature_for_extrinsic(
            &self.signer,
            &self.call.call,
            self.call.client.clone(),
        )
        .await?;

        Ok(JsSubmittableTransaction(signed))
    }
}

#[wasm_bindgen(js_name = "SubmittableTransaction")]
pub struct JsSubmittableTransaction(AllfeatSubmittableTransaction);

#[wasm_bindgen(js_class = "SubmittableTransaction")]
impl JsSubmittableTransaction {
    #[wasm_bindgen(js_name = "submitAndWatch")]
    pub async fn submit_and_watch(&self, callback: Function) -> Result<(), JsError> {
        let mut tx_progress = self.0.submit_and_watch().await?;

        while let Some(status) = tx_progress.next().await {
            let status: JsTxStatus = status?.into();
            callback
                .call1(&JsValue::NULL, &serde_wasm_bindgen::to_value(&status)?)
                .map_err(|e| JsError::new(&format!("Callback failed: {e:?}")))?;
        }

        Ok(())
    }

    #[wasm_bindgen]
    pub fn encoded(&self) -> Vec<u8> {
        self.0.encoded().to_vec()
    }
}

/// A subxt TxStatus which is compatible with a JS wasm_bindgen environment.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum JsTxStatus {
    Validated,
    Broadcasted,
    NoLongerInBestBlock,
    InBestBlock { block_hash: String, tx_hash: String },
    InFinalizedBlock { block_hash: String, tx_hash: String },
    Error { message: String },
    Invalid { message: String },
    Dropped { message: String },
}

impl<T, C> From<TxStatus<T, C>> for JsTxStatus
where
    T: Config,
    HashFor<T>: std::fmt::LowerHex,
{
    fn from(status: TxStatus<T, C>) -> Self {
        match status {
            TxStatus::Validated => JsTxStatus::Validated,
            TxStatus::Broadcasted => JsTxStatus::Broadcasted,
            TxStatus::NoLongerInBestBlock => JsTxStatus::NoLongerInBestBlock,

            TxStatus::InBestBlock(in_block) => JsTxStatus::InBestBlock {
                block_hash: format!("0x{:x}", in_block.block_hash()),
                tx_hash: format!("0x{:x}", in_block.extrinsic_hash()),
            },

            TxStatus::InFinalizedBlock(in_block) => JsTxStatus::InFinalizedBlock {
                block_hash: format!("0x{:x}", in_block.block_hash()),
                tx_hash: format!("0x{:x}", in_block.extrinsic_hash()),
            },

            TxStatus::Error { message } => JsTxStatus::Error { message },
            TxStatus::Invalid { message } => JsTxStatus::Invalid { message },
            TxStatus::Dropped { message } => JsTxStatus::Dropped { message },
        }
    }
}
