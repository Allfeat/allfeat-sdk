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

#[cfg(not(feature = "std"))]
use alloc::{format, string::{String, ToString}, vec::Vec};
#[cfg(feature = "std")]
use alloc::{format, string::String, vec::Vec};
use midds_types_codegen::{midds_collection, midds_string};
#[cfg(feature = "js")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "js")]
use wasm_bindgen::prelude::*;

use frame_support::sp_runtime::RuntimeDebug;
use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

/// Full legal name of an artist.
#[midds_string(256)]
pub struct ArtistFullName;

/// An alias or stage name for an artist.
#[midds_string(128)]
pub struct ArtistAlias;

/// Name of a legal entity (e.g., label, publisher, rights organization).
#[midds_string(128)]
pub struct EntityName;

/// List of aliases for an artist.
#[midds_collection(ArtistAlias, 12)]
pub struct ArtistAliases;

/// ISNI (International Standard Name Identifier) code.
/// Format: 16 digits (stored without spaces)
#[midds_string(16, regex = r"^\d{15}[\dX]$")]
pub struct Isni;

/// IPI (Interested Parties Information) code.
/// Stored as u64 for efficient numeric operations.
pub type Ipi = u64;

/// Identifies the type of artist, specifying whether the artist is a solo performer or a collective entity.
///
/// - `Person`: A single individual artist.
/// - `Group`: A group of people, such as a band or duo.
/// - `Orchestra`: A large instrumental ensemble, typically classical.
/// - `Choir`: A group of singers performing together, typically choral music.
/// - `Other`: Any other type of artist not covered by the above categories.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Encode,
    Decode,
    DecodeWithMemTracking,
    MaxEncodedLen,
    RuntimeDebug,
    TypeInfo,
)]
#[cfg_attr(feature = "js", wasm_bindgen)]
#[cfg_attr(feature = "js", derive(Deserialize, Serialize))]
pub enum ArtistType {
    Person,
    Group,
    Orchestra,
    Choir,
    Other,
}

/// Identifies the the type of an organization.
/// - `Publisher`: responsible for rights and licensing.
/// - `Producer`: oversees the creation of musical works.
/// - `DistribAggr`: distributes or aggregates content to platforms.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Encode,
    Decode,
    DecodeWithMemTracking,
    MaxEncodedLen,
    RuntimeDebug,
    TypeInfo,
)]
#[cfg_attr(feature = "js", wasm_bindgen)]
#[cfg_attr(feature = "js", derive(Deserialize, Serialize))]
pub enum EntityType {
    Publisher,
    Producer,
}

/// Declared gender identity of a artist.
/// - `Male`: male.
/// - `Female`: female.
/// - `Neither`: unspecified, non-binary, or not disclosed.
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
#[cfg_attr(feature = "js", derive(Deserialize, Serialize))]
pub enum ArtistGender {
    Male,
    Female,
    Neither,
}


