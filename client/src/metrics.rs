//! Metrics Collection Module
//!
//! This module provides functionality for collecting and analyzing metrics
//! from the Allfeat blockchain network. It includes traits and implementations
//! for fetching various statistics about the blockchain state.
//!
//! # Features
//!
//! - Active wallet counting based on existential deposit
//! - MIDDS creation statistics (tracks, releases, parties, musical works)
//! - Aggregated metrics for comprehensive network analysis
//!
//! # Example
//!
//! ```rust,ignore
//! use allfeat_client::{AllfeatOnlineClient, AllfeatMetrics};
//!
//! async fn get_stats(client: &AllfeatOnlineClient) -> Result<(), Box<dyn std::error::Error>> {
//!     let active_wallets = client.get_active_wallets_count().await?;
//!     let total_midds = client.get_all_midds_created_count().await?;
//!
//!     println!("Active wallets: {}, Total MIDDS: {}", active_wallets, total_midds);
//!     Ok(())
//! }
//! ```

use crate::AllfeatOnlineClient;

use super::metadata::melodie;
use async_trait::async_trait;
use subxt::{storage::DefaultAddress, utils::Yes};

/// A trait that defines methods for a client to fetch statistics data about the Allfeat chains.
///
/// This trait provides access to various blockchain metrics including wallet activity
/// and MIDDS (Music Industry Decentralized Data Structures) creation statistics.
/// All methods are async and return results that can be used for analytics and monitoring.
#[async_trait]
pub trait AllfeatMetrics {
    type Error;

    /// Returns the count of active wallets on the Allfeat blockchain.
    ///
    /// An active wallet is defined as an account with a balance greater than
    /// the existential deposit. This metric provides insight into the number
    /// of accounts actively participating in the network.
    ///
    /// # Returns
    ///
    /// * `Ok(u64)` - The number of active wallets
    /// * `Err(Self::Error)` - If the query fails
    async fn get_active_wallets_count(&self) -> Result<u64, Self::Error>;

    /// Returns the total number of musical works created on the blockchain.
    ///
    /// Musical works represent compositions and songs. This metric shows the
    /// volume of creative content being registered on the platform.
    ///
    /// # Returns
    ///
    /// * `Ok(u64)` - The number of musical works created
    /// * `Err(Self::Error)` - If the query fails
    async fn get_works_created_count(&self) -> Result<u64, Self::Error>;

    /// Returns the total number of tracks created on the blockchain.
    ///
    /// Tracks represent individual recordings of musical works. This count
    /// indicates the volume of recorded music being registered.
    ///
    /// # Returns
    ///
    /// * `Ok(u64)` - The number of tracks created
    /// * `Err(Self::Error)` - If the query fails
    async fn get_tracks_created_count(&self) -> Result<u64, Self::Error>;

    /// Returns the total number of releases created on the blockchain.
    ///
    /// Releases represent albums, EPs, singles and other music collections.
    /// This metric shows the publishing activity on the platform.
    ///
    /// # Returns
    ///
    /// * `Ok(u64)` - The number of releases created
    /// * `Err(Self::Error)` - If the query fails
    async fn get_releases_created_count(&self) -> Result<u64, Self::Error>;

    /// Returns the aggregate count of all MIDDS created on the blockchain.
    ///
    /// This is the sum of all musical works, tracks, and
    /// releases. It provides a comprehensive view of the total content
    /// registered on the Allfeat platform.
    ///
    /// # Returns
    ///
    /// * `Ok(u64)` - The total number of all MIDDS created
    /// * `Err(Self::Error)` - If any of the underlying queries fail
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
            + self.get_works_created_count().await?)
    }
}

/// Helper function to fetch the next ID from storage, indicating the total count of items.
///
/// This function queries the blockchain storage for a "next_id" value, which typically
/// represents the number of items that have been created (since IDs start from 0 or 1).
///
/// # Arguments
///
/// * `client` - The Allfeat client to use for the query
/// * `query_fn` - A closure that returns the storage query address
///
/// # Returns
///
/// * `Ok(u64)` - The next ID value, representing the count of created items
/// * `Err(subxt::Error)` - If the storage query fails
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
