//! International Standard Musical Work Code (ISWC) identifier.
//!
//! The ISWC is a unique identifier for musical works, following the format:
//! `T-NNNNNNNN-C` where:
//! - `T` indicates this is an ISWC
//! - `NNNNNNNN` is a 9-digit number
//! - `C` is a check digit
//!
//! This module provides a comprehensive API for creating, validating, and manipulating
//! ISWC identifiers with proper error handling and format normalization.

use allfeat_midds_v2_codegen::runtime_midds;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

/// International Standard Musical Work Code (ISWC) identifier.
///
/// The ISWC is a unique identifier for musical works, following the format:
/// `T-NNNNNNNN-C` where:
/// - `T` indicates this is an ISWC
/// - `NNNNNNNN` is a 9-digit number
/// - `C` is a check digit
///
/// # Examples
/// - `T-034524680-1`
/// - `T-123456789-0`
///
/// # Type Transformation
/// In runtime mode: `String` → `BoundedVec<u8, ConstU32<13>>`
#[runtime_midds]
#[cfg_attr(feature = "web", wasm_bindgen)]
pub struct Iswc(
    /// The ISWC string, limited to 13 characters in runtime mode.
    #[runtime_bound(13)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub String,
);

#[cfg(feature = "std")]
mod api {
    use super::Iswc;
    use regex::Regex;
    use std::fmt;
    use std::str::FromStr;
    use thiserror::Error;

    static ISWC_REGEX: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static CAPTURE_REGEX: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static NORMALIZE_REGEX: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static SIMPLE_REGEX: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();

    /// Error types for ISWC operations
    #[derive(Error, Debug, Clone, PartialEq, Eq)]
    pub enum IswcError {
        /// Invalid ISWC format
        #[error("Invalid ISWC format: {0}")]
        InvalidFormat(String),
        /// Invalid check digit
        #[error("Invalid ISWC check digit")]
        InvalidCheckDigit,
        /// Invalid prefix (must be 'T')
        #[error("ISWC must start with 'T'")]
        InvalidPrefix,
        /// Invalid length
        #[error("ISWC must be exactly 13 characters long")]
        InvalidLength,
        /// Non-numeric work code
        #[error("ISWC work code must be numeric")]
        NonNumericWorkCode,
    }

    impl Iswc {
        /// Creates a new ISWC from a string with validation.
        ///
        /// # Arguments
        /// * `value` - The ISWC string to validate
        ///
        /// # Returns
        /// * `Ok(Iswc)` if the string is a valid ISWC
        /// * `Err(IswcError)` if the string is invalid
        ///
        /// # Examples
        /// ```
        /// use allfeat_midds_v2::musical_work::iswc::Iswc;
        ///
        /// let iswc = Iswc::new("T-034524680-8").unwrap();
        /// assert_eq!(iswc.to_string(), "T-034524680-8");
        /// ```
        pub fn new(value: impl Into<String>) -> Result<Self, IswcError> {
            let value = value.into();
            Self::validate(&value)?;
            Ok(Self(value.to_string()))
        }

        /// Creates a new ISWC without validation (unsafe).
        ///
        /// # Safety
        /// The caller must ensure that the value is a valid ISWC format.
        ///
        /// # Arguments
        /// * `value` - The ISWC string
        ///
        /// # Examples
        /// ```
        /// use allfeat_midds_v2::musical_work::iswc::Iswc;
        ///
        /// let iswc = unsafe { Iswc::new_unchecked("T-034524680-1") };
        /// ```
        pub fn new_unchecked(value: impl Into<String>) -> Self {
            Self(value.into())
        }

        /// Generates a new ISWC with the given work code.
        ///
        /// # Arguments
        /// * `work_code` - A 9-digit work code
        ///
        /// # Returns
        /// * `Ok(Iswc)` with calculated check digit
        /// * `Err(IswcError)` if the work code is invalid
        ///
        /// # Examples
        /// ```
        /// use allfeat_midds_v2::musical_work::iswc::Iswc;
        ///
        /// let iswc = Iswc::from_work_code(34524680).unwrap();
        /// assert_eq!(iswc.work_code(), 34524680);
        /// ```
        pub fn from_work_code(work_code: u32) -> Result<Self, IswcError> {
            if work_code > 999_999_999 {
                return Err(IswcError::NonNumericWorkCode);
            }

            let work_code_str = format!("{:09}", work_code);
            let check_digit = Self::calculate_check_digit(&work_code_str);
            let iswc_string = format!("T-{}-{}", work_code_str, check_digit);

            Ok(Self(iswc_string))
        }

