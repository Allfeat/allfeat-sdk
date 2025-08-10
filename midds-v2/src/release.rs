use allfeat_midds_v2_codegen::runtime_midds;

use crate::{
    utils::{Country, Date},
    MiddsId,
};

#[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
use crate::benchmarking::{
    BenchmarkHelper, create_bounded_string, create_bounded_vec
};

/// A MIDDS representing a musical release (album, EP, single, etc.).
/// It contains metadata and references to related MIDDS like tracks, producers, and artist.
///
/// This structure is used to register and manage a complete music release on-chain.
#[runtime_midds]
pub struct Release {
    /// EAN or UPC code identifying the release (physical or digital).
    pub ean_upc: Ean,

    /// The main artist MIDDS ID associated with this release.
    pub artist: MiddsId,

    /// List of producer MIDDS IDs who contributed to this release.
    #[runtime_bound(256)]
    pub producers: Vec<MiddsId>,

    /// List of track MIDDS IDs that are part of this release.
    #[runtime_bound(1024)]
    pub tracks: Vec<MiddsId>,

    /// Name of the distributor responsible for the release.
    #[runtime_bound(256)]
    pub distributor_name: String,

    /// Name of the manufacturer responsible for physical production.
    #[runtime_bound(256)]
    pub manufacturer_name: String,

    /// Contributors to the release cover (designers, photographers, etc.).
    #[runtime_bound(64)]
    pub cover_contributors: Vec<CoverContributorName>,

    /// Official title of the release.
    pub title: ReleaseTitle,

    /// Alternative titles (e.g. translations, acronyms, stylistic variations).
    #[runtime_bound(16)]
    pub title_aliases: Vec<ReleaseTitle>,

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

#[runtime_midds]
pub struct Ean(#[runtime_bound(13)] String);

#[runtime_midds]
pub struct ReleaseTitle(#[runtime_bound(256)] pub String);

#[runtime_midds]
pub struct CoverContributorName(#[runtime_bound(256)] pub String);

/// The general type of release based on track count or intent.
#[repr(u8)]
#[runtime_midds]
pub enum ReleaseType {
    /// Long Play album (usually 8+ tracks).
    Lp = 0,
    /// Double album (2 discs or extensive track list).
    DoubleLp = 1,
    /// Extended Play (typically 4–6 tracks).
    Ep = 2,
    /// A standalone track or 2-track release.
    Single = 3,
    /// Informal or promotional compilation, often non-commercial.
    Mixtape = 4,
}

/// The format of the physical or digital medium used for distribution.
#[repr(u8)]
#[runtime_midds]
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
#[runtime_midds]
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
#[runtime_midds]
pub enum ReleaseStatus {
    /// Properly released by the artist or label.
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

// Benchmark implementation for the main MIDDS type
#[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
impl BenchmarkHelper<Release> for Release {
    fn benchmark_instance(i: u32) -> Release {
        Release {
            ean_upc: Ean(create_bounded_string::<13>(i)),
            artist: 1u64,
            producers: create_bounded_vec::<MiddsId, 256>(10u64, i),
            tracks: create_bounded_vec::<MiddsId, 1024>(100u64, i),
            distributor_name: create_bounded_string::<256>(i),
            manufacturer_name: create_bounded_string::<256>(i),
            cover_contributors: create_bounded_vec::<CoverContributorName, 64>(
                CoverContributorName(create_bounded_string::<256>(1)), // Use minimal size for nested items
                i
            ),
            title: ReleaseTitle(create_bounded_string::<256>(i)),
            title_aliases: create_bounded_vec::<ReleaseTitle, 16>(
                ReleaseTitle(create_bounded_string::<256>(1)), // Use minimal size for nested items
                i
            ),
            release_type: match i % 5 {
                0 => ReleaseType::Single,
                1 => ReleaseType::Ep,
                2 => ReleaseType::Lp,
                3 => ReleaseType::DoubleLp,
                _ => ReleaseType::Mixtape,
            },
            format: match i % 6 {
                0 => ReleaseFormat::Cd,
                1 => ReleaseFormat::DoubleCd,
                2 => ReleaseFormat::Vynil7,
                3 => ReleaseFormat::Vinyl10,
                4 => ReleaseFormat::Cassette,
                _ => ReleaseFormat::AudioDvd,
            },
            packaging: match i % 3 {
                0 => ReleasePackaging::JewelCase,
                1 => ReleasePackaging::Digipack,
                _ => ReleasePackaging::SnapCase,
            },
            status: match i % 10 {
                0 => ReleaseStatus::Official,
                1 => ReleaseStatus::Promotional,
                2 => ReleaseStatus::ReRelease,
                3 => ReleaseStatus::SpecialEdition,
                4 => ReleaseStatus::Remastered,
                5 => ReleaseStatus::Bootleg,
                6 => ReleaseStatus::PseudoRelease,
                7 => ReleaseStatus::Withdrawn,
                8 => ReleaseStatus::Expunged,
                _ => ReleaseStatus::Cancelled,
            },
            date: Date { year: 2023, month: 6, day: 15 },
            country: Country::US,
        }
    }
}
