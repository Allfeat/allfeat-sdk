//! Unified error handling for MIDDS V2
//!
//! This module provides a comprehensive error hierarchy that unifies all error types
//! across the MIDDS V2 codebase. Instead of having separate error types for each
//! module, we provide a single `MiddsError` type that can represent any error
//! condition that may occur.
//!
//! # Design Principles
//!
//! - **Unified Interface**: All public APIs return `Result<T, MiddsError>`
//! - **Rich Context**: Errors include detailed context about what went wrong
//! - **Easy Conversion**: Automatic conversion from specific error types
//! - **Categorization**: Errors are categorized by type for easy handling
//! - **User Friendly**: Clear, actionable error messages
//!
//! # Error Categories
//!
//! - **Validation**: Input validation failures (format, range, etc.)
//! - **Capacity**: Runtime capacity limit violations
//! - **NotFound**: Required data not found
//! - **Conversion**: Type conversion failures
//! - **Macro**: Procedural macro compilation errors
//! - **Runtime**: Substrate runtime specific errors
//!
//! # Example Usage
//!
//! ```rust
//! use allfeat_midds_v2::{MiddsError, MiddsResult};
//!
//! fn create_track(title: &str) -> MiddsResult<Track> {
//!     if title.is_empty() {
//!         return Err(MiddsError::validation()
//!             .field("title")
//!             .reason("Title cannot be empty")
//!             .build());
//!     }
//!     // ... track creation logic
//!     Ok(track)
//! }
//! ```

use thiserror::Error;

use core::fmt;

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

// Import unified error types (work in both std and runtime modes)
use crate::{
    musical_work::error::MusicalWorkError, release::error::ReleaseError, track::error::TrackError,
};

/// Result type alias for MIDDS operations
pub type MiddsResult<T> = Result<T, MiddsError>;

/// Unified error type for all MIDDS operations
///
/// This error type provides a comprehensive representation of any error that
/// can occur within the MIDDS system. It includes detailed context about
/// what went wrong and why, making it easier to debug issues and provide
/// meaningful error messages to users.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub struct MiddsError {
    /// The category of error that occurred
    pub kind: ErrorKind,
    /// Human-readable description of what went wrong
    pub message: String,
    /// The field or component that caused the error, if applicable
    pub field: Option<String>,
    /// Additional context about the error
    pub context: ErrorContext,
}

/// Categories of errors that can occur in MIDDS
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    /// Input validation failed
    Validation,
    /// Runtime capacity limit exceeded
    Capacity,
    /// Required data not found
    NotFound,
    /// Type conversion failed
    Conversion,
    /// Procedural macro error
    Macro,
    /// Substrate runtime error
    Runtime,
    /// Configuration error
    Configuration,
    /// Serialization/deserialization error
    Serialization,
}

/// Additional context information for errors
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ErrorContext {
    /// The actual value that caused the error
    pub value: Option<String>,
    /// Expected value or format
    pub expected: Option<String>,
    /// Current limit that was exceeded
    pub limit: Option<usize>,
    /// Actual value that exceeded the limit
    pub actual: Option<usize>,
    /// Additional key-value pairs for context
    pub details: Vec<(String, String)>,
}

impl MiddsError {
    /// Create a new validation error
    pub fn validation() -> ErrorBuilder {
        ErrorBuilder::new(ErrorKind::Validation)
    }

    /// Create a new capacity error
    pub fn capacity() -> ErrorBuilder {
        ErrorBuilder::new(ErrorKind::Capacity)
    }

    /// Create a new not found error
    pub fn not_found() -> ErrorBuilder {
        ErrorBuilder::new(ErrorKind::NotFound)
    }

    /// Create a new conversion error
    pub fn conversion() -> ErrorBuilder {
        ErrorBuilder::new(ErrorKind::Conversion)
    }

    /// Create a new macro error
    pub fn macro_error() -> ErrorBuilder {
        ErrorBuilder::new(ErrorKind::Macro)
    }

    /// Create a new runtime error
    pub fn runtime() -> ErrorBuilder {
        ErrorBuilder::new(ErrorKind::Runtime)
    }

    /// Create a new configuration error
    pub fn configuration() -> ErrorBuilder {
        ErrorBuilder::new(ErrorKind::Configuration)
    }

