use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
    fullname: String,
    email: String,
    roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorInput {
    pub fullname: String,
    pub email: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtsCertificateInput {
    pub id_allfeat: String,
    pub version_number: String,
    pub title: String,
    pub asset_filename: String,
    pub creators: Vec<CreatorInput>,
}

#[wasm_bindgen]
impl Creator {
    #[wasm_bindgen(constructor)]
    pub fn new(fullname: String, email: String, roles: Vec<String>) -> Self {
        Self {
            fullname,
            email,
            roles,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn fullname(&self) -> String {
        self.fullname.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_fullname(&mut self, fullname: String) {
        self.fullname = fullname;
    }

    #[wasm_bindgen(getter)]
    pub fn email(&self) -> String {
        self.email.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    #[wasm_bindgen(getter)]
    pub fn roles(&self) -> Vec<String> {
        self.roles.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_roles(&mut self, roles: Vec<String>) {
        self.roles = roles;
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtsCertificate {
    id_allfeat: String,
    version_number: String,
    title: String,
    asset_filename: String,
    creators: Vec<Creator>,
}

#[wasm_bindgen]
impl AtsCertificate {
    #[wasm_bindgen(constructor)]
    pub fn new(
        id_allfeat: String,
        version_number: String,
        title: String,
        asset_filename: String,
    ) -> Self {
        Self {
            id_allfeat,
            version_number,
            title,
            asset_filename,
            creators: Vec::new(),
        }
    }

    #[wasm_bindgen(getter, js_name = idAllfeat)]
    pub fn id_allfeat(&self) -> String {
        self.id_allfeat.clone()
    }

    #[wasm_bindgen(setter, js_name = idAllfeat)]
    pub fn set_id_allfeat(&mut self, id: String) {
        self.id_allfeat = id;
    }

    #[wasm_bindgen(getter, js_name = versionNumber)]
    pub fn version_number(&self) -> String {
        self.version_number.clone()
    }

    #[wasm_bindgen(setter, js_name = versionNumber)]
    pub fn set_version_number(&mut self, version: String) {
        self.version_number = version;
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.title.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    #[wasm_bindgen(getter, js_name = assetFilename)]
    pub fn asset_filename(&self) -> String {
        self.asset_filename.clone()
    }

    #[wasm_bindgen(setter, js_name = assetFilename)]
    pub fn set_asset_filename(&mut self, filename: String) {
        self.asset_filename = filename;
    }

    #[wasm_bindgen(js_name = addCreator)]
    pub fn add_creator(&mut self, creator: Creator) {
        self.creators.push(creator);
    }

    #[wasm_bindgen(js_name = getCreatorsCount)]
    pub fn get_creators_count(&self) -> usize {
        self.creators.len()
    }

    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = fromJson)]
    pub fn from_json(value: JsValue) -> Result<AtsCertificate, JsValue> {
        serde_wasm_bindgen::from_value(value).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

// FLOW 1: Parse JSON file content to structured data for form pre-filling
#[wasm_bindgen(js_name = parseAtsCertificate)]
pub fn parse_ats_certificate(json_str: &str) -> Result<AtsCertificate, JsValue> {
    serde_json::from_str(json_str)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {}", e)))
}

#[wasm_bindgen(js_name = parseAtsCertificateToJs)]
pub fn parse_ats_certificate_to_js(json_str: &str) -> Result<JsValue, JsValue> {
    let cert = parse_ats_certificate(json_str)?;
    cert.to_json()
}

#[wasm_bindgen(js_name = parseAtsCertificateFromFile)]
pub fn parse_ats_certificate_from_file(file_content: &str) -> Result<JsValue, JsValue> {
    let input: AtsCertificateInput = serde_json::from_str(file_content)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse ATS file: {}", e)))?;

    // Convert to our WASM-compatible structure
    let mut cert = AtsCertificate::new(
        input.id_allfeat,
        input.version_number,
        input.title,
        input.asset_filename,
    );

    // Add creators
    for creator_input in input.creators {
        let creator = Creator::new(
            creator_input.fullname,
            creator_input.email,
            creator_input.roles,
        );
        cert.add_creator(creator);
    }

    cert.to_json()
}

// FLOW 2: Generate JSON from structured data (form data → JSON file)
#[wasm_bindgen(js_name = generateAtsCertificateJson)]
pub fn generate_ats_certificate_json(cert: &AtsCertificate) -> Result<String, JsValue> {
    serde_json::to_string_pretty(cert)
        .map_err(|e| JsValue::from_str(&format!("Failed to generate JSON: {}", e)))
}

#[wasm_bindgen(js_name = generateAtsCertificateFromData)]
pub fn generate_ats_certificate_from_data(
    id_allfeat: &str,
    version_number: &str,
    title: &str,
    asset_filename: &str,
    creators_json: &str,
) -> Result<String, JsValue> {
    // Parse creators from JSON
    let creators_input: Vec<CreatorInput> = serde_json::from_str(creators_json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse creators: {}", e)))?;

    // Create certificate
    let mut cert = AtsCertificate::new(
        id_allfeat.to_string(),
        version_number.to_string(),
        title.to_string(),
        asset_filename.to_string(),
    );

    // Add creators
    for creator_input in creators_input {
        let creator = Creator::new(
            creator_input.fullname,
            creator_input.email,
            creator_input.roles,
        );
        cert.add_creator(creator);
    }

    generate_ats_certificate_json(&cert)
}

#[wasm_bindgen(js_name = createAtsCertificateFromJsObject)]
pub fn create_ats_certificate_from_js_object(js_obj: JsValue) -> Result<String, JsValue> {
    let input: AtsCertificateInput = serde_wasm_bindgen::from_value(js_obj)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse JavaScript object: {}", e)))?;

    let certificate_json = serde_json::to_string_pretty(&input)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize to JSON: {}", e)))?;

    Ok(certificate_json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_certificate() {
        let json = r#"{
            "id_allfeat": "285328923",
            "version_number": "toto",
            "title": "titi",
            "asset_filename": "test",
            "creators": [
                {
                    "fullname": "jad",
                    "email": "jad@gmail.com",
                    "roles": ["Author", "Composer"]
                }
            ]
        }"#;

        let result = parse_ats_certificate(json);
        assert!(result.is_ok());

        let cert = result.unwrap();
        assert_eq!(cert.id_allfeat, "285328923");
        assert_eq!(cert.version_number, "toto");
        assert_eq!(cert.title, "titi");
        assert_eq!(cert.asset_filename, "test");
        assert_eq!(cert.creators.len(), 1);
    }
}
