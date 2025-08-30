//! Comprehensive tests for file filtering and size parsing functionality
//!
//! This module tests the filtering system including:
//! - File size parsing and formatting
//! - Extension-based filtering (include/exclude)
//! - Size-based filtering
//! - Filter integration with CLI commands

use ia_get::filters::*;
use ia_get::cli::Commands;
use ia_get::archive_metadata::JsonFile;

/// Helper function to create test files for all test modules
fn create_test_file(name: &str, size: Option<u64>, format: Option<&str>) -> JsonFile {
    JsonFile {
        name: name.to_string(),
        source: "original".to_string(),
        mtime: None,
        size,
        format: format.map(|s| s.to_string()),
        rotation: None,
        md5: None,
        crc32: None,
        sha1: None,
        btih: None,
        summation: None,
        original: None,
    }
}

#[cfg(test)]
mod size_parsing_tests {
    use super::*;

    /// Test parsing of various size formats
    #[test]
    fn test_parse_size_string_basic() {
        let test_cases = vec![
            ("100", 100),
            ("1024", 1024),
            ("0", 0),
        ];

        for (input, expected) in test_cases {
            let result = parse_size_string(input);
            assert!(result.is_ok(), "Should parse size string '{}'", input);
            assert_eq!(result.unwrap(), expected, "Parsed size should match expected");
        }
    }

    /// Test parsing of sizes with byte units
    #[test]
    fn test_parse_size_string_byte_units() {
        let test_cases = vec![
            ("100B", 100),
            ("1KB", 1024),
            ("2KB", 2048),
            ("1MB", 1024 * 1024),
            ("2MB", 2 * 1024 * 1024),
            ("1GB", 1024 * 1024 * 1024),
            ("2GB", 2 * 1024 * 1024 * 1024),
            ("1TB", 1024_u64.pow(4)),
        ];

        for (input, expected) in test_cases {
            let result = parse_size_string(input);
            assert!(result.is_ok(), "Should parse size string '{}'", input);
            assert_eq!(result.unwrap(), expected, "Parsed size should match expected for '{}'", input);
        }
    }

    /// Test parsing with decimal sizes
    #[test]
    fn test_parse_size_string_decimal() {
        let test_cases = vec![
            ("1.5KB", (1.5 * 1024.0) as u64),
            ("2.5MB", (2.5 * 1024.0 * 1024.0) as u64),
            ("1.25GB", (1.25 * 1024.0 * 1024.0 * 1024.0) as u64),
            ("0.5KB", 512),
        ];

        for (input, expected) in test_cases {
            let result = parse_size_string(input);
            assert!(result.is_ok(), "Should parse decimal size string '{}'", input);
            assert_eq!(result.unwrap(), expected, "Parsed size should match expected for '{}'", input);
        }
    }

    /// Test case insensitive parsing
    #[test]
    fn test_parse_size_string_case_insensitive() {
        let test_cases = vec![
            ("1kb", 1024),
            ("1KB", 1024),
            ("1Kb", 1024),
            ("1kB", 1024),
            ("1mb", 1024 * 1024),
            ("1MB", 1024 * 1024),
            ("1Mb", 1024 * 1024),
            ("1mB", 1024 * 1024),
        ];

        for (input, expected) in test_cases {
            let result = parse_size_string(input);
            assert!(result.is_ok(), "Should parse case-insensitive size string '{}'", input);
            assert_eq!(result.unwrap(), expected, "Parsed size should match expected for '{}'", input);
        }
    }

    /// Test parsing of invalid size strings
    #[test]
    fn test_parse_size_string_invalid() {
        let invalid_inputs = vec![
            "",
            "invalid",
            "1XX",
            "-1KB",
            "1.KB",
            ".5KB",
            "1 KB", // Space not allowed
            "KB1",
            "1KBB",
        ];

        for input in invalid_inputs {
            let result = parse_size_string(input);
            assert!(result.is_err(), "Should fail to parse invalid size string '{}'", input);
        }
    }

    /// Test edge cases in size parsing
    #[test]
    fn test_parse_size_string_edge_cases() {
        // Very large sizes
        assert!(parse_size_string("999TB").is_ok());
        
        // Zero sizes
        assert_eq!(parse_size_string("0B").unwrap(), 0);
        assert_eq!(parse_size_string("0KB").unwrap(), 0);
        
        // Very small decimal
        assert!(parse_size_string("0.001KB").is_ok());
    }
}

