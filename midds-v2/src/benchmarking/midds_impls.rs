//! Benchmarking helper implementations for main MIDDS types
//!
//! This module contains BenchmarkHelper implementations for the core MIDDS structures
//! that will be used in Substrate extrinsics for weight calculation.

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{format, vec::Vec};

use super::{BenchmarkHelper, BenchmarkMapper};
use crate::shared::genres::GenreId;
use crate::{
    MiddsString, MiddsVec,
    musical_work::{ClassicalInfo, Creator, CreatorRole, MusicalWork, MusicalWorkType},
    recording::{Recording, RecordingVersion},
    release::{ProducerInfo, Release, ReleaseFormat, ReleasePackaging, ReleaseStatus, ReleaseType},
    shared::{BothIdsContainer, PartyId},
    shared::{Country, Date, Key, Language},
};

// Helper function to generate benchmark PartyId
#[allow(dead_code)]
fn benchmark_party_id(complexity: u32) -> PartyId {
    if complexity == 0 {
        // Minimal case: just IPI
        let ipi = 100_000_000_u64; // Minimum 9-digit IPI
        PartyId::Ipi(ipi)
    } else if complexity < u32::MAX / 2 {
        // Medium complexity: IPI only with variable value
        let ipi_val = 100_000_000 + (complexity as u64 % (99_999_999_999 - 100_000_000));
        PartyId::Ipi(ipi_val)
    } else {
        // High complexity: Both IPI and ISNI
        let ipi_val = 100_000_000 + (complexity as u64 % (99_999_999_999 - 100_000_000));
        let isni = "000000012345678X"
            .as_bytes()
            .to_vec()
            .try_into()
            .unwrap_or_default();
        PartyId::Both(BothIdsContainer { ipi: ipi_val, isni })
    }
}

// Helper function to generate benchmark creators
#[allow(dead_code)]
fn benchmark_creators(complexity: u32) -> MiddsVec<Creator, 256> {
    let count = BenchmarkMapper::complexity_to_collection_size(complexity, 256);
    let actual_count = count.max(1); // Ensure at least one creator

    let creators: Vec<Creator> = (0..actual_count)
        .map(|i| {
            let role_index = i % 5; // Cycle through roles
            let role = match role_index {
                0 => CreatorRole::Author,
                1 => CreatorRole::Composer,
                2 => CreatorRole::Arranger,
                3 => CreatorRole::Adapter,
                _ => CreatorRole::Publisher,
            };

            Creator {
                id: benchmark_party_id(complexity.saturating_add(i)),
                role,
            }
        })
        .collect();

    creators
        .try_into()
        .expect("Should always have at least one creator")
}

// Benchmark helper for Creator
#[allow(dead_code)]
pub struct CreatorBenchmarkHelper;

impl BenchmarkHelper<Creator> for CreatorBenchmarkHelper {
    fn benchmark_instance(complexity: u32) -> Creator {
        let role = match complexity % 5 {
            0 => CreatorRole::Author,
            1 => CreatorRole::Composer,
            2 => CreatorRole::Arranger,
            3 => CreatorRole::Adapter,
            _ => CreatorRole::Publisher,
        };

        Creator {
            id: benchmark_party_id(complexity),
            role,
        }
    }
}

// Benchmark helper for MusicalWork
#[allow(dead_code)]
pub struct MusicalWorkBenchmarkHelper;

