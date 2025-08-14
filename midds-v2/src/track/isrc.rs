//! International Standard Recording Code (ISRC) identifier.
//!
//! ISRC is the international standard for uniquely identifying sound recordings
//! and music videos. Each ISRC is a 12-character alphanumeric code.
//!
//! This module provides a comprehensive API for creating, validating, and manipulating
//! ISRC identifiers with proper error handling and format normalization.

use allfeat_midds_v2_codegen::runtime_midds;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

/// International Standard Recording Code (ISRC) identifier.
///
/// Used to uniquely identify sound recordings and music videos globally.
/// ISRC follows the format: `CCOOOYYNNNNN` where:
/// - `CC` - Country code (2 letters)
/// - `OOO` - Registrant code (3 alphanumeric characters)
/// - `YY` - Year of registration (2 digits)
/// - `NNNNN` - Designation code (5 digits)
///
/// # Type Transformation
/// In runtime mode: `String` → `BoundedVec<u8, ConstU32<12>>`
///
/// # Examples
/// ```rust
/// use allfeat_midds_v2::track::isrc::Isrc;
///
/// // Create from valid ISRC
/// let isrc = Isrc::new("USUM71703861").unwrap();
/// assert_eq!(isrc.to_string(), "USUM71703861");
///
/// // Access components
/// assert_eq!(isrc.country_code(), "US");
/// assert_eq!(isrc.registrant_code(), "UM7");
/// assert_eq!(isrc.year_of_registration(), 17);
/// assert_eq!(isrc.designation_code(), "03861");
/// ```
#[runtime_midds]
#[cfg_attr(feature = "web", wasm_bindgen)]
pub struct Isrc(
    /// The ISRC string, limited to 12 characters in runtime mode.
    #[runtime_bound(12)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub String,
);

#[cfg(feature = "std")]
mod api {
    use super::Isrc;
    use regex::Regex;
    use std::fmt;
    use std::str::FromStr;
    use thiserror::Error;

    static ISRC_REGEX: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();

    /// Error types for ISRC operations
    #[derive(Error, Debug, Clone, PartialEq, Eq)]
    pub enum IsrcError {
        /// Invalid ISRC format
        #[error("Invalid ISRC format: {0}")]
        InvalidFormat(String),
        /// Invalid country code
        #[error("Invalid country code: {0}")]
        InvalidCountryCode(String),
        /// Invalid registrant code
        #[error("Invalid registrant code: {0}")]
        InvalidRegistrantCode(String),
        /// Invalid year of registration
        #[error("Invalid year of registration: {0}")]
        InvalidYear(String),
        /// Invalid designation code
        #[error("Invalid designation code: {0}")]
        InvalidDesignationCode(String),
        /// Invalid length
        #[error("ISRC must be exactly 12 characters long")]
        InvalidLength,
        /// Non-alphanumeric characters
        #[error("ISRC must contain only alphanumeric characters")]
        NonAlphanumeric,
    }

    impl Isrc {
        /// Creates a new ISRC from a string with validation.
        ///
        /// # Arguments
        /// * `value` - The ISRC string to validate
        ///
        /// # Returns
        /// * `Ok(Isrc)` if the string is a valid ISRC
        /// * `Err(IsrcError)` if the string is invalid
        ///
        /// # Examples
        /// ```
        /// use allfeat_midds_v2::track::isrc::Isrc;
        ///
        /// let isrc = Isrc::new("USUM71703861").unwrap();
        /// assert_eq!(isrc.to_string(), "USUM71703861");
        /// ```
        pub fn new(value: impl Into<String>) -> Result<Self, IsrcError> {
            let value = value.into();
            Self::validate(&value)?;
            Ok(Self(value.to_string()))
        }

        /// Creates a new ISRC without validation (unsafe).
        ///
        /// # Safety
        /// The caller must ensure that the value is a valid ISRC format.
        pub fn new_unchecked(value: impl Into<String>) -> Self {
            Self(value.into())
        }

