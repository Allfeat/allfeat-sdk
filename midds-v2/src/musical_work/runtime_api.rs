
use super::*;
use frame_support::BoundedVec;

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use thiserror::Error;

/// Error types for RuntimeMusicalWork operations
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum RuntimeMusicalWorkError {
    /// Data exceeds capacity limits
    #[error("Data exceeds capacity limits: {0}")]
    ExceedsCapacity(String),
    /// Invalid UTF-8 data
    #[error("Invalid UTF-8 data")]
    InvalidUtf8,
    /// Empty creators list
    #[error("Musical work must have at least one creator")]
    EmptyCreators,
}

impl RuntimeMusicalWork {
    /// Creates a new RuntimeMusicalWork from components without validation.
    ///
    /// # Arguments
    /// * `iswc` - The ISWC identifier
    /// * `title_bytes` - The title as bytes
    /// * `creators` - List of creators
    ///
    /// # Returns
    /// * `Ok(RuntimeMusicalWork)` if all data fits within bounds
    /// * `Err(RuntimeMusicalWorkError)` if data exceeds capacity
    pub fn new_from_components(
        iswc: iswc::RuntimeIswc,
        title_bytes: impl AsRef<[u8]>,
        creators: Vec<Creator>,
    ) -> Result<Self, RuntimeMusicalWorkError> {
        if creators.is_empty() {
            return Err(RuntimeMusicalWorkError::EmptyCreators);
        }

        let title = BoundedVec::try_from(title_bytes.as_ref().to_vec())
            .map_err(|_| RuntimeMusicalWorkError::ExceedsCapacity("title".to_string()))?;

        let creators_bounded = BoundedVec::try_from(creators)
            .map_err(|_| RuntimeMusicalWorkError::ExceedsCapacity("creators".to_string()))?;

        Ok(Self {
            iswc,
            title,
            creation_year: None,
            instrumental: None,
            language: None,
            bpm: None,
            key: None,
            work_type: None,
            creators: creators_bounded,
            classical_info: None,
        })
    }

    /// Creates a new RuntimeMusicalWork from string data.
    pub fn new_from_strings(
        iswc_str: &str,
        title: &str,
        creators: Vec<Creator>,
    ) -> Result<Self, RuntimeMusicalWorkError> {
        let iswc = iswc::RuntimeIswc::new_from_str(iswc_str)
            .map_err(|_| RuntimeMusicalWorkError::ExceedsCapacity("iswc".to_string()))?;

        Self::new_from_components(iswc, title.as_bytes(), creators)
    }

    /// Returns the title as a string if it contains valid UTF-8.
    pub fn title_string(&self) -> Result<String, RuntimeMusicalWorkError> {
        String::from_utf8(self.title.to_vec()).map_err(|_| RuntimeMusicalWorkError::InvalidUtf8)
    }

    /// Returns the title as a lossy string.
    pub fn title_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.title).to_string()
    }

    /// Returns the title as bytes.
    pub fn title_bytes(&self) -> &[u8] {
        &self.title
    }

    /// Returns the ISWC as bytes.
    pub fn iswc_bytes(&self) -> &[u8] {
        self.iswc.as_bytes()
    }

    /// Returns the number of creators.
    pub fn creator_count(&self) -> usize {
        self.creators.len()
    }

    /// Checks if the work has lyrics.
    pub fn has_lyrics(&self) -> bool {
        !self.instrumental.unwrap_or(false)
    }

    /// Checks if this is a classical work.
    pub fn is_classical(&self) -> bool {
        self.classical_info.is_some()
    }

    /// Returns capacity information.
    pub fn capacity_info() -> (u32, u32) {
        (256, 256) // (title_capacity, creators_capacity)
    }

    /// Returns current usage.
    pub fn current_usage(&self) -> (usize, usize) {
        (self.title.len(), self.creators.len())
    }
}

impl RuntimeClassicalInfo {
    /// Creates new RuntimeClassicalInfo from string data.
    pub fn new_from_strings(
        opus: Option<&str>,
        catalog_number: Option<&str>,
        number_of_voices: Option<u16>,
    ) -> Result<Self, RuntimeMusicalWorkError> {
        let opus_bounded = if let Some(opus_str) = opus {
            Some(
                BoundedVec::try_from(opus_str.as_bytes().to_vec())
                    .map_err(|_| RuntimeMusicalWorkError::ExceedsCapacity("opus".to_string()))?,
            )
        } else {
            None
        };

        let catalog_bounded = if let Some(catalog_str) = catalog_number {
            Some(
                BoundedVec::try_from(catalog_str.as_bytes().to_vec()).map_err(|_| {
                    RuntimeMusicalWorkError::ExceedsCapacity("catalog_number".to_string())
                })?,
            )
        } else {
            None
        };

        Ok(Self {
            opus: opus_bounded,
            catalog_number: catalog_bounded,
            number_of_voices,
        })
    }

    /// Returns the opus as a string if it contains valid UTF-8.
    pub fn opus_string(&self) -> Option<Result<String, RuntimeMusicalWorkError>> {
        self.opus.as_ref().map(|opus| {
            String::from_utf8(opus.to_vec()).map_err(|_| RuntimeMusicalWorkError::InvalidUtf8)
        })
    }

    /// Returns the catalog number as a string if it contains valid UTF-8.
    pub fn catalog_number_string(&self) -> Option<Result<String, RuntimeMusicalWorkError>> {
        self.catalog_number.as_ref().map(|catalog| {
            String::from_utf8(catalog.to_vec()).map_err(|_| RuntimeMusicalWorkError::InvalidUtf8)
        })
    }
}
