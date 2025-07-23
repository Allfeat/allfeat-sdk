//! Shared types and utilities used across all MIDDS

#[cfg(feature = "std")]
pub mod conversion;

pub mod utils;

// Re-exports for convenience
#[cfg(feature = "std")]
pub use conversion::{ConversionError, Validatable, ValidationError};

pub use utils::{Country, Date, Key, Language};