#[cfg(test)]
mod size_formatting_tests {
    use super::*;

    /// Test formatting of byte sizes to human-readable strings
    #[test]
    fn test_format_size_basic() {
        let test_cases = vec![
            (0, "0 B"),
            (1, "1 B"),
            (512, "512 B"),
            (1023, "1023 B"),
            (1024, "1.0 KB"),
            (1536, "1.5 KB"),
            (1024 * 1024, "1.0 MB"),
            (1024 * 1024 * 1024, "1.0 GB"),
            (1024_u64.pow(4), "1.0 TB"),
        ];

        for (input, expected) in test_cases {
            let result = format_size(input);
            assert_eq!(result, expected, "Size {} should format to '{}'", input, expected);
        }
    }

    /// Test formatting of complex sizes
    #[test]
    fn test_format_size_complex() {
        let test_cases = vec![
            (1536, "1.5 KB"),               // 1.5 KB
            (2560, "2.5 KB"),               // 2.5 KB
            (1048576 + 524288, "1.5 MB"),   // 1.5 MB
            (3 * 1024 * 1024 * 1024, "3.0 GB"), // 3 GB
        ];

        for (input, expected) in test_cases {
            let result = format_size(input);
            assert_eq!(result, expected, "Size {} should format to '{}'", input, expected);
        }
    }

    /// Test formatting precision
    #[test]
    fn test_format_size_precision() {
        // Test that formatting uses appropriate precision
        assert_eq!(format_size(1024 + 100), "1.1 KB"); // Should round appropriately
        assert_eq!(format_size(1024 + 512), "1.5 KB"); // Exact half
        assert_eq!(format_size(1024 + 1000), "2.0 KB"); // Should round up
    }
}

#[cfg(test)]
mod file_filtering_tests {
    use super::*;

    /// Test basic file filtering with extensions
    #[test]
    fn test_filter_files_by_extension() {
        let files = vec![
            create_test_file("document.pdf", Some(1024), Some("PDF")),
            create_test_file("text.txt", Some(512), Some("Text")),
            create_test_file("image.jpg", Some(2048), Some("JPEG")),
            create_test_file("archive.zip", Some(5000), Some("ZIP")),
            create_test_file("readme.md", Some(256), None),
        ];

        let command = Commands::Download {
            url: "test".to_string(),
            output: None,
            include_ext: Some("pdf,txt".to_string()),
            exclude_ext: None,
            max_file_size: None,
            compress: false,
        };

        let filtered = filter_files(files, &command);
        assert_eq!(filtered.len(), 2, "Should filter to 2 files (pdf and txt)");
        
        let names: Vec<&str> = filtered.iter().map(|f| f.name.as_str()).collect();
        assert!(names.contains(&"document.pdf"));
        assert!(names.contains(&"text.txt"));
    }

    /// Test file filtering with exclusions
    #[test]
    fn test_filter_files_exclude_extensions() {
        let files = vec![
            create_test_file("document.pdf", Some(1024), Some("PDF")),
            create_test_file("image.jpg", Some(2048), Some("JPEG")),
            create_test_file("metadata.json", Some(256), Some("JSON")),
            create_test_file("log.txt", Some(128), Some("Text")),
        ];

        let command = Commands::Download {
            url: "test".to_string(),
            output: None,
            include_ext: None,
            exclude_ext: Some("json,txt".to_string()),
            max_file_size: None,
            compress: false,
        };

        let filtered = filter_files(files, &command);
        assert_eq!(filtered.len(), 2, "Should exclude json and txt files");
        
        let names: Vec<&str> = filtered.iter().map(|f| f.name.as_str()).collect();
        assert!(names.contains(&"document.pdf"));
        assert!(names.contains(&"image.jpg"));
    }

    /// Test file filtering by size
    #[test]
    fn test_filter_files_by_size() {
        let files = vec![
            create_test_file("small.txt", Some(500), Some("Text")),      // 500 bytes
            create_test_file("medium.pdf", Some(1024 * 500), Some("PDF")), // 500 KB
            create_test_file("large.zip", Some(1024 * 1024 * 5), Some("ZIP")), // 5 MB
            create_test_file("huge.tar", Some(1024 * 1024 * 1024), Some("TAR")), // 1 GB
        ];

        let command = Commands::Download {
            url: "test".to_string(),
            output: None,
            include_ext: None,
            exclude_ext: None,
            max_file_size: Some("1MB".to_string()),
            compress: false,
        };

        let filtered = filter_files(files, &command);
        assert_eq!(filtered.len(), 2, "Should filter to files under 1MB");
        
        let names: Vec<&str> = filtered.iter().map(|f| f.name.as_str()).collect();
        assert!(names.contains(&"small.txt"));
        assert!(names.contains(&"medium.pdf"));
    }

