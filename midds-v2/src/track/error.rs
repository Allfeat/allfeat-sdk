use thiserror::Error;

#[cfg(feature = "runtime")]
extern crate alloc;
#[cfg(feature = "runtime")]
use alloc::string::String;

/// Error types for Track operations (both std and runtime modes)
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum TrackError {
    /// Invalid track title
    #[error("Invalid track title: {0}")]
    InvalidTitle(String),
    /// Invalid ISRC
    #[error("Invalid ISRC: {0}")]
    InvalidIsrc(String),
    /// Invalid duration
    #[error("Invalid duration: {0}")]
    InvalidDuration(String),
    /// Invalid BPM
    #[error("Invalid BPM: {0}")]
    InvalidBpm(String),
    /// Invalid recording year
    #[error("Invalid recording year: {0}")]
    InvalidRecordingYear(String),
    /// Too many producers
    #[error("Too many producers (max 64): {0}")]
    TooManyProducers(usize),
    /// Too many performers
    #[error("Too many performers (max 256): {0}")]
    TooManyPerformers(usize),
    /// Too many contributors
    #[error("Too many contributors (max 256): {0}")]
    TooManyContributors(usize),
    /// Too many title aliases
    #[error("Too many title aliases (max 16): {0}")]
    TooManyTitleAliases(usize),
    /// Too many genres
    #[error("Too many genres (max 5): {0}")]
    TooManyGenres(usize),
    /// Empty title
    #[error("Track title cannot be empty")]
    EmptyTitle,
    /// Invalid place name
    #[error("Invalid place name: {0}")]
    InvalidPlace(String),
    /// Data exceeds capacity limits (runtime mode)
    #[error("Data exceeds capacity limits")]
    ExceedsCapacity,
    /// Invalid UTF-8 data (runtime mode)
    #[error("Invalid UTF-8 data")]
    InvalidUtf8,
}
