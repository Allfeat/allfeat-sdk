# 🎵 Allfeat SDK

A comprehensive Rust SDK for interacting with the Allfeat blockchain ecosystem, featuring Music Industry Decentralized Data Structures (MIDDS) and blockchain metrics.

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)

## 🌟 Overview

The Allfeat SDK is a powerful toolkit for building applications on the Allfeat blockchain, specifically designed for the music industry. It provides type-safe interfaces for managing music industry data structures and interacting with the blockchain network.

### Key Components

- **🎼 MIDDS (Music Industry Decentralized Data Structures)**: Standardized data structures for music industry entities
- **🔌 Client SDK**: High-level client for blockchain interactions
- **🌐 WebAssembly Bindings**: JavaScript/TypeScript compatibility via WASM
- **📊 Metrics Collection**: Comprehensive blockchain analytics
- **🛠️ Type Generation**: Procedural macros for bounded types

## 🏗️ Architecture

This workspace consists of several interconnected crates:

```
allfeat-sdk/
├── client/           # Blockchain client and metrics
├── midds/            # Music Industry Data Structures
│   ├── src/          # Core MIDDS implementation
│   └── midds-types-codegen/  # Procedural macros
└── examples/         # Usage examples and demos
```

### Crate Overview

| Crate                 | Description                    | Features                                   |
| --------------------- | ------------------------------ | ------------------------------------------ |
| `allfeat-client`      | Blockchain client with metrics | Subxt integration, metrics collection      |
| `allfeat-midds`       | Music industry data structures | Type-safe MIDDS, validation, benchmarking  |
| `midds-types-codegen` | Code generation macros         | Bounded strings/collections, WASM bindings |

## 🎯 MIDDS: Music Industry Data Structures

MIDDS provides standardized, blockchain-compatible representations of music industry entities:

### Core Entities

| Entity            | Description               | Standard Identifier                             |
| ----------------- | ------------------------- | ----------------------------------------------- |
| **Musical Works** | Compositions and songs    | ISWC (International Standard Musical Work Code) |
| **Releases**      | Albums, EPs, compilations | EAN/UPC (European/Universal Product Code)       |
| **Tracks**        | Individual recordings     | ISRC (International Standard Recording Code)    |

### Key Features

- 🔒 **Type Safety**: Strong typing with comprehensive validation
- ⚡ **Performance**: Optimized for on-chain storage and operations
- 🌐 **Dual Types**: Separate SDK and Runtime types for optimal UX and efficiency
- 📏 **Standards Compliant**: Implements music industry standard identifiers
- 🧪 **Benchmarking**: Built-in benchmarking for Substrate pallets

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ with `wasm32-unknown-unknown` target
- Node.js 18+ (for JavaScript examples)

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
allfeat-client = { path = "client" }
allfeat-midds = { path = "midds" }
```

### Basic Usage

#### Creating Music Industry Identifiers

```rust
use allfeat_midds::{
    musical_work::Iswc,
    shared::conversion::Validatable,
};
use std::str::FromStr;

// Create music industry identifiers
let ipi: Ipi = 123456789;
let iswc = Iswc::from_str("T1234567890").unwrap();

// Validate identifiers (std feature required)
#[cfg(feature = "std")]
{
    assert!(iswc.validate().is_ok());
}
```

#### Blockchain Metrics

```rust
use allfeat_client::{AllfeatOnlineClient, AllfeatMetrics};
use subxt::OnlineClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OnlineClient::from_url("wss://melodie-rpc.allfeat.io").await?;

    // Get blockchain metrics
    let active_wallets = client.get_active_wallets_count().await?;
    let total_midds = client.get_all_midds_created_count().await?;

    println!("Active wallets: {}", active_wallets);
    println!("Total MIDDS created: {}", total_midds);

    Ok(())
}
```

#### Using the Type Generation Macros

```rust
use midds_types_codegen::{midds_string, midds_collection};

// Generate a bounded string type with 256-byte limit
#[midds_string(256)]
pub struct TrackTitle;

// Generate a bounded collection type for 64 u64 values
#[midds_collection(u64, 64)]
pub struct ProducerIds;

// Usage
let mut title = TrackTitle::from_str("My Song").unwrap();
title.push_str(" - Extended Mix").unwrap();

