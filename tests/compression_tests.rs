//! Comprehensive tests for compression functionality
//!
//! This module tests compression-related functionality including:
//! - Compression format detection from filenames and metadata
//! - Archive file compression detection and metadata
//! - Automatic decompression configuration and behavior
//! - Real compression/decompression operations
//! - Integration with file metadata structures

use ia_get::compression::{CompressionFormat, decompress_file, should_decompress};
use ia_get::metadata_storage::ArchiveFile;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

#[cfg(test)]
mod compression_format_detection_tests {
    use super::*;

    /// Test compression format detection from various file extensions
    ///
    /// This test verifies that the `CompressionFormat::from_filename` function
    /// correctly identifies compression formats from file extensions.
    #[test]
    fn test_compression_format_detection() {
        let test_cases = vec![
            ("archive.tar.gz", Some(CompressionFormat::TarGz)),
            ("data.bz2", Some(CompressionFormat::Bzip2)),
            ("file.xz", Some(CompressionFormat::Xz)),
            ("document.zip", Some(CompressionFormat::Zip)),
            ("backup.tar.bz2", Some(CompressionFormat::TarBz2)),
            ("compressed.tar.xz", Some(CompressionFormat::TarXz)),
            ("simple.gz", Some(CompressionFormat::Gzip)),
            ("archive.tar", Some(CompressionFormat::Tar)),
            ("plain.txt", None),                    // Not compressed
            ("unknown.xyz", None),                  // Unknown extension
            ("", None),                             // Empty filename
        ];

        for (filename, expected) in test_cases {
            let result = CompressionFormat::from_filename(filename);
            assert_eq!(result, expected, "Format detection failed for '{}'", filename);
        }
    }

    /// Test case-insensitive compression format detection
    ///
    /// This test verifies that compression format detection works regardless
    /// of the case of the file extension.
    #[test]
    fn test_compression_format_detection_case_insensitive() {
        let test_cases = vec![
            ("file.GZ", Some(CompressionFormat::Gzip)),
            ("ARCHIVE.TAR.GZ", Some(CompressionFormat::TarGz)),
            ("data.BZ2", Some(CompressionFormat::Bzip2)),
            ("backup.Zip", Some(CompressionFormat::Zip)),
            ("compressed.XZ", Some(CompressionFormat::Xz)),
        ];

        for (filename, expected) in test_cases {
            let result = CompressionFormat::from_filename(filename);
            assert_eq!(result, expected, "Case-insensitive detection failed for '{}'", filename);
        }
    }
}

#[cfg(test)]
mod archive_file_compression_tests {
    use super::*;

    /// Helper function to create test archive files
    fn create_test_archive_file(name: &str, format: Option<&str>, size: Option<u64>) -> ArchiveFile {
        ArchiveFile {
            name: name.to_string(),
            source: "original".to_string(),
            format: format.map(|s| s.to_string()),
            mtime: None,
            size,
            md5: None,
            crc32: None,
            sha1: None,
            btih: None,
            summation: None,
            original: None,
            rotation: None,
        }
    }

    /// Test compression detection in ArchiveFile with explicit format field
    ///
    /// This test verifies that ArchiveFile correctly detects compression
    /// when the format field explicitly indicates a compressed format.
    #[test]
    fn test_archive_file_compression_detection() {
        // File with explicit format field matching
        let compressed_file = create_test_archive_file("data.tar.gz", Some("gzip"), Some(1024));
        
        assert!(compressed_file.is_compressed(), "File with gzip format should be detected as compressed");
        assert_eq!(compressed_file.get_compression_format(), Some("gzip".to_string()));
        assert_eq!(compressed_file.get_decompressed_name(), "data.tar");

        // File without explicit format (extension-based detection)
        let extension_based = create_test_archive_file("archive.tar.bz2", None, Some(2048));
        
        assert!(extension_based.is_compressed(), "File with .tar.bz2 extension should be detected as compressed");
        assert_eq!(extension_based.get_compression_format(), Some("bzip2".to_string()));
        assert_eq!(extension_based.get_decompressed_name(), "archive.tar");

        // Test non-compressed file
        let plain_file = create_test_archive_file("document.txt", Some("text"), Some(512));
        
        assert!(!plain_file.is_compressed(), "Text file should not be detected as compressed");
        assert_eq!(plain_file.get_compression_format(), None);
        assert_eq!(plain_file.get_decompressed_name(), "document.txt");
    }

