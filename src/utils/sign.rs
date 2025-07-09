use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use subxt::{
    SubstrateConfig,
    config::DefaultExtrinsicParamsBuilder,
    ext::codec::{Compact, Decode, Encode},
    tx::{Payload, SubmittableTransaction},
    utils::{AccountId32, MultiSignature, to_hex},
};
use wasm_bindgen::{JsError, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

use crate::{Client, tx::JsSigner};

pub type AllfeatSubmittableTransaction = SubmittableTransaction<SubstrateConfig, Client>;

#[derive(Serialize, Debug, Decode)]
#[serde(rename_all = "camelCase")]
/// The payload JSON object which is required for extensions which use a signPayload function.
pub struct TransactionPayload {
    pub spec_version: String,
    pub transaction_version: String,
    pub address: String,
    pub block_hash: String,
    pub block_number: String,
    pub era: String,
    pub genesis_hash: String,
    pub method: String,
    pub nonce: String,
    pub signed_extensions: Vec<String>,
    pub tip: String,
    pub version: u32,
}

/// The JS object response which is returned from a signPayload function of an extension.
#[derive(Deserialize, Debug)]
struct SignatureResponse {
    signature: String,
}

/// communicates with JavaScript to obtain a signature for the `partial_extrinsic` via a browser extension (e.g. polkadot-js or Talisman)
///
/// Some parameters are hard-coded here and not taken from the partial_extrinsic itself (mortality_checkpoint, era, tip).
pub async fn extension_signature_for_extrinsic<Call: Payload>(
    signer: &JsSigner,
    call: &Call,
    api: Arc<Client>,
) -> Result<AllfeatSubmittableTransaction, JsError> {
    let payload = get_payload_to_sign(
        api.clone(),
        &call.encode_call_data(&api.metadata()).unwrap(),
        signer.account_ref(),
        Some(30),
    )
    .await?;

    let js_result = signer
        .sign_payload_fn()
        .call3(
            &JsValue::NULL,
            &to_value(&payload)?,
            &JsValue::from("".to_string()), // source
            &JsValue::from(signer.address()),
        )
        .map_err(|e| JsError::new(&format!("Error when calling signer function: {e:?}")))?;
    let js_promise = Promise::from(js_result);
    let result = JsFuture::from(js_promise)
        .await
        .map_err(|e| JsError::new(&format!("Error from promise: {e:?}")))?;

    let sig: SignatureResponse = serde_wasm_bindgen::from_value(result)
        .map_err(|e| JsError::new(&format!("Failed to parse signature object: {e}")))?;
    let signature = hex::decode(&sig.signature[2..])?;
    let multi_signature = MultiSignature::decode(&mut &signature[..])?;

    let params = DefaultExtrinsicParamsBuilder::new().mortal(30).build();
    let mut partial_signed = api
        .tx()
        .create_partial(call, signer.account_ref(), params)
        .await?;

    // Apply the signature
    let signed_extrinsic =
        partial_signed.sign_with_account_and_signature(signer.account_ref(), &multi_signature);

    let dry_res = signed_extrinsic.validate().await;
    web_sys::console::log_1(&format!("Validation Result: {dry_res:?}").into());

    Ok(signed_extrinsic)
}

async fn get_payload_to_sign(
    api: Arc<Client>,
    call_data: &[u8],
    account: &AccountId32,
    mortal: Option<u64>,
) -> Result<TransactionPayload, JsError> {
    let genesis_hash = encode_then_hex(&api.genesis_hash());
    // These numbers aren't SCALE encoded; their bytes are just converted to hex:
    let spec_version = to_hex(api.runtime_version().spec_version.to_be_bytes());
    let transaction_version = to_hex(api.runtime_version().transaction_version.to_be_bytes());
    let nonce = to_hex(api.tx().account_nonce(account).await?.to_be_bytes());
    // If you construct a mortal transaction, then this block hash needs to correspond
    // to the block number passed to `Era::mortal()`.
    let (era, mortality_checkpoint, block_number) = match mortal {
        Some(period) => {
            let current = api.blocks().at_latest().await?;
            let era = encode_then_hex(&subxt::utils::Era::mortal(period, current.number() as u64));
            (
                era,
                encode_then_hex(&current.hash()),
                current.number().to_string(),
            )
        }
        None => (
            encode_then_hex(&subxt::utils::Era::Immortal),
            encode_then_hex(&api.genesis_hash()),
            "0x00000000".to_string(),
        ),
    };

    let method = to_hex(call_data);
    let signed_extensions: Vec<String> = api
        .metadata()
        .extrinsic()
        .transaction_extensions_by_version(0)
        .unwrap()
        .map(|e| e.identifier().to_string())
        .collect();
    let tip = encode_then_hex(&Compact(0u128));

    let payload = TransactionPayload {
        spec_version,
        transaction_version,
        address: account.to_string(),
        block_hash: mortality_checkpoint,
        block_number,
        era,
        genesis_hash,
        method,
        nonce,
        signed_extensions,
        tip,
        version: 4,
    };

    Ok(payload)
}

fn encode_then_hex<E: Encode>(input: &E) -> String {
    format!("0x{}", hex::encode(input.encode()))
}
