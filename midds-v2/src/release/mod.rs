//! Release types and distribution metadata.
//!
//! This module contains types for representing music releases such as albums,
//! EPs, singles, and their associated distribution and packaging metadata.

use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

use crate::{
    shared::PartyId,
    shared::{Country, Date},
    MiddsId, MiddsString, MiddsVec,
};

#[cfg(feature = "std")]
use ts_rs::TS;

#[cfg(feature = "std")]
const TS_DIR: &str = "release/";

/// European Article Number (EAN) or Universal Product Code (UPC) identifier.
///
/// Used to uniquely identify commercial releases in retail and digital distribution.
/// EAN/UPC codes are typically 13 digits for international use.
///
/// # Example
///
/// ```rust
/// use allfeat_midds_v2::release::Ean;
///
/// let ean: Ean = b"1234567890123".to_vec().try_into().unwrap();
/// ```
pub type Ean = MiddsString<13>;

/// Represents a commercial music release.
///
/// This structure contains all metadata related to the distribution and packaging
/// of musical content, including track listings, production details, and commercial information.
///
/// # Examples
///
/// ## Album Release
///
/// ```rust
/// use allfeat_midds_v2::{
///     release::{Release, ReleaseType, ReleaseFormat, ReleasePackaging, ReleaseStatus},
///     shared::PartyId,
///     shared::{Date, Country},
/// };
///
/// let album = Release {
///     ean_upc: b"1234567890123".to_vec().try_into().unwrap(),
///     creator: PartyId::Ipi(12345),
///     producers: vec![].try_into().unwrap(),
///     recordings: vec![].try_into().unwrap(),
///     distributor_name: b"Music Distributor Inc".to_vec().try_into().unwrap(),
///     manufacturer_name: b"Vinyl Press Co".to_vec().try_into().unwrap(),
///     cover_contributors: vec![].try_into().unwrap(),
///     title: b"My Album".to_vec().try_into().unwrap(),
///     title_aliases: vec![].try_into().unwrap(),
///     release_type: ReleaseType::Lp,
///     format: ReleaseFormat::Cd,
///     packaging: ReleasePackaging::JewelCase,
///     date: Date { year: 2024, month: 6, day: 15 },
///     country: Country::US,
///     status: ReleaseStatus::Official,
/// };
/// ```
///
/// ## Single Release
///
/// ```rust
/// use allfeat_midds_v2::{
///     release::{Release, ReleaseType, ReleaseFormat, ReleasePackaging, ReleaseStatus},
///     shared::PartyId,
///     shared::{Date, Country},
/// };
///
/// let single = Release {
///     ean_upc: b"9876543210987".to_vec().try_into().unwrap(),
///     creator: PartyId::Ipi(67890),
///     producers: vec![PartyId::Ipi(111111111)].try_into().unwrap(),
///     recordings: vec![222222222].try_into().unwrap(),
///     distributor_name: b"Digital Distributor".to_vec().try_into().unwrap(),
///     manufacturer_name: b"Digital".to_vec().try_into().unwrap(),
///     cover_contributors: vec![b"Cover Artist".to_vec().try_into().unwrap()].try_into().unwrap(),
///     title: b"Hit Single".to_vec().try_into().unwrap(),
///     title_aliases: vec![].try_into().unwrap(),
///     release_type: ReleaseType::Single,
///     format: ReleaseFormat::Cd,
///     packaging: ReleasePackaging::Digipack,
///     date: Date { year: 2024, month: 3, day: 1 },
///     country: Country::GB,
///     status: ReleaseStatus::Official,
/// };
/// ```
#[derive(
    Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, DecodeWithMemTracking, TypeInfo,
)]
#[cfg_attr(feature = "std", derive(TS), ts(export, export_to = TS_DIR, optional_fields, rename_all = "camelCase"))]
pub struct Release {
    /// EAN or UPC code identifying the release (physical or digital).
    #[cfg_attr(feature = "std", ts(as = "String"))]
    pub ean_upc: Ean,

    /// The main creator IDs associated with this release.
    pub creator: PartyId,

    /// List of producer MIDDS IDs who contributed to this release.
    #[cfg_attr(feature = "std", ts(as = "Vec<ProducerInfo>"))]
    pub producers: MiddsVec<ProducerInfo, 256>,

    /// List of track MIDDS IDs that are part of this release.
    #[cfg_attr(feature = "std", ts(as = "Vec<MiddsId>"))]
    pub recordings: MiddsVec<MiddsId, 1024>,

    /// Name of the distributor responsible for the release.
    #[cfg_attr(feature = "std", ts(as = "String"))]
    pub distributor_name: MiddsString<256>,

