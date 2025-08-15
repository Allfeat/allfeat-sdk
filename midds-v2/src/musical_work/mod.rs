//! Musical work definitions and metadata structures.
//!
//! This module contains the core data structures for representing musical works (compositions),
//! including their identification, metadata, creators, and classification.
//!
//! # Core Types
//!
//! - [`MusicalWork`] - The main structure representing a musical composition
//! - [`MusicalWorkType`] - Classification of work types (original, medley, mashup, adaptation)
//! - [`Creator`] - Contributors to the musical work
//! - [`CreatorRole`] - Roles that creators can have
//! - [`ClassicalInfo`] - Additional metadata specific to classical music
//!
//! # Usage
//!
//! ```rust
//! use allfeat_midds_v2::{
//!     musical_work::{MusicalWork, Creator, CreatorRole, iswc::Iswc},
//!     utils::{Language, Key},
//! };
//!
//! let work = MusicalWork {
//!     iswc: Iswc::new("T-034524680-8").unwrap(),
//!     title: "My Composition".to_string(),
//!     creation_year: Some(2024),
//!     instrumental: Some(false),
//!     language: Some(Language::English),
//!     bpm: Some(120),
//!     key: Some(Key::C),
//!     work_type: None,
//!     creators: vec![
//!         Creator {
//!             id: 12345,
//!             role: CreatorRole::Composer,
//!         }
//!     ],
//!     classical_info: None,
//! };
//! ```

pub mod iswc;

#[cfg(feature = "std")]
use self::iswc::Iswc;
use crate::{
    utils::{Key, Language},
    MiddsId,
};
use allfeat_midds_v2_codegen::runtime_midds;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

#[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
use crate::benchmarking::{
    create_bounded_string, create_bounded_vec, create_optional_bounded_string, BenchmarkHelper,
};

/// Core data structure representing a musical work (composition).
///
/// A musical work encapsulates metadata about an original or derived
/// musical creation, including its creators, structure, and identity.
///
/// In the context of music industry standards, a musical work represents
/// the underlying composition - the melody, lyrics, and structure that can
/// be performed or recorded in multiple ways.
///
/// # Fields
///
/// ## Identification
/// - `iswc` - International Standard Musical Work Code for unique identification
/// - `title` - Primary title of the work
///
/// ## Temporal Information
/// - `creation_year` - Year the work was created (4-digit Gregorian year)
///
/// ## Musical Properties
/// - `instrumental` - Whether the work is purely instrumental (no lyrics)
/// - `language` - Language of the lyrics (if any)
/// - `bpm` - Beats per minute (tempo)
/// - `key` - Musical key of the work
///
/// ## Classification and Structure
/// - `work_type` - Type of work (original, medley, mashup, adaptation)
/// - `creators` - List of people and entities involved in creating the work
/// - `classical_info` - Additional metadata for classical works
///
/// # Type Transformations
///
/// In runtime mode, the following transformations apply:
/// - `title: String` → `title: BoundedVec<u8, ConstU32<256>>`
/// - `creators: Vec<Creator>` → `creators: BoundedVec<Creator, ConstU32<256>>`
///
/// # Examples
///
/// ## Simple Song
/// ```rust
/// # use allfeat_midds_v2::musical_work::*;
/// # use allfeat_midds_v2::musical_work::iswc::Iswc;
/// # use allfeat_midds_v2::utils::{Language, Key};
/// let song = MusicalWork {
///     iswc: Iswc::new("T-123456789-5").unwrap(),
///     title: "Yesterday".to_string(),
///     creation_year: Some(1965),
///     instrumental: Some(false),
///     language: Some(Language::English),
///     bpm: Some(76),
///     key: Some(Key::F),
///     work_type: None,
///     creators: vec![
///         Creator {
///             id: 1001,
///             role: CreatorRole::Composer,
///         }
///     ],
///     classical_info: None,
/// };
/// ```
///
/// ## Classical Work
/// ```rust
/// # use allfeat_midds_v2::musical_work::*;
/// # use allfeat_midds_v2::musical_work::iswc::Iswc;
/// # use allfeat_midds_v2::utils::{Language, Key};
/// let symphony = MusicalWork {
///     iswc: Iswc::new("T-987654321-5").unwrap(),
///     title: "Symphony No. 5 in C minor".to_string(),
///     creation_year: Some(1808),
///     instrumental: Some(true),
///     language: None,
///     bpm: Some(108),
///     key: Some(Key::Cm),
///     work_type: Some(MusicalWorkType::Original),
///     creators: vec![
///         Creator {
///             id: 2001,
///             role: CreatorRole::Composer,
///         }
///     ],
///     classical_info: Some(ClassicalInfo {
///         opus: Some("Op. 67".to_string()),
///         catalog_number: Some("LvB 67".to_string()),
///         number_of_voices: None,
///     }),
/// };
/// ```
#[runtime_midds]
#[cfg_attr(feature = "web", wasm_bindgen(inspectable))]
pub struct MusicalWork {
    /// The ISWC (International Standard Musical Work Code) uniquely identifying the work.
    #[as_runtime_type(path = "iswc")]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub iswc: Iswc,