    /// Test compression detection with various format strings
    ///
    /// This test verifies that compression detection works with different
    /// ways of expressing compression formats in the metadata.
    #[test]
    fn test_archive_file_format_variations() {
        let format_variations = vec![
            ("file.gz", Some("gzip"), true),
            ("file.gz", Some("GZIP"), true),          // Case insensitive
            ("file.gz", Some("Gzip"), true),          // Mixed case
            ("file.bz2", Some("bzip2"), true),
            ("file.bz2", Some("BZIP2"), true),
            ("file.xz", Some("xz"), true),
            ("file.zip", Some("zip"), true),
            ("file.zip", Some("ZIP"), true),
            ("file.tar", Some("tar"), true),
            ("file.txt", Some("text"), false),        // Not compressed
            ("file.pdf", Some("PDF"), false),         // Not compressed
        ];

        for (filename, format, should_be_compressed) in format_variations {
            let file = create_test_archive_file(filename, format, Some(1024));
            assert_eq!(file.is_compressed(), should_be_compressed, 
                "Compression detection failed for '{}' with format '{:?}'", filename, format);
        }
    }
}

#[cfg(test)]
mod decompression_configuration_tests {
    use super::*;

    /// Test should_decompress configuration with default settings
    ///
    /// This test verifies the default decompression behavior when no specific
    /// formats are configured (should enable common formats automatically).
    #[test]
    fn test_should_decompress_configuration() {
        let gzip_format = CompressionFormat::Gzip;
        let zip_format = CompressionFormat::Zip;
        let tar_gz_format = CompressionFormat::TarGz;

        // Test default behavior (empty config enables common formats)
        assert!(should_decompress(&gzip_format, &[]), "Gzip should be enabled by default");
        assert!(should_decompress(&tar_gz_format, &[]), "Tar.gz should be enabled by default");
        assert!(!should_decompress(&zip_format, &[]), "Zip should not be enabled by default");

        // Test explicit configuration
        let config = vec!["zip".to_string(), "gzip".to_string()];
        assert!(should_decompress(&gzip_format, &config), "Gzip should be enabled when explicitly configured");
        assert!(should_decompress(&zip_format, &config), "Zip should be enabled when explicitly configured");
        assert!(!should_decompress(&tar_gz_format, &config), "Tar.gz should not be enabled when not in explicit config");
    }

    /// Test should_decompress with various format name variations
    ///
    /// This test verifies that format matching is case-insensitive and handles
    /// different ways of specifying compression formats.
    #[test]
    fn test_should_decompress_format_variations() {
        let gzip_format = CompressionFormat::Gzip;
        
        let format_variations = vec![
            vec!["gzip".to_string()],
            vec!["GZIP".to_string()],
            vec!["Gzip".to_string()],
            vec!["gz".to_string()],         // Alternative name
        ];

        for config in format_variations {
            assert!(should_decompress(&gzip_format, &config), 
                "Should decompress gzip with config: {:?}", config);
        }
    }
}

#[cfg(test)]
mod decompressed_name_generation_tests {
    use super::*;

    /// Test decompressed filename generation for various compression formats
    ///
    /// This test verifies that decompressed filenames are generated correctly
    /// by removing the appropriate compression extensions.
    #[test]
    fn test_decompressed_name_generation() {
        let test_cases = vec![
            (CompressionFormat::Gzip, "file.txt.gz", "file.txt"),
            (CompressionFormat::Bzip2, "data.bz2", "data"),
            (CompressionFormat::Xz, "archive.xz", "archive"),
            (CompressionFormat::TarGz, "backup.tar.gz", "backup.tar"),
            (CompressionFormat::TarBz2, "backup.tar.bz2", "backup.tar"),
            (CompressionFormat::TarXz, "backup.tar.xz", "backup.tar"),
            (CompressionFormat::Zip, "folder.zip", "folder"),
            (CompressionFormat::Tar, "archive.tar", "archive"),
        ];

        for (format, input, expected) in test_cases {
            let result = format.get_decompressed_name(input);
            assert_eq!(result, expected, "Decompressed name generation failed for format {:?} with input '{}'", format, input);
        }
    }

