use crate::musical_work::error::MusicalWorkError;

use super::*;
use regex::Regex;
use std::fmt;

static TITLE_REGEX: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
static OPUS_REGEX: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();

impl MusicalWork {
    /// Creates a new MusicalWork with validation.
    ///
    /// # Arguments
    /// * `iswc` - The ISWC identifier
    /// * `title` - The title of the work
    /// * `creators` - List of creators (must not be empty)
    ///
    /// # Returns
    /// * `Ok(MusicalWork)` if all fields are valid
    /// * `Err(MusicalWorkError)` if validation fails
    ///
    /// # Examples
    /// ```
    /// use allfeat_midds_v2::musical_work::*;
    /// use allfeat_midds_v2::musical_work::iswc::Iswc;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let work = MusicalWork::new(
    ///     Iswc::new("T-034524680-8")?,
    ///     "My Song",
    ///     vec![Creator { id: 123, role: CreatorRole::Composer }]
    /// )?;
    /// assert_eq!(work.title, "My Song");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(
        iswc: Iswc,
        title: impl Into<String>,
        creators: Vec<Creator>,
    ) -> Result<Self, MusicalWorkError> {
        let title = title.into();

        Self::validate_title(&title)?;
        Self::validate_creators(&creators)?;

        Ok(Self {
            iswc,
            title: title.to_string(),
            creation_year: None,
            instrumental: None,
            language: None,
            bpm: None,
            key: None,
            work_type: None,
            creators,
            classical_info: None,
        })
    }

    /// Creates a new MusicalWork without validation (unsafe).
    ///
    /// # Safety
    /// The caller must ensure all fields are valid.
    pub fn new_unchecked(iswc: Iswc, title: impl Into<String>, creators: Vec<Creator>) -> Self {
        Self {
            iswc,
            title: title.into(),
            creation_year: None,
            instrumental: None,
            language: None,
            bpm: None,
            key: None,
            work_type: None,
            creators,
            classical_info: None,
        }
    }

    /// Builder pattern - sets the creation year with validation.
    ///
    /// # Arguments
    /// * `year` - Year between 1000 and current year + 10
    ///
    /// # Examples
    /// ```
    /// # use allfeat_midds_v2::musical_work::*;
    /// # use allfeat_midds_v2::musical_work::iswc::Iswc;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let work = MusicalWork::new(
    ///     Iswc::new("T-034524680-8")?,
    ///     "Song",
    ///     vec![Creator { id: 1, role: CreatorRole::Composer }]
    /// )?
    /// .with_creation_year(2024)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_creation_year(mut self, year: u16) -> Result<Self, MusicalWorkError> {
        Self::validate_creation_year(year)?;
        self.creation_year = Some(year);
        Ok(self)
    }

    /// Builder pattern - sets whether the work is instrumental.
    pub fn with_instrumental(mut self, instrumental: bool) -> Self {
        self.instrumental = Some(instrumental);
        self
    }

    /// Builder pattern - sets the language.
    pub fn with_language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    /// Builder pattern - sets the BPM with validation.
    ///
    /// # Arguments
    /// * `bpm` - BPM between 30 and 300
    pub fn with_bpm(mut self, bpm: u16) -> Result<Self, MusicalWorkError> {
        Self::validate_bpm(bpm)?;
        self.bpm = Some(bpm);
        Ok(self)
    }

    /// Builder pattern - sets the musical key.
    pub fn with_key(mut self, key: Key) -> Self {
        self.key = Some(key);
        self
    }

    /// Builder pattern - sets the work type.
    pub fn with_work_type(mut self, work_type: MusicalWorkType) -> Self {
        self.work_type = Some(work_type);
        self
    }

    /// Builder pattern - sets classical information.
    pub fn with_classical_info(mut self, classical_info: ClassicalInfo) -> Self {
        self.classical_info = Some(classical_info);
        self
    }

    /// Adds a creator to the work.
    pub fn add_creator(&mut self, creator: Creator) {
        self.creators.push(creator);
    }

    /// Removes creators with the specified role.
    pub fn remove_creators_by_role(&mut self, role: CreatorRole) {
        self.creators.retain(|c| c.role != role);
    }

    /// Gets all creators with a specific role.
    pub fn get_creators_by_role(&self, role: CreatorRole) -> Vec<&Creator> {
        self.creators.iter().filter(|c| c.role == role).collect()
    }

    /// Gets the primary composer (first composer in the list).
    pub fn primary_composer(&self) -> Option<&Creator> {
        self.creators
            .iter()
            .find(|c| c.role == CreatorRole::Composer)
    }

    /// Checks if the work has lyrics (not instrumental).
    pub fn has_lyrics(&self) -> bool {
        !self.instrumental.unwrap_or(false)
    }

    /// Checks if this is a classical work.
    pub fn is_classical(&self) -> bool {
        self.classical_info.is_some()
    }

    /// Checks if this is an original work.
    pub fn is_original(&self) -> bool {
        matches!(self.work_type, Some(MusicalWorkType::Original) | None)
    }

    /// Gets related work IDs if this is a derived work.
    pub fn related_work_ids(&self) -> Vec<MiddsId> {
        match &self.work_type {
            Some(MusicalWorkType::Medley(ids)) => ids.clone(),
            Some(MusicalWorkType::Mashup(ids)) => ids.clone(),
            Some(MusicalWorkType::Adaptation(id)) => vec![*id],
            _ => vec![],
        }
    }

    /// Normalizes the title by removing extra whitespace and trimming.
    pub fn normalize_title(title: &str) -> String {
        // Remove multiple spaces and trim
        let regex = TITLE_REGEX
            .get_or_init(|| Regex::new(r"\s+").expect("Title normalization regex is valid"));

        regex.replace_all(title.trim(), " ").to_string()
    }

    /// Creates a search-friendly version of the title.
    pub fn searchable_title(&self) -> String {
        Self::normalize_title(&self.title).to_lowercase()
    }

    /// Validates a title.
    fn validate_title(title: &str) -> Result<(), MusicalWorkError> {
        let normalized = Self::normalize_title(title);

        if normalized.is_empty() {
            return Err(MusicalWorkError::InvalidTitle(
                "Title cannot be empty".to_string(),
            ));
        }

        if normalized.len() > 256 {
            return Err(MusicalWorkError::InvalidTitle(
                "Title too long (max 256 chars)".to_string(),
            ));
        }

        Ok(())
    }

    /// Validates a creation year.
    fn validate_creation_year(year: u16) -> Result<(), MusicalWorkError> {
        let current_year = 2025u16;

        if year < 1000 || year > current_year + 10 {
            return Err(MusicalWorkError::InvalidCreationYear(year));
        }

        Ok(())
    }

    /// Validates a BPM value.
    fn validate_bpm(bpm: u16) -> Result<(), MusicalWorkError> {
        if !(30..=300).contains(&bpm) {
            return Err(MusicalWorkError::InvalidBpm(bpm));
        }

        Ok(())
    }

    /// Validates the creators list.
    fn validate_creators(creators: &[Creator]) -> Result<(), MusicalWorkError> {
        if creators.is_empty() {
            return Err(MusicalWorkError::EmptyCreators);
        }

        Ok(())
    }
}