        /// Validates an ISWC string format using regex pattern matching.
        ///
        /// # Arguments
        /// * `value` - The string to validate
        ///
        /// # Returns
        /// * `Ok(())` if valid
        /// * `Err(IswcError)` if invalid
        ///
        /// # Format
        /// The ISWC must match the pattern: `T-\d{9}-\d` where:
        /// - `T` is the literal prefix
        /// - `\d{9}` is exactly 9 digits (work code)
        /// - `\d` is exactly 1 digit (check digit)
        pub fn validate(value: &str) -> Result<(), IswcError> {
            // Use regex for comprehensive format validation
            let regex = ISWC_REGEX
                .get_or_init(|| Regex::new(r"^T-\d{9}-\d$").expect("ISWC regex pattern is valid"));

            // First check basic format with regex
            if !regex.is_match(value) {
                // Provide more specific error messages based on common issues
                if value.len() != 13 {
                    return Err(IswcError::InvalidLength);
                }
                if !value.starts_with('T') {
                    return Err(IswcError::InvalidPrefix);
                }
                if !value.chars().skip(2).take(9).all(|c| c.is_ascii_digit()) {
                    return Err(IswcError::NonNumericWorkCode);
                }
                return Err(IswcError::InvalidFormat(value.to_string()));
            }

            // Extract components using regex captures for safety
            let capture_regex = CAPTURE_REGEX.get_or_init(|| {
                Regex::new(r"^T-(\d{9})-(\d)$").expect("ISWC capture regex pattern is valid")
            });

            let captures = capture_regex
                .captures(value)
                .ok_or_else(|| IswcError::InvalidFormat(value.to_string()))?;

            let work_code = captures
                .get(1)
                .ok_or_else(|| IswcError::InvalidFormat(value.to_string()))?
                .as_str();

            let check_digit_str = captures
                .get(2)
                .ok_or_else(|| IswcError::InvalidFormat(value.to_string()))?
                .as_str();

            // Validate check digit
            let expected_check_digit = Self::calculate_check_digit(work_code);
            let actual_check_digit = check_digit_str
                .parse::<u8>()
                .map_err(|_| IswcError::InvalidFormat(value.to_string()))?;

            if expected_check_digit != actual_check_digit {
                return Err(IswcError::InvalidCheckDigit);
            }

            Ok(())
        }

        /// Calculates the check digit for a given work code using the ISWC standard algorithm.
        ///
        /// The ISWC check digit is calculated using the modulo 10 algorithm:
        /// Each digit is multiplied by its position (1-based) and summed,
        /// then the check digit is the result modulo 10.
        ///
        /// # Arguments
        /// * `work_code` - A 9-digit work code string
        ///
        /// # Returns
        /// The calculated check digit (0-9)
        ///
        /// # Examples
        /// For work code "034524680":
        /// (0×1 + 3×2 + 4×3 + 5×4 + 2×5 + 4×6 + 6×7 + 8×8 + 0×9) mod 10 = 8
        fn calculate_check_digit(work_code: &str) -> u8 {
            let sum: u32 = work_code
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    let digit = c.to_digit(10).unwrap();
                    digit * (i as u32 + 1)
                })
                .sum();

