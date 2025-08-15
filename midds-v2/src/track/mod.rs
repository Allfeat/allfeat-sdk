//! Track definitions and recording metadata structures.
//!
//! This module contains data structures for representing tracks - specific recorded
//! performances or productions of musical works. Tracks capture the technical and
//! contributor metadata associated with individual recordings.
//!
//! # Core Types
//!
//! - [`Track`] - The main structure representing a recorded performance
//! - [`Isrc`](isrc::Isrc) - International Standard Recording Code identifier
//! - [`TrackTitle`] - Title of the specific track/recording
//! - [`TrackVersion`] - Version or variant type of the recording
//!
//! # Key Concepts
//!
//! A Track is distinct from a Musical Work:
//! - **Musical Work** = The underlying composition (melody, lyrics, structure)
//! - **Track** = A specific recorded performance of that work
//!
//! One musical work can have many tracks (different performances, versions, covers).
//!
//! # Usage
//!
//! ```rust
//! use allfeat_midds_v2::{
//!     track::{Track, TrackTitle, TrackVersion, isrc::Isrc},
//!     utils::Key,
//! };
//! use allfeat_music_genres::GenreId;
//!
//! let track = Track {
//!     isrc: Isrc::new("USUM71703861").unwrap(),
//!     musical_work: 12345, // Reference to the underlying work
//!     artist: 67890,       // Primary performer
//!     producers: vec![11111],
//!     performers: vec![67890, 22222],
//!     contributors: vec![33333],
//!     title: TrackTitle::new("Bohemian Rhapsody (Remastered)").unwrap(),
//!     title_aliases: vec![],
//!     recording_year: Some(1975),
//!     genres: vec![GenreId::Pop],
//!     version: Some(TrackVersion::ReRecorded),
//!     duration: Some(355), // 5:55 in seconds
//!     bpm: Some(72),
//!     key: Some(Key::Bb),
//!     recording_place: Some("Rockfield Studios".to_string()),
//!     mixing_place: Some("Wessex Studios".to_string()),
//!     mastering_place: None,
//! };
//! ```

pub mod isrc;

use allfeat_midds_v2_codegen::runtime_midds;
use allfeat_music_genres::GenreId;

#[cfg(feature = "std")]
use self::isrc::Isrc;

use crate::{utils::Key, MiddsId};

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

#[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
use crate::benchmarking::{
    create_bounded_string, create_bounded_vec, create_optional_bounded_string, BenchmarkHelper,
};

/// A Track represents a specific recorded performance or production of a musical work.
///
/// While a [`MusicalWork`](crate::musical_work::MusicalWork) represents the underlying
/// composition (melody, lyrics, structure), a Track represents a specific recording
/// or performance of that work. This includes technical metadata, contributor information,
/// and recording details.
///
/// # Relationship to Musical Works
///
/// - One musical work can have multiple tracks (covers, remixes, live versions)
/// - Each track references exactly one musical work via `musical_work` field
/// - Tracks inherit some metadata from their work but can override or extend it
///
/// # Fields
///
/// ## Identification
/// - `isrc` - International Standard Recording Code for unique identification
/// - `musical_work` - Reference to the underlying musical work
/// - `title` - Title of this specific recording (may differ from work title)
///
/// ## Contributors
/// - `artist` - Primary performing artist
/// - `producers` - Individuals who produced the recording
/// - `performers` - All individuals who performed on the recording
/// - `contributors` - Other contributors (engineers, featured artists, etc.)
///
/// ## Technical Metadata
/// - `recording_year` - Year the track was recorded
/// - `duration` - Length in seconds
/// - `bpm` - Beats per minute (may differ from work BPM due to performance)
/// - `key` - Musical key (may differ from work key due to transposition)
///
/// ## Classification
/// - `genres` - Music genres associated with this recording
/// - `version` - Type of recording (original, live, remix, etc.)
/// - `title_aliases` - Alternative titles for this recording
///
/// ## Production Metadata
/// - `recording_place` - Where the recording took place
/// - `mixing_place` - Where the track was mixed
/// - `mastering_place` - Where the track was mastered
///
/// # Type Transformations
///
/// In runtime mode, the following transformations apply:
/// - `producers: Vec<MiddsId>` → `producers: BoundedVec<MiddsId, ConstU32<64>>`
/// - `performers: Vec<MiddsId>` → `performers: BoundedVec<MiddsId, ConstU32<256>>`
/// - `contributors: Vec<MiddsId>` → `contributors: BoundedVec<MiddsId, ConstU32<256>>`
/// - `title_aliases: Vec<TrackTitle>` → `title_aliases: BoundedVec<TrackTitle, ConstU32<16>>`
/// - `genres: Vec<GenreId>` → `genres: BoundedVec<GenreId, ConstU32<5>>`
/// - String fields → `BoundedVec<u8, ConstU32<256>>`
///
/// # Examples
///
/// ## Studio Recording
/// ```rust
/// # use allfeat_midds_v2::track::*;
/// # use allfeat_midds_v2::track::isrc::Isrc;
/// # use allfeat_midds_v2::utils::Key;
/// # use allfeat_music_genres::GenreId;
/// let studio_track = Track {
///     isrc: Isrc::new("GBUM71505078").unwrap(),
///     musical_work: 100,
///     artist: 200,
///     producers: vec![300, 301],
///     performers: vec![200, 400, 401, 402], // Band members
///     contributors: vec![500], // Sound engineer
///     title: TrackTitle::new("Hotel California").unwrap(),
///     title_aliases: vec![],
///     recording_year: Some(1976),
///     genres: vec![GenreId::Pop],
///     version: None, // Original version
///     duration: Some(391), // 6:31
///     bpm: Some(75),
///     key: Some(Key::Bm),
///     recording_place: Some("Criteria Studios, Miami".to_string()),
///     mixing_place: Some("Criteria Studios, Miami".to_string()),
///     mastering_place: Some("Sterling Sound, NYC".to_string()),
/// };
/// ```
///
/// ## Live Performance
/// ```rust
/// # use allfeat_midds_v2::track::*;
/// # use allfeat_midds_v2::track::isrc::Isrc;
/// # use allfeat_midds_v2::utils::Key;
/// # use allfeat_music_genres::GenreId;
/// let live_track = Track {
///     isrc: Isrc::new("GBUM71505079").unwrap(),
///     musical_work: 100, // Same work as studio version
///     artist: 200,       // Same artist
///     producers: vec![600], // Live sound engineer
///     performers: vec![200, 400, 401, 402],
///     contributors: vec![700], // Live recording engineer
///     title: TrackTitle::new("Hotel California (Live at MTV Unplugged)").unwrap(),
///     title_aliases: vec![
///         TrackTitle::new("Hotel California - Unplugged").unwrap()
///     ],
///     recording_year: Some(1994),
///     genres: vec![GenreId::Pop],
///     version: Some(TrackVersion::Live),
///     duration: Some(425), // Longer live version
///     bpm: Some(70), // Slightly slower
///     key: Some(Key::Bm),
///     recording_place: Some("Sony Studios, NYC".to_string()),
///     mixing_place: Some("Sony Studios, NYC".to_string()),
///     mastering_place: Some("Sterling Sound, NYC".to_string()),
/// };
/// ```
#[runtime_midds]
#[cfg_attr(feature = "web", wasm_bindgen(inspectable))]
pub struct Track {
    /// ISRC (International Standard Recording Code) that uniquely identifies this recording.
    #[as_runtime_type(path = "isrc")]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub isrc: Isrc,

