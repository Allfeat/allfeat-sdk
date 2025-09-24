use allfeat_ats_zkp::{
    Creator, Roles, fr_to_hex_be, hash_audio_fr, hash_creators_fr, hash_title_fr,
    poseidon_h2_offchain, poseidon_h4_offchain, poseidon_params,
};
use ark_bn254::Fr;
use ark_ff::UniformRand;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// -------------------- Hash Creators ------------------------------------------

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

// -------------------- ZKP Bundle ---------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct ZkpBundle {
    // inputs (hex, except timestamp which is numeric)
    pub secret_hex: String,
    pub timestamp: u64,
    pub hash_title_hex: String,
    pub hash_audio_hex: String,
    pub hash_creators_hex: String,
    // derived
    pub commitment_hex: String,
    pub nullifier_hex: String,
}

/// Build a full precomputed bundle, in one call:
/// - inputs: title (str), audio bytes (Uint8Array), creators (array)
/// - inside: generates random secret & current timestamp
/// - returns: all hashes + commitment + nullifier (hex), plus numeric timestamp
#[wasm_bindgen]
pub fn build_zkp_bundle(
    title: &str,
    audio_bytes: &[u8],
    creators_js: JsValue,
    timestamp: u64, // seconds since epoch (JS time is ms)
) -> Result<JsValue, JsValue> {
    // 1) secret & timestamp
    let mut rng = OsRng;
    let secret = Fr::rand(&mut rng);
    let timestamp_fr = Fr::from(timestamp);

    // 2) hashes (Fr)
    let hash_title = hash_title_fr(title);
    let hash_audio = hash_audio_fr(audio_bytes);

    let creators_in: Vec<JsCreator> = serde_wasm_bindgen::from_value(creators_js)
        .map_err(|e| JsValue::from_str(&format!("Invalid creators JSON: {e}")))?;
    let creators_core: Vec<Creator> = creators_in
        .into_iter()
        .map(|j| Creator {
            full_name: j.full_name,
            email: j.email,
            roles: roles_from_codes(j.roles.iter().map(|s| s.as_str())),
            ipi: j.ipi,
            isni: j.isni,
        })
        .collect();
    let hash_creators = hash_creators_fr(&creators_core);

    // 3) commitment + nullifier with the SAME Poseidon params as circuit
    let cfg = poseidon_params();
    let commitment = poseidon_h4_offchain(hash_audio, hash_title, hash_creators, secret, &cfg);
    let nullifier = poseidon_h2_offchain(commitment, timestamp_fr, &cfg);

    // 4) hex-encode outputs for JS
    let out = ZkpBundle {
        secret_hex: fr_to_hex_be(&secret),
        timestamp: timestamp,
        hash_title_hex: fr_to_hex_be(&hash_title),
        hash_audio_hex: fr_to_hex_be(&hash_audio),
        hash_creators_hex: fr_to_hex_be(&hash_creators),
        commitment_hex: fr_to_hex_be(&commitment),
        nullifier_hex: fr_to_hex_be(&nullifier),
    };

    Ok(serde_wasm_bindgen::to_value(&out)?)
}