            (sum % 10) as u8
        }

        /// Returns the work code as a u32.
        ///
        /// # Examples
        /// ```
        /// use allfeat_midds_v2::musical_work::iswc::Iswc;
        ///
        /// let iswc = Iswc::new("T-034524680-8").unwrap();
        /// assert_eq!(iswc.work_code(), 34524680);
        /// ```
        pub fn work_code(&self) -> u32 {
            self.0[2..11].parse().unwrap()
        }

        /// Returns the check digit.
        ///
        /// # Examples
        /// ```
        /// use allfeat_midds_v2::musical_work::iswc::Iswc;
        ///
        /// let iswc = Iswc::new("T-034524680-8").unwrap();
        /// assert_eq!(iswc.check_digit(), 8);
        /// ```
        pub fn check_digit(&self) -> u8 {
            self.0.chars().nth(12).unwrap().to_digit(10).unwrap() as u8
        }

        /// Returns the raw ISWC string.
        ///
        /// # Examples
        /// ```
        /// use allfeat_midds_v2::musical_work::iswc::Iswc;
        ///
        /// let iswc = Iswc::new("T-034524680-8").unwrap();
        /// assert_eq!(iswc.as_str(), "T-034524680-8");
        /// ```
        pub fn as_str(&self) -> &str {
            &self.0
        }

        /// Normalizes an ISWC string by removing spaces and ensuring proper format using regex.
        ///
        /// This function attempts to clean and reformat various input formats into the
        /// standard ISWC format `T-NNNNNNNNN-C`.
        ///
        /// # Arguments
        /// * `value` - The string to normalize
        ///
        /// # Returns
        /// The normalized ISWC string if possible, or the cleaned input if normalization fails
        ///
        /// # Examples
        /// ```
        /// use allfeat_midds_v2::musical_work::iswc::Iswc;
        ///
        /// assert_eq!(Iswc::normalize(" T 034524680 1 "), "T-034524680-1");
        /// assert_eq!(Iswc::normalize("T0345246801"), "T-034524680-1");
        /// assert_eq!(Iswc::normalize("T-034-524-680-1"), "T-034524680-1");
        /// ```
        pub fn normalize(value: &str) -> String {
            // Remove all whitespace and convert to uppercase
            let cleaned: String = value
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| c.to_ascii_uppercase())
                .collect();

            // Use regex to extract potential ISWC components from various formats
            let regex = NORMALIZE_REGEX.get_or_init(|| {
                // Matches: T followed by 9-10 digits (with optional separators)
                // Captures the 9-digit work code and optional check digit
                Regex::new(r"^T[-\s]*(\d{3})[-\s]*(\d{3})[-\s]*(\d{3})[-\s]*(\d?)$")
                    .expect("ISWC normalization regex pattern is valid")
            });

            if let Some(captures) = regex.captures(&cleaned) {
                let part1 = captures.get(1).map(|m| m.as_str()).unwrap_or("");
                let part2 = captures.get(2).map(|m| m.as_str()).unwrap_or("");
                let part3 = captures.get(3).map(|m| m.as_str()).unwrap_or("");
                let check_digit = captures.get(4).map(|m| m.as_str()).unwrap_or("");

                let work_code = format!("{}{}{}", part1, part2, part3);

                if work_code.len() == 9 {
                    if check_digit.is_empty() {
                        // Calculate missing check digit
                        let calculated_check = Self::calculate_check_digit(&work_code);
                        return format!("T-{}-{}", work_code, calculated_check);
                    } else if check_digit.len() == 1 {
                        // Use provided check digit
                        return format!("T-{}-{}", work_code, check_digit);
                    }
                }
            }

            // Fallback: try simple T + 10 digits format
            let simple_regex = SIMPLE_REGEX.get_or_init(|| {
                Regex::new(r"^T(\d{9})(\d?)$").expect("Simple ISWC regex pattern is valid")
            });

            if let Some(captures) = simple_regex.captures(&cleaned) {
                let work_code = captures.get(1).map(|m| m.as_str()).unwrap_or("");
                let check_digit = captures.get(2).map(|m| m.as_str()).unwrap_or("");

                if work_code.len() == 9 {
                    if check_digit.is_empty() {
                        let calculated_check = Self::calculate_check_digit(work_code);
                        return format!("T-{}-{}", work_code, calculated_check);
                    } else {
                        return format!("T-{}-{}", work_code, check_digit);
                    }
                }
            }

            // Return cleaned input if no normalization is possible
            cleaned
        }
    }

    impl FromStr for Iswc {
        type Err = IswcError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::new(s)
        }
    }

    impl fmt::Display for Iswc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl AsRef<str> for Iswc {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    impl From<Iswc> for String {
        fn from(iswc: Iswc) -> String {
            iswc.0
        }
    }
}

// Re-export API types based on features
#[cfg(feature = "std")]
pub use api::*;

// Re-export runtime types when runtime feature is enabled
#[cfg(feature = "runtime")]
pub use runtime_api::RuntimeIswcError;

