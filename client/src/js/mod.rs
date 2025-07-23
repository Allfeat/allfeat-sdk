//! JS WASM wrappers for the native clients functionnalities.

use crate::AllfeatOnlineClient;

use super::metadata::melodie;
use std::{str::FromStr, sync::Arc};
use subxt::{OnlineClient, SubstrateConfig, utils::AccountId32};
use wasm_bindgen::prelude::*;
use web_sys::console;

pub mod metrics;
pub mod tx;
pub mod utils;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();

    console::log_1(&"🧬 Allfeat Client loaded!".into());
}

#[wasm_bindgen(js_name = "AllfeatClient")]
pub struct JsAllfeatClient {
    inner: Arc<AllfeatOnlineClient>,
}

#[wasm_bindgen(js_class = "AllfeatClient")]
impl JsAllfeatClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<JsAllfeatClient, JsError> {
        Err(JsError::new("Use createClient() instead of constructor."))
    }

    // #[wasm_bindgen]
    // pub fn tx(&self) -> JsTx {
    // JsTx(self.inner.clone())
    //}

    #[wasm_bindgen(js_name = "createClient")]
    pub async fn create_client() -> Result<JsAllfeatClient, JsError> {
        let client = OnlineClient::<SubstrateConfig>::new()
            .await
            .map_err(|e| JsError::new(&format!("Failed to create client: {e}")))?;

        Ok(JsAllfeatClient {
            inner: Arc::new(client),
        })
    }

    #[wasm_bindgen(js_name = "getBalanceOf")]
    pub async fn get_balance_of(&self, address: String) -> Result<Option<u128>, JsError> {
        let account_id = AccountId32::from_str(&address)?;

        let query = melodie::storage().system().account(account_id);

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
