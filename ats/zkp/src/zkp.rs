//! Groth16 proof system helpers (BN254).
//!
//! This module provides a high-level API for working with Groth16 proofs
//! over the BN254 curve, wrapping the low-level Arkworks primitives.
//!
//! # Features
//!
//! - Key generation ([`setup`]): produce proving and verifying keys.
//! - Proof generation ([`prove`]): create proofs from witness + public inputs.
//! - Proof verification ([`verify`]): check proofs against prepared verifying keys.
//!
//! # Public vs Witness inputs
//!
//! - [`PublicInputs`] (public): `hash_title, hash_audio, hash_creators, commitment, timestamp, nullifier`
//!   Must always appear in this exact order for the circuit and verifier.
//! - [`Witness`] (private): the `secret` field element.

use crate::circuit::Circuit;
use crate::error::{Result, ZkpError};
use crate::{Curve, fr_from_hex_be, fr_to_hex_be};
use ark_bn254::Fr;
use ark_groth16::{Groth16, Proof, ProvingKey, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

/// Strip leading `0x` from a hex string if present.
fn strip_0x(s: &str) -> &str {
    s.strip_prefix("0x").unwrap_or(s)
}

/// Decode a hex string (with or without `0x`) into raw bytes.
///
/// Returns [`ZkpError::InvalidHex`] on malformed hex.
fn hex_to_bytes(s: &str) -> Result<Vec<u8>> {
    hex::decode(strip_0x(s)).map_err(|_| ZkpError::InvalidHex)
}

/// Encode raw bytes into a `0x`-prefixed lowercase hex string.
fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(2 + bytes.len() * 2);
    out.push_str("0x");
    out.push_str(&hex::encode(bytes));
    out
}

/// ---------- internal inputs (kept private) ----------

#[derive(Clone, Copy)]
struct PublicInputs {
    hash_title: Fr,
    hash_audio: Fr,
    hash_creators: Fr,
    commitment: Fr,
    timestamp: Fr,
    nullifier: Fr,
}

#[derive(Clone, Copy)]
struct Witness {
    secret: Fr,
}

fn decode_publics_hex(publics: &[&str]) -> Result<[Fr; 6]> {
    if publics.len() != 6 {
        return Err(ZkpError::WrongPublicInputCount);
    }
    Ok([
        fr_from_hex_be(publics[0])?,
        fr_from_hex_be(publics[1])?,
        fr_from_hex_be(publics[2])?,
        fr_from_hex_be(publics[3])?,
        fr_from_hex_be(publics[4])?,
        fr_from_hex_be(publics[5])?,
    ])
}

// ---------- public: hex-only SETUP ----------

/// Generate PK/VK from hex inputs.
/// Inputs:
/// - `secret`: 0x-hex Fr
/// - `publics`: 6 x 0x-hex Fr in circuit order:
///   [hash_title, hash_audio, hash_creators, commitment, timestamp, nullifier]
/// Output: (pk, vk)
#[cfg(feature = "std")]
pub fn setup(secret: &str, publics: &[&str]) -> Result<(String, String)> {
    // Decode
    let secret = fr_from_hex_be(secret)?;
    let arr = decode_publics_hex(publics)?;
    let p = PublicInputs {
        hash_title: arr[0],
        hash_audio: arr[1],
        hash_creators: arr[2],
        commitment: arr[3],
        timestamp: arr[4],
        nullifier: arr[5],
    };
    let w = Witness { secret };

    // Build circuit sized by example inputs
    let circuit = Circuit {
        secret: w.secret,
        hash_title: p.hash_title,
        hash_audio: p.hash_audio,
        hash_creators: p.hash_creators,
        commitment: p.commitment,
        timestamp: p.timestamp,
        nullifier: p.nullifier,
    };

    // Groth16 setup
    let mut rng = rand::rngs::OsRng;
    let pk = Groth16::<Curve>::generate_random_parameters_with_reduction(circuit, &mut rng)
        .map_err(|_| ZkpError::ProofGenerationFailed)?;
    let vk = pk.vk.clone();

    // Serialize (compressed) -> hex
    let mut pk_bytes = Vec::new();
    pk.serialize_compressed(&mut pk_bytes)
        .map_err(|_| ZkpError::SerializationFailed)?;
    let mut vk_bytes = Vec::new();
    vk.serialize_compressed(&mut vk_bytes)
        .map_err(|_| ZkpError::SerializationFailed)?;

    Ok((bytes_to_hex(&pk_bytes), bytes_to_hex(&vk_bytes)))
}

