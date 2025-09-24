use allfeat_ats_zkp::{Creator, Roles, fr_to_hex_be, hash_creators_fr, hash_title_fr};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

// -------------------- Hash Title ---------------------------------------------

#[wasm_bindgen]
pub fn hash_title(title: &str) -> String {
    let fr = hash_title_fr(title);
    fr_to_hex_be(&fr)
}

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

#[wasm_bindgen]
pub fn hash_creators(creators_js: JsValue) -> Result<String, JsValue> {
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

    let fr = hash_creators_fr(&creators_core);
    Ok(fr_to_hex_be(&fr))
}
