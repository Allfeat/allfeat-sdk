use metrics::AllfeatMetrics;
use subxt::{OnlineClient, SubstrateConfig};

pub mod metadata;
pub mod metrics;

#[cfg(feature = "js")]
pub mod js;

/// Allfeat leverage the default Substrate Config types.
pub type AllfeatOnlineClient = OnlineClient<SubstrateConfig>;

/// Trait extension which extends functionnalities of a client capable to connect to a
/// Polkadot/Substrate blockchain.
pub trait AllfeatExt: AllfeatMetrics {}