// ---------- public: hex-only PROVE ----------

/// Create a proof from hex:
/// - `pk`: 0x-hex compressed PK
/// - `secret`: 0x-hex Fr
/// - `publics`: 6 x 0x-hex Fr (circuit order)
/// Returns: (proof, publics_out[6])
#[cfg(feature = "std")]
pub fn prove(pk: &str, secret: &str, publics: &[&str]) -> Result<(String, [String; 6])> {
    // PK
    let pk_bytes = hex_to_bytes(pk)?;
    let pk = ProvingKey::<Curve>::deserialize_compressed(&pk_bytes[..])
        .map_err(|_| ZkpError::DeserializationFailed)?;

    // Inputs
    let secret = fr_from_hex_be(secret)?;
    let arr = decode_publics_hex(publics)?;
    let p = PublicInputs {
        hash_title: arr[0],
        hash_audio: arr[1],
        hash_creators: arr[2],
        commitment: arr[3],
        timestamp: arr[4],
        nullifier: arr[5],
    };

    // Circuit
    let circuit = Circuit {
        secret,
        hash_title: p.hash_title,
        hash_audio: p.hash_audio,
        hash_creators: p.hash_creators,
        commitment: p.commitment,
        timestamp: p.timestamp,
        nullifier: p.nullifier,
    };

    // Proof
    let mut rng = rand::rngs::OsRng;
    let proof = Groth16::<Curve>::create_random_proof_with_reduction(circuit, &pk, &mut rng)
        .map_err(|_| ZkpError::ProofGenerationFailed)?;

    // Serialize proof + echo publics as hex
    let mut proof_bytes = Vec::new();
    proof
        .serialize_compressed(&mut proof_bytes)
        .map_err(|_| ZkpError::SerializationFailed)?;
    let proof = bytes_to_hex(&proof_bytes);

    let publics_out = [
        fr_to_hex_be(&arr[0]),
        fr_to_hex_be(&arr[1]),
        fr_to_hex_be(&arr[2]),
        fr_to_hex_be(&arr[3]),
        fr_to_hex_be(&arr[4]),
        fr_to_hex_be(&arr[5]),
    ];

    Ok((proof, publics_out))
}

// ---------- public: hex-only VERIFY ----------