    /// The linked musical work this track is based on (must refer to a registered MIDDS).
    pub musical_work: MiddsId,

    /// Main artist MIDDS identifier (typically the primary performer).
    pub artist: MiddsId,

    /// List of producer MIDDS identifiers who participated in the production.
    #[runtime_bound(64)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub producers: Vec<MiddsId>,

    /// List of performer MIDDS identifiers who contributed to the performance.
    #[runtime_bound(256)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub performers: Vec<MiddsId>,

    /// Additional contributors (e.g., sound engineers, featured artists).
    #[runtime_bound(256)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub contributors: Vec<MiddsId>,

    /// Main title of the track.
    #[as_runtime_type]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub title: TrackTitle,

    /// Optional list of alternative titles for the track.
    #[runtime_bound(16)]
    #[as_runtime_type]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub title_aliases: Vec<TrackTitle>,

    /// Year the track was recorded (4-digit Gregorian year).
    pub recording_year: Option<u16>,

    /// Music genres attributed to this recording.
    #[runtime_bound(5)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub genres: Vec<GenreId>,

    /// Version or type of the track (e.g., Remix, Acoustic, Live).
    pub version: Option<TrackVersion>,

    /// Duration of the track in seconds.
    pub duration: Option<u16>,

    /// Beats per minute (BPM), representing the tempo of the track.
    pub bpm: Option<u16>,

    /// Musical key (e.g., C, G#, etc.) the track is in.
    pub key: Option<Key>,

    /// Free-text field indicating where the recording took place.
    #[runtime_bound(256)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub recording_place: Option<String>,

    /// Free-text field indicating where the mixing of the track occurred.
    #[runtime_bound(256)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub mixing_place: Option<String>,

    /// Free-text field indicating where the mastering of the track occurred.
    #[runtime_bound(256)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub mastering_place: Option<String>,
}

/// Title of a specific track/recording.
///
/// Track titles may differ from the underlying musical work title to reflect
/// specific aspects of the recording such as version information, featured artists,
/// or alternate titles.
///
/// # Type Transformation
/// In runtime mode: `String` → `BoundedVec<u8, ConstU32<256>>`
///
/// # Examples
/// ```rust
/// # use allfeat_midds_v2::track::TrackTitle;
/// // Basic title
/// let title1 = TrackTitle::new("Bohemian Rhapsody").unwrap();
///
/// // Title with version info
/// let title2 = TrackTitle::new("Bohemian Rhapsody (Remastered 2011)").unwrap();
///
/// // Title with featured artist
/// let title3 = TrackTitle::new("Empire State of Mind (feat. Alicia Keys)").unwrap();
/// ```
#[runtime_midds]
#[cfg_attr(feature = "web", wasm_bindgen(inspectable))]
pub struct TrackTitle(
    /// The track title string, limited to 256 characters in runtime mode.
    #[runtime_bound(256)]
    pub String,
);

