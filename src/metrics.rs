use crate::metadata::melodie;

use super::Client;

use super::AllfeatClient;
use subxt::storage::DefaultAddress;
use subxt::utils::Yes;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl AllfeatClient {
    #[wasm_bindgen(js_name = "getActiveWalletsCount")]
    pub async fn get_active_wallets_count(&self) -> Result<u64, JsError> {
        get_active_wallets_count(&self.inner).await
    }

    #[wasm_bindgen(js_name = "getPartyCreatedCount")]
    pub async fn get_party_created_count(&self) -> Result<u64, JsError> {
        get_party_created_count(&self.inner).await
    }
    #[wasm_bindgen(js_name = "getWorksCreatedCount")]
    pub async fn get_works_created_count(&self) -> Result<u64, JsError> {
        get_works_created_count(&self.inner).await
    }
    #[wasm_bindgen(js_name = "getTracksCreatedCount")]
    pub async fn get_tracks_created_count(&self) -> Result<u64, JsError> {
        get_tracks_created_count(&self.inner).await
    }
    #[wasm_bindgen(js_name = "getReleasesCreatedCount")]
    pub async fn get_releases_created_count(&self) -> Result<u64, JsError> {
        get_releases_created_count(&self.inner).await
    }

    #[wasm_bindgen(js_name = "getAllMiddsCreatedCount")]
    pub async fn get_all_midds_created_count(&self) -> Result<u64, JsError> {
        get_all_midds_created_count(&self.inner).await
    }
}

async fn get_active_wallets_count(client: &Client) -> Result<u64, JsError> {
    let ed_query = melodie::constants().balances().existential_deposit();
    let ed = client.constants().at(&ed_query)?;

    let account_query = melodie::storage().system().account_iter();
    let mut all_accounts = client
        .storage()
        .at_latest()
        .await?
        .iter(account_query)
        .await?;

    let mut count: u64 = 0;

    while let Some(res) = all_accounts.next().await {
        let kv = res?;
        if kv.value.data.free > ed {
            count += 1;
        }
    }

    Ok(count)
}

async fn get_next_id<F>(client: &Client, query_fn: F) -> Result<u64, JsError>
where
    F: FnOnce() -> DefaultAddress<(), u64, Yes, Yes, ()>,
{
    let value = client
        .storage()
        .at_latest()
        .await?
        .fetch(&query_fn())
        .await?
        .unwrap_or_default();
    Ok(value)
}

async fn get_party_created_count(client: &Client) -> Result<u64, JsError> {
    get_next_id(client, || melodie::storage().party_identifiers().next_id()).await
}

async fn get_works_created_count(client: &Client) -> Result<u64, JsError> {
    get_next_id(client, || melodie::storage().musical_works().next_id()).await
}

async fn get_tracks_created_count(client: &Client) -> Result<u64, JsError> {
    get_next_id(client, || melodie::storage().tracks().next_id()).await
}

async fn get_releases_created_count(client: &Client) -> Result<u64, JsError> {
    get_next_id(client, || melodie::storage().releases().next_id()).await
}

async fn get_all_midds_created_count(client: &Client) -> Result<u64, JsError> {
    Ok(get_tracks_created_count(client).await?
        + get_releases_created_count(client).await?
        + get_party_created_count(client).await?
        + get_works_created_count(client).await?)
}
