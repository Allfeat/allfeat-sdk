use super::ean::Ean;
use super::Release;
use crate::{
    utils::{Country, Date},
    MiddsId,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Release {
    /// Creates a new Release for JavaScript
    #[wasm_bindgen(constructor)]
    pub fn new_web(
        ean_upc: &str,
        artist: MiddsId,
        title: &str,
        tracks: &[MiddsId],
        year: u16,
        month: u8,
        day: u8,
        country: u32,
    ) -> Result<Release, JsError> {
        let ean = Ean::new(ean_upc).map_err(|e| JsError::new(&format!("Invalid EAN: {}", e)))?;
        if title.trim().is_empty() {
            return Err(JsError::new("Title cannot be empty"));
        }

        let date = Date { year, month, day };
        let country_enum = match country {
            840 => Country::US,
            826 => Country::GB,
            250 => Country::FR,
            276 => Country::DE,
            392 => Country::JP,
            _ => Country::US, // Default fallback
        };

        Release::new(
            ean,
            artist,
            title.to_string(),
            tracks.to_vec(),
            date,
            country_enum,
        )
        .map_err(|e| JsError::new(&format!("Failed to create release: {}", e)))
    }
}

/// Utility functions for JavaScript
#[wasm_bindgen]
pub struct ReleaseUtils;

#[wasm_bindgen]
impl ReleaseUtils {
    /// Validates an EAN/UPC string
    #[wasm_bindgen(js_name = validateEan)]
    pub fn validate_ean_web(ean: &str) -> bool {
        Ean::new(ean).is_ok()
    }

    /// Validates a release title
    #[wasm_bindgen(js_name = validateTitle)]
    pub fn validate_title_web(title: &str) -> bool {
        !title.trim().is_empty() && title.len() <= 256
    }

    /// Validates a contributor name
    #[wasm_bindgen(js_name = validateContributor)]
    pub fn validate_contributor_web(name: &str) -> bool {
        !name.trim().is_empty() && name.len() <= 256
    }

    /// Gets format name by index
    #[wasm_bindgen(js_name = getFormatName)]
    pub fn get_format_name_web(index: u32) -> String {
        match index {
            0 => "CD".to_string(),
            1 => "Double CD".to_string(),
            2 => "7\" Vinyl".to_string(),
            3 => "10\" Vinyl".to_string(),
            4 => "Cassette".to_string(),
            5 => "Audio DVD".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    /// Gets packaging name by index
    #[wasm_bindgen(js_name = getPackagingName)]
    pub fn get_packaging_name_web(index: u32) -> String {
        match index {
            0 => "Digipack".to_string(),
            1 => "Jewel Case".to_string(),
            2 => "Snap Case".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    /// Gets status name by index
    #[wasm_bindgen(js_name = getStatusName)]
    pub fn get_status_name_web(index: u32) -> String {
        match index {
            0 => "Official".to_string(),
            1 => "Promotional".to_string(),
            2 => "Re-Release".to_string(),
            3 => "Special Edition".to_string(),
            4 => "Remastered".to_string(),
            5 => "Bootleg".to_string(),
            6 => "Pseudo Release".to_string(),
            7 => "Withdrawn".to_string(),
            8 => "Expunged".to_string(),
            9 => "Cancelled".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}