    /// The title of the musical work.
    #[runtime_bound(256)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub title: String,

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
    #[as_runtime_type]
    #[cfg_attr(feature = "web", wasm_bindgen(skip))]
    /// TODO: make work types js compatible (enum variants non-supported for wasm_bindgen)
    pub work_type: Option<MusicalWorkType>,

    /// List of contributors to the work, along with their roles.
    #[runtime_bound(256)]
    #[as_runtime_type]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub creators: Vec<Creator>,

    /// Additional info if the work is a classical one.
    #[as_runtime_type]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub classical_info: Option<ClassicalInfo>,
}

/// Classification of different types of musical works.
///
/// This enum distinguishes between original compositions and works derived
/// from or combining existing musical works.
///
/// # Variants
///
/// - `Original` - A standalone, newly created composition
/// - `Medley` - A structured combination of multiple existing works performed sequentially
/// - `Mashup` - A creative blend mixing elements from multiple existing works simultaneously
/// - `Adaptation` - A modified version of a single existing work
///
/// # Type Transformations
/// In runtime mode, Vec fields are transformed to bounded vectors:
/// - `Medley(Vec<MiddsId>)` → `Medley(BoundedVec<MiddsId, ConstU32<512>>)`
/// - `Mashup(Vec<MiddsId>)` → `Mashup(BoundedVec<MiddsId, ConstU32<512>>)`
///
/// # Examples
/// ```rust
/// # use allfeat_midds_v2::musical_work::MusicalWorkType;
/// # use allfeat_midds_v2::MiddsId;
/// // Original composition
/// let original = MusicalWorkType::Original;
///
/// // Medley combining multiple works
/// let medley = MusicalWorkType::Medley(vec![123, 456, 789]);
///
/// // Adaptation of existing work
/// let adaptation = MusicalWorkType::Adaptation(999);
/// ```
#[runtime_midds]
pub enum MusicalWorkType {
    /// A standalone, original composition with no derivation from existing works.
    Original,

    /// A combination of multiple existing works arranged in sequence.
    ///
    /// Medleys typically present existing works in their recognizable form
    /// but arranged to flow together as a cohesive performance.
    #[runtime_bound(512)]
    Medley(Vec<MiddsId>),

    /// A creative blend mixing elements from multiple existing works.
    ///
    /// Mashups typically combine melodic, harmonic, or rhythmic elements
    /// from different works to create something new while maintaining
    /// recognizable elements from the source material.
    #[runtime_bound(512)]
    Mashup(Vec<MiddsId>),

    /// A modified version of a single existing work.
    ///
    /// Adaptations include arrangements, translations, or other modifications
    /// that create a derivative work from a single source.
    Adaptation(MiddsId),
}

/// A contributor to the creation of a musical work.
///
/// Creators represent individuals or entities involved in the creative
/// or administrative process of bringing a musical work to completion.
///
/// # Fields
/// - `id` - Unique MIDDS identifier for the person or entity
/// - `role` - The specific contribution or role they played
///
/// # Examples
/// ```rust
/// # use allfeat_midds_v2::musical_work::{Creator, CreatorRole};
/// let composer = Creator {
///     id: 12345,
///     role: CreatorRole::Composer,
/// };
///
/// let lyricist = Creator {
///     id: 67890,
///     role: CreatorRole::Author,
/// };
/// ```
#[runtime_midds]
#[cfg_attr(feature = "web", wasm_bindgen)]
pub struct Creator {
    /// MIDDS ID reference of the person or entity involved in the work.
    pub id: MiddsId,
    /// The specific role this creators played in the creation of the work.
    pub role: CreatorRole,
}

/// Roles that creators can have in the creation of a musical work.
///
/// These roles distinguish between different types of creative and administrative
/// contributions to a musical work's creation and publication.
///
/// # Variants
///
/// ## Creative Roles
/// - `Author` - Writer of lyrics or libretto
/// - `Composer` - Creator of the musical composition (melody, harmony, structure)
/// - `Arranger` - Creator of arrangements or orchestrations
/// - `Adapter` - Creator of adaptations, translations, or derivative versions
///
/// ## Administrative Roles
/// - `Publisher` - Entity responsible for publication and rights management
///
/// # Examples
/// ```rust
/// # use allfeat_midds_v2::musical_work::CreatorRole;
/// let roles = vec![
///     CreatorRole::Composer,   // Created the music
///     CreatorRole::Author,     // Wrote the lyrics
///     CreatorRole::Arranger,   // Created orchestral arrangement
///     CreatorRole::Publisher,  // Manages publication rights
/// ];
/// ```
#[runtime_midds]
#[cfg_attr(feature = "web", wasm_bindgen)]
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

