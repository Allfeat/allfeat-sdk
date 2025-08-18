use super::{
    ean::RuntimeEan, ReleaseFormat, ReleasePackaging, ReleaseStatus, ReleaseType, RuntimeRelease,
};
use crate::{
    release::error::ReleaseError,
    utils::{Country, Date},
    MiddsId,
};
use frame_support::{traits::ConstU32, BoundedVec};

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

impl RuntimeRelease {
    /// Creates a new RuntimeRelease from raw parts
    pub fn new_from_parts(
        ean_upc: RuntimeEan,
        artist: MiddsId,
        title: BoundedVec<u8, ConstU32<256>>,
        tracks: BoundedVec<MiddsId, frame_support::traits::ConstU32<1024>>,
        date: Date,
        country: Country,
    ) -> Result<Self, ReleaseError> {
        Ok(Self {
            ean_upc,
            artist,
            title,
            tracks,
            date,
            country,
            title_aliases: BoundedVec::new(),
            format: ReleaseFormat::Cd,
            packaging: ReleasePackaging::JewelCase,
            status: ReleaseStatus::Official,
            producers: BoundedVec::new(),
            distributor_name: BoundedVec::new(),
            manufacturer_name: BoundedVec::new(),
            cover_contributors: BoundedVec::new(),
            release_type: ReleaseType::Lp,
        })
    }

    /// Returns the capacity limits for bounded fields
    pub const fn capacity_limits() -> RuntimeReleaseCapacityLimits {
        RuntimeReleaseCapacityLimits {
            tracks: 1024,
            title_aliases: 16,
            producers: 32,
            cover_contributors: 16,
        }
    }

    /// Returns the current number of tracks
    pub fn track_count(&self) -> u32 {
        self.tracks.len() as u32
    }

    /// Returns true if tracks list is empty
    pub fn has_no_tracks(&self) -> bool {
        self.tracks.is_empty()
    }

    /// Returns the current number of title aliases
    pub fn alias_count(&self) -> u32 {
        self.title_aliases.len() as u32
    }

    /// Returns the current number of producers
    pub fn producer_count(&self) -> u32 {
        self.producers.len() as u32
    }

    /// Returns true if distributor name is set
    pub fn has_distributor(&self) -> bool {
        !self.distributor_name.is_empty()
    }

    /// Returns true if manufacturer name is set
    pub fn has_manufacturer(&self) -> bool {
        !self.manufacturer_name.is_empty()
    }

    /// Returns the current number of cover contributors
    pub fn cover_contributor_count(&self) -> u32 {
        self.cover_contributors.len() as u32
    }

    /// Returns the release year
    pub fn year(&self) -> u16 {
        self.date.year
    }

    /// Converts runtime tracks to string representation
    pub fn tracks_to_strings(&self) -> Vec<String> {
        self.tracks.iter().map(|id| id.to_string()).collect()
    }
}

/// Capacity limits for RuntimeRelease bounded fields
pub struct RuntimeReleaseCapacityLimits {
    pub tracks: u32,
    pub title_aliases: u32,
    pub producers: u32,
    pub cover_contributors: u32,
}