    /// Test combined filtering (extension and size)
    #[test]
    fn test_filter_files_combined() {
        let files = vec![
            create_test_file("small.pdf", Some(500), Some("PDF")),        // Matches both
            create_test_file("large.pdf", Some(1024 * 1024 * 5), Some("PDF")), // Too large
            create_test_file("small.txt", Some(256), Some("Text")),       // Wrong extension
            create_test_file("medium.pdf", Some(1024 * 500), Some("PDF")), // Matches both
        ];

        let command = Commands::Download {
            url: "test".to_string(),
            output: None,
            include_ext: Some("pdf".to_string()),
            exclude_ext: None,
            max_file_size: Some("1MB".to_string()),
            compress: false,
        };

        let filtered = filter_files(files, &command);
        assert_eq!(filtered.len(), 2, "Should filter to small and medium PDF files");
        
        let names: Vec<&str> = filtered.iter().map(|f| f.name.as_str()).collect();
        assert!(names.contains(&"small.pdf"));
        assert!(names.contains(&"medium.pdf"));
    }

    /// Test filtering with no criteria (should return all files)
    #[test]
    fn test_filter_files_no_criteria() {
        let files = vec![
            create_test_file("file1.pdf", Some(1024), Some("PDF")),
            create_test_file("file2.txt", Some(512), Some("Text")),
            create_test_file("file3.jpg", Some(2048), Some("JPEG")),
        ];

        let command = Commands::Download {
            url: "test".to_string(),
            output: None,
            include_ext: None,
            exclude_ext: None,
            max_file_size: None,
            compress: false,
        };

        let files_len = files.len();
        let filtered = filter_files(files, &command);
        assert_eq!(filtered.len(), files_len, "Should return all files when no filters applied");
    }

    /// Test filtering with files that have no size information
    #[test]
    fn test_filter_files_no_size_info() {
        let files = vec![
            create_test_file("file1.pdf", None, Some("PDF")),         // No size
            create_test_file("file2.txt", Some(512), Some("Text")),   // Has size
            create_test_file("file3.jpg", None, Some("JPEG")),        // No size
        ];

        let command = Commands::Download {
            url: "test".to_string(),
            output: None,
            include_ext: None,
            exclude_ext: None,
            max_file_size: Some("1KB".to_string()),
            compress: false,
        };

        let filtered = filter_files(files, &command);
        // Files with no size should be included (treated as size 0)
        assert_eq!(filtered.len(), 3, "Files with no size should be included");
    }

    /// Test case-insensitive extension matching
    #[test]
    fn test_filter_files_case_insensitive() {
        let files = vec![
            create_test_file("document.PDF", Some(1024), Some("pdf")),
            create_test_file("text.TXT", Some(512), Some("Text")),
            create_test_file("image.JPG", Some(2048), Some("jpeg")),
        ];

        let command = Commands::Download {
            url: "test".to_string(),
            output: None,
            include_ext: Some("pdf,txt".to_string()),
            exclude_ext: None,
            max_file_size: None,
            compress: false,
        };

        let filtered = filter_files(files, &command);
        assert_eq!(filtered.len(), 2, "Should match extensions case-insensitively");
    }
}

#[cfg(test)]
mod extension_matching_tests {
    use super::*;

    /// Test extension matching against both format field and file extension
    #[test]
    fn test_extension_matching_dual_check() {
        let files = vec![
            // File with format field matching
            create_test_file("document.dat", Some(1024), Some("PDF")),
            // File with extension matching but different format
            create_test_file("text.pdf", Some(512), Some("Binary")),
            // File with both matching
            create_test_file("book.pdf", Some(2048), Some("PDF")),
        ];

        let command = Commands::Download {
            url: "test".to_string(),
            output: None,
            include_ext: Some("pdf".to_string()),
            exclude_ext: None,
            max_file_size: None,
            compress: false,
        };

        let filtered = filter_files(files, &command);
        assert_eq!(filtered.len(), 3, "Should match on either format field or file extension");
    }

