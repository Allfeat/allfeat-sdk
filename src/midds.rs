use primitives_macros::allfeat_string;
use wasm_bindgen::{JsError, prelude::wasm_bindgen};

use crate::metadata::melodie::{self, runtime_types::midds::musical_work::MusicalWork};

#[wasm_bindgen(js_name = "MusicalWork")]
#[derive(Debug)]
pub struct MusicalWorkWrapper(MusicalWork);

#[wasm_bindgen]
impl MusicalWorkWrapper {
    #[wasm_bindgen(getter)]
    pub fn iswc(&self) -> Result<Iswc, JsError> {
        (&self.0.iswc).try_into()
    }
    #[wasm_bindgen(setter)]
    pub fn set_iswc(&mut self, iswc: Iswc) {
        self.0.iswc = iswc.into()
    }
}

#[allfeat_string]
pub struct Iswc;