impl BenchmarkHelper<MusicalWork> for MusicalWorkBenchmarkHelper {
    fn benchmark_instance(complexity: u32) -> MusicalWork {
        // Generate complexity-based components
        let title_complexity = complexity / 10;
        let year_complexity = complexity / 100;
        let bpm_complexity = complexity / 50;
        let creators_complexity = complexity / 5;

        // Generate title based on complexity
        let title_len = BenchmarkMapper::complexity_to_string_length(title_complexity, 256).max(1);
        let title = "A"
            .repeat(title_len as usize)
            .as_bytes()
            .to_vec()
            .try_into()
            .unwrap_or_default();

        // Generate ISWC - simplified for benchmark
        let iswc = "T1234567890"
            .as_bytes()
            .to_vec()
            .try_into()
            .unwrap_or_default();

        MusicalWork {
            iswc,
            title,
            creation_year: if year_complexity > 0 {
                Some(1900 + (year_complexity as u16 % 150))
            } else {
                None
            },
            instrumental: Some(complexity % 2 == 0),
            language: if complexity % 3 == 0 {
                Some(Language::English)
            } else {
                None
            },
            bpm: if bpm_complexity > 0 {
                Some(60 + (bpm_complexity as u16 % 180))
            } else {
                None
            },
            key: if complexity % 4 == 0 {
                Some(Key::C)
            } else {
                None
            },
            work_type: if complexity % 5 == 0 {
                Some(MusicalWorkType::Original)
            } else {
                None
            },
            creators: benchmark_creators(creators_complexity),
            classical_info: if complexity > u32::MAX / 2 {
                Some(ClassicalInfo {
                    opus: Some("Op. 1".as_bytes().to_vec().try_into().unwrap_or_default()),
                    catalog_number: Some("K. 1".as_bytes().to_vec().try_into().unwrap_or_default()),
                    number_of_voices: Some(4),
                })
            } else {
                None
            },
        }
    }
}

// Benchmark helper for Recording
#[allow(dead_code)]
pub struct RecordingBenchmarkHelper;

