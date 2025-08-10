//! # MIDDS - Musical Industry Decentralized Data Standard
//!
//! This crate provides a comprehensive set of data structures and utilities for representing
//! musical metadata in both native Rust environments and Substrate runtime environments.
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
//! - **Native Mode** (`native` feature): Uses standard Rust types (`String`, `Vec<T>`) for native applications with a powerful API.
//! - **Runtime Mode** (`runtime` feature): Uses Substrate-compatible types (`BoundedVec`) for blockchain runtime, with minimal stuff to store it on-chain.
//!
//! ### Automatic Type Transformation
//! The `runtime_midds` procedural macro automatically transforms types between native and runtime modes:
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
//! - `native` - Use native Rust types (`String`, `Vec`) (default, conflicts with `runtime`)
//! - `runtime` - Use Substrate runtime types (`BoundedVec`) (conflicts with `native`)
//! - `runtime-benchmarks` - Enable benchmarking utilities (requires `runtime`)
//!
//! ## Quick Start
//!
//! ### Native Usage
//! ```rust
//! use allfeat_midds_v2::{MusicalWork, Track, Release};
//!
//! // In native mode, uses standard String and Vec types
//! let work = MusicalWork {
//!     title: "My Song".to_string(),
//!     participants: vec![/* ... */],
//!     // ...
//! };
//! ```
//!
//! ### Runtime Usage
//! ```rust
//! use allfeat_midds_v2::{MusicalWork, Track, Release};
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
//! ```rust
//! #[cfg(feature = "runtime-benchmarks")]
//! use allfeat_midds_v2::benchmarking::BenchmarkHelper;
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

// Compile-time check to prevent incompatible feature combinations
#[cfg(all(feature = "runtime", feature = "native"))]
compile_error!(
    "Features 'runtime' and 'native' are mutually exclusive and cannot both be enabled. \
     Use 'native' for native SDK types (String, Vec) or 'runtime' for Substrate BoundedVec types."
);

/// Universal identifier type for all MIDDS entities.
///
/// This type represents unique identifiers across all MIDDS types (Musical Works, Tracks, Releases).
/// In a blockchain context, this would typically be assigned by the runtime when entities are registered.
pub type MiddsId = u64;

/// Musical work definitions and metadata structures.
///
/// Contains the core [`MusicalWork`] type and related definitions for representing
/// compositions, songs, and other musical creations with their participants and metadata.
///
/// [`MusicalWork`]: musical_work::MusicalWork
pub mod musical_work;

/// Release definitions and commercial metadata structures.
///
/// Contains the [`Release`] type and related definitions for representing
/// albums, EPs, singles, and other commercial music releases with distribution metadata.
///
/// [`Release`]: release::Release
pub mod release;

/// Track definitions and recording metadata structures.
///
/// Contains the [`Track`] type and related definitions for representing
/// specific recordings or performances of musical works with technical and contributor metadata.
///
/// [`Track`]: track::Track
pub mod track;

/// Shared utility types and common enumerations.
///
/// Contains common types used across all MIDDS structures including dates,
/// countries, languages, and musical keys.
pub mod utils;

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

#[cfg(test)]
mod mock_tests;
