//! Command Line Interface Tests
//!
//! This module contains comprehensive tests for the CLI parsing, validation,
//! and configuration processing. It ensures that the command-line interface
//! correctly handles all user input combinations and provides appropriate
//! error messages for invalid configurations.
//!
//! ## Test Coverage
//!
//! - Argument parsing and validation
//! - Interactive mode detection
//! - Extension filtering configuration
//! - File size limit parsing
//! - Concurrent download limits
//! - Subcommand handling
//! - Default value behavior

use ia_get::cli::{Cli, Commands};
use clap::Parser;

/// Test basic CLI argument parsing with URL input
///
/// Validates that the CLI correctly parses a simple URL argument
/// and sets the expected defaults for other options.
#[test]
fn test_cli_basic_url_parsing() {
    let cli = Cli::parse_from(["ia-get", "https://archive.org/details/test"]);
    
    assert_eq!(cli.url, Some("https://archive.org/details/test".to_string()));
    assert_eq!(cli.concurrent_downloads, 3); // Default value
    assert!(!cli.verbose);
    assert!(!cli.dry_run);
    assert!(cli.command.is_none());
}

/// Test CLI parsing with verbose and dry-run flags
///
/// Ensures that boolean flags are correctly parsed and stored.
#[test]
fn test_cli_verbose_and_dry_run_flags() {
    let cli = Cli::parse_from(["ia-get", "--verbose", "--dry-run", "test-item"]);
    
    assert!(cli.verbose);
    assert!(cli.dry_run);
    assert_eq!(cli.url, Some("test-item".to_string()));
}

/// Test CLI parsing with numeric arguments
///
/// Validates that numeric arguments like concurrent downloads and
/// max retries are correctly parsed and validated.
#[test]
fn test_cli_numeric_arguments() {
    let cli = Cli::parse_from([
        "ia-get",
        "--concurrent-downloads", "8",
        "--max-retries", "5",
        "test-url"
    ]);
    
    assert_eq!(cli.concurrent_downloads, 8);
    assert_eq!(cli.max_retries, 5);
    assert_eq!(cli.url, Some("test-url".to_string()));
}

/// Test CLI validation with valid configuration
///
/// Ensures that valid CLI configurations pass validation checks.
#[test]
fn test_cli_validation_success() {
    let cli = Cli {
        concurrent_downloads: 5,
        max_retries: 10,
        ..Default::default()
    };
    
    assert!(cli.validate().is_ok());
}

/// Test CLI validation with invalid concurrent downloads
///
/// Validates that the CLI properly rejects invalid concurrent download values.
#[test]
fn test_cli_validation_invalid_concurrent_downloads() {
    let mut cli = Cli::default();
    
    // Test zero concurrent downloads
    cli.concurrent_downloads = 0;
    assert!(cli.validate().is_err());
    
    // Test excessive concurrent downloads
    cli.concurrent_downloads = 15;
    assert!(cli.validate().is_err());
}

/// Test CLI validation with invalid max retries
///
/// Ensures that excessive retry values are rejected.
#[test]
fn test_cli_validation_invalid_max_retries() {
    let mut cli = Cli::default();
    cli.max_retries = 25; // Too many retries
    
    assert!(cli.validate().is_err());
}

/// Test extension parsing functionality
///
/// Validates that comma-separated extension lists are correctly
/// parsed and normalized to lowercase.
#[test]
fn test_extension_parsing() {
    let cli = Cli {
        include_ext: Some("PDF,TXT, mp3 ".to_string()),
        exclude_ext: Some("XML,Log".to_string()),
        ..Default::default()
    };

    let include = cli.include_extensions();
    assert_eq!(include, vec!["pdf", "txt", "mp3"]);

    let exclude = cli.exclude_extensions();
    assert_eq!(exclude, vec!["xml", "log"]);
}

/// Test empty extension handling
///
/// Ensures that empty or missing extension configurations
/// return empty vectors.
#[test]
fn test_empty_extension_handling() {
    let cli = Cli::default();
    
    assert!(cli.include_extensions().is_empty());
    assert!(cli.exclude_extensions().is_empty());
}

/// Test interactive mode detection
///
/// Validates that interactive mode is correctly detected when
/// no URL or command is provided.
#[test]
fn test_interactive_mode_detection() {
    // No URL or command - should be interactive
    let cli = Cli::default();
    assert!(cli.is_interactive_mode());

    // With URL - should not be interactive
    let cli = Cli {
        url: Some("https://archive.org/details/test".to_string()),
        ..Default::default()
    };
    assert!(!cli.is_interactive_mode());

    // With command - should not be interactive
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
    assert!(!cli.is_interactive_mode());
}

