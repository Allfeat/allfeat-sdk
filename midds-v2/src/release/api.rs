use super::*;
use std::fmt;
use thiserror::Error;

/// Error types for Release operations
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ReleaseError {
    /// Invalid EAN/UPC
    #[error("Invalid EAN/UPC: {0}")]
    InvalidEan(String),
    /// Empty tracks list
    #[error("Release must have at least one track")]
    EmptyTracks,
    /// Invalid title
    #[error("Invalid title: {0}")]
    InvalidTitle(String),
    /// Invalid distributor name
    #[error("Invalid distributor name: {0}")]
    InvalidDistributor(String),
    /// Invalid manufacturer name
    #[error("Invalid manufacturer name: {0}")]
    InvalidManufacturer(String),
    /// Invalid release date
    #[error("Invalid release date")]
    InvalidDate,
    /// Too many tracks
    #[error("Too many tracks (max 1024): {0}")]
    TooManyTracks(usize),
    /// Too many producers
    #[error("Too many producers (max 256): {0}")]
    TooManyProducers(usize),
    /// Too many cover contributors
    #[error("Too many cover contributors (max 64): {0}")]
    TooManyCoverContributors(usize),
    /// Too many title aliases
    #[error("Too many title aliases (max 16): {0}")]
    TooManyTitleAliases(usize),
}

impl Release {
    /// Creates a new Release with required fields.
    ///
    /// # Arguments
    /// * `ean_upc` - The EAN/UPC code
    /// * `artist` - The main artist ID
    /// * `title` - The release title
    /// * `tracks` - List of track IDs
    /// * `date` - Release date
    /// * `country` - Release country
    ///
    /// # Examples
    /// ```
    /// use allfeat_midds_v2::release::*;
    /// use allfeat_midds_v2::release::ean::Ean;
    /// use allfeat_midds_v2::utils::{Date, Country};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let release = Release::new(
    ///     Ean::new("1234567890128")?,
    ///     123, // artist ID
    ///     ReleaseTitle::new("My Album")?,
    ///     vec![456, 789], // track IDs
    ///     Date { year: 2024, month: 6, day: 15 },
    ///     Country::US
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(
        ean_upc: Ean,
        artist: MiddsId,
        title: String,
        tracks: Vec<MiddsId>,
        date: Date,
        country: Country,
    ) -> Result<Self, ReleaseError> {
        Self::validate_tracks(&tracks)?;
        Self::validate_date(&date)?;

        Ok(Self {
            ean_upc,
            artist,
            producers: vec![],
            tracks,
            distributor_name: String::new(),
            manufacturer_name: String::new(),
            cover_contributors: vec![],
            title,
            title_aliases: vec![],
            release_type: ReleaseType::Lp,
            format: ReleaseFormat::Cd,
            packaging: ReleasePackaging::JewelCase,
            status: ReleaseStatus::Official,
            date,
            country,
        })
    }

    /// Builder pattern - sets the EAN/UPC.
    pub fn with_ean(mut self, ean_upc: Ean) -> Self {
        self.ean_upc = ean_upc;
        self
    }

    /// Builder pattern - sets the release type.
    pub fn with_type(mut self, release_type: ReleaseType) -> Self {
        self.release_type = release_type;
        self
    }

    /// Builder pattern - sets the format.
    pub fn with_format(mut self, format: ReleaseFormat) -> Self {
        self.format = format;
        self
    }

    /// Builder pattern - sets the packaging.
    pub fn with_packaging(mut self, packaging: ReleasePackaging) -> Self {
        self.packaging = packaging;
        self
    }

    /// Builder pattern - sets the status.
    pub fn with_status(mut self, status: ReleaseStatus) -> Self {
        self.status = status;
        self
    }

    /// Builder pattern - sets the distributor.
    pub fn with_distributor(
        mut self,
        distributor: impl Into<String>,
    ) -> Result<Self, ReleaseError> {
        let distributor = distributor.into();
        Self::validate_distributor(&distributor)?;
        self.distributor_name = distributor.to_string();
        Ok(self)
    }

