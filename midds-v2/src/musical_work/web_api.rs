use super::*;

#[wasm_bindgen]
impl MusicalWork {
    /// Creates a new MusicalWork from JavaScript.
    ///
    /// # Arguments
    /// * `iswc_str` - The ISWC string
    /// * `title` - The title of the work
    ///
    /// # Examples
    /// ```javascript
    /// import { MusicalWork, Creator, CreatorRole } from 'allfeat-midds-v2';
    ///
    /// try {
    ///   const work = MusicalWork.new("T-034524680-8", "My Song");
    ///   work.addCreator(new Creator(123, CreatorRole.Composer));
    /// } catch (error) {
    ///   console.error("Invalid musical work:", error);
    /// }
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new_web(iswc_str: &str, title: &str) -> Result<MusicalWork, JsError> {
        let iswc = match iswc::Iswc::new_web(iswc_str) {
            Ok(iswc) => iswc,
            Err(_) => return Err(JsError::new("Invalid ISWC")),
        };

        if title.trim().is_empty() {
            return Err(JsError::new("Title cannot be empty"));
        }

        Ok(MusicalWork {
            iswc,
            title: title.to_string(),
            creation_year: None,
            instrumental: None,
            language: None,
            bpm: None,
            key: None,
            work_type: None,
            creators: vec![],
            classical_info: None,
        })
    }

    /// Sets the creation year.
    #[wasm_bindgen(js_name = setCreationYear)]
    pub fn set_creation_year_web(&mut self, year: u16) -> Result<(), JsError> {
        if year < 1000 || year > 2034 {
            return Err(JsError::new("Invalid creation year"));
        }
        self.creation_year = Some(year);
        Ok(())
    }

    /// Adds a creator to the work.
    #[wasm_bindgen(js_name = addCreator)]
    pub fn add_creator_web(&mut self, creator: Creator) {
        self.creators.push(creator);
    }

    /// Gets the number of creators.
    #[wasm_bindgen(js_name = creatorCount)]
    pub fn creator_count_web(&self) -> usize {
        self.creators.len()
    }

    /// Checks if this is a classical work.
    #[wasm_bindgen(js_name = isClassical)]
    pub fn is_classical_web(&self) -> bool {
        self.classical_info.is_some()
    }
}
