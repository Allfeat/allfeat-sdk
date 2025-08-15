pub mod ean;

use allfeat_midds_v2_codegen::runtime_midds;

#[cfg(feature = "std")]
use self::ean::Ean;
use crate::{
    utils::{Country, Date},
    MiddsId,
};

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

#[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
use crate::benchmarking::{create_bounded_string, create_bounded_vec, BenchmarkHelper};

/// A MIDDS representing a musical release (album, EP, single, etc.).
/// It contains metadata and references to related MIDDS like tracks, producers, and artist.
///
/// This structure is used to register and manage a complete music release on-chain.
#[runtime_midds]
#[cfg_attr(feature = "web", wasm_bindgen(inspectable))]
pub struct Release {
    /// EAN or UPC code identifying the release (physical or digital).
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    #[as_runtime_type(path = "ean")]
    pub ean_upc: Ean,

    /// The main artist MIDDS ID associated with this release.
    pub artist: MiddsId,

    /// List of producer MIDDS IDs who contributed to this release.
    #[runtime_bound(256)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub producers: Vec<MiddsId>,

    /// List of track MIDDS IDs that are part of this release.
    #[runtime_bound(1024)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub tracks: Vec<MiddsId>,

    /// Name of the distributor responsible for the release.
    #[runtime_bound(256)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub distributor_name: String,

    /// Name of the manufacturer responsible for physical production.
    #[runtime_bound(256)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub manufacturer_name: String,

    /// Contributors to the release cover (designers, photographers, etc.).
    #[runtime_bound(64, 256)]
    #[as_runtime_type]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub cover_contributors: Vec<String>,

    /// Official title of the release.
    #[runtime_bound(256)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub title: String,

    /// Alternative titles (e.g. translations, acronyms, stylistic variations).
    #[runtime_bound(16, 256)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub title_aliases: Vec<String>,

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
#[runtime_midds]
#[cfg_attr(feature = "web", wasm_bindgen)]
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
#[cfg_attr(feature = "web", wasm_bindgen)]
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
#[cfg_attr(feature = "web", wasm_bindgen)]
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
#[cfg_attr(feature = "web", wasm_bindgen)]
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
impl BenchmarkHelper<RuntimeRelease> for RuntimeRelease {
    fn benchmark_instance(i: u32) -> RuntimeRelease {
        RuntimeRelease {
            ean_upc: ean::RuntimeEan::generate_benchmark(i),
            artist: 1u64,
            producers: create_bounded_vec::<MiddsId, 256>(10u64, i),
            tracks: create_bounded_vec::<MiddsId, 1024>(100u64, i),
            distributor_name: create_bounded_string::<256>(i),
            manufacturer_name: create_bounded_string::<256>(i),
            cover_contributors: create_bounded_vec::<RuntimeCoverContributorName, 64>(
                RuntimeCoverContributorName::generate_benchmark(1), // Use minimal size for nested items
                i,
            ),
            title: RuntimeReleaseTitle::generate_benchmark(i),
            title_aliases: create_bounded_vec::<RuntimeReleaseTitle, 16>(
                RuntimeReleaseTitle::generate_benchmark(1), // Use minimal size for nested items
                i,
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
            date: Date {
                year: 2023,
                month: 6,
                day: 15,
            },
            country: Country::US,
        }
    }
}

#[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
impl crate::benchmarking::BenchmarkHelper<RuntimeReleaseTitle> for RuntimeReleaseTitle {
    fn benchmark_instance(i: u32) -> RuntimeReleaseTitle {
        use crate::benchmarking::create_bounded_string;
        RuntimeReleaseTitle(create_bounded_string::<256>(i))
    }
}

#[cfg(feature = "std")]
mod api {
    use super::*;
    use std::fmt;
    use thiserror::Error;

