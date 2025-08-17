//! # MIDDS - Musical Industry Decentralized Data Standard
//!
//! This crate provides a comprehensive set of data structures and utilities for representing
//! musical metadata in both std Rust environments and Substrate runtime environments.
//!
//! ## Overview
//!
//! MIDDS currently defines standardized data structures for three core entities in the music industry:
//! - **Musical Works** - Compositions, songs, and other musical creations
//! - **Tracks** - Specific recordings or performances of musical works
//! - **Releases** - Albums, EPs, singles, and other commercial releases
//!
//! ## Key Features
//!
//! ### Dual Compilation Modes
//! This crate supports two mutually exclusive compilation modes:
//! - **Std Mode** (`std` feature): Uses standard Rust types (`String`, `Vec<T>`) for std applications with a powerful API.
//! - **Runtime Mode** (`runtime` feature): Uses Substrate-compatible types (`BoundedVec`) for blockchain runtime, with minimal stuff to store it on-chain.
//!
//! ### Automatic Type Transformation
//! The `runtime_midds` procedural macro automatically transforms types between std and runtime modes:
//! - `String` ↔ `BoundedVec<u8, ConstU32<N>>`
//! - `Vec<T>` ↔ `BoundedVec<T, ConstU32<N>>`
//! - Preserves `Option<T>` wrappers with recursive transformation
//!
//! ### Runtime Benchmarking Support
//! When both `runtime` and `runtime-benchmarks` features are enabled, the crate provides
//! comprehensive benchmarking utilities for performance testing in Substrate pallets.
//!
//! ## Feature Flags
//!
//! - `std` - Enable standard library support (default)
//! - `std` - Use std Rust types (`String`, `Vec`) (default, conflicts with `runtime`)
//! - `runtime` - Use Substrate runtime types (`BoundedVec`) (conflicts with `std`)
//! - `runtime-benchmarks` - Enable benchmarking utilities (requires `runtime`)
//!
//! ## Quick Start
//!
//! ### Std Usage
//! ```rust,ignore
//! use allfeat_midds_v2::{musical_work::MusicalWork, track::Track, release::Release};
//!
//! // In std mode, uses standard String and Vec types
//! let work = MusicalWork {
//!     title: "My Song".to_string(),
//!     participants: vec![/* ... */],
//!     // ...
//! };
//! ```
//!
//! ### Runtime Usage
//! ```rust,ignore
//! use allfeat_midds_v2::{musical_work::MusicalWork, track::Track, release::Release};
//! use frame_support::BoundedVec;
//!
//! // In runtime mode, uses BoundedVec types
//! let work = MusicalWork {
//!     title: BoundedVec::try_from("My Song".as_bytes().to_vec()).unwrap(),
//!     participants: BoundedVec::try_from(vec![/* ... */]).unwrap(),
//!     // ...
//! };
//! ```
//!
//! ### Benchmarking Usage
//! ```rust,ignore
//! #[cfg(feature = "runtime-benchmarks")]
//! use allfeat_midds_v2::benchmarking::BenchmarkHelper;
//! use allfeat_midds_v2::musical_work::MusicalWork;
//!
//! // Generate test instances for benchmarking
//! let work = MusicalWork::benchmark_instance(100); // Scale with i=100
//! ```
//!
//! ## Architecture
//!
//! The crate is structured into several modules:
//! - [`musical_work`] - Core musical work definitions and metadata
//! - [`track`] - Recording and performance-specific metadata
//! - [`release`] - Commercial release and distribution metadata
//! - [`utils`] - Shared utility types (dates, countries, languages, keys)
//! - [`benchmarking`] - Runtime benchmarking utilities (feature-gated)
//!
//! ## Type Bounds
//!
//! All bounded types use the `#[runtime_bound(N)]` attribute to specify maximum sizes:
//! - Small identifiers: 11-13 characters (ISWC, EAN, etc.)
//! - Titles and names: 256 characters
//! - Large collections: 256-1024 items depending on use case
//!
//! ## Safety and Validation
//!
//! - All bounded types enforce size limits at compile time in runtime mode
//! - Attribute validation ensures proper usage of `#[runtime_bound(N)]`
//! - Comprehensive test coverage ensures type transformation correctness

#![cfg_attr(not(feature = "std"), no_std)]

// Ensure web and runtime features are mutually exclusive
#[cfg(all(feature = "web", feature = "runtime"))]
compile_error!("MIDDS-V2: 'web' and 'runtime' features are mutually exclusive. Use either 'web' for WASM compatibility or 'runtime' for Substrate runtime.");

// With the new separate type generation approach, std and runtime can coexist
// Std types: MusicalWork, Iswc, etc.
// Runtime types: RuntimeMusicalWork, RuntimeIswc, etc.

/// Universal identifier type for all MIDDS entities.
///
/// This type represents unique identifiers across all MIDDS types (Musical Works, Tracks, Releases).
/// In a blockchain context, this would typically be assigned by the runtime when entities are registered.
pub type MiddsId = u64;

// Re-export the main error types for convenience
pub use error::{ErrorKind, MiddsError, MiddsResult};

/// Musical work definitions and metadata structures.
///
/// Contains the core [`MusicalWork`] type and related definitions for representing
/// compositions, songs, and other musical creations with their participants and metadata.
/// Also includes the [`Iswc`] identifier type.
///
/// [`MusicalWork`]: musical_work::MusicalWork
/// [`Iswc`]: musical_work::iswc::Iswc
pub mod musical_work;

/// Release definitions and commercial metadata structures.
///
/// Contains the [`Release`] type and related definitions for representing
/// albums, EPs, singles, and other commercial music releases with distribution metadata.
/// Also includes the [`Ean`] identifier type.
///
/// [`Release`]: release::Release
/// [`Ean`]: release::ean::Ean
pub mod release;

/// Track definitions and recording metadata structures.
///
/// Contains the [`Track`] type and related definitions for representing
/// specific recordings or performances of musical works with technical and contributor metadata.
/// Also includes the [`Isrc`] identifier type.
///
/// [`Track`]: track::Track
/// [`Isrc`]: track::isrc::Isrc
pub mod track;

/// Shared utility types and common enumerations.
///
/// Contains common types used across all MIDDS structures including dates,
/// countries, languages, and musical keys.
pub mod utils;

/// Unified error handling for all MIDDS operations.
///
/// This module provides a comprehensive error hierarchy that unifies all error types
/// across the MIDDS V2 codebase. Instead of having separate error types for each
/// module, it provides a single [`MiddsError`] type that can represent any error
/// condition that may occur.
///
/// [`MiddsError`]: error::MiddsError
pub mod error;

/// Runtime benchmarking utilities for performance testing.
///
/// This module is only available when both `runtime` and `runtime-benchmarks`
/// features are enabled. It provides tools for generating test instances of
/// MIDDS types with linear scaling for Substrate pallet benchmarking.
///
/// # Examples
///
/// ```rust,ignore
/// use allfeat_midds_v2::benchmarking::BenchmarkHelper;
/// use allfeat_midds_v2::MusicalWork;
///
/// // Generate a MusicalWork instance scaled for benchmarking
/// let work = MusicalWork::benchmark_instance(512);
/// ```
#[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
pub mod benchmarking;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

// Initialize the WebAssembly module
#[cfg(feature = "web")]
#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"🚀 Allfeat MIDDS SDK WASM loaded!".into());
}
