//! Poseidon-based commitment & nullifier circuit (BN254, Groth16).
//!
//! This module defines an R1CS circuit that enforces two relations:
//!
//! 1) `commitment = Poseidon(hash_audio, hash_title, hash_creators, secret)`
//! 2) `nullifier  = Poseidon(commitment, timestamp)`
//!
//! The circuit uses the Poseidon sponge both as R1CS gadgets (in-circuit) and
//! off-chain for computing the expected public inputs.
//!
//! # Circuit Inputs
//!
//! - **Witness (private):**
//!   - `secret`: `Fr`
//!
//! - **Public (must follow this exact order):**
//!   1. `hash_audio`
//!   2. `hash_title`
//!   3. `hash_creators`
//!   4. `commitment`
//!   5. `timestamp`
//!   6. `nullifier`
//!
//! Input order is critical and must be respected during verification.
//!
//! # Example (see tests)
//!
//! The tests at the bottom of this file demonstrate a full flow with Groth16:
//! setup, proof generation, and proof verification using consistent public inputs.

use ark_bn254::Fr;
use ark_crypto_primitives::sponge::{
    constraints::CryptographicSpongeVar,
    poseidon::{PoseidonConfig, constraints::PoseidonSpongeVar},
};
use ark_ff::One;
use ark_r1cs_std::{R1CSVar, alloc::AllocVar, eq::EqGadget, fields::fp::FpVar};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::vec::Vec;

// -------------------- Poseidon config ----------------------------------------

/// Returns a **placeholder** Poseidon configuration for BN254.
///
/// This configuration is only meant for compilation and demo purposes.
/// It uses trivial matrices filled with `Fr::one()` and is **not secure**!
///
/// # ⚠️ Security
///
/// Replace with real Poseidon parameters (MDS, round constants, round numbers)
/// before using in production.
pub fn poseidon_params() -> PoseidonConfig<Fr> {
    // For testing/demo: width=3, rate=2, capacity=1
    // Replace with real Poseidon params for BN254 (MDS, ark, etc.)
    let full_rounds = 8;
    let partial_rounds = 57;
    let alpha = 5;
    let rate = 2;
    let capacity = 1;

    // Dummy small matrices just for compiling (NOT secure!)
    let mds = vec![vec![Fr::one(); rate + capacity]; rate + capacity];
    let ark = vec![vec![Fr::one(); rate + capacity]; full_rounds + partial_rounds];

    PoseidonConfig::new(full_rounds, partial_rounds, alpha, mds, ark, rate, capacity)
}

// -------------------- Circuit ------------------------------------------------

/// R1CS circuit for verifying a Poseidon-based commitment and nullifier.
///
/// Enforces:
/// - `commitment = Poseidon(hash_audio, hash_title, hash_creators, secret)`
/// - `nullifier  = Poseidon(commitment, timestamp)`
///
/// All values are BN254 field elements (`Fr`).
#[derive(Clone)]
pub struct Circuit {
    // Witness
    pub secret: Fr,
    // Publics
    pub hash_audio: Fr,
    pub hash_title: Fr,
    pub hash_creators: Fr,
    pub commitment: Fr,
    pub timestamp: Fr,
    pub nullifier: Fr,
}

impl Circuit {
    /// Poseidon hash gadget with 4 field inputs.
    ///
    /// Builds a Poseidon sponge inside the circuit, absorbs `[a, b, c, d]`,
    /// and squeezes one field element as output.
    fn h4_var(
        a: &FpVar<Fr>,
        b: &FpVar<Fr>,
        c: &FpVar<Fr>,
        d: &FpVar<Fr>,
        cfg: &PoseidonConfig<Fr>,
    ) -> Result<FpVar<Fr>, SynthesisError> {
        let mut sp = PoseidonSpongeVar::<Fr>::new(a.cs(), cfg);
        let inputs: Vec<FpVar<Fr>> = vec![a.clone(), b.clone(), c.clone(), d.clone()];
        sp.absorb(&inputs)?; // <- pass a slice
        let out = sp.squeeze_field_elements(1)?;
        Ok(out[0].clone())
    }

