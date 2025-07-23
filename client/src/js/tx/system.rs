use std::sync::Arc;

use crate::{AllfeatOnlineClient, metadata::melodie};
use wasm_bindgen::prelude::wasm_bindgen;

use super::JsCall;

#[wasm_bindgen]
pub struct AllfeatTxSystem(pub(super) Arc<AllfeatOnlineClient>);

#[wasm_bindgen]
impl AllfeatTxSystem {
    #[wasm_bindgen]
    pub fn remark(&self, remark: String) -> JsCall {
        JsCall {
            client: self.0.clone(),
            call: Box::new(melodie::tx().system().remark(remark.into())),
        }
    }
}
