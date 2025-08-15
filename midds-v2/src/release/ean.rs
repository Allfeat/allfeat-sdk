//! European Article Number (EAN) and Universal Product Code (UPC) identifier.
//!
//! EAN and UPC are standardized barcode systems used to identify products,
//! including music releases in both physical and digital formats.
//!
//! This module provides a comprehensive API for creating, validating, and manipulating
//! EAN/UPC identifiers with proper error handling and format normalization.

use allfeat_midds_v2_codegen::runtime_midds;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

/// European Article Number (EAN) or Universal Product Code (UPC) identifier.
///
/// Used to identify music releases in both physical and digital formats.
/// EAN codes are typically 13 digits, while UPC codes are 12 digits.
///
/// # Supported Formats
/// - **EAN-13**: 13-digit code (e.g., `1234567890123`)
/// - **UPC-A**: 12-digit code (automatically converted to EAN-13)
/// - **EAN-8**: 8-digit short code (for small packages)
///
/// # Type Transformation
/// In runtime mode: `String` → `BoundedVec<u8, ConstU32<13>>`
///
/// # Examples
/// ```rust
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use allfeat_midds_v2::release::ean::{Ean, CodeType};
///
/// // Create from EAN-13
/// let ean = Ean::new("1234567890128")?;
/// assert_eq!(ean.to_string(), "1234567890128");
///
/// // Create from UPC-A (automatically converted)
/// let upc = Ean::from_upc("012345678905")?;
/// assert_eq!(upc.code_type(), CodeType::Ean13);
/// # Ok(())
/// # }
/// ```
#[runtime_midds]
#[cfg_attr(feature = "web", wasm_bindgen)]
pub struct Ean(
    /// The EAN/UPC string, limited to 13 characters in runtime mode.
    #[runtime_bound(13)]
    #[cfg_attr(feature = "web", wasm_bindgen(getter_with_clone))]
    pub String,
);

#[cfg(feature = "std")]
mod api {
    use super::Ean;
    use regex::Regex;
    use std::fmt;
    use std::str::FromStr;
    use thiserror::Error;

    static EAN_REGEX: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static UPC_REGEX: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static EAN8_REGEX: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();

    /// Error types for EAN/UPC operations
    #[derive(Error, Debug, Clone, PartialEq, Eq)]
    pub enum EanError {
        /// Invalid EAN/UPC format
        #[error("Invalid EAN/UPC format: {0}")]
        InvalidFormat(String),
        /// Invalid check digit
        #[error("Invalid EAN/UPC check digit")]
        InvalidCheckDigit,
        /// Invalid length
        #[error("EAN/UPC must be 8, 12, or 13 digits long")]
        InvalidLength,
        /// Non-numeric code
        #[error("EAN/UPC must contain only digits")]
        NonNumeric,
    }

    /// EAN/UPC code type
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum CodeType {
        /// 8-digit EAN for small packages
        Ean8,
        /// 12-digit UPC-A code
        UpcA,
        /// 13-digit EAN code
        Ean13,
    }

    impl Ean {
        /// Creates a new EAN from a string with validation.
        ///
        /// # Arguments
        /// * `value` - The EAN/UPC string to validate
        ///
        /// # Returns
        /// * `Ok(Ean)` if the string is a valid EAN/UPC
        /// * `Err(EanError)` if the string is invalid
        ///
        /// # Examples
        /// ```
        /// # use std::error::Error;
        /// # fn main() -> Result<(), Box<dyn Error>> {
        /// use allfeat_midds_v2::release::ean::Ean;
        ///
        /// let ean = Ean::new("1234567890128")?;
        /// assert_eq!(ean.to_string(), "1234567890128");
        /// # Ok(())
        /// # }
        /// ```
        pub fn new(value: impl Into<String>) -> Result<Self, EanError> {
            let value = value.into();
            Self::validate(&value)?;
            Ok(Self(value.to_string()))
        }

        /// Creates a new EAN without validation (unsafe).
        ///
        /// # Safety
        /// The caller must ensure that the value is a valid EAN/UPC format.
        pub fn new_unchecked(value: impl Into<String>) -> Self {
            Self(value.into())
        }