#[cfg(feature = "web")]
mod web_api {
    use super::Iswc;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    impl Iswc {
        /// Creates a new ISWC from a string with validation.
        ///
        /// # Arguments
        /// * `value` - The ISWC string to validate
        ///
        /// # Returns
        /// * `Ok(Iswc)` if the string is a valid ISWC
        /// * Throws an error if the string is invalid
        ///
        /// # Examples
        /// ```javascript
        /// import { Iswc } from 'allfeat-midds-v2';
        ///
        /// try {
        ///   const iswc = Iswc.new("T-034524680-8");
        ///   console.log(iswc.toString());
        /// } catch (error) {
        ///   console.error("Invalid ISWC:", error);
        /// }
        /// ```
        #[wasm_bindgen(constructor)]
        pub fn new_web(value: &str) -> Result<Iswc, JsError> {
            // Basic validation for web mode
            if value.len() != 13 || !value.starts_with('T') {
                return Err(JsError::new("Invalid ISWC format"));
            }
            Ok(Iswc(value.to_string()))
        }

        /// Creates a new ISWC without validation (unsafe).
        ///
        /// # Arguments
        /// * `value` - The ISWC string
        ///
        /// # Examples
        /// ```javascript
        /// import { Iswc } from 'allfeat-midds-v2';
        ///
        /// const iswc = Iswc.newUnchecked("T-034524680-8");
        /// ```
        #[wasm_bindgen(js_name = newUnchecked)]
        pub fn new_unchecked_web(value: &str) -> Iswc {
            Iswc(value.to_string())
        }

        /// Generates a new ISWC with the given work code.
        ///
        /// # Arguments
        /// * `work_code` - A 9-digit work code
        ///
        /// # Returns
        /// * `Ok(Iswc)` with calculated check digit
        /// * Throws an error if the work code is invalid
        ///
        /// # Examples
        /// ```javascript
        /// import { Iswc } from 'allfeat-midds-v2';
        ///
        /// try {
        ///   const iswc = Iswc.fromWorkCode(34524680);
        ///   console.log("Generated ISWC:", iswc.toString());
        /// } catch (error) {
        ///   console.error("Invalid work code:", error);
        /// }
        /// ```
        #[wasm_bindgen(js_name = fromWorkCode)]
        pub fn from_work_code_web(work_code: u32) -> Result<Iswc, JsError> {
            if work_code > 999_999_999 {
                return Err(JsError::new("Invalid work code"));
            }
            let work_code_str = format!("{:09}", work_code);
            let check_digit = Self::calculate_check_digit_web(&work_code_str);
            let iswc_string = format!("T-{}-{}", work_code_str, check_digit);
            Ok(Iswc(iswc_string))
        }

        /// Validates an ISWC string format.
        ///
        /// # Arguments
        /// * `value` - The string to validate
        ///
        /// # Returns
        /// * `true` if valid, `false` otherwise
        ///
        /// # Examples
        /// ```javascript
        /// import { Iswc } from 'allfeat-midds-v2';
        ///
        /// console.log(Iswc.isValid("T-034524680-8")); // true
        /// console.log(Iswc.isValid("invalid"));       // false
        /// ```
        #[wasm_bindgen(js_name = isValid)]
        pub fn is_valid_web(value: &str) -> bool {
            value.len() == 13 && value.starts_with('T')
        }

        /// Returns the work code as a number.
        ///
        /// # Examples
        /// ```javascript
        /// import { Iswc } from 'allfeat-midds-v2';
        ///
        /// const iswc = new Iswc("T-034524680-8");
        /// console.log(iswc.workCode()); // 34524680
        /// ```
        #[wasm_bindgen(js_name = workCode)]
        pub fn work_code_web(&self) -> u32 {
            self.0[2..11].parse().unwrap_or(0)
        }

        /// Returns the check digit.
        ///
        /// # Examples
        /// ```javascript
        /// import { Iswc } from 'allfeat-midds-v2';
        ///
        /// const iswc = new Iswc("T-034524680-8");
        /// console.log(iswc.checkDigit()); // 8
        /// ```
        #[wasm_bindgen(js_name = checkDigit)]
        pub fn check_digit_web(&self) -> u8 {
            self.0.chars().nth(12).unwrap().to_digit(10).unwrap_or(0) as u8
        }

        /// Returns the raw ISWC string.
        ///
        /// # Examples
        /// ```javascript
        /// import { Iswc } from 'allfeat-midds-v2';
        ///
        /// const iswc = new Iswc("T-034524680-8");
        /// console.log(iswc.asString()); // "T-034524680-8"
        /// ```
        #[wasm_bindgen(js_name = asString)]
        pub fn as_string_web(&self) -> String {
            self.0.clone()
        }

        /// Normalizes an ISWC string by removing spaces and ensuring proper format.
        ///
        /// # Arguments
        /// * `value` - The string to normalize
        ///
        /// # Returns
        /// The normalized ISWC string
        ///
        /// # Examples
        /// ```javascript
        /// import { Iswc } from 'allfeat-midds-v2';
        ///
        /// const normalized = Iswc.normalize(" T 034524680 8 ");
        /// console.log(normalized); // "T-034524680-8"
        /// ```
        #[wasm_bindgen(js_name = normalize)]
        pub fn normalize_web(value: &str) -> String {
            let cleaned: String = value.chars().filter(|c| !c.is_whitespace()).collect();

            if cleaned.len() >= 11 && cleaned.starts_with('T') {
                let work_part = &cleaned[1..10];
                let check_part = cleaned.chars().nth(10).unwrap_or('0');
                format!("T-{}-{}", work_part, check_part)
            } else {
                cleaned
            }
        }

        /// Returns the string representation of the ISWC.
        ///
        /// # Examples
        /// ```javascript
        /// import { Iswc } from 'allfeat-midds-v2';
        ///
        /// const iswc = new Iswc("T-034524680-8");
        /// console.log(iswc.toString()); // "T-034524680-8"
        /// ```
        #[wasm_bindgen(js_name = toString)]
        pub fn to_string_web(&self) -> String {
            self.0.clone()
        }

        /// Internal helper for calculating check digit
        fn calculate_check_digit_web(work_code: &str) -> u8 {
            let sum: u32 = work_code
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    let digit = c.to_digit(10).unwrap_or(0);
                    digit * (i as u32 + 1)
                })
                .sum();

