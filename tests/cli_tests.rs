//! Comprehensive tests for CLI argument parsing and validation
//!
//! This module tests the command-line interface functionality including:
//! - Argument parsing for different input formats
//! - Option validation and default values
//! - Subcommand handling
//! - Extension parsing and filtering
//! - Interactive mode detection

use ia_get::cli::{Cli, Commands};
use clap::Parser;

#[cfg(test)]
mod cli_parsing_tests {
    use super::*;

    /// Test basic CLI parsing with no arguments (interactive mode)
    #[test]
    fn test_cli_no_args_interactive() {
        let cli = Cli::parse_from(["ia-get"]);
        assert!(cli.command.is_none(), "CLI with no arguments should have no command (interactive mode)");
        assert!(!cli.verbose, "Verbose should default to false");
        assert!(!cli.dry_run, "Dry run should default to false");
        assert_eq!(cli.concurrent_downloads, 3, "Concurrent downloads should default to 3");
    }

    /// Test CLI parsing with URL as positional argument
    #[test]
    fn test_cli_with_url_argument() {
        let cli = Cli::parse_from(["ia-get", "https://archive.org/details/test"]);
        assert_eq!(cli.url, Some("https://archive.org/details/test".to_string()));
        assert!(cli.command.is_none(), "Positional URL should not create a subcommand");
    }

    /// Test CLI parsing with global flags
    #[test]
    fn test_cli_with_global_flags() {
        let cli = Cli::parse_from([
            "ia-get", 
            "https://archive.org/details/test",
            "--verbose",
            "--dry-run",
            "--concurrent-downloads", "5",
            "--max-retries", "2",
            "--include-ext", "pdf,txt",
            "--exclude-ext", "json,log",
            "--max-file-size", "10MB",
            "--compress",
            "--decompress",
            "--decompress-formats", "gzip,bzip2"
        ]);

        assert_eq!(cli.url, Some("https://archive.org/details/test".to_string()));
        assert!(cli.verbose, "Verbose flag should be set");
        assert!(cli.dry_run, "Dry run flag should be set");
        assert_eq!(cli.concurrent_downloads, 5, "Concurrent downloads should be 5");
        assert_eq!(cli.max_retries, 2, "Max retries should be 2");
        assert_eq!(cli.include_ext, Some("pdf,txt".to_string()));
        assert_eq!(cli.exclude_ext, Some("json,log".to_string()));
        assert_eq!(cli.max_file_size, Some("10MB".to_string()));
        assert!(cli.compress, "Compress flag should be set");
        assert!(cli.decompress, "Decompress flag should be set");
        assert_eq!(cli.decompress_formats, vec!["gzip", "bzip2"]);
    }

    /// Test download subcommand parsing
    #[test]
    fn test_download_subcommand() {
        let cli = Cli::parse_from(["ia-get", "download", "https://archive.org/details/test"]);
        match cli.command {
            Some(Commands::Download { url, .. }) => {
                assert_eq!(url, "https://archive.org/details/test");
            }
            _ => panic!("Expected Download command"),
        }
    }

    /// Test download subcommand with options
    #[test]
    fn test_download_subcommand_with_options() {
        let cli = Cli::parse_from([
            "ia-get", 
            "download", 
            "test-item",
            "--output", "/tmp/downloads",
            "--include-ext", "pdf,txt",
            "--exclude-ext", "json",
            "--max-file-size", "5MB",
            "--compress"
        ]);

        match cli.command {
            Some(Commands::Download { 
                url, 
                output, 
                include_ext, 
                exclude_ext, 
                max_file_size, 
                compress 
            }) => {
                assert_eq!(url, "test-item");
                assert_eq!(output, Some("/tmp/downloads".to_string()));
                assert_eq!(include_ext, Some("pdf,txt".to_string()));
                assert_eq!(exclude_ext, Some("json".to_string()));
                assert_eq!(max_file_size, Some("5MB".to_string()));
                assert!(compress, "Compress flag should be set");
            }
            _ => panic!("Expected Download command with options"),
        }
    }

    /// Test identifier-only input (should work with download subcommand)
    #[test]
    fn test_download_with_identifier() {
        let cli = Cli::parse_from(["ia-get", "download", "internetarchive"]);
        match cli.command {
            Some(Commands::Download { url, .. }) => {
                assert_eq!(url, "internetarchive");
            }
            _ => panic!("Expected Download command with identifier"),
        }
    }
}

#[cfg(test)]
mod cli_validation_tests {
    use super::*;

    /// Test CLI validation with valid configuration
    #[test]
    fn test_cli_validation_valid() {
        let cli = Cli::default();
        assert!(cli.validate().is_ok(), "Default CLI configuration should be valid");
    }

