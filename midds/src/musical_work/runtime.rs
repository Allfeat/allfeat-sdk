// This file is part of Allfeat.

// Copyright (C) 2022-2025 Allfeat.
// SPDX-License-Identifier: GPL-3.0-or-later

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

extern crate alloc;

#[cfg(feature = "runtime-benchmarks")]
use alloc::format;

use alloc::collections::BTreeSet;

use frame_support::sp_runtime::RuntimeDebug;
use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

#[cfg(feature = "js")]
use wasm_bindgen::prelude::*;

use crate::{
    musical_work::types::{
        ClassicalInfo, Iswc, MusicalWorkBpm, MusicalWorkCreationYear, MusicalWorkParticipants,
        MusicalWorkTitle, MusicalWorkType,
    },
    shared::{
        conversion::{Validatable, ValidationError},
        Key, Language,
    },
    Midds,
};

/// Core data structure representing a musical work (composition).
///
/// A musical work encapsulates metadata about an original or derived
/// musical creation, including its participants, structure, and identity.
#[derive(
    Clone,
    Eq,
    PartialEq,
    Encode,
    Decode,
    DecodeWithMemTracking,
    TypeInfo,
    MaxEncodedLen,
    RuntimeDebug,
)]
#[cfg_attr(feature = "js", wasm_bindgen(inspectable))]
pub struct MusicalWork {
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    /// The ISWC (International Standard Musical Work Code) uniquely identifying the work.
    pub iswc: Iswc,

    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    /// The title of the musical work.
    pub title: MusicalWorkTitle,

    /// The year the work was created (4-digit Gregorian year).
    pub creation_year: Option<MusicalWorkCreationYear>,

    /// Indicates whether the work is instrumental (i.e., without lyrics).
    pub instrumental: Option<bool>,

    /// The optional language of the lyrics (if any).
    pub language: Option<Language>,

    /// Optional tempo in beats per minute (BPM).
    pub bpm: Option<MusicalWorkBpm>,

    /// Optional musical key of the work (e.g., C, G#, etc.).
    pub key: Option<Key>,

    #[cfg_attr(feature = "js", wasm_bindgen(skip))]
    /// Type of the musical work (original, medley, mashup, or adaptation).
    pub work_type: Option<MusicalWorkType>,

    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    /// List of contributors to the work, along with their roles.
    pub participants: MusicalWorkParticipants,

    /// Additional info if the work is a classical one.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub classical_info: Option<ClassicalInfo>,
}

/// Trait implementation allowing the musical work to be used
/// as a MIDDS (Music Industry Decentralized Data Structure).
impl Midds for MusicalWork {
    const NAME: &'static str = "Musical Work";

    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = MusicalWorkBenchmarkHelper;
}

/// Validation implementation for `MusicalWork`
impl Validatable for MusicalWork {
    type Error = ValidationError;

    fn validate(&self) -> Result<(), Self::Error> {
        // Validate title is not empty
        if self.title.is_empty() {
            return Err(ValidationError::invalid_format(
                "Title",
                "Title cannot be empty",
            ));
        }

        // Validate creation year if present
        if let Some(year) = self.creation_year {
            if !(1000..=9999).contains(&year) {
                return Err(ValidationError::invalid_format(
                    "CreationYear",
                    "Year must be 4 digits",
                ));
            }
        }

        // Validate BPM if present
        if let Some(bpm) = self.bpm {
            if bpm == 0 || bpm > 300 {
                return Err(ValidationError::invalid_format(
                    "BPM",
                    "BPM must be between 1 and 300",
                ));
            }
        }

        // Validate participants don't have duplicate roles from the same person
        let mut seen_participants = BTreeSet::new();
        for participant in &self.participants {
            let key = (participant.id, &participant.role);
            if !seen_participants.insert(key) {
                return Err(ValidationError::invalid_format(
                    "Participants",
                    "Duplicate participant role for the same person",
                ));
            }
        }

        Ok(())
    }
}

#[cfg(feature = "runtime-benchmarks")]
pub struct MusicalWorkBenchmarkHelper;

#[cfg(feature = "runtime-benchmarks")]
impl crate::benchmarking::BenchmarkHelperT<MusicalWork> for MusicalWorkBenchmarkHelper {
    fn min_size() -> MusicalWork {
        use crate::musical_work::types::*;
        use core::str::FromStr;

        MusicalWork {
            iswc: Iswc::from_str("T-1234567-8").unwrap(),
            title: MusicalWorkTitle::from_str("T").unwrap(),
            creation_year: None,
            instrumental: None,
            language: None,
            bpm: None,
            key: None,
            work_type: None,
            participants: MusicalWorkParticipants::new(),
            classical_info: None,
        }
    }