        /// Creates an ISRC from components.
        ///
        /// # Arguments
        /// * `country_code` - 2-letter country code (e.g., "US", "GB")
        /// * `registrant_code` - 3-character registrant code (e.g., "UM7")
        /// * `year` - Year of registration (2 digits, e.g., 17 for 2017)
        /// * `designation_code` - 5-digit designation code (e.g., "03861")
        ///
        /// # Returns
        /// * `Ok(Isrc)` with the constructed ISRC
        /// * `Err(IsrcError)` if any component is invalid
        ///
        /// # Examples
        /// ```
        /// use allfeat_midds_v2::track::isrc::Isrc;
        ///
        /// let isrc = Isrc::from_components("US", "UM7", 17, "03861").unwrap();
        /// assert_eq!(isrc.to_string(), "USUM71703861");
        /// ```
        pub fn from_components(
            country_code: &str,
            registrant_code: &str,
            year: u8,
            designation_code: &str,
        ) -> Result<Self, IsrcError> {
            Self::validate_country_code(country_code)?;
            Self::validate_registrant_code(registrant_code)?;
            Self::validate_year(year)?;
            Self::validate_designation_code(designation_code)?;

            let isrc = format!(
                "{}{}{:02}{}",
                country_code, registrant_code, year, designation_code
            );
            Ok(Self(isrc))
        }

        /// Validates an ISRC string format using regex pattern matching.
        ///
        /// # Arguments
        /// * `value` - The string to validate
        ///
        /// # Returns
        /// * `Ok(())` if valid
        /// * `Err(IsrcError)` if invalid
        pub fn validate(value: &str) -> Result<(), IsrcError> {
            let cleaned = value
                .chars()
                .filter(|c| c.is_ascii_alphanumeric())
                .collect::<String>();

            if cleaned.len() != 12 {
                return Err(IsrcError::InvalidLength);
            }

            let isrc_regex = ISRC_REGEX.get_or_init(|| {
                Regex::new(r"^[A-Z]{2}[A-Z0-9]{3}\d{2}\d{5}$").expect("ISRC regex pattern is valid")
            });

            let uppercase = cleaned.to_uppercase();
            if !isrc_regex.is_match(&uppercase) {
                return Err(IsrcError::InvalidFormat(value.to_string()));
            }

            // Validate individual components
            let country_code = &uppercase[0..2];
            let registrant_code = &uppercase[2..5];
            let year_str = &uppercase[5..7];
            let designation_code = &uppercase[7..12];

            Self::validate_country_code(country_code)?;
            Self::validate_registrant_code(registrant_code)?;

            if let Ok(year) = year_str.parse::<u8>() {
                Self::validate_year(year)?;
            } else {
                return Err(IsrcError::InvalidYear(year_str.to_string()));
            }

            Self::validate_designation_code(designation_code)?;

            Ok(())
        }

        /// Validates country code (2 letters).
        fn validate_country_code(code: &str) -> Result<(), IsrcError> {
            if code.len() != 2 {
                return Err(IsrcError::InvalidCountryCode(code.to_string()));
            }
            if !code.chars().all(|c| c.is_ascii_alphabetic()) {
                return Err(IsrcError::InvalidCountryCode(code.to_string()));
            }
            Ok(())
        }

        /// Validates registrant code (3 alphanumeric characters).
        fn validate_registrant_code(code: &str) -> Result<(), IsrcError> {
            if code.len() != 3 {
                return Err(IsrcError::InvalidRegistrantCode(code.to_string()));
            }
            if !code.chars().all(|c| c.is_ascii_alphanumeric()) {
                return Err(IsrcError::InvalidRegistrantCode(code.to_string()));
            }
            Ok(())
        }

        /// Validates year (00-99).
        fn validate_year(year: u8) -> Result<(), IsrcError> {
            if year > 99 {
                return Err(IsrcError::InvalidYear(year.to_string()));
            }
            Ok(())
        }

        /// Validates designation code (5 digits).
        fn validate_designation_code(code: &str) -> Result<(), IsrcError> {
            if code.len() != 5 {
                return Err(IsrcError::InvalidDesignationCode(code.to_string()));
            }
            if !code.chars().all(|c| c.is_ascii_digit()) {
                return Err(IsrcError::InvalidDesignationCode(code.to_string()));
            }
            Ok(())
        }

