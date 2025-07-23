// This file is part of Allfeat.

// Copyright (C) 2022-2025 Allfeat.
// SPDX-License-Identifier: GPL-3.0-or-later

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! # Allfeat MIDDS - Music Industry Decentralized Data Structures
//!
//! This crate provides Substrate-compatible implementations of MIDDS (Music Industry
//! Decentralized Data Structures) for the Allfeat blockchain ecosystem.
//!
//! ## Overview
//!
//! MIDDS defines standardized data structures for representing music industry entities:
//! - **Musical Works**: Compositions and songs (identified by ISWC)
//! - **Party Identifiers**: Artists, publishers, and other music industry parties (identified by IPI/ISNI)
//! - **Releases**: Albums, EPs, and other music releases (identified by EAN/UPC)
//! - **Tracks**: Individual recordings (identified by ISRC)
//!
//! ## Key Features
//!
//! - 🔗 **Substrate Integration**: Full compatibility with Substrate/Polkadot ecosystem
//! - 🛡️ **Type Safety**: Strong typing with comprehensive validation
//! - 🚀 **Performance**: Optimized for on-chain storage and operations
//! - 🔄 **Dual Types**: Separate SDK and Runtime types for optimal UX and efficiency
//! - 📊 **Standards Compliant**: Implements music industry standard identifiers
//!
//! ## Quick Start
//!
//! ```rust
//! use allfeat_midds::{
//!     party_identifier::Ipi,
//!     musical_work::Iswc,
//!     shared::conversion::Validatable,
//! };
//! use std::str::FromStr;
//!
//! // Create music industry identifiers
//! let ipi: Ipi = 123456789;
//! let iswc = Iswc::from_str("T1234567890").unwrap();
//!
//! // Validate identifiers (std feature required)
//! #[cfg(feature = "std")]
//! {
//!     assert!(iswc.validate().is_ok());
//! }
//! ```
//!
//! ## Architecture
//!
//! The crate is organized into several modules:
//!
//! - [`shared`]: Common types, identifiers, and utilities
//! - [`musical_work`]: Musical work (composition) data structures
//! - [`party_identifier`]: Party (artist/entity) identification
//! - [`release`]: Music release data structures
//! - [`track`]: Individual track/recording data structures
//!
//! Each module provides both SDK types (for application development) and Runtime types
//! (for blockchain storage), with automatic conversion between them.
//!
//! ## Feature Flags
//!
//! - `std` (default): Enables SDK types and validation features
//! - `sdk`: Alias for std features
//! - `try-runtime`: Enables try-runtime features for Substrate integration
//! - `runtime-benchmarks`: Enables benchmarking utilities for Substrate pallets
//!
//! ## Benchmarking Support
//!
//! When the `runtime-benchmarks` feature is enabled, each MIDDS type provides
//! benchmarking helpers to generate data of varying complexity for accurate
//! weight calculation in Substrate pallets:
//!
//! ```rust,ignore
//! use allfeat_midds::{
//!     benchmarking::BenchmarkHelperT,
//!     track::Track,
//!     Midds,
//! };
//!
//! // Generate MIDDS with specific complexity (0.0 = minimal, 1.0 = maximal)
//! let track = <Track as Midds>::BenchmarkHelper::variable_size(0.5);
//!
//! // Or use predefined sizes
//! let min_track = <Track as Midds>::BenchmarkHelper::min_size();
//! let max_track = <Track as Midds>::BenchmarkHelper::max_size();
//! ```
//!
//! This enables precise weight calculation based on actual data size variations,
//! following Substrate's best practices for benchmarking.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    Blake2_256, Parameter, StorageHasher, dispatch::DispatchResult, pallet_prelude::Member,
};
use parity_scale_codec::MaxEncodedLen;

// MIDDS modules
pub mod musical_work;
pub mod party_identifier;
pub mod release;
pub mod track;

// Shared utilities
pub mod shared;

// Re-export conversion module at root level for MiddsSdk macro compatibility
#[cfg(feature = "std")]
pub use shared::conversion;

// Benchmarking utilities
#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

#[cfg(feature = "js")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "js")]
#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}

/// Generic Midds Identifier expected to be used for storing in pallets.
pub type MiddsId = u64;

/// Substrate-compatible MIDDS (Music Industry Decantralized Data Structure) interface definition.
pub trait Midds: Parameter + Member + MaxEncodedLen {
    const NAME: &'static str;

    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper: benchmarking::BenchmarkHelperT<Self>;

    /// Return the integrity hash (with Blake2) of the encoded MIDDS.
    fn hash(&self) -> [u8; 32] {
        Blake2_256::hash(&self.encode())
    }

    /// A function that a MIDDS can implement to enforce specific validation logic.
    ///
    /// # Errors
    ///
    /// Returns a dispatch error if the MIDDS data is invalid
    fn validate(&self) -> DispatchResult {
        Ok(())
    }
}

pub mod pallet_prelude {
    //! Prelude for pallet developers - contains all runtime types and commonly used imports

    // Runtime types (for Substrate/Pallets)
    pub use super::{
        musical_work::MusicalWork,
        party_identifier::{Artist, Entity, PartyIdentifier, PartyType},
        release::Release,
        track::Track,
    };

    // Shared types
    pub use super::shared::{Country, Date, Key, Language};

    // Core types
    pub use super::{Midds, MiddsId};

    // MIDDS-specific types
    #[cfg(feature = "std")]
    pub use super::shared::{ConversionError, Validatable, ValidationError};
}
