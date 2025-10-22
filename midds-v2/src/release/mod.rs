//! Release types and distribution metadata.
//!
//! This module contains types for representing music releases such as albums,
//! EPs, singles, and their associated distribution and packaging metadata.

use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

use crate::{
    MiddsId, MiddsString, MiddsVec,
    shared::PartyId,
    shared::{Country, Date},
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
///     packaging: ReleasePackaging::Digipak,
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

    /// Packaging used for the physical release (e.g. Digipak, Jewel Case).
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
    // CDs and variants
    /// Compact Disc.
    Cd = 0,
    /// Double Compact Disc (2× CD).
    DoubleCd = 1,
    /// Recordable CD (CD-R).
    Cdr = 2,
    /// Enhanced CD (CD-Extra/CD-Plus with data session).
    EnhancedCd = 3,
    /// CD+G (Audio CD with graphics, e.g. karaoke).
    CdG = 4,
    /// High Definition Compatible Digital (HDCD).
    Hdcd = 5,
    /// Super High Material CD (SHM-CD).
    ShmCd = 6,
    /// Blu-spec CD.
    BluSpecCd = 7,
    /// Mixed Mode CD (audio + data tracks).
    MixedModeCd = 8,
    /// Minimax CD (undersized data area, transparent outer ring).
    MinimaxCd = 9,
    /// 8 cm (3-inch) CD.
    EightCmCd = 10,
    /// Copy Control CD (copy-protected CD variant).
    CopyControlCd = 11,

    // Vinyl and related
    /// Generic vinyl record (size unspecified).
    Vinyl = 12,
    /// 7-inch vinyl record.
    Vinyl7 = 13,
    /// 10-inch vinyl record.
    Vinyl10 = 14,
    /// 12-inch vinyl record.
    Vinyl12 = 15,
    /// Flexible vinyl (flexi-disc/phonosheet).
    FlexiDisc = 16,
    /// Quadraphonic vinyl.
    QuadVinyl = 17,

    // Digital
    /// Born-digital media (any audio file format).
    DigitalMedia = 18,
    /// Physical download card with redemption code.
    DownloadCard = 19,

    // Tapes and magnetic
    /// Compact audio cassette.
    Cassette = 20,
    /// Microcassette.
    Microcassette = 21,
    /// 4-track cartridge.
    Cartridge4Track = 22,
    /// 8-track cartridge.
    Cartridge8Track = 23,
    /// Quad 8-track cartridge.
    Quad8Track = 24,
    /// MiniDisc (ATRAC-based magneto-optical).
    MiniDisc = 25,
    /// Digital Audio Tape (DAT).
    Dat = 26,
    /// Digital Compact Cassette (DCC).
    Dcc = 27,
    /// Reel-to-reel tape.
    ReelToReel = 28,
    /// Wire recording (early magnetic recording).
    WireRecording = 29,

    // DVD / Blu-ray and derivatives
    /// DVD-Audio disc.
    DvdAudio = 30,
    /// DVD-Video disc.
    DvdVideo = 31,
    /// DualDisc (CD side + DVD side).
    DualDisc = 32,
    /// DVDplus (CD layer + DVD layer).
    DvdPlus = 33,
    /// Blu-ray Disc.
    BluRay = 34,
    /// Recordable Blu-ray Disc (BD-R).
    BluRayR = 35,
    /// HD DVD disc.
    HdDvd = 36,

    // Video/optical
    /// Video CD (VCD).
    Vcd = 37,
    /// Super Video CD (SVCD).
    Svcd = 38,
    /// CD Video (CDV).
    Cdv = 39,
    /// LaserDisc (analog video optical disc).
    LaserDisc = 40,
    /// Universal Media Disc (UMD) for PSP.
    Umd = 41,

    // Historical discs
    /// Shellac disc, 7-inch.
    Shellac7 = 42,
    /// Shellac disc, 10-inch.
    Shellac10 = 43,
    /// Shellac disc, 12-inch.
    Shellac12 = 44,
    /// Acetate (lacquer) disc, 7-inch.
    Acetate7 = 45,
    /// Acetate (lacquer) disc, 10-inch.
    Acetate10 = 46,
    /// Acetate (lacquer) disc, 12-inch.
    Acetate12 = 47,
    /// Edison Diamond Disc.
    EdisonDiamondDisc = 48,
    /// Pathé vertical-cut disc.
    PatheDisc = 49,
    /// Player piano roll.
    PianoRoll = 50,
    /// Wax cylinder.
    WaxCylinder = 51,

    // Other media
    /// USB flash drive.
    UsbFlashDrive = 52,
    /// SD card.
    SdCard = 53,
    /// 3.5-inch floppy disk.
    Floppy35 = 54,
    /// 5.25-inch floppy disk.
    Floppy525 = 55,
    /// Zip disk.
    ZipDisk = 56,
    /// slotMusic (microSD with preloaded music).
    SlotMusic = 57,
    /// Playbutton (self-contained wearable player badge).
    Playbutton = 58,
    /// Tefifon (grooved plastic tape cartridge).
    Tefifon = 59,
    /// Video High Density (VHD) disc.
    Vhd = 60,
    /// VHS videocassette.
    Vhs = 61,
    /// VinylDisc (hybrid CD/DVD with vinyl layer).
    VinylDisc = 62,

    /// Other or unspecified format.
    Other = 255,
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
    /// Standard plastic CD case.
    JewelCase = 0,
    /// Slim version of the jewel case.
    SlimJewelCase = 1,
    /// Super Jewel Box (enhanced CD case type).
    SuperJewelCase = 2,
    /// Fold-out cardboard packaging (Digipak).
    Digipak = 3,
    /// Generic cardboard sleeve.
    CardboardSleeve = 4,
    /// Gatefold fold-out sleeve (common for vinyl LPs).
    Gatefold = 5,
    /// Paper sleeve (simple paper jacket).
    PaperSleeve = 6,
    /// Keep case (DVD-style plastic case); includes generic keep/keepcase.
    KeepCase = 7,
    /// SteelBook metal case.
    SteelBook = 8,
    /// Amaray-branded keep case (common for DVDs/Blu-rays).
    AmarayCase = 9,
    /// Snap case (thin plastic snap-in case).
    SnapCase = 10,
    /// Longbox retail packaging (tall cardboard box).
    Longbox = 11,
    /// Box set container (multi-disc box).
    Box = 12,
    /// Clamshell plastic case.
    Clamshell = 13,
    /// Tin metal box.
    Tin = 14,
    /// Blister pack.
    BlisterPack = 15,
    /// Other or unspecified packaging.
    Other = 255,
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
    pub producer_id: PartyId,
    #[cfg_attr(feature = "std", ts(as = "String"))]
    pub catalog_nb: Option<MiddsString<32>>,
}
