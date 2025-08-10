//! Metadata Generation Module
//!
//! This module generates Rust types from the Allfeat blockchain's runtime metadata.
//! The metadata is loaded from a pre-compiled scale file and used to create
//! type-safe interfaces for interacting with the Allfeat blockchain.
//!
//! The `melodie` module contains auto-generated types for:
//! - Storage queries
//! - Extrinsic calls
//! - Events
//! - Constants
//! - Runtime APIs
//!
//! These types are generated at compile time using the subxt macro and provide
//! a strongly-typed interface to the Allfeat blockchain runtime.

/// Auto-generated module containing all Allfeat blockchain runtime types.
///
/// This module is generated from the runtime metadata and provides type-safe
/// access to the Allfeat blockchain's storage, calls, events, and constants.
///
/// The JS transaction wrappers are automatically generated from this metadata.

#[subxt::subxt(
    runtime_metadata_path = "artifacts/melodie_metadata.scale",
    substitute_type(
        path = "allfeat_midds::musical_work::runtime::MusicalWork",
        with = "::subxt::utils::Static<::allfeat_midds::musical_work::MusicalWork>"
    ),
    substitute_type(
        path = "allfeat_midds::track::runtime::Track",
        with = "::subxt::utils::Static<::allfeat_midds::track::Track>"
    ),
    substitute_type(
        path = "allfeat_midds::release::runtime::Release",
        with = "::subxt::utils::Static<::allfeat_midds::release::Release>"
    )
)]
pub mod melodie {}
