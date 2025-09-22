use crate::circuit::Circuit;
use crate::Curve;
use ark_bn254::Fr;
use ark_groth16::{Groth16, PreparedVerifyingKey, Proof, ProvingKey, VerifyingKey};
use ark_relations::r1cs::SynthesisError;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};

fn strip_0x(s: &str) -> &str {
    s.strip_prefix("0x").unwrap_or(s)
}

fn hex_to_bytes(s: &str) -> Result<Vec<u8>, SerializationError> {
    hex::decode(strip_0x(s)).map_err(|_| SerializationError::InvalidData)
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(2 + bytes.len() * 2);
    out.push_str("0x");
    out.push_str(&hex::encode(bytes));
    out
}

/// Public inputs (in the exact order expected by the circuit)
#[derive(Clone, Copy, Debug)]
pub struct PublicInputs {
    pub hash_audio: Fr,
    pub hash_title: Fr,
    pub hash_creators: Fr,
    pub commitment: Fr,
    pub timestamp: Fr,
    pub nullifier: Fr,
}

impl PublicInputs {
    pub fn as_slice(&self) -> [Fr; 6] {
        [
            self.hash_audio,
            self.hash_title,
            self.hash_creators,
            self.commitment,
            self.timestamp,
            self.nullifier,
        ]
    }
}

/// Witness inputs (kept private)
#[derive(Clone, Copy, Debug)]
pub struct Witness {
    pub secret: Fr,
}

/// Generate Groth16 proving and verifying keys (setup).
pub fn setup<R: rand::RngCore + rand::CryptoRng>(
    rng: &mut R,
    example_inputs: (Witness, PublicInputs),
) -> Result<(ProvingKey<Curve>, VerifyingKey<Curve>), SynthesisError> {
    let (w, p) = example_inputs;
    let circuit = Circuit {
        secret: w.secret,
        hash_audio: p.hash_audio,
        hash_title: p.hash_title,
        hash_creators: p.hash_creators,
        commitment: p.commitment,
        timestamp: p.timestamp,
        nullifier: p.nullifier,
    };

    // Returns a ProvingKey<Curve>
    let pk = Groth16::<Curve>::generate_random_parameters_with_reduction(circuit, rng)?;
    let vk = pk.vk.clone(); // extract VK from PK

    Ok((pk, vk))
}

/// Generate a proof given a proving key, witness and public inputs.
pub fn prove<R: rand::RngCore + rand::CryptoRng>(
    pk: &ProvingKey<Curve>,
    witness: Witness,
    publics: PublicInputs,
    rng: &mut R,
) -> Result<(Proof<Curve>, [Fr; 6]), SynthesisError> {
    let circuit = Circuit {
        secret: witness.secret,
        hash_audio: publics.hash_audio,
        hash_title: publics.hash_title,
        hash_creators: publics.hash_creators,
        commitment: publics.commitment,
        timestamp: publics.timestamp,
        nullifier: publics.nullifier,
    };
    let proof = Groth16::<Curve>::create_random_proof_with_reduction(circuit, pk, rng)?;
    Ok((proof, publics.as_slice()))
}

/// Prepare the verifying key for efficient verification.
pub fn prepare_vk(vk: &VerifyingKey<Curve>) -> PreparedVerifyingKey<Curve> {
    ark_groth16::prepare_verifying_key(vk)
}

/// Verify a proof against public inputs.
pub fn verify(
    pvk: &PreparedVerifyingKey<Curve>,
    proof: &Proof<Curve>,
    public_inputs: &[Fr],
) -> Result<bool, SynthesisError> {
    Groth16::<Curve>::verify_proof(pvk, proof, public_inputs)
}