    /// Test decompressed name generation edge cases
    ///
    /// This test verifies that decompressed name generation handles edge cases
    /// gracefully, such as files with multiple extensions or unusual names.
    #[test]
    fn test_decompressed_name_edge_cases() {
        let edge_cases = vec![
            (CompressionFormat::Gzip, "file.gz", "file"),              // Simple case
            (CompressionFormat::Gzip, "file.backup.gz", "file.backup"), // Multiple dots
            (CompressionFormat::TarGz, "file.tar.gz", "file.tar"),     // Complex extension
            (CompressionFormat::Zip, "file", "file"),                  // No extension
            (CompressionFormat::Gzip, ".gz", ""),                      // Hidden file
            (CompressionFormat::Bzip2, "archive.data.bz2", "archive.data"), // Multiple extensions
        ];

        for (format, input, expected) in edge_cases {
            let result = format.get_decompressed_name(input);
            assert_eq!(result, expected, "Edge case failed for format {:?} with input '{}'", format, input);
        }
    }
}

#[cfg(test)]
mod real_compression_tests {
    use super::*;

    /// Test actual gzip compression and decompression
    ///
    /// This test performs real compression and decompression operations to verify
    /// that the decompression functionality works correctly with actual compressed data.
    #[test]
    fn test_gzip_decompression() {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Read;

        let temp_dir = TempDir::new().unwrap();
        
        // Create test data
        let test_data = b"Hello, World! This is a test of gzip compression and decompression functionality.";
        
        // Create compressed file
        let compressed_path = temp_dir.path().join("test.txt.gz");
        let compressed_file = File::create(&compressed_path).unwrap();
        let mut encoder = GzEncoder::new(compressed_file, Compression::default());
        encoder.write_all(test_data).unwrap();
        encoder.finish().unwrap();
        
        // Decompress using our function (no progress callback)
        let decompressed_path = temp_dir.path().join("test.txt");
        let result = decompress_file(
            &compressed_path,
            &decompressed_path,
            CompressionFormat::Gzip,
            None,
        );
        
        assert!(result.is_ok(), "Decompression should succeed: {:?}", result);
        
        // Verify decompressed content
        let mut decompressed_file = File::open(&decompressed_path).unwrap();
        let mut decompressed_content = Vec::new();
        decompressed_file.read_to_end(&mut decompressed_content).unwrap();
        
        assert_eq!(decompressed_content, test_data, "Decompressed content should match original");
    }

