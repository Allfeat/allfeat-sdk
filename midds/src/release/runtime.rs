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
use alloc::format;
use frame_support::sp_runtime::RuntimeDebug;
use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

#[cfg(feature = "js")]
use wasm_bindgen::prelude::*;

use crate::{
    Midds, MiddsId,
    release::types::{
        Ean, ReleaseCoverContributors, ReleaseDistributor, ReleaseFormat, ReleaseManufacturer,
        ReleasePackaging, ReleaseProducers, ReleaseStatus, ReleaseTitle, ReleaseTitleAliases,
        ReleaseTracks, ReleaseType,
    },
    shared::{Country, Date},
};

/// A MIDDS representing a musical release (album, EP, single, etc.).
/// It contains metadata and references to related MIDDS like tracks, producers, and artist.
///
/// This structure is used to register and manage a complete music release on-chain.
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
pub struct Release {
    /// EAN or UPC code identifying the release (physical or digital).
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub ean_upc: Ean,

    /// The main artist MIDDS ID associated with this release.
    pub artist: MiddsId,

    /// List of producer MIDDS IDs who contributed to this release.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub producers: ReleaseProducers,

    /// List of track MIDDS IDs that are part of this release.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub tracks: ReleaseTracks,

    /// Name of the distributor responsible for the release.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub distributor_name: ReleaseDistributor,

    /// Name of the manufacturer responsible for physical production.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub manufacturer_name: ReleaseManufacturer,

    /// Contributors to the release cover (designers, photographers, etc.).
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub cover_contributors: ReleaseCoverContributors,

    /// Official title of the release.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub title: ReleaseTitle,

    /// Alternative titles (e.g. translations, acronyms, stylistic variations).
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub title_aliases: ReleaseTitleAliases,

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

impl Midds for Release {
    const NAME: &'static str = "Release";

    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = ReleaseBenchmarkHelper;
}

#[cfg(feature = "runtime-benchmarks")]
pub struct ReleaseBenchmarkHelper;

#[cfg(feature = "runtime-benchmarks")]
impl crate::benchmarking::BenchmarkHelperT<Release> for ReleaseBenchmarkHelper {
    fn min_size() -> Release {
        use crate::release::types::*;
        use core::str::FromStr;

        Release {
            ean_upc: Ean::from_str("1234567890123").unwrap(),
            artist: 1,
            producers: ReleaseProducers::new(),
            tracks: ReleaseTracks::new(),
            distributor_name: ReleaseDistributor::from_str("D").unwrap(),
            manufacturer_name: ReleaseManufacturer::from_str("M").unwrap(),
            cover_contributors: ReleaseCoverContributors::new(),
            title: ReleaseTitle::from_str("T").unwrap(),
            title_aliases: ReleaseTitleAliases::new(),
            release_type: ReleaseType::Single,
            format: ReleaseFormat::Cd,
            packaging: ReleasePackaging::JewelCase,
            status: ReleaseStatus::Official,
            date: crate::shared::Date {
                day: 1,
                month: 1,
                year: 2024,
            },
            country: crate::shared::Country::US,
        }
    }

    fn max_size() -> Release {
        use crate::release::types::*;
        use core::str::FromStr;

        // Helper to create maximum length strings
        let max_ean = "1".repeat(13); // 13 chars max
        let max_title = "T".repeat(256); // 256 chars max
        let max_distributor = "D".repeat(256); // 256 chars max
        let max_manufacturer = "M".repeat(256); // 256 chars max

        // Helper to create maximum collections
        let mut producers = ReleaseProducers::new();
        // Fill to maximum capacity (256 producers)
        for i in 0..256 {
            if producers.push(u64::MAX - i as u64).is_err() {
                break;
            }
        }

        let mut tracks = ReleaseTracks::new();
        // Fill to maximum capacity (1024 tracks)
        for i in 0..1024 {
            if tracks.push(u64::MAX - i as u64).is_err() {
                break;
            }
        }

        let mut cover_contributors = ReleaseCoverContributors::new();
        // Fill to maximum capacity (64 contributors)
        for i in 0..64 {
            let contributor =
                ReleaseCoverContributor::from_str(&format!("Contributor{i}")).unwrap();
            if cover_contributors.push(contributor).is_err() {
                break;
            }
        }

        let mut title_aliases = ReleaseTitleAliases::new();
        // Fill to maximum capacity (16 aliases)
        for i in 0..16 {
            let alias_title = ReleaseTitle::from_str(&format!("Alias{i}")).unwrap();
            if title_aliases.push(alias_title).is_err() {
                break;
            }
        }

        Release {
            ean_upc: Ean::from_str(&max_ean).unwrap(),
            artist: u64::MAX,
            producers,
            tracks,
            distributor_name: ReleaseDistributor::from_str(&max_distributor).unwrap(),
            manufacturer_name: ReleaseManufacturer::from_str(&max_manufacturer).unwrap(),
            cover_contributors,
            title: ReleaseTitle::from_str(&max_title).unwrap(),
            title_aliases,
            release_type: ReleaseType::DoubleLp,
            format: ReleaseFormat::AudioDvd,
            packaging: ReleasePackaging::Digipack,
            status: ReleaseStatus::Cancelled,
            date: crate::shared::Date {
                day: 31,
                month: 12,
                year: 9999,
            },
            country: crate::shared::Country::ZW,
        }
    }