    /// Create a new serialization error
    pub fn serialization() -> ErrorBuilder {
        ErrorBuilder::new(ErrorKind::Serialization)
    }

    /// Check if this is a validation error
    pub fn is_validation(&self) -> bool {
        matches!(self.kind, ErrorKind::Validation)
    }

    /// Check if this is a capacity error
    pub fn is_capacity(&self) -> bool {
        matches!(self.kind, ErrorKind::Capacity)
    }

    /// Check if this is a runtime error
    pub fn is_runtime(&self) -> bool {
        matches!(self.kind, ErrorKind::Runtime)
    }

    /// Get the field that caused the error, if any
    pub fn field(&self) -> Option<&str> {
        self.field.as_deref()
    }

    /// Get additional context details
    pub fn context(&self) -> &ErrorContext {
        &self.context
    }

    /// Add additional context to this error
    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.details.push((key.into(), value.into()));
        self
    }
}

/// Builder for constructing `MiddsError` instances
///
/// This builder provides a fluent interface for creating detailed error instances
/// with rich context information.
#[derive(Debug)]
pub struct ErrorBuilder {
    kind: ErrorKind,
    message: Option<String>,
    field: Option<String>,
    context: ErrorContext,
}

impl ErrorBuilder {
    /// Create a new error builder with the specified kind
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            message: None,
            field: None,
            context: ErrorContext::default(),
        }
    }

    /// Set the error message
    pub fn message<S: Into<String>>(mut self, message: S) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Set the reason for the error (alias for message)
    pub fn reason<S: Into<String>>(self, reason: S) -> Self {
        self.message(reason)
    }

    /// Set the field that caused the error
    pub fn field<S: Into<String>>(mut self, field: S) -> Self {
        self.field = Some(field.into());
        self
    }

    /// Set the actual value that caused the error
    pub fn value<S: Into<String>>(mut self, value: S) -> Self {
        self.context.value = Some(value.into());
        self
    }

    /// Set the expected value or format
    pub fn expected<S: Into<String>>(mut self, expected: S) -> Self {
        self.context.expected = Some(expected.into());
        self
    }

    /// Set capacity limits (for capacity errors)
    pub fn limits(mut self, limit: usize, actual: usize) -> Self {
        self.context.limit = Some(limit);
        self.context.actual = Some(actual);
        self
    }

    /// Add additional context detail
    pub fn detail<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.context.details.push((key.into(), value.into()));
        self
    }

    /// Build the final error
    pub fn build(self) -> MiddsError {
        let message = self.message.unwrap_or_else(|| match self.kind {
            ErrorKind::Validation => "Validation failed".to_string(),
            ErrorKind::Capacity => "Capacity limit exceeded".to_string(),
            ErrorKind::NotFound => "Required data not found".to_string(),
            ErrorKind::Conversion => "Type conversion failed".to_string(),
            ErrorKind::Macro => "Macro expansion failed".to_string(),
            ErrorKind::Runtime => "Runtime operation failed".to_string(),
            ErrorKind::Configuration => "Configuration error".to_string(),
            ErrorKind::Serialization => "Serialization failed".to_string(),
        });

        MiddsError {
            kind: self.kind,
            message,
            field: self.field,
            context: self.context,
        }
    }
}

impl fmt::Display for MiddsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(field) = &self.field {
            write!(f, "{} (field: {})", self.message, field)?;
        } else {
            write!(f, "{}", self.message)?;
        }

        // Add context information
        let mut context_parts = Vec::new();

        if let Some(value) = &self.context.value {
            context_parts.push(format!("value: {}", value));
        }

        if let Some(expected) = &self.context.expected {
            context_parts.push(format!("expected: {}", expected));
        }

        if let (Some(limit), Some(actual)) = (self.context.limit, self.context.actual) {
            context_parts.push(format!("limit: {}, actual: {}", limit, actual));
        }

        for (key, value) in &self.context.details {
            context_parts.push(format!("{}: {}", key, value));
        }

        if !context_parts.is_empty() {
            write!(f, " [{}]", context_parts.join(", "))?;
        }

        Ok(())
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Validation => write!(f, "validation"),
            ErrorKind::Capacity => write!(f, "capacity"),
            ErrorKind::NotFound => write!(f, "not_found"),
            ErrorKind::Conversion => write!(f, "conversion"),
            ErrorKind::Macro => write!(f, "macro"),
            ErrorKind::Runtime => write!(f, "runtime"),
            ErrorKind::Configuration => write!(f, "configuration"),
            ErrorKind::Serialization => write!(f, "serialization"),
        }
    }
}

