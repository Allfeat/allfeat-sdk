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

use allfeat_music_genres::GenreId;
use frame_support::sp_runtime::RuntimeDebug;
use midds_types_codegen::{midds_collection, midds_string};
use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

use crate::MiddsId;

#[cfg(feature = "js")]
use wasm_bindgen::prelude::*;

/// The ISRC (International Standard Recording Code) for uniquely identifying a recording.
#[midds_string(12, regex = r"^[A-Z]{2}[A-Z0-9]{3}\d{7}$")]
pub struct Isrc;

/// The main title of the track.
#[midds_string(256)]
pub struct TrackTitle;

/// Alternative titles or aliases for the track.
#[midds_collection(TrackTitle, 16)]
pub struct TrackTitleAliases;

/// The year the track was recorded (4-digit Gregorian year).
pub type TrackRecordYear = u16;

/// Additional genres that describe the track.
#[midds_collection(GenreId, 5)]
pub struct TrackGenres;

/// Total duration of the track in seconds.
pub type TrackDuration = u16;

/// Beats per minute (BPM) representing the tempo of the track.
pub type TrackBeatsPerMinute = u16;

/// List of producer MIDDS identifiers involved in the track.
#[midds_collection(MiddsId, 64)]
pub struct TrackProducers;

/// List of performer MIDDS identifiers (e.g., singers, instrumentalists).
#[midds_collection(MiddsId, 256)]
pub struct TrackPerformers;

/// List of additional contributors (e.g., engineers, featured artists).
#[midds_collection(MiddsId, 256)]
pub struct TrackContributors;

/// Free-text field indicating the place where the recording took place.
#[midds_string(256)]
pub struct TrackRecordingPlace;

/// Free-text field indicating where the mixing of the track occurred.
#[midds_string(256)]
pub struct TrackMixingPlace;

/// Free-text field indicating where the mastering of the track was performed.
#[midds_string(256)]
pub struct TrackMasteringPlace;

#[repr(u8)]
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Encode,
    Decode,
    DecodeWithMemTracking,
    TypeInfo,
    MaxEncodedLen,
    RuntimeDebug,
)]
#[cfg_attr(feature = "js", wasm_bindgen)]
pub enum TrackVersion {
    /// Original recording version.
    Original = 0,
    /// A recording of a live performance.
    Live = 1,
    /// Shortened version for radio broadcasting.
    RadioEdit = 2,
    /// TV-friendly version used in broadcast.
    TvTrack = 3,
    /// Single release version.
    Single = 4,
    /// A modified or remixed version by another artist or producer.
    Remix = 5,
    /// A cover version performed by a different artist.
    Cover = 6,
    /// An acoustic version, usually unplugged.
    Acoustic = 7,
    /// Vocals-only version.
    Acapella = 8,
    /// Instrument-only version.
    Instrumental = 9,
    /// Version recorded with an orchestral arrangement.
    Orchestral = 10,
    /// Extended version, typically with added sections.
    Extended = 11,
    /// Different take/version of the same session.
    AlternateTake = 12,
    /// Newly recorded version of an existing track.
    ReRecorded = 13,
    /// Karaoke version without lead vocals.
    Karaoke = 14,
    /// Dance version, often remixed for clubs.
    Dance = 15,
    /// Dub version, typically with reverb-heavy effects.
    Dub = 16,
    /// Version with explicit lyrics.
    Clean = 17,
    /// Rehearsal take, often raw or unpolished.
    Rehearsal = 18,
    /// Early or incomplete version of a track.
    Demo = 19,
    /// Generic edit, purpose-specific.
    Edit = 20,
}


