use super::{isrc::Isrc, Track, TrackVersion};
use crate::{track::error::TrackError, utils::Key, MiddsId};
use allfeat_music_genres::GenreId;
use std::fmt;

impl Track {
    /// Creates a new track with validation.
    ///
    /// # Arguments
    /// * `isrc` - International Standard Recording Code
    /// * `musical_work` - Reference to the underlying musical work
    /// * `artist` - Primary performing artist ID
    /// * `title` - Title of the track
    /// * `producers` - List of producer IDs
    /// * `performers` - List of performer IDs
    /// * `contributors` - List of contributor IDs
    ///
    /// # Returns
    /// * `Ok(Track)` if all parameters are valid
    /// * `Err(TrackError)` if any parameter is invalid
    pub fn new(
        isrc: Isrc,
        musical_work: MiddsId,
        artist: MiddsId,
        title: String,
        producers: Vec<MiddsId>,
        performers: Vec<MiddsId>,
        contributors: Vec<MiddsId>,
    ) -> Result<Self, TrackError> {
        Self::validate_producers(&producers)?;
        Self::validate_performers(&performers)?;
        Self::validate_contributors(&contributors)?;

        Ok(Self {
            isrc,
            musical_work,
            artist,
            producers,
            performers,
            contributors,
            title,
            title_aliases: Vec::new(),
            recording_year: None,
            genres: Vec::new(),
            version: None,
            duration: None,
            bpm: None,
            key: None,
            recording_place: None,
            mixing_place: None,
            mastering_place: None,
        })
    }

    /// Adds a producer to the track.
    pub fn add_producer(&mut self, producer_id: MiddsId) -> Result<(), TrackError> {
        if self.producers.len() >= 64 {
            return Err(TrackError::TooManyProducers(self.producers.len() + 1));
        }
        self.producers.push(producer_id);
        Ok(())
    }

    /// Removes a producer from the track.
    pub fn remove_producer(&mut self, producer_id: MiddsId) -> bool {
        if let Some(pos) = self.producers.iter().position(|&x| x == producer_id) {
            self.producers.remove(pos);
            true
        } else {
            false
        }
    }

    /// Adds a performer to the track.
    pub fn add_performer(&mut self, performer_id: MiddsId) -> Result<(), TrackError> {
        if self.performers.len() >= 256 {
            return Err(TrackError::TooManyPerformers(self.performers.len() + 1));
        }
        self.performers.push(performer_id);
        Ok(())
    }

    /// Removes a performer from the track.
    pub fn remove_performer(&mut self, performer_id: MiddsId) -> bool {
        if let Some(pos) = self.performers.iter().position(|&x| x == performer_id) {
            self.performers.remove(pos);
            true
        } else {
            false
        }
    }

    /// Adds a contributor to the track.
    pub fn add_contributor(&mut self, contributor_id: MiddsId) -> Result<(), TrackError> {
        if self.contributors.len() >= 256 {
            return Err(TrackError::TooManyContributors(self.contributors.len() + 1));
        }
        self.contributors.push(contributor_id);
        Ok(())
    }

    /// Removes a contributor from the track.
    pub fn remove_contributor(&mut self, contributor_id: MiddsId) -> bool {
        if let Some(pos) = self.contributors.iter().position(|&x| x == contributor_id) {
            self.contributors.remove(pos);
            true
        } else {
            false
        }
    }

    /// Adds a title alias.
    pub fn add_title_alias(&mut self, alias: String) -> Result<(), TrackError> {
        if self.title_aliases.len() >= 16 {
            return Err(TrackError::TooManyTitleAliases(
                self.title_aliases.len() + 1,
            ));
        }
        self.title_aliases.push(alias);
        Ok(())
    }

    /// Adds a genre.
    pub fn add_genre(&mut self, genre: GenreId) -> Result<(), TrackError> {
        if self.genres.len() >= 5 {
            return Err(TrackError::TooManyGenres(self.genres.len() + 1));
        }
        if !self.genres.contains(&genre) {
            self.genres.push(genre);
        }
        Ok(())
    }

