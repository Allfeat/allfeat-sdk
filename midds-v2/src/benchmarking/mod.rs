//! Benchmarking utilities for MIDDS types
//!
//! This module provides a unified system for generating benchmark instances of MIDDS types
//! with variable complexity, allowing Substrate weight benchmarking across the full range
//! of possible data sizes.

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{string::String, vec, vec::Vec};

pub use midds_impls::{
    MusicalWorkBenchmarkHelper, RecordingBenchmarkHelper, ReleaseBenchmarkHelper,
};

/// Main trait for MIDDS benchmarking helpers
///
/// Provides a unified interface for generating benchmark instances of varying complexity
/// suitable for Substrate weight calculations.
pub trait BenchmarkHelper<T> {
    /// Generates a benchmark instance with the given complexity
    ///
    /// # Arguments
    /// * `complexity` - 0 = minimal, u32::MAX = theoretical maximum
    fn benchmark_instance(complexity: u32) -> T;
}

/// Utilities for mapping complexity to concrete parameters
pub struct BenchmarkMapper;
impl BenchmarkMapper {
    /// Maps linear complexity to string length
    ///
    /// # Arguments
    /// * `complexity` - Input complexity (0 to u32::MAX)
    /// * `max_length` - Maximum allowed length
    ///
    /// # Returns
    /// String length proportional to complexity
    pub fn complexity_to_string_length(complexity: u32, max_length: u32) -> u32 {
        if complexity == 0 {
            1 // Minimum non-zero length
        } else {
            let result = ((complexity as u64 * max_length as u64) / u32::MAX as u64) as u32;
            result.max(1).min(max_length)
        }
    }

    /// Maps complexity to collection size
    ///
    /// # Arguments
    /// * `complexity` - Input complexity (0 to u32::MAX)
    /// * `max_size` - Maximum allowed size
    ///
    /// # Returns
    /// Collection size proportional to complexity
    pub fn complexity_to_collection_size(complexity: u32, max_size: u32) -> u32 {
        if complexity == 0 {
            1 // Minimum size for non-empty collections
        } else {
            let result = ((complexity as u64 * max_size as u64) / u32::MAX as u64) as u32;
            result.max(1).min(max_size)
        }
    }

    /// Generates a string of specific size for benchmarking
    pub fn benchmark_string(length: u32) -> String {
        if length == 0 {
            String::new()
        } else {
            "x".repeat(length as usize)
        }
    }

    /// Generates a sequential ID based on complexity and index
    pub fn complexity_to_id(complexity: u32, index: u32) -> u64 {
        // Generates unique but deterministic IDs
        ((complexity as u64) << 32) | (index as u64)
    }

    /// Divides complexity between multiple components
    ///
    /// Useful for distributing total complexity between different struct fields
    pub fn split_complexity(complexity: u32, parts: u32) -> Vec<u32> {
        if parts == 0 {
            return vec![];
        }

        let base = complexity / parts;
        let remainder = complexity % parts;

        (0..parts)
            .map(|i| if i < remainder { base + 1 } else { base })
            .collect()
    }

    /// Generates a boolean based on complexity
    pub fn complexity_to_bool(complexity: u32) -> bool {
        (complexity % 2) == 1
    }

    /// Generates an optional value based on complexity
    pub fn complexity_to_optional<T, F>(complexity: u32, generator: F) -> Option<T>
    where
        F: FnOnce(u32) -> T,
    {
        if complexity == 0 {
            None
        } else {
            Some(generator(complexity))
        }
    }
}

/// Macro to easily create BenchmarkHelper structs for primitive types
#[macro_export]
macro_rules! impl_primitive_benchmark_helper {
    ($helper_name:ident, $type:ty, $default_value:expr) => {
        pub struct $helper_name;

        impl $crate::benchmarking::BenchmarkHelper<$type> for $helper_name {
            fn benchmark_instance(_complexity: u32) -> $type {
                $default_value
            }
        }
    };
    ($helper_name:ident, $type:ty, $closure:expr) => {
        pub struct $helper_name;

        impl $crate::benchmarking::BenchmarkHelper<$type> for $helper_name {
            fn benchmark_instance(complexity: u32) -> $type {
                ($closure)(complexity)
            }
        }
    };
}

/// Macro to create BenchmarkHelper for simple enums
#[macro_export]
macro_rules! impl_enum_benchmark_helper {
    ($helper_name:ident, $enum:ty, [$($variant:expr),+ $(,)?]) => {
        pub struct $helper_name;

        impl $crate::benchmarking::BenchmarkHelper<$enum> for $helper_name {
            fn benchmark_instance(complexity: u32) -> $enum {
                let variants = [$($variant),+];
                let index = complexity as usize % variants.len();
                variants[index]
            }
        }
    };
}

/// Macro to create a BenchmarkHelper implementation for a struct
#[macro_export]
macro_rules! impl_struct_benchmark_helper {
    (
        $helper_name:ident,
        $struct_name:ty,
        {
            $(
                $field:ident: $field_expr:expr
            ),+ $(,)?
        }
    ) => {
        pub struct $helper_name;
        impl $crate::benchmarking::BenchmarkHelper<$struct_name> for $helper_name {
            fn benchmark_instance(complexity: u32) -> $struct_name {
                use $crate::benchmarking::BenchmarkMapper;

                <$struct_name> {
                    $(
                        $field: $field_expr,
                    )+
                }
            }
        }
    };
}

// MIDDS implementations
mod midds_impls;

#[cfg(test)]
mod tests {
    use super::*;
    use parity_scale_codec::Encode;

    // Import benchmark helpers for tests