/// Additional metadata specific to classical musical works.
///
/// Classical music often has additional cataloging and structural information
/// that is not relevant to popular music. This struct captures that metadata.
///
/// # Fields
/// - `opus` - Opus number assigned by the composer or cataloger
/// - `catalog_number` - Number in a scholarly catalog (e.g., Köchel, BWV)
/// - `number_of_voices` - Number of vocal parts in the work
///
/// # Type Transformations
/// In runtime mode, String fields are transformed:
/// - `opus: Option<String>` → `opus: Option<BoundedVec<u8, ConstU32<256>>>`
/// - `catalog_number: Option<String>` → `catalog_number: Option<BoundedVec<u8, ConstU32<256>>>`
///
/// # Examples
/// ```rust
/// # use allfeat_midds_v2::musical_work::ClassicalInfo;
/// // Beethoven's 9th Symphony
/// let beethoven_9th = ClassicalInfo {
///     opus: Some("Op. 125".to_string()),
///     catalog_number: Some("LvB 125".to_string()),
///     number_of_voices: Some(4), // SATB choir
/// };
///
/// // Mozart Piano Sonata
/// let mozart_sonata = ClassicalInfo {
///     opus: None, // Mozart didn't use opus numbers consistently
///     catalog_number: Some("K. 331".to_string()), // Köchel catalog
///     number_of_voices: None, // Instrumental work
/// };
/// ```
#[runtime_midds]
#[cfg_attr(feature = "web", wasm_bindgen)]
pub struct ClassicalInfo {
    /// Opus number assigned by the composer or music cataloger.
    ///
    /// Opus numbers are sequential numbers assigned to works, often by the composer,
    /// to indicate order of composition or publication. Format examples:
    /// - "Op. 27 No. 2" (Beethoven's Moonlight Sonata)
    /// - "Op. 9" (simple opus number)
    /// - "Op. posthumous" (published after death)
    #[runtime_bound(256)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub opus: Option<String>,

    /// Catalog number from a scholarly music catalog.
    ///
    /// Professional musicologists create comprehensive catalogs of composers' works.
    /// Examples include:
    /// - "K. 551" (Mozart's Jupiter Symphony in Köchel catalog)
    /// - "BWV 1006" (Bach work in Bach-Werke-Verzeichnis)
    /// - "D. 944" (Schubert work in Deutsch catalog)
    /// - "Hob. XVI:50" (Haydn work in Hoboken catalog)
    #[runtime_bound(256)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub catalog_number: Option<String>,

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

#[cfg(feature = "std")]
mod api {
    use super::*;
    use regex::Regex;
    use std::fmt;
    use thiserror::Error;

    static TITLE_REGEX: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static OPUS_REGEX: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();

    /// Error types for MusicalWork operations
    #[derive(Error, Debug, Clone, PartialEq, Eq)]
    pub enum MusicalWorkError {
        /// Invalid title format
        #[error("Invalid title format: {0}")]
        InvalidTitle(String),
        /// Invalid creation year
        #[error("Invalid creation year: {0}")]
        InvalidCreationYear(u16),
        /// Invalid BPM value
        #[error("Invalid BPM value: {0}")]
        InvalidBpm(u16),
        /// Invalid number of voices
        #[error("Invalid number of voices: {0}")]
        InvalidVoices(u16),
        /// Invalid ISWC
        #[error("Invalid ISWC: {0}")]
        InvalidIswc(String),
        /// Empty creators list
        #[error("Musical work must have at least one creator")]
        EmptyCreators,
        /// Invalid opus format
        #[error("Invalid opus format: {0}")]
        InvalidOpus(String),
        /// Invalid catalog number format
        #[error("Invalid catalog number format: {0}")]
        InvalidCatalogNumber(String),
    }

    impl MusicalWork {
        /// Creates a new MusicalWork with validation.
        ///
        /// # Arguments
        /// * `iswc` - The ISWC identifier
        /// * `title` - The title of the work
        /// * `creators` - List of creators (must not be empty)
        ///
        /// # Returns
        /// * `Ok(MusicalWork)` if all fields are valid
        /// * `Err(MusicalWorkError)` if validation fails
        ///
        /// # Examples
        /// ```
        /// use allfeat_midds_v2::musical_work::*;
        /// use allfeat_midds_v2::musical_work::iswc::Iswc;
        ///
        /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
        /// let work = MusicalWork::new(
        ///     Iswc::new("T-034524680-8")?,
        ///     "My Song",
        ///     vec![Creator { id: 123, role: CreatorRole::Composer }]
        /// )?;
        /// assert_eq!(work.title, "My Song");
        /// # Ok(())
        /// # }
        /// ```
        pub fn new(
            iswc: Iswc,
            title: impl Into<String>,
            creators: Vec<Creator>,
        ) -> Result<Self, MusicalWorkError> {
            let title = title.into();

            Self::validate_title(&title)?;
            Self::validate_creators(&creators)?;

            Ok(Self {
                iswc,
                title: title.to_string(),
                creation_year: None,
                instrumental: None,
                language: None,
                bpm: None,
                key: None,
                work_type: None,
                creators,
                classical_info: None,
            })
        }