    /// Error types for Release operations
    #[derive(Error, Debug, Clone, PartialEq, Eq)]
    pub enum ReleaseError {
        /// Invalid EAN/UPC
        #[error("Invalid EAN/UPC: {0}")]
        InvalidEan(String),
        /// Empty tracks list
        #[error("Release must have at least one track")]
        EmptyTracks,
        /// Invalid title
        #[error("Invalid title: {0}")]
        InvalidTitle(String),
        /// Invalid distributor name
        #[error("Invalid distributor name: {0}")]
        InvalidDistributor(String),
        /// Invalid manufacturer name
        #[error("Invalid manufacturer name: {0}")]
        InvalidManufacturer(String),
        /// Invalid release date
        #[error("Invalid release date")]
        InvalidDate,
        /// Too many tracks
        #[error("Too many tracks (max 1024): {0}")]
        TooManyTracks(usize),
        /// Too many producers
        #[error("Too many producers (max 256): {0}")]
        TooManyProducers(usize),
        /// Too many cover contributors
        #[error("Too many cover contributors (max 64): {0}")]
        TooManyCoverContributors(usize),
        /// Too many title aliases
        #[error("Too many title aliases (max 16): {0}")]
        TooManyTitleAliases(usize),
    }

    impl Release {
        /// Creates a new Release with required fields.
        ///
        /// # Arguments
        /// * `ean_upc` - The EAN/UPC code
        /// * `artist` - The main artist ID
        /// * `title` - The release title
        /// * `tracks` - List of track IDs
        /// * `date` - Release date
        /// * `country` - Release country
        ///
        /// # Examples
        /// ```
        /// use allfeat_midds_v2::release::*;
        /// use allfeat_midds_v2::release::ean::Ean;
        /// use allfeat_midds_v2::utils::{Date, Country};
        ///
        /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
        /// let release = Release::new(
        ///     Ean::new("1234567890128")?,
        ///     123, // artist ID
        ///     ReleaseTitle::new("My Album")?,
        ///     vec![456, 789], // track IDs
        ///     Date { year: 2024, month: 6, day: 15 },
        ///     Country::US
        /// )?;
        /// # Ok(())
        /// # }
        /// ```
        pub fn new(
            ean_upc: Ean,
            artist: MiddsId,
            title: String,
            tracks: Vec<MiddsId>,
            date: Date,
            country: Country,
        ) -> Result<Self, ReleaseError> {
            Self::validate_tracks(&tracks)?;
            Self::validate_date(&date)?;

            Ok(Self {
                ean_upc,
                artist,
                producers: vec![],
                tracks,
                distributor_name: String::new(),
                manufacturer_name: String::new(),
                cover_contributors: vec![],
                title,
                title_aliases: vec![],
                release_type: ReleaseType::Lp,
                format: ReleaseFormat::Cd,
                packaging: ReleasePackaging::JewelCase,
                status: ReleaseStatus::Official,
                date,
                country,
            })
        }

        /// Builder pattern - sets the EAN/UPC.
        pub fn with_ean(mut self, ean_upc: Ean) -> Self {
            self.ean_upc = ean_upc;
            self
        }

        /// Builder pattern - sets the release type.
        pub fn with_type(mut self, release_type: ReleaseType) -> Self {
            self.release_type = release_type;
            self
        }

        /// Builder pattern - sets the format.
        pub fn with_format(mut self, format: ReleaseFormat) -> Self {
            self.format = format;
            self
        }

        /// Builder pattern - sets the packaging.
        pub fn with_packaging(mut self, packaging: ReleasePackaging) -> Self {
            self.packaging = packaging;
            self
        }

        /// Builder pattern - sets the status.
        pub fn with_status(mut self, status: ReleaseStatus) -> Self {
            self.status = status;
            self
        }

        /// Builder pattern - sets the distributor.
        pub fn with_distributor(
            mut self,
            distributor: impl Into<String>,
        ) -> Result<Self, ReleaseError> {
            let distributor = distributor.into();
            Self::validate_distributor(&distributor)?;
            self.distributor_name = distributor.to_string();
            Ok(self)
        }

        /// Builder pattern - sets the manufacturer.
        pub fn with_manufacturer(
            mut self,
            manufacturer: impl Into<String>,
        ) -> Result<Self, ReleaseError> {
            let manufacturer = manufacturer.into();
            Self::validate_manufacturer(&manufacturer)?;
            self.manufacturer_name = manufacturer.to_string();
            Ok(self)
        }

