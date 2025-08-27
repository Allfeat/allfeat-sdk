//! Musical work types and related structures.
//!
//! This module contains types for representing musical compositions, including
//! songwriting metadata, creator information, and classical work details.

use crate::{
    shared::PartyId,
    shared::{Key, Language},
    MiddsId, MiddsString, MiddsVec,
};
use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

#[cfg(feature = "std")]
use ts_rs::TS;

#[cfg(feature = "std")]
const TS_DIR: &str = "musical_work/";

/// International Standard Musical Work Code (ISWC) identifier.
///
/// ISWC is used to uniquely identify musical works (compositions) across
/// the global music industry. ISWC codes are 11 characters long.
///
/// # Format
///
/// ISWC codes follow the pattern: T-XXXXXXXXX-C where:
/// - T = literal 'T'
/// - X = 9 digits
/// - C = check digit
///
/// # Example
///
/// ```rust
/// use allfeat_midds_v2::musical_work::Iswc;
///
/// let iswc: Iswc = b"T1234567890".to_vec().try_into().unwrap();
/// ```
pub type Iswc = MiddsString<11>;

/// Represents a musical composition or songwriting work.
///
/// This structure contains all metadata related to the creation and composition
/// of a musical work, including creator information, musical characteristics,
/// and industry identifiers.
///
/// # Examples
///
/// ## Simple Song
///
/// ```rust
/// use allfeat_midds_v2::{
///     musical_work::{MusicalWork, Creator, CreatorRole},
///     shared::PartyId,
///     shared::{Language, Key},
/// };
///
/// let song = MusicalWork {
///     iswc: b"T1234567890".to_vec().try_into().unwrap(),
///     title: b"My Song".to_vec().try_into().unwrap(),
///     creation_year: Some(2024),
///     instrumental: Some(false),
///     language: Some(Language::English),
///     bpm: Some(120),
///     key: Some(Key::C),
///     work_type: None,
///     creators: vec![Creator {
///         id: PartyId::Ipi(123456789),
///         role: CreatorRole::Composer,
///     }].try_into().unwrap(),
///     classical_info: None,
/// };
/// ```
///
/// ## Collaborative Work
///
/// ```rust
/// use allfeat_midds_v2::{
///     musical_work::{MusicalWork, Creator, CreatorRole},
///     shared::PartyId,
///     shared::Language,
/// };
///
/// let collaborative_work = MusicalWork {
///     iswc: b"T9876543210".to_vec().try_into().unwrap(),
///     title: b"Collaborative Song".to_vec().try_into().unwrap(),
///     creation_year: Some(2024),
///     instrumental: Some(false),
///     language: Some(Language::English),
///     bpm: None,
///     key: None,
///     work_type: None,
///     creators: vec![
///         Creator {
///             id: PartyId::Ipi(111111111),
///             role: CreatorRole::Author,
///         },
///         Creator {
///             id: PartyId::Ipi(222222222),
///             role: CreatorRole::Composer,
///         },
///     ].try_into().unwrap(),
///     classical_info: None,
/// };
/// ```
#[derive(
    Debug, Clone, PartialEq, Eq, Encode, Decode, DecodeWithMemTracking, TypeInfo, MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(TS), ts(export, export_to = TS_DIR, optional_fields, rename_all = "camelCase"))]
pub struct MusicalWork {
    /// The ISWC (International Standard Musical Work Code) uniquely identifying the work.
    #[cfg_attr(feature = "std", ts(as = "String"))]
    pub iswc: Iswc,

    /// The title of the musical work.
    #[cfg_attr(feature = "std", ts(as = "String"))]
    pub title: MiddsString<256>,

    /// The year the work was created (4-digit Gregorian year).
    pub creation_year: Option<u16>,

    /// Indicates whether the work is instrumental (i.e., without lyrics).
    pub instrumental: Option<bool>,

    /// The optional language of the lyrics (if any).
    pub language: Option<Language>,

    /// Optional tempo in beats per minute (BPM).
    pub bpm: Option<u16>,

    /// Optional musical key of the work (e.g., C, G#, etc.).
    pub key: Option<Key>,

    /// Type of the musical work (original, medley, mashup, or adaptation).
    pub work_type: Option<MusicalWorkType>,

    /// List of contributors to the work, along with their roles.
    #[cfg_attr(feature = "std", ts(as = "Vec<Creator>"))]
    pub creators: MiddsVec<Creator, 256>,

    /// Additional info if the work is a classical one.
    pub classical_info: Option<ClassicalInfo>,
}

#[derive(
    Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, DecodeWithMemTracking, TypeInfo,
)]
#[cfg_attr(feature = "std", derive(TS), ts(export, export_to = TS_DIR))]
pub enum MusicalWorkType {
    /// A standalone, original composition with no derivation from existing works.
    Original,

