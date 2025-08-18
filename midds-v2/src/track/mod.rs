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
//!     track::{Track, TrackVersion, isrc::Isrc},
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
//!     title: "Bohemian Rhapsody (Remastered)".to_string(),
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

pub mod error;
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
/// - `title_aliases: Vec<String>` → `title_aliases: BoundedVec<BoundedVec<u8, ConstU32<256>>, ConstU32<16>>`
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
///     title: "Hotel California".to_string(),
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
///     title: "Hotel California (Live at MTV Unplugged)".to_string(),
///     title_aliases: vec![
///         "Hotel California - Unplugged".to_string()
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
    #[runtime_bound(256)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub title: String,

    /// Optional list of alternative titles for the track.
    #[runtime_bound(16, 256)]
    #[as_runtime_type]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub title_aliases: Vec<String>,

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
pub mod api;

#[cfg(feature = "runtime")]
pub mod runtime_api;

// Benchmark implementation for the main MIDDS type
#[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
impl BenchmarkHelper<RuntimeTrack> for RuntimeTrack {
    fn benchmark_instance(i: u32) -> RuntimeTrack {
        use crate::track::isrc::RuntimeIsrc;
        use frame_support::{traits::ConstU32, BoundedVec};

        RuntimeTrack {
            isrc: RuntimeIsrc::new(create_bounded_string::<12>(i)),
            musical_work: 1u64,
            artist: 2u64,
            producers: create_bounded_vec::<MiddsId, 64>(10u64, i),
            performers: create_bounded_vec::<MiddsId, 256>(20u64, i),
            contributors: create_bounded_vec::<MiddsId, 256>(30u64, i),
            title: create_bounded_string::<256>(i),
            title_aliases: create_bounded_vec::<BoundedVec<u8, ConstU32<256>>, 16>(
                create_bounded_string::<256>(1), // Use minimal size for nested items
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
pub mod web_api;