        /// Adds a producer to the release.
        pub fn add_producer(&mut self, producer_id: MiddsId) -> Result<(), ReleaseError> {
            if self.producers.len() >= 256 {
                return Err(ReleaseError::TooManyProducers(self.producers.len() + 1));
            }
            self.producers.push(producer_id);
            Ok(())
        }

        /// Adds a track to the release.
        pub fn add_track(&mut self, track_id: MiddsId) -> Result<(), ReleaseError> {
            if self.tracks.len() >= 1024 {
                return Err(ReleaseError::TooManyTracks(self.tracks.len() + 1));
            }
            self.tracks.push(track_id);
            Ok(())
        }

        /// Adds a cover contributor.
        pub fn add_cover_contributor(&mut self, contributor: String) -> Result<(), ReleaseError> {
            if self.cover_contributors.len() >= 64 {
                return Err(ReleaseError::TooManyCoverContributors(
                    self.cover_contributors.len() + 1,
                ));
            }
            self.cover_contributors.push(contributor);
            Ok(())
        }

        /// Adds a title alias.
        pub fn add_title_alias(&mut self, alias: String) -> Result<(), ReleaseError> {
            if self.title_aliases.len() >= 16 {
                return Err(ReleaseError::TooManyTitleAliases(
                    self.title_aliases.len() + 1,
                ));
            }
            self.title_aliases.push(alias);
            Ok(())
        }

        /// Removes a producer from the release.
        pub fn remove_producer(&mut self, producer_id: MiddsId) {
            self.producers.retain(|&id| id != producer_id);
        }

        /// Removes a track from the release.
        pub fn remove_track(&mut self, track_id: MiddsId) {
            self.tracks.retain(|&id| id != track_id);
        }

        /// Gets the total number of tracks.
        pub fn track_count(&self) -> usize {
            self.tracks.len()
        }

        /// Gets the total number of producers.
        pub fn producer_count(&self) -> usize {
            self.producers.len()
        }

        /// Checks if this is a single release.
        pub fn is_single(&self) -> bool {
            matches!(self.release_type, ReleaseType::Single)
        }

        /// Checks if this is an EP.
        pub fn is_ep(&self) -> bool {
            matches!(self.release_type, ReleaseType::Ep)
        }

        /// Checks if this is an LP (album).
        pub fn is_album(&self) -> bool {
            matches!(self.release_type, ReleaseType::Lp | ReleaseType::DoubleLp)
        }

        /// Checks if this is a digital format.
        pub fn is_digital(&self) -> bool {
            // In the current enum, all formats are physical
            // This could be extended with digital formats
            false
        }

        /// Checks if this is a physical format.
        pub fn is_physical(&self) -> bool {
            !self.is_digital()
        }

        /// Gets the suggested release type based on track count.
        pub fn suggested_type_by_track_count(&self) -> ReleaseType {
            match self.tracks.len() {
                1..=2 => ReleaseType::Single,
                3..=7 => ReleaseType::Ep,
                8..=15 => ReleaseType::Lp,
                _ => ReleaseType::DoubleLp,
            }
        }

