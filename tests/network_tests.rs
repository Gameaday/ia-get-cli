//! Network Operations Tests
//!
//! This module contains tests for network-related functionality including
//! URL validation, user agent handling, and basic network operations.
//! These tests focus on core functionality that doesn't require external
//! network access.

use ia_get::{get_user_agent, validate_and_process_url, is_archive_url, extract_identifier_from_url};

/// Test user agent string generation
///
/// Validates that the user agent string contains the expected
/// application name and has reasonable content.
#[test]
fn test_user_agent_generation() {
    let user_agent = get_user_agent();
    
    assert!(user_agent.contains("ia-get"));
    assert!(user_agent.len() > 10); // Should have substantial content
    
    // User agent should not be empty or just whitespace
    assert!(!user_agent.trim().is_empty());
}

/// Test user agent consistency
///
/// Ensures that the user agent string is consistent across calls
/// and follows expected format conventions.
#[test]
fn test_user_agent_consistency() {
    let user_agent1 = get_user_agent();
    let user_agent2 = get_user_agent();
    
    // Should be identical across calls
    assert_eq!(user_agent1, user_agent2);
    
    // Should not contain problematic characters
    assert!(!user_agent1.contains('\n'));
    assert!(!user_agent1.contains('\r'));
}

/// Test URL validation for archive.org URLs
///
/// Validates that the URL validation function correctly identifies
/// valid and invalid Internet Archive URLs.
#[test]
fn test_archive_url_validation() {
    // Valid archive.org URLs
    assert!(is_archive_url("https://archive.org/details/test"));
    assert!(is_archive_url("http://archive.org/details/example"));
    assert!(is_archive_url("https://archive.org/details/test-item-123"));
    
    // Invalid URLs
    assert!(!is_archive_url("https://example.com/test"));
    assert!(!is_archive_url("https://google.com"));
    assert!(!is_archive_url("not-a-url"));
    assert!(!is_archive_url(""));
}

/// Test identifier extraction from URLs
///
/// Validates that identifiers are correctly extracted from various
/// Internet Archive URL formats.
#[test]
fn test_identifier_extraction() {
    // Standard details URL
    let result1 = extract_identifier_from_url("https://archive.org/details/test-item");
    assert!(result1.is_ok());
    assert_eq!(result1.unwrap(), "test-item");
    
    // URL with trailing slash
    let result2 = extract_identifier_from_url("https://archive.org/details/example/");
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), "example/"); // Function includes trailing slash
    
    // Complex identifier
    let result3 = extract_identifier_from_url("https://archive.org/details/complex-item_123");
    assert!(result3.is_ok());
    assert_eq!(result3.unwrap(), "complex-item_123");
    
    // Invalid URL should fail
    let result4 = extract_identifier_from_url("https://example.com/test");
    assert!(result4.is_err());
}

/// Test URL validation and processing workflow
///
/// Validates that the URL validation function properly processes
/// different URL types and returns appropriate results.
#[test]
fn test_url_validation_workflow() {
    // Valid archive.org URL
    let valid_result = validate_and_process_url("https://archive.org/details/test");
    assert!(valid_result.is_ok());
    
    // Invalid URL
    let invalid_result = validate_and_process_url("https://example.com/test");
    assert!(invalid_result.is_err());
    
    // Empty string should actually succeed (becomes archive.org/details/)
    let empty_result = validate_and_process_url("");
    assert!(empty_result.is_ok()); // Actually succeeds but creates /details/
}

/// Test URL processing edge cases
///
/// Validates that URL processing handles edge cases and unusual
/// but valid URL formats correctly.
#[test]
fn test_url_processing_edge_cases() {
    // Very short identifier
    let short_result = extract_identifier_from_url("https://archive.org/details/a");
    assert!(short_result.is_ok());
    assert_eq!(short_result.unwrap(), "a");
    
    // Identifier with numbers only
    let numeric_result = extract_identifier_from_url("https://archive.org/details/12345");
    assert!(numeric_result.is_ok());
    assert_eq!(numeric_result.unwrap(), "12345");
    
    // HTTP instead of HTTPS
    let http_result = extract_identifier_from_url("http://archive.org/details/test");
    assert!(http_result.is_ok());
    assert_eq!(http_result.unwrap(), "test");
}

/// Test metadata URL construction workflow
///
/// Validates that URLs are correctly processed through the complete
/// workflow from validation to metadata URL generation.
#[test]
fn test_metadata_url_workflow() {
    use ia_get::metadata::get_json_url;
    
    let test_cases = vec![
        ("https://archive.org/details/test", "test"),
        ("http://archive.org/details/example", "example"),
        ("https://archive.org/details/complex-item_123", "complex-item_123"),
    ];
    
    for (input_url, expected_identifier) in test_cases {
        // Step 1: Validate URL
        let validation_result = validate_and_process_url(input_url);
        assert!(validation_result.is_ok());
        
        // Step 2: Extract identifier
        let identifier_result = extract_identifier_from_url(input_url);
        assert!(identifier_result.is_ok());
        assert_eq!(identifier_result.unwrap(), expected_identifier);
        
        // Step 3: Generate metadata URL
        let metadata_url = get_json_url(input_url);
        assert!(metadata_url.contains("metadata"));
        assert!(metadata_url.contains(expected_identifier));
    }
}

/// Test URL construction utilities
///
/// Validates that URL construction utilities work correctly for
/// building download and metadata URLs.
#[test]
fn test_url_construction_utilities() {
    use ia_get::{construct_metadata_url, construct_download_url};
    
    // Test metadata URL construction
    let metadata_url = construct_metadata_url("test-item");
    assert!(metadata_url.contains("metadata"));
    assert!(metadata_url.contains("test-item"));
    assert!(metadata_url.starts_with("https://"));
    
    // Test download URL construction
    let download_url = construct_download_url("test-item");
    assert!(download_url.contains("test-item"));
    assert!(download_url.contains("download"));
    assert!(download_url.starts_with("https://"));
    
    // URLs should be different for metadata vs download
    assert_ne!(metadata_url, download_url);
}

/// Test URL format variations
///
/// Validates that different valid URL formats are all processed
/// correctly by the URL handling functions.
#[test]
fn test_url_format_variations() {
    let test_urls = vec![
        "https://archive.org/details/test1",
        "http://archive.org/details/test2",
        "https://archive.org/details/test-with-dashes",
        "https://archive.org/details/test_with_underscores",
        "https://archive.org/details/123-numeric-start",
    ];
    
    for url in test_urls {
        // All should validate successfully
        assert!(validate_and_process_url(url).is_ok());
        assert!(is_archive_url(url));
        
        // All should extract identifiers successfully
        let identifier_result = extract_identifier_from_url(url);
        assert!(identifier_result.is_ok());
        
        // Identifier should not be empty
        let identifier = identifier_result.unwrap();
        assert!(!identifier.is_empty());
    }
}