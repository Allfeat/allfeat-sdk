use allfeat_ats_zkp::{
    Creator, Roles, fr_to_hex_be, fr_u64, hash_audio, hash_creators, hash_title,
    poseidon_commitment_offchain, poseidon_nullifier_offchain, poseidon_params,
};
use ark_bn254::Fr;
use ark_ff::UniformRand;
use ark_serialize::SerializationError;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsCreator {
    #[serde(rename = "fullName")]
    pub full_name: String,
    pub email: String,
    /// Accepts ["AT","CP","AR","AD"] or ["Author","Composer","Arranger","Adapter"] (case-insensitive)
    pub roles: Vec<String>,
    pub ipi: Option<String>,
    pub isni: Option<String>,
}

fn roles_from_codes<'a, I: IntoIterator<Item = &'a str>>(codes: I) -> Roles {
    let mut r = Roles::default();
    for c in codes {
        match c.to_ascii_uppercase().as_str() {
            "AT" | "AUTHOR" => r.author = true,
            "CP" | "COMPOSER" => r.composer = true,
            "AR" | "ARRANGER" => r.arranger = true,
            "AD" | "ADAPTER" => r.adapter = true,
            _ => {}
        }
    }
    r
}

fn js_creators_to_core(creators_js: JsValue) -> Result<Vec<Creator>, JsValue> {
    let creators_in: Vec<JsCreator> = serde_wasm_bindgen::from_value(creators_js)
        .map_err(|e| JsValue::from_str(&format!("Invalid creators JSON: {e}")))?;
    Ok(creators_in
        .into_iter()
        .map(|j| Creator {
            full_name: j.full_name,
            email: j.email,
            roles: roles_from_codes(j.roles.iter().map(|s| s.as_str())),
            ipi: j.ipi,
            isni: j.isni,
        })
        .collect())
}

// -------------------- Data Structures: Hex & Fr ------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkpBundleHex {
    pub hash_title: String,
    pub hash_audio: String,
    pub hash_creators: String,
    pub secret: String,
    pub commitment: String,
    pub timestamp: String,
    pub nullifier: String,
}

// -------------------- Off-chain Poseidon (hex in/out) ------------------------

fn compute_commitment_nullifier(
    hash_title: &str,
    hash_audio: &str,
    hash_creators: &str,
    secret: &str,
    timestamp: &str,
) -> Result<(String, String), SerializationError> {
    let cfg = poseidon_params();
    let commitment =
        poseidon_commitment_offchain(hash_title, hash_audio, hash_creators, secret, &cfg)?;
    let nullifier = poseidon_nullifier_offchain(&commitment, timestamp, &cfg)?;
    Ok((commitment, nullifier))
}

// -------------------- Exposed WASM functions ---------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildBundleOutput {
    pub bundle: ZkpBundleHex,
}