        /// Creates a new MusicalWork without validation (unsafe).
        ///
        /// # Safety
        /// The caller must ensure all fields are valid.
        pub fn new_unchecked(iswc: Iswc, title: impl Into<String>, creators: Vec<Creator>) -> Self {
            Self {
                iswc,
                title: title.into(),
                creation_year: None,
                instrumental: None,
                language: None,
                bpm: None,
                key: None,
                work_type: None,
                creators,
                classical_info: None,
            }
        }

        /// Builder pattern - sets the creation year with validation.
        ///
        /// # Arguments
        /// * `year` - Year between 1000 and current year + 10
        ///
        /// # Examples
        /// ```
        /// # use allfeat_midds_v2::musical_work::*;
        /// # use allfeat_midds_v2::musical_work::iswc::Iswc;
        /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
        /// let work = MusicalWork::new(
        ///     Iswc::new("T-034524680-8")?,
        ///     "Song",
        ///     vec![Creator { id: 1, role: CreatorRole::Composer }]
        /// )?
        /// .with_creation_year(2024)?;
        /// # Ok(())
        /// # }
        /// ```
        pub fn with_creation_year(mut self, year: u16) -> Result<Self, MusicalWorkError> {
            Self::validate_creation_year(year)?;
            self.creation_year = Some(year);
            Ok(self)
        }

        /// Builder pattern - sets whether the work is instrumental.
        pub fn with_instrumental(mut self, instrumental: bool) -> Self {
            self.instrumental = Some(instrumental);
            self
        }

        /// Builder pattern - sets the language.
        pub fn with_language(mut self, language: Language) -> Self {
            self.language = Some(language);
            self
        }

        /// Builder pattern - sets the BPM with validation.
        ///
        /// # Arguments
        /// * `bpm` - BPM between 30 and 300
        pub fn with_bpm(mut self, bpm: u16) -> Result<Self, MusicalWorkError> {
            Self::validate_bpm(bpm)?;
            self.bpm = Some(bpm);
            Ok(self)
        }

        /// Builder pattern - sets the musical key.
        pub fn with_key(mut self, key: Key) -> Self {
            self.key = Some(key);
            self
        }

        /// Builder pattern - sets the work type.
        pub fn with_work_type(mut self, work_type: MusicalWorkType) -> Self {
            self.work_type = Some(work_type);
            self
        }

        /// Builder pattern - sets classical information.
        pub fn with_classical_info(mut self, classical_info: ClassicalInfo) -> Self {
            self.classical_info = Some(classical_info);
            self
        }

        /// Adds a creator to the work.
        pub fn add_creator(&mut self, creator: Creator) {
            self.creators.push(creator);
        }

        /// Removes creators with the specified role.
        pub fn remove_creators_by_role(&mut self, role: CreatorRole) {
            self.creators.retain(|c| c.role != role);
        }

        /// Gets all creators with a specific role.
        pub fn get_creators_by_role(&self, role: CreatorRole) -> Vec<&Creator> {
            self.creators.iter().filter(|c| c.role == role).collect()
        }

        /// Gets the primary composer (first composer in the list).
        pub fn primary_composer(&self) -> Option<&Creator> {
            self.creators
                .iter()
                .find(|c| c.role == CreatorRole::Composer)
        }

        /// Checks if the work has lyrics (not instrumental).
        pub fn has_lyrics(&self) -> bool {
            !self.instrumental.unwrap_or(false)
        }

        /// Checks if this is a classical work.
        pub fn is_classical(&self) -> bool {
            self.classical_info.is_some()
        }

        /// Checks if this is an original work.
        pub fn is_original(&self) -> bool {
            matches!(self.work_type, Some(MusicalWorkType::Original) | None)
        }

        /// Gets related work IDs if this is a derived work.
        pub fn related_work_ids(&self) -> Vec<MiddsId> {
            match &self.work_type {
                Some(MusicalWorkType::Medley(ids)) => ids.clone(),
                Some(MusicalWorkType::Mashup(ids)) => ids.clone(),
                Some(MusicalWorkType::Adaptation(id)) => vec![*id],
                _ => vec![],
            }
        }

        /// Normalizes the title by removing extra whitespace and trimming.
        pub fn normalize_title(title: &str) -> String {
            // Remove multiple spaces and trim
            let regex = TITLE_REGEX
                .get_or_init(|| Regex::new(r"\s+").expect("Title normalization regex is valid"));

            regex.replace_all(title.trim(), " ").to_string()
        }

        /// Creates a search-friendly version of the title.
        pub fn searchable_title(&self) -> String {
            Self::normalize_title(&self.title).to_lowercase()
        }

        /// Validates a title.
        fn validate_title(title: &str) -> Result<(), MusicalWorkError> {
            let normalized = Self::normalize_title(title);

            if normalized.is_empty() {
                return Err(MusicalWorkError::InvalidTitle(
                    "Title cannot be empty".to_string(),
                ));
            }

            if normalized.len() > 256 {
                return Err(MusicalWorkError::InvalidTitle(
                    "Title too long (max 256 chars)".to_string(),
                ));
            }

            Ok(())
        }

