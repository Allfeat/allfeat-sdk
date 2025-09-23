//! Utilities for BN254 field (`Fr`) encoding/decoding, Poseidon off-chain helpers,
//! and randomness helpers used by tests and off-chain tooling.
//!
//! This module provides:
//! - Hex <-> field conversions in **big-endian** with an explicit `"0x"` prefix on output.
//! - Off-chain Poseidon helpers (`poseidon_h4_offchain`, `poseidon_h2_offchain`) that mirror
//!   the in-circuit sponge flow (absorb → squeeze).
//! - Random `Fr` sampling via a caller-provided RNG (`no_std` compatible) and an
//!   OS-backed RNG behind `std`.
//!
//! # Endianness & Hex Format
//!
//! - `fr_to_hex_be` returns **fixed-width** `0x` + 64 hex chars (32 bytes), lowercase.
//! - `fr_from_hex_be` accepts both `"0x"`-prefixed and unprefixed hex, **big-endian**,
//!   zero-pads on the left to 32 bytes, and reduces modulo `Fr::MODULUS`.
//!
//! Keeping a canonical, fixed-width hex form on output simplifies off-chain/on-chain
//! comparisons and avoids ambiguity around leading zeros.

use ark_bn254::Fr;
use ark_crypto_primitives::sponge::{
    CryptographicSponge,
    poseidon::{PoseidonConfig, PoseidonSponge},
};
use ark_ff::{BigInteger, PrimeField, UniformRand};
use ark_std::rand::RngCore;

/// Convert an `Fr` into a **0x-prefixed, lowercase, big-endian, fixed-width** hex string.
///
/// - Always returns `0x` + **64** hex chars (32 bytes).
/// - Lowercase hex.
/// - Big-endian byte order.
///
/// This mirrors the inverse operation in [`fr_from_hex_be`].
pub fn fr_to_hex_be(x: &Fr) -> String {
    let be = x.into_bigint().to_bytes_be();
    let mut s = String::from("0x");
    s.push_str(&hex::encode(be));
    s
}

/// Parse a big-endian hex string into `Fr`, accepting `"0x"`-prefixed or bare hex.
///
/// - Trims an optional `"0x"` prefix.
/// - Decodes big-endian bytes, **left-pads to 32 bytes**, and then reduces mod `Fr::MODULUS`.
/// - Panics on malformed hex (intended for test/util contexts).
///
/// The output round-trips with [`fr_to_hex_be`] into a canonical, fixed-width form.
pub fn fr_from_hex_be(h: &str) -> Fr {
    let s = h.trim_start_matches("0x");
    let bytes = hex::decode(s).expect("invalid hex");
    let mut be = [0u8; 32];
    let off = 32 - bytes.len();
    be[off..].copy_from_slice(&bytes);
    Fr::from_be_bytes_mod_order(&be)
}

/// Convenience: `Fr` from `u64`.
pub fn fr_u64(x: u64) -> Fr {
    Fr::from(x)
}

/// Off-chain Poseidon helper over **4 inputs** (a,b,c,d) with the given config.
///
/// Mirrors the in-circuit sponge flow:
/// 1) `PoseidonSponge::new(cfg)`
/// 2) `absorb([a,b,c,d])`
/// 3) `squeeze_field_elements(1)[0]`
pub fn poseidon_h4_offchain(a: Fr, b: Fr, c: Fr, d: Fr, cfg: &PoseidonConfig<Fr>) -> Fr {
    let mut sp = PoseidonSponge::<Fr>::new(cfg);
    sp.absorb(&vec![a, b, c, d]);
    sp.squeeze_field_elements(1)[0]
}

/// Off-chain Poseidon helper over **2 inputs** (x,y) with the given config.
///
/// See [`poseidon_h4_offchain`] for the sponge flow; this variant absorbs only two elements.
pub fn poseidon_h2_offchain(x: Fr, y: Fr, cfg: &PoseidonConfig<Fr>) -> Fr {
    let mut sp = PoseidonSponge::<Fr>::new(cfg);
    sp.absorb(&vec![x, y]);
    sp.squeeze_field_elements(1)[0]
}

/// Sample a uniformly random `Fr` from a caller-provided RNG.
///
/// Works in `no_std` provided you pass an RNG that implements `RngCore`
/// (e.g., a ChaCha20 RNG).
pub fn secret_random<R: RngCore + ?Sized>(rng: &mut R) -> Fr {
    Fr::rand(rng)
}

