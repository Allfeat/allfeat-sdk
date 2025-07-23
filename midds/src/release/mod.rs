//! Release MIDDS
//!
//! Contains everything related to musical releases: albums, EPs, singles, and their metadata.

pub mod runtime;
pub mod types;

// Re-exports
pub use runtime::Release;
pub use types::*;