            (sum % 10) as u8
        }
    }
}

#[cfg(feature = "runtime")]
mod runtime_api {
    use super::RuntimeIswc;
    use frame_support::BoundedVec;

    #[cfg(not(feature = "std"))]
    extern crate alloc;

    #[cfg(not(feature = "std"))]
    use alloc::string::{String, ToString};

    /// Error types for RuntimeIswc operations
    #[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
    pub enum RuntimeIswcError {
        /// Data exceeds the 13-byte limit
        #[error("Data exceeds the 13-byte capacity limit")]
        ExceedsCapacity,
        /// Invalid UTF-8 data
        #[error("Invalid UTF-8 data")]
        InvalidUtf8,
    }

    impl RuntimeIswc {
        /// Creates a new RuntimeIswc from bytes without validation.
        ///
        /// # Arguments
        /// * `bytes` - The ISWC bytes (must be valid UTF-8)
        ///
        /// # Returns
        /// * `Ok(RuntimeIswc)` if the bytes fit within the bound
        /// * `Err(RuntimeIswcError)` if the bytes exceed the 13-byte limit
        pub fn new_from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, RuntimeIswcError> {
            let bytes = bytes.as_ref();
            BoundedVec::try_from(bytes.to_vec())
                .map(Self)
                .map_err(|_| RuntimeIswcError::ExceedsCapacity)
        }

        /// Creates a new RuntimeIswc from a string slice without validation.
        ///
        /// # Arguments
        /// * `value` - The ISWC string slice
        ///
        /// # Returns
        /// * `Ok(RuntimeIswc)` if the string fits within the bound
        /// * `Err(RuntimeIswcError)` if the string exceeds the 13-byte limit
        pub fn new_from_str(value: &str) -> Result<Self, RuntimeIswcError> {
            Self::new_from_bytes(value.as_bytes())
        }

        /// Returns the ISWC as a byte slice.
        pub fn as_bytes(&self) -> &[u8] {
            &self.0
        }

        /// Converts the RuntimeIswc to a string if it contains valid UTF-8.
        ///
        /// # Returns
        /// * `Ok(String)` if the bytes are valid UTF-8
        /// * `Err(())` if the bytes are not valid UTF-8
        pub fn to_string_lossy(&self) -> String {
            String::from_utf8_lossy(&self.0).to_string()
        }

        /// Converts the RuntimeIswc to a string if it contains valid UTF-8.
        ///
        /// # Returns
        /// * `Ok(String)` if the bytes are valid UTF-8
        /// * `Err(RuntimeIswcError)` if the bytes are not valid UTF-8
        pub fn to_string(&self) -> Result<String, RuntimeIswcError> {
            String::from_utf8(self.0.to_vec()).map_err(|_| RuntimeIswcError::InvalidUtf8)
        }

        /// Returns the maximum capacity of the RuntimeIswc.
        pub const fn capacity() -> u32 {
            13
        }

        /// Returns the current length in bytes.
        pub fn len(&self) -> usize {
            self.0.len()
        }

        /// Returns true if the RuntimeIswc is empty.
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    }

    #[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
    impl RuntimeIswc {
        /// Generate a benchmark instance
        pub fn generate_benchmark(i: u32) -> Self {
            use crate::benchmarking::create_bounded_string;

            Self(create_bounded_string::<13>(i))
        }
    }
}
