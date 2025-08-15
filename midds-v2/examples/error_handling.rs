//! Example demonstrating the unified error handling system in MIDDS V2
//!
//! This example shows how the new `MiddsError` type provides a consistent
//! error handling experience across all MIDDS operations.

use allfeat_midds_v2::{
    MiddsError, MiddsResult,
    musical_work::{MusicalWork, Creator, CreatorRole, iswc::Iswc},
    track::{Track, TrackTitle, isrc::Isrc},
    release::{Release, ReleaseTitle, ean::Ean},
    utils::{Date, Country},
    MiddsId,
};

fn main() -> MiddsResult<()> {
    println!("🎵 MIDDS V2 Unified Error Handling Example");
    println!("==========================================\n");

    // Example 1: Creating a musical work with error handling
    println!("📝 Example 1: Creating a Musical Work");
    match create_musical_work() {
        Ok(work) => println!("✅ Successfully created musical work: {:?}", work.title),
        Err(e) => println!("❌ Failed to create musical work: {}", e),
    }
    println!();

    // Example 2: Invalid ISWC format
    println!("📝 Example 2: Invalid ISWC Format");
    match Iswc::new("INVALID-ISWC") {
        Ok(_) => println!("✅ ISWC created successfully"),
        Err(e) => {
            let midds_err: MiddsError = e.into(); // Automatic conversion
            println!("❌ ISWC creation failed: {}", midds_err);
            println!("   Error kind: {:?}", midds_err.kind);
            println!("   Field: {:?}", midds_err.field());
        }
    }
    println!();

    // Example 3: Capacity exceeded error
    println!("📝 Example 3: Capacity Exceeded");
    match create_track_with_too_many_producers() {
        Ok(_) => println!("✅ Track created successfully"),
        Err(e) => {
            println!("❌ Track creation failed: {}", e);
            if e.is_capacity() {
                println!("   This is a capacity error - you can handle it specifically");
                if let (Some(limit), Some(actual)) = (e.context().limit, e.context().actual) {
                    println!("   Limit: {}, Actual: {}", limit, actual);
                }
            }
        }
    }
    println!();

    // Example 4: Validation error with rich context
    println!("📝 Example 4: Rich Validation Error");
    match validate_release_title("") {
        Ok(_) => println!("✅ Title validation passed"),
        Err(e) => {
            println!("❌ Title validation failed: {}", e);
            println!("   Error details:");
            for (key, value) in &e.context().details {
                println!("     {}: {}", key, value);
            }
        }
    }
    println!();

    // Example 5: Error chaining and conversion
    println!("📝 Example 5: Error Chaining");
    match complex_operation() {
        Ok(_) => println!("✅ Complex operation succeeded"),
        Err(e) => {
            println!("❌ Complex operation failed: {}", e);
            println!("   You can handle any MIDDS error the same way!");
        }
    }

    Ok(())
}

/// Example function that creates a musical work
fn create_musical_work() -> MiddsResult<MusicalWork> {
    let iswc = Iswc::new("T-123456789-5")?; // Automatic conversion from IswcError
    
    let work = MusicalWork {
        iswc,
        title: "Bohemian Rhapsody".to_string(),
        creation_year: Some(1975),
        instrumental: Some(false),
        language: Some(allfeat_midds_v2::utils::Language::English),
        bpm: Some(72),
        key: Some(allfeat_midds_v2::utils::Key::Bb),
        work_type: None,
        creators: vec![
            Creator {
                id: 12345,
                role: CreatorRole::Composer,
            }
        ],
        classical_info: None,
    };

    Ok(work)
}

/// Example function that tries to create a track with too many producers
fn create_track_with_too_many_producers() -> MiddsResult<Track> {
    let isrc = Isrc::new("USUM71703861")?;
    let title = TrackTitle::new("My Track")?;
    
    let mut track = Track {
        isrc,
        musical_work: 12345,
        artist: 67890,
        producers: vec![],
        performers: vec![67890],
        contributors: vec![],
        title,
        title_aliases: vec![],
        recording_year: Some(1975),
        genres: vec![],
        version: None,
        duration: Some(355),
        bpm: Some(72),
        key: Some(allfeat_midds_v2::utils::Key::Bb),
        recording_place: None,
        mixing_place: None,
        mastering_place: None,
    };

    // Try to add too many producers (limit is 64)
    for i in 0..70 {
        track.add_producer(i)?; // This will fail and convert to MiddsError
    }

    Ok(track)
}

/// Example validation function that returns a rich error
fn validate_release_title(title: &str) -> MiddsResult<()> {
    if title.is_empty() {
        return Err(MiddsError::empty_field("title")
            .with_context("min_length", "1")
            .with_context("actual_length", "0")
            .with_context("suggestion", "Provide a meaningful title"));
    }

    if title.len() > 256 {
        return Err(MiddsError::invalid_length("title", title.len().to_string(), "max 256 characters")
            .with_context("actual_length", title.len().to_string())
            .with_context("max_length", "256"));
    }

    Ok(())
}

/// Example of a complex operation that can fail in multiple ways
fn complex_operation() -> MiddsResult<String> {
    // This could be any combination of operations that return different error types
    let iswc = Iswc::new("T-987654321-2")?; // IswcError -> MiddsError
    let ean = Ean::new("1234567890123")?;   // EanError -> MiddsError
    let title = ReleaseTitle::new("My Album")?; // ReleaseError -> MiddsError
    
    // All errors are automatically converted to MiddsError
    // and can be handled uniformly by the caller
    
    Ok(format!("Created entities: {}, {}, {}", iswc, ean, title.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversion() {
        // Test that specific errors convert properly to MiddsError
        let iswc_result = Iswc::new("invalid");
        assert!(iswc_result.is_err());
        
        let midds_err: MiddsError = iswc_result.unwrap_err().into();
        assert!(midds_err.is_validation());
        assert_eq!(midds_err.field(), Some("iswc"));
    }

    #[test]
    fn test_error_context() {
        let error = MiddsError::capacity_exceeded("producers", 64, 70);
        assert!(error.is_capacity());
        assert_eq!(error.context().limit, Some(64));
        assert_eq!(error.context().actual, Some(70));
    }

    #[test]
    fn test_error_builder() {
        let error = MiddsError::validation()
            .field("title")
            .value("invalid@title")
            .reason("Contains invalid characters")
            .detail("allowed_chars", "alphanumeric only")
            .build();

        assert_eq!(error.field(), Some("title"));
        assert_eq!(error.context().value, Some("invalid@title".to_string()));
        assert!(!error.context().details.is_empty());
    }
}