//! Shared types and utilities used across all MIDDS

pub mod conversion;

pub mod utils;

// Re-exports for convenience
pub use conversion::{ConversionError, Validatable, ValidationError};

pub use utils::{Country, Date, Key, Language};