    fn max_size() -> MusicalWork {
        use crate::musical_work::types::*;
        use core::str::FromStr;

        // Helper to create maximum length strings
        let max_iswc = "T-123456789-1"; // 11 chars max
        let max_title = "T".repeat(256); // 256 chars max
        let max_opus = "O".repeat(128); // 128 chars max
        let max_catalog = "C".repeat(128); // 128 chars max

        // Helper to create maximum participants
        let max_participant = Participant {
            id: u64::MAX,
            role: ParticipantRole::Publisher,
        };

        let mut participants = MusicalWorkParticipants::new();
        // Fill to maximum capacity (512 participants)
        for _ in 0..512 {
            if participants.push(max_participant.clone()).is_err() {
                break;
            }
        }

        // Create maximum derived works collection
        let mut derived_works = DerivedWorks::new();
        for i in 0..512 {
            if derived_works.push(i as u64).is_err() {
                break;
            }
        }

        MusicalWork {
            iswc: Iswc::from_str(max_iswc).unwrap(),
            title: MusicalWorkTitle::from_str(&max_title).unwrap(),
            creation_year: Some(2024),
            instrumental: Some(true),
            language: Some(crate::shared::Language::English),
            bpm: Some(u16::MAX),
            key: Some(crate::shared::Key::Cs),
            work_type: Some(MusicalWorkType::Medley(derived_works)),
            participants,
            classical_info: Some(ClassicalInfo::new(
                Opus::from_str(&max_opus).ok(),
                CatalogNumber::from_str(&max_catalog).ok(),
                Some(u16::MAX),
            )),
        }
    }

    fn variable_size(complexity: f32) -> MusicalWork {
        use crate::benchmarking::utils::*;
        use crate::musical_work::types::*;
        use core::str::FromStr;

        // Calculate dynamic lengths based on complexity
        let title_len =
            target_length_for_complexity::<frame_support::traits::ConstU32<256>>(complexity);
        let iswc_len = (7.0 + complexity * 4.0) as usize; // 7-11 characters

        // Create complexity-based ISWC
        let iswc_base = format!("T-{:0width$}-1", 1234567, width = iswc_len - 4);

        // Create complexity-based title
        let title_content = if complexity > 0.8 {
            format!(
                "Complex Musical Work Title {}",
                "X".repeat(title_len.saturating_sub(30))
            )
        } else if complexity > 0.5 {
            format!("Musical Work {}", "T".repeat(title_len.saturating_sub(15)))
        } else {
            "T".repeat(title_len.max(1))
        };

        // Create participants based on complexity
        let mut participants = MusicalWorkParticipants::new();
        let participant_count = (complexity * 512.0) as usize;
        for i in 0..participant_count {
            let participant = Participant {
                id: i as u64,
                role: if i % 4 == 0 {
                    ParticipantRole::Composer
                } else if i % 4 == 1 {
                    ParticipantRole::Author
                } else if i % 4 == 2 {
                    ParticipantRole::Publisher
                } else {
                    ParticipantRole::Arranger
                },
            };
            if participants.push(participant).is_err() {
                break;
            }
        }

        // Create work type based on complexity
        let work_type = if complexity > 0.8 {
            // Complex medley with derived works
            let mut derived_works = DerivedWorks::new();
            let derived_count = (complexity * 100.0) as usize;
            for i in 0..derived_count {
                if derived_works.push(i as u64).is_err() {
                    break;
                }
            }
            Some(MusicalWorkType::Medley(derived_works))
        } else if complexity > 0.6 {
            Some(MusicalWorkType::Adaptation((complexity * 1000000.0) as u64))
        } else if complexity > 0.4 {
            Some(MusicalWorkType::Original)
        } else {
            None
        };

        // Create classical info based on complexity
        let classical_info = if complexity > 0.7 {
            let opus_content = if complexity > 0.8 {
                let opus_len = ((complexity * 50.0) as usize + 1).min(128);
                Opus::from_str(&"O".repeat(opus_len)).ok()
            } else {
                None
            };

            let catalog_content = if complexity > 0.9 {
                let catalog_len = ((complexity * 50.0) as usize + 1).min(128);
                CatalogNumber::from_str(&"C".repeat(catalog_len)).ok()
            } else {
                None
            };

            Some(ClassicalInfo::new(
                opus_content,
                catalog_content,
                if complexity > 0.85 {
                    Some((complexity * 20.0) as u16)
                } else {
                    None
                },
            ))
        } else {
            None
        };

        // Build the final MusicalWork
        MusicalWork {
            iswc: Iswc::from_str(&iswc_base)
                .unwrap_or_else(|_| Iswc::from_str("T-1234567-8").unwrap()),
            title: MusicalWorkTitle::from_str(&title_content)
                .unwrap_or_else(|_| MusicalWorkTitle::from_str("T").unwrap()),
            creation_year: if complexity > 0.3 {
                Some((1900.0 + complexity * 124.0) as u16)
            } else {
                None
            },
            instrumental: if complexity > 0.4 {
                Some(complexity > 0.7)
            } else {
                None
            },
            language: if complexity > 0.5 {
                Some(crate::shared::Language::English)
            } else {
                None
            },
            bpm: if complexity > 0.4 {
                Some((complexity * 200.0 + 60.0) as u16)
            } else {
                None
            },
            key: if complexity > 0.6 {
                Some(crate::shared::Key::C)
            } else {
                None
            },
            work_type,
            participants,
            classical_info,
        }
    }
}