/// Common validation error constructors
impl MiddsError {
    /// Create an invalid format error
    pub fn invalid_format<F, V>(field: F, value: V) -> Self
    where
        F: Into<String>,
        V: Into<String>,
    {
        Self::validation()
            .field(field)
            .value(value)
            .reason("Invalid format")
            .build()
    }

    /// Create an invalid length error
    pub fn invalid_length<F, V>(field: F, value: V, expected: &str) -> Self
    where
        F: Into<String>,
        V: Into<String>,
    {
        Self::validation()
            .field(field)
            .value(value)
            .expected(expected)
            .reason("Invalid length")
            .build()
    }

    /// Create an invalid range error
    pub fn invalid_range<F, V>(field: F, value: V, range: &str) -> Self
    where
        F: Into<String>,
        V: Into<String>,
    {
        Self::validation()
            .field(field)
            .value(value)
            .expected(range)
            .reason("Value out of range")
            .build()
    }

    /// Create an empty field error
    pub fn empty_field<F>(field: F) -> Self
    where
        F: Into<String>,
    {
        Self::validation()
            .field(field)
            .reason("Field cannot be empty")
            .build()
    }

    /// Create a capacity exceeded error
    pub fn capacity_exceeded<F>(field: F, limit: usize, actual: usize) -> Self
    where
        F: Into<String>,
    {
        Self::capacity()
            .field(field)
            .limits(limit, actual)
            .reason("Capacity limit exceeded")
            .build()
    }

    /// Create an invalid checksum error
    pub fn invalid_checksum<F, V>(field: F, value: V) -> Self
    where
        F: Into<String>,
        V: Into<String>,
    {
        Self::validation()
            .field(field)
            .value(value)
            .reason("Invalid checksum or check digit")
            .build()
    }

    /// Create an unsupported operation error
    pub fn unsupported_operation<R>(reason: R) -> Self
    where
        R: Into<String>,
    {
        Self::configuration().reason(reason).build()
    }
}

// ================================================================================================
// Automatic Conversions from Existing Error Types
// ================================================================================================

// Musical Work Errors (unified for both std and runtime)
impl From<MusicalWorkError> for MiddsError {
    fn from(error: MusicalWorkError) -> Self {
        use MusicalWorkError;

        match error {
            MusicalWorkError::InvalidTitle(msg) => MiddsError::invalid_format("title", msg),
            MusicalWorkError::InvalidCreationYear(year) => {
                MiddsError::invalid_range("creation_year", year.to_string(), "1000-2100")
            }
            MusicalWorkError::InvalidBpm(bpm) => {
                MiddsError::invalid_range("bpm", bpm.to_string(), "40-300")
            }
            MusicalWorkError::InvalidVoices(voices) => {
                MiddsError::invalid_range("voices", voices.to_string(), "1-100")
            }
            MusicalWorkError::InvalidIswc(msg) => MiddsError::invalid_format("iswc", msg),
            MusicalWorkError::EmptyCreators => MiddsError::empty_field("creators"),
            MusicalWorkError::InvalidOpus(msg) => MiddsError::invalid_format("opus", msg),
            MusicalWorkError::InvalidCatalogNumber(msg) => {
                MiddsError::invalid_format("catalog_number", msg)
            }
            MusicalWorkError::ExceedsCapacity(msg) => MiddsError::runtime()
                .reason(format!("Data exceeds runtime capacity limits: {}", msg))
                .build(),
            MusicalWorkError::InvalidUtf8 => {
                MiddsError::runtime().reason("Invalid UTF-8 data").build()
            }
        }
    }
}

