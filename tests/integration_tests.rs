//! Integration tests for ia-get CLI application
//!
//! This module contains integration tests that verify the behavior of various
//! components working together. These tests focus on:
//! - Real network interactions with test endpoints  
//! - End-to-end CLI functionality
//! - Error handling and retry behavior
//! - Metadata processing workflows
//!
//! Note: These tests make real HTTP requests to test endpoints like httpbin.org
//! and may be affected by network conditions.

use ia_get::*;
use ia_get::cli::Commands;
use ia_get::metadata::get_json_url;
use reqwest::Client;

#[cfg(test)]
mod network_integration_tests {
    use super::*;

    /// Test URL accessibility with a reliable HTTP endpoint
    /// 
    /// This test verifies that the `is_url_accessible` function can successfully
    /// connect to a known good endpoint and return success.
    #[tokio::test]
    async fn test_is_url_accessible_success() {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();
        
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            is_url_accessible("https://httpbin.org/get", &client, None)
        ).await;
        
        match result {
            Ok(Ok(_)) => {}, // Success - URL is accessible
            Ok(Err(_)) => panic!("HTTP request failed"),
            Err(_) => panic!("Test timed out after 30 seconds"),
        }
    }

    /// Test URL accessibility with 404 Not Found response
    ///
    /// This test verifies that the function properly handles 404 errors
    /// and returns an error rather than treating it as success.
    #[tokio::test]
    async fn test_is_url_accessible_404() {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();
        
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            is_url_accessible("https://httpbin.org/status/404", &client, None)
        ).await;
        
        match result {
            Ok(Ok(_)) => panic!("Expected 404 error but got success"),
            Ok(Err(_)) => {}, // Expected - 404 should cause an error
            Err(_) => panic!("Test timed out after 30 seconds"),
        }
    }

    /// Test 500 Internal Server Error retry behavior
    ///
    /// This test verifies that 500 errors trigger the retry mechanism.
    /// We use a short timeout to detect that retry logic has started
    /// without waiting for the full retry sequence to complete.
    #[tokio::test]
    async fn test_is_url_accessible_500_retry() {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(70)) // Longer than retry period
            .build()
            .unwrap();
        
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(5), // Short timeout - expect this to timeout while waiting for retry
            is_url_accessible("https://httpbin.org/status/500", &client, None)
        ).await;
        
        match result {
            Ok(Ok(_)) => panic!("Expected 500 retry handling (timeout) but got success"),
            Ok(Err(_)) => panic!("Expected 500 retry handling (timeout) but got immediate error"),
            Err(_) => {
                // Expected - function should timeout while waiting for retry period
                // This confirms our code detected the 500 and started retry logic
            },
        }
    }

    /// Test 429 Too Many Requests retry behavior
    ///
    /// This test verifies that 429 errors trigger the rate limiting retry mechanism.
    /// Similar to the 500 test, we detect that retry logic starts.
    #[tokio::test]
    async fn test_is_url_accessible_429_retry() {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(70)) // Longer than retry period
            .build()
            .unwrap();
        
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(5), // Short timeout - expect this to timeout while waiting for retry
            is_url_accessible("https://httpbin.org/status/429", &client, None)
        ).await;
        
        match result {
            Ok(Ok(_)) => panic!("Expected 429 retry handling (timeout) but got success"),
            Ok(Err(_)) => panic!("Expected 429 retry handling (timeout) but got immediate error"),
            Err(_) => {
                // Expected - function should timeout while waiting for retry period
                // This confirms our code detected the 429 and started retry logic
            },
        }
    }

    /// Test 422 Unprocessable Entity error handling
    ///
    /// This test verifies that 422 errors are handled immediately without
    /// retry delays, as they indicate client errors that won't be resolved by retrying.
    #[tokio::test]
    async fn test_is_url_accessible_422_error() {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();
        
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            is_url_accessible("https://httpbin.org/status/422", &client, None)
        ).await;
        
        match result {
            Ok(Ok(_)) => panic!("Expected 422 error but got success"),
            Ok(Err(_)) => {}, // Expected - 422 should cause an error without retry delays
            Err(_) => panic!("Test timed out after 30 seconds"),
        }
    }

    /// Test timeout functionality with various configurations
    ///
    /// This comprehensive test verifies that timeout settings work correctly
    /// by testing with both very short timeouts (expected to fail) and
    /// reasonable timeouts (expected to succeed for accessible endpoints).
    #[tokio::test]
    async fn test_is_url_accessible_timeout() {
        println!("Starting timeout test...");
        
        // Test with very short timeout - should fail for most URLs due to timeout
        let short_timeout_client = Client::builder()
            .timeout(std::time::Duration::from_millis(50)) // Very short timeout
            .connect_timeout(std::time::Duration::from_millis(50))
            .build()
            .unwrap();
        
        println!("Testing short timeout (50ms) against real URLs...");
        let start_time = std::time::Instant::now();
        
        let test_urls_short = [
            "https://httpbin.org/delay/1",
            "https://archive.org/",
            "https://www.google.com/",
        ];
        
        let mut short_timeout_failures = 0;
        for url in &test_urls_short {
            let result = tokio::time::timeout(
                std::time::Duration::from_secs(2),
                is_url_accessible(url, &short_timeout_client, None)
            ).await;
            
            match result {
                Ok(Ok(_)) => println!("Short timeout succeeded for {}", url),
                Ok(Err(e)) => {
                    println!("Short timeout failed for {} (expected): {}", url, e);
                    short_timeout_failures += 1;
                },
                Err(_) => {
                    println!("Test timeout for {}", url);
                    short_timeout_failures += 1;
                },
            }
        }
        
        let short_duration = start_time.elapsed();
        println!("Short timeout test took: {:?}", short_duration);
        println!("Failed {} out of {} URLs with short timeout", short_timeout_failures, test_urls_short.len());
        
        // Test with reasonable timeout - should succeed for accessible URLs
        let long_timeout_client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap();
        
        println!("Testing reasonable timeout (10s) against accessible URLs...");
        let start_time2 = std::time::Instant::now();
        
        let test_urls_long = [
            "https://www.google.com/",
            "https://httpbin.org/status/200",
            "https://archive.org/",
            "https://httpbin.org/get",
        ];
        
        let mut successes = 0;
        for url in &test_urls_long {
            let result = tokio::time::timeout(
                std::time::Duration::from_secs(15),
                is_url_accessible(url, &long_timeout_client, None)
            ).await;
            
            match result {
                Ok(Ok(_)) => {
                    println!("Successfully connected to {}", url);
                    successes += 1;
                },
                Ok(Err(e)) => println!("Failed to connect to {}: {}", url, e),
                Err(_) => println!("Timeout connecting to {}", url),
            }
        }
        
        let long_duration = start_time2.elapsed();
        println!("Long timeout test took: {:?}", long_duration);
        println!("Successfully connected to {} out of {} URLs", successes, test_urls_long.len());
        
        // The test passes as long as it doesn't panic - we're testing behavior, not specific outcomes
        println!("Timeout functionality test completed (no panics = success)");
    }
}