    #[cfg(feature = "runtime-benchmarks")]
    use super::midds_impls::{
        MiddsStringBenchmarkHelper, MusicalWorkBenchmarkHelper, PartyIdBenchmarkHelper,
        RecordingBenchmarkHelper, ReleaseBenchmarkHelper,
    };

    #[test]
    fn test_complexity_to_string_length() {
        assert_eq!(BenchmarkMapper::complexity_to_string_length(0, 100), 1);
        assert_eq!(
            BenchmarkMapper::complexity_to_string_length(u32::MAX, 100),
            100
        );

        let mid_complexity = u32::MAX / 2;
        let result = BenchmarkMapper::complexity_to_string_length(mid_complexity, 100);
        assert!(result > 40 && result < 60); // Approximately halfway
    }

    #[test]
    fn test_complexity_to_collection_size() {
        assert_eq!(BenchmarkMapper::complexity_to_collection_size(0, 50), 1);
        assert_eq!(
            BenchmarkMapper::complexity_to_collection_size(u32::MAX, 50),
            50
        );
    }

    #[test]
    fn test_split_complexity() {
        let splits = BenchmarkMapper::split_complexity(10, 3);
        assert_eq!(splits.len(), 3);
        assert_eq!(splits.iter().sum::<u32>(), 10);

        let splits = BenchmarkMapper::split_complexity(7, 3);
        assert_eq!(splits, vec![3, 2, 2]);
    }

    #[test]
    fn test_benchmark_string() {
        assert_eq!(BenchmarkMapper::benchmark_string(0), "");
        assert_eq!(BenchmarkMapper::benchmark_string(5), "xxxxx");
    }

    #[test]
    #[cfg(feature = "runtime-benchmarks")]
    fn test_midds_benchmark_musical_work() {
        // Test with different complexities
        let minimal = MusicalWorkBenchmarkHelper::benchmark_instance(0);
        let medium = MusicalWorkBenchmarkHelper::benchmark_instance(u32::MAX / 2);
        let maximal = MusicalWorkBenchmarkHelper::benchmark_instance(u32::MAX);

        // Verify that encoding sizes increase with complexity
        let minimal_size = minimal.encoded_size();
        let medium_size = medium.encoded_size();
        let maximal_size = maximal.encoded_size();

        assert!(minimal_size <= medium_size);
        assert!(medium_size <= maximal_size);
    }

    #[test]
    #[cfg(feature = "runtime-benchmarks")]
    fn test_midds_benchmark_recording() {
        let minimal = RecordingBenchmarkHelper::benchmark_instance(0);
        let maximal = RecordingBenchmarkHelper::benchmark_instance(u32::MAX);

        // Verify that collections grow with complexity
        assert!(minimal.producers.len() <= maximal.producers.len());
        assert!(minimal.performers.len() <= maximal.performers.len());
        assert!(minimal.contributors.len() <= maximal.contributors.len());
        assert!(minimal.title_aliases.len() <= maximal.title_aliases.len());
    }

    #[test]
    #[cfg(feature = "runtime-benchmarks")]
    fn test_midds_benchmark_release() {
        let minimal = ReleaseBenchmarkHelper::benchmark_instance(0);
        let maximal = ReleaseBenchmarkHelper::benchmark_instance(u32::MAX);

        // Verify that collections grow with complexity
        assert!(minimal.producers.len() <= maximal.producers.len());
        assert!(minimal.recordings.len() <= maximal.recordings.len());
        assert!(minimal.cover_contributors.len() <= maximal.cover_contributors.len());
        assert!(minimal.title_aliases.len() <= maximal.title_aliases.len());
    }

    #[test]
    #[cfg(feature = "runtime-benchmarks")]
    fn test_midds_benchmark_party_id() {
        use crate::shared::PartyId;

        let minimal = PartyIdBenchmarkHelper::benchmark_instance(0);
        let _medium = PartyIdBenchmarkHelper::benchmark_instance(u32::MAX / 2);
        let maximal = PartyIdBenchmarkHelper::benchmark_instance(u32::MAX);

        // Minimal should be simple IPI
        assert!(matches!(minimal, PartyId::Ipi(_)));

        // Maximal should have both
        assert!(matches!(maximal, PartyId::Both(_)));
    }

    #[test]
    #[cfg(feature = "runtime-benchmarks")]
    fn test_midds_string_benchmark() {
        type TestStringHelper = MiddsStringBenchmarkHelper<100>;

        let minimal = TestStringHelper::benchmark_instance(0);
        let maximal = TestStringHelper::benchmark_instance(u32::MAX);

        // Verify that length increases with complexity
        assert!(minimal.len() <= maximal.len());
        assert_eq!(minimal.len(), 1); // Minimum non-zero
        assert_eq!(maximal.len(), 100); // Maximum
    }

    #[test]
    fn test_benchmark_mapper_utilities() {
        // Test complexity_to_id
        let id1 = BenchmarkMapper::complexity_to_id(1000, 5);
        let id2 = BenchmarkMapper::complexity_to_id(1000, 6);
        assert_ne!(id1, id2); // Different indices give different IDs

        // Test complexity_to_bool
        assert_eq!(BenchmarkMapper::complexity_to_bool(0), false);
        assert_eq!(BenchmarkMapper::complexity_to_bool(1), true);
        assert_eq!(BenchmarkMapper::complexity_to_bool(2), false);

        // Test complexity_to_optional
        let none_result: Option<u32> = BenchmarkMapper::complexity_to_optional(0, |c| c + 1);
        let some_result: Option<u32> = BenchmarkMapper::complexity_to_optional(10, |c| c + 1);
        assert_eq!(none_result, None);
        assert_eq!(some_result, Some(11));
    }
}