        /// Creates an EAN-13 from a UPC-A code by adding leading zero.
        ///
        /// # Arguments
        /// * `upc` - A 12-digit UPC-A code
        ///
        /// # Returns
        /// * `Ok(Ean)` with the converted EAN-13 code
        /// * `Err(EanError)` if the UPC code is invalid
        ///
        /// # Examples
        /// ```
        /// # use std::error::Error;
        /// # fn main() -> Result<(), Box<dyn Error>> {
        /// use allfeat_midds_v2::release::ean::Ean;
        ///
        /// let ean = Ean::from_upc("012345678905")?;
        /// assert_eq!(ean.to_string(), "0012345678905");
        /// # Ok(())
        /// # }
        /// ```
        pub fn from_upc(upc: impl Into<String>) -> Result<Self, EanError> {
            let upc = upc.into();

            // Validate UPC format
            let upc_regex = UPC_REGEX
                .get_or_init(|| Regex::new(r"^\d{12}$").expect("UPC regex pattern is valid"));

            if !upc_regex.is_match(&upc) {
                return Err(EanError::InvalidFormat(format!(
                    "Invalid UPC format: {}",
                    upc
                )));
            }

            // Validate UPC check digit
            if !Self::validate_upc_check_digit(&upc) {
                return Err(EanError::InvalidCheckDigit);
            }

            // Convert UPC-A to EAN-13 by adding leading zero
            let ean13 = format!("0{}", upc);
            Ok(Self(ean13))
        }

        /// Generates a valid EAN-13 from the first 12 digits.
        ///
        /// # Arguments
        /// * `prefix` - 12-digit prefix
        ///
        /// # Returns
        /// * `Ok(Ean)` with calculated check digit
        /// * `Err(EanError)` if the prefix is invalid
        pub fn from_prefix(prefix: impl Into<String>) -> Result<Self, EanError> {
            let prefix = prefix.into();

            if prefix.len() != 12 {
                return Err(EanError::InvalidLength);
            }

            if !prefix.chars().all(|c| c.is_ascii_digit()) {
                return Err(EanError::NonNumeric);
            }

            let check_digit = Self::calculate_ean13_check_digit(&prefix);
            let ean = format!("{}{}", prefix, check_digit);

            Ok(Self(ean))
        }

        /// Validates an EAN/UPC string format using regex pattern matching.
        ///
        /// # Arguments
        /// * `value` - The string to validate
        ///
        /// # Returns
        /// * `Ok(())` if valid
        /// * `Err(EanError)` if invalid
        pub fn validate(value: &str) -> Result<(), EanError> {
            let cleaned = value
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>();

            match cleaned.len() {
                8 => Self::validate_ean8(&cleaned),
                12 => Self::validate_upc_a(&cleaned),
                13 => Self::validate_ean13(&cleaned),
                _ => Err(EanError::InvalidLength),
            }
        }

        /// Validates EAN-8 format.
        fn validate_ean8(value: &str) -> Result<(), EanError> {
            let ean8_regex = EAN8_REGEX
                .get_or_init(|| Regex::new(r"^\d{8}$").expect("EAN-8 regex pattern is valid"));

            if !ean8_regex.is_match(value) {
                return Err(EanError::InvalidFormat(value.to_string()));
            }

            if !Self::validate_ean8_check_digit(value) {
                return Err(EanError::InvalidCheckDigit);
            }

            Ok(())
        }

        /// Validates UPC-A format.
        fn validate_upc_a(value: &str) -> Result<(), EanError> {
            let upc_regex = UPC_REGEX
                .get_or_init(|| Regex::new(r"^\d{12}$").expect("UPC regex pattern is valid"));

            if !upc_regex.is_match(value) {
                return Err(EanError::InvalidFormat(value.to_string()));
            }

            if !Self::validate_upc_check_digit(value) {
                return Err(EanError::InvalidCheckDigit);
            }

            Ok(())
        }

        /// Validates EAN-13 format.
        fn validate_ean13(value: &str) -> Result<(), EanError> {
            let ean_regex = EAN_REGEX
                .get_or_init(|| Regex::new(r"^\d{13}$").expect("EAN regex pattern is valid"));

            if !ean_regex.is_match(value) {
                return Err(EanError::InvalidFormat(value.to_string()));
            }

            if !Self::validate_ean13_check_digit(value) {
                return Err(EanError::InvalidCheckDigit);
            }

            Ok(())
        }

        /// Calculates the check digit for EAN-13 using the standard algorithm.
        ///
        /// The EAN-13 check digit is calculated as:
        /// 1. Sum all odd-positioned digits (1st, 3rd, 5th, etc.)
        /// 2. Sum all even-positioned digits and multiply by 3
        /// 3. Add both sums
        /// 4. Check digit = (10 - (sum mod 10)) mod 10
        fn calculate_ean13_check_digit(code: &str) -> u8 {
            let sum: u32 = code
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    let digit = c.to_digit(10).unwrap();
                    if i % 2 == 0 {
                        digit // Odd position (1-indexed)
                    } else {
                        digit * 3 // Even position (1-indexed)
                    }
                })
                .sum();