impl ClassicalInfo {
    /// Creates new classical information with validation.
    pub fn new(
        opus: Option<String>,
        catalog_number: Option<String>,
        number_of_voices: Option<u16>,
    ) -> Result<Self, MusicalWorkError> {
        if let Some(ref opus) = opus {
            Self::validate_opus(opus)?;
        }

        if let Some(ref catalog) = catalog_number {
            Self::validate_catalog_number(catalog)?;
        }

        if let Some(voices) = number_of_voices {
            Self::validate_voices(voices)?;
        }

        Ok(Self {
            opus,
            catalog_number,
            number_of_voices,
        })
    }

    /// Normalizes an opus string.
    pub fn normalize_opus(opus: &str) -> String {
        let regex = OPUS_REGEX
            .get_or_init(|| Regex::new(r"(?i)^op\.?\s*(.+)$").expect("Opus regex is valid"));

        if let Some(captures) = regex.captures(opus.trim()) {
            format!("Op. {}", captures.get(1).unwrap().as_str())
        } else {
            opus.trim().to_string()
        }
    }

    /// Normalizes a catalog number.
    pub fn normalize_catalog_number(catalog: &str) -> String {
        catalog.trim().to_uppercase()
    }

    /// Validates an opus string.
    fn validate_opus(opus: &str) -> Result<(), MusicalWorkError> {
        if opus.trim().is_empty() {
            return Err(MusicalWorkError::InvalidOpus(
                "Opus cannot be empty".to_string(),
            ));
        }

        if opus.len() > 256 {
            return Err(MusicalWorkError::InvalidOpus(
                "Opus too long (max 256 chars)".to_string(),
            ));
        }

        Ok(())
    }

