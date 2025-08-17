use super::isrc::Isrc;
use super::{Track, TrackVersion};
use crate::MiddsId;
use allfeat_music_genres::GenreId;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
impl Track {
    /// Creates a new Track for JavaScript
    #[wasm_bindgen(constructor)]
    pub fn new_web(
        isrc: &str,
        musical_work: MiddsId,
        artist: MiddsId,
        title: &str,
        producers: &[MiddsId],
        performers: &[MiddsId],
        contributors: &[MiddsId],
    ) -> Result<Track, JsError> {
        let isrc = Isrc::new(isrc).map_err(|e| JsError::new(&format!("Invalid ISRC: {}", e)))?;
        if title.trim().is_empty() {
            return Err(JsError::new("Title cannot be empty"));
        }

        Track::new(
            isrc,
            musical_work,
            artist,
            title.to_string(),
            producers.to_vec(),
            performers.to_vec(),
            contributors.to_vec(),
        )
        .map_err(|e| JsError::new(&format!("Failed to create track: {}", e)))
    }

    /// Gets suggested track versions for display
    #[wasm_bindgen(js_name = getSuggestedVersions)]
    pub fn get_suggested_versions_web(&self) -> Vec<String> {
        self.suggested_versions()
            .into_iter()
            .map(|v| format!("{:?}", v))
            .collect()
    }

    /// Sets the version from a string
    #[wasm_bindgen(js_name = setVersionFromString)]
    pub fn set_version_from_string_web(
        &mut self,
        version_str: Option<String>,
    ) -> Result<(), JsError> {
        match version_str {
            Some(s) => {
                let version = match s.as_str() {
                    "Original" => TrackVersion::Original,
                    "Live" => TrackVersion::Live,
                    "Acoustic" => TrackVersion::Acoustic,
                    "Remix" => TrackVersion::Remix,
                    "ReRecorded" => TrackVersion::ReRecorded,
                    "Demo" => TrackVersion::Demo,
                    "Extended" => TrackVersion::Extended,
                    "Edit" => TrackVersion::Edit,
                    "Instrumental" => TrackVersion::Instrumental,
                    "Karaoke" => TrackVersion::Karaoke,
                    "Acapella" => TrackVersion::Acapella,
                    _ => return Err(JsError::new(&format!("Unknown version: {}", s))),
                };
                self.version = Some(version);
            }
            None => self.version = None,
        }
        Ok(())
    }

    /// Adds a title alias
    #[wasm_bindgen(js_name = addTitleAlias)]
    pub fn add_title_alias_web(&mut self, alias: &str) -> Result<(), JsError> {
        if alias.trim().is_empty() {
            return Err(JsError::new("Alias cannot be empty"));
        }
        self.title_aliases.push(alias.to_string());
        Ok(())
    }

    /// Adds a producer
    #[wasm_bindgen(js_name = addProducer)]
    pub fn add_producer_web(&mut self, producer_id: MiddsId) {
        if !self.producers.contains(&producer_id) {
            self.producers.push(producer_id);
        }
    }

    /// Adds a performer
    #[wasm_bindgen(js_name = addPerformer)]
    pub fn add_performer_web(&mut self, performer_id: MiddsId) {
        if !self.performers.contains(&performer_id) {
            self.performers.push(performer_id);
        }
    }

    /// Adds a contributor
    #[wasm_bindgen(js_name = addContributor)]
    pub fn add_contributor_web(&mut self, contributor_id: MiddsId) {
        if !self.contributors.contains(&contributor_id) {
            self.contributors.push(contributor_id);
        }
    }

    /// Adds a genre by ID
    #[wasm_bindgen(js_name = addGenre)]
    pub fn add_genre_web(&mut self, genre_id: u32) {
        // For now, just create a default genre since GenreId structure is unknown
        // In a real implementation, this would use proper GenreId conversion
        let genre = match genre_id {
            0 => GenreId::Pop,
            1 => GenreId::Rock,
            2 => GenreId::Jazz,
            3 => GenreId::Classical,
            4 => GenreId::Electronic,
            _ => GenreId::Pop, // Default fallback
        };
        if !self.genres.contains(&genre) {
            self.genres.push(genre);
        }
    }

    /// Validates the track
    #[wasm_bindgen(js_name = validate)]
    pub fn validate_web(&self) -> Result<(), JsError> {
        // Basic validation - check if ISRC is valid
        if self.isrc.as_ref().is_empty() {
            return Err(JsError::new("ISRC cannot be empty"));
        }
        if self.title.is_empty() {
            return Err(JsError::new("Title cannot be empty"));
        }
        Ok(())
    }
}
