//! Metadata Processing Tests
//!
//! This module contains tests for Internet Archive metadata processing,
//! including JSON URL generation and metadata structure validation.
//! These tests focus on the metadata handling workflow without requiring
//! external network access.

use ia_get::metadata::get_json_url;

/// Test JSON URL generation from details URL
///
/// Validates that Internet Archive details URLs are correctly
/// converted to their corresponding JSON metadata URLs.
#[test]
fn test_json_url_from_details() {
    let details_url = "https://archive.org/details/internetarchive";
    let expected = "https://archive.org/metadata/internetarchive";
    
    assert_eq!(get_json_url(details_url), expected);
}

/// Test JSON URL generation from identifier
///
/// Validates that bare identifiers are correctly converted
/// to full JSON metadata URLs.
#[test]
fn test_json_url_from_identifier() {
    let identifier = "internetarchive";
    let expected = "https://archive.org/metadata/internetarchive";
    
    assert_eq!(get_json_url(identifier), expected);
}

/// Test JSON URL generation from existing metadata URL
///
/// Validates that existing metadata URLs are passed through unchanged
/// to avoid double-conversion.
#[test]
fn test_json_url_passthrough() {
    let metadata_url = "https://archive.org/metadata/internetarchive";
    
    assert_eq!(get_json_url(metadata_url), metadata_url);
}

/// Test JSON URL generation with various formats
///
/// Validates that different input formats all produce correct
/// metadata URLs.
#[test]
fn test_json_url_format_variations() {
    let test_cases = vec![
        ("internetarchive", "https://archive.org/metadata/internetarchive"),
        ("https://archive.org/details/test-item", "https://archive.org/metadata/test-item"),
        ("http://archive.org/details/example", "http://archive.org/metadata/example"), // Preserves HTTP
        ("https://archive.org/metadata/existing", "https://archive.org/metadata/existing"),
    ];
    
    for (input, expected) in test_cases {
        assert_eq!(get_json_url(input), expected, "Failed for input: {}", input);
    }
}

/// Test URL generation with special characters
///
/// Validates that identifiers with special characters are
/// correctly handled in URL generation.
#[test]
fn test_url_generation_special_characters() {
    let special_cases = vec![
        "test-item-with-dashes",
        "test_item_with_underscores", 
        "TestItemWithCaps",
        "123-numeric-start",
    ];
    
    for identifier in special_cases {
        let url = get_json_url(identifier);
        assert!(url.contains(identifier));
        assert!(url.starts_with("https://archive.org/metadata/"));
    }
}

/// Test metadata URL workflow integration
///
/// Validates that metadata URL generation integrates properly
/// with other URL processing functions.
#[test]
fn test_metadata_url_workflow_integration() {
    use ia_get::{extract_identifier_from_url, validate_and_process_url};
    
    let test_urls = vec![
        "https://archive.org/details/workflow-test1",
        "http://archive.org/details/workflow-test2",
        "https://archive.org/details/complex-workflow_test",
    ];
    
    for url in test_urls {
        // Step 1: Validate URL
        assert!(validate_and_process_url(url).is_ok());
        
        // Step 2: Extract identifier
        let identifier = extract_identifier_from_url(url).unwrap();
        
        // Step 3: Generate metadata URL
        let metadata_url = get_json_url(url);
        
        // Verify metadata URL contains identifier
        assert!(metadata_url.contains(&identifier));
        assert!(metadata_url.contains("metadata"));
        assert!(!metadata_url.contains("details"));
    }
}

/// Test metadata URL consistency
///
/// Validates that metadata URL generation is consistent
/// regardless of input format.
#[test]
fn test_metadata_url_consistency() {
    let identifier = "consistency-test";
    
    // Different input formats for the same identifier
    let details_url = format!("https://archive.org/details/{}", identifier);
    let http_details_url = format!("http://archive.org/details/{}", identifier);
    let metadata_url = format!("https://archive.org/metadata/{}", identifier);
    
    // Since the function preserves the scheme from the input, we need different expected results
    let base_url = format!("https://archive.org/metadata/{}", identifier);
    let http_metadata_url = format!("http://archive.org/metadata/{}", identifier);
    
    // Test cases with their expected outputs
    let test_cases = vec![
        (identifier, &base_url),
        (&details_url, &base_url), // HTTPS details -> HTTPS metadata
        (&http_details_url, &http_metadata_url), // HTTP preserved
        (&metadata_url, &base_url), // Already metadata URL
    ];
    
    for (input, expected) in test_cases {
        let result = get_json_url(input);
        assert_eq!(result, *expected, "Inconsistent result for input: {}", input);
    }
}

/// Test edge cases in URL generation
///
/// Validates that URL generation handles edge cases gracefully
/// without panicking or producing invalid URLs.
#[test]
fn test_url_generation_edge_cases() {
    // Very short identifier
    let short_url = get_json_url("a");
    assert_eq!(short_url, "https://archive.org/metadata/a");
    
    // Numeric identifier
    let numeric_url = get_json_url("123");
    assert_eq!(numeric_url, "https://archive.org/metadata/123");
    
    // Empty string (should still produce valid URL structure)
    let empty_url = get_json_url("");
    assert!(empty_url.contains("metadata"));
    
    // URL with trailing slash (preserves the trailing slash)
    let trailing_slash = get_json_url("https://archive.org/details/test/");
    assert_eq!(trailing_slash, "https://archive.org/metadata/test/"); // Preserves trailing slash
}

/// Test URL generation performance
///
/// Validates that URL generation performs efficiently even
/// with many calls.
#[test]
fn test_url_generation_performance() {
    let start = std::time::Instant::now();
    
    // Generate many URLs
    for i in 0..1000 {
        let identifier = format!("test-item-{}", i);
        let _url = get_json_url(&identifier);
    }
    
    let duration = start.elapsed();
    
    // Should complete quickly (under 10ms for 1000 URLs)
    assert!(duration.as_millis() < 10, "URL generation took too long: {:?}", duration);
}

/// Test URL format validation
///
/// Validates that generated URLs follow the expected format
/// and structure consistently.
#[test]
fn test_generated_url_format_validation() {
    let test_identifiers = vec![
        "simple",
        "test-with-dashes",
        "test_with_underscores",
        "123numeric",
        "MixedCase",
    ];
    
    for identifier in test_identifiers {
        let url = get_json_url(identifier);
        
        // Validate URL structure
        assert!(url.starts_with("https://"));
        assert!(url.contains("archive.org"));
        assert!(url.contains("metadata"));
        assert!(url.contains(identifier));
        
        // Should be a valid URL format
        assert!(!url.contains("//metadata")); // No double slashes
        assert!(!url.ends_with("/")); // No trailing slash in metadata URLs
        
        // Should not contain details in metadata URLs
        assert!(!url.contains("details"));
    }
}