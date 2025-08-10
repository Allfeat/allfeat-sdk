# MIDDS V2 - Musical Industry Decentralized Data Standard

[![Rust](https://github.com/allfeat/allfeat-sdk/workflows/Rust/badge.svg)](https://github.com/allfeat/allfeat-sdk)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

A comprehensive Rust crate providing standardized data structures for musical metadata, designed to work seamlessly in both native Rust applications and Substrate blockchain runtime environments.

## Overview

MIDDS V2 defines three core entities in the music industry:

- 🎵 **Musical Works** - Compositions, songs, and musical creations
- 🎤 **Tracks** - Specific recordings or performances of musical works  
- 💿 **Releases** - Albums, EPs, singles, and commercial releases

## Key Features

### 🔄 Dual Compilation Modes
- **Native Mode**: Uses standard Rust types (`String`, `Vec<T>`) for applications
- **Runtime Mode**: Uses Substrate-compatible types (`BoundedVec`) for blockchain runtime

### 🚀 Automatic Type Transformation
The `runtime_midds` procedural macro automatically transforms types:
```rust
// Native mode
pub title: String,
pub genres: Vec<GenreId>,

// Runtime mode (automatically transformed)  
pub title: BoundedVec<u8, ConstU32<256>>,
pub genres: BoundedVec<GenreId, ConstU32<5>>,
```

### 📊 Benchmarking Support
Comprehensive benchmarking utilities for Substrate pallet performance testing with linear scaling.

### 🛡️ Type Safety
- Compile-time bounds checking in runtime mode
- Mutually exclusive feature flags prevent configuration errors
- Comprehensive test coverage

## Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
# For native applications
allfeat-midds-v2 = { version = "0.1.0", features = ["native"] }

# For Substrate runtime
allfeat-midds-v2 = { version = "0.1.0", features = ["runtime"] }

# For runtime with benchmarking
allfeat-midds-v2 = { version = "0.1.0", features = ["runtime", "runtime-benchmarks"] }
```

### Basic Usage

#### Native Mode
```rust
use allfeat_midds_v2::{
    musical_work::{MusicalWork, Iswc, Participant, ParticipantRole},
    utils::{Language, Key},
    MiddsId,
};

let work = MusicalWork {
    iswc: Iswc("T-034524680-1".to_string()),
    title: "Bohemian Rhapsody".to_string(),
    creation_year: Some(1975),
    instrumental: Some(false),
    language: Some(Language::English),
    bpm: Some(72),
    key: Some(Key::Bb),
    work_type: None,
    participants: vec![
        Participant {
            id: 12345,
            role: ParticipantRole::Composer,
        }
    ],
    classical_info: None,
};
```

#### Runtime Mode
```rust
use allfeat_midds_v2::{
    musical_work::{MusicalWork, Iswc, Participant, ParticipantRole},
    utils::{Language, Key},
    MiddsId,
};
use frame_support::{BoundedVec, traits::ConstU32};

let work = MusicalWork {
    iswc: Iswc(
        BoundedVec::try_from("T-034524680-1".as_bytes().to_vec()).unwrap()
    ),
    title: BoundedVec::try_from("Bohemian Rhapsody".as_bytes().to_vec()).unwrap(),
    creation_year: Some(1975),
    instrumental: Some(false),
    language: Some(Language::English),
    bpm: Some(72),
    key: Some(Key::Bb),
    work_type: None,
    participants: BoundedVec::try_from(vec![
        Participant {
            id: 12345,
            role: ParticipantRole::Composer,
        }
    ]).unwrap(),
    classical_info: None,
};
```

#### Benchmarking
```rust
#[cfg(feature = "runtime-benchmarks")]
use allfeat_midds_v2::benchmarking::BenchmarkHelper;

// Generate instances with linear scaling for benchmarking
let small_work = MusicalWork::benchmark_instance(10);   // Small data
let large_work = MusicalWork::benchmark_instance(1000); // Large data
```

## Architecture

### Core Types

| Type | Description | Key Features |
|------|-------------|--------------|
| `MusicalWork` | Musical compositions and songs | ISWC identification, participant tracking, classical music support |
| `Track` | Specific recordings/performances | ISRC identification, technical metadata, contributor tracking |
| `Release` | Commercial releases (albums, etc.) | EAN/UPC codes, distribution metadata, format specifications |

### Utility Types

- **Date** - Simple date representation without timezone complexity
- **Language** - Comprehensive language enumeration for internationalization  
- **Country** - ISO 3166-1 alpha-2 country codes
- **Key** - Musical key notation (major/minor, sharps/flats, enharmonic equivalents)

## The `runtime_midds` Macro

The core of MIDDS V2's dual-mode functionality:

### Syntax
```rust
#[runtime_midds]
pub struct MyStruct {
    #[runtime_bound(256)]
    pub title: String,
    
    #[runtime_bound(64)] 
    pub tags: Vec<String>,
    
    #[runtime_bound(32)]
    pub optional_data: Option<Vec<u32>>,
    
    pub id: u64, // No transformation needed
}
```

### Supported Transformations
- `String` → `BoundedVec<u8, ConstU32<N>>`
- `Vec<T>` → `BoundedVec<T, ConstU32<N>>`  
- `Option<String>` → `Option<BoundedVec<u8, ConstU32<N>>>`
- `Option<Vec<T>>` → `Option<BoundedVec<T, ConstU32<N>>>`
- Recursive transformation for nested `Option` types

### Generated Traits
- **Runtime mode**: `Encode`, `Decode`, `DecodeWithMemTracking`, `TypeInfo`, `MaxEncodedLen`, `Debug`, `Clone`, `PartialEq`, `Eq`
- **Native mode**: `Debug`, `Clone`, `PartialEq`, `Eq`

## Feature Flags

| Feature | Description | Conflicts |
|---------|-------------|-----------|
| `std` | Standard library support | None |
| `native` | Native Rust types (default) | `runtime` |
| `runtime` | Substrate runtime types | `native` |
| `runtime-benchmarks` | Benchmarking utilities | Requires `runtime` |

## Type Bounds Reference

### Identifiers
- **ISWC**: 11 characters (`#[runtime_bound(11)]`)
- **ISRC**: 12 characters (`#[runtime_bound(12)]`)  
- **EAN/UPC**: 13 characters (`#[runtime_bound(13)]`)

### Text Fields
- **Titles/Names**: 256 characters (`#[runtime_bound(256)]`)
- **Optional Text**: 256 characters (`#[runtime_bound(256)]`)

### Collections
- **Small Lists**: 5-64 items depending on type
- **Medium Lists**: 256 items (participants, contributors)
- **Large Lists**: 512-1024 items (medley references, tracks)

## Examples

### Creating a Complete Musical Work
```rust
use allfeat_midds_v2::{
    musical_work::*,
    utils::{Language, Key},
    MiddsId,
};

let classical_work = MusicalWork {
    iswc: Iswc("T-123456789-0".to_string()),
    title: "Symphony No. 9 in D minor".to_string(),
    creation_year: Some(1824),
    instrumental: Some(true),
    language: None,
    bpm: Some(72),
    key: Some(Key::Dm),
    work_type: Some(MusicalWorkType::Original),
    participants: vec![
        Participant {
            id: 1,
            role: ParticipantRole::Composer,
        }
    ],
    classical_info: Some(ClassicalInfo {
        opus: Some("Op. 125".to_string()),
        catalog_number: Some("D 125".to_string()),
        number_of_voices: Some(4),
    }),
};
```

### Creating a Track
```rust
use allfeat_midds_v2::{
    track::*,
    utils::Key,
    MiddsId,
};
use allfeat_music_genres::GenreId;

let track = Track {
    isrc: Isrc("USUM71703861".to_string()),
    musical_work: 12345,
    artist: 67890,
    producers: vec![11111, 22222],
    performers: vec![67890, 33333, 44444],
    contributors: vec![55555, 66666],
    title: TrackTitle("Bohemian Rhapsody (Remastered 2011)".to_string()),
    title_aliases: vec![
        TrackTitle("Bohemian Rhapsody".to_string())
    ],
    recording_year: Some(1975),
    genres: vec![GenreId::Rock, GenreId::Progressive],
    version: Some(TrackVersion::Remastered),
    duration: Some(355), // 5:55 in seconds
    bpm: Some(72),
    key: Some(Key::Bb),
    recording_place: Some("Rockfield Studios, Wales".to_string()),
    mixing_place: Some("Wessex Studios, London".to_string()),
    mastering_place: None,
};
```

## Best Practices

### 1. Bounds Selection
Choose appropriate bounds based on real-world usage:
```rust
#[runtime_bound(64)]   // Artist/band names (typically < 50 chars)
#[runtime_bound(256)]  // Song titles (rarely > 100 chars)  
#[runtime_bound(1024)] // Track lists (albums rarely > 50 tracks)
```

### 2. Optional vs Required Fields
Use `Option<T>` for truly optional metadata:
```rust
pub creation_year: Option<u16>,     // Not always known
pub bpm: Option<u16>,               // Not always measured
pub recording_place: Option<String>, // Not always documented
```

### 3. Feature Flag Management
Never enable conflicting features:
```toml
# ❌ Wrong - conflicting features
features = ["runtime", "native"]

# ✅ Correct - single mode
features = ["runtime", "runtime-benchmarks"]
```

## Testing

Run tests for different feature combinations:

```bash
# Native mode tests
cargo test --features "native"

# Runtime mode tests  
cargo test --no-default-features --features "runtime"

# Benchmarking tests
cargo test --no-default-features --features "runtime,runtime-benchmarks"
```

## Contributing

1. Ensure all new types use the `#[runtime_midds]` macro
2. Add appropriate `#[runtime_bound(N)]` attributes for sized fields
3. Include comprehensive documentation for all public types
4. Add both native and runtime test cases
5. Follow existing naming conventions and code style

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Related Projects

- [Allfeat SDK](https://github.com/allfeat/allfeat-sdk) - Complete Allfeat development toolkit
- [Substrate](https://substrate.io/) - Blockchain development framework
- [MIDDS V1](../midds) - Previous generation MIDDS implementation