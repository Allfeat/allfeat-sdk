//! Recording types and audio recording metadata.
//!
//! This module contains types for representing music recordings, including
//! performance metadata, production details, and industry identifiers.

use crate::shared::genres::GenreId;

use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

use crate::{
    shared::Key,
    shared::{Bpm, PartyId, Year},
    MiddsId, MiddsString, MiddsVec,
};

#[cfg(feature = "std")]
use ts_rs::TS;

#[cfg(feature = "std")]
const TS_DIR: &str = "recording/";

/// Duration type in seconds.
///
/// Used to represent the length of audio recordings.
///
/// # Example
///
/// ```rust
/// use allfeat_midds_v2::recording::Duration;
///
/// let duration: Duration = 180; // 3 minutes
/// ```
pub type Duration = u16;

/// International Standard Recording Code (ISRC) identifier.
///
/// ISRC is used to uniquely identify sound recordings and music videos.
/// ISRC codes are 12 characters long.
///
/// # Format
///
/// ISRC codes follow the pattern: CCXXXYYNNNNN where:
/// - CC = Country code (2 letters)
/// - XXX = Registrant code (3 alphanumeric)
/// - YY = Year (2 digits)
/// - NNNNN = Designation code (5 digits)
///
/// # Example
///
/// ```rust
/// use allfeat_midds_v2::recording::Isrc;
///
/// let isrc: Isrc = b"USABC2312345".to_vec().try_into().unwrap();
/// ```
pub type Isrc = MiddsString<12>;

/// Represents a music recording.
///
/// This structure contains all metadata related to a recorded performance,
/// including production details, performers, and technical specifications.
///
/// # Examples
///
/// ## Basic Recording
///
/// ```rust
/// use allfeat_midds_v2::{
///     recording::{Recording, RecordingVersion},
///     shared::PartyId,
///     shared::Key,
///     shared::genres::GenreId
/// };
///
/// let recording = Recording {
///     isrc: b"USABC2312345".to_vec().try_into().unwrap(),
///     musical_work: 12345,
///     artist: PartyId::Ipi(123456789),
///     producers: vec![].try_into().unwrap(),
///     performers: vec![].try_into().unwrap(),
///     contributors: vec![].try_into().unwrap(),
///     title: b"My Recording".to_vec().try_into().unwrap(),
///     title_aliases: vec![].try_into().unwrap(),
///     recording_year: Some(2024),
///     genres: vec![GenreId::Pop].try_into().unwrap(),
///     version: Some(RecordingVersion::Original),
///     duration: Some(180),
///     bpm: Some(120),
///     key: Some(Key::C),
///     recording_place: None,
///     mixing_place: None,
///     mastering_place: None,
/// };
/// ```
#[derive(
    Debug, Clone, PartialEq, Eq, Encode, Decode, DecodeWithMemTracking, TypeInfo, MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(TS), ts(export, export_to = TS_DIR, optional_fields, rename_all = "camelCase"))]
pub struct Recording {
    /// ISRC (International Standard Recording Code) that uniquely identifies this recording.
    #[cfg_attr(feature = "std", ts(as = "String"))]
    pub isrc: Isrc,

    /// The linked musical work this recording is based on (must refer to a registered MIDDS).
    pub musical_work: MiddsId,

    pub artist: PartyId,

    #[cfg_attr(feature = "std", ts(as = "Vec<PartyId>"))]
    pub producers: MiddsVec<PartyId, 64>,

    #[cfg_attr(feature = "std", ts(as = "Vec<PartyId>"))]
    pub performers: MiddsVec<PartyId, 256>,

    #[cfg_attr(feature = "std", ts(as = "Vec<PartyId>"))]
    pub contributors: MiddsVec<PartyId, 256>,

    /// Main title of the recording.
    #[cfg_attr(feature = "std", ts(as = "String"))]
    pub title: MiddsString<256>,

    /// Optional list of alternative titles for the recording.
    #[cfg_attr(feature = "std", ts(as = "Vec<String>"))]
    pub title_aliases: MiddsVec<MiddsString<256>, 16>,

    /// Year the recording was made (4-digit Gregorian year).
    pub recording_year: Option<Year>,

    /// Music genres attributed to this recording.
    #[cfg_attr(feature = "std", ts(as = "Vec<GenreId>"))]
    pub genres: MiddsVec<GenreId, 5>,

    /// Version or type of the recording (e.g., Remix, Acoustic, Live).
    pub version: Option<RecordingVersion>,

    /// Duration of the recording in seconds.
    pub duration: Option<Duration>,

    /// Beats per minute (BPM), representing the tempo of the recording.
    pub bpm: Option<Bpm>,

    /// Musical key (e.g., C, G#, etc.) the recording is in.
    pub key: Option<Key>,

    /// Free-text field indicating where the recording took place.
    #[cfg_attr(feature = "std", ts(as = "String"))]
    pub recording_place: Option<MiddsString<256>>,

    /// Free-text field indicating where the mixing of the recording occurred.
    #[cfg_attr(feature = "std", ts(as = "String"))]
    pub mixing_place: Option<MiddsString<256>>,

    /// Free-text field indicating where the mastering of the recording occurred.
    #[cfg_attr(feature = "std", ts(as = "String"))]
    pub mastering_place: Option<MiddsString<256>>,
}

#[repr(u8)]
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Encode,
    Decode,
    DecodeWithMemTracking,
    TypeInfo,
    MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(TS), ts(export, export_to = TS_DIR))]
pub enum RecordingVersion {
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
