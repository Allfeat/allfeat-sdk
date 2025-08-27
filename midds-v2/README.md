# MIDDS - Musical Industry Decentralized Data Standard

[![Rust](https://github.com/allfeat/allfeat-sdk/workflows/Rust/badge.svg)](https://github.com/allfeat/allfeat-sdk)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

A comprehensive Rust crate providing standardized data structures for musical metadata, designed specifically for Substrate blockchain runtime environments with optional standard library support.

## Overview

MIDDS V2 defines three core entities in the music industry:

- 🎵 **Musical Works** - Compositions, songs, and musical creations
- 🎤 **Recordings** - Specific recordings or performances of musical works
- 💿 **Releases** - Albums, EPs, singles, and commercial releases

## Key Features

### 🏗️ Substrate-Compatible Architecture

- **BoundedVec Types**: Uses `BoundedVec<T, ConstU32<N>>` for efficient on-chain storage
- **No Runtime Validation**: Designed for application-level validation, not runtime checks
- **Industry Standards**: Built around ISWC, ISRC, EAN/UPC identifiers

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
# For std applications (default)
allfeat-midds-v2 = "0.2.0"

# For no-std environments (Substrate runtime)
allfeat-midds-v2 = { version = "0.2.0", default-features = false }

# For runtime with benchmarking
allfeat-midds-v2 = { version = "0.2.0", features = ["runtime-benchmarks"] }
```

### Basic Usage

#### Basic Usage

```rust
use allfeat_midds_v2::{
    musical_work::{MusicalWork, Creator, CreatorRole},
    shared::{PartyId, Language, Key},
};

let work = MusicalWork {
    iswc: b"T1234567890".to_vec().try_into().unwrap(),
    title: b"Bohemian Rhapsody".to_vec().try_into().unwrap(),
    creation_year: Some(1975),
    instrumental: Some(false),
    language: Some(Language::English),
    bpm: Some(72),
    key: Some(Key::Bb),
    work_type: None,
    creators: vec![
        Creator {
            id: PartyId::Ipi(123456789),
            role: CreatorRole::Composer,
        }
    ].try_into().unwrap(),
    classical_info: None,
};
```


#### Benchmarking

```rust
#[cfg(feature = "runtime-benchmarks")]
use allfeat_midds_v2::{
    benchmarking::BenchmarkHelper,
    musical_work::MusicalWork,
};

// Generate instances for benchmarking
let work = MusicalWork::benchmark_instance(100);
```

## Architecture

### Core Types

| Type          | Description                        | Key Features                                                   |
| ------------- | ---------------------------------- | -------------------------------------------------------------- |
| `MusicalWork` | Musical compositions and songs     | ISWC identification, creator tracking, classical music support |
| `Recording`   | Specific recordings/performances   | ISRC identification, technical metadata, contributor tracking  |
| `Release`     | Commercial releases (albums, etc.) | EAN/UPC codes, distribution metadata, format specifications    |

### Utility Types

- **Date** - Simple date representation without timezone complexity
- **Language** - Comprehensive language enumeration for internationalization
- **Country** - ISO 3166-1 alpha-2 country codes
- **Key** - Musical key notation (major/minor, sharps/flats, enharmonic equivalents)

## Core Types

All MIDDS types are built using bounded vectors for efficient storage:

### Type Aliases

```rust
// Bounded string with maximum length S
pub type MiddsString<const S: u32> = BoundedVec<u8, ConstU32<S>>;

// Bounded vector with maximum S elements
pub type MiddsVec<T, const S: u32> = BoundedVec<T, ConstU32<S>>;

// Unique identifier type
pub type MiddsId = u64;
```

### Generated Traits

All types implement:
- `Encode`, `Decode`, `DecodeWithMemTracking` (Codec traits)
- `TypeInfo`, `MaxEncodedLen` (Substrate metadata traits)
- `Debug`, `Clone`, `PartialEq`, `Eq` (Standard traits)

## Feature Flags

| Feature              | Description                        | Default |
| -------------------- | ---------------------------------- | ------- |
| `std`                | Standard library support           | ✅      |
| `runtime-benchmarks` | Benchmarking utilities             | ❌      |

## Type Bounds Reference

### Identifiers

- **ISWC**: 11 characters (`MiddsString<11>`)
- **ISRC**: 12 characters (`MiddsString<12>`)
- **EAN/UPC**: 13 characters (`MiddsString<13>`)
- **ISNI**: 16 characters (`MiddsString<16>`)

### Text Fields

- **Titles/Names**: 256 characters (`MiddsString<256>`)
- **Optional Text**: 256 characters (`MiddsString<256>`)

### Collections

- **Small Lists**: 5-64 items depending on type (`MiddsVec<T, 64>`)
- **Medium Lists**: 256 items (creators, contributors) (`MiddsVec<T, 256>`)
- **Large Lists**: 512-1024 items (recordings) (`MiddsVec<T, 1024>`)

## Examples

### Creating a Complete Musical Work

```rust
use allfeat_midds_v2::{
    musical_work::*,
    shared::{Language, Key, PartyId},
};

let classical_work = MusicalWork {
    iswc: b"T1234567890".to_vec().try_into().unwrap(),
    title: b"Symphony No. 9 in D minor".to_vec().try_into().unwrap(),
    creation_year: Some(1824),
    instrumental: Some(true),
    language: None,
    bpm: Some(72),
    key: Some(Key::Dm),
    work_type: Some(MusicalWorkType::Original),
    creators: vec![
        Creator {
            id: PartyId::Ipi(1),
            role: CreatorRole::Composer,
        }
    ].try_into().unwrap(),
    classical_info: Some(ClassicalInfo {
        opus: Some(b"Op. 125".to_vec().try_into().unwrap()),
        catalog_number: Some(b"LvB 125".to_vec().try_into().unwrap()),
        number_of_voices: Some(4),
    }),
};
```

### Creating a Recording

```rust
use allfeat_midds_v2::{
    recording::*,
    shared::{Key, PartyId, genres::GenreId},
    MiddsId,
};

let recording = Recording {
    isrc: b"USUM71703861".to_vec().try_into().unwrap(),
    musical_work: 12345, // Reference to the underlying work
    performer: PartyId::Ipi(67890), // Primary performer
    producers: vec![PartyId::Ipi(11111), PartyId::Ipi(22222)].try_into().unwrap(),
    contributors: vec![PartyId::Ipi(55555), PartyId::Ipi(66666)].try_into().unwrap(),
    recording_year: Some(1975),
    genres: vec![GenreId::Rock, GenreId::Pop].try_into().unwrap(),
    duration: Some(355), // 5:55 in seconds
    bpm: Some(72),
    key: Some(Key::Bb),
    recording_place: Some(b"Rockfield Studios, Wales".to_vec().try_into().unwrap()),
    mixing_place: Some(b"Wessex Studios, London".to_vec().try_into().unwrap()),
    mastering_place: None,
};
```

### Creating a Release

```rust
use allfeat_midds_v2::{
    release::*,
    shared::{Date, Country, PartyId},
    MiddsId,
};

let release = Release {
    ean: b"1234567890123".to_vec().try_into().unwrap(),
    artist: PartyId::Ipi(67890),
    title: b"A Night at the Opera".to_vec().try_into().unwrap(),
    recordings: vec![100, 101, 102].try_into().unwrap(), // recording IDs
    release_date: Date { year: 1975, month: 11, day: 21 },
    release_country: Country::GB,
    release_type: Some(ReleaseType::Lp),
    release_format: Some(ReleaseFormat::Cd),
    release_status: Some(ReleaseStatus::Remastered),
    // ... other optional fields
};
```

## Best Practices

### 1. Bounds Selection

Choose appropriate bounds based on real-world usage:

```rust
type ArtistName = MiddsString<64>;   // Artist/band names (typically < 50 chars)
type SongTitle = MiddsString<256>;   // Song titles (rarely > 100 chars)
type RecordingList = MiddsVec<MiddsId, 1024>; // Recording lists (albums rarely > 50 tracks)
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
# ❌ Wrong - enable default and no-std
default-features = true
features = []

# ✅ Correct - no-std with benchmarking
default-features = false
features = ["runtime-benchmarks"]
```

## Testing

Run tests for different feature combinations:

```bash
# Std mode tests (default)
cargo test

# No-std mode tests
cargo test --no-default-features

# Benchmarking tests
cargo test --features "runtime-benchmarks"
```

## Contributing

1. Use appropriate `MiddsString<N>` and `MiddsVec<T, N>` bounds for fields
2. Implement required Substrate traits (`Encode`, `Decode`, `TypeInfo`, etc.)
3. Include comprehensive documentation for all public types
4. Add test cases for both std and no-std modes
5. Follow existing naming conventions and code style

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Related Projects

- [Allfeat SDK](https://github.com/allfeat/allfeat-sdk) - Complete Allfeat development toolkit
- [Substrate](https://substrate.io/) - Blockchain development framework
