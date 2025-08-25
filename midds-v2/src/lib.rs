//! # MIDDS v2 - Music Industry Data Description Standard
//!
//! This crate provides comprehensive data structures for representing music industry metadata
//! in a blockchain-compatible format. MIDDS v2 includes structures for musical works, tracks,
//! releases, and associated metadata.
//!
//! ## Core Types
//!
//! - [`MusicalWork`](musical_work::MusicalWork) - Represents compositions and songwriting metadata
//! - [`Recording`](recording::Recording) - Represents recordings and performance metadata
//! - [`Release`](release::Release) - Represents albums, EPs, singles and distribution metadata
//!
//! ## Key Features
//!
//! - **Substrate Compatible**: All types implement necessary traits for blockchain storage
//! - **Validation-Free**: No runtime validation, designed for application-level validation
//! - **Industry Standards**: Uses ISWC, ISRC, EAN/UPC and other industry identifiers
//! - **Comprehensive Metadata**: Supports extensive metadata for all music industry use cases
//!
//! ## Example Usage
//!
//! ```rust
//! use allfeat_midds_v2::{
//!     musical_work::{MusicalWork, Creator, CreatorRole},
//!     shared::{PartyId, Key, Language},
//! };
//!
//! // Create a musical work
//! let work = MusicalWork {
//!     iswc: b"T1234567890".to_vec().try_into().unwrap(),
//!     title: b"Example Song".to_vec().try_into().unwrap(),
//!     creation_year: Some(2024),
//!     instrumental: Some(false),
//!     language: Some(Language::English),
//!     bpm: Some(120),
//!     key: Some(Key::C),
//!     work_type: None,
//!     creators: vec![Creator {
//!         id: PartyId::Ipi(123456789),
//!         role: CreatorRole::Composer,
//!     }].try_into().unwrap(),
//!     classical_info: None,
//! };
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{traits::ConstU32, BoundedVec};

/// Unique identifier type used across all MIDDS entities.
///
/// This type represents a unique 64-bit identifier that can be used to reference
/// musical works, tracks, releases, or parties within the MIDDS ecosystem.
///
/// # Example
///
/// ```rust
/// use allfeat_midds_v2::MiddsId;
///
/// let work_id: MiddsId = 12345;
/// let recording_id: MiddsId = 67890;
/// ```
pub type MiddsId = u64;

/// Bounded string type used throughout MIDDS for text fields.
///
/// This type provides a space-efficient, bounded string representation that is compatible
/// with Substrate's storage requirements. The generic parameter `S` defines the maximum
/// length in bytes.
///
/// # Example
///
/// ```rust
/// use allfeat_midds_v2::MiddsString;
///
/// // Create a bounded string with max 256 bytes
/// let title: MiddsString<256> = b"My Song Title".to_vec().try_into().unwrap();
/// assert_eq!(title.len(), 13);
/// ```
pub type MiddsString<const S: u32> = BoundedVec<u8, ConstU32<S>>;

/// Bounded vector type used throughout MIDDS for collections.
///
/// This type provides a space-efficient, bounded collection that is compatible
/// with Substrate's storage requirements. The generic parameter `S` defines the maximum
/// number of elements.
///
/// # Example
///
/// ```rust
/// use allfeat_midds_v2::{MiddsVec, MiddsId};
///
/// // Create a bounded vector of recording IDs with max 10 elements
/// let recording_ids: MiddsVec<MiddsId, 10> = vec![1, 2, 3].try_into().unwrap();
/// assert_eq!(recording_ids.len(), 3);
/// ```
pub type MiddsVec<T, const S: u32> = BoundedVec<T, ConstU32<S>>;

pub mod musical_work;

pub mod release;

pub mod recording;

/// Shared utility types and common enumerations.
///
/// Contains common types used across all MIDDS structures including dates,
/// countries, languages, and musical keys.
pub mod shared;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;