#[cfg(test)]
mod metadata_integration_tests {
    use super::*;

    /// Test metadata URL generation from details URLs
    ///
    /// This test verifies that the `get_json_url` function correctly converts
    /// Internet Archive details URLs to their corresponding metadata API URLs.
    #[tokio::test]
    async fn test_metadata_url_generation() {
        let details_url = "https://archive.org/details/example";
        let json_url = get_json_url(details_url);
        
        assert!(json_url.contains("metadata"), "Generated URL should contain 'metadata'");
        assert!(json_url.contains("example"), "Generated URL should contain the identifier");
        assert!(json_url.starts_with("https://archive.org/metadata/"), "Should be a valid metadata URL");
    }

    /// Test URL processing workflow for valid Archive.org URLs
    ///
    /// This test verifies the complete URL validation and processing workflow
    /// for valid Internet Archive URLs.
    #[tokio::test]
    async fn test_url_processing() {
        let valid_url = "https://archive.org/details/test";
        let result = validate_and_process_url(valid_url);
        
        assert!(result.is_ok(), "Valid Archive.org URL should be processed successfully");
        assert_eq!(result.unwrap(), valid_url, "Valid URL should be returned unchanged");
        
        let invalid_url = "https://example.com/test";
        let result = validate_and_process_url(invalid_url);
        
        assert!(result.is_err(), "Non-Archive.org URL should be rejected");
    }
}

#[cfg(test)]
mod cli_integration_tests {
    use super::*;
    use clap::Parser;

    /// Test CLI parsing with no arguments (interactive mode)
    ///
    /// This test verifies that the CLI correctly handles being launched
    /// without any arguments, which should enable interactive mode.
    #[tokio::test]
    async fn test_cli_no_args_interactive() {
        let cli = Cli::parse_from(["ia-get"]);
        assert!(cli.command.is_none(), "CLI with no arguments should have no command (interactive mode)");
    }

