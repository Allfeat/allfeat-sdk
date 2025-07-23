//! Party Identifier MIDDS
//!
//! Contains everything related to party identification: artists, entities, and their metadata.

pub mod runtime;
pub mod types;

// Re-exports
pub use runtime::{PartyType, Artist, Entity, PartyIdentifier};
pub use types::*;