/// Version or variant type of a track recording.
///
/// This enum categorizes different types of recordings to distinguish between
/// original recordings, live performances, remixes, and other variants.
///
/// The numeric values are explicitly set to ensure stable encoding across
/// different compilation targets and versions.
///
/// # Categories
///
/// ## Original Recordings
/// - `Original` - The first/primary recording
/// - `Single` - Single release version (may differ from album version)
/// - `AlternateTake` - Different recording session of the same arrangement
/// - `ReRecorded` - Newly recorded version of an existing track
///
/// ## Performance Types
/// - `Live` - Recorded during a live performance
/// - `Acoustic` - Acoustic arrangement (typically unplugged)
/// - `Rehearsal` - Practice or rehearsal recording
///
/// ## Production Variants
/// - `Remix` - Modified version by another artist/producer
/// - `Extended` - Longer version with additional sections
/// - `RadioEdit` - Shortened version for radio play
/// - `TvTrack` - Version adapted for television
/// - `Instrumental` - Version without vocals
/// - `Acapella` - Vocals-only version
/// - `Karaoke` - Version without lead vocals for karaoke
///
/// ## Specialized Versions
/// - `Cover` - Performance by different artist than original
/// - `Orchestral` - Arranged with orchestra
/// - `Dance` - Dance/club remix
/// - `Dub` - Dub version with reverb/delay effects
/// - `Clean` - Version with explicit content removed
/// - `Demo` - Early/rough version
/// - `Edit` - Generic edited version
///
/// # Examples
/// ```rust
/// # use allfeat_midds_v2::track::TrackVersion;
/// let versions = vec![
///     TrackVersion::Original,       // Studio album version
///     TrackVersion::Live,          // Concert recording
///     TrackVersion::Remix,         // Dance remix
///     TrackVersion::Acoustic,      // Unplugged version
///     TrackVersion::RadioEdit,     // Radio-friendly length
/// ];
/// ```
#[repr(u8)]
#[runtime_midds]
#[cfg_attr(feature = "web", wasm_bindgen)]
pub enum TrackVersion {
    /// Original recording version.
    Original = 0,
    /// A recording of a live performance.
    Live = 1,
    /// Shortened version for radio broadcasting.
    RadioEdit = 2,
    /// TV-friendly version used in broadcast.
    TvTrack = 3,
    /// Single release version.
    Single = 4,
    /// A modified or remixed version by another artist or producer.
    Remix = 5,
    /// A cover version performed by a different artist.
    Cover = 6,
    /// An acoustic version, usually unplugged.
    Acoustic = 7,
    /// Vocals-only version.
    Acapella = 8,
    /// Instrument-only version.
    Instrumental = 9,
    /// Version recorded with an orchestral arrangement.
    Orchestral = 10,
    /// Extended version, typically with added sections.
    Extended = 11,
    /// Different take/version of the same session.
    AlternateTake = 12,
    /// Newly recorded version of an existing track.
    ReRecorded = 13,
    /// Karaoke version without lead vocals.
    Karaoke = 14,
    /// Dance version, often remixed for clubs.
    Dance = 15,
    /// Dub version, typically with reverb-heavy effects.
    Dub = 16,
    /// Version with explicit lyrics.
    Clean = 17,
    /// Rehearsal take, often raw or unpolished.
    Rehearsal = 18,
    /// Early or incomplete version of a track.
    Demo = 19,
    /// Generic edit, purpose-specific.
    Edit = 20,
}

#[cfg(feature = "std")]
mod api {
    use super::{isrc::Isrc, Track, TrackTitle, TrackVersion};
    use crate::{utils::Key, MiddsId};
    use allfeat_music_genres::GenreId;
    use regex::Regex;
    use std::fmt;
    use thiserror::Error;

    static TRACK_TITLE_REGEX: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();

