use super::JsAllfeatClient;
use wasm_bindgen::prelude::*;

use crate::metrics::AllfeatMetrics;

#[wasm_bindgen(js_class = "AllfeatClient")]
impl JsAllfeatClient {
    #[wasm_bindgen(js_name = "getActiveWalletsCount")]
    pub async fn get_active_wallets_count(&self) -> Result<u64, JsError> {
        self.inner
            .get_active_wallets_count()
            .await
            .map_err(JsError::from)
    }

    #[wasm_bindgen(js_name = "getPartyCreatedCount")]
    pub async fn get_party_created_count(&self) -> Result<u64, JsError> {
        self.inner
            .get_party_created_count()
            .await
            .map_err(JsError::from)
    }

    #[wasm_bindgen(js_name = "getWorksCreatedCount")]
    pub async fn get_works_created_count(&self) -> Result<u64, JsError> {
        self.inner
            .get_works_created_count()
            .await
            .map_err(JsError::from)
    }

    #[wasm_bindgen(js_name = "getTracksCreatedCount")]
    pub async fn get_tracks_created_count(&self) -> Result<u64, JsError> {
        self.inner
            .get_tracks_created_count()
            .await
            .map_err(JsError::from)
    }

    #[wasm_bindgen(js_name = "getReleasesCreatedCount")]
    pub async fn get_releases_created_count(&self) -> Result<u64, JsError> {
        self.inner
            .get_releases_created_count()
            .await
            .map_err(JsError::from)
    }

    #[wasm_bindgen(js_name = "getAllMiddsCreatedCount")]
    pub async fn get_all_midds_created_count(&self) -> Result<u64, JsError> {
        self.inner
            .get_all_midds_created_count()
            .await
            .map_err(JsError::from)
    }
}