let mut producers = ProducerIds::new();
producers.push(12345).unwrap();
```

## 🌐 WebAssembly Support

The SDK provides comprehensive WebAssembly bindings for JavaScript/TypeScript applications:

### Features

- 🔧 **Auto-generated bindings**: Type-safe interfaces
- 📦 **NPM packages**: Published packages for easy integration
- 🎛️ **Transaction building**: Create and sign transactions from JavaScript
- 📊 **Metrics access**: Query blockchain statistics from web apps

### JavaScript Usage

```javascript
import { AllfeatClient, TrackTitle } from "@allfeat/client";

// Create bounded string types
const title = TrackTitle.fromString("My Awesome Track");
console.log(title.value); // "MyAwesomeTrack" (normalized)

// Connect to blockchain
const client = new AllfeatClient("wss://rpc.allfeat.network");
const metrics = await client.getAllMiddsCreatedCount();
```

## 📊 Metrics Collection

The SDK provides comprehensive metrics for monitoring the Allfeat network:

### Available Metrics

| Metric                          | Description                                          |
| ------------------------------- | ---------------------------------------------------- |
| `get_active_wallets_count()`    | Number of wallets with balance > existential deposit |
| `get_works_created_count()`     | Total musical works registered                       |
| `get_tracks_created_count()`    | Total tracks registered                              |
| `get_releases_created_count()`  | Total releases registered                            |
| `get_all_midds_created_count()` | Aggregate of all MIDDS types                         |

## 🏷️ Feature Flags

Configure the SDK with feature flags:

| Feature              | Description                     | Default |
| -------------------- | ------------------------------- | ------- |
| `std`                | Enable SDK types and validation | ✅      |
| `js`                 | Enable WebAssembly bindings     | ❌      |
| `runtime-benchmarks` | Enable benchmarking utilities   | ❌      |
| `try-runtime`        | Enable try-runtime features     | ❌      |

Example `Cargo.toml`:

```toml
[dependencies]
allfeat-midds = { path = "midds", features = ["js"] }
```

## 🧪 Benchmarking

When the `runtime-benchmarks` feature is enabled, MIDDS types provide benchmarking helpers:

```rust
use allfeat_midds::{
    benchmarking::BenchmarkHelperT,
    track::Track,
    Midds,
};

// Generate MIDDS with specific complexity (0.0 = minimal, 1.0 = maximal)
let track = <Track as Midds>::BenchmarkHelper::variable_size(0.5);

// Or use predefined sizes
let min_track = <Track as Midds>::BenchmarkHelper::min_size();
let max_track = <Track as Midds>::BenchmarkHelper::max_size();
```

## 🛠️ Development

### Building the Project

```bash
# Build all crates
cargo build

# Build with WebAssembly support
cargo build --features js

# Run tests
cargo test

# Build WebAssembly packages
cargo build --target wasm32-unknown-unknown --release
```

### Running Examples

```bash
# Rust examples
cd examples/client && cargo run

# JavaScript examples
cd examples/client && npm install && npm run dev

# Nuxt.js app example
cd examples/remark-app && npm install && npm run dev
```

## 📖 Examples

The repository includes comprehensive examples:

- **Rust Client Examples**: Basic blockchain interactions and metrics
- **JavaScript Examples**: WebAssembly integration and transaction building
- **Nuxt.js Demo App**: Full-featured web application showcasing the SDK

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Setup

1. Clone the repository
2. Install Rust with `wasm32-unknown-unknown` target
3. Install Node.js for JavaScript examples
4. Run `cargo test` to verify setup

### Code Style

- Follow Rust conventions and run `cargo fmt`
- Document all public APIs with `///` comments
- Add examples to complex functionality
- Ensure WASM compatibility for public types

## 📄 License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- **Website**: [https://allfeat.org](https://allfeat.org)
- **Documentation**: [https://docs.allfeat.org](https://docs.allfeat.org)
- **Repository**: [https://github.com/Allfeat/allfeat-sdk](https://github.com/Allfeat/allfeat-sdk)
- **Discord**: [Join our community](https://discord.gg/allfeat)

## 🙏 Acknowledgments

- Built with [Subxt](https://github.com/paritytech/subxt) for Substrate blockchain interactions
- Powered by [Substrate](https://substrate.io/) blockchain framework
- Music industry standards compliance via CISAC and other organizations

---

**Made with ❤️ by the Allfeat Team**
