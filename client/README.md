# 🔌 Allfeat Client

A Rust crate that extends [subxt](https://github.com/paritytech/subxt) with additional capabilities specifically designed for the Allfeat ecosystem, providing enhanced functionality for interacting with Allfeat blockchain networks and exporting metadata for ecosystem networks.

## Features

- 🌐 **Subxt Extension**: Enhanced capabilities built on top of the subxt client library
- 📊 **Metrics Collection**: Comprehensive network statistics for Allfeat ecosystem
- 🔧 **Transaction Building**: Streamlined transaction creation and submission
- 🏗️ **Ecosystem Integration**: Pre-configured metadata for Allfeat network chains

## Quick Start

```rust
use allfeat_client::{AllfeatOnlineClient, AllfeatMetrics};
use subxt::OnlineClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OnlineClient::from_url("wss://melodie-rpc.allfeat.io").await?;

    let active_wallets = client.get_active_wallets_count().await?;
    let total_midds = client.get_all_midds_created_count().await?;

    println!("Active wallets: {}, Total MIDDS: {}", active_wallets, total_midds);
    Ok(())
}
```

## Available Metrics

| Method                          | Description                                       |
| ------------------------------- | ------------------------------------------------- |
| `get_active_wallets_count()`    | Active wallets with balance > existential deposit |
| `get_works_created_count()`     | Total musical works registered                    |
| `get_tracks_created_count()`    | Total tracks registered                           |
| `get_releases_created_count()`  | Total releases registered                         |
| `get_all_midds_created_count()` | Sum of all MIDDS types                            |

## Dependencies

- [subxt](https://github.com/paritytech/subxt) - Substrate client library
- [async-trait](https://crates.io/crates/async-trait) - Async traits support