// Track Errors (unified for both std and runtime)
impl From<TrackError> for MiddsError {
    fn from(error: TrackError) -> Self {
        match error {
            TrackError::InvalidTitle(msg) => MiddsError::invalid_format("title", msg),
            TrackError::InvalidIsrc(msg) => MiddsError::invalid_format("isrc", msg),
            TrackError::InvalidDuration(msg) => {
                MiddsError::invalid_range("duration", msg, "positive seconds")
            }
            TrackError::InvalidBpm(msg) => MiddsError::invalid_range("bpm", msg, "40-300"),
            TrackError::InvalidRecordingYear(msg) => {
                MiddsError::invalid_range("recording_year", msg, "1800-2100")
            }
            TrackError::TooManyProducers(actual) => {
                MiddsError::capacity_exceeded("producers", 64, actual)
            }
            TrackError::TooManyPerformers(actual) => {
                MiddsError::capacity_exceeded("performers", 256, actual)
            }
            TrackError::TooManyContributors(actual) => {
                MiddsError::capacity_exceeded("contributors", 256, actual)
            }
            TrackError::TooManyTitleAliases(actual) => {
                MiddsError::capacity_exceeded("title_aliases", 16, actual)
            }
            TrackError::TooManyGenres(actual) => MiddsError::capacity_exceeded("genres", 5, actual),
            TrackError::EmptyTitle => MiddsError::empty_field("title"),
            TrackError::InvalidPlace(msg) => MiddsError::invalid_format("place", msg),
            TrackError::ExceedsCapacity => MiddsError::runtime()
                .reason("Data exceeds runtime capacity limits")
                .build(),
            TrackError::InvalidUtf8 => MiddsError::runtime().reason("Invalid UTF-8 data").build(),
        }
    }
}

// Release Errors (unified for both std and runtime)
impl From<ReleaseError> for MiddsError {
    fn from(error: ReleaseError) -> Self {
        match error {
            ReleaseError::InvalidEan(msg) => MiddsError::invalid_format("ean_upc", msg),
            ReleaseError::EmptyTracks => MiddsError::empty_field("tracks"),
            ReleaseError::InvalidTitle(msg) => MiddsError::invalid_format("title", msg),
            ReleaseError::InvalidDistributor(msg) => {
                MiddsError::invalid_format("distributor_name", msg)
            }
            ReleaseError::InvalidManufacturer(msg) => {
                MiddsError::invalid_format("manufacturer_name", msg)
            }
            ReleaseError::InvalidDate => MiddsError::validation()
                .field("date")
                .reason("Invalid release date")
                .build(),
            ReleaseError::TooManyTracks(actual) => {
                MiddsError::capacity_exceeded("tracks", 1024, actual)
            }
            ReleaseError::TooManyProducers(actual) => {
                MiddsError::capacity_exceeded("producers", 256, actual)
            }
            ReleaseError::TooManyCoverContributors(actual) => {
                MiddsError::capacity_exceeded("cover_contributors", 64, actual)
            }
            ReleaseError::TooManyTitleAliases(actual) => {
                MiddsError::capacity_exceeded("title_aliases", 16, actual)
            }
            ReleaseError::ExceedsCapacity => MiddsError::runtime()
                .reason("Data exceeds runtime capacity limits")
                .build(),
            ReleaseError::InvalidUtf8 => MiddsError::runtime().reason("Invalid UTF-8 data").build(),
        }
    }
}

// ISWC Errors
#[cfg(feature = "std")]
impl From<crate::musical_work::iswc::IswcError> for MiddsError {
    fn from(error: crate::musical_work::iswc::IswcError) -> Self {
        use crate::musical_work::iswc::IswcError;

        match error {
            IswcError::InvalidFormat(msg) => MiddsError::invalid_format("iswc", msg),
            IswcError::InvalidCheckDigit => {
                MiddsError::invalid_checksum("iswc", "check digit verification failed")
            }
            IswcError::InvalidPrefix => MiddsError::validation()
                .field("iswc")
                .expected("T-prefix")
                .reason("ISWC must start with 'T'")
                .build(),
            IswcError::InvalidLength => {
                MiddsError::invalid_length("iswc", "", "exactly 13 characters")
            }
            IswcError::NonNumericWorkCode => MiddsError::validation()
                .field("iswc")
                .reason("Work code must be numeric")
                .build(),
        }
    }
}