    /// Poseidon hash gadget with 2 field inputs.
    ///
    /// Same as [`h4_var`] but absorbs only `[x, y]`.
    fn h2_var(
        x: &FpVar<Fr>,
        y: &FpVar<Fr>,
        cfg: &PoseidonConfig<Fr>,
    ) -> Result<FpVar<Fr>, SynthesisError> {
        let mut sp = PoseidonSpongeVar::<Fr>::new(x.cs(), cfg);
        let inputs: Vec<FpVar<Fr>> = vec![x.clone(), y.clone()];
        sp.absorb(&inputs)?; // <- pass a slice
        let out = sp.squeeze_field_elements(1)?;
        Ok(out[0].clone())
    }
}

impl ConstraintSynthesizer<Fr> for Circuit {
    /// Builds the R1CS constraints for the commitment and nullifier equations.
    ///
    /// Steps:
    /// 1. Allocate `secret` as witness.
    /// 2. Allocate all public inputs in the required order.
    /// 3. Compute `commitment_var = Poseidon(audio, title, creators, secret)`.
    /// 4. Enforce `commitment_var == commitment`.
    /// 5. Compute `nullifier_var = Poseidon(commitment, timestamp)`.
    /// 6. Enforce `nullifier_var == nullifier`.
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let params = poseidon_params();

        // Witness
        let w_secret = FpVar::<Fr>::new_witness(cs.clone(), || Ok(self.secret))?;

        // Public inputs
        let p_hash_audio = FpVar::<Fr>::new_input(cs.clone(), || Ok(self.hash_audio))?;
        let p_hash_title = FpVar::<Fr>::new_input(cs.clone(), || Ok(self.hash_title))?;
        let p_hash_creators = FpVar::<Fr>::new_input(cs.clone(), || Ok(self.hash_creators))?;
        let p_commitment = FpVar::<Fr>::new_input(cs.clone(), || Ok(self.commitment))?;
        let p_timestamp = FpVar::<Fr>::new_input(cs.clone(), || Ok(self.timestamp))?;
        let p_nullifier = FpVar::<Fr>::new_input(cs.clone(), || Ok(self.nullifier))?;

        // 1) commitment = Poseidon(audio, title, creators, secret)
        let commitment_var = Self::h4_var(
            &p_hash_audio,
            &p_hash_title,
            &p_hash_creators,
            &w_secret,
            &params,
        )?;
        commitment_var.enforce_equal(&p_commitment)?;

        // 2) nullifier = Poseidon(commitment, timestamp)
        let nullifier_var = Self::h2_var(&commitment_var, &p_timestamp, &params)?;
        nullifier_var.enforce_equal(&p_nullifier)?;

        Ok(())
    }
}