    /// Builder pattern - sets the manufacturer.
    pub fn with_manufacturer(
        mut self,
        manufacturer: impl Into<String>,
    ) -> Result<Self, ReleaseError> {
        let manufacturer = manufacturer.into();
        Self::validate_manufacturer(&manufacturer)?;
        self.manufacturer_name = manufacturer.to_string();
        Ok(self)
    }

    /// Adds a producer to the release.
    pub fn add_producer(&mut self, producer_id: MiddsId) -> Result<(), ReleaseError> {
        if self.producers.len() >= 256 {
            return Err(ReleaseError::TooManyProducers(self.producers.len() + 1));
        }
        self.producers.push(producer_id);
        Ok(())
    }

    /// Adds a track to the release.
    pub fn add_track(&mut self, track_id: MiddsId) -> Result<(), ReleaseError> {
        if self.tracks.len() >= 1024 {
            return Err(ReleaseError::TooManyTracks(self.tracks.len() + 1));
        }
        self.tracks.push(track_id);
        Ok(())
    }

    /// Adds a cover contributor.
    pub fn add_cover_contributor(&mut self, contributor: String) -> Result<(), ReleaseError> {
        if self.cover_contributors.len() >= 64 {
            return Err(ReleaseError::TooManyCoverContributors(
                self.cover_contributors.len() + 1,
            ));
        }
        self.cover_contributors.push(contributor);
        Ok(())
    }

    /// Adds a title alias.
    pub fn add_title_alias(&mut self, alias: String) -> Result<(), ReleaseError> {
        if self.title_aliases.len() >= 16 {
            return Err(ReleaseError::TooManyTitleAliases(
                self.title_aliases.len() + 1,
            ));
        }
        self.title_aliases.push(alias);
        Ok(())
    }

    /// Removes a producer from the release.
    pub fn remove_producer(&mut self, producer_id: MiddsId) {
        self.producers.retain(|&id| id != producer_id);
    }

    /// Removes a track from the release.
    pub fn remove_track(&mut self, track_id: MiddsId) {
        self.tracks.retain(|&id| id != track_id);
    }

