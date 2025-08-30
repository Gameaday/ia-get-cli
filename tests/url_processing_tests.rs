//! Comprehensive tests for URL processing and validation
//!
//! This module tests URL processing functionality including:
//! - URL validation for Internet Archive URLs
//! - Identifier extraction from URLs
//! - Metadata and download URL construction
//! - Support for various URL formats

use ia_get::url_processing::*;

#[cfg(test)]
mod url_validation_tests {
    use super::*;

    /// Test validation of valid Internet Archive URLs
    #[test]
    fn test_validate_archive_urls() {
        let valid_urls = vec![
            "https://archive.org/details/example",
            "https://web.archive.org/details/example",
            "http://archive.org/details/test123",
            "https://archive.org/details/my-awesome-book",
            "https://archive.org/details/test_with_underscores",
        ];

        for url in valid_urls {
            let result = validate_and_process_url(url);
            assert!(result.is_ok(), "URL '{}' should be valid", url);
            assert_eq!(result.unwrap(), url, "Valid URL should be returned unchanged");
        }
    }

    /// Test validation of invalid URLs
    #[test]
    fn test_validate_invalid_urls() {
        let invalid_urls = vec![
            "https://example.com/test",
            "https://google.com",
            "http://not-archive.org/details/test",
            "ftp://archive.org/details/test",
            "not-a-url-at-all",
            "",
        ];

        for url in invalid_urls {
            let result = validate_and_process_url(url);
            assert!(result.is_err(), "URL '{}' should be invalid", url);
        }
    }

    /// Test processing of plain identifiers (converted to full URLs)
    #[test]
    fn test_process_identifiers() {
        let test_cases = vec![
            ("example", "https://archive.org/details/example"),
            ("test123", "https://archive.org/details/test123"),
            ("my-book", "https://archive.org/details/my-book"),
            ("test_underscores", "https://archive.org/details/test_underscores"),
        ];

        for (identifier, expected_url) in test_cases {
            let result = validate_and_process_url(identifier);
            assert!(result.is_ok(), "Identifier '{}' should be valid", identifier);
            assert_eq!(result.unwrap(), expected_url, "Identifier should be converted to full URL");
        }
    }

    /// Test URL validation edge cases
    #[test]
    fn test_url_validation_edge_cases() {
        // URLs with extra paths should be valid
        assert!(validate_and_process_url("https://archive.org/details/test/extra/path").is_ok());
        
        // URLs with query parameters should be valid
        assert!(validate_and_process_url("https://archive.org/details/test?param=value").is_ok());
        
        // URLs with fragments should be valid
        assert!(validate_and_process_url("https://archive.org/details/test#fragment").is_ok());
        
        // Case sensitivity test - archive.org domains should work
        assert!(validate_and_process_url("https://ARCHIVE.ORG/details/test").is_ok());
    }
}

#[cfg(test)]
mod url_detection_tests {
    use super::*;

    /// Test detection of Internet Archive URLs
    #[test]
    fn test_is_archive_url() {
        // Valid archive.org URLs
        assert!(is_archive_url("https://archive.org/details/example"));
        assert!(is_archive_url("https://web.archive.org/details/example"));
        assert!(is_archive_url("http://archive.org/details/test"));
        assert!(is_archive_url("https://ARCHIVE.ORG/details/test")); // Case insensitive
        
        // Invalid URLs
        assert!(!is_archive_url("https://example.com"));
        assert!(!is_archive_url("https://not-archive.org/details/test"));
        assert!(!is_archive_url("invalid"));
        assert!(!is_archive_url(""));
        
        // Edge cases
        assert!(!is_archive_url("archive.org")); // Missing protocol
        assert!(!is_archive_url("https://archive.org")); // Missing path
    }

    /// Test archive URL detection with various formats
    #[test]
    fn test_archive_url_format_variations() {
        let valid_formats = vec![
            "https://archive.org/details/test",
            "http://archive.org/details/test",
            "https://web.archive.org/details/test",
            "https://archive.org/details/test/",
            "https://archive.org/details/test/file.pdf",
            "https://archive.org/details/test?tab=about",
        ];

        for url in valid_formats {
            assert!(is_archive_url(url), "URL '{}' should be detected as archive URL", url);
        }
    }
}