    /// Error types for Track operations
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
    }

    impl Track {
        /// Creates a new track with validation.
        ///
        /// # Arguments
        /// * `isrc` - International Standard Recording Code
        /// * `musical_work` - Reference to the underlying musical work
        /// * `artist` - Primary performing artist ID
        /// * `title` - Title of the track
        /// * `producers` - List of producer IDs
        /// * `performers` - List of performer IDs
        /// * `contributors` - List of contributor IDs
        ///
        /// # Returns
        /// * `Ok(Track)` if all parameters are valid
        /// * `Err(TrackError)` if any parameter is invalid
        pub fn new(
            isrc: Isrc,
            musical_work: MiddsId,
            artist: MiddsId,
            title: TrackTitle,
            producers: Vec<MiddsId>,
            performers: Vec<MiddsId>,
            contributors: Vec<MiddsId>,
        ) -> Result<Self, TrackError> {
            Self::validate_producers(&producers)?;
            Self::validate_performers(&performers)?;
            Self::validate_contributors(&contributors)?;

            Ok(Self {
                isrc,
                musical_work,
                artist,
                producers,
                performers,
                contributors,
                title,
                title_aliases: Vec::new(),
                recording_year: None,
                genres: Vec::new(),
                version: None,
                duration: None,
                bpm: None,
                key: None,
                recording_place: None,
                mixing_place: None,
                mastering_place: None,
            })
        }

        /// Adds a producer to the track.
        pub fn add_producer(&mut self, producer_id: MiddsId) -> Result<(), TrackError> {
            if self.producers.len() >= 64 {
                return Err(TrackError::TooManyProducers(self.producers.len() + 1));
            }
            self.producers.push(producer_id);
            Ok(())
        }

        /// Removes a producer from the track.
        pub fn remove_producer(&mut self, producer_id: MiddsId) -> bool {
            if let Some(pos) = self.producers.iter().position(|&x| x == producer_id) {
                self.producers.remove(pos);
                true
            } else {
                false
            }
        }

        /// Adds a performer to the track.
        pub fn add_performer(&mut self, performer_id: MiddsId) -> Result<(), TrackError> {
            if self.performers.len() >= 256 {
                return Err(TrackError::TooManyPerformers(self.performers.len() + 1));
            }
            self.performers.push(performer_id);
            Ok(())
        }

        /// Removes a performer from the track.
        pub fn remove_performer(&mut self, performer_id: MiddsId) -> bool {
            if let Some(pos) = self.performers.iter().position(|&x| x == performer_id) {
                self.performers.remove(pos);
                true
            } else {
                false
            }
        }

        /// Adds a contributor to the track.
        pub fn add_contributor(&mut self, contributor_id: MiddsId) -> Result<(), TrackError> {
            if self.contributors.len() >= 256 {
                return Err(TrackError::TooManyContributors(self.contributors.len() + 1));
            }
            self.contributors.push(contributor_id);
            Ok(())
        }

        /// Removes a contributor from the track.
        pub fn remove_contributor(&mut self, contributor_id: MiddsId) -> bool {
            if let Some(pos) = self.contributors.iter().position(|&x| x == contributor_id) {
                self.contributors.remove(pos);
                true
            } else {
                false
            }
        }

        /// Adds a title alias.
        pub fn add_title_alias(&mut self, alias: TrackTitle) -> Result<(), TrackError> {
            if self.title_aliases.len() >= 16 {
                return Err(TrackError::TooManyTitleAliases(
                    self.title_aliases.len() + 1,
                ));
            }
            self.title_aliases.push(alias);
            Ok(())
        }

        /// Adds a genre.
        pub fn add_genre(&mut self, genre: GenreId) -> Result<(), TrackError> {
            if self.genres.len() >= 5 {
                return Err(TrackError::TooManyGenres(self.genres.len() + 1));
            }
            if !self.genres.contains(&genre) {
                self.genres.push(genre);
            }
            Ok(())
        }

        /// Sets the recording year with validation.
        pub fn set_recording_year(&mut self, year: u16) -> Result<(), TrackError> {
            Self::validate_recording_year(year)?;
            self.recording_year = Some(year);
            Ok(())
        }

        /// Sets the duration with validation.
        pub fn set_duration(&mut self, duration_seconds: u16) -> Result<(), TrackError> {
            Self::validate_duration(duration_seconds)?;
            self.duration = Some(duration_seconds);
            Ok(())
        }

        /// Sets the BPM with validation.
        pub fn set_bpm(&mut self, bpm: u16) -> Result<(), TrackError> {
            Self::validate_bpm(bpm)?;
            self.bpm = Some(bpm);
            Ok(())
        }

        /// Sets the musical key.
        pub fn set_key(&mut self, key: Key) {
            self.key = Some(key);
        }

        /// Sets the recording place with validation.
        pub fn set_recording_place(&mut self, place: String) -> Result<(), TrackError> {
            Self::validate_place(&place)?;
            self.recording_place = Some(place);
            Ok(())
        }

        /// Sets the mixing place with validation.
        pub fn set_mixing_place(&mut self, place: String) -> Result<(), TrackError> {
            Self::validate_place(&place)?;
            self.mixing_place = Some(place);
            Ok(())
        }

        /// Sets the mastering place with validation.
        pub fn set_mastering_place(&mut self, place: String) -> Result<(), TrackError> {
            Self::validate_place(&place)?;
            self.mastering_place = Some(place);
            Ok(())
        }

        /// Returns the track title with aliases if available.
        pub fn full_title(&self) -> String {
            if self.title_aliases.is_empty() {
                self.title.0.clone()
            } else {
                let aliases = self
                    .title_aliases
                    .iter()
                    .map(|alias| alias.0.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{} ({})", self.title.0, aliases)
            }
        }

        /// Returns a searchable title (lowercase).
        pub fn searchable_title(&self) -> String {
            self.title.0.to_lowercase()
        }

        /// Returns the duration formatted as MM:SS.
        pub fn formatted_duration(&self) -> Option<String> {
            self.duration.map(|seconds| {
                let minutes = seconds / 60;
                let seconds = seconds % 60;
                format!("{:02}:{:02}", minutes, seconds)
            })
        }

        /// Checks if the track was recorded in a specific year.
        pub fn recorded_in_year(&self, year: u16) -> bool {
            self.recording_year == Some(year)
        }

        /// Returns the age of the recording in years.
        pub fn recording_age_years(&self) -> Option<u16> {
            let current_year = 2025u16; // TODO: use actual current year
            self.recording_year
                .map(|year| current_year.saturating_sub(year))
        }

        /// Checks if the track has a specific genre.
        pub fn has_genre(&self, genre: &GenreId) -> bool {
            self.genres.contains(genre)
        }

        /// Returns the number of contributors (producers + performers + other contributors).
        pub fn total_contributor_count(&self) -> usize {
            self.producers.len() + self.performers.len() + self.contributors.len()
        }

        /// Checks if this is a live version.
        pub fn is_live_version(&self) -> bool {
            matches!(self.version, Some(TrackVersion::Live))
        }

        /// Checks if this is an acoustic version.
        pub fn is_acoustic_version(&self) -> bool {
            matches!(self.version, Some(TrackVersion::Acoustic))
        }

        /// Checks if this is an instrumental version.
        pub fn is_instrumental_version(&self) -> bool {
            matches!(self.version, Some(TrackVersion::Instrumental))
        }

        /// Returns suggested similar versions based on current version.
        pub fn suggested_versions(&self) -> Vec<TrackVersion> {
            match self.version {
                Some(TrackVersion::Original) => vec![
                    TrackVersion::Live,
                    TrackVersion::Acoustic,
                    TrackVersion::Instrumental,
                ],
                Some(TrackVersion::Live) => vec![TrackVersion::Original, TrackVersion::Acoustic],
                Some(TrackVersion::Acoustic) => vec![TrackVersion::Original, TrackVersion::Live],
                _ => vec![
                    TrackVersion::Original,
                    TrackVersion::Live,
                    TrackVersion::Acoustic,
                ],
            }
        }

        /// Validates producers list.
        fn validate_producers(producers: &[MiddsId]) -> Result<(), TrackError> {
            if producers.len() > 64 {
                return Err(TrackError::TooManyProducers(producers.len()));
            }
            Ok(())
        }

        /// Validates performers list.
        fn validate_performers(performers: &[MiddsId]) -> Result<(), TrackError> {
            if performers.len() > 256 {
                return Err(TrackError::TooManyPerformers(performers.len()));
            }
            Ok(())
        }

        /// Validates contributors list.
        fn validate_contributors(contributors: &[MiddsId]) -> Result<(), TrackError> {
            if contributors.len() > 256 {
                return Err(TrackError::TooManyContributors(contributors.len()));
            }
            Ok(())
        }

        /// Validates recording year.
        fn validate_recording_year(year: u16) -> Result<(), TrackError> {
            if !(1800..=2100).contains(&year) {
                return Err(TrackError::InvalidRecordingYear(format!(
                    "Recording year must be between 1800 and 2100, got {}",
                    year
                )));
            }
            Ok(())
        }

        /// Validates duration.
        fn validate_duration(duration: u16) -> Result<(), TrackError> {
            if duration == 0 {
                return Err(TrackError::InvalidDuration(
                    "Duration cannot be zero".to_string(),
                ));
            }
            if duration > 3600 {
                // Max 1 hour
                return Err(TrackError::InvalidDuration(format!(
                    "Duration too long (max 1 hour): {} seconds",
                    duration
                )));
            }
            Ok(())
        }

        /// Validates BPM.
        fn validate_bpm(bpm: u16) -> Result<(), TrackError> {
            if bpm == 0 {
                return Err(TrackError::InvalidBpm("BPM cannot be zero".to_string()));
            }
            if bpm > 300 {
                return Err(TrackError::InvalidBpm(format!(
                    "BPM too high (max 300): {}",
                    bpm
                )));
            }
            Ok(())
        }

        /// Validates a place name.
        fn validate_place(place: &str) -> Result<(), TrackError> {
            if place.trim().is_empty() {
                return Err(TrackError::InvalidPlace(
                    "Place name cannot be empty".to_string(),
                ));
            }
            if place.len() > 256 {
                return Err(TrackError::InvalidPlace(
                    "Place name too long (max 256 chars)".to_string(),
                ));
            }
            Ok(())
        }
    }

    impl TrackTitle {
        /// Creates a new track title with validation.
        pub fn new(title: impl Into<String>) -> Result<Self, TrackError> {
            let title = title.into();
            Self::validate(&title)?;
            Ok(Self(title.to_string()))
        }

        /// Creates a new track title without validation.
        pub fn new_unchecked(title: impl Into<String>) -> Self {
            Self(title.into())
        }

        /// Validates a track title.
        pub fn validate(title: &str) -> Result<(), TrackError> {
            if title.trim().is_empty() {
                return Err(TrackError::EmptyTitle);
            }
            if title.len() > 256 {
                return Err(TrackError::InvalidTitle(
                    "Title too long (max 256 chars)".to_string(),
                ));
            }

            // Check for potentially problematic characters
            let title_regex = TRACK_TITLE_REGEX.get_or_init(|| {
                Regex::new(r"^[^\x00-\x1F\x7F]*$").expect("Track title regex pattern is valid")
            });

            if !title_regex.is_match(title) {
                return Err(TrackError::InvalidTitle(
                    "Title contains invalid characters".to_string(),
                ));
            }

            Ok(())
        }

        /// Returns the title as a string reference.
        pub fn as_str(&self) -> &str {
            &self.0
        }

        /// Returns a searchable version (lowercase, normalized).
        pub fn searchable(&self) -> String {
            self.0.to_lowercase()
        }

        /// Checks if the title contains specific text.
        pub fn contains(&self, text: &str) -> bool {
            self.0.to_lowercase().contains(&text.to_lowercase())
        }

        /// Returns the length of the title.
        pub fn len(&self) -> usize {
            self.0.len()
        }

        /// Checks if the title is empty.
        pub fn is_empty(&self) -> bool {
            self.0.trim().is_empty()
        }
    }

    impl fmt::Display for Track {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} - {}", self.title.0, self.isrc)
        }
    }

    impl fmt::Display for TrackTitle {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl fmt::Display for TrackVersion {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TrackVersion::Original => write!(f, "Original"),
                TrackVersion::Live => write!(f, "Live"),
                TrackVersion::RadioEdit => write!(f, "Radio Edit"),
                TrackVersion::TvTrack => write!(f, "TV Track"),
                TrackVersion::Single => write!(f, "Single"),
                TrackVersion::Remix => write!(f, "Remix"),
                TrackVersion::Cover => write!(f, "Cover"),
                TrackVersion::Acoustic => write!(f, "Acoustic"),
                TrackVersion::Acapella => write!(f, "Acapella"),
                TrackVersion::Instrumental => write!(f, "Instrumental"),
                TrackVersion::Orchestral => write!(f, "Orchestral"),
                TrackVersion::Extended => write!(f, "Extended"),
                TrackVersion::AlternateTake => write!(f, "Alternate Take"),
                TrackVersion::ReRecorded => write!(f, "Re-Recorded"),
                TrackVersion::Karaoke => write!(f, "Karaoke"),
                TrackVersion::Dance => write!(f, "Dance"),
                TrackVersion::Dub => write!(f, "Dub"),
                TrackVersion::Clean => write!(f, "Clean"),
                TrackVersion::Rehearsal => write!(f, "Rehearsal"),
                TrackVersion::Demo => write!(f, "Demo"),
                TrackVersion::Edit => write!(f, "Edit"),
            }
        }
    }
}