// -------------------- Tests --------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::utils::{
        fr_from_hex_be, fr_u64, poseidon_commitment_offchain, poseidon_nullifier_offchain,
    };

    use super::*;
    use ark_bn254::Bn254;
    use ark_groth16::{Groth16, prepare_verifying_key};
    use rand::thread_rng;

    #[test]
    fn prove_and_verify_ok() {
        let cfg = poseidon_params();

        // 1) Example inputs
        let secret =
            "0x23864adb160dddf590f1d3303683ebcb914f828e2635f6e85a32f0a1aecd3dd8".to_string();
        let hash_audio =
            "0x26d273f7c73a635f6eaeb904e116ec4cd887fb5a87fc7427c95279e6053e5bf0".to_string();
        let hash_title =
            "0x175eeef716d52cf8ee972c6fefd60e47df5084efde3c188c40a81a42e72dfb04".to_string();
        let hash_creators =
            "0x017ac5e7a52bec07ca8ee344a9979aa083b7713f1196af35310de21746985079".to_string();
        let timestamp = 10000;

        // 2) Publics (off-chain Poseidon)
        let commitment =
            poseidon_commitment_offchain(&hash_audio, &hash_title, &hash_creators, &secret, &cfg);
        let nullifier = poseidon_nullifier_offchain(&commitment, timestamp, &cfg);

        // 3) Setup
        let mut rng = thread_rng();
        let params = Groth16::<Bn254>::generate_random_parameters_with_reduction(
            Circuit {
                secret: fr_from_hex_be(&secret),
                hash_audio: fr_from_hex_be(&hash_audio),
                hash_title: fr_from_hex_be(&hash_title),
                hash_creators: fr_from_hex_be(&hash_creators),
                commitment: fr_from_hex_be(&commitment),
                timestamp: fr_u64(timestamp),
                nullifier: fr_from_hex_be(&nullifier),
            },
            &mut rng,
        )
        .unwrap();

        // 4) Proof
        let proof = Groth16::<Bn254>::create_random_proof_with_reduction(
            Circuit {
                secret: fr_from_hex_be(&secret),
                hash_audio: fr_from_hex_be(&hash_audio),
                hash_title: fr_from_hex_be(&hash_title),
                hash_creators: fr_from_hex_be(&hash_creators),
                commitment: fr_from_hex_be(&commitment),
                timestamp: fr_u64(timestamp),
                nullifier: fr_from_hex_be(&nullifier),
            },
            &params,
            &mut rng,
        )
        .unwrap();

        // 5) Verify
        let pvk = prepare_verifying_key(&params.vk);
        let public_inputs = [
            fr_from_hex_be(&hash_audio),
            fr_from_hex_be(&hash_title),
            fr_from_hex_be(&hash_creators),
            fr_from_hex_be(&commitment),
            fr_u64(timestamp),
            fr_from_hex_be(&nullifier),
        ];
        let ok = Groth16::<Bn254>::verify_proof(&pvk, &proof, &public_inputs).unwrap();
        assert!(ok, "verification should succeed");
    }

    #[test]
    fn verify_fails_with_wrong_publics() {
        let cfg = poseidon_params();

        // Inputs
        let secret =
            "0x23864adb160dddf590f1d3303683ebcb914f828e2635f6e85a32f0a1aecd3dd8".to_string();
        let hash_audio =
            "0x26d273f7c73a635f6eaeb904e116ec4cd887fb5a87fc7427c95279e6053e5bf0".to_string();
        let hash_title =
            "0x175eeef716d52cf8ee972c6fefd60e47df5084efde3c188c40a81a42e72dfb04".to_string();
        let hash_creators =
            "0x017ac5e7a52bec07ca8ee344a9979aa083b7713f1196af35310de21746985079".to_string();
        let timestamp = 10000;

        let commitment =
            poseidon_commitment_offchain(&hash_audio, &hash_title, &hash_creators, &secret, &cfg);
        let nullifier = poseidon_nullifier_offchain(&commitment, timestamp, &cfg);

        let mut rng = thread_rng();
        let params = Groth16::<Bn254>::generate_random_parameters_with_reduction(
            Circuit {
                secret: fr_from_hex_be(&secret),
                hash_audio: fr_from_hex_be(&hash_audio),
                hash_title: fr_from_hex_be(&hash_title),
                hash_creators: fr_from_hex_be(&hash_creators),
                commitment: fr_from_hex_be(&commitment),
                timestamp: fr_u64(timestamp),
                nullifier: fr_from_hex_be(&nullifier),
            },
            &mut rng,
        )
        .unwrap();

        let proof = Groth16::<Bn254>::create_random_proof_with_reduction(
            Circuit {
                secret: fr_from_hex_be(&secret),
                hash_audio: fr_from_hex_be(&hash_audio),
                hash_title: fr_from_hex_be(&hash_title),
                hash_creators: fr_from_hex_be(&hash_creators),
                commitment: fr_from_hex_be(&commitment),
                timestamp: fr_u64(timestamp),
                nullifier: fr_from_hex_be(&nullifier),
            },
            &params,
            &mut rng,
        )
        .unwrap();

        // Wrong publics (timestamp + 1)
        let pvk = prepare_verifying_key(&params.vk);
        let wrong_public_inputs = [
            fr_from_hex_be(&hash_audio),
            fr_from_hex_be(&hash_title),
            fr_from_hex_be(&hash_creators),
            fr_from_hex_be(&commitment),
            fr_u64(10001),
            fr_from_hex_be(&nullifier),
        ];
        let ok = Groth16::<Bn254>::verify_proof(&pvk, &proof, &wrong_public_inputs).unwrap();
        assert!(!ok, "verification should fail with wrong publics");
    }
}