        /// Returns the country code (first 2 characters).
        pub fn country_code(&self) -> &str {
            &self.0[0..2]
        }

        /// Returns the registrant code (characters 3-5).
        pub fn registrant_code(&self) -> &str {
            &self.0[2..5]
        }

        /// Returns the year of registration (characters 6-7 as number).
        pub fn year_of_registration(&self) -> u8 {
            self.0[5..7].parse().unwrap_or(0)
        }

        /// Returns the full year of registration (adding 2000 or 1900).
        pub fn full_year_of_registration(&self) -> u16 {
            let year = self.year_of_registration() as u16;
            // Assume years 00-30 are 2000s, 31-99 are 1900s (common convention)
            if year <= 30 {
                2000 + year
            } else {
                1900 + year
            }
        }

        /// Returns the designation code (last 5 characters).
        pub fn designation_code(&self) -> &str {
            &self.0[7..12]
        }

        /// Returns the raw ISRC string.
        pub fn as_str(&self) -> &str {
            &self.0
        }

        /// Normalizes an ISRC string by removing spaces and hyphens.
        pub fn normalize(value: &str) -> String {
            value
                .chars()
                .filter(|c| c.is_ascii_alphanumeric())
                .collect::<String>()
                .to_uppercase()
        }

        /// Converts to a formatted display string with hyphens.
        pub fn formatted(&self) -> String {
            format!(
                "{}-{}-{}-{}",
                &self.0[0..2],  // Country code
                &self.0[2..5],  // Registrant code
                &self.0[5..7],  // Year
                &self.0[7..12]  // Designation code
            )
        }

        /// Checks if this ISRC is from a specific country.
        pub fn is_from_country(&self, country_code: &str) -> bool {
            self.country_code().eq_ignore_ascii_case(country_code)
        }

        /// Checks if this ISRC was registered in a specific year.
        pub fn registered_in_year(&self, year: u16) -> bool {
            self.full_year_of_registration() == year
        }

        /// Returns the age of the registration in years.
        pub fn registration_age_years(&self) -> u16 {
            let current_year = 2025u16; // TODO: use actual current year
            current_year.saturating_sub(self.full_year_of_registration())
        }
    }

    impl FromStr for Isrc {
        type Err = IsrcError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::new(s)
        }
    }

    impl fmt::Display for Isrc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl AsRef<str> for Isrc {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    impl From<Isrc> for String {
        fn from(isrc: Isrc) -> String {
            isrc.0
        }
    }
}

// Re-export API types based on features
#[cfg(feature = "std")]
pub use api::*;

#[cfg(feature = "web")]
mod web_api {
    use super::Isrc;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    impl Isrc {
        /// Creates a new ISRC from JavaScript.
        #[wasm_bindgen(constructor)]
        pub fn new_web(value: &str) -> Result<Isrc, JsError> {
            Self::new(value).map_err(|e| JsError::new(&e.to_string()))
        }

        /// Creates an ISRC from components.
        #[wasm_bindgen(js_name = fromComponents)]
        pub fn from_components_web(
            country_code: &str,
            registrant_code: &str,
            year: u8,
            designation_code: &str,
        ) -> Result<Isrc, JsError> {
            Self::from_components(country_code, registrant_code, year, designation_code)
                .map_err(|e| JsError::new(&e.to_string()))
        }

        /// Validates an ISRC string.
        #[wasm_bindgen(js_name = isValid)]
        pub fn is_valid_web(value: &str) -> bool {
            Self::validate(value).is_ok()
        }

        /// Returns the country code.
        #[wasm_bindgen(js_name = getCountryCode)]
        pub fn get_country_code_web(&self) -> String {
            self.country_code().to_string()
        }

        /// Returns the registrant code.
        #[wasm_bindgen(js_name = getRegistrantCode)]
        pub fn get_registrant_code_web(&self) -> String {
            self.registrant_code().to_string()
        }

        /// Returns the year of registration.
        #[wasm_bindgen(js_name = getYearOfRegistration)]
        pub fn get_year_of_registration_web(&self) -> u8 {
            self.year_of_registration()
        }

        /// Returns the full year of registration.
        #[wasm_bindgen(js_name = getFullYearOfRegistration)]
        pub fn get_full_year_of_registration_web(&self) -> u16 {
            self.full_year_of_registration()
        }

        /// Returns the designation code.
        #[wasm_bindgen(js_name = getDesignationCode)]
        pub fn get_designation_code_web(&self) -> String {
            self.designation_code().to_string()
        }

        /// Returns the formatted string.
        #[wasm_bindgen(js_name = formatted)]
        pub fn formatted_web(&self) -> String {
            self.formatted()
        }

        /// Returns string representation.
        #[wasm_bindgen(js_name = toString)]
        pub fn to_string_web(&self) -> String {
            self.0.clone()
        }

        /// Normalizes an ISRC string.
        #[wasm_bindgen(js_name = normalize)]
        pub fn normalize_web(value: &str) -> String {
            Self::normalize(value)
        }

        /// Checks if ISRC is from a specific country.
        #[wasm_bindgen(js_name = isFromCountry)]
        pub fn is_from_country_web(&self, country_code: &str) -> bool {
            self.is_from_country(country_code)
        }

        /// Checks if registered in a specific year.
        #[wasm_bindgen(js_name = registeredInYear)]
        pub fn registered_in_year_web(&self, year: u16) -> bool {
            self.registered_in_year(year)
        }

        /// Returns registration age in years.
        #[wasm_bindgen(js_name = getRegistrationAgeYears)]
        pub fn get_registration_age_years_web(&self) -> u16 {
            self.registration_age_years()
        }
    }
}