/// Test download subcommand parsing
///
/// Validates that the download subcommand is correctly parsed
/// with its arguments.
#[test]
fn test_download_subcommand_parsing() {
    let cli = Cli::parse_from(["ia-get", "download", "https://archive.org/details/test"]);
    
    match cli.command {
        Some(Commands::Download { url, .. }) => {
            assert_eq!(url, "https://archive.org/details/test");
        }
        _ => panic!("Expected Download command"),
    }
}

/// Test download subcommand with flags
///
/// Ensures that download subcommand flags are correctly parsed.
#[test]
fn test_download_subcommand_with_flags() {
    let cli = Cli::parse_from([
        "ia-get",
        "download",
        "--compress",
        "--output", "downloads",
        "--max-file-size", "100MB",
        "test-item"
    ]);
    
    match cli.command {
        Some(Commands::Download { 
            url, 
            compress, 
            output, 
            max_file_size,
            .. 
        }) => {
            assert_eq!(url, "test-item");
            assert!(compress);
            assert_eq!(output, Some("downloads".to_string()));
            assert_eq!(max_file_size, Some("100MB".to_string()));
        }
        _ => panic!("Expected Download command"),
    }
}

/// Test URL extraction from CLI arguments
///
/// Validates that URLs can be extracted from either direct
/// arguments or subcommands.
#[test]
fn test_url_extraction() {
    // URL from direct argument
    let cli = Cli {
        url: Some("direct-url".to_string()),
        ..Default::default()
    };
    assert_eq!(cli.get_url(), Some("direct-url"));

    // URL from download subcommand
    let cli = Cli {
        command: Some(Commands::Download {
            url: "subcommand-url".to_string(),
            output: None,
            include_ext: None,
            exclude_ext: None,
            max_file_size: None,
            compress: false,
        }),
        ..Default::default()
    };
    assert_eq!(cli.get_url(), Some("subcommand-url"));

    // No URL
    let cli = Cli::default();
    assert_eq!(cli.get_url(), None);
}

/// Test output directory extraction
///
/// Validates that output directories can be extracted from either
/// main arguments or subcommands, with proper fallback logic.
#[test]
fn test_output_directory_extraction() {
    // Output from main argument
    let cli = Cli {
        output_path: Some("main-output".to_string()),
        ..Default::default()
    };
    assert_eq!(cli.get_output_dir(), Some("main-output"));

    // Output from download subcommand
    let cli = Cli {
        command: Some(Commands::Download {
            url: "test".to_string(),
            output: Some("sub-output".to_string()),
            include_ext: None,
            exclude_ext: None,
            max_file_size: None,
            compress: false,
        }),
        ..Default::default()
    };
    assert_eq!(cli.get_output_dir(), Some("sub-output"));

    // No output directory specified
    let cli = Cli::default();
    assert_eq!(cli.get_output_dir(), None);
}

/// Test file size parsing
///
/// Validates that file size strings are correctly parsed into bytes.
#[test]
fn test_file_size_parsing() {
    let cli = Cli {
        max_file_size: Some("100MB".to_string()),
        ..Default::default()
    };
    
    let size = cli.max_file_size_bytes();
    assert!(size.is_some());
    assert_eq!(size.unwrap(), 100 * 1024 * 1024); // 100MB in bytes
}

/// Test no-argument interactive mode
///
/// Ensures that CLI parsing with no arguments sets up interactive mode.
#[test]
fn test_no_args_interactive() {
    let cli = Cli::parse_from(["ia-get"]);
    assert!(cli.command.is_none());
    assert!(cli.url.is_none());
    assert!(cli.is_interactive_mode());
}

/// Test comprehensive CLI argument parsing
///
/// Tests a complex combination of arguments to ensure all options
/// work together correctly.
#[test]
fn test_comprehensive_cli_parsing() {
    let args = vec![
        "ia-get",
        "https://archive.org/details/test",
        "--verbose",
        "--concurrent-downloads", "5",
        "--include-ext", "pdf,txt",
        "--exclude-ext", "xml",
        "--max-file-size", "500MB",
        "--compress",
        "--decompress",
        "--output-path", "my-downloads"  // Use --output-path instead of --output
    ];

    let cli = Cli::try_parse_from(args).unwrap();
    
    assert_eq!(cli.url, Some("https://archive.org/details/test".to_string()));
    assert!(cli.verbose);
    assert_eq!(cli.concurrent_downloads, 5);
    assert_eq!(cli.include_ext, Some("pdf,txt".to_string()));
    assert_eq!(cli.exclude_ext, Some("xml".to_string()));
    assert_eq!(cli.max_file_size, Some("500MB".to_string()));
    assert!(cli.compress);
    assert!(cli.decompress);
    assert_eq!(cli.output_path, Some("my-downloads".to_string()));
}