    /// Name of the manufacturer responsible for physical production.
    #[cfg_attr(feature = "std", ts(as = "String"))]
    pub manufacturer_name: MiddsString<256>,

    /// Contributors to the release cover (designers, photographers, etc.).
    #[cfg_attr(feature = "std", ts(as = "Vec<String>"))]
    pub cover_contributors: MiddsVec<MiddsString<256>, 64>,

    /// Official title of the release.
    #[cfg_attr(feature = "std", ts(as = "String"))]
    pub title: MiddsString<256>,

    /// Alternative titles (e.g. translations, acronyms, stylistic variations).
    #[cfg_attr(feature = "std", ts(as = "Vec<String>"))]
    pub title_aliases: MiddsVec<MiddsString<256>, 16>,

    /// Type of the release (e.g. LP, EP, Single, Mixtape).
    pub release_type: ReleaseType,

    /// Format of the release medium (e.g. CD, Vinyl, Cassette).
    pub format: ReleaseFormat,

    /// Packaging used for the physical release (e.g. Digipack, Jewel Case).
    pub packaging: ReleasePackaging,

    /// Official status of the release (e.g. Official, Promotional, Remastered).
    pub status: ReleaseStatus,

    /// Release date.
    pub date: Date,

    /// Country where the release was published or made available.
    pub country: Country,
}

/// The general type of release based on track count or intent.
#[repr(u8)]
#[derive(
    Clone,
    Debug,
    Copy,
    PartialEq,
    Eq,
    Encode,
    Decode,
    MaxEncodedLen,
    DecodeWithMemTracking,
    TypeInfo,
)]
#[cfg_attr(feature = "std", derive(TS), ts(export, export_to = TS_DIR))]
pub enum ReleaseType {
    /// Long Play album (usually 8+ recordings).
    Lp = 0,
    /// Double album (2 discs or extensive track list).
    DoubleLp = 1,
    /// Extended Play (typically 4–6 recordings).
    Ep = 2,
    /// A standalone track or 2-track release.
    Single = 3,
    /// Informal or promotional compilation, often non-commercial.
    Mixtape = 4,
    Compilation = 5,
}

/// The format of the physical or digital medium used for distribution.
#[repr(u8)]
#[derive(
    Clone,
    Debug,
    Copy,
    PartialEq,
    Eq,
    Encode,
    Decode,
    MaxEncodedLen,
    DecodeWithMemTracking,
    TypeInfo,
)]
#[cfg_attr(feature = "std", derive(TS), ts(export, export_to = TS_DIR))]
pub enum ReleaseFormat {
    /// Compact Disc.
    Cd = 0,
    /// Double Compact Disc.
    DoubleCd = 1,
    /// 7-inch vinyl record.
    Vynil7 = 2,
    /// 10-inch vinyl record.
    Vinyl10 = 3,
    /// Audio cassette.
    Cassette = 4,
    /// Digital Versatile Disc containing audio.
    AudioDvd = 5,
}

/// The packaging type used for the physical release.
#[repr(u8)]
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
pub enum ReleasePackaging {
    /// Fold-out cardboard packaging.
    Digipack = 0,
    /// Standard plastic CD case.
    JewelCase = 1,
    /// Thin, plastic alternative packaging.
    SnapCase = 2,
}

/// The official status of the release in its publication lifecycle.
#[repr(u8)]
#[derive(
    Clone,
    Debug,
    Copy,
    PartialEq,
    Eq,
    Encode,
    Decode,
    MaxEncodedLen,
    DecodeWithMemTracking,
    TypeInfo,
)]
#[cfg_attr(feature = "std", derive(TS), ts(export, export_to = TS_DIR))]
pub enum ReleaseStatus {
    /// Properly released by the creator or label.
    Official = 0,
    /// Used for marketing or sent to press/radio.
    Promotional = 1,
    /// Reissued at a later date (possibly remastered).
    ReRelease = 2,
    /// Includes bonus content or packaging.
    SpecialEdition = 3,
    /// Improved audio version of an earlier release.
    Remastered = 4,
    /// Unofficial or unauthorized release.
    Bootleg = 5,
    /// Placeholder or unverified metadata.
    PseudoRelease = 6,
    /// Removed shortly after being released.
    Withdrawn = 7,
    /// Intentionally removed from catalog/history.
    Expunged = 8,
    /// Planned but never released.
    Cancelled = 9,
}

#[derive(
    Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, DecodeWithMemTracking, TypeInfo,
)]
#[cfg_attr(feature = "std", derive(TS), ts(export, export_to = TS_DIR, optional_fields, rename_all = "camelCase"))]
pub struct ProducerInfo {
    producer_id: PartyId,
    #[cfg_attr(feature = "std", ts(as = "String"))]
    catalog_nb: Option<MiddsString<32>>,
}