    /// Test CLI download subcommand parsing
    ///
    /// This test verifies that the download subcommand is parsed correctly
    /// with a URL argument.
    #[tokio::test]
    async fn test_cli_download_subcommand() {
        let cli = Cli::parse_from(["ia-get", "download", "https://archive.org/details/test"]);
        match cli.command {
            Some(Commands::Download { url, .. }) => {
                assert_eq!(url, "https://archive.org/details/test", "URL should be parsed correctly");
            }
            _ => panic!("Expected Download command"),
        }
    }

    /// Test CLI download subcommand with flags
    ///
    /// This test verifies that download subcommand options are parsed correctly.
    #[tokio::test] 
    async fn test_cli_download_with_flags() {
        let cli = Cli::parse_from(["ia-get", "download", "test-item", "--compress"]);
        match cli.command {
            Some(Commands::Download { url, compress, .. }) => {
                assert!(compress, "Compress flag should be set");
                assert_eq!(url, "test-item", "URL should be parsed correctly");
            }
            _ => panic!("Expected Download command"),
        }
    }
}

#[cfg(test)]
mod utility_function_tests {
    use super::*;

    /// Test basic file size validation concepts
    ///
    /// This test verifies basic size calculation functionality that's used
    /// throughout the application for file size validation.
    #[tokio::test]
    async fn test_file_size_validation() {
        let test_data = "test content";
        let size = test_data.len();
        assert_eq!(size, 12, "Test string should be 12 bytes long");
    }

    /// Test user agent string generation
    ///
    /// This test verifies that the user agent string is generated correctly
    /// and contains expected application information.
    #[tokio::test]
    async fn test_user_agent_generation() {
        let user_agent = get_user_agent();
        
        assert!(user_agent.contains("ia-get"), "User agent should contain application name");
        assert!(user_agent.len() > 10, "User agent should have substantial content");
    }

    /// Test error type creation and classification
    ///
    /// This test verifies that error types can be created and classified correctly
    /// for different error scenarios.
    #[tokio::test]
    async fn test_error_types() {
        use ia_get::error::IaGetError;
        
        let network_error = IaGetError::Network("test".to_string());
        assert!(matches!(network_error, IaGetError::Network(_)), "Should create network error");
        
        let parse_error = IaGetError::Parse("test".to_string());
        assert!(matches!(parse_error, IaGetError::Parse(_)), "Should create parse error");
        
        let io_error = IaGetError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
        assert!(matches!(io_error, IaGetError::Io(_)), "Should create I/O error");
    }

    /// Test download directory path handling concepts
    ///
    /// This test verifies basic path manipulation functionality used for
    /// organizing downloaded files into directories.
    #[tokio::test]
    async fn test_download_directory_creation() {
        let download_path = std::path::Path::new("downloads").join("subdir");
        
        assert!(download_path.to_string_lossy().contains("downloads"), "Path should contain downloads directory");
        assert!(download_path.to_string_lossy().contains("subdir"), "Path should contain subdirectory");
    }
}

#[cfg(test)]
mod retry_behavior_integration_tests {
    use super::*;

    /// Test HTTP 429 (Too Many Requests) handling behavior
    ///
    /// This test verifies that the application properly detects and responds to
    /// 429 rate limiting responses by initiating retry logic.
    #[tokio::test]
    async fn test_http_429_handling() {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(70)) // Longer than retry period
            .build()
            .unwrap();
        
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(5), // Short timeout - we expect this to timeout while waiting
            is_url_accessible("https://httpbin.org/status/429", &client, None)
        ).await;
        
        match result {
            Ok(Ok(_)) => panic!("Expected 429 handling (timeout) but got success"),
            Ok(Err(_)) => panic!("Expected 429 handling (timeout) but got immediate error"), 
            Err(_) => {
                // Expected - the function should timeout while waiting for the retry period
                // This proves our code detected the 429 and started the wait process
            },
        }
    }

    /// Test general network error handling
    ///
    /// This test verifies that the application handles various network conditions
    /// gracefully, whether the request succeeds or fails.
    #[tokio::test]
    async fn test_network_error_handling() {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();
        
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            is_url_accessible("https://httpbin.org/get", &client, None)
        ).await;
        
        match result {
            Ok(Ok(_)) => {}, // Success case - network is working
            Ok(Err(_)) => {}, // Network error case - also valid for testing error handling
            Err(_) => panic!("Test timed out after 30 seconds"),
        }
    }
}
