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

use frame_support::sp_runtime::RuntimeDebug;
use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

#[cfg(feature = "js")]
use wasm_bindgen::prelude::*;

use crate::{
    Midds, MiddsId,
    shared::{
        Key,
        conversion::{Validatable, ValidationError},
    },
    track::types::{
        Isrc, TrackBeatsPerMinute, TrackContributors, TrackDuration, TrackGenres,
        TrackMasteringPlace, TrackMixingPlace, TrackPerformers, TrackProducers, TrackRecordYear,
        TrackRecordingPlace, TrackTitle, TrackTitleAliases, TrackVersion,
    },
};

/// A Track represents a specific recorded performance or production
/// of a musical work. It links metadata such as contributors,
/// recording details, and identification codes.
#[derive(
    Clone,
    Eq,
    PartialEq,
    Encode,
    Decode,
    TypeInfo,
    DecodeWithMemTracking,
    MaxEncodedLen,
    RuntimeDebug,
)]
#[cfg_attr(feature = "js", wasm_bindgen(inspectable))]
pub struct Track {
    /// ISRC (International Standard Recording Code) that uniquely identifies this recording.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub isrc: Isrc,

    /// The linked musical work this track is based on (must refer to a registered MIDDS).
    pub musical_work: MiddsId,

    /// Main artist MIDDS identifier (typically the primary performer).
    pub artist: MiddsId,

    /// List of producer MIDDS identifiers who participated in the production.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub producers: TrackProducers,

    /// List of performer MIDDS identifiers who contributed to the performance.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub performers: TrackPerformers,

    /// Additional contributors (e.g., sound engineers, featured artists).
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub contributors: TrackContributors,

    /// Main title of the track.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub title: TrackTitle,

    /// Optional list of alternative titles for the track.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub title_aliases: TrackTitleAliases,

    /// Year the track was recorded (4-digit Gregorian year).
    pub recording_year: Option<TrackRecordYear>,

    /// Music genres attributed to this recording.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub genres: TrackGenres,

    /// Version or type of the track (e.g., Remix, Acoustic, Live).
    pub version: Option<TrackVersion>,

    /// Duration of the track in seconds.
    pub duration: Option<TrackDuration>,

    /// Beats per minute (BPM), representing the tempo of the track.
    pub bpm: Option<TrackBeatsPerMinute>,

    /// Musical key (e.g., C, G#, etc.) the track is in.
    pub key: Option<Key>,

    /// Free-text field indicating where the recording took place.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub recording_place: Option<TrackRecordingPlace>,

    /// Free-text field indicating where the mixing of the track occurred.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub mixing_place: Option<TrackMixingPlace>,

    /// Free-text field indicating where the mastering of the track occurred.
    #[cfg_attr(feature = "js", wasm_bindgen(getter_with_clone))]
    pub mastering_place: Option<TrackMasteringPlace>,
}

// Implements the `Midds` trait, marking this struct as a MIDDS object.
impl Midds for Track {
    const NAME: &'static str = "Track";

    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = TrackBenchmarkHelper;
}

/// Validation implementation for Track
impl Validatable for Track {
    type Error = ValidationError;

