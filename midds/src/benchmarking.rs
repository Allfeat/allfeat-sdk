//! Benchmarking utilities for MIDDS types
//!
//! This module provides utilities to generate MIDDS of varying sizes for Substrate benchmarking.
//! It follows Substrate's best practices of testing from minimum to maximum encoded length.

use crate::{Midds, MiddsId};
use frame_support::BoundedVec;

#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};
#[cfg(feature = "std")]
use std::{vec, vec::Vec};

/// Trait for generating MIDDS data structures for benchmarking purposes.
///
/// This trait provides methods to create instances of varying sizes, which is essential
/// for accurate weight calculation in Substrate pallets.
pub trait BenchmarkHelperT<T: Midds> {
    /// Generate a MIDDS instance with the minimum possible encoded size.
    /// This typically means empty BoundedVecs and minimal required fields.
    fn min_size() -> T;

    /// Generate a MIDDS instance with the maximum possible encoded size.
    /// This means filling all BoundedVecs to their capacity limits.
    fn max_size() -> T;

    /// Generate a MIDDS instance with a specific complexity factor.
    ///
    /// # Parameters
    /// - `complexity`: A value between 0.0 and 1.0 indicating the desired size
    ///   - 0.0 = minimum size
    ///   - 1.0 = maximum size
    ///   - 0.5 = roughly half the maximum capacity for variable-length fields
    fn variable_size(complexity: f32) -> T;

    /// Generate a realistic MIDDS instance for typical use cases.
    /// This is used as a baseline for common scenarios.
    fn typical_size() -> T {
        Self::variable_size(0.3) // 30% of max capacity is often realistic
    }
}

/// Utility functions for benchmark data generation
pub mod utils {
    use super::*;
    use frame_support::traits::Get;

    /// Generate a BoundedVec with a specific fill ratio
    pub fn bounded_vec_with_ratio<T: Clone + core::fmt::Debug, S: Get<u32>>(
        item: T,
        fill_ratio: f32,
    ) -> BoundedVec<T, S> {
        let max_len = S::get() as usize;
        let target_len = ((max_len as f32) * fill_ratio.clamp(0.0, 1.0)) as usize;

        let mut vec = Vec::with_capacity(target_len);
        for _ in 0..target_len {
            vec.push(item.clone());
        }

        BoundedVec::try_from(vec).expect("Length should be within bounds")
    }

    /// Generate a BoundedVec<u8> (string-like) with specific byte length
    pub fn bounded_string_with_length<S: Get<u32>>(
        length: usize,
        pattern: u8,
    ) -> BoundedVec<u8, S> {
        let max_len = S::get() as usize;
        let actual_length = length.min(max_len);

        let bytes = vec![pattern; actual_length];
        BoundedVec::try_from(bytes).expect("Length should be within bounds")
    }

    /// Generate a sequence of MIDDS IDs for testing
    pub fn midds_id_sequence(count: usize, start_id: MiddsId) -> Vec<MiddsId> {
        (start_id..start_id + count as u64).collect()
    }

    /// Calculate the target length for a BoundedVec based on complexity factor
    pub fn target_length_for_complexity<S: Get<u32>>(complexity: f32) -> usize {
        let max_len = S::get() as usize;
        ((max_len as f32) * complexity.clamp(0.0, 1.0)) as usize
    }
}

/// Pre-defined complexity steps useful for benchmark scenarios
pub mod complexity {
    /// Complexity steps commonly used in Substrate benchmarks
    /// These follow the standard pattern of testing from minimum to maximum complexity
    pub const STEPS: &[f32] = &[0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];

    /// Minimum complexity (empty/minimal data)
    pub const MIN: f32 = 0.0;

    /// Typical complexity for common use cases
    pub const TYPICAL: f32 = 0.3;

    /// Medium complexity
    pub const MEDIUM: f32 = 0.5;

    /// High complexity (near maximum)
    pub const HIGH: f32 = 0.8;

    /// Maximum complexity (all fields at max capacity)
    pub const MAX: f32 = 1.0;
}

/// Macro to generate a series of MIDDS instances for benchmarking
///
/// This macro helps generate test cases with different complexity levels
/// following Substrate's benchmarking best practices.
///
/// # Example
/// ```rust,ignore
/// use allfeat_midds::{benchmarking::*, track::Track};
///
/// let test_tracks = generate_benchmark_series!(Track, &[0.0, 0.5, 1.0]);
/// ```
#[macro_export]
macro_rules! generate_benchmark_series {
    ($midds_type:ty, $complexities:expr) => {{
        $complexities
            .iter()
            .map(|&complexity| {
                <$midds_type as $crate::Midds>::BenchmarkHelper::variable_size(complexity)
            })
            .collect::<Vec<$midds_type>>()
    }};
}

/// Convenience function to generate all standard complexity steps for a MIDDS type
pub fn generate_standard_series<T: Midds>() -> Vec<T> {
    complexity::STEPS
        .iter()
        .map(|&complexity| T::BenchmarkHelper::variable_size(complexity))
        .collect()
}