    fn variable_size(complexity: f32) -> Release {
        use crate::benchmarking::utils::*;
        use crate::release::types::*;
        use core::str::FromStr;

        // Calculate dynamic lengths based on complexity
        let title_len =
            target_length_for_complexity::<frame_support::traits::ConstU32<256>>(complexity);
        let distributor_len =
            target_length_for_complexity::<frame_support::traits::ConstU32<256>>(complexity);
        let ean_len = (8.0 + complexity * 5.0) as usize; // 8-13 characters

        // Create complexity-based EAN
        let ean_content = "1".repeat(ean_len.clamp(8, 13));

        // Create complexity-based title
        let title_content = if complexity > 0.8 {
            format!(
                "Complex Release Title {}",
                "X".repeat(title_len.saturating_sub(23))
            )
        } else if complexity > 0.5 {
            format!("Release {}", "T".repeat(title_len.saturating_sub(9)))
        } else {
            "T".repeat(title_len.max(1))
        };

        // Create complexity-based distributor and manufacturer names
        let distributor_content = if complexity > 0.6 {
            format!(
                "Distributor {}",
                "D".repeat(distributor_len.saturating_sub(12))
            )
        } else {
            "D".repeat(distributor_len.max(1))
        };

        let manufacturer_content = if complexity > 0.6 {
            format!(
                "Manufacturer {}",
                "M".repeat(distributor_len.saturating_sub(13))
            )
        } else {
            "M".repeat(distributor_len.max(1))
        };

        // Create producers based on complexity
        let mut producers = ReleaseProducers::new();
        let producer_count = (complexity * 256.0) as usize;
        for i in 0..producer_count {
            if producers.push(i as u64 + 1).is_err() {
                break;
            }
        }

        // Create tracks based on complexity
        let mut tracks = ReleaseTracks::new();
        let track_count = (complexity * 512.0) as usize; // Scale down for realism
        for i in 0..track_count {
            if tracks.push(i as u64 + 1000).is_err() {
                break;
            }
        }

        // Create cover contributors based on complexity
        let mut cover_contributors = ReleaseCoverContributors::new();
        if complexity > 0.5 {
            let contributor_count = (complexity * 64.0) as usize;
            for i in 0..contributor_count {
                let contributor_len = ((complexity * 50.0) as usize + 1).min(256);
                let contributor_content = format!(
                    "Contributor{}{}",
                    i,
                    "C".repeat(contributor_len.saturating_sub(12))
                );
                if let Ok(contributor) = ReleaseCoverContributor::from_str(&contributor_content) {
                    if cover_contributors.push(contributor).is_err() {
                        break;
                    }
                }
            }
        }

        // Create title aliases based on complexity
        let mut title_aliases = ReleaseTitleAliases::new();
        if complexity > 0.4 {
            let alias_count = (complexity * 16.0) as usize;
            for i in 0..alias_count {
                let alias_content = format!("Alias{i}");
                if let Ok(alias) = ReleaseTitle::from_str(&alias_content) {
                    if title_aliases.push(alias).is_err() {
                        break;
                    }
                }
            }
        }

        // Build the final Release
        Release {
            ean_upc: Ean::from_str(&ean_content)
                .unwrap_or_else(|_| Ean::from_str("1234567890123").unwrap()),
            artist: (complexity * 1000000.0) as u64 + 1,
            producers,
            tracks,
            distributor_name: ReleaseDistributor::from_str(&distributor_content)
                .unwrap_or_else(|_| ReleaseDistributor::from_str("D").unwrap()),
            manufacturer_name: ReleaseManufacturer::from_str(&manufacturer_content)
                .unwrap_or_else(|_| ReleaseManufacturer::from_str("M").unwrap()),
            cover_contributors,
            title: ReleaseTitle::from_str(&title_content)
                .unwrap_or_else(|_| ReleaseTitle::from_str("T").unwrap()),
            title_aliases,
            release_type: if complexity > 0.8 {
                ReleaseType::DoubleLp
            } else if complexity > 0.5 {
                ReleaseType::Lp
            } else if complexity > 0.3 {
                ReleaseType::Ep
            } else {
                ReleaseType::Single
            },
            format: if complexity > 0.7 {
                ReleaseFormat::AudioDvd
            } else if complexity > 0.4 {
                ReleaseFormat::Vynil7
            } else {
                ReleaseFormat::Cd
            },
            packaging: if complexity > 0.6 {
                ReleasePackaging::Digipack
            } else {
                ReleasePackaging::JewelCase
            },
            status: if complexity > 0.9 {
                ReleaseStatus::SpecialEdition
            } else if complexity > 0.5 {
                ReleaseStatus::Remastered
            } else {
                ReleaseStatus::Official
            },
            date: crate::shared::Date {
                day: ((complexity * 28.0) as u8 + 1),
                month: ((complexity * 11.0) as u8 + 1),
                year: (2000.0 + complexity * 24.0) as u16,
            },
            country: if complexity > 0.7 {
                crate::shared::Country::FR
            } else if complexity > 0.4 {
                crate::shared::Country::GB
            } else {
                crate::shared::Country::US
            },
        }
    }
}