    /// Test decompression with progress callback
    ///
    /// This test verifies that the decompression function works correctly.
    /// Note: The actual function signature uses Option<&ProgressBar>, not a callback.
    #[test]
    fn test_decompression_with_progress() {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use indicatif::ProgressBar;

        let temp_dir = TempDir::new().unwrap();
        
        // Create test data (larger to test functionality)
        let test_data = vec![42u8; 10000]; // 10KB of data
        
        // Create compressed file
        let compressed_path = temp_dir.path().join("large_test.gz");
        let compressed_file = File::create(&compressed_path).unwrap();
        let mut encoder = GzEncoder::new(compressed_file, Compression::default());
        encoder.write_all(&test_data).unwrap();
        encoder.finish().unwrap();
        
        // Create progress bar
        let progress_bar = ProgressBar::new(100);
        
        // Decompress with progress bar
        let decompressed_path = temp_dir.path().join("large_test");
        let result = decompress_file(
            &compressed_path,
            &decompressed_path,
            CompressionFormat::Gzip,
            Some(&progress_bar),
        );
        
        assert!(result.is_ok(), "Decompression with progress should succeed");
        
        // Verify the file was decompressed
        assert!(decompressed_path.exists(), "Decompressed file should exist");
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Test complete compression workflow with ArchiveFile
    ///
    /// This test verifies the complete workflow from detecting compression
    /// in an ArchiveFile to determining whether it should be decompressed.
    #[test]
    fn test_complete_compression_workflow() {
        // Create a compressed archive file
        let archive_file = ArchiveFile {
            name: "document.pdf.gz".to_string(),
            source: "original".to_string(),
            format: Some("gzip".to_string()),
            mtime: None,
            size: Some(1024 * 1024), // 1MB
            md5: None,
            crc32: None,
            sha1: None,
            btih: None,
            summation: None,
            original: None,
            rotation: None,
        };

        // Step 1: Detect compression
        assert!(archive_file.is_compressed(), "File should be detected as compressed");
        
        // Step 2: Get compression format
        let format_name = archive_file.get_compression_format().unwrap();
        assert_eq!(format_name, "gzip");
        
        // Step 3: Get decompressed name
        let decompressed_name = archive_file.get_decompressed_name();
        assert_eq!(decompressed_name, "document.pdf");
        
        // Step 4: Check if should decompress (default config)
        let compression_format = CompressionFormat::from_filename(&archive_file.name).unwrap();
        assert!(should_decompress(&compression_format, &[]), "Gzip should be decompressed by default");
        
        // Step 5: Check with explicit config
        let config = vec!["gzip".to_string()];
        assert!(should_decompress(&compression_format, &config), "Gzip should be decompressed when explicitly configured");
        
        let no_gzip_config = vec!["zip".to_string()];
        assert!(!should_decompress(&compression_format, &no_gzip_config), "Gzip should not be decompressed when not in config");
    }

    /// Test compression detection priority (format field vs. extension)
    ///
    /// This test verifies that when both format field and file extension
    /// indicate compression, the system handles them correctly.
    #[test]
    fn test_compression_detection_priority() {
        // File where format and extension agree
        let consistent_file = ArchiveFile {
            name: "file.tar.gz".to_string(),
            format: Some("gzip".to_string()),
            source: "original".to_string(),
            mtime: None,
            size: None,
            md5: None,
            crc32: None,
            sha1: None,
            btih: None,
            summation: None,
            original: None,
            rotation: None,
        };
        
        assert!(consistent_file.is_compressed());
        assert_eq!(consistent_file.get_compression_format(), Some("gzip".to_string()));
        
        // File where format indicates compression but extension doesn't
        let format_only_file = ArchiveFile {
            name: "file.dat".to_string(),
            format: Some("gzip".to_string()),
            source: "original".to_string(),
            mtime: None,
            size: None,
            md5: None,
            crc32: None,
            sha1: None,
            btih: None,
            summation: None,
            original: None,
            rotation: None,
        };
        
        assert!(format_only_file.is_compressed());
        assert_eq!(format_only_file.get_compression_format(), Some("gzip".to_string()));
        
        // File where extension indicates compression but format doesn't
        let extension_only_file = ArchiveFile {
            name: "file.tar.gz".to_string(),
            format: Some("binary".to_string()),
            source: "original".to_string(),
            mtime: None,
            size: None,
            md5: None,
            crc32: None,
            sha1: None,
            btih: None,
            summation: None,
            original: None,
            rotation: None,
        };
        
        // Should still detect compression based on extension
        assert!(extension_only_file.is_compressed());
        // Format field takes precedence for the compression format name
        assert_eq!(extension_only_file.get_compression_format(), Some("binary".to_string()));
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    /// Test decompression error handling
    ///
    /// This test verifies that decompression functions handle errors gracefully
    /// when given invalid or corrupted input files.
    #[test]
    fn test_decompression_error_handling() {
        let temp_dir = TempDir::new().unwrap();
        
        // Test with non-existent input file
        let non_existent_input = temp_dir.path().join("does_not_exist.gz");
        let output_path = temp_dir.path().join("output.txt");
        
        let result = decompress_file(
            &non_existent_input,
            &output_path,
            CompressionFormat::Gzip,
            None,
        );
        
        assert!(result.is_err(), "Should fail when input file doesn't exist");
        
        // Test with corrupted gzip file
        let corrupted_input = temp_dir.path().join("corrupted.gz");
        std::fs::write(&corrupted_input, b"not a valid gzip file").unwrap();
        
        let result = decompress_file(
            &corrupted_input,
            &output_path,
            CompressionFormat::Gzip,
            None,
        );
        
        assert!(result.is_err(), "Should fail when input file is corrupted");
    }

    /// Test edge cases in compression format detection
    ///
    /// This test verifies that compression format detection handles edge cases
    /// and unusual inputs without panicking.
    #[test]
    fn test_compression_detection_edge_cases() {
        let edge_cases = vec![
            "",                          // Empty filename
            ".",                         // Just a dot
            "..",                        // Parent directory
            "file.",                     // Ends with dot
            ".hidden",                   // Hidden file
            "file.tar.gz.bak",          // Multiple extensions
            "very-long-filename-with-no-extension-but-very-descriptive-name",
        ];
        
        for filename in edge_cases {
            // Should not panic
            let result = CompressionFormat::from_filename(filename);
            println!("Filename '{}' -> {:?}", filename, result);
        }
    }
}