            ((10 - (sum % 10)) % 10) as u8
        }

        /// Validates EAN-13 check digit.
        fn validate_ean13_check_digit(code: &str) -> bool {
            if code.len() != 13 {
                return false;
            }

            let prefix = &code[..12];
            let check_digit = code.chars().nth(12).unwrap().to_digit(10).unwrap() as u8;
            let expected = Self::calculate_ean13_check_digit(prefix);

            check_digit == expected
        }

        /// Validates UPC-A check digit.
        fn validate_upc_check_digit(code: &str) -> bool {
            if code.len() != 12 {
                return false;
            }

            let sum: u32 = code
                .chars()
                .take(11)
                .enumerate()
                .map(|(i, c)| {
                    let digit = c.to_digit(10).unwrap();
                    if i % 2 == 0 {
                        digit * 3 // Odd position (1-indexed)
                    } else {
                        digit // Even position (1-indexed)
                    }
                })
                .sum();

            let check_digit = code.chars().nth(11).unwrap().to_digit(10).unwrap();
            let expected = (10 - (sum % 10)) % 10;

            check_digit == expected
        }

        /// Validates EAN-8 check digit.
        fn validate_ean8_check_digit(code: &str) -> bool {
            if code.len() != 8 {
                return false;
            }

            let sum: u32 = code
                .chars()
                .take(7)
                .enumerate()
                .map(|(i, c)| {
                    let digit = c.to_digit(10).unwrap();
                    if i % 2 == 0 {
                        digit * 3 // Odd position (1-indexed)
                    } else {
                        digit // Even position (1-indexed)
                    }
                })
                .sum();

            let check_digit = code.chars().nth(7).unwrap().to_digit(10).unwrap();
            let expected = (10 - (sum % 10)) % 10;

            check_digit == expected
        }

        /// Returns the type of the code (EAN-8, UPC-A, or EAN-13).
        pub fn code_type(&self) -> CodeType {
            match self.0.len() {
                8 => CodeType::Ean8,
                12 => CodeType::UpcA,
                13 => CodeType::Ean13,
                _ => CodeType::Ean13, // Default fallback
            }
        }

        /// Returns the raw EAN/UPC string.
        pub fn as_str(&self) -> &str {
            &self.0
        }

        /// Returns the check digit.
        pub fn check_digit(&self) -> u8 {
            self.0.chars().last().unwrap().to_digit(10).unwrap() as u8
        }

        /// Returns the country code (first 2-3 digits for EAN-13).
        pub fn country_code(&self) -> Option<&str> {
            match self.code_type() {
                CodeType::Ean13 => {
                    if self.0.starts_with('0') {
                        // UPC converted to EAN-13
                        Some("00") // USA/Canada
                    } else {
                        Some(&self.0[..3])
                    }
                }
                CodeType::Ean8 => Some(&self.0[..2]),
                CodeType::UpcA => Some("00"), // USA/Canada
            }
        }

        /// Returns the manufacturer code.
        pub fn manufacturer_code(&self) -> &str {
            match self.code_type() {
                CodeType::Ean13 => {
                    if self.0.starts_with('0') {
                        &self.0[1..6] // UPC manufacturer code
                    } else {
                        &self.0[3..8] // EAN manufacturer code
                    }
                }
                CodeType::UpcA => &self.0[..5],
                CodeType::Ean8 => &self.0[2..5],
            }
        }

        /// Returns the product code.
        pub fn product_code(&self) -> &str {
            match self.code_type() {
                CodeType::Ean13 => {
                    if self.0.starts_with('0') {
                        &self.0[6..11] // UPC product code
                    } else {
                        &self.0[8..12] // EAN product code
                    }
                }
                CodeType::UpcA => &self.0[5..10],
                CodeType::Ean8 => &self.0[5..7],
            }
        }

        /// Normalizes an EAN/UPC string by removing spaces and hyphens.
        pub fn normalize(value: &str) -> String {
            value.chars().filter(|c| c.is_ascii_digit()).collect()
        }

        /// Converts to a formatted display string with appropriate separators.
        pub fn formatted(&self) -> String {
            match self.code_type() {
                CodeType::Ean8 => {
                    format!("{}-{}-{}", &self.0[..2], &self.0[2..7], &self.0[7..])
                }
                CodeType::UpcA => {
                    format!(
                        "{}-{}-{}-{}",
                        &self.0[..1],
                        &self.0[1..6],
                        &self.0[6..11],
                        &self.0[11..]
                    )
                }
                CodeType::Ean13 => {
                    format!(
                        "{}-{}-{}-{}",
                        &self.0[..1],
                        &self.0[1..7],
                        &self.0[7..12],
                        &self.0[12..]
                    )
                }
            }
        }
    }

    impl FromStr for Ean {
        type Err = EanError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::new(s)
        }
    }

    impl fmt::Display for Ean {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl fmt::Display for CodeType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                CodeType::Ean8 => write!(f, "EAN-8"),
                CodeType::UpcA => write!(f, "UPC-A"),
                CodeType::Ean13 => write!(f, "EAN-13"),
            }
        }
    }

    impl AsRef<str> for Ean {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    impl From<Ean> for String {
        fn from(ean: Ean) -> String {
            ean.0
        }
    }
}