impl BenchmarkHelper<Recording> for RecordingBenchmarkHelper {
    fn benchmark_instance(complexity: u32) -> Recording {
        // Generate complexity-based components
        let general_complexity = complexity / 10;
        let collections_complexity = complexity / 20;

        // Generate collections
        let producers_count =
            BenchmarkMapper::complexity_to_collection_size(collections_complexity, 64);
        let performers_count = BenchmarkMapper::complexity_to_collection_size(
            collections_complexity.saturating_mul(2),
            256,
        );
        let contributors_count = BenchmarkMapper::complexity_to_collection_size(
            collections_complexity.saturating_mul(3),
            256,
        );
        let aliases_count =
            BenchmarkMapper::complexity_to_collection_size(collections_complexity / 2, 16);
        let genres_count =
            BenchmarkMapper::complexity_to_collection_size(collections_complexity / 5, 5).max(1);

        // Generate title
        let _title_len =
            BenchmarkMapper::complexity_to_string_length(general_complexity, 256).max(1);
        let title = "Recording Title"
            .as_bytes()
            .to_vec()
            .try_into()
            .unwrap_or_default();

        // Generate ISRC - simplified for benchmark
        let isrc = "USABC2312345"
            .as_bytes()
            .to_vec()
            .try_into()
            .unwrap_or_default();

        Recording {
            isrc,
            musical_work: general_complexity as u64,
            artist: benchmark_party_id(complexity),
            producers: (0..producers_count)
                .map(|i| benchmark_party_id(complexity.saturating_add(i)))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_default(),
            performers: (0..performers_count)
                .map(|i| benchmark_party_id(complexity.saturating_add(i * 2)))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_default(),
            contributors: (0..contributors_count)
                .map(|i| benchmark_party_id(complexity.saturating_add(i * 3)))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_default(),
            title,
            title_aliases: (0..aliases_count)
                .map(|i| {
                    format!("Alias {}", i)
                        .as_bytes()
                        .to_vec()
                        .try_into()
                        .unwrap_or_default()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_default(),
            recording_year: if general_complexity > 0 {
                Some(2000 + (general_complexity as u16 % 25))
            } else {
                None
            },
            genres: {
                let mut genres = Vec::new();
                for _ in 0..genres_count {
                    genres.push(GenreId::Pop); // Use a default genre for benchmarking
                }
                genres.try_into().unwrap_or_default()
            },
            version: if complexity % 3 == 0 {
                Some(RecordingVersion::Original)
            } else {
                None
            },
            duration: if general_complexity > 0 {
                Some(180 + (general_complexity as u16 % 300))
            } else {
                None
            },
            bpm: if general_complexity > 0 {
                Some(60 + (general_complexity as u16 % 180))
            } else {
                None
            },
            key: if complexity % 4 == 0 {
                Some(Key::C)
            } else {
                None
            },
            recording_place: if complexity % 5 == 0 {
                Some(
                    "Studio A"
                        .as_bytes()
                        .to_vec()
                        .try_into()
                        .unwrap_or_default(),
                )
            } else {
                None
            },
            mixing_place: if complexity % 6 == 0 {
                Some(
                    "Mix Studio"
                        .as_bytes()
                        .to_vec()
                        .try_into()
                        .unwrap_or_default(),
                )
            } else {
                None
            },
            mastering_place: if complexity % 7 == 0 {
                Some(
                    "Mastering Suite"
                        .as_bytes()
                        .to_vec()
                        .try_into()
                        .unwrap_or_default(),
                )
            } else {
                None
            },
        }
    }
}

// Benchmark helper for Release
#[allow(dead_code)]
pub struct ReleaseBenchmarkHelper;

impl BenchmarkHelper<Release> for ReleaseBenchmarkHelper {
    fn benchmark_instance(complexity: u32) -> Release {
        let general_complexity = complexity / 10;
        let collections_complexity = complexity / 20;

        // Generate collections
        let producers_count =
            BenchmarkMapper::complexity_to_collection_size(collections_complexity, 256);
        let recordings_count = BenchmarkMapper::complexity_to_collection_size(
            collections_complexity.saturating_mul(2),
            1024,
        )
        .max(1);
        let cover_contributors_count =
            BenchmarkMapper::complexity_to_collection_size(collections_complexity / 2, 64);
        let aliases_count =
            BenchmarkMapper::complexity_to_collection_size(collections_complexity / 3, 16);

        // Generate EAN - simplified for benchmark
        let ean_upc = "1234567890123"
            .as_bytes()
            .to_vec()
            .try_into()
            .unwrap_or_default();

        // Generate title
        let title = "Release Title"
            .as_bytes()
            .to_vec()
            .try_into()
            .unwrap_or_default();

        Release {
            ean_upc,
            creator: benchmark_party_id(complexity),
            producers: (0..producers_count)
                .map(|i| ProducerInfo {
                    producer_id: benchmark_party_id(complexity.saturating_add(i)),
                    catalog_nb: if i % 2 == 0 {
                        Some(
                            format!("CAT{:04}", i)
                                .as_bytes()
                                .to_vec()
                                .try_into()
                                .unwrap_or_default(),
                        )
                    } else {
                        None
                    },
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_default(),
            recordings: (0..recordings_count)
                .map(|i| BenchmarkMapper::complexity_to_id(complexity, i))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_default(),
            distributor_name: "Distributor"
                .as_bytes()
                .to_vec()
                .try_into()
                .unwrap_or_default(),
            manufacturer_name: "Manufacturer"
                .as_bytes()
                .to_vec()
                .try_into()
                .unwrap_or_default(),
            cover_contributors: (0..cover_contributors_count)
                .map(|i| {
                    format!("Cover Artist {}", i)
                        .as_bytes()
                        .to_vec()
                        .try_into()
                        .unwrap_or_default()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_default(),
            title,
            title_aliases: (0..aliases_count)
                .map(|i| {
                    format!("Release Alias {}", i)
                        .as_bytes()
                        .to_vec()
                        .try_into()
                        .unwrap_or_default()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_default(),
            release_type: ReleaseType::Lp,
            format: ReleaseFormat::Cd,
            packaging: ReleasePackaging::JewelCase,
            date: Date {
                year: 2000 + (general_complexity as u16 % 25),
                month: 1 + (general_complexity as u8 % 12),
                day: 1 + (general_complexity as u8 % 28),
            },
            country: Country::US,
            status: ReleaseStatus::Official,
        }
    }
}

// Benchmark helper for PartyId
#[allow(dead_code)]
pub struct PartyIdBenchmarkHelper;

impl BenchmarkHelper<PartyId> for PartyIdBenchmarkHelper {
    fn benchmark_instance(complexity: u32) -> PartyId {
        benchmark_party_id(complexity)
    }
}

// Simple benchmark implementation for MiddsString
#[allow(dead_code)]
pub struct MiddsStringBenchmarkHelper<const BOUND: u32>;

impl<const BOUND: u32> BenchmarkHelper<MiddsString<BOUND>> for MiddsStringBenchmarkHelper<BOUND> {
    fn benchmark_instance(complexity: u32) -> MiddsString<BOUND> {
        let length = BenchmarkMapper::complexity_to_string_length(complexity, BOUND).max(1);
        "A".repeat(length as usize)
            .as_bytes()
            .to_vec()
            .try_into()
            .unwrap_or_default()
    }
}
