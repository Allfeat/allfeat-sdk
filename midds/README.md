# 🎼 Allfeat MIDDS

Music Industry Decentralized Data Structures (MIDDS) for the Allfeat blockchain ecosystem.

## Overview

MIDDS provides standardized, type-safe data structures for music industry entities with full Substrate runtime compatibility.

## Core Entities

| Entity | Description | Standard ID |
|--------|-------------|-------------|
| **Musical Works** | Compositions and songs | ISWC |
| **Party Identifiers** | Artists, labels, publishers | IPI/ISNI |
| **Releases** | Albums, EPs, compilations | EAN/UPC |
| **Tracks** | Individual recordings | ISRC |

## Quick Start

```rust
use allfeat_midds::{
    party_identifier::Ipi,
    musical_work::Iswc,
    shared::conversion::Validatable,
};
use std::str::FromStr;

// Create identifiers
let ipi: Ipi = 123456789;
let iswc = Iswc::from_str("T1234567890").unwrap();

// Validate (std feature required)
#[cfg(feature = "std")]
{
    assert!(iswc.validate().is_ok());
}
```

## Features

- 🔒 **Type Safety**: Strong typing with validation
- ⚡ **Performance**: Optimized for on-chain storage
- 🌐 **Dual Types**: SDK and Runtime type separation
- 📏 **Standards Compliant**: Music industry standards
- 🧪 **Benchmarking**: Built-in benchmark helpers

## Architecture

```
midds/
├── src/
│   ├── musical_work/     # Musical work data structures
│   ├── party_identifier/ # Artist/entity identification
│   ├── release/          # Music release structures
│   ├── track/            # Track/recording structures
│   └── shared/           # Common utilities
└── midds-types-codegen/  # Procedural macros
```

## Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `std` | SDK types and validation | ✅ |
| `js` | WebAssembly bindings | ❌ |
| `runtime-benchmarks` | Benchmark utilities | ❌ |
| `try-runtime` | Try-runtime support | ❌ |

## Benchmarking

```rust
use allfeat_midds::{benchmarking::BenchmarkHelperT, track::Track, Midds};

// Generate test data with varying complexity
let track = <Track as Midds>::BenchmarkHelper::variable_size(0.5);
let min_track = <Track as Midds>::BenchmarkHelper::min_size();
let max_track = <Track as Midds>::BenchmarkHelper::max_size();
```

## Dependencies

- [frame-support](https://docs.rs/frame-support) - Substrate runtime framework
- [parity-scale-codec](https://docs.rs/parity-scale-codec) - SCALE codec
- [sp-runtime](https://docs.rs/sp-runtime) - Substrate runtime primitives