// Re-export API types based on features
#[cfg(feature = "std")]
pub use api::*;

// Re-export runtime types when runtime feature is enabled
#[cfg(feature = "runtime")]
pub use runtime_api::RuntimeEanError;

#[cfg(feature = "web")]
mod web_api {
    use super::Ean;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    impl Ean {
        /// Creates a new EAN from JavaScript.
        #[wasm_bindgen(constructor)]
        pub fn new_web(value: &str) -> Result<Ean, JsError> {
            Self::new(value).map_err(|e| JsError::new(&e.to_string()))
        }

        /// Creates an EAN from UPC.
        #[wasm_bindgen(js_name = fromUpc)]
        pub fn from_upc_web(upc: &str) -> Result<Ean, JsError> {
            Self::from_upc(upc).map_err(|e| JsError::new(&e.to_string()))
        }

        /// Validates an EAN/UPC string.
        #[wasm_bindgen(js_name = isValid)]
        pub fn is_valid_web(value: &str) -> bool {
            Self::validate(value).is_ok()
        }

        /// Returns the code type as a string.
        #[wasm_bindgen(js_name = codeType)]
        pub fn code_type_web(&self) -> String {
            self.code_type().to_string()
        }

        /// Returns the check digit.
        #[wasm_bindgen(js_name = checkDigit)]
        pub fn check_digit_web(&self) -> u8 {
            self.check_digit()
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

        /// Normalizes an EAN/UPC string.
        #[wasm_bindgen(js_name = normalize)]
        pub fn normalize_web(value: &str) -> String {
            Self::normalize(value)
        }
    }
}

#[cfg(feature = "runtime")]
mod runtime_api {
    use super::RuntimeEan;
    use frame_support::BoundedVec;

    #[cfg(not(feature = "std"))]
    extern crate alloc;

    #[cfg(not(feature = "std"))]
    use alloc::string::{String, ToString};

    /// Error types for RuntimeEan operations
    #[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
    pub enum RuntimeEanError {
        /// Data exceeds the 13-byte limit
        #[error("Data exceeds the 13-byte capacity limit")]
        ExceedsCapacity,
        /// Invalid UTF-8 data
        #[error("Invalid UTF-8 data")]
        InvalidUtf8,
    }

    impl RuntimeEan {
        /// Creates a new RuntimeEan from bytes without validation.
        pub fn new_from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, RuntimeEanError> {
            let bytes = bytes.as_ref();
            BoundedVec::try_from(bytes.to_vec())
                .map(Self)
                .map_err(|_| RuntimeEanError::ExceedsCapacity)
        }

        /// Creates a new RuntimeEan from a string slice without validation.
        pub fn new_from_str(value: &str) -> Result<Self, RuntimeEanError> {
            Self::new_from_bytes(value.as_bytes())
        }

        /// Returns the EAN as a byte slice.
        pub fn as_bytes(&self) -> &[u8] {
            &self.0
        }

        /// Converts the RuntimeEan to a string if it contains valid UTF-8.
        pub fn to_string_lossy(&self) -> String {
            String::from_utf8_lossy(&self.0).to_string()
        }

        /// Converts the RuntimeEan to a string if it contains valid UTF-8.
        pub fn to_string(&self) -> Result<String, RuntimeEanError> {
            String::from_utf8(self.0.to_vec()).map_err(|_| RuntimeEanError::InvalidUtf8)
        }

        /// Returns the maximum capacity.
        pub const fn capacity() -> u32 {
            13
        }

        /// Returns the current length in bytes.
        pub fn len(&self) -> usize {
            self.0.len()
        }

        /// Returns true if the RuntimeEan is empty.
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    }
}

#[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
impl RuntimeEan {
    /// Generate a benchmark instance
    pub fn generate_benchmark(i: u32) -> Self {
        use crate::benchmarking::create_bounded_string;
        Self(create_bounded_string::<13>(i))
    }
}