#[cfg(feature = "runtime")]
mod runtime_api {
    use super::RuntimeIsrc;
    use frame_support::BoundedVec;

    #[cfg(not(feature = "std"))]
    extern crate alloc;

    #[cfg(not(feature = "std"))]
    use alloc::string::{String, ToString};

    /// Error types for RuntimeIsrc operations
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(thiserror::Error))]
    pub enum RuntimeIsrcError {
        /// Data exceeds the 12-byte limit
        #[cfg_attr(feature = "std", error("Data exceeds the 12-byte capacity limit"))]
        ExceedsCapacity,
        /// Invalid UTF-8 data
        #[cfg_attr(feature = "std", error("Invalid UTF-8 data"))]
        InvalidUtf8,
    }

    impl RuntimeIsrc {
        /// Creates a new RuntimeIsrc from bytes without validation.
        pub fn new_from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, RuntimeIsrcError> {
            let bytes = bytes.as_ref();
            BoundedVec::try_from(bytes.to_vec())
                .map(Self)
                .map_err(|_| RuntimeIsrcError::ExceedsCapacity)
        }

        /// Creates a new RuntimeIsrc from a string slice without validation.
        pub fn new_from_str(value: &str) -> Result<Self, RuntimeIsrcError> {
            Self::new_from_bytes(value.as_bytes())
        }

        /// Returns the ISRC as a byte slice.
        pub fn as_bytes(&self) -> &[u8] {
            &self.0
        }

        /// Converts the RuntimeIsrc to a string if it contains valid UTF-8.
        pub fn to_string_lossy(&self) -> String {
            String::from_utf8_lossy(&self.0).to_string()
        }

        /// Converts the RuntimeIsrc to a string if it contains valid UTF-8.
        pub fn to_string(&self) -> Result<String, RuntimeIsrcError> {
            String::from_utf8(self.0.to_vec()).map_err(|_| RuntimeIsrcError::InvalidUtf8)
        }

        /// Returns the maximum capacity.
        pub const fn capacity() -> u32 {
            12
        }

        /// Returns the current length in bytes.
        pub fn len(&self) -> usize {
            self.0.len()
        }

        /// Returns true if the RuntimeIsrc is empty.
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    }
}

#[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
impl RuntimeIsrc {
    /// Create a new RuntimeIsrc from a BoundedVec
    pub fn new(value: frame_support::BoundedVec<u8, frame_support::traits::ConstU32<12>>) -> Self {
        Self(value)
    }

    /// Generate a benchmark instance
    pub fn generate_benchmark(i: u32) -> Self {
        use crate::benchmarking::create_bounded_string;
        Self(create_bounded_string::<12>(i))
    }
}