        /// Returns a formatted title with aliases.
        pub fn full_title(&self) -> String {
            if self.title_aliases.is_empty() {
                self.title.clone()
            } else {
                let aliases = self
                    .title_aliases
                    .iter()
                    .map(|alias| alias.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{} ({})", self.title, aliases)
            }
        }

        /// Returns the release year.
        pub fn year(&self) -> u16 {
            self.date.year
        }

        /// Checks if the release was published in a given year.
        pub fn released_in_year(&self, year: u16) -> bool {
            self.date.year == year
        }

        /// Checks if the release was published in a given decade.
        pub fn released_in_decade(&self, decade: u16) -> bool {
            let release_decade = (self.date.year / 10) * 10;
            release_decade == decade
        }

        /// Gets a search-friendly title.
        pub fn searchable_title(&self) -> String {
            self.title.to_lowercase()
        }

        /// Returns age of the release in years.
        pub fn age_in_years(&self) -> u16 {
            let current_year = 2025u16; // TODO: use actual current year
            current_year.saturating_sub(self.date.year)
        }

        /// Validates tracks list.
        fn validate_tracks(tracks: &[MiddsId]) -> Result<(), ReleaseError> {
            if tracks.is_empty() {
                return Err(ReleaseError::EmptyTracks);
            }
            if tracks.len() > 1024 {
                return Err(ReleaseError::TooManyTracks(tracks.len()));
            }
            Ok(())
        }

        /// Validates a distributor name.
        fn validate_distributor(name: &str) -> Result<(), ReleaseError> {
            if name.trim().is_empty() {
                return Err(ReleaseError::InvalidDistributor(
                    "Distributor name cannot be empty".to_string(),
                ));
            }
            if name.len() > 256 {
                return Err(ReleaseError::InvalidDistributor(
                    "Distributor name too long (max 256 chars)".to_string(),
                ));
            }
            Ok(())
        }

        /// Validates a manufacturer name.
        fn validate_manufacturer(name: &str) -> Result<(), ReleaseError> {
            if name.trim().is_empty() {
                return Err(ReleaseError::InvalidManufacturer(
                    "Manufacturer name cannot be empty".to_string(),
                ));
            }
            if name.len() > 256 {
                return Err(ReleaseError::InvalidManufacturer(
                    "Manufacturer name too long (max 256 chars)".to_string(),
                ));
            }
            Ok(())
        }

        /// Validates a release date.
        fn validate_date(date: &Date) -> Result<(), ReleaseError> {
            if date.year < 1000 || date.year > 2100 {
                return Err(ReleaseError::InvalidDate);
            }
            if !(1..=12).contains(&date.month) {
                return Err(ReleaseError::InvalidDate);
            }
            if !(1..=31).contains(&date.day) {
                return Err(ReleaseError::InvalidDate);
            }
            Ok(())
        }
    }

    impl fmt::Display for Release {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} - {} ({})", self.title, self.date.year, self.ean_upc)
        }
    }

    impl fmt::Display for ReleaseType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ReleaseType::Lp => write!(f, "LP"),
                ReleaseType::DoubleLp => write!(f, "Double LP"),
                ReleaseType::Ep => write!(f, "EP"),
                ReleaseType::Single => write!(f, "Single"),
                ReleaseType::Mixtape => write!(f, "Mixtape"),
            }
        }
    }

    impl fmt::Display for ReleaseFormat {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ReleaseFormat::Cd => write!(f, "CD"),
                ReleaseFormat::DoubleCd => write!(f, "Double CD"),
                ReleaseFormat::Vynil7 => write!(f, "7\" Vinyl"),
                ReleaseFormat::Vinyl10 => write!(f, "10\" Vinyl"),
                ReleaseFormat::Cassette => write!(f, "Cassette"),
                ReleaseFormat::AudioDvd => write!(f, "Audio DVD"),
            }
        }
    }

    impl fmt::Display for ReleasePackaging {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ReleasePackaging::Digipack => write!(f, "Digipack"),
                ReleasePackaging::JewelCase => write!(f, "Jewel Case"),
                ReleasePackaging::SnapCase => write!(f, "Snap Case"),
            }
        }
    }

    impl fmt::Display for ReleaseStatus {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ReleaseStatus::Official => write!(f, "Official"),
                ReleaseStatus::Promotional => write!(f, "Promotional"),
                ReleaseStatus::ReRelease => write!(f, "Re-Release"),
                ReleaseStatus::SpecialEdition => write!(f, "Special Edition"),
                ReleaseStatus::Remastered => write!(f, "Remastered"),
                ReleaseStatus::Bootleg => write!(f, "Bootleg"),
                ReleaseStatus::PseudoRelease => write!(f, "Pseudo Release"),
                ReleaseStatus::Withdrawn => write!(f, "Withdrawn"),
                ReleaseStatus::Expunged => write!(f, "Expunged"),
                ReleaseStatus::Cancelled => write!(f, "Cancelled"),
            }
        }
    }
}

// Re-export API types based on features
#[cfg(feature = "std")]
pub use api::*;

