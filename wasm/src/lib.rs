use allfeat_sdk_core::metadata::melodie;
use std::{str::FromStr, sync::Arc};
use subxt::{OnlineClient, SubstrateConfig, utils::AccountId32};
use tx::JsTx;
use wasm_bindgen::prelude::*;
use web_sys::console;

pub mod metrics;
pub mod tx;
pub mod utils;

pub type Client = OnlineClient<SubstrateConfig>;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();

    console::log_1(&"🧬 Allfeat WASM SDK loaded!".into());
}

#[wasm_bindgen]
pub struct AllfeatClient {
    inner: Arc<Client>,
}

#[wasm_bindgen]
impl AllfeatClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<AllfeatClient, JsError> {
        Err(JsError::new("Use createClient() instead of constructor."))
    }

    #[wasm_bindgen]
    pub fn tx(&self) -> JsTx {
        JsTx(self.inner.clone())
    }

    #[wasm_bindgen(js_name = "createClient")]
    pub async fn create_client() -> Result<AllfeatClient, JsError> {
        let client = OnlineClient::<SubstrateConfig>::new()
            .await
            .map_err(|e| JsError::new(&format!("Failed to create client: {e}")))?;

        Ok(AllfeatClient {
            inner: Arc::new(client),
        })
    }

    #[wasm_bindgen(js_name = "getBalanceOf")]
    pub async fn get_balance_of(&self, address: String) -> Result<Option<u128>, JsError> {
        let account_id = AccountId32::from_str(&address)?;

        let query = melodie::storage().system().account(&account_id);

        let res = self
            .inner
            .storage()
            .at_latest()
            .await?
            .fetch(&query)
            .await?;

        match res {
            Some(x) => Ok(Some(x.data.free)),
            None => Ok(None),
        }
    }
}
