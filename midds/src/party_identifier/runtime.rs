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

use crate::{
    Midds,
    party_identifier::types::{ArtistAliases, ArtistFullName, ArtistGender, ArtistType},
};

use frame_support::{dispatch::DispatchResult, sp_runtime::RuntimeDebug};
use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::DispatchError;

#[cfg(feature = "js")]
use bomboni_wasm::Wasm;
#[cfg(feature = "js")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "js")]
use wasm_bindgen::prelude::*;

/// Party identifier validation errors
#[derive(RuntimeDebug)]
pub enum PartyIdentifierError {
    /// Neither IPI nor ISNI was provided
    MissingIdentifiers,
}

pub use super::types::{EntityName, EntityType, Ipi, Isni};

/// Core struct used to uniquely identify a music industry party (either a person or an entity)
/// as a MIDDS.
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
pub struct PartyIdentifier {
    /// ISNI identifier (max 16 characters). Optional but either `isni` or `ipi`
    /// must be provided.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub isni: Option<Isni>,
    /// IPI identifier (11-digit u64). Optional but either `isni` or `ipi` must
    /// be provided.
    pub ipi: Option<Ipi>,
    /// Variant defining if the party is a `Artist` or an `Entity` with data.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub party_type: PartyType,
}

// Implements the `Midds` trait to integrate this type into the MIDDS protocol.
impl Midds for PartyIdentifier {
    const NAME: &'static str = "Party Identifier";

    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = PartyIdentifierBenchmarkHelper;

    fn validate(&self) -> DispatchResult {
        if self.isni.is_some() || self.ipi.is_some() {
            Ok(())
        } else {
            Err(DispatchError::Other(
                "At least one identifier (IPI or ISNI) must be provided",
            ))
        }
    }
}

#[cfg(feature = "runtime-benchmarks")]
pub struct PartyIdentifierBenchmarkHelper;

#[cfg(feature = "runtime-benchmarks")]
impl crate::benchmarking::BenchmarkHelperT<PartyIdentifier>
    for PartyIdentifierBenchmarkHelper
{
    fn min_size() -> PartyIdentifier {
        use crate::party_identifier::types::*;
        use core::str::FromStr;

        PartyIdentifier {
            isni: None,
            ipi: Some(123456789),
            party_type: PartyType::Artist(Artist {
                full_name: ArtistFullName::from_str("A").unwrap(),
                aliases: ArtistAliases::new(),
                artist_type: ArtistType::Person,
                gender: None,
            }),
        }
    }

    fn max_size() -> PartyIdentifier {
        use crate::party_identifier::types::*;
        use core::str::FromStr;

        // Helper to create maximum length strings
        let max_isni = "1".repeat(12); // 12 chars max for ISNI
        let max_full_name = "A".repeat(256); // 256 chars max

        // Helper to create maximum aliases
        let mut aliases = ArtistAliases::new();
        // Fill to maximum capacity (12 aliases)
        for i in 0..12 {
            let alias_content = format!("Alias{i}");
            if let Ok(alias) = ArtistAlias::from_str(&alias_content) {
                if aliases.push(alias).is_err() {
                    break;
                }
            }
        }

        PartyIdentifier {
            isni: Some(Isni::from_str(&max_isni).unwrap()),
            ipi: Some(u64::MAX),
            party_type: PartyType::Artist(Artist {
                full_name: ArtistFullName::from_str(&max_full_name).unwrap(),
                aliases,
                artist_type: ArtistType::Other,
                gender: Some(ArtistGender::Neither),
            }),
        }
    }

    fn variable_size(complexity: f32) -> PartyIdentifier {
        use crate::benchmarking::utils::*;
        use crate::party_identifier::types::*;
        use core::str::FromStr;

        // Calculate dynamic lengths based on complexity
        let name_len =
            target_length_for_complexity::<frame_support::traits::ConstU32<256>>(complexity);
        let alias_len =
            target_length_for_complexity::<frame_support::traits::ConstU32<128>>(complexity);
        let isni_len = (8.0 + complexity * 4.0) as usize; // 8-12 characters

        // Create complexity-based ISNI
        let isni_content = if complexity > 0.5 {
            Some(Isni::from_str(&"1".repeat(isni_len.clamp(8, 12))).ok())
        } else {
            None
        };

        // Create complexity-based full name
        let full_name_content = if complexity > 0.8 {
            format!(
                "Complex Artist Name {}",
                "X".repeat(name_len.saturating_sub(20))
            )
        } else if complexity > 0.5 {
            format!("Artist {}", "A".repeat(name_len.saturating_sub(8)))
        } else {
            "A".repeat(name_len.max(1))
        };

        // Create complexity-based entity name
        let entity_name_content = if complexity > 0.6 {
            format!("Entity {}", "E".repeat(alias_len.saturating_sub(8)))
        } else {
            "E".repeat(alias_len.max(1))
        };

        // Create aliases based on complexity
        let mut aliases = ArtistAliases::new();
        if complexity > 0.3 {
            let alias_count = (complexity * 12.0) as usize;
            for i in 0..alias_count {
                let alias_content = if complexity > 0.7 {
                    format!("Stage{}{}", i, "N".repeat(alias_len.saturating_sub(8)))
                } else {
                    format!("Alias{i}")
                };
                if let Ok(alias) = ArtistAlias::from_str(&alias_content) {
                    if aliases.push(alias).is_err() {
                        break;
                    }
                }
            }
        }

        PartyIdentifier {
            isni: isni_content.unwrap_or(None),
            ipi: Some((complexity * 99999999999.0) as u64 + 100000000), // Ensure valid IPI range
            party_type: if complexity > 0.6 {
                PartyType::Entity(Entity {
                    name: EntityName::from_str(&entity_name_content)
                        .unwrap_or_else(|_| EntityName::from_str("E").unwrap()),
                    entity_type: if complexity > 0.8 {
                        EntityType::Producer
                    } else {
                        EntityType::Publisher
                    },
                })
            } else {
                PartyType::Artist(Artist {
                    full_name: ArtistFullName::from_str(&full_name_content)
                        .unwrap_or_else(|_| ArtistFullName::from_str("A").unwrap()),
                    aliases,
                    artist_type: if complexity > 0.8 {
                        ArtistType::Orchestra
                    } else if complexity > 0.6 {
                        ArtistType::Group
                    } else if complexity > 0.4 {
                        ArtistType::Choir
                    } else {
                        ArtistType::Person
                    },
                    gender: if complexity > 0.4 {
                        Some(if complexity > 0.7 {
                            ArtistGender::Neither
                        } else if complexity > 0.55 {
                            ArtistGender::Female
                        } else {
                            ArtistGender::Male
                        })
                    } else {
                        None
                    },
                })
            },
        }
    }
}