    /// Gets the total number of tracks.
    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }

    /// Gets the total number of producers.
    pub fn producer_count(&self) -> usize {
        self.producers.len()
    }

    /// Checks if this is a single release.
    pub fn is_single(&self) -> bool {
        matches!(self.release_type, ReleaseType::Single)
    }

    /// Checks if this is an EP.
    pub fn is_ep(&self) -> bool {
        matches!(self.release_type, ReleaseType::Ep)
    }

    /// Checks if this is an LP (album).
    pub fn is_album(&self) -> bool {
        matches!(self.release_type, ReleaseType::Lp | ReleaseType::DoubleLp)
    }

    /// Checks if this is a digital format.
    pub fn is_digital(&self) -> bool {
        // In the current enum, all formats are physical
        // This could be extended with digital formats
        false
    }

    /// Checks if this is a physical format.
    pub fn is_physical(&self) -> bool {
        !self.is_digital()
    }

    /// Gets the suggested release type based on track count.
    pub fn suggested_type_by_track_count(&self) -> ReleaseType {
        match self.tracks.len() {
            1..=2 => ReleaseType::Single,
            3..=7 => ReleaseType::Ep,
            8..=15 => ReleaseType::Lp,
            _ => ReleaseType::DoubleLp,
        }
    }

    /// Returns a formatted title with aliases.
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

    /// Returns the release year.
    pub fn year(&self) -> u16 {
        self.date.year
    }

    /// Checks if the release was published in a given year.
    pub fn released_in_year(&self, year: u16) -> bool {
        self.date.year == year
    }

    /// Checks if the release was published in a given decade.
    pub fn released_in_decade(&self, decade: u16) -> bool {
        let release_decade = (self.date.year / 10) * 10;
        release_decade == decade
    }

    /// Gets a search-friendly title.
    pub fn searchable_title(&self) -> String {
        self.title.to_lowercase()
    }

    /// Returns age of the release in years.
    pub fn age_in_years(&self) -> u16 {
        let current_year = 2025u16; // TODO: use actual current year
        current_year.saturating_sub(self.date.year)
    }

    /// Validates tracks list.
    fn validate_tracks(tracks: &[MiddsId]) -> Result<(), ReleaseError> {
        if tracks.is_empty() {
            return Err(ReleaseError::EmptyTracks);
        }
        if tracks.len() > 1024 {
            return Err(ReleaseError::TooManyTracks(tracks.len()));
        }
        Ok(())
    }

    /// Validates a distributor name.
    fn validate_distributor(name: &str) -> Result<(), ReleaseError> {
        if name.trim().is_empty() {
            return Err(ReleaseError::InvalidDistributor(
                "Distributor name cannot be empty".to_string(),
            ));
        }
        if name.len() > 256 {
            return Err(ReleaseError::InvalidDistributor(
                "Distributor name too long (max 256 chars)".to_string(),
            ));
        }
        Ok(())
    }

    /// Validates a manufacturer name.
    fn validate_manufacturer(name: &str) -> Result<(), ReleaseError> {
        if name.trim().is_empty() {
            return Err(ReleaseError::InvalidManufacturer(
                "Manufacturer name cannot be empty".to_string(),
            ));
        }
        if name.len() > 256 {
            return Err(ReleaseError::InvalidManufacturer(
                "Manufacturer name too long (max 256 chars)".to_string(),
            ));
        }
        Ok(())
    }

    /// Validates a release date.
    fn validate_date(date: &Date) -> Result<(), ReleaseError> {
        if date.year < 1000 || date.year > 2100 {
            return Err(ReleaseError::InvalidDate);
        }
        if !(1..=12).contains(&date.month) {
            return Err(ReleaseError::InvalidDate);
        }
        if !(1..=31).contains(&date.day) {
            return Err(ReleaseError::InvalidDate);
        }
        Ok(())
    }
}

impl fmt::Display for Release {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {} ({})", self.title, self.date.year, self.ean_upc)
    }
}

impl fmt::Display for ReleaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReleaseType::Lp => write!(f, "LP"),
            ReleaseType::DoubleLp => write!(f, "Double LP"),
            ReleaseType::Ep => write!(f, "EP"),
            ReleaseType::Single => write!(f, "Single"),
            ReleaseType::Mixtape => write!(f, "Mixtape"),
        }
    }
}

impl fmt::Display for ReleaseFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReleaseFormat::Cd => write!(f, "CD"),
            ReleaseFormat::DoubleCd => write!(f, "Double CD"),
            ReleaseFormat::Vynil7 => write!(f, "7\" Vinyl"),
            ReleaseFormat::Vinyl10 => write!(f, "10\" Vinyl"),
            ReleaseFormat::Cassette => write!(f, "Cassette"),
            ReleaseFormat::AudioDvd => write!(f, "Audio DVD"),
        }
    }
}

impl fmt::Display for ReleasePackaging {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReleasePackaging::Digipack => write!(f, "Digipack"),
            ReleasePackaging::JewelCase => write!(f, "Jewel Case"),
            ReleasePackaging::SnapCase => write!(f, "Snap Case"),
        }
    }
}

impl fmt::Display for ReleaseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReleaseStatus::Official => write!(f, "Official"),
            ReleaseStatus::Promotional => write!(f, "Promotional"),
            ReleaseStatus::ReRelease => write!(f, "Re-Release"),
            ReleaseStatus::SpecialEdition => write!(f, "Special Edition"),
            ReleaseStatus::Remastered => write!(f, "Remastered"),
            ReleaseStatus::Bootleg => write!(f, "Bootleg"),
            ReleaseStatus::PseudoRelease => write!(f, "Pseudo Release"),
            ReleaseStatus::Withdrawn => write!(f, "Withdrawn"),
            ReleaseStatus::Expunged => write!(f, "Expunged"),
            ReleaseStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}
