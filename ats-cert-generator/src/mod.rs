use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

mod pdf;

// Optional: use wee_alloc as the global allocator for smaller WASM size
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Set up console logging for WASM using web_sys
macro_rules! console_log {
    ($($t:tt)*) => {
        console::log_1(&format!($($t)*).into())
    }
}

// Certificate data structures (internal only, accessed via JS objects)
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
    fullname: String,
    email: String,
    roles: Vec<String>,
    #[serde(default)]
    ipi: String,
    #[serde(default)]
    isni: String,
}

#[wasm_bindgen]
impl Creator {
    #[wasm_bindgen(constructor)]
    pub fn new(
        fullname: String,
        email: String,
        roles: Vec<String>,
        ipi: String,
        isni: String,
    ) -> Self {
        Self {
            fullname,
            email,
            roles,
            ipi,
            isni,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn fullname(&self) -> String {
        self.fullname.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn email(&self) -> String {
        self.email.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn roles(&self) -> Vec<String> {
        self.roles.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn ipi(&self) -> String {
        self.ipi.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn isni(&self) -> String {
        self.isni.clone()
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateData {
    title: String,
    asset_filename: String,
    creators: Vec<Creator>,
    hash_audio: String,
    hash_title: String,
    hash_creators: String,
    secret: String,
    hash_commitment: String,

    timestamp: String,
}

#[wasm_bindgen]
impl CertificateData {
    #[wasm_bindgen(constructor)]
    pub fn new(
        title: String,
        asset_filename: String,
        hash_audio: String,
        hash_title: String,
        hash_creators: String,
        secret: String,
        hash_commitment: String,
        timestamp: String,
    ) -> Self {
        Self {
            title,
            asset_filename,
            creators: Vec::new(),
            hash_audio,
            hash_title,
            hash_creators,
            secret,
            hash_commitment,
            timestamp,
        }
    }

    #[wasm_bindgen(js_name = addCreator)]
    pub fn add_creator(&mut self, creator: Creator) {
        self.creators.push(creator);
    }
}

// Main WASM export - PDF generation only

// PDF generation from JavaScript object
#[wasm_bindgen]
pub fn generate_pdf_from_js_object(certificate_data: CertificateData) -> Result<Vec<u8>, JsValue> {
    console_log!("Generating PDF from CertificateData");

    pdf::PdfGenerator::generate_certificate_pdf(&certificate_data)
        .map_err(|e| JsValue::from_str(&format!("Failed to generate PDF: {}", e)))
}

// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    console_log!("ATS Certificate Generator WASM module initialized");
}