        /// Validates a creation year.
        fn validate_creation_year(year: u16) -> Result<(), MusicalWorkError> {
            let current_year = 2025u16;

            if year < 1000 || year > current_year + 10 {
                return Err(MusicalWorkError::InvalidCreationYear(year));
            }

            Ok(())
        }

        /// Validates a BPM value.
        fn validate_bpm(bpm: u16) -> Result<(), MusicalWorkError> {
            if !(30..=300).contains(&bpm) {
                return Err(MusicalWorkError::InvalidBpm(bpm));
            }

            Ok(())
        }

        /// Validates the creators list.
        fn validate_creators(creators: &[Creator]) -> Result<(), MusicalWorkError> {
            if creators.is_empty() {
                return Err(MusicalWorkError::EmptyCreators);
            }

            Ok(())
        }
    }

    impl ClassicalInfo {
        /// Creates new classical information with validation.
        pub fn new(
            opus: Option<String>,
            catalog_number: Option<String>,
            number_of_voices: Option<u16>,
        ) -> Result<Self, MusicalWorkError> {
            if let Some(ref opus) = opus {
                Self::validate_opus(opus)?;
            }

            if let Some(ref catalog) = catalog_number {
                Self::validate_catalog_number(catalog)?;
            }

            if let Some(voices) = number_of_voices {
                Self::validate_voices(voices)?;
            }

            Ok(Self {
                opus,
                catalog_number,
                number_of_voices,
            })
        }

        /// Normalizes an opus string.
        pub fn normalize_opus(opus: &str) -> String {
            let regex = OPUS_REGEX
                .get_or_init(|| Regex::new(r"(?i)^op\.?\s*(.+)$").expect("Opus regex is valid"));

            if let Some(captures) = regex.captures(opus.trim()) {
                format!("Op. {}", captures.get(1).unwrap().as_str())
            } else {
                opus.trim().to_string()
            }
        }

        /// Normalizes a catalog number.
        pub fn normalize_catalog_number(catalog: &str) -> String {
            catalog.trim().to_uppercase()
        }

        /// Validates an opus string.
        fn validate_opus(opus: &str) -> Result<(), MusicalWorkError> {
            if opus.trim().is_empty() {
                return Err(MusicalWorkError::InvalidOpus(
                    "Opus cannot be empty".to_string(),
                ));
            }

            if opus.len() > 256 {
                return Err(MusicalWorkError::InvalidOpus(
                    "Opus too long (max 256 chars)".to_string(),
                ));
            }

            Ok(())
        }

        /// Validates a catalog number.
        fn validate_catalog_number(catalog: &str) -> Result<(), MusicalWorkError> {
            if catalog.trim().is_empty() {
                return Err(MusicalWorkError::InvalidCatalogNumber(
                    "Catalog number cannot be empty".to_string(),
                ));
            }

            if catalog.len() > 256 {
                return Err(MusicalWorkError::InvalidCatalogNumber(
                    "Catalog number too long (max 256 chars)".to_string(),
                ));
            }

            Ok(())
        }

        /// Validates number of voices.
        fn validate_voices(voices: u16) -> Result<(), MusicalWorkError> {
            if voices == 0 || voices > 100 {
                return Err(MusicalWorkError::InvalidVoices(voices));
            }

            Ok(())
        }
    }

    impl Creator {
        /// Creates a new creator.
        pub fn new(id: MiddsId, role: CreatorRole) -> Self {
            Self { id, role }
        }

        /// Creates a composer.
        pub fn composer(id: MiddsId) -> Self {
            Self::new(id, CreatorRole::Composer)
        }

        /// Creates an author (lyricist).
        pub fn author(id: MiddsId) -> Self {
            Self::new(id, CreatorRole::Author)
        }

        /// Creates an arranger.
        pub fn arranger(id: MiddsId) -> Self {
            Self::new(id, CreatorRole::Arranger)
        }

        /// Creates an adapter.
        pub fn adapter(id: MiddsId) -> Self {
            Self::new(id, CreatorRole::Adapter)
        }

        /// Creates a publisher.
        pub fn publisher(id: MiddsId) -> Self {
            Self::new(id, CreatorRole::Publisher)
        }
    }

    impl MusicalWorkType {
        /// Creates an original work type.
        pub fn original() -> Self {
            Self::Original
        }

        /// Creates a medley work type.
        pub fn medley(work_ids: Vec<MiddsId>) -> Self {
            Self::Medley(work_ids)
        }

        /// Creates a mashup work type.
        pub fn mashup(work_ids: Vec<MiddsId>) -> Self {
            Self::Mashup(work_ids)
        }

        /// Creates an adaptation work type.
        pub fn adaptation(work_id: MiddsId) -> Self {
            Self::Adaptation(work_id)
        }

        /// Checks if this is an original work.
        pub fn is_original(&self) -> bool {
            matches!(self, Self::Original)
        }

        /// Checks if this is a derived work.
        pub fn is_derived(&self) -> bool {
            !self.is_original()
        }

