use thiserror::Error;

#[cfg(feature = "runtime")]
extern crate alloc;
#[cfg(feature = "runtime")]
use alloc::string::String;

/// Error types for Release operations (both std and runtime modes)
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ReleaseError {
    /// Invalid EAN/UPC
    #[error("Invalid EAN/UPC: {0}")]
    InvalidEan(String),
    /// Empty tracks list
    #[error("Release must have at least one track")]
    EmptyTracks,
    /// Invalid title
    #[error("Invalid title: {0}")]
    InvalidTitle(String),
    /// Invalid distributor name
    #[error("Invalid distributor name: {0}")]
    InvalidDistributor(String),
    /// Invalid manufacturer name
    #[error("Invalid manufacturer name: {0}")]
    InvalidManufacturer(String),
    /// Invalid release date
    #[error("Invalid release date")]
    InvalidDate,
    /// Too many tracks
    #[error("Too many tracks (max 1024): {0}")]
    TooManyTracks(usize),
    /// Too many producers
    #[error("Too many producers (max 256): {0}")]
    TooManyProducers(usize),
    /// Too many cover contributors
    #[error("Too many cover contributors (max 64): {0}")]
    TooManyCoverContributors(usize),
    /// Too many title aliases
    #[error("Too many title aliases (max 16): {0}")]
    TooManyTitleAliases(usize),
    /// Data exceeds capacity limits (runtime mode)
    #[error("Data exceeds capacity limits")]
    ExceedsCapacity,
    /// Invalid UTF-8 data (runtime mode)
    #[error("Invalid UTF-8 data")]
    InvalidUtf8,
}