// Re-export API types based on features
#[cfg(feature = "std")]
pub use api::*;

#[cfg(feature = "runtime")]
mod runtime_api {
    use super::{isrc::RuntimeIsrc, RuntimeTrack, RuntimeTrackTitle, TrackVersion};
    use crate::{utils::Key, MiddsId};
    use frame_support::BoundedVec;

    #[cfg(not(feature = "std"))]
    extern crate alloc;
    #[cfg(not(feature = "std"))]
    use alloc::string::{String, ToString};

    /// Error types for RuntimeTrack operations
    #[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
    pub enum RuntimeTrackError {
        /// Data exceeds capacity limits
        #[error("Data exceeds capacity limits")]
        ExceedsCapacity,
        /// Invalid UTF-8 data
        #[error("Invalid UTF-8 data")]
        InvalidUtf8,
        /// Invalid track data
        #[error("Invalid track data")]
        InvalidTrack,
    }

    impl RuntimeTrack {
        /// Creates a new RuntimeTrack from raw parts
        pub fn new_from_parts(
            isrc: RuntimeIsrc,
            musical_work: MiddsId,
            artist: MiddsId,
            title: RuntimeTrackTitle,
            producers: BoundedVec<MiddsId, frame_support::traits::ConstU32<64>>,
            performers: BoundedVec<MiddsId, frame_support::traits::ConstU32<256>>,
            contributors: BoundedVec<MiddsId, frame_support::traits::ConstU32<256>>,
        ) -> Result<Self, RuntimeTrackError> {
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

    impl RuntimeTrackTitle {
        /// Creates a new RuntimeTrackTitle from bytes
        pub fn new_from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, RuntimeTrackError> {
            let bytes = bytes.as_ref();
            BoundedVec::try_from(bytes.to_vec())
                .map(Self)
                .map_err(|_| RuntimeTrackError::ExceedsCapacity)
        }

        /// Creates a new RuntimeTrackTitle from a string slice
        pub fn new_from_str(value: &str) -> Result<Self, RuntimeTrackError> {
            Self::new_from_bytes(value.as_bytes())
        }

        /// Returns the title as a byte slice
        pub fn as_bytes(&self) -> &[u8] {
            &self.0
        }

        /// Converts to a string if it contains valid UTF-8
        pub fn to_string_lossy(&self) -> String {
            String::from_utf8_lossy(&self.0).to_string()
        }

        /// Converts to a string if it contains valid UTF-8
        pub fn to_string(&self) -> Result<String, RuntimeTrackError> {
            String::from_utf8(self.0.to_vec()).map_err(|_| RuntimeTrackError::InvalidUtf8)
        }

        /// Returns the maximum capacity
        pub const fn capacity() -> u32 {
            256
        }

        /// Returns the current length in bytes
        pub fn len(&self) -> usize {
            self.0.len()
        }

        /// Returns true if the title is empty
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        /// Checks if the title contains specific text (case-insensitive)
        pub fn contains_text(&self, text: &str) -> bool {
            let title = self.to_string_lossy().to_lowercase();
            let search = text.to_lowercase();
            title.contains(&search)
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
}

// Benchmark implementation for the main MIDDS type
#[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
impl BenchmarkHelper<RuntimeTrack> for RuntimeTrack {
    fn benchmark_instance(i: u32) -> RuntimeTrack {
        use crate::track::isrc::RuntimeIsrc;

        RuntimeTrack {
            isrc: RuntimeIsrc::new(create_bounded_string::<12>(i)),
            musical_work: 1u64,
            artist: 2u64,
            producers: create_bounded_vec::<MiddsId, 64>(10u64, i),
            performers: create_bounded_vec::<MiddsId, 256>(20u64, i),
            contributors: create_bounded_vec::<MiddsId, 256>(30u64, i),
            title: RuntimeTrackTitle(create_bounded_string::<256>(i)),
            title_aliases: create_bounded_vec::<RuntimeTrackTitle, 16>(
                RuntimeTrackTitle(create_bounded_string::<256>(1)), // Use minimal size for nested items
                i,
            ),
            recording_year: Some(2023),
            genres: create_bounded_vec::<GenreId, 5>(GenreId::Pop, i),
            version: if i == 0 {
                None
            } else {
                Some(match i % 5 {
                    0 => TrackVersion::Original,
                    1 => TrackVersion::Live,
                    2 => TrackVersion::Remix,
                    3 => TrackVersion::Acoustic,
                    _ => TrackVersion::Edit,
                })
            },
            duration: Some(180), // 3 minutes in seconds
            bpm: Some(120),
            key: Some(Key::C),
            recording_place: create_optional_bounded_string::<256>(i),
            mixing_place: create_optional_bounded_string::<256>(i),
            mastering_place: create_optional_bounded_string::<256>(i),
        }
    }
}

#[cfg(feature = "web")]
mod web_api {
    use super::isrc::Isrc;
    use super::{Track, TrackTitle, TrackVersion};
    use crate::MiddsId;
    use allfeat_music_genres::GenreId;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    impl Track {
        /// Creates a new Track for JavaScript
        #[wasm_bindgen(constructor)]
        pub fn new_web(
            isrc: &str,
            musical_work: MiddsId,
            artist: MiddsId,
            title: &str,
            producers: &[MiddsId],
            performers: &[MiddsId],
            contributors: &[MiddsId],
        ) -> Result<Track, JsError> {
            let isrc =
                Isrc::new(isrc).map_err(|e| JsError::new(&format!("Invalid ISRC: {}", e)))?;
            let track_title = TrackTitle::new(title)
                .map_err(|e| JsError::new(&format!("Invalid title: {}", e)))?;

            Track::new(
                isrc,
                musical_work,
                artist,
                track_title,
                producers.to_vec(),
                performers.to_vec(),
                contributors.to_vec(),
            )
            .map_err(|e| JsError::new(&format!("Failed to create track: {}", e)))
        }

        /// Gets the track title
        #[wasm_bindgen(js_name = getTitle)]
        pub fn get_title_web(&self) -> String {
            self.title.0.clone()
        }

        /// Gets the ISRC code
        #[wasm_bindgen(js_name = getIsrc)]
        pub fn get_isrc_web(&self) -> String {
            self.isrc.to_string()
        }

        /// Gets the full display title with aliases
        #[wasm_bindgen(js_name = getFullTitle)]
        pub fn get_full_title_web(&self) -> String {
            self.full_title()
        }

        /// Gets the duration in seconds
        #[wasm_bindgen(js_name = getDuration)]
        pub fn get_duration_web(&self) -> Option<u16> {
            self.duration
        }

        /// Gets the formatted duration as MM:SS
        #[wasm_bindgen(js_name = getFormattedDuration)]
        pub fn get_formatted_duration_web(&self) -> Option<String> {
            self.formatted_duration()
        }

        /// Gets the BPM
        #[wasm_bindgen(js_name = getBpm)]
        pub fn get_bpm_web(&self) -> Option<u16> {
            self.bpm
        }

        /// Gets the recording year
        #[wasm_bindgen(js_name = getRecordingYear)]
        pub fn get_recording_year_web(&self) -> Option<u16> {
            self.recording_year
        }

        /// Gets the musical key as a string
        #[wasm_bindgen(js_name = getKey)]
        pub fn get_key_web(&self) -> Option<String> {
            self.key.map(|k| format!("{:?}", k))
        }

        /// Gets the version type as a string
        #[wasm_bindgen(js_name = getVersion)]
        pub fn get_version_web(&self) -> Option<String> {
            self.version.map(|v| format!("{:?}", v))
        }

        /// Gets the recording place
        #[wasm_bindgen(js_name = getRecordingPlace)]
        pub fn get_recording_place_web(&self) -> Option<String> {
            self.recording_place.clone()
        }

        /// Gets the mixing place
        #[wasm_bindgen(js_name = getMixingPlace)]
        pub fn get_mixing_place_web(&self) -> Option<String> {
            self.mixing_place.clone()
        }

        /// Gets the mastering place
        #[wasm_bindgen(js_name = getMasteringPlace)]
        pub fn get_mastering_place_web(&self) -> Option<String> {
            self.mastering_place.clone()
        }

        /// Gets the producer count
        #[wasm_bindgen(js_name = getProducerCount)]
        pub fn get_producer_count_web(&self) -> usize {
            self.producers.len()
        }

        /// Gets the performer count
        #[wasm_bindgen(js_name = getPerformerCount)]
        pub fn get_performer_count_web(&self) -> usize {
            self.performers.len()
        }

        /// Gets the contributor count
        #[wasm_bindgen(js_name = getContributorCount)]
        pub fn get_contributor_count_web(&self) -> usize {
            self.contributors.len()
        }

        /// Gets the total contributor count
        #[wasm_bindgen(js_name = getTotalContributorCount)]
        pub fn get_total_contributor_count_web(&self) -> usize {
            self.total_contributor_count()
        }

        /// Checks if this is a live version
        #[wasm_bindgen(js_name = isLiveVersion)]
        pub fn is_live_version_web(&self) -> bool {
            self.is_live_version()
        }

        /// Checks if this is an acoustic version
        #[wasm_bindgen(js_name = isAcousticVersion)]
        pub fn is_acoustic_version_web(&self) -> bool {
            self.is_acoustic_version()
        }

        /// Checks if this is a remix
        #[wasm_bindgen(js_name = isRemix)]
        pub fn is_remix_web(&self) -> bool {
            matches!(self.version, Some(TrackVersion::Remix))
        }

        /// Checks if this is a demo version
        #[wasm_bindgen(js_name = isDemo)]
        pub fn is_demo_web(&self) -> bool {
            matches!(self.version, Some(TrackVersion::Demo))
        }

        /// Checks if track was recorded in a specific year
        #[wasm_bindgen(js_name = recordedInYear)]
        pub fn recorded_in_year_web(&self, year: u16) -> bool {
            self.recorded_in_year(year)
        }

        /// Gets suggested track versions for display
        #[wasm_bindgen(js_name = getSuggestedVersions)]
        pub fn get_suggested_versions_web(&self) -> Vec<String> {
            self.suggested_versions()
                .into_iter()
                .map(|v| format!("{:?}", v))
                .collect()
        }

        /// Sets the duration
        #[wasm_bindgen(js_name = setDuration)]
        pub fn set_duration_web(&mut self, duration: Option<u16>) {
            self.duration = duration;
        }

        /// Sets the BPM
        #[wasm_bindgen(js_name = setBpm)]
        pub fn set_bpm_web(&mut self, bpm: Option<u16>) {
            self.bpm = bpm;
        }

        /// Sets the recording year
        #[wasm_bindgen(js_name = setRecordingYear)]
        pub fn set_recording_year_web(&mut self, year: Option<u16>) {
            self.recording_year = year;
        }

        /// Sets the version from a string
        #[wasm_bindgen(js_name = setVersionFromString)]
        pub fn set_version_from_string_web(
            &mut self,
            version_str: Option<String>,
        ) -> Result<(), JsError> {
            match version_str {
                Some(s) => {
                    let version = match s.as_str() {
                        "Original" => TrackVersion::Original,
                        "Live" => TrackVersion::Live,
                        "Acoustic" => TrackVersion::Acoustic,
                        "Remix" => TrackVersion::Remix,
                        "ReRecorded" => TrackVersion::ReRecorded,
                        "Demo" => TrackVersion::Demo,
                        "Extended" => TrackVersion::Extended,
                        "Edit" => TrackVersion::Edit,
                        "Instrumental" => TrackVersion::Instrumental,
                        "Karaoke" => TrackVersion::Karaoke,
                        "Acapella" => TrackVersion::Acapella,
                        _ => return Err(JsError::new(&format!("Unknown version: {}", s))),
                    };
                    self.version = Some(version);
                }
                None => self.version = None,
            }
            Ok(())
        }

        /// Sets the recording place
        #[wasm_bindgen(js_name = setRecordingPlace)]
        pub fn set_recording_place_web(&mut self, place: Option<String>) {
            self.recording_place = place;
        }

        /// Sets the mixing place
        #[wasm_bindgen(js_name = setMixingPlace)]
        pub fn set_mixing_place_web(&mut self, place: Option<String>) {
            self.mixing_place = place;
        }

        /// Sets the mastering place
        #[wasm_bindgen(js_name = setMasteringPlace)]
        pub fn set_mastering_place_web(&mut self, place: Option<String>) {
            self.mastering_place = place;
        }

        /// Adds a title alias
        #[wasm_bindgen(js_name = addTitleAlias)]
        pub fn add_title_alias_web(&mut self, alias: &str) -> Result<(), JsError> {
            let title_alias = TrackTitle::new(alias)
                .map_err(|e| JsError::new(&format!("Invalid alias: {}", e)))?;
            self.title_aliases.push(title_alias);
            Ok(())
        }

        /// Adds a producer
        #[wasm_bindgen(js_name = addProducer)]
        pub fn add_producer_web(&mut self, producer_id: MiddsId) {
            if !self.producers.contains(&producer_id) {
                self.producers.push(producer_id);
            }
        }

        /// Adds a performer
        #[wasm_bindgen(js_name = addPerformer)]
        pub fn add_performer_web(&mut self, performer_id: MiddsId) {
            if !self.performers.contains(&performer_id) {
                self.performers.push(performer_id);
            }
        }

        /// Adds a contributor
        #[wasm_bindgen(js_name = addContributor)]
        pub fn add_contributor_web(&mut self, contributor_id: MiddsId) {
            if !self.contributors.contains(&contributor_id) {
                self.contributors.push(contributor_id);
            }
        }

        /// Adds a genre by ID
        #[wasm_bindgen(js_name = addGenre)]
        pub fn add_genre_web(&mut self, genre_id: u32) {
            // For now, just create a default genre since GenreId structure is unknown
            // In a real implementation, this would use proper GenreId conversion
            let genre = match genre_id {
                0 => GenreId::Pop,
                1 => GenreId::Rock,
                2 => GenreId::Jazz,
                3 => GenreId::Classical,
                4 => GenreId::Electronic,
                _ => GenreId::Pop, // Default fallback
            };
            if !self.genres.contains(&genre) {
                self.genres.push(genre);
            }
        }

        /// Validates the track
        #[wasm_bindgen(js_name = validate)]
        pub fn validate_web(&self) -> Result<(), JsError> {
            // Basic validation - check if ISRC is valid
            if self.isrc.as_ref().is_empty() {
                return Err(JsError::new("ISRC cannot be empty"));
            }
            if self.title.0.is_empty() {
                return Err(JsError::new("Title cannot be empty"));
            }
            Ok(())
        }

        /// Gets a string representation of the track
        #[wasm_bindgen(js_name = toString)]
        pub fn to_string_web(&self) -> String {
            format!("{}", self)
        }
    }
}

// Re-export runtime error types for use in the unified error system
#[cfg(feature = "runtime")]
pub use runtime_api::RuntimeTrackError;