/// Deserialize verifying key and proof from bytes and run verification.
/// Returns `SerializationError` if decoding fails, or `SynthesisError` if verification fails.
pub fn verify_from_bytes(
    vk_bytes: &[u8],
    proof_bytes: &[u8],
    public_inputs: &[Fr],
) -> Result<bool, SerializationError> {
    // 1) Deserialize
    let vk = VerifyingKey::<Curve>::deserialize_compressed(vk_bytes)
        .map_err(|_| SerializationError::InvalidData)?;
    let pvk = prepare_vk(&vk);

    let proof = Proof::<Curve>::deserialize_compressed(proof_bytes)
        .map_err(|_| SerializationError::InvalidData)?;

    // 2) Verify (map SynthesisError to SerializationError if you want one Result type)
    // Here, we preserve the distinct error kinds by returning Ok/Err separately:
    match Groth16::<Curve>::verify_proof(&pvk, &proof, public_inputs) {
        Ok(true) => Ok(true),
        Ok(false) => Err(SerializationError::InvalidData),
        Err(_) => Err(SerializationError::InvalidData),
    }
}

/// Verify using hex strings (friendly for Substrate + SDKs).
/// - `vk_hex`: hex of compressed `VerifyingKey`
/// - `proof_hex`: hex of compressed `Proof`
/// - `public_inputs_hex`: hex field elements (big-endian), order must match the circuit
///
/// Returns `SerializationError::InvalidData` if deserialization fails **or** if
/// the proof fails to verify (keeps a single simple error type for callers).
pub fn verify_from_hex(
    vk_hex: &str,
    proof_hex: &str,
    public_inputs_hex: &[&str],
) -> Result<bool, SerializationError> {
    use ark_serialize::SerializationError;

    let vk_bytes = hex_to_bytes(vk_hex)?; // already returns InvalidData on bad hex
    let proof_bytes = hex_to_bytes(proof_hex)?;

    // Normalize deserialization errors to InvalidData
    let vk = VerifyingKey::<Curve>::deserialize_compressed(&vk_bytes[..])
        .map_err(|_| SerializationError::InvalidData)?;
    let pvk = prepare_vk(&vk);
    let proof = Proof::<Curve>::deserialize_compressed(&proof_bytes[..])
        .map_err(|_| SerializationError::InvalidData)?;

    // Decode publics from hex (big-endian) using your helper
    let mut publics: Vec<Fr> = Vec::with_capacity(public_inputs_hex.len());
    for h in public_inputs_hex {
        publics.push(crate::utils::fr_from_hex_be(h));
    }

    // Map both Ok(false) and Err(_) to InvalidData
    match Groth16::<Curve>::verify_proof(&pvk, &proof, &publics) {
        Ok(true) => Ok(true),
        Ok(false) => Err(SerializationError::InvalidData),
        Err(_) => Err(SerializationError::InvalidData),
    }
}

// ---------- public: serialization to HEX ----------

/// Serialize a verifying key to compressed bytes and return a `0x`-prefixed hex string.
pub fn serialize_vk_to_hex(vk: &VerifyingKey<Curve>) -> String {
    let mut buf = Vec::new();
    vk.serialize_compressed(&mut buf).expect("serialize vk");
    bytes_to_hex(&buf)
}