#[cfg(feature = "runtime")]
mod runtime_api {
    use super::{
        ean::RuntimeEan, ReleaseFormat, ReleasePackaging, ReleaseStatus, ReleaseType,
        RuntimeRelease,
    };
    use crate::{
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

    /// Error types for RuntimeRelease operations
    #[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
    pub enum RuntimeReleaseError {
        /// Data exceeds capacity limits
        #[error("Data exceeds capacity limits")]
        ExceedsCapacity,
        /// Invalid UTF-8 data
        #[error("Invalid UTF-8 data")]
        InvalidUtf8,
        /// Invalid track list
        #[error("Invalid track list")]
        InvalidTracks,
    }

    impl RuntimeRelease {
        /// Creates a new RuntimeRelease from raw parts
        pub fn new_from_parts(
            ean_upc: RuntimeEan,
            artist: MiddsId,
            title: BoundedVec<u8, ConstU32<256>>,
            tracks: BoundedVec<MiddsId, frame_support::traits::ConstU32<1024>>,
            date: Date,
            country: Country,
        ) -> Result<Self, RuntimeReleaseError> {
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
}

#[cfg(feature = "web")]
mod web_api {
    use super::ean::Ean;
    use super::Release;
    use crate::{
        utils::{Country, Date},
        MiddsId,
    };
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    impl Release {
        /// Creates a new Release for JavaScript
        #[wasm_bindgen(constructor)]
        pub fn new_web(
            ean_upc: &str,
            artist: MiddsId,
            title: &str,
            tracks: &[MiddsId],
            year: u16,
            month: u8,
            day: u8,
            country: u32,
        ) -> Result<Release, JsError> {
            let ean =
                Ean::new(ean_upc).map_err(|e| JsError::new(&format!("Invalid EAN: {}", e)))?;
            if title.trim().is_empty() {
                return Err(JsError::new("Title cannot be empty"));
            }

            let date = Date { year, month, day };
            let country_enum = match country {
                840 => Country::US,
                826 => Country::GB,
                250 => Country::FR,
                276 => Country::DE,
                392 => Country::JP,
                _ => Country::US, // Default fallback
            };

            Release::new(
                ean,
                artist,
                title.to_string(),
                tracks.to_vec(),
                date,
                country_enum,
            )
            .map_err(|e| JsError::new(&format!("Failed to create release: {}", e)))
        }

        /// Gets the release title
        #[wasm_bindgen(js_name = getTitle)]
        pub fn get_title_web(&self) -> String {
            self.title.clone()
        }

        /// Gets the EAN/UPC code
        #[wasm_bindgen(js_name = getEanUpc)]
        pub fn get_ean_upc_web(&self) -> String {
            self.ean_upc.to_string()
        }

        /// Gets the artist ID
        #[wasm_bindgen(js_name = getArtist)]
        pub fn get_artist_web(&self) -> MiddsId {
            self.artist
        }

        /// Gets the track IDs
        #[wasm_bindgen(js_name = getTracks)]
        pub fn get_tracks_web(&self) -> Vec<MiddsId> {
            self.tracks.clone()
        }

        /// Gets the release year
        #[wasm_bindgen(js_name = getYear)]
        pub fn get_year_web(&self) -> u16 {
            self.date.year
        }

        /// Gets the full title with aliases
        #[wasm_bindgen(js_name = getFullTitle)]
        pub fn get_full_title_web(&self) -> String {
            self.full_title()
        }

        /// Checks if released in a specific year
        #[wasm_bindgen(js_name = releasedInYear)]
        pub fn released_in_year_web(&self, year: u16) -> bool {
            self.released_in_year(year)
        }

        /// Checks if released in a specific decade
        #[wasm_bindgen(js_name = releasedInDecade)]
        pub fn released_in_decade_web(&self, decade: u16) -> bool {
            self.released_in_decade(decade)
        }

        /// Gets the number of tracks
        #[wasm_bindgen(js_name = getTrackCount)]
        pub fn get_track_count_web(&self) -> usize {
            self.tracks.len()
        }

        /// Gets the number of producers
        #[wasm_bindgen(js_name = getProducerCount)]
        pub fn get_producer_count_web(&self) -> usize {
            self.producers.len()
        }

        /// Gets the release age in years
        #[wasm_bindgen(js_name = getAgeInYears)]
        pub fn get_age_in_years_web(&self) -> u16 {
            self.age_in_years()
        }

        /// Gets a searchable title
        #[wasm_bindgen(js_name = getSearchableTitle)]
        pub fn get_searchable_title_web(&self) -> String {
            self.searchable_title()
        }

        /// Returns string representation
        #[wasm_bindgen(js_name = toString)]
        pub fn to_string_web(&self) -> String {
            self.to_string()
        }

        /// Gets the format as string
        #[wasm_bindgen(js_name = getFormat)]
        pub fn get_format_web(&self) -> String {
            self.format.to_string()
        }

        /// Gets the packaging as string
        #[wasm_bindgen(js_name = getPackaging)]
        pub fn get_packaging_web(&self) -> String {
            self.packaging.to_string()
        }

        /// Gets the status as string
        #[wasm_bindgen(js_name = getStatus)]
        pub fn get_status_web(&self) -> String {
            self.status.to_string()
        }

        /// Gets the release type as string
        #[wasm_bindgen(js_name = getReleaseType)]
        pub fn get_release_type_web(&self) -> String {
            self.release_type.to_string()
        }
    }


    /// Utility functions for JavaScript
    #[wasm_bindgen]
    pub struct ReleaseUtils;

    #[wasm_bindgen]
    impl ReleaseUtils {
        /// Validates an EAN/UPC string
        #[wasm_bindgen(js_name = validateEan)]
        pub fn validate_ean_web(ean: &str) -> bool {
            Ean::new(ean).is_ok()
        }

        /// Validates a release title
        #[wasm_bindgen(js_name = validateTitle)]
        pub fn validate_title_web(title: &str) -> bool {
            !title.trim().is_empty() && title.len() <= 256
        }

        /// Validates a contributor name
        #[wasm_bindgen(js_name = validateContributor)]
        pub fn validate_contributor_web(name: &str) -> bool {
            !name.trim().is_empty() && name.len() <= 256
        }

        /// Gets format name by index
        #[wasm_bindgen(js_name = getFormatName)]
        pub fn get_format_name_web(index: u32) -> String {
            match index {
                0 => "CD".to_string(),
                1 => "Double CD".to_string(),
                2 => "7\" Vinyl".to_string(),
                3 => "10\" Vinyl".to_string(),
                4 => "Cassette".to_string(),
                5 => "Audio DVD".to_string(),
                _ => "Unknown".to_string(),
            }
        }

        /// Gets packaging name by index
        #[wasm_bindgen(js_name = getPackagingName)]
        pub fn get_packaging_name_web(index: u32) -> String {
            match index {
                0 => "Digipack".to_string(),
                1 => "Jewel Case".to_string(),
                2 => "Snap Case".to_string(),
                _ => "Unknown".to_string(),
            }
        }

        /// Gets status name by index
        #[wasm_bindgen(js_name = getStatusName)]
        pub fn get_status_name_web(index: u32) -> String {
            match index {
                0 => "Official".to_string(),
                1 => "Promotional".to_string(),
                2 => "Re-Release".to_string(),
                3 => "Special Edition".to_string(),
                4 => "Remastered".to_string(),
                5 => "Bootleg".to_string(),
                6 => "Pseudo Release".to_string(),
                7 => "Withdrawn".to_string(),
                8 => "Expunged".to_string(),
                9 => "Cancelled".to_string(),
                _ => "Unknown".to_string(),
            }
        }
    }
}

#[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
impl crate::benchmarking::BenchmarkHelper<RuntimeCoverContributorName>
    for RuntimeCoverContributorName
{
    fn benchmark_instance(i: u32) -> RuntimeCoverContributorName {
        use crate::benchmarking::create_bounded_string;
        RuntimeCoverContributorName(create_bounded_string::<256>(i))
    }
}

// Re-export runtime error types for use in the unified error system
#[cfg(feature = "runtime")]
pub use runtime_api::RuntimeReleaseError;