    /// Sets the recording year with validation.
    pub fn set_recording_year(&mut self, year: u16) -> Result<(), TrackError> {
        Self::validate_recording_year(year)?;
        self.recording_year = Some(year);
        Ok(())
    }

    /// Sets the duration with validation.
    pub fn set_duration(&mut self, duration_seconds: u16) -> Result<(), TrackError> {
        Self::validate_duration(duration_seconds)?;
        self.duration = Some(duration_seconds);
        Ok(())
    }

    /// Sets the BPM with validation.
    pub fn set_bpm(&mut self, bpm: u16) -> Result<(), TrackError> {
        Self::validate_bpm(bpm)?;
        self.bpm = Some(bpm);
        Ok(())
    }

    /// Sets the musical key.
    pub fn set_key(&mut self, key: Key) {
        self.key = Some(key);
    }

    /// Sets the recording place with validation.
    pub fn set_recording_place(&mut self, place: String) -> Result<(), TrackError> {
        Self::validate_place(&place)?;
        self.recording_place = Some(place);
        Ok(())
    }

    /// Sets the mixing place with validation.
    pub fn set_mixing_place(&mut self, place: String) -> Result<(), TrackError> {
        Self::validate_place(&place)?;
        self.mixing_place = Some(place);
        Ok(())
    }

    /// Sets the mastering place with validation.
    pub fn set_mastering_place(&mut self, place: String) -> Result<(), TrackError> {
        Self::validate_place(&place)?;
        self.mastering_place = Some(place);
        Ok(())
    }