    /// Test CLI validation with invalid concurrent downloads
    #[test]
    fn test_cli_validation_invalid_concurrent_downloads() {
        let mut cli = Cli::default();
        
        // Test zero concurrent downloads
        cli.concurrent_downloads = 0;
        assert!(cli.validate().is_err(), "Zero concurrent downloads should be invalid");
        
        // Test too many concurrent downloads
        cli.concurrent_downloads = 15;
        assert!(cli.validate().is_err(), "More than 10 concurrent downloads should be invalid");
    }

    /// Test CLI validation with invalid max retries
    #[test]
    fn test_cli_validation_invalid_max_retries() {
        let mut cli = Cli::default();
        cli.concurrent_downloads = 3; // Valid value
        cli.max_retries = 25;
        assert!(cli.validate().is_err(), "More than 20 max retries should be invalid");
    }

    /// Test extension parsing functionality
    #[test]
    fn test_extension_parsing() {
        let cli = Cli {
            include_ext: Some("pdf,txt, mp3 ".to_string()),
            exclude_ext: Some("JSON,Log".to_string()),
            ..Default::default()
        };

        let include = cli.include_extensions();
        assert_eq!(include, vec!["pdf", "txt", "mp3"], "Include extensions should be parsed and trimmed");

        let exclude = cli.exclude_extensions();
        assert_eq!(exclude, vec!["json", "log"], "Exclude extensions should be parsed and lowercased");
    }

    /// Test interactive mode detection
    #[test]
    fn test_interactive_mode_detection() {
        // Default CLI should be in interactive mode
        let cli = Cli::default();
        assert!(cli.is_interactive_mode(), "Default CLI should be in interactive mode");

        // CLI with URL should not be in interactive mode
        let cli = Cli {
            url: Some("https://archive.org/details/test".to_string()),
            ..Default::default()
        };
        assert!(!cli.is_interactive_mode(), "CLI with URL should not be in interactive mode");

        // CLI with download command should not be in interactive mode
        let cli = Cli {
            command: Some(Commands::Download {
                url: "test".to_string(),
                output: None,
                include_ext: None,
                exclude_ext: None,
                max_file_size: None,
                compress: false,
            }),
            ..Default::default()
        };
        assert!(!cli.is_interactive_mode(), "CLI with download command should not be in interactive mode");
    }

    /// Test output directory resolution
    #[test]
    fn test_output_directory_resolution() {
        // Test global output path
        let cli = Cli {
            output_path: Some("/global/path".to_string()),
            ..Default::default()
        };
        assert_eq!(cli.get_output_dir(), Some("/global/path"));

        // Test download command output path
        let cli = Cli {
            command: Some(Commands::Download {
                url: "test".to_string(),
                output: Some("/download/path".to_string()),
                include_ext: None,
                exclude_ext: None,
                max_file_size: None,
                compress: false,
            }),
            ..Default::default()
        };
        assert_eq!(cli.get_output_dir(), Some("/download/path"));

        // Test no output path specified
        let cli = Cli::default();
        assert_eq!(cli.get_output_dir(), None);
    }
}

#[cfg(test)]
mod edge_cases {
    use super::*;

    /// Test CLI parsing with empty extension strings
    #[test]
    fn test_empty_extension_strings() {
        let cli = Cli {
            include_ext: Some("".to_string()),
            exclude_ext: Some("   ".to_string()),
            ..Default::default()
        };

        let include = cli.include_extensions();
        assert!(include.is_empty(), "Empty include extension string should result in empty vector");

        let exclude = cli.exclude_extensions();
        assert!(exclude.is_empty(), "Whitespace-only exclude extension string should result in empty vector");
    }

    /// Test CLI parsing with special characters in extensions
    #[test]
    fn test_special_characters_in_extensions() {
        let cli = Cli {
            include_ext: Some("pdf,txt.gz,tar.bz2".to_string()),
            ..Default::default()
        };

        let include = cli.include_extensions();
        assert_eq!(include, vec!["pdf", "txt.gz", "tar.bz2"], "Special characters in extensions should be preserved");
    }

    /// Test CLI with very long identifier
    #[test]
    fn test_long_identifier() {
        let long_id = "a".repeat(1000);
        let cli = Cli::parse_from(["ia-get", "download", &long_id]);
        
        match cli.command {
            Some(Commands::Download { url, .. }) => {
                assert_eq!(url, long_id, "Long identifier should be preserved");
            }
            _ => panic!("Expected Download command with long identifier"),
        }
    }
}