    /// A combination of multiple existing works arranged in sequence.
    ///
    /// Medleys typically present existing works in their recognizable form
    /// but arranged to flow together as a cohesive performance.
    #[cfg_attr(feature = "std", ts(as = "Vec<MiddsId>"))]
    Medley(MiddsVec<MiddsId, 512>),

    /// A creative blend mixing elements from multiple existing works.
    ///
    /// Mashups typically combine melodic, harmonic, or rhythmic elements
    /// from different works to create something new while maintaining
    /// recognizable elements from the source material.
    #[cfg_attr(feature = "std", ts(as = "Vec<MiddsId>"))]
    Mashup(MiddsVec<MiddsId, 512>),

    /// A modified version of a single existing work.
    ///
    /// Adaptations include arrangements, translations, or other modifications
    /// that create a derivative work from a single source.
    Adaptation(MiddsId),
}

/// Represents a creator or contributor to a musical work.
///
/// This structure links a party (identified by their industry IDs) to their
/// specific role in the creation of a musical work.
///
/// # Example
///
/// ```rust
/// use allfeat_midds_v2::{
///     musical_work::{Creator, CreatorRole},
///     shared::PartyId,
/// };
///
/// let composer = Creator {
///     id: PartyId::Ipi(123456789),
///     role: CreatorRole::Composer,
/// };
///
/// let lyricist = Creator {
///     id: PartyId::Ipi(987654321),
///     role: CreatorRole::Author,
/// };
/// ```
#[derive(
    Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, DecodeWithMemTracking, TypeInfo,
)]
#[cfg_attr(feature = "std", derive(TS), ts(export, export_to = TS_DIR))]
pub struct Creator {
    /// Identifier of the person or entity involved in the work.
    pub id: PartyId,
    /// The specific role this creator played in the creation of the work.
    pub role: CreatorRole,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Encode,
    Decode,
    MaxEncodedLen,
    DecodeWithMemTracking,
    TypeInfo,
)]
#[cfg_attr(feature = "std", derive(TS), ts(export, export_to = TS_DIR))]
pub enum CreatorRole {
    /// Original author of the lyrics or libretto.
    ///
    /// The person who wrote the words/text that accompany the musical composition.
    Author,

    /// Composer of the musical elements.
    ///
    /// The person who created the melody, harmony, rhythm, and overall musical structure.
    Composer,

    /// Arranger of the musical work.
    ///
    /// The person who created arrangements, orchestrations, or adaptations of the
    /// original composition for different instruments or ensembles.
    Arranger,

    /// Adapter of music or lyrics from original sources.
    ///
    /// The person who modified, translated, or adapted existing musical or lyrical
    /// content to create a derivative work.
    Adapter,

    /// Publisher responsible for commercial and administrative aspects.
    ///
    /// The entity (person or company) who handles publication, distribution,
    /// rights management, and other non-creative business aspects.
    Publisher,
}

#[derive(
    Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, DecodeWithMemTracking, TypeInfo,
)]
#[cfg_attr(feature = "std", derive(TS), ts(export, export_to = TS_DIR, optional_fields, rename_all = "camelCase"))]
pub struct ClassicalInfo {
    /// Opus number assigned by the composer or music cataloger.
    ///
    /// Opus numbers are sequential numbers assigned to works, often by the composer,
    /// to indicate order of composition or publication. Format examples:
    /// - "Op. 27 No. 2" (Beethoven's Moonlight Sonata)
    /// - "Op. 9" (simple opus number)
    /// - "Op. posthumous" (published after death)
    #[cfg_attr(feature = "std", ts(as = "Option<String>"))]
    pub opus: Option<MiddsString<256>>,

    /// Catalog number from a scholarly music catalog.
    ///
    /// Professional musicologists create comprehensive catalogs of composers' works.
    /// Examples include:
    /// - "K. 551" (Mozart's Jupiter Symphony in Köchel catalog)
    /// - "BWV 1006" (Bach work in Bach-Werke-Verzeichnis)
    /// - "D. 944" (Schubert work in Deutsch catalog)
    /// - "Hob. XVI:50" (Haydn work in Hoboken catalog)
    #[cfg_attr(feature = "std", ts(as = "Option<String>"))]
    pub catalog_number: Option<MiddsString<256>>,

    /// Number of distinct vocal parts in the composition.
    ///
    /// For works with vocal components, this indicates how many separate
    /// vocal lines exist:
    /// - 1 = Solo voice
    /// - 4 = SATB choir (Soprano, Alto, Tenor, Bass)
    /// - 8 = Double choir
    /// - None = Instrumental work with no vocal parts
    pub number_of_voices: Option<u16>,
}