/// Build a full precomputed bundle (random secret):
/// - inputs: `title`, `audio_bytes` (Uint8Array), `creators` (array of JsCreator), `timestamp` (seconds)
/// - returns: all hashes + commitment + nullifier as hex, plus the numeric timestamp
#[wasm_bindgen]
pub fn build_bundle(
    title: &str,
    audio_bytes: &[u8],
    creators_js: JsValue,
    timestamp: u64,
) -> Result<JsValue, JsValue> {
    // 1) random secret (Fr -> hex)
    let mut rng = OsRng;
    let secret_fr = Fr::rand(&mut rng);
    let secret = fr_to_hex_be(&secret_fr);

    // 2) hashes (your current helpers return HEX `String`)
    let hash_title = hash_title(title);
    let hash_audio = hash_audio(audio_bytes);
    let creators_core = js_creators_to_core(creators_js)?;
    let hash_creators = hash_creators(&creators_core);
    let timestamp_hex = fr_to_hex_be(&fr_u64(timestamp));

    // 3) commitment + nullifier (hex)
    let (commitment, nullifier) = compute_commitment_nullifier(
        &hash_title,
        &hash_audio,
        &hash_creators,
        &secret,
        &timestamp_hex,
    )
    .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // 4) build outputs (all hex)
    let out = BuildBundleOutput {
        bundle: ZkpBundleHex {
            hash_title,
            hash_audio,
            hash_creators,
            commitment,
            timestamp: timestamp_hex,
            secret,
            nullifier,
        },
    };

    serde_wasm_bindgen::to_value(&out).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProveOutput {
    pub proof: String,
    /// Publics in circuit order (hex):
    /// [hash_title, hash_audio, hash_creators, commitment, timestamp, nullifier]
    pub publics: [String; 6],
}

/// Groth16 proof (hex-only API passthrough):
/// - `pk`: compressed PK (0x-hex)
/// - `secret`: 0x-hex Fr
/// - `publics`: array(6) of 0x-hex Fr in circuit order
#[wasm_bindgen]
pub fn prove(pk: &str, secret: &str, publics: JsValue) -> Result<JsValue, JsValue> {
    let publics: Vec<String> = serde_wasm_bindgen::from_value(publics)
        .map_err(|e| JsValue::from_str(&format!("publics must be 6 hex strings: {e}")))?;
    if publics.len() != 6 {
        return Err(JsValue::from_str("publics must have length 6"));
    }
    let publics_refs: Vec<&str> = publics.iter().map(|s| s.as_str()).collect();

    // Call your zkp.rs hex-only prove (it already manages RNG internally)
    let (proof, publics_out) = allfeat_ats_zkp::zkp::prove(pk, secret, &publics_refs)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    serde_wasm_bindgen::to_value(&ProveOutput {
        proof,
        publics: publics_out,
    })
    .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Groth16 verify (hex-only API passthrough):
/// - `vk`: compressed VK (0x-hex)
/// - `proof`: 0x-hex compressed proof
/// - `publics`: array(6) of 0x-hex Fr in circuit order
#[wasm_bindgen]
pub fn verify(vk: &str, proof: &str, publics: JsValue) -> Result<bool, JsValue> {
    // 1) Parse publics des de JS
    let publics: Vec<String> = serde_wasm_bindgen::from_value(publics)
        .map_err(|e| JsValue::from_str(&format!("publics must be 6 hex strings: {e}")))?;
    if publics.len() != 6 {
        return Err(JsValue::from_str("publics must have length 6"));
    }
    let publics_refs: Vec<&str> = publics.iter().map(|s| s.as_str()).collect();

    // 2) Crida el core verify i propaga l’error cap a JS
    let ok = allfeat_ats_zkp::zkp::verify(vk, proof, &publics_refs)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // 3) Retorna el booleà (es marshalleja a JS com `true/false`)
    Ok(ok)
}

#[cfg(test)]
mod tests_host {
    use allfeat_ats_zkp::{fr_to_hex_be, fr_u64};
    use ark_serialize::SerializationError;

    #[test]
    fn roles_from_codes_variants() {
        let r = super::roles_from_codes(["AT", "cp", "Arranger", "adapter"].iter().copied());
        assert!(r.author);
        assert!(r.composer);
        assert!(r.arranger);
        assert!(r.adapter);

        let r2 = super::roles_from_codes(["unknown", "ZZ"].iter().copied());
        assert!(!r2.author && !r2.composer && !r2.arranger && !r2.adapter);
    }

    #[test]
    fn compute_commitment_nullifier_is_deterministic() -> Result<(), SerializationError> {
        let secret = "0x01";
        let ha = "0x02";
        let ht = "0x03";
        let hc = "0x04";
        let ts = fr_to_hex_be(&fr_u64(42u64));

        let (c1, n1) = super::compute_commitment_nullifier(ha, ht, hc, secret, &ts)?;
        let (c2, n2) = super::compute_commitment_nullifier(ha, ht, hc, secret, &ts)?;
        assert_eq!(c1, c2);
        assert_eq!(n1, n2);
        Ok(())
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests_wasm {
    use super::*;
    use serde_wasm_bindgen as swb;
    use wasm_bindgen_test::*;

    // If needed by your core crate (adjust path if different)
    use allfeat_ats_zkp::zkp::{setup as zkp_setup, verify as zkp_verify};

    // wasm_bindgen_test_configure!(run_in_browser); // or omit to run under node

    fn is_fr_hex(s: &str) -> bool {
        // 0x + 64 hexdigits (32 bytes), typical for Fr
        s.len() == 66 && s.starts_with("0x") && s.chars().skip(2).all(|c| c.is_ascii_hexdigit())
    }

    fn is_hex_prefixed(s: &str) -> bool {
        // generic hex: 0x prefix, even number of hex digits
        s.starts_with("0x")
            && ((s.len() - 2) % 2 == 0)
            && s.chars().skip(2).all(|c| c.is_ascii_hexdigit())
    }

    #[wasm_bindgen_test]
    fn js_creators_to_core_maps_fields() {
        let js_creators = vec![
            JsCreator {
                full_name: "Alice".into(),
                email: "alice@example.com".into(),
                roles: vec!["AT".into(), "Composer".into()],
                ipi: Some("123".into()),
                isni: None,
            },
            JsCreator {
                full_name: "Bob".into(),
                email: "bob@example.com".into(),
                roles: vec!["AR".into()],
                ipi: None,
                isni: Some("0000 0001 2281 955X".into()),
            },
        ];
        let js_val = swb::to_value(&js_creators).unwrap();
        let core = js_creators_to_core(js_val).expect("map creators");

        assert_eq!(core.len(), 2);
        assert_eq!(core[0].full_name, "Alice");
        assert!(core[0].roles.author && core[0].roles.composer);
        assert!(core[1].roles.arranger);
    }

    #[wasm_bindgen_test]
    fn build_bundle_is_consistent_and_hex_formatted() -> Result<(), JsValue> {
        let title = "Song Title";
        let audio: Vec<u8> = b"dummy-audio".to_vec();
        let creators = vec![JsCreator {
            full_name: "Alice".into(),
            email: "alice@example.com".into(),
            roles: vec!["AT".into()],
            ipi: None,
            isni: None,
        }];
        let creators_js = swb::to_value(&creators)?;
        let timestamp = 10_000u64;

        let js_out = build_bundle(title, &audio, creators_js, timestamp)?;
        let out: BuildBundleOutput = swb::from_value(js_out)?;

        assert!(is_fr_hex(&out.bundle.secret));
        assert!(is_fr_hex(&out.bundle.hash_title));
        assert!(is_fr_hex(&out.bundle.hash_audio));
        assert!(is_fr_hex(&out.bundle.hash_creators));
        assert!(is_hex_prefixed(&out.bundle.commitment));
        assert!(is_hex_prefixed(&out.bundle.nullifier));
        assert_eq!(out.bundle.timestamp, fr_to_hex_be(&fr_u64(timestamp)));

        // recompute and compare
        let (c2, n2) = super::compute_commitment_nullifier(
            &out.bundle.hash_title,
            &out.bundle.hash_audio,
            &out.bundle.hash_creators,
            &out.bundle.secret,
            &out.bundle.timestamp,
        )
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
        assert_eq!(c2, out.bundle.commitment);
        assert_eq!(n2, out.bundle.nullifier);

        Ok(())
    }

    #[wasm_bindgen_test]
    fn prove_roundtrip_and_verify() -> Result<(), JsValue> {
        // (publics order): [hash_title, hash_audio, hash_creators, commitment, timestamp, nullifier]
        let secret = "0x23864adb160dddf590f1d3303683ebcb914f828e2635f6e85a32f0a1aecd3dd8";
        let hash_title = "0x175eeef716d52cf8ee972c6fefd60e47df5084efde3c188c40a81a42e72dfb04";
        let hash_audio = "0x26d273f7c73a635f6eaeb904e116ec4cd887fb5a87fc7427c95279e6053e5bf0";
        let hash_creators = "0x017ac5e7a52bec07ca8ee344a9979aa083b7713f1196af35310de21746985079";
        let timestamp = fr_to_hex_be(&fr_u64(10_000u64));

        let (commitment, nullifier) = super::compute_commitment_nullifier(
            hash_title,
            hash_audio,
            hash_creators,
            secret,
            &timestamp,
        )
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let publics_vec = vec![
            hash_title.to_string(),
            hash_audio.to_string(),
            hash_creators.to_string(),
            commitment.clone(),
            timestamp.clone(),
            nullifier.clone(),
        ];
        let publics_refs: Vec<&str> = publics_vec.iter().map(|s| s.as_str()).collect();

        // 1) Setup (PK/VK as hex)
        let (pk_hex, vk_hex) = zkp_setup(secret, &publics_refs).expect("setup");

        // 2) Prove via WASM wrapper
        let publics_js = swb::to_value(&publics_vec).unwrap();
        let prove_js = super::prove(&pk_hex, secret, publics_js).expect("prove wrapper");
        let prove_out: super::ProveOutput = swb::from_value(prove_js).expect("decode prove");

        // proof is NOT a single Fr; just check it's valid hex with 0x prefix
        assert!(is_hex_prefixed(&prove_out.proof));
        // publics are Fr-sized hex values
        for p in &prove_out.publics {
            assert!(is_fr_hex(p));
        }

        // 3) Verify via crate’s verify
        let publics_verify_refs: Vec<&str> = prove_out.publics.iter().map(|s| s.as_str()).collect();
        let ok = zkp_verify(&vk_hex, &prove_out.proof, &publics_verify_refs).expect("verify");
        assert!(ok, "verification should succeed");

        Ok(())
    }

    #[wasm_bindgen_test]
    fn prove_then_verify_wrapper_ok_and_tamper_fails() -> Result<(), JsValue> {
        // (publics order): [hash_title, hash_audio, hash_creators, commitment, timestamp, nullifier]
        let secret = "0x23864adb160dddf590f1d3303683ebcb914f828e2635f6e85a32f0a1aecd3dd8";
        let hash_title = "0x175eeef716d52cf8ee972c6fefd60e47df5084efde3c188c40a81a42e72dfb04";
        let hash_audio = "0x26d273f7c73a635f6eaeb904e116ec4cd887fb5a87fc7427c95279e6053e5bf0";
        let hash_creators = "0x017ac5e7a52bec07ca8ee344a9979aa083b7713f1196af35310de21746985079";
        let timestamp = fr_to_hex_be(&fr_u64(10_000u64));

        // Commitment + nullifier off-chain
        let (commitment, nullifier) = super::compute_commitment_nullifier(
            hash_title,
            hash_audio,
            hash_creators,
            secret,
            &timestamp,
        )
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let publics_vec = vec![
            hash_title.to_string(),
            hash_audio.to_string(),
            hash_creators.to_string(),
            commitment.clone(),
            timestamp.clone(),
            nullifier.clone(),
        ];
        let publics_refs: Vec<&str> = publics_vec.iter().map(|s| s.as_str()).collect();

        // 1) Setup (PK/VK as hex)
        let (pk_hex, vk_hex) = zkp_setup(secret, &publics_refs).expect("setup");

        // 2) Prove via WASM wrapper
        let publics_js = swb::to_value(&publics_vec).unwrap();
        let prove_js = super::prove(&pk_hex, secret, publics_js).expect("prove wrapper");
        let prove_out: super::ProveOutput = swb::from_value(prove_js).expect("decode prove");

        // 3) Verify via module's verify
        let publics_js = serde_wasm_bindgen::to_value(&prove_out.publics)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let ok = super::verify(&vk_hex, &prove_out.proof, publics_js)?;
        assert!(ok, "verify(true) should succeed with consistent publics");

        // Tamper (per ex. timestamp + 1)
        let mut tampered = prove_out.publics.clone();
        tampered[4] = fr_to_hex_be(&fr_u64(10_001u64));
        let tampered_js = serde_wasm_bindgen::to_value(&tampered)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let ok2 = super::verify(&vk_hex, &prove_out.proof, tampered_js)?;
        assert!(!ok2, "verify(false) should fail when publics mismatch");

        Ok(())
    }
}
