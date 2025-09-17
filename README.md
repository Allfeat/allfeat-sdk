# 🎵 Allfeat SDK

A comprehensive Rust SDK for interacting with the Allfeat blockchain ecosystem, featuring Music Industry Decentralized Data Structures (MIDDS), blockchain metrics, and zero-knowledge proof circuits.

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)

## 🌟 Overview

The Allfeat SDK is a powerful toolkit for building applications on the Allfeat blockchain, specifically designed for the music industry. It provides type-safe interfaces for managing music industry data structures, interacting with the blockchain network, and generating/verifying zero-knowledge proofs.

### Key Components

- **🎼 MIDDS (Music Industry Decentralized Data Structures)**: Standardized data structures for music industry entities
- **🔌 Client SDK**: High-level client for blockchain interactions
- **🌐 WebAssembly Bindings**: JavaScript/TypeScript compatibility via WASM
- **📊 Metrics Collection**: Comprehensive blockchain analytics
- **🛠️ Type Generation**: Procedural macros for bounded types
- **🧾 ATS/ZKP**: Allfeat Time Stamp Song Commitment Circuit (zkSNARKs in Arkworks)

## 🏗️ Architecture

This workspace consists of several interconnected crates:

```
allfeat-sdk/
├── ats/zkp/          # Allfeat Time Stamp Song Commitment Circuit (zkSNARKs)
├── client/           # Blockchain client and metrics
├── midds-v2/         # Music Industry Data Structures v2
│   ├── src/          # Core MIDDS implementation
│   └── midds-v2-codegen/  # Code generation utilities
└── packages/         # Additional packages and utilities
```

### Crate Overview

| Crate                 | Description                        | Features                                   |
| --------------------- | ---------------------------------- | ------------------------------------------ |
| `allfeat-client`      | Blockchain client with metrics     | Subxt integration, metrics collection      |
| `allfeat-midds-v2`    | Music industry data structures     | Substrate-compatible MIDDS, benchmarking   |
| `midds-v2-codegen`    | Code generation utilities          | Music genre enums, TypeScript bindings     |
| `allfeat-ats-zkp`     | Time Stamp Song Commitment Circuit | BN254, Groth16, Poseidon, Substrate-ready  |

## 🧾 ATS/ZKP: Time Stamp Song Commitment Circuit

The `ats/zkp` crate implements the **Allfeat Time Stamp Song Commitment Circuit** using the **Arkworks** Rust ecosystem for zkSNARK programming.

### Features

- 🔒 **Poseidon-based commitments** with a secret + song metadata
- 🕒 **Timestamp + nullifier** to prevent replay attacks
- ⚡ **Groth16 on BN254** for efficient proof generation/verification
- 🔗 **Substrate + SDK integration** via hex/bytes serialization APIs
- 🧪 **Comprehensive tests**, including negative cases (tampered inputs, malformed proofs)

### Public API

- `setup`: generate proving & verifying keys
- `prove`: generate a proof for witness + public inputs
- `verify`: verify a proof against public inputs
- `verify_from_bytes`: verify from raw bytes (Substrate-friendly)
- `verify_from_hex`: verify from hex strings (SDK-friendly)
- `serialize_vk_to_hex` / `serialize_proof_to_hex`: serialization helpers

### Example Usage

```rust
use allfeat_ats_zkp::{
    setup, prove, verify, prepare_vk, Witness, PublicInputs,
    serialize_vk_to_hex, serialize_proof_to_hex, verify_from_hex,
};
use rand::thread_rng;

// Example witness and inputs
let witness = Witness { secret: /* Fr value */ };
let publics = PublicInputs {
    hash_audio: /* Fr */,
    hash_title: /* Fr */,
    hash_creators: /* Fr */,
    commitment: /* Fr */,
    timestamp: /* Fr */,
    nullifier: /* Fr */,
};

// Setup
let mut rng = thread_rng();
let (pk, vk) = setup(&mut rng, (witness, publics)).unwrap();

// Prove
let (proof, public_inputs) = prove(&pk, witness, publics, &mut rng).unwrap();

// Verify
let pvk = prepare_vk(&vk);
assert!(verify(&pvk, &proof, &public_inputs).unwrap());

// Hex-based verify (for SDK/Blockchain)
let vk_hex = serialize_vk_to_hex(&vk);
let proof_hex = serialize_proof_to_hex(&proof);
let publics_hex: Vec<String> = public_inputs.iter().map(|fr| format!("{:?}", fr)).collect();
let publics_refs: Vec<&str> = publics_hex.iter().map(|s| s.as_str()).collect();
assert!(verify_from_hex(&vk_hex, &proof_hex, &publics_refs).unwrap());
```

## 🎯 MIDDS: Music Industry Data Structures

MIDDS provides standardized, blockchain-compatible representations of music industry entities:

### Core Entities

| Entity            | Description               | Standard Identifier                             |
| ----------------- | ------------------------- | ----------------------------------------------- |
| **Musical Works** | Compositions and songs    | ISWC (International Standard Musical Work Code) |
| **Releases**      | Albums, EPs, compilations | EAN/UPC (European/Universal Product Code)       |
| **Recordings**    | Individual recordings     | ISRC (International Standard Recording Code)    |

### Key Features

- 🔒 **Type Safety**: Strong typing with comprehensive validation
- ⚡ **Performance**: Optimized for on-chain storage and operations
- 🔗 **Substrate Compatible**: All types implement traits required for blockchain storage
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
allfeat-midds-v2 = { path = "midds-v2" }
allfeat-ats-zkp = { path = "ats/zkp" }
```

### Basic Usage

#### Creating Music Industry Identifiers

```rust
use allfeat_midds_v2::{
    musical_work::Iswc,
    shared::{PartyId, Ipi},
};

// Create music industry identifiers
let ipi: Ipi = 123456789;
let party_id = PartyId::Ipi(ipi);
let iswc: Iswc = b"T1234567890".to_vec().try_into().unwrap();
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
use allfeat_midds_v2::{
    MiddsString, MiddsVec, MiddsId,
    recording::Recording,
    shared::{PartyId, Key},
};

// Create bounded string and vector types
let title: MiddsString<256> = b"My Song - Extended Mix".to_vec().try_into().unwrap();
let producer_ids: MiddsVec<MiddsId, 64> = vec![12345].try_into().unwrap();

// Use in MIDDS structures
let recording = Recording {
    isrc: b"USUM71703861".to_vec().try_into().unwrap(),
    musical_work: 1,
    performer: PartyId::Ipi(67890),
    producers: producer_ids,
    // ... other fields
};
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
import { AllfeatClient } from "@allfeat/client";

// Connect to blockchain
const client = new AllfeatClient("wss://melodie-rpc.allfeat.io");
const metrics = await client.getAllMiddsCreatedCount();
console.log("Total MIDDS created:", metrics);
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
| `std`                | Enable standard library support | ✅      |
| `runtime-benchmarks` | Enable benchmarking utilities   | ❌      |

Example `Cargo.toml`:

```toml
[dependencies]
allfeat-midds-v2 = { path = "midds-v2", features = ["std"] }
```

## 🧪 Benchmarking

When the `runtime-benchmarks` feature is enabled, MIDDS types provide benchmarking helpers:

```rust
#[cfg(feature = "runtime-benchmarks")]
use allfeat_midds_v2::{
    benchmarking::BenchmarkHelper,
    recording::Recording,
};

// Generate MIDDS instances for benchmarking
#[cfg(feature = "runtime-benchmarks")]
{
    let recording = Recording::benchmark_instance(100);
}
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