    /// Returns the track title with aliases if available.
    pub fn full_title(&self) -> String {
        if self.title_aliases.is_empty() {
            self.title.clone()
        } else {
            let aliases = self
                .title_aliases
                .iter()
                .map(|alias| alias.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            format!("{} ({})", self.title, aliases)
        }
    }

    /// Returns a searchable title (lowercase).
    pub fn searchable_title(&self) -> String {
        self.title.to_lowercase()
    }

    /// Returns the duration formatted as MM:SS.
    pub fn formatted_duration(&self) -> Option<String> {
        self.duration.map(|seconds| {
            let minutes = seconds / 60;
            let seconds = seconds % 60;
            format!("{:02}:{:02}", minutes, seconds)
        })
    }

    /// Checks if the track was recorded in a specific year.
    pub fn recorded_in_year(&self, year: u16) -> bool {
        self.recording_year == Some(year)
    }

    /// Returns the age of the recording in years.
    pub fn recording_age_years(&self) -> Option<u16> {
        let current_year = 2025u16; // TODO: use actual current year
        self.recording_year
            .map(|year| current_year.saturating_sub(year))
    }

    /// Checks if the track has a specific genre.
    pub fn has_genre(&self, genre: &GenreId) -> bool {
        self.genres.contains(genre)
    }

    /// Returns the number of contributors (producers + performers + other contributors).
    pub fn total_contributor_count(&self) -> usize {
        self.producers.len() + self.performers.len() + self.contributors.len()
    }

    /// Checks if this is a live version.
    pub fn is_live_version(&self) -> bool {
        matches!(self.version, Some(TrackVersion::Live))
    }

    /// Checks if this is an acoustic version.
    pub fn is_acoustic_version(&self) -> bool {
        matches!(self.version, Some(TrackVersion::Acoustic))
    }

    /// Checks if this is an instrumental version.
    pub fn is_instrumental_version(&self) -> bool {
        matches!(self.version, Some(TrackVersion::Instrumental))
    }

    /// Returns suggested similar versions based on current version.
    pub fn suggested_versions(&self) -> Vec<TrackVersion> {
        match self.version {
            Some(TrackVersion::Original) => vec![
                TrackVersion::Live,
                TrackVersion::Acoustic,
                TrackVersion::Instrumental,
            ],
            Some(TrackVersion::Live) => vec![TrackVersion::Original, TrackVersion::Acoustic],
            Some(TrackVersion::Acoustic) => vec![TrackVersion::Original, TrackVersion::Live],
            _ => vec![
                TrackVersion::Original,
                TrackVersion::Live,
                TrackVersion::Acoustic,
            ],
        }
    }

    /// Validates producers list.
    fn validate_producers(producers: &[MiddsId]) -> Result<(), TrackError> {
        if producers.len() > 64 {
            return Err(TrackError::TooManyProducers(producers.len()));
        }
        Ok(())
    }

    /// Validates performers list.
    fn validate_performers(performers: &[MiddsId]) -> Result<(), TrackError> {
        if performers.len() > 256 {
            return Err(TrackError::TooManyPerformers(performers.len()));
        }
        Ok(())
    }

    /// Validates contributors list.
    fn validate_contributors(contributors: &[MiddsId]) -> Result<(), TrackError> {
        if contributors.len() > 256 {
            return Err(TrackError::TooManyContributors(contributors.len()));
        }
        Ok(())
    }

    /// Validates recording year.
    fn validate_recording_year(year: u16) -> Result<(), TrackError> {
        if !(1800..=2100).contains(&year) {
            return Err(TrackError::InvalidRecordingYear(format!(
                "Recording year must be between 1800 and 2100, got {}",
                year
            )));
        }
        Ok(())
    }

    /// Validates duration.
    fn validate_duration(duration: u16) -> Result<(), TrackError> {
        if duration == 0 {
            return Err(TrackError::InvalidDuration(
                "Duration cannot be zero".to_string(),
            ));
        }
        if duration > 3600 {
            // Max 1 hour
            return Err(TrackError::InvalidDuration(format!(
                "Duration too long (max 1 hour): {} seconds",
                duration
            )));
        }
        Ok(())
    }

    /// Validates BPM.
    fn validate_bpm(bpm: u16) -> Result<(), TrackError> {
        if bpm == 0 {
            return Err(TrackError::InvalidBpm("BPM cannot be zero".to_string()));
        }
        if bpm > 300 {
            return Err(TrackError::InvalidBpm(format!(
                "BPM too high (max 300): {}",
                bpm
            )));
        }
        Ok(())
    }

    /// Validates a place name.
    fn validate_place(place: &str) -> Result<(), TrackError> {
        if place.trim().is_empty() {
            return Err(TrackError::InvalidPlace(
                "Place name cannot be empty".to_string(),
            ));
        }
        if place.len() > 256 {
            return Err(TrackError::InvalidPlace(
                "Place name too long (max 256 chars)".to_string(),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.title, self.isrc)
    }
}

impl fmt::Display for TrackVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrackVersion::Original => write!(f, "Original"),
            TrackVersion::Live => write!(f, "Live"),
            TrackVersion::RadioEdit => write!(f, "Radio Edit"),
            TrackVersion::TvTrack => write!(f, "TV Track"),
            TrackVersion::Single => write!(f, "Single"),
            TrackVersion::Remix => write!(f, "Remix"),
            TrackVersion::Cover => write!(f, "Cover"),
            TrackVersion::Acoustic => write!(f, "Acoustic"),
            TrackVersion::Acapella => write!(f, "Acapella"),
            TrackVersion::Instrumental => write!(f, "Instrumental"),
            TrackVersion::Orchestral => write!(f, "Orchestral"),
            TrackVersion::Extended => write!(f, "Extended"),
            TrackVersion::AlternateTake => write!(f, "Alternate Take"),
            TrackVersion::ReRecorded => write!(f, "Re-Recorded"),
            TrackVersion::Karaoke => write!(f, "Karaoke"),
            TrackVersion::Dance => write!(f, "Dance"),
            TrackVersion::Dub => write!(f, "Dub"),
            TrackVersion::Clean => write!(f, "Clean"),
            TrackVersion::Rehearsal => write!(f, "Rehearsal"),
            TrackVersion::Demo => write!(f, "Demo"),
            TrackVersion::Edit => write!(f, "Edit"),
        }
    }
}
