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

#[cfg(not(feature = "std"))]
use alloc::{fmt, format, string::{FromUtf8Error, String, ToString}};
#[cfg(feature = "std")]
use std::{fmt, string::FromUtf8Error};

/// Errors that can occur when converting between Substrate and std types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConversionError {
    /// UTF-8 decoding failed for a field
    InvalidUtf8(String, FromUtf8Error),
    /// Validation failed for a field
    ValidationFailed(String, String),
    /// `BoundedVec` capacity overflow
    BoundedVecOverflow(String),
    /// Custom error message
    Custom(String),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionError::InvalidUtf8(field, err) => {
                write!(f, "Invalid UTF-8 in field '{field}': {err}")
            }
            ConversionError::ValidationFailed(field, msg) => {
                write!(f, "Validation failed for field '{field}': {msg}")
            }
            ConversionError::BoundedVecOverflow(field) => {
                write!(f, "BoundedVec capacity overflow in field '{field}'")
            }
            ConversionError::Custom(msg) => write!(f, "{msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ConversionError {}

impl From<ValidationError> for ConversionError {
    fn from(err: ValidationError) -> Self {
        ConversionError::ValidationFailed(
            err.field.unwrap_or_else(|| "Unknown".to_string()),
            err.message,
        )
    }
}

/// Trait for types that can be validated
pub trait Validatable {
    type Error;

    /// Validate the type and return an error if invalid
    ///
    /// # Errors
    ///
    /// Returns validation error if the data structure contains invalid data
    fn validate(&self) -> Result<(), Self::Error>;
}

/// Error type for validation failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub field: Option<String>,
    pub message: String,
}

impl ValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            field: None,
            message: message.into(),
        }
    }

    pub fn field(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: Some(field.into()),
            message: message.into(),
        }
    }

    #[must_use]
    pub fn invalid_format(type_name: &str, value: &str) -> Self {
        Self::new(format!("Invalid {type_name} format: '{value}'"))
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref field) = self.field {
            write!(f, "Validation error in field '{}': {}", field, self.message)
        } else {
            write!(f, "Validation error: {}", self.message)
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ValidationError {}

