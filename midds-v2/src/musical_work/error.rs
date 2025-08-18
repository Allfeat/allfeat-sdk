use thiserror::Error;

#[cfg(feature = "runtime")]
extern crate alloc;
#[cfg(feature = "runtime")]
use alloc::string::String;

/// Error types for MusicalWork operations (both std and runtime modes)
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum MusicalWorkError {
    /// Invalid title format
    #[error("Invalid title format: {0}")]
    InvalidTitle(String),
    /// Invalid creation year
    #[error("Invalid creation year: {0}")]
    InvalidCreationYear(u16),
    /// Invalid BPM value
    #[error("Invalid BPM value: {0}")]
    InvalidBpm(u16),
    /// Invalid number of voices
    #[error("Invalid number of voices: {0}")]
    InvalidVoices(u16),
    /// Invalid ISWC
    #[error("Invalid ISWC: {0}")]
    InvalidIswc(String),
    /// Empty creators list
    #[error("Musical work must have at least one creator")]
    EmptyCreators,
    /// Invalid opus format
    #[error("Invalid opus format: {0}")]
    InvalidOpus(String),
    /// Invalid catalog number format
    #[error("Invalid catalog number format: {0}")]
    InvalidCatalogNumber(String),
    /// Data exceeds capacity limits (runtime mode)
    #[error("Data exceeds capacity limits: {0}")]
    ExceedsCapacity(String),
    /// Invalid UTF-8 data (runtime mode)
    #[error("Invalid UTF-8 data")]
    InvalidUtf8,
}
