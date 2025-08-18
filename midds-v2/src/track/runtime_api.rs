use super::{isrc::RuntimeIsrc, RuntimeTrack, TrackVersion};
use crate::{track::error::TrackError, utils::Key, MiddsId};
use frame_support::BoundedVec;

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};

impl RuntimeTrack {
    /// Creates a new RuntimeTrack from raw parts
    pub fn new_from_parts(
        isrc: RuntimeIsrc,
        musical_work: MiddsId,
        artist: MiddsId,
        title: BoundedVec<u8, frame_support::traits::ConstU32<256>>,
        producers: BoundedVec<MiddsId, frame_support::traits::ConstU32<64>>,
        performers: BoundedVec<MiddsId, frame_support::traits::ConstU32<256>>,
        contributors: BoundedVec<MiddsId, frame_support::traits::ConstU32<256>>,
    ) -> Result<Self, TrackError> {
        Ok(Self {
            isrc,
            musical_work,
            artist,
            producers,
            performers,
            contributors,
            title,
            title_aliases: BoundedVec::new(),
            recording_year: None,
            genres: BoundedVec::new(),
            version: None,
            duration: None,
            bpm: None,
            key: None,
            recording_place: None,
            mixing_place: None,
            mastering_place: None,
        })
    }

    /// Returns the capacity limits for bounded fields
    pub const fn capacity_limits() -> RuntimeTrackCapacityLimits {
        RuntimeTrackCapacityLimits {
            producers: 64,
            performers: 256,
            contributors: 256,
            title_aliases: 16,
            genres: 5,
        }
    }

    /// Returns the current number of producers
    pub fn producer_count(&self) -> u32 {
        self.producers.len() as u32
    }

    /// Returns the current number of performers
    pub fn performer_count(&self) -> u32 {
        self.performers.len() as u32
    }

    /// Returns the current number of contributors
    pub fn contributor_count(&self) -> u32 {
        self.contributors.len() as u32
    }

    /// Returns the total number of contributors
    pub fn total_contributor_count(&self) -> u32 {
        self.producer_count() + self.performer_count() + self.contributor_count()
    }

    /// Returns the current number of title aliases
    pub fn alias_count(&self) -> u32 {
        self.title_aliases.len() as u32
    }

    /// Returns the current number of genres
    pub fn genre_count(&self) -> u32 {
        self.genres.len() as u32
    }

    /// Returns the recording year if set
    pub fn get_recording_year(&self) -> Option<u16> {
        self.recording_year
    }

    /// Returns the duration if set
    pub fn get_duration(&self) -> Option<u16> {
        self.duration
    }

    /// Returns the BPM if set
    pub fn get_bpm(&self) -> Option<u16> {
        self.bpm
    }

    /// Returns the musical key if set
    pub fn get_key(&self) -> Option<Key> {
        self.key
    }

    /// Returns the track version if set
    pub fn get_version(&self) -> Option<TrackVersion> {
        self.version
    }

    /// Checks if this is a live version
    pub fn is_live_version(&self) -> bool {
        matches!(self.version, Some(TrackVersion::Live))
    }

    /// Checks if this is an acoustic version
    pub fn is_acoustic_version(&self) -> bool {
        matches!(self.version, Some(TrackVersion::Acoustic))
    }

    /// Checks if this is an instrumental version
    pub fn is_instrumental_version(&self) -> bool {
        matches!(self.version, Some(TrackVersion::Instrumental))
    }

    /// Checks if the track was recorded in a specific year
    pub fn recorded_in_year(&self, year: u16) -> bool {
        self.recording_year == Some(year)
    }

    /// Returns the age of the recording in years
    pub fn recording_age_years(&self) -> Option<u16> {
        let current_year = 2025u16; // TODO: use actual current year
        self.recording_year
            .map(|year| current_year.saturating_sub(year))
    }

    /// Converts runtime places to strings if they contain valid UTF-8
    pub fn places_to_strings(&self) -> RuntimeTrackPlaces {
        RuntimeTrackPlaces {
            recording_place: self
                .recording_place
                .as_ref()
                .map(|p| String::from_utf8_lossy(p).to_string()),
            mixing_place: self
                .mixing_place
                .as_ref()
                .map(|p| String::from_utf8_lossy(p).to_string()),
            mastering_place: self
                .mastering_place
                .as_ref()
                .map(|p| String::from_utf8_lossy(p).to_string()),
        }
    }
}

/// Capacity limits for RuntimeTrack bounded fields
pub struct RuntimeTrackCapacityLimits {
    pub producers: u32,
    pub performers: u32,
    pub contributors: u32,
    pub title_aliases: u32,
    pub genres: u32,
}

/// Runtime track places as strings
pub struct RuntimeTrackPlaces {
    pub recording_place: Option<String>,
    pub mixing_place: Option<String>,
    pub mastering_place: Option<String>,
}