/// Sample a random `Fr` using the OS RNG (available behind the `std` feature).
///
/// Intended for quick tests or tooling. For reproducible tests, prefer a
/// seeded RNG and [`secret_random`].
#[cfg(feature = "std")]
pub fn secret_os_random() -> Fr {
    use ark_ff::UniformRand;
    use ark_std::rand::thread_rng;

    let mut rng = thread_rng();
    Fr::rand(&mut rng)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::{One, Zero};
    use ark_std::rand::{SeedableRng, rngs::StdRng};

    // Deterministic, NON-secure Poseidon params for tests.
    // Make them non-symmetric so order-sensitivity holds.
    fn poseidon_test_params() -> PoseidonConfig<Fr> {
        let full_rounds: usize = 8;
        let partial_rounds: usize = 57;
        let alpha: u64 = 5; // <-- must be u64

        let rate: usize = 2;
        let capacity: usize = 1;
        let width = rate + capacity; // 3

        // Non-symmetric 3x3 MDS
        let mds: Vec<Vec<Fr>> = (0..width)
            .map(|r| {
                (0..width)
                    .map(|c| {
                        let base: u64 = (r as u64) * 3 + (c as u64) + 1;
                        // tiny tweak to break patterns
                        Fr::from(base + if r == 2 && c == 2 { 1 } else { 0 })
                    })
                    .collect()
            })
            .collect();

        // Round constants: distinct per round & lane
        let total_rounds = full_rounds + partial_rounds;
        let ark: Vec<Vec<Fr>> = (0..total_rounds)
            .map(|round| {
                (0..width)
                    .map(|i| {
                        let v = (round as u64) * 17 + (i as u64) * 5 + 1;
                        Fr::from(v)
                    })
                    .collect()
            })
            .collect();

        PoseidonConfig::new(full_rounds, partial_rounds, alpha, mds, ark, rate, capacity)
    }

    #[test]
    fn fr_to_hex_be_roundtrip_small_values() {
        for v in [0u64, 1, 2, 10, 255, 256, 65535] {
            let x = Fr::from(v);
            let s = fr_to_hex_be(&x);
            // Fixed-width: 0x + 64 hex chars
            assert!(s.starts_with("0x"));
            assert_eq!(s.len(), 66, "must always be 32-byte hex (64 chars) plus 0x");
            assert_eq!(s, s.to_lowercase(), "hex must be lowercase");
            // Roundtrip
            let y = fr_from_hex_be(&s);
            assert_eq!(x, y);
        }
    }

    #[test]
    fn fr_to_hex_be_normalizes_leading_zeros_on_input() {
        // Input "0x01" must be re-encoded as full 32-byte hex
        let x = fr_from_hex_be("0x01");
        let s = fr_to_hex_be(&x);
        assert_eq!(
            s,
            "0x0000000000000000000000000000000000000000000000000000000000000001"
        );
        assert_eq!(fr_from_hex_be(&s), x);
    }

    #[test]
    fn fr_to_hex_be_roundtrip_random_values() {
        use ark_std::rand::{SeedableRng, rngs::StdRng};
        let mut rng = StdRng::seed_from_u64(1337);
        for _ in 0..32 {
            let x = Fr::rand(&mut rng);
            let s = fr_to_hex_be(&x);
            assert!(s.starts_with("0x"));
            assert_eq!(s, s.to_lowercase());
            let y = fr_from_hex_be(&s);
            assert_eq!(x, y);
        }
    }

    #[test]
    fn fr_to_hex_be_zero_is_canonical_fixed_width() {
        let z = Fr::from(0u64);
        let s = fr_to_hex_be(&z);

        // Always "0x" + 64 hex chars (32 bytes), all zeros for Fr::zero()
        assert_eq!(
            s,
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(s.len(), 66);
        assert_eq!(fr_from_hex_be(&s), z);
    }

    #[test]
    fn fr_to_hex_be_handles_full_32byte_values() {
        // A 32-byte big-endian value with a non-zero top byte.
        // (This is > u128; ensures we're not accidentally truncating.)
        let h = "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
        let x = fr_from_hex_be(h);
        let s = fr_to_hex_be(&x);

        // Re-encoding may or may not stay at full 32 bytes depending on modulus reduction,
        // but it must roundtrip to the same field element.
        assert!(s.starts_with("0x"));
        assert_eq!(fr_from_hex_be(&s), x);
    }

    #[test]
    fn fr_from_hex_be_parses_prefixed_and_unprefixed() {
        // "0x01" and "01" should both parse to 1
        let a = fr_from_hex_be("0x01");
        let b = fr_from_hex_be("01");
        assert_eq!(a, Fr::one());
        assert_eq!(b, Fr::one());
    }

    #[test]
    fn fr_from_hex_be_pads_short_inputs_big_endian() {
        // Single byte 0xab should be interpreted as BE => value 171
        let x = fr_from_hex_be("0xab");
        assert_eq!(x, Fr::from(171u64));

        // 32 bytes of 0x00..01 => still 1
        let y = fr_from_hex_be("0000000000000000000000000000000000000000000000000000000000000001");
        assert_eq!(y, Fr::one());
    }

    #[test]
    fn fr_u64_matches_field_from() {
        for v in [0u64, 1, 2, 10, u32::MAX as u64, u64::from(u32::MAX) + 1] {
            assert_eq!(fr_u64(v), Fr::from(v));
        }
        // Basic algebra sanity: Fr::from(2) + Fr::from(3) == Fr::from(5)
        assert_eq!(fr_u64(2) + fr_u64(3), fr_u64(5));
    }

    #[test]
    fn poseidon_h2_offchain_is_deterministic_and_order_sensitive() {
        let cfg = poseidon_test_params();
        let x = fr_u64(123);
        let y = fr_u64(456);

        let h1 = poseidon_h2_offchain(x, y, &cfg);
        let h2 = poseidon_h2_offchain(x, y, &cfg);
        assert_eq!(h1, h2, "same inputs must yield same hash");

        let h_swapped = poseidon_h2_offchain(y, x, &cfg);
        assert_ne!(h1, h_swapped, "hash must be order-sensitive");
    }

    #[test]
    fn poseidon_h4_offchain_is_deterministic_and_order_sensitive() {
        let cfg = poseidon_test_params();
        let a = fr_u64(1);
        let b = fr_u64(2);
        let c = fr_u64(3);
        let d = fr_u64(4);

        let h1 = poseidon_h4_offchain(a, b, c, d, &cfg);
        let h2 = poseidon_h4_offchain(a, b, c, d, &cfg);
        assert_eq!(h1, h2, "same 4-tuple must yield same hash");

        let h_perm = poseidon_h4_offchain(a, b, d, c, &cfg);
        assert_ne!(h1, h_perm, "permutations should change the hash");
    }

    #[test]
    fn poseidon_h4_consistency_with_manual_sponge_flow() {
        // Ensure poseidon_h4_offchain equals doing the same with a raw PoseidonSponge
        let cfg = poseidon_test_params();
        let a = fr_u64(10);
        let b = fr_u64(20);
        let c = fr_u64(30);
        let d = fr_u64(40);

        let via_helper = poseidon_h4_offchain(a, b, c, d, &cfg);

        let mut sp = PoseidonSponge::<Fr>::new(&cfg);
        sp.absorb(&vec![a, b, c, d]);
        let via_manual = sp.squeeze_field_elements(1)[0];

        assert_eq!(via_helper, via_manual);
    }

    #[test]
    fn poseidon_h2_consistency_with_manual_sponge_flow() {
        let cfg = poseidon_test_params();
        let x = fr_u64(777);
        let y = fr_u64(888);

        let via_helper = poseidon_h2_offchain(x, y, &cfg);

        let mut sp = PoseidonSponge::<Fr>::new(&cfg);
        sp.absorb(&vec![x, y]);
        let via_manual = sp.squeeze_field_elements(1)[0];

        assert_eq!(via_helper, via_manual);
    }

    #[test]
    fn secret_random_is_uniform_over_rng_state() {
        // Same seed => same stream => same first value
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);
        let s1 = secret_random(&mut rng1);
        let s2 = secret_random(&mut rng2);
        assert_eq!(s1, s2, "same seed should produce same first element");

        // Different seed => very likely different first value
        let mut rng3 = StdRng::seed_from_u64(43);
        let s3 = secret_random(&mut rng3);
        assert_ne!(
            s1, s3,
            "different seed should produce different first element"
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn secret_os_random_produces_nontrivial_values() {
        // Not strictly guaranteed to be distinct, but extremely likely.
        let a = secret_os_random();
        let b = secret_os_random();
        // Sanity: values are in field (always true), and likely different.
        assert!(
            a != Fr::zero() || b != Fr::zero(),
            "both zero is extremely unlikely"
        );
        assert_ne!(
            a, b,
            "OS randomness should produce different values most of the time"
        );
    }
}