/// Serialize a proof to compressed bytes and return a `0x`-prefixed hex string.
pub fn serialize_proof_to_hex(proof: &Proof<Curve>) -> String {
    let mut buf = Vec::new();
    proof
        .serialize_compressed(&mut buf)
        .expect("serialize proof");
    bytes_to_hex(&buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::BigInteger;
    use ark_serialize::CanonicalSerialize;
    use rand::thread_rng;

    // If these helpers live in another module, adjust imports accordingly:
    use crate::utils::{fr_from_hex_be, fr_u64, poseidon_h2_offchain, poseidon_h4_offchain};
    // If your helpers expect a config, expose or re-export your params function.
    // Here we assume you re-exported it as `crate::poseidon_params`.
    use crate::circuit::poseidon_params;

    fn example_io() -> (Witness, PublicInputs) {
        let cfg = poseidon_params();

        // Example values (same as your earlier unit tests)
        let secret =
            fr_from_hex_be("0x23864adb160dddf590f1d3303683ebcb914f828e2635f6e85a32f0a1aecd3dd8");
        let hash_audio =
            fr_from_hex_be("0x26d273f7c73a635f6eaeb904e116ec4cd887fb5a87fc7427c95279e6053e5bf0");
        let hash_title =
            fr_from_hex_be("0x175eeef716d52cf8ee972c6fefd60e47df5084efde3c188c40a81a42e72dfb04");
        let hash_creators =
            fr_from_hex_be("0x017ac5e7a52bec07ca8ee344a9979aa083b7713f1196af35310de21746985079");
        let timestamp = fr_u64(10_000);

        // Compute publics off-chain with the same Poseidon config
        let commitment = poseidon_h4_offchain(hash_audio, hash_title, hash_creators, secret, &cfg);
        let nullifier = poseidon_h2_offchain(commitment, timestamp, &cfg);

        (
            Witness { secret },
            PublicInputs {
                hash_audio,
                hash_title,
                hash_creators,
                commitment,
                timestamp,
                nullifier,
            },
        )
    }

    #[test]
    fn setup_prove_verify_roundtrip() {
        let (w, p) = example_io();
        let mut rng = thread_rng();

        // 1) Setup
        let (pk, vk) = setup(&mut rng, (w, p)).expect("setup");

        // 2) Prove
        let (proof, publics) = prove(&pk, w, p, &mut rng).expect("prove");

        // 3) Verify using prepared VK
        let pvk = prepare_vk(&vk);
        let ok = verify(&pvk, &proof, &publics).expect("verify call");
        assert!(ok, "verification should succeed");
    }

    #[test]
    fn verify_from_bytes_roundtrip() {
        let (w, p) = example_io();
        let mut rng = thread_rng();

        // Setup + Prove
        let (pk, vk) = setup(&mut rng, (w, p)).expect("setup");
        let (proof, publics) = prove(&pk, w, p, &mut rng).expect("prove");

        // Serialize vk + proof to compressed bytes
        let mut vk_bytes = Vec::new();
        vk.serialize_compressed(&mut vk_bytes).expect("ser vk");

        let mut proof_bytes = Vec::new();
        proof
            .serialize_compressed(&mut proof_bytes)
            .expect("ser proof");

        // Verify via the convenience API
        let ok = verify_from_bytes(&vk_bytes, &proof_bytes, &publics)
            .expect("verify_from_bytes should deserialize");
        assert!(ok, "verification should succeed");
    }

    #[test]
    fn verify_fails_with_tampered_publics() {
        let (w, mut p) = example_io();
        let mut rng = thread_rng();

        // Setup + Prove with correct publics
        let (pk, vk) = setup(&mut rng, (w, p)).expect("setup");
        let (proof, _) = prove(&pk, w, p, &mut rng).expect("prove");

        // Tamper with the timestamp (public input mismatch)
        p.timestamp = fr_u64(10_001);
        let publics = p.as_slice();

        let pvk = prepare_vk(&vk);
        let ok = verify(&pvk, &proof, &publics).expect("verify call");
        assert!(
            !ok,
            "verification should fail when public inputs are inconsistent"
        );

        // Byte-based verifier now returns Err(InvalidData) on any failure
        let mut vk_bytes = Vec::new();
        vk.serialize_compressed(&mut vk_bytes).expect("ser vk");
        let mut proof_bytes = Vec::new();
        proof
            .serialize_compressed(&mut proof_bytes)
            .expect("ser proof");

        let err = verify_from_bytes(&vk_bytes, &proof_bytes, &publics).unwrap_err();
        use SerializationError;
        assert!(matches!(err, SerializationError::InvalidData));
    }

    // Convert Fr -> 0x-prefixed big-endian hex (mirrors fr_from_hex_be)
    fn fr_to_hex_be(x: &Fr) -> String {
        use ark_ff::PrimeField;
        let be = x.into_bigint().to_bytes_be();
        let mut s = String::from("0x");
        s.push_str(&hex::encode(be));
        s
    }

    // ---------- helper/utility coverage ----------

    #[test]
    fn hex_utils_roundtrip() {
        // bytes_to_hex -> hex_to_bytes roundtrip
        let data = vec![0u8, 1, 2, 0xaa, 0xff, 0x10, 0x00];
        let hx = super::bytes_to_hex(&data);
        assert!(hx.starts_with("0x"));
        let back = super::hex_to_bytes(&hx).expect("decode ok");
        assert_eq!(back, data);

        // strip_0x correctness (both with and without prefix)
        let no0x = "deadbeef";
        let with0x = "0xdeadbeef";
        assert_eq!(super::strip_0x(no0x), "deadbeef");
        assert_eq!(super::strip_0x(with0x), "deadbeef");
    }

    // ---------- serialize_*_to_hex + verify_from_hex success ----------

    #[test]
    fn verify_from_hex_roundtrip_success() {
        let (w, p) = example_io();
        let mut rng = thread_rng();

        // Setup + Prove
        let (pk, vk) = setup(&mut rng, (w, p)).expect("setup");
        let (proof, publics) = prove(&pk, w, p, &mut rng).expect("prove");

        // Serialize to hex using the API under test
        let vk_hex = serialize_vk_to_hex(&vk);
        let proof_hex = serialize_proof_to_hex(&proof);

        // Public inputs as hex (big-endian), in the exact expected order
        let publics_hex: Vec<String> = publics.iter().map(fr_to_hex_be).collect();
        let publics_hex_refs: Vec<&str> = publics_hex.iter().map(|s| s.as_str()).collect();

        // Verify via hex API
        let ok = verify_from_hex(&vk_hex, &proof_hex, &publics_hex_refs)
            .expect("verify_from_hex should deserialize");
        assert!(ok, "verification should succeed");
    }

    // ---------- verify_from_hex failure: malformed hex ----------

    #[test]
    fn verify_from_hex_rejects_malformed_hex() {
        // Intentionally malformed hex blobs
        let bad_vk = "0xzz11"; // invalid chars
        let bad_proof = "dead"; // missing 0x is fine, but odd length + nonsense data leads to invalid deserialization

        // Any public inputs; they won't be reached if vk/proof decode fails
        let publics_hex = &["0x01"; 6];

        // Either vk or proof hex decoding / deserialization should fail -> Err(InvalidData)
        let err1 = verify_from_hex(bad_vk, "0x00", publics_hex).unwrap_err();
        let err2 = verify_from_hex("0x00", bad_proof, publics_hex).unwrap_err();

        use SerializationError;
        assert!(matches!(err1, SerializationError::InvalidData));
        assert!(matches!(err2, SerializationError::InvalidData));
    }

    // ---------- verify_from_hex failure: tampered publics ----------

    #[test]
    fn verify_from_hex_fails_with_tampered_publics() {
        let (w, p0) = example_io();
        let mut rng = thread_rng();

        // Setup + Prove with correct publics
        let (pk, vk) = setup(&mut rng, (w, p0)).expect("setup");
        let (proof, _publics) = prove(&pk, w, p0, &mut rng).expect("prove");

        // Serialize vk + proof to hex
        let vk_hex = serialize_vk_to_hex(&vk);
        let proof_hex = serialize_proof_to_hex(&proof);

        // Tamper one public (timestamp + 1)
        let mut p_bad = p0;
        p_bad.timestamp = fr_u64(10_001);
        let publics_hex_bad: Vec<String> = p_bad.as_slice().iter().map(fr_to_hex_be).collect();
        let publics_hex_bad_refs: Vec<&str> = publics_hex_bad.iter().map(|s| s.as_str()).collect();

        // Our verify_from_hex maps verification failure to SerializationError::InvalidData
        let err = verify_from_hex(&vk_hex, &proof_hex, &publics_hex_bad_refs).unwrap_err();
        use SerializationError;
        assert!(matches!(err, SerializationError::InvalidData));
    }

    // ---------- prepare_vk sanity ----------

    #[test]
    fn prepare_vk_is_consistent() {
        let (w, p) = example_io();
        let mut rng = thread_rng();
        let (_pk, vk) = setup(&mut rng, (w, p)).expect("setup");
        let pvk = prepare_vk(&vk);

        // Quick smoke: pvk contains the same raw vk (via From)
        let roundtrip_vk: VerifyingKey<Curve> = pvk.clone().into();
        assert_eq!(roundtrip_vk, vk);
    }
}