#[cfg(test)]
mod identifier_extraction_tests {
    use super::*;

    /// Test extraction of identifiers from valid URLs
    #[test]
    fn test_extract_identifier_from_valid_urls() {
        let test_cases = vec![
            ("https://archive.org/details/example", "example"),
            ("https://web.archive.org/details/test123", "test123"),
            ("http://archive.org/details/my-book", "my-book"),
            ("https://archive.org/details/test_underscores", "test_underscores"),
            ("https://archive.org/details/example/", "example"),
            ("https://archive.org/details/example/file.pdf", "example"),
            ("https://archive.org/details/example?tab=about", "example"),
        ];

        for (url, expected_id) in test_cases {
            let result = extract_identifier_from_url(url);
            assert!(result.is_ok(), "Should extract identifier from URL '{}'", url);
            assert_eq!(result.unwrap(), expected_id, "Extracted identifier should match expected");
        }
    }

    /// Test extraction from invalid URLs
    #[test]
    fn test_extract_identifier_from_invalid_urls() {
        let invalid_urls = vec![
            "https://example.com/test",
            "https://archive.org",
            "https://archive.org/",
            "https://archive.org/details",
            "https://archive.org/details/",
            "not-a-url",
            "",
        ];

        for url in invalid_urls {
            let result = extract_identifier_from_url(url);
            assert!(result.is_err(), "Should fail to extract identifier from invalid URL '{}'", url);
        }
    }

    /// Test extraction with complex identifiers
    #[test]
    fn test_extract_complex_identifiers() {
        let complex_cases = vec![
            ("https://archive.org/details/item-with-dashes", "item-with-dashes"),
            ("https://archive.org/details/item_with_underscores", "item_with_underscores"),
            ("https://archive.org/details/item123numbers", "item123numbers"),
            ("https://archive.org/details/very-long-identifier-name-here", "very-long-identifier-name-here"),
        ];

        for (url, expected_id) in complex_cases {
            let result = extract_identifier_from_url(url);
            assert!(result.is_ok(), "Should extract complex identifier from URL '{}'", url);
            assert_eq!(result.unwrap(), expected_id, "Complex identifier should be extracted correctly");
        }
    }
}

#[cfg(test)]
mod url_construction_tests {
    use super::*;

    /// Test construction of metadata URLs from identifiers
    #[test]
    fn test_construct_metadata_url() {
        let test_cases = vec![
            ("example", "https://archive.org/metadata/example"),
            ("test123", "https://archive.org/metadata/test123"),
            ("my-book", "https://archive.org/metadata/my-book"),
            ("test_underscores", "https://archive.org/metadata/test_underscores"),
        ];

        for (identifier, expected_url) in test_cases {
            let result = construct_metadata_url(identifier);
            assert_eq!(result, expected_url, "Metadata URL should be constructed correctly");
        }
    }

    /// Test construction of download URLs from identifiers
    #[test]
    fn test_construct_download_url() {
        let test_cases = vec![
            ("example", "https://archive.org/download/example"),
            ("test123", "https://archive.org/download/test123"),
            ("my-book", "https://archive.org/download/my-book"),
            ("test_underscores", "https://archive.org/download/test_underscores"),
        ];

        for (identifier, expected_url) in test_cases {
            let result = construct_download_url(identifier);
            assert_eq!(result, expected_url, "Download URL should be constructed correctly");
        }
    }