/// Enum representing whether a party is a person or an entity.
///
/// Used to branch logic and data structure based on the nature of the party.
#[derive(
    RuntimeDebug,
    Clone,
    PartialEq,
    Eq,
    Encode,
    Decode,
    DecodeWithMemTracking,
    MaxEncodedLen,
    TypeInfo,
)]
#[cfg_attr(feature = "js", derive(Wasm, Deserialize, Serialize))]
#[cfg_attr(feature = "js", serde(tag = "type", content = "data"))]
#[cfg_attr(feature = "js", wasm(wasm_abi))]
pub enum PartyType {
    Artist(Artist),
    Entity(Entity),
}

/// Data structure representing an individual involved in, as example, music production or rights.
#[derive(
    Clone,
    PartialEq,
    Eq,
    Encode,
    Decode,
    DecodeWithMemTracking,
    MaxEncodedLen,
    RuntimeDebug,
    TypeInfo,
)]
#[cfg_attr(feature = "js", wasm_bindgen(inspectable))]
#[cfg_attr(feature = "js", derive(Deserialize, Serialize))]
pub struct Artist {
    /// Legal name of the artist.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub full_name: ArtistFullName,
    /// Alternative names/stage names.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub aliases: ArtistAliases,
    /// Indicates if this is a solo artist or a group.
    pub artist_type: ArtistType,
    /// Declared gender identity.
    pub gender: Option<ArtistGender>,
}

/// Data structure representing an organization or company involved in music industry.
#[derive(
    Clone,
    PartialEq,
    Eq,
    Encode,
    Decode,
    DecodeWithMemTracking,
    MaxEncodedLen,
    RuntimeDebug,
    TypeInfo,
)]
#[cfg_attr(feature = "js", wasm_bindgen(inspectable))]
#[cfg_attr(feature = "js", derive(Deserialize, Serialize))]
pub struct Entity {
    /// Entity Name.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub name: EntityName,
    /// The role played by the organization (e.g., publisher, producer).
    pub entity_type: EntityType,
}