    fn validate(&self) -> Result<(), Self::Error> {
        // Validate ISRC format - the macro-generated type already handles validation
        if self.isrc.is_empty() {
            return Err(ValidationError::invalid_format(
                "ISRC",
                "ISRC cannot be empty",
            ));
        }

        // Validate title is not empty
        if self.title.is_empty() {
            return Err(ValidationError::invalid_format(
                "Title",
                "Title cannot be empty",
            ));
        }

        // Validate recording year if present
        if let Some(year) = self.recording_year {
            if !(1000..=9999).contains(&year) {
                return Err(ValidationError::invalid_format(
                    "RecordingYear",
                    "Year must be 4 digits",
                ));
            }
        }

        // Validate duration if present
        if let Some(duration) = self.duration {
            if duration == 0 {
                return Err(ValidationError::invalid_format(
                    "Duration",
                    "Duration must be greater than 0",
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

        // Validate MIDDS ID references are not zero
        if self.musical_work == 0 {
            return Err(ValidationError::invalid_format(
                "MusicalWork",
                "Musical work ID cannot be 0",
            ));
        }
        if self.artist == 0 {
            return Err(ValidationError::invalid_format(
                "Artist",
                "Artist ID cannot be 0",
            ));
        }

        // Validate all producer IDs are not zero
        for &producer_id in &self.producers {
            if producer_id == 0 {
                return Err(ValidationError::invalid_format(
                    "Producers",
                    "Producer ID cannot be 0",
                ));
            }
        }

        // Validate all performer IDs are not zero
        for &performer_id in &self.performers {
            if performer_id == 0 {
                return Err(ValidationError::invalid_format(
                    "Performers",
                    "Performer ID cannot be 0",
                ));
            }
        }

        // Validate all contributor IDs are not zero
        for &contributor_id in &self.contributors {
            if contributor_id == 0 {
                return Err(ValidationError::invalid_format(
                    "Contributors",
                    "Contributor ID cannot be 0",
                ));
            }
        }

        // Validate title aliases are not empty
        for alias in &self.title_aliases {
            if alias.is_empty() {
                return Err(ValidationError::invalid_format(
                    "TitleAliases",
                    "Title alias cannot be empty",
                ));
            }
        }

        Ok(())
    }
}

#[cfg(feature = "runtime-benchmarks")]
pub struct TrackBenchmarkHelper;

#[cfg(feature = "runtime-benchmarks")]
impl crate::benchmarking::BenchmarkHelperT<Track> for TrackBenchmarkHelper {
    fn min_size() -> Track {
        use crate::track::types::*;
        use core::str::FromStr;

        Track {
            isrc: Isrc::from_str("US1234567890").unwrap(),
            musical_work: 1,
            artist: 1,
            producers: TrackProducers::new(),
            performers: TrackPerformers::new(),
            contributors: TrackContributors::new(),
            title: TrackTitle::from_str("T").unwrap(),
            title_aliases: TrackTitleAliases::new(),
            recording_year: None,
            genres: TrackGenres::new(),
            version: None,
            duration: None,
            bpm: None,
            key: None,
            recording_place: None,
            mixing_place: None,
            mastering_place: None,
        }
    }

    fn max_size() -> Track {
        use crate::track::types::*;
        use core::str::FromStr;

        // Helper to create maximum length strings
        let max_isrc = "US".to_string() + &"A".repeat(10); // 12 chars max
        let max_title = "T".repeat(256); // 256 chars max
        let max_place = "P".repeat(256); // 256 chars max

        // Helper to create maximum collections
        let mut producers = TrackProducers::new();
        // Fill to maximum capacity (64 producers)
        for i in 0..64 {
            if producers.push(u64::MAX - i as u64).is_err() {
                break;
            }
        }

        let mut performers = TrackPerformers::new();
        // Fill to maximum capacity (256 performers)
        for i in 0..256 {
            if performers.push(u64::MAX - i as u64).is_err() {
                break;
            }
        }

        let mut contributors = TrackContributors::new();
        // Fill to maximum capacity (256 contributors)
        for i in 0..256 {
            if contributors.push(u64::MAX - i as u64).is_err() {
                break;
            }
        }

        let mut title_aliases = TrackTitleAliases::new();
        // Fill to maximum capacity (16 aliases)
        for i in 0..16 {
            let alias_title = TrackTitle::from_str(&format!("Alias{i}")).unwrap();
            if title_aliases.push(alias_title).is_err() {
                break;
            }
        }

        let mut genres = TrackGenres::new();
        // Fill to maximum capacity (5 genres)
        for _ in 0..5 {
            if genres
                .push(allfeat_music_genres::GenreId::Electropop)
                .is_err()
            {
                break;
            }
        }

        Track {
            isrc: Isrc::from_str(&max_isrc).unwrap(),
            musical_work: u64::MAX,
            artist: u64::MAX,
            producers,
            performers,
            contributors,
            title: TrackTitle::from_str(&max_title).unwrap(),
            title_aliases,
            recording_year: Some(2024),
            genres,
            version: Some(TrackVersion::Edit),
            duration: Some(u16::MAX),
            bpm: Some(u16::MAX),
            key: Some(crate::shared::Key::Cs),
            recording_place: Some(TrackRecordingPlace::from_str(&max_place).unwrap()),
            mixing_place: Some(TrackMixingPlace::from_str(&max_place).unwrap()),
            mastering_place: Some(TrackMasteringPlace::from_str(&max_place).unwrap()),
        }
    }

    fn variable_size(complexity: f32) -> Track {
        use crate::benchmarking::utils::*;
        use crate::track::types::*;
        use core::str::FromStr;

        // Calculate dynamic lengths based on complexity
        let title_len =
            target_length_for_complexity::<frame_support::traits::ConstU32<256>>(complexity);
        let isrc_len = (7.0 + complexity * 5.0) as usize; // 7-12 characters

        // Create complexity-based ISRC
        let isrc_base = format!(
            "US{:0width$}",
            1234567890u64,
            width = isrc_len.saturating_sub(2).min(10)
        );
        let isrc_content = if isrc_base.len() > 12 {
            isrc_base[..12].to_string()
        } else {
            isrc_base
        };

        // Create complexity-based title
        let title_content = if complexity > 0.8 {
            format!(
                "Complex Track Title {}",
                "X".repeat(title_len.saturating_sub(20))
            )
        } else if complexity > 0.5 {
            format!("Track {}", "T".repeat(title_len.saturating_sub(7)))
        } else {
            "T".repeat(title_len.max(1))
        };

        // Create producers based on complexity
        let mut producers = TrackProducers::new();
        let producer_count = (complexity * 64.0) as usize;
        for i in 0..producer_count {
            if producers.push(i as u64 + 1).is_err() {
                break;
            }
        }

        // Create performers based on complexity
        let mut performers = TrackPerformers::new();
        let performer_count = (complexity * 256.0) as usize;
        for i in 0..performer_count {
            if performers.push(i as u64 + 1000).is_err() {
                break;
            }
        }

        // Create contributors based on complexity
        let mut contributors = TrackContributors::new();
        let contributor_count = (complexity * 128.0) as usize; // Half of max for variety
        for i in 0..contributor_count {
            if contributors.push(i as u64 + 2000).is_err() {
                break;
            }
        }

        // Create title aliases based on complexity
        let mut title_aliases = TrackTitleAliases::new();
        if complexity > 0.4 {
            let alias_count = (complexity * 16.0) as usize;
            for i in 0..alias_count {
                let alias_content = format!("Alias{i}");
                if let Ok(alias) = TrackTitle::from_str(&alias_content) {
                    if title_aliases.push(alias).is_err() {
                        break;
                    }
                }
            }
        }

        // Create genres based on complexity
        let mut genres = TrackGenres::new();
        if complexity > 0.3 {
            let genre_count = (complexity * 5.0) as usize;
            for _ in 0..genre_count {
                if genres
                    .push(allfeat_music_genres::GenreId::Electropop)
                    .is_err()
                {
                    break;
                }
            }
        }

        // Create place names based on complexity
        let recording_place = if complexity > 0.7 {
            let place_len = ((complexity * 100.0) as usize + 1).min(256);
            Some(TrackRecordingPlace::from_str(&"R".repeat(place_len)).ok())
        } else {
            None
        };

        let mixing_place = if complexity > 0.8 {
            let place_len = ((complexity * 100.0) as usize + 1).min(256);
            Some(TrackMixingPlace::from_str(&"M".repeat(place_len)).ok())
        } else {
            None
        };

        let mastering_place = if complexity > 0.9 {
            let place_len = ((complexity * 100.0) as usize + 1).min(256);
            Some(TrackMasteringPlace::from_str(&"S".repeat(place_len)).ok())
        } else {
            None
        };

        // Build the final Track
        Track {
            isrc: Isrc::from_str(&isrc_content)
                .unwrap_or_else(|_| Isrc::from_str("US1234567890").unwrap()),
            musical_work: (complexity * 1000000.0) as u64 + 1,
            artist: (complexity * 1000000.0) as u64 + 1,
            producers,
            performers,
            contributors,
            title: TrackTitle::from_str(&title_content)
                .unwrap_or_else(|_| TrackTitle::from_str("T").unwrap()),
            title_aliases,
            recording_year: if complexity > 0.2 {
                Some((1900.0 + complexity * 124.0) as u16)
            } else {
                None
            },
            genres,
            version: if complexity > 0.4 {
                Some(TrackVersion::Original)
            } else {
                None
            },
            duration: if complexity > 0.3 {
                Some((complexity * 600.0) as u16)
            } else {
                None
            },
            bpm: if complexity > 0.5 {
                Some((complexity * 200.0 + 60.0) as u16)
            } else {
                None
            },
            key: if complexity > 0.6 {
                Some(crate::shared::Key::C)
            } else {
                None
            },
            recording_place: recording_place.unwrap_or(None),
            mixing_place: mixing_place.unwrap_or(None),
            mastering_place: mastering_place.unwrap_or(None),
        }
    }
}
