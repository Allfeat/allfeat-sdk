# 🔌 Allfeat Client

A high-level client library for interacting with the Allfeat blockchain, featuring metrics collection and WebAssembly bindings.

## Features

- 🌐 **Subxt Integration**: Type-safe blockchain interactions
- 📊 **Metrics Collection**: Comprehensive network statistics
- 🔧 **Transaction Building**: Create and submit transactions
- 🌍 **WASM Support**: JavaScript/TypeScript bindings

## Quick Start

### Rust Usage

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

### JavaScript Usage

```javascript
import { AllfeatClient } from "@allfeat/client";

const client = new AllfeatClient("wss://melodie-rpc.allfeat.io");
const metrics = await client.getAllMiddsCreatedCount();
console.log("Total MIDDS:", metrics);
```

## Available Metrics

| Method                          | Description                                       |
| ------------------------------- | ------------------------------------------------- |
| `get_active_wallets_count()`    | Active wallets with balance > existential deposit |
| `get_works_created_count()`     | Total musical works registered                    |
| `get_tracks_created_count()`    | Total tracks registered                           |
| `get_releases_created_count()`  | Total releases registered                         |
| `get_all_midds_created_count()` | Sum of all MIDDS types                            |

## Features

- `js` - Enable WebAssembly bindings for JavaScript

## Dependencies

- [subxt](https://github.com/paritytech/subxt) - Substrate client library
- [async-trait](https://crates.io/crates/async-trait) - Async traits support

