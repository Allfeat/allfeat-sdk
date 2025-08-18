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

pub mod error;
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
pub mod api;

#[cfg(feature = "web")]
pub mod web_api;

#[cfg(feature = "runtime")]
pub mod runtime_api;

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
