use crate::AllfeatOnlineClient;

use super::metadata::melodie;
use async_trait::async_trait;
use subxt::{storage::DefaultAddress, utils::Yes};

#[async_trait]
/// A trait that defines method for a client to fetch statistics data about the Allfeat chains.
pub trait AllfeatMetrics {
    type Error;

    async fn get_active_wallets_count(&self) -> Result<u64, Self::Error>;
    async fn get_party_created_count(&self) -> Result<u64, Self::Error>;
    async fn get_works_created_count(&self) -> Result<u64, Self::Error>;
    async fn get_tracks_created_count(&self) -> Result<u64, Self::Error>;
    async fn get_releases_created_count(&self) -> Result<u64, Self::Error>;
    async fn get_all_midds_created_count(&self) -> Result<u64, Self::Error>;
}

#[async_trait]
impl AllfeatMetrics for AllfeatOnlineClient {
    type Error = subxt::Error;

    async fn get_active_wallets_count(&self) -> Result<u64, Self::Error> {
        let ed_query = melodie::constants().balances().existential_deposit();
        let ed = self.constants().at(&ed_query)?;

        let account_query = melodie::storage().system().account_iter();
        let mut all_accounts = self
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

    async fn get_party_created_count(&self) -> Result<u64, Self::Error> {
        get_next_id(self, || melodie::storage().party_identifiers().next_id()).await
    }

    async fn get_works_created_count(&self) -> Result<u64, Self::Error> {
        get_next_id(self, || melodie::storage().musical_works().next_id()).await
    }

    async fn get_tracks_created_count(&self) -> Result<u64, Self::Error> {
        get_next_id(self, || melodie::storage().tracks().next_id()).await
    }

    async fn get_releases_created_count(&self) -> Result<u64, Self::Error> {
        get_next_id(self, || melodie::storage().releases().next_id()).await
    }

    async fn get_all_midds_created_count(&self) -> Result<u64, Self::Error> {
        Ok(self.get_tracks_created_count().await?
            + self.get_releases_created_count().await?
            + self.get_party_created_count().await?
            + self.get_works_created_count().await?)
    }
}

async fn get_next_id<F>(client: &AllfeatOnlineClient, query_fn: F) -> Result<u64, subxt::Error>
where
    F: FnOnce() -> DefaultAddress<(), u64, Yes, Yes, ()> + Send,
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