    /// Validates a catalog number.
    fn validate_catalog_number(catalog: &str) -> Result<(), MusicalWorkError> {
        if catalog.trim().is_empty() {
            return Err(MusicalWorkError::InvalidCatalogNumber(
                "Catalog number cannot be empty".to_string(),
            ));
        }

        if catalog.len() > 256 {
            return Err(MusicalWorkError::InvalidCatalogNumber(
                "Catalog number too long (max 256 chars)".to_string(),
            ));
        }

        Ok(())
    }

    /// Validates number of voices.
    fn validate_voices(voices: u16) -> Result<(), MusicalWorkError> {
        if voices == 0 || voices > 100 {
            return Err(MusicalWorkError::InvalidVoices(voices));
        }

        Ok(())
    }
}

impl Creator {
    /// Creates a new creator.
    pub fn new(id: MiddsId, role: CreatorRole) -> Self {
        Self { id, role }
    }

    /// Creates a composer.
    pub fn composer(id: MiddsId) -> Self {
        Self::new(id, CreatorRole::Composer)
    }

    /// Creates an author (lyricist).
    pub fn author(id: MiddsId) -> Self {
        Self::new(id, CreatorRole::Author)
    }

    /// Creates an arranger.
    pub fn arranger(id: MiddsId) -> Self {
        Self::new(id, CreatorRole::Arranger)
    }

    /// Creates an adapter.
    pub fn adapter(id: MiddsId) -> Self {
        Self::new(id, CreatorRole::Adapter)
    }

    /// Creates a publisher.
    pub fn publisher(id: MiddsId) -> Self {
        Self::new(id, CreatorRole::Publisher)
    }
}

impl MusicalWorkType {
    /// Creates an original work type.
    pub fn original() -> Self {
        Self::Original
    }

    /// Creates a medley work type.
    pub fn medley(work_ids: Vec<MiddsId>) -> Self {
        Self::Medley(work_ids)
    }

    /// Creates a mashup work type.
    pub fn mashup(work_ids: Vec<MiddsId>) -> Self {
        Self::Mashup(work_ids)
    }

    /// Creates an adaptation work type.
    pub fn adaptation(work_id: MiddsId) -> Self {
        Self::Adaptation(work_id)
    }

    /// Checks if this is an original work.
    pub fn is_original(&self) -> bool {
        matches!(self, Self::Original)
    }

    /// Checks if this is a derived work.
    pub fn is_derived(&self) -> bool {
        !self.is_original()
    }

    /// Gets the source work IDs if this is a derived work.
    pub fn source_work_ids(&self) -> Vec<MiddsId> {
        match self {
            Self::Medley(ids) => ids.clone(),
            Self::Mashup(ids) => ids.clone(),
            Self::Adaptation(id) => vec![*id],
            Self::Original => vec![],
        }
    }
}

impl fmt::Display for MusicalWork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.title, self.iswc)
    }
}

impl fmt::Display for CreatorRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CreatorRole::Author => write!(f, "Author"),
            CreatorRole::Composer => write!(f, "Composer"),
            CreatorRole::Arranger => write!(f, "Arranger"),
            CreatorRole::Adapter => write!(f, "Adapter"),
            CreatorRole::Publisher => write!(f, "Publisher"),
        }
    }
}

impl fmt::Display for MusicalWorkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MusicalWorkType::Original => write!(f, "Original"),
            MusicalWorkType::Medley(ids) => write!(f, "Medley of {} works", ids.len()),
            MusicalWorkType::Mashup(ids) => write!(f, "Mashup of {} works", ids.len()),
            MusicalWorkType::Adaptation(id) => write!(f, "Adaptation of work {}", id),
        }
    }
}