        /// Gets the source work IDs if this is a derived work.
        pub fn source_work_ids(&self) -> Vec<MiddsId> {
            match self {
                Self::Medley(ids) => ids.clone(),
                Self::Mashup(ids) => ids.clone(),
                Self::Adaptation(id) => vec![*id],
                Self::Original => vec![],
            }
        }
    }

    impl fmt::Display for MusicalWork {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} ({})", self.title, self.iswc)
        }
    }

    impl fmt::Display for CreatorRole {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                CreatorRole::Author => write!(f, "Author"),
                CreatorRole::Composer => write!(f, "Composer"),
                CreatorRole::Arranger => write!(f, "Arranger"),
                CreatorRole::Adapter => write!(f, "Adapter"),
                CreatorRole::Publisher => write!(f, "Publisher"),
            }
        }
    }

    impl fmt::Display for MusicalWorkType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                MusicalWorkType::Original => write!(f, "Original"),
                MusicalWorkType::Medley(ids) => write!(f, "Medley of {} works", ids.len()),
                MusicalWorkType::Mashup(ids) => write!(f, "Mashup of {} works", ids.len()),
                MusicalWorkType::Adaptation(id) => write!(f, "Adaptation of work {}", id),
            }
        }
    }
}

#[cfg(feature = "web")]
mod web_api {
    use super::*;

    #[wasm_bindgen]
    impl MusicalWork {
        /// Creates a new MusicalWork from JavaScript.
        ///
        /// # Arguments
        /// * `iswc_str` - The ISWC string
        /// * `title` - The title of the work
        ///
        /// # Examples
        /// ```javascript
        /// import { MusicalWork, Creator, CreatorRole } from 'allfeat-midds-v2';
        ///
        /// try {
        ///   const work = MusicalWork.new("T-034524680-8", "My Song");
        ///   work.addCreator(new Creator(123, CreatorRole.Composer));
        /// } catch (error) {
        ///   console.error("Invalid musical work:", error);
        /// }
        /// ```
        #[wasm_bindgen(constructor)]
        pub fn new_web(iswc_str: &str, title: &str) -> Result<MusicalWork, JsError> {
            let iswc = match iswc::Iswc::new_web(iswc_str) {
                Ok(iswc) => iswc,
                Err(_) => return Err(JsError::new("Invalid ISWC")),
            };

            if title.trim().is_empty() {
                return Err(JsError::new("Title cannot be empty"));
            }

            Ok(MusicalWork {
                iswc,
                title: title.to_string(),
                creation_year: None,
                instrumental: None,
                language: None,
                bpm: None,
                key: None,
                work_type: None,
                creators: vec![],
                classical_info: None,
            })
        }

        /// Sets the creation year.
        #[wasm_bindgen(js_name = setCreationYear)]
        pub fn set_creation_year_web(&mut self, year: u16) -> Result<(), JsError> {
            if year < 1000 || year > 2034 {
                return Err(JsError::new("Invalid creation year"));
            }
            self.creation_year = Some(year);
            Ok(())
        }

        /// Sets whether the work is instrumental.
        #[wasm_bindgen(js_name = setInstrumental)]
        pub fn set_instrumental_web(&mut self, instrumental: bool) {
            self.instrumental = Some(instrumental);
        }

        /// Sets the BPM.
        #[wasm_bindgen(js_name = setBpm)]
        pub fn set_bpm_web(&mut self, bpm: u16) -> Result<(), JsError> {
            if bpm < 30 || bpm > 300 {
                return Err(JsError::new("BPM must be between 30 and 300"));
            }
            self.bpm = Some(bpm);
            Ok(())
        }

        /// Adds a creator to the work.
        #[wasm_bindgen(js_name = addCreator)]
        pub fn add_creator_web(&mut self, creator: Creator) {
            self.creators.push(creator);
        }

        /// Gets the number of creators.
        #[wasm_bindgen(js_name = creatorCount)]
        pub fn creator_count_web(&self) -> usize {
            self.creators.len()
        }

        /// Checks if the work has lyrics.
        #[wasm_bindgen(js_name = hasLyrics)]
        pub fn has_lyrics_web(&self) -> bool {
            !self.instrumental.unwrap_or(false)
        }

        /// Checks if this is a classical work.
        #[wasm_bindgen(js_name = isClassical)]
        pub fn is_classical_web(&self) -> bool {
            self.classical_info.is_some()
        }

        /// Gets a normalized, searchable title.
        #[wasm_bindgen(js_name = searchableTitle)]
        pub fn searchable_title_web(&self) -> String {
            self.title.trim().to_lowercase()
        }

        /// Returns string representation.
        #[wasm_bindgen(js_name = toString)]
        pub fn to_string_web(&self) -> String {
            format!("{} ({})", self.title, self.iswc.as_string_web())
        }
    }

    #[wasm_bindgen]
    impl Creator {
        /// Creates a new creator.
        #[wasm_bindgen(constructor)]
        pub fn new_web(id: u64, role: CreatorRole) -> Creator {
            Creator { id, role }
        }

        /// Creates a composer.
        #[wasm_bindgen(js_name = composer)]
        pub fn composer_web(id: u64) -> Creator {
            Creator::new_web(id, CreatorRole::Composer)
        }

