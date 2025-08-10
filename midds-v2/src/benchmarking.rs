//! Benchmarking utilities for MIDDS types in runtime mode.
//!
//! This module provides the `BenchmarkHelper` trait that allows pallets
//! to generate different size scenarios for linear benchmarking purposes.
//! All functionality is gated behind both `runtime` and `runtime-benchmarks` features.

#[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
use frame_support::{traits::ConstU32, BoundedVec};

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::vec;

/// Trait for generating benchmark instances of the three main MIDDS types.
///
/// This trait provides methods to create instances with linear scaling
/// for comprehensive benchmarking in Substrate pallets. The parameter `i`
/// represents the linear variable that scales from 0 to X in benchmarks.
#[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
pub trait BenchmarkHelper<T> {
    /// Create an instance for linear benchmarking with parameter `i`.
    ///
    /// The parameter `i` should be used to scale dynamic data sizes linearly,
    /// allowing Substrate benchmarks to measure performance across different
    /// data sizes from 0 to maximum bounds.
    fn benchmark_instance(i: u32) -> T;
}

/// Helper function to create BoundedVec<u8, N> from string with linear scaling
#[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
pub fn create_bounded_string<const N: u32>(i: u32) -> BoundedVec<u8, ConstU32<N>> {
    let size = if i == 0 {
        1
    } else {
        (i as usize).min(N as usize)
    };
    let content = "A".repeat(size);
    BoundedVec::try_from(content.as_bytes().to_vec())
        .expect("BenchmarkHelper should create valid bounded strings")
}

/// Helper function to create BoundedVec<T, N> with linear scaling
#[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
pub fn create_bounded_vec<T: Clone + core::fmt::Debug, const N: u32>(
    item: T,
    i: u32,
) -> BoundedVec<T, ConstU32<N>> {
    let count = if i == 0 {
        1
    } else {
        (i as usize).min(N as usize)
    };
    BoundedVec::try_from(vec![item; count])
        .expect("BenchmarkHelper should create valid bounded vecs")
}

/// Helper function to create Option<BoundedVec<u8, N>> with linear scaling
#[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
pub fn create_optional_bounded_string<const N: u32>(i: u32) -> Option<BoundedVec<u8, ConstU32<N>>> {
    if i == 0 {
        None // For i=0, return None to test the minimal case
    } else {
        Some(create_bounded_string::<N>(i))
    }
}

/// Helper function to create Option<BoundedVec<T, N>> with linear scaling
#[cfg(all(feature = "runtime", feature = "runtime-benchmarks"))]
pub fn create_optional_bounded_vec<T: Clone + core::fmt::Debug, const N: u32>(
    item: T,
    i: u32,
) -> Option<BoundedVec<T, ConstU32<N>>> {
    if i == 0 {
        None // For i=0, return None to test the minimal case
    } else {
        Some(create_bounded_vec::<T, N>(item, i))
    }
}

#[cfg(all(feature = "runtime", feature = "runtime-benchmarks", test))]
mod tests {
    use super::*;

    #[test]
    fn test_create_bounded_string() {
        let zero = create_bounded_string::<10>(0);
        assert_eq!(zero.len(), 1); // Minimum size is 1

        let small = create_bounded_string::<10>(5);
        assert_eq!(small.len(), 5);

        let max_bounded = create_bounded_string::<10>(15);
        assert_eq!(max_bounded.len(), 10); // Capped at bound N
    }

    #[test]
    fn test_create_bounded_vec() {
        let zero = create_bounded_vec::<u32, 10>(42, 0);
        assert_eq!(zero.len(), 1); // Minimum size is 1
        assert_eq!(zero[0], 42);

        let small = create_bounded_vec::<u32, 10>(42, 5);
        assert_eq!(small.len(), 5);
        assert_eq!(small[0], 42);

        let max_bounded = create_bounded_vec::<u32, 10>(42, 15);
        assert_eq!(max_bounded.len(), 10); // Capped at bound N
    }

    #[test]
    fn test_create_optional_bounded_string() {
        let zero = create_optional_bounded_string::<10>(0);
        assert!(zero.is_none()); // For i=0, return None

        let some = create_optional_bounded_string::<10>(5);
        assert!(some.is_some());
        assert_eq!(some.unwrap().len(), 5);

        let max_bounded = create_optional_bounded_string::<10>(15);
        assert!(max_bounded.is_some());
        assert_eq!(max_bounded.unwrap().len(), 10); // Capped at bound N
    }
}