    /// Test URL construction with special characters
    #[test]
    fn test_url_construction_special_characters() {
        // Test that special characters are preserved in URLs
        let special_identifiers = vec![
            "item-with-dashes",
            "item_with_underscores", 
            "item.with.dots",
            "item123numbers",
        ];

        for identifier in special_identifiers {
            let metadata_url = construct_metadata_url(identifier);
            let download_url = construct_download_url(identifier);
            
            assert!(metadata_url.contains(identifier), "Metadata URL should contain identifier '{}'", identifier);
            assert!(download_url.contains(identifier), "Download URL should contain identifier '{}'", identifier);
            assert!(metadata_url.starts_with("https://archive.org/metadata/"));
            assert!(download_url.starts_with("https://archive.org/download/"));
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Test the complete URL processing workflow
    #[test]
    fn test_complete_url_processing_workflow() {
        let test_url = "https://archive.org/details/example";
        
        // Step 1: Validate the URL
        let processed_url = validate_and_process_url(test_url).unwrap();
        assert_eq!(processed_url, test_url);
        
        // Step 2: Extract identifier
        let identifier = extract_identifier_from_url(&processed_url).unwrap();
        assert_eq!(identifier, "example");
        
        // Step 3: Construct metadata URL
        let metadata_url = construct_metadata_url(&identifier);
        assert_eq!(metadata_url, "https://archive.org/metadata/example");
        
        // Step 4: Construct download URL
        let download_url = construct_download_url(&identifier);
        assert_eq!(download_url, "https://archive.org/download/example");
    }

    /// Test workflow with identifier input
    #[test]
    fn test_identifier_workflow() {
        let identifier = "test123";
        
        // Step 1: Process identifier (should become full URL)
        let processed_url = validate_and_process_url(identifier).unwrap();
        assert_eq!(processed_url, "https://archive.org/details/test123");
        
        // Step 2: Extract identifier back
        let extracted_id = extract_identifier_from_url(&processed_url).unwrap();
        assert_eq!(extracted_id, identifier);
        
        // Step 3: Construct URLs
        let metadata_url = construct_metadata_url(&extracted_id);
        let download_url = construct_download_url(&extracted_id);
        
        assert_eq!(metadata_url, "https://archive.org/metadata/test123");
        assert_eq!(download_url, "https://archive.org/download/test123");
    }

    /// Test error propagation through workflow
    #[test]
    fn test_error_propagation() {
        let invalid_url = "https://example.com/not-archive";
        
        // Should fail at validation step
        assert!(validate_and_process_url(invalid_url).is_err());
        
        // Should also fail at identifier extraction
        assert!(extract_identifier_from_url(invalid_url).is_err());
    }
}

#[cfg(test)]
mod edge_cases_and_robustness {
    use super::*;

    /// Test handling of empty and whitespace inputs
    #[test]
    fn test_empty_inputs() {
        assert!(validate_and_process_url("").is_err());
        assert!(validate_and_process_url("   ").is_err());
        assert!(extract_identifier_from_url("").is_err());
        assert!(!is_archive_url(""));
    }

    /// Test very long identifiers
    #[test]
    fn test_very_long_identifiers() {
        let long_identifier = "a".repeat(1000);
        let long_url = format!("https://archive.org/details/{}", long_identifier);
        
        let result = validate_and_process_url(&long_url);
        assert!(result.is_ok(), "Very long URL should be valid");
        
        let extracted = extract_identifier_from_url(&long_url).unwrap();
        assert_eq!(extracted, long_identifier, "Long identifier should be extracted correctly");
    }

    /// Test URL with unusual but valid characters
    #[test]
    fn test_unusual_valid_characters() {
        let test_cases = vec![
            "item.with.dots",
            "item-with-dashes", 
            "item_with_underscores",
            "item123with456numbers",
            "ItemWithCaps",
        ];

        for identifier in test_cases {
            let url = format!("https://archive.org/details/{}", identifier);
            assert!(validate_and_process_url(&url).is_ok(), "URL with identifier '{}' should be valid", identifier);
            
            let extracted = extract_identifier_from_url(&url).unwrap();
            assert_eq!(extracted, identifier, "Identifier '{}' should be extracted correctly", identifier);
        }
    }

    /// Test case sensitivity handling
    #[test]
    fn test_case_sensitivity() {
        // Domain should be case insensitive
        assert!(is_archive_url("https://ARCHIVE.ORG/details/test"));
        assert!(is_archive_url("https://Archive.Org/details/test"));
        
        // But identifier should preserve case
        let mixed_case_url = "https://archive.org/details/MixedCaseIdentifier";
        let extracted = extract_identifier_from_url(mixed_case_url).unwrap();
        assert_eq!(extracted, "MixedCaseIdentifier", "Identifier case should be preserved");
    }
}