        /// Creates an author (lyricist).
        #[wasm_bindgen(js_name = author)]
        pub fn author_web(id: u64) -> Creator {
            Creator::new_web(id, CreatorRole::Author)
        }

        /// Creates an arranger.
        #[wasm_bindgen(js_name = arranger)]
        pub fn arranger_web(id: u64) -> Creator {
            Creator::new_web(id, CreatorRole::Arranger)
        }

        /// Creates a publisher.
        #[wasm_bindgen(js_name = publisher)]
        pub fn publisher_web(id: u64) -> Creator {
            Creator::new_web(id, CreatorRole::Publisher)
        }

        /// Gets the role as a string.
        #[wasm_bindgen(js_name = roleString)]
        pub fn role_string_web(&self) -> String {
            match self.role {
                CreatorRole::Author => "Author".to_string(),
                CreatorRole::Composer => "Composer".to_string(),
                CreatorRole::Arranger => "Arranger".to_string(),
                CreatorRole::Adapter => "Adapter".to_string(),
                CreatorRole::Publisher => "Publisher".to_string(),
            }
        }
    }

    #[wasm_bindgen]
    impl ClassicalInfo {
        /// Creates new classical information.
        #[wasm_bindgen(constructor)]
        pub fn new_web(
            opus: Option<String>,
            catalog_number: Option<String>,
            number_of_voices: Option<u16>,
        ) -> Result<ClassicalInfo, JsError> {
            if let Some(voices) = number_of_voices {
                if voices == 0 || voices > 100 {
                    return Err(JsError::new("Invalid number of voices"));
                }
            }

            Ok(ClassicalInfo {
                opus,
                catalog_number,
                number_of_voices,
            })
        }

        /// Normalizes an opus string.
        #[wasm_bindgen(js_name = normalizeOpus)]
        pub fn normalize_opus_web(opus: &str) -> String {
            let trimmed = opus.trim();
            if trimmed.to_lowercase().starts_with("op") {
                if trimmed.contains('.') {
                    trimmed.to_string()
                } else {
                    format!("Op. {}", &trimmed[2..].trim())
                }
            } else {
                trimmed.to_string()
            }
        }

        /// Normalizes a catalog number.
        #[wasm_bindgen(js_name = normalizeCatalogNumber)]
        pub fn normalize_catalog_number_web(catalog: &str) -> String {
            catalog.trim().to_uppercase()
        }
    }
}

#[cfg(feature = "runtime")]
mod runtime_api {
    use super::*;
    use frame_support::BoundedVec;

    #[cfg(not(feature = "std"))]
    extern crate alloc;

    #[cfg(not(feature = "std"))]
    use alloc::{
        string::{String, ToString},
        vec::Vec,
    };

    use thiserror::Error;

    /// Error types for RuntimeMusicalWork operations
    #[derive(Error, Debug, Clone, PartialEq, Eq)]
    pub enum RuntimeMusicalWorkError {
        /// Data exceeds capacity limits
        #[error("Data exceeds capacity limits: {0}")]
        ExceedsCapacity(String),
        /// Invalid UTF-8 data
        #[error("Invalid UTF-8 data")]
        InvalidUtf8,
        /// Empty creators list
        #[error("Musical work must have at least one creator")]
        EmptyCreators,
    }

    impl RuntimeMusicalWork {
        /// Creates a new RuntimeMusicalWork from components without validation.
        ///
        /// # Arguments
        /// * `iswc` - The ISWC identifier
        /// * `title_bytes` - The title as bytes
        /// * `creators` - List of creators
        ///
        /// # Returns
        /// * `Ok(RuntimeMusicalWork)` if all data fits within bounds
        /// * `Err(RuntimeMusicalWorkError)` if data exceeds capacity
        pub fn new_from_components(
            iswc: iswc::RuntimeIswc,
            title_bytes: impl AsRef<[u8]>,
            creators: Vec<Creator>,
        ) -> Result<Self, RuntimeMusicalWorkError> {
            if creators.is_empty() {
                return Err(RuntimeMusicalWorkError::EmptyCreators);
            }

            let title = BoundedVec::try_from(title_bytes.as_ref().to_vec())
                .map_err(|_| RuntimeMusicalWorkError::ExceedsCapacity("title".to_string()))?;

            let creators_bounded = BoundedVec::try_from(creators)
                .map_err(|_| RuntimeMusicalWorkError::ExceedsCapacity("creators".to_string()))?;

            Ok(Self {
                iswc,
                title,
                creation_year: None,
                instrumental: None,
                language: None,
                bpm: None,
                key: None,
                work_type: None,
                creators: creators_bounded,
                classical_info: None,
            })
        }

        /// Creates a new RuntimeMusicalWork from string data.
        pub fn new_from_strings(
            iswc_str: &str,
            title: &str,
            creators: Vec<Creator>,
        ) -> Result<Self, RuntimeMusicalWorkError> {
            let iswc = iswc::RuntimeIswc::new_from_str(iswc_str)
                .map_err(|_| RuntimeMusicalWorkError::ExceedsCapacity("iswc".to_string()))?;

            Self::new_from_components(iswc, title.as_bytes(), creators)
        }