// ISRC Errors
#[cfg(feature = "std")]
impl From<crate::track::isrc::IsrcError> for MiddsError {
    fn from(error: crate::track::isrc::IsrcError) -> Self {
        use crate::track::isrc::IsrcError;

        match error {
            IsrcError::InvalidFormat(msg) => MiddsError::invalid_format("isrc", msg),
            IsrcError::InvalidCountryCode(msg) => MiddsError::validation()
                .field("isrc")
                .value(msg)
                .reason("Invalid country code")
                .detail("expected", "2-letter ISO country code")
                .build(),
            IsrcError::InvalidRegistrantCode(msg) => MiddsError::validation()
                .field("isrc")
                .value(msg)
                .reason("Invalid registrant code")
                .detail("expected", "3-character alphanumeric code")
                .build(),
            IsrcError::InvalidYear(msg) => MiddsError::validation()
                .field("isrc")
                .value(msg)
                .reason("Invalid year")
                .detail("expected", "2-digit year")
                .build(),
            IsrcError::InvalidDesignationCode(msg) => MiddsError::validation()
                .field("isrc")
                .value(msg)
                .reason("Invalid designation code")
                .detail("expected", "5-digit designation code")
                .build(),
            IsrcError::InvalidLength => {
                MiddsError::invalid_length("isrc", "", "exactly 12 characters")
            }
            IsrcError::NonAlphanumeric => MiddsError::validation()
                .field("isrc")
                .reason("Must contain only alphanumeric characters")
                .build(),
        }
    }
}

// EAN Errors
#[cfg(feature = "std")]
impl From<crate::release::ean::EanError> for MiddsError {
    fn from(error: crate::release::ean::EanError) -> Self {
        use crate::release::ean::EanError;

        match error {
            EanError::InvalidFormat(msg) => MiddsError::invalid_format("ean_upc", msg),
            EanError::InvalidCheckDigit => {
                MiddsError::invalid_checksum("ean_upc", "check digit verification failed")
            }
            EanError::InvalidLength => {
                MiddsError::invalid_length("ean_upc", "", "8, 12, or 13 digits")
            }
            EanError::NonNumeric => MiddsError::validation()
                .field("ean_upc")
                .reason("Must contain only digits")
                .build(),
        }
    }
}

// Note: Runtime error types have been consolidated into the main error types above.
// This reduces duplication and provides a unified error handling experience.

#[cfg(feature = "runtime")]
impl From<crate::musical_work::iswc::RuntimeIswcError> for MiddsError {
    fn from(error: crate::musical_work::iswc::RuntimeIswcError) -> Self {
        use crate::musical_work::iswc::RuntimeIswcError;

        match error {
            RuntimeIswcError::ExceedsCapacity => MiddsError::runtime()
                .field("iswc")
                .reason("ISWC data exceeds runtime capacity")
                .build(),
            RuntimeIswcError::InvalidUtf8 => MiddsError::runtime()
                .field("iswc")
                .reason("Invalid UTF-8 in ISWC data")
                .build(),
        }
    }
}

#[cfg(feature = "runtime")]
impl From<crate::track::isrc::RuntimeIsrcError> for MiddsError {
    fn from(error: crate::track::isrc::RuntimeIsrcError) -> Self {
        use crate::track::isrc::RuntimeIsrcError;

        match error {
            RuntimeIsrcError::ExceedsCapacity => MiddsError::runtime()
                .field("isrc")
                .reason("ISRC data exceeds runtime capacity")
                .build(),
            RuntimeIsrcError::InvalidUtf8 => MiddsError::runtime()
                .field("isrc")
                .reason("Invalid UTF-8 in ISRC data")
                .build(),
        }
    }
}

#[cfg(feature = "runtime")]
impl From<crate::release::ean::RuntimeEanError> for MiddsError {
    fn from(error: crate::release::ean::RuntimeEanError) -> Self {
        use crate::release::ean::RuntimeEanError;

        match error {
            RuntimeEanError::ExceedsCapacity => MiddsError::runtime()
                .field("ean_upc")
                .reason("EAN/UPC data exceeds runtime capacity")
                .build(),
            RuntimeEanError::InvalidUtf8 => MiddsError::runtime()
                .field("ean_upc")
                .reason("Invalid UTF-8 in EAN/UPC data")
                .build(),
        }
    }
}

// Standard Library Error Conversions
#[cfg(feature = "std")]
impl From<std::io::Error> for MiddsError {
    fn from(error: std::io::Error) -> Self {
        MiddsError::serialization()
            .reason(format!("IO error: {}", error))
            .detail("error_kind", format!("{:?}", error.kind()))
            .build()
    }
}