    /// Test multiple extension filtering
    #[test]
    fn test_multiple_extension_filtering() {
        let files = vec![
            create_test_file("doc.pdf", Some(1024), Some("PDF")),
            create_test_file("text.txt", Some(512), Some("Text")),
            create_test_file("image.jpg", Some(2048), Some("JPEG")),
            create_test_file("video.mp4", Some(5000), Some("MP4")),
            create_test_file("archive.zip", Some(3000), Some("ZIP")),
        ];

        let command = Commands::Download {
            url: "test".to_string(),
            output: None,
            include_ext: Some("pdf,txt,jpg".to_string()),
            exclude_ext: None,
            max_file_size: None,
            compress: false,
        };

        let filtered = filter_files(files, &command);
        assert_eq!(filtered.len(), 3, "Should match multiple extensions");
        
        let names: Vec<&str> = filtered.iter().map(|f| f.name.as_str()).collect();
        assert!(names.contains(&"doc.pdf"));
        assert!(names.contains(&"text.txt"));
        assert!(names.contains(&"image.jpg"));
    }

    /// Test complex file extensions
    #[test]
    fn test_complex_file_extensions() {
        let files = vec![
            create_test_file("archive.tar.gz", Some(1024), Some("gzip")),
            create_test_file("backup.tar.bz2", Some(2048), Some("bzip2")),
            create_test_file("data.json.gz", Some(512), Some("gzip")),
            create_test_file("simple.txt", Some(256), Some("text")),
        ];

        // Test matching on complex extensions
        let command = Commands::Download {
            url: "test".to_string(),
            output: None,
            include_ext: Some("gz".to_string()),
            exclude_ext: None,
            max_file_size: None,
            compress: false,
        };

        let filtered = filter_files(files, &command);
        assert_eq!(filtered.len(), 2, "Should match files ending with .gz");
    }
}

#[cfg(test)]
mod edge_cases_and_robustness {
    use super::*;

    /// Test empty file lists
    #[test]
    fn test_filter_empty_file_list() {
        let files: Vec<JsonFile> = vec![];
        let command = Commands::Download {
            url: "test".to_string(),
            output: None,
            include_ext: Some("pdf".to_string()),
            exclude_ext: None,
            max_file_size: None,
            compress: false,
        };

        let filtered = filter_files(files, &command);
        assert_eq!(filtered.len(), 0, "Empty file list should remain empty");
    }

    /// Test files with empty names
    #[test]
    fn test_filter_files_empty_names() {
        let files = vec![
            create_test_file("", Some(1024), Some("PDF")),
            create_test_file("normal.pdf", Some(512), Some("PDF")),
        ];

        let command = Commands::Download {
            url: "test".to_string(),
            output: None,
            include_ext: Some("pdf".to_string()),
            exclude_ext: None,
            max_file_size: None,
            compress: false,
        };

        let filtered = filter_files(files, &command);
        // Should handle empty names gracefully
        assert_eq!(filtered.len(), 1, "Should handle empty file names");
        assert_eq!(filtered[0].name, "normal.pdf");
    }

    /// Test very large size filters
    #[test]
    fn test_very_large_size_filters() {
        let files = vec![
            create_test_file("huge.tar", Some(1024 * 1024 * 1024 * 1024), Some("TAR")), // 1TB
        ];

        let command = Commands::Download {
            url: "test".to_string(),
            output: None,
            include_ext: None,
            exclude_ext: None,
            max_file_size: Some("999TB".to_string()),
            compress: false,
        };

        let filtered = filter_files(files, &command);
        assert_eq!(filtered.len(), 1, "Should handle very large size filters");
    }

    /// Test whitespace in extension lists
    #[test]
    fn test_whitespace_in_extension_lists() {
        let files = vec![
            create_test_file("doc.pdf", Some(1024), Some("PDF")),
            create_test_file("text.txt", Some(512), Some("Text")),
        ];

        let command = Commands::Download {
            url: "test".to_string(),
            output: None,
            include_ext: Some(" pdf , txt ".to_string()), // Extra whitespace
            exclude_ext: None,
            max_file_size: None,
            compress: false,
        };

        let filtered = filter_files(files, &command);
        assert_eq!(filtered.len(), 2, "Should handle whitespace in extension lists");
    }
}