        /// Returns the title as a string if it contains valid UTF-8.
        pub fn title_string(&self) -> Result<String, RuntimeMusicalWorkError> {
            String::from_utf8(self.title.to_vec()).map_err(|_| RuntimeMusicalWorkError::InvalidUtf8)
        }

        /// Returns the title as a lossy string.
        pub fn title_string_lossy(&self) -> String {
            String::from_utf8_lossy(&self.title).to_string()
        }

        /// Returns the title as bytes.
        pub fn title_bytes(&self) -> &[u8] {
            &self.title
        }

        /// Returns the ISWC as bytes.
        pub fn iswc_bytes(&self) -> &[u8] {
            self.iswc.as_bytes()
        }

        /// Returns the number of creators.
        pub fn creator_count(&self) -> usize {
            self.creators.len()
        }

        /// Checks if the work has lyrics.
        pub fn has_lyrics(&self) -> bool {
            !self.instrumental.unwrap_or(false)
        }

        /// Checks if this is a classical work.
        pub fn is_classical(&self) -> bool {
            self.classical_info.is_some()
        }

        /// Returns capacity information.
        pub fn capacity_info() -> (u32, u32) {
            (256, 256) // (title_capacity, creators_capacity)
        }

        /// Returns current usage.
        pub fn current_usage(&self) -> (usize, usize) {
            (self.title.len(), self.creators.len())
        }
    }

    impl RuntimeClassicalInfo {
        /// Creates new RuntimeClassicalInfo from string data.
        pub fn new_from_strings(
            opus: Option<&str>,
            catalog_number: Option<&str>,
            number_of_voices: Option<u16>,
        ) -> Result<Self, RuntimeMusicalWorkError> {
            let opus_bounded = if let Some(opus_str) = opus {
                Some(
                    BoundedVec::try_from(opus_str.as_bytes().to_vec()).map_err(|_| {
                        RuntimeMusicalWorkError::ExceedsCapacity("opus".to_string())
                    })?,
                )
            } else {
                None
            };

            let catalog_bounded = if let Some(catalog_str) = catalog_number {
                Some(
                    BoundedVec::try_from(catalog_str.as_bytes().to_vec()).map_err(|_| {
                        RuntimeMusicalWorkError::ExceedsCapacity("catalog_number".to_string())
                    })?,
                )
            } else {
                None
            };

            Ok(Self {
                opus: opus_bounded,
                catalog_number: catalog_bounded,
                number_of_voices,
            })
        }

        /// Returns the opus as a string if it contains valid UTF-8.
        pub fn opus_string(&self) -> Option<Result<String, RuntimeMusicalWorkError>> {
            self.opus.as_ref().map(|opus| {
                String::from_utf8(opus.to_vec()).map_err(|_| RuntimeMusicalWorkError::InvalidUtf8)
            })
        }

        /// Returns the catalog number as a string if it contains valid UTF-8.
        pub fn catalog_number_string(&self) -> Option<Result<String, RuntimeMusicalWorkError>> {
            self.catalog_number.as_ref().map(|catalog| {
                String::from_utf8(catalog.to_vec())
                    .map_err(|_| RuntimeMusicalWorkError::InvalidUtf8)
            })
        }
    }
}

// Re-export API types based on features
#[cfg(feature = "std")]
pub use api::*;

// Benchmark implementation for the main MIDDS type
#[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
impl BenchmarkHelper<RuntimeMusicalWork> for RuntimeMusicalWork {
    fn benchmark_instance(i: u32) -> RuntimeMusicalWork {
        RuntimeMusicalWork {
            iswc: iswc::RuntimeIswc::generate_benchmark(i),
            title: create_bounded_string::<256>(i),
            creation_year: Some(2023),
            instrumental: Some(false),
            language: Some(Language::English),
            bpm: Some(120),
            key: Some(Key::C),
            work_type: if i == 0 {
                None
            } else if i % 3 == 0 {
                Some(RuntimeMusicalWorkType::Medley(create_bounded_vec::<
                    MiddsId,
                    512,
                >(42u64, i)))
            } else if i % 3 == 1 {
                Some(RuntimeMusicalWorkType::Mashup(create_bounded_vec::<
                    MiddsId,
                    512,
                >(42u64, i)))
            } else {
                Some(RuntimeMusicalWorkType::Original)
            },
            creators: create_bounded_vec::<Creator, 256>(
                Creator {
                    id: 42u64,
                    role: CreatorRole::Composer,
                },
                i,
            ),
            classical_info: if i == 0 {
                None
            } else {
                Some(RuntimeClassicalInfo {
                    opus: create_optional_bounded_string::<256>(i),
                    catalog_number: create_optional_bounded_string::<256>(i),
                    number_of_voices: Some(4),
                })
            },
        }
    }
}

// Re-export runtime error types for use in the unified error system
#[cfg(feature = "runtime")]
pub use runtime_api::RuntimeMusicalWorkError;