/// Verify from hex:
/// - `vk`: 0x-hex compressed VK
/// - `proof`: 0x-hex compressed proof
/// - `publics`: 6 x 0x-hex Fr
pub fn verify(vk: &str, proof: &str, publics: &[&str]) -> Result<bool> {
    let vk_bytes = hex_to_bytes(vk)?;
    let proof_bytes = hex_to_bytes(proof)?;
    let vk = VerifyingKey::<Curve>::deserialize_compressed(&vk_bytes[..])
        .map_err(|_| ZkpError::DeserializationFailed)?;
    let proof = Proof::<Curve>::deserialize_compressed(&proof_bytes[..])
        .map_err(|_| ZkpError::DeserializationFailed)?;

    // Decode publics
    let arr = decode_publics_hex(publics)?;
    let ok = Groth16::<Curve>::verify_proof(&ark_groth16::prepare_verifying_key(&vk), &proof, &arr)
        .map_err(|_| ZkpError::VerificationError)?;

    Ok(ok)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fr_to_hex_be;

    // If these helpers live in another module, adjust imports accordingly:
    use crate::utils::{fr_u64, poseidon_commitment_offchain, poseidon_nullifier_offchain};
    // If your helpers expect a config, expose or re-export your params function.
    // Here we assume you re-exported it as `crate::poseidon_params`.
    use crate::circuit::poseidon_params;

    /// Build a consistent example as hex strings:
    /// returns (secret, publics[6]) with publics in circuit order:
    /// [hash_title, hash_audio, hash_creators, commitment, timestamp, nullifier]
    fn example_hex() -> Result<(String, [String; 6])> {
        let cfg = poseidon_params();

        // Example values (same as your earlier unit tests)
        let secret =
            "0x23864adb160dddf590f1d3303683ebcb914f828e2635f6e85a32f0a1aecd3dd8".to_string();
        let hash_title =
            "0x175eeef716d52cf8ee972c6fefd60e47df5084efde3c188c40a81a42e72dfb04".to_string();
        let hash_audio =
            "0x26d273f7c73a635f6eaeb904e116ec4cd887fb5a87fc7427c95279e6053e5bf0".to_string();
        let hash_creators =
            "0x017ac5e7a52bec07ca8ee344a9979aa083b7713f1196af35310de21746985079".to_string();
        let timestamp = fr_to_hex_be(&fr_u64(10_000));

        // Compute publics off-chain with the same Poseidon config
        let commitment =
            poseidon_commitment_offchain(&hash_title, &hash_audio, &hash_creators, &secret, &cfg)?;
        let nullifier = poseidon_nullifier_offchain(&commitment, &timestamp, &cfg)?;

        let publics = [
            hash_title,
            hash_audio,
            hash_creators,
            commitment,
            timestamp,
            nullifier,
        ];

        Ok((secret, publics))
    }

    #[test]
    #[cfg(feature = "std")]
    fn setup_prove_verify_roundtrip() -> Result<()> {
        let (secret, publics) = example_hex()?;
        let publics_refs: Vec<&str> = publics.iter().map(|s| s.as_str()).collect();

        // 1) Setup (PK/VK as hex)
        let (pk, vk) = setup(&secret, &publics_refs)?;

        // 2) Prove (proof + echo publics as hex)
        let (proof, publics_echo) = prove(&pk, &secret, &publics_refs)?;

        // Make sure publics echoed by prove() match the input publics
        assert_eq!(publics_echo.as_slice(), publics.as_slice());

        // 3) Verify using hex API
        let ok = verify(&vk, &proof, &publics_refs)?;
        assert!(ok, "verification should succeed");
        Ok(())
    }

    #[test]
    fn verify_fails_with_tampered_publics() -> Result<()> {
        let (secret, publics) = example_hex()?;
        let mut publics_refs: Vec<&str> = publics.iter().map(|s| s.as_str()).collect();

        // Setup + Prove with correct publics
        let (pk, vk) = setup(&secret, &publics_refs)?;
        let (proof, _) = prove(&pk, &secret, &publics_refs)?;

        // Tamper with the timestamp (public input mismatch)
        let tampered = fr_to_hex_be(&fr_u64(10_001)); // <-- keep it alive
        publics_refs[4] = &tampered;

        let ok = verify(&vk, &proof, &publics_refs)?;

        assert!(
            !ok,
            "verification should fail when public inputs are inconsistent"
        );
        Ok(())
    }

    // ---------- helper/utility coverage ----------

    #[test]
    fn hex_utils_roundtrip() -> Result<()> {
        // bytes_to_hex -> hex_to_bytes roundtrip
        let data = vec![0u8, 1, 2, 0xaa, 0xff, 0x10, 0x00];
        let hx = super::bytes_to_hex(&data);
        assert!(hx.starts_with("0x"));
        let back = super::hex_to_bytes(&hx)?;
        assert_eq!(back, data);

        // strip_0x correctness (both with and without prefix)
        let no0x = "deadbeef";
        let with0x = "0xdeadbeef";
        assert_eq!(super::strip_0x(no0x), "deadbeef");
        assert_eq!(super::strip_0x(with0x), "deadbeef");
        Ok(())
    }
}
