use allfeat_ats_zkp::{
    Creator, Roles, fr_to_hex_be, hash_audio, hash_creators, hash_title,
    poseidon_commitment_offchain, poseidon_nullifier_offchain, poseidon_params,
    zkp::prove as groth16_prove_hex,
};
use ark_bn254::Fr;
use ark_ff::UniformRand;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Deserialize)]
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

// Internal holder for already-computed items, all HEX except timestamp.
#[derive(Debug, Clone)]
struct ZkpInputsHex {
    secret: String,
    hash_title: String,
    hash_audio: String,
    hash_creators: String,
    commitment: String,
    nullifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkpBundleHex {
    pub secret: String,
    pub timestamp: u64,
    pub hash_title: String,
    pub hash_audio: String,
    pub hash_creators: String,
    pub commitment: String,
    pub nullifier: String,
}

fn to_hex_bundle(h: &ZkpInputsHex, timestamp: u64) -> ZkpBundleHex {
    ZkpBundleHex {
        secret: h.secret.clone(),
        timestamp,
        hash_title: h.hash_title.clone(),
        hash_audio: h.hash_audio.clone(),
        hash_creators: h.hash_creators.clone(),
        commitment: h.commitment.clone(),
        nullifier: h.nullifier.clone(),
    }
}

// -------------------- Off-chain Poseidon (hex in/out) ------------------------

fn compute_commitment_nullifier(
    hash_audio: &str,
    hash_title: &str,
    hash_creators: &str,
    secret: &str,
    timestamp: u64,
) -> (String, String) {
    let cfg = poseidon_params();
    let commitment =
        poseidon_commitment_offchain(hash_audio, hash_title, hash_creators, secret, &cfg);
    let nullifier = poseidon_nullifier_offchain(&commitment, timestamp, &cfg);
    (commitment, nullifier)
}

// -------------------- Exposed WASM functions ---------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct BuildBundleOutput {
    #[serde(flatten)]
    pub bundle: ZkpBundleHex,
}

/// Build a full precomputed bundle (random secret):
/// - inputs: `title`, `audio_bytes` (Uint8Array), `creators` (array of JsCreator), `timestamp` (seconds)
/// - returns: all hashes + commitment + nullifier as hex, plus the numeric timestamp
#[wasm_bindgen]
pub fn build_zkp_bundle(
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

    // 3) commitment + nullifier (hex)
    let (commitment, nullifier) =
        compute_commitment_nullifier(&hash_audio, &hash_title, &hash_creators, &secret, timestamp);

    // 4) build outputs (all hex)
    let hex_inputs = ZkpInputsHex {
        secret,
        hash_title,
        hash_audio,
        hash_creators,
        commitment,
        nullifier,
    };

    let out = BuildBundleOutput {
        bundle: to_hex_bundle(&hex_inputs, timestamp),
    };

    Ok(serde_wasm_bindgen::to_value(&out)?)
}

#[derive(Debug, Clone, Serialize)]
pub struct ProveOutput {
    pub proof: String,
    /// Publics in circuit order (hex):
    /// [hash_audio, hash_title, hash_creators, commitment, timestamp, nullifier]
    pub publics: [String; 6],
}

/// Groth16 proof (hex-only API passthrough):
/// - `pk`: compressed PK (0x-hex)
/// - `secret`: 0x-hex Fr
/// - `publics`: array(6) of 0x-hex Fr in circuit order
#[wasm_bindgen]
pub fn zkp_prove(pk: &str, secret: &str, publics: JsValue) -> Result<JsValue, JsValue> {
    let publics: Vec<String> = serde_wasm_bindgen::from_value(publics)
        .map_err(|e| JsValue::from_str(&format!("publics must be 6 hex strings: {e}")))?;
    if publics.len() != 6 {
        return Err(JsValue::from_str("publics must have length 6"));
    }
    let publics_refs: Vec<&str> = publics.iter().map(|s| s.as_str()).collect();

    // Call your zkp.rs hex-only prove (it already manages RNG internally)
    let (proof, publics_out) = groth16_prove_hex(pk, secret, &publics_refs)
        .map_err(|_| JsValue::from_str("prove() failed"))?;

    let out = ProveOutput {
        proof,
        publics: publics_out,
    };
    Ok(serde_wasm_bindgen::to_value(&out)?)
}
