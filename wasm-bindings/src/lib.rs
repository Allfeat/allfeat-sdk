//! Unified WebAssembly bindings for Allfeat SDK
//!
//! This crate provides a single entry point for MIDDS functionality,
//! organized for optimal TypeScript integration.

use wasm_bindgen::prelude::*;

// Initialize the WebAssembly module
#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"🚀 Allfeat SDK bindings loaded!".into());
}

pub use allfeat_midds::*;

// Utility functions for the unified bindings
#[wasm_bindgen(js_name = "getSdkVersion")]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