#[cfg(feature = "std")]
impl From<std::str::Utf8Error> for MiddsError {
    fn from(error: std::str::Utf8Error) -> Self {
        MiddsError::conversion()
            .reason("Invalid UTF-8 sequence")
            .detail("valid_up_to", error.valid_up_to().to_string())
            .build()
    }
}

#[cfg(feature = "std")]
impl From<std::string::FromUtf8Error> for MiddsError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        MiddsError::conversion()
            .reason("Invalid UTF-8 in byte vector")
            .detail("valid_up_to", error.utf8_error().valid_up_to().to_string())
            .build()
    }
}

#[cfg(feature = "std")]
impl From<std::num::ParseIntError> for MiddsError {
    fn from(error: std::num::ParseIntError) -> Self {
        MiddsError::conversion()
            .reason(format!("Failed to parse integer: {}", error))
            .detail("error_kind", format!("{:?}", error.kind()))
            .build()
    }
}

// Regex Error (when available)
#[cfg(feature = "std")]
impl From<regex::Error> for MiddsError {
    fn from(error: regex::Error) -> Self {
        MiddsError::configuration()
            .reason(format!("Regex error: {}", error))
            .build()
    }
}

// For compatibility with Result<T, Box<dyn std::error::Error>>
#[cfg(feature = "std")]
impl From<Box<dyn std::error::Error + Send + Sync>> for MiddsError {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        MiddsError::conversion()
            .reason(format!("Boxed error: {}", error))
            .build()
    }
}

// Procedural Macro Error Conversions
// Note: These are conditionally compiled and only available when these crates are used
// These implementations will be used by the codegen crate for better error reporting

// This function can be used by the codegen crate to create macro errors
impl MiddsError {
    /// Create a macro error from a span and message (for procedural macros)
    pub fn from_macro_span<T: ToString>(span_debug: T, message: impl Into<String>) -> Self {
        MiddsError::macro_error()
            .reason(message.into())
            .detail("span", span_debug.to_string())
            .build()
    }
}

// WebAssembly Error Conversions
#[cfg(feature = "web")]
impl From<wasm_bindgen::JsValue> for MiddsError {
    fn from(error: wasm_bindgen::JsValue) -> Self {
        let error_string = error
            .as_string()
            .unwrap_or_else(|| "Unknown JavaScript error".to_string());

        MiddsError::conversion()
            .reason(format!("JavaScript error: {}", error_string))
            .build()
    }
}

#[cfg(feature = "web")]
impl From<MiddsError> for wasm_bindgen::JsValue {
    fn from(error: MiddsError) -> Self {
        wasm_bindgen::JsValue::from_str(&error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_builder() {
        let error = MiddsError::validation()
            .field("title")
            .value("invalid@title")
            .expected("alphanumeric characters only")
            .reason("Invalid title format")
            .build();

        assert_eq!(error.kind, ErrorKind::Validation);
        assert_eq!(error.field, Some("title".to_string()));
        assert_eq!(error.context.value, Some("invalid@title".to_string()));
        assert_eq!(
            error.context.expected,
            Some("alphanumeric characters only".to_string())
        );
    }

    #[test]
    fn test_capacity_error() {
        let error = MiddsError::capacity_exceeded("tracks", 1024, 1025);

        assert!(error.is_capacity());
        assert_eq!(error.field(), Some("tracks"));
        assert_eq!(error.context.limit, Some(1024));
        assert_eq!(error.context.actual, Some(1025));
    }

    #[test]
    fn test_error_display() {
        let error = MiddsError::invalid_format("iswc", "invalid-code");
        let display = format!("{}", error);

        assert!(display.contains("Invalid format"));
        assert!(display.contains("field: iswc"));
        assert!(display.contains("value: invalid-code"));
    }

    #[test]
    fn test_error_context() {
        let error = MiddsError::validation()
            .field("title")
            .reason("Too long")
            .detail("max_length", "256")
            .detail("actual_length", "300")
            .build();

        assert_eq!(error.context.details.len(), 2);
        assert_eq!(
            error.context.details[0],
            ("max_length".to_string(), "256".to_string())
        );
        assert_eq!(
            error.context.details[1],
            ("actual_length".to_string(), "300".to_string())
        );
    }
}
