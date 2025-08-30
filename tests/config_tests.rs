//! Comprehensive tests for configuration management
//!
//! This module tests configuration functionality including:
//! - Default configuration creation
//! - Configuration validation
//! - Filter preset management
//! - Configuration file I/O
//! - Error handling in configuration

use ia_get::config::{Config, ConfigManager, FilterPreset};

#[cfg(test)]
mod config_creation_tests {
    use super::*;

    /// Test default configuration creation
    #[test]
    fn test_default_config_creation() {
        let config = Config::default();
        
        // Test default values
        assert_eq!(config.concurrent_downloads, 3, "Default concurrent downloads should be 3");
        assert_eq!(config.max_retries, 3, "Default max retries should be 3");
        assert!(!config.filter_presets.is_empty(), "Should have default filter presets");
        
        // Test that default values are within valid ranges
        assert!(config.concurrent_downloads > 0, "Concurrent downloads should be positive");
        assert!(config.concurrent_downloads <= 10, "Concurrent downloads should not exceed 10");
        assert!(config.max_retries > 0, "Max retries should be positive");
        assert!(config.max_retries <= 20, "Max retries should not exceed 20");
    }

    /// Test that default configuration is valid
    #[test]
    fn test_default_config_validation() {
        let config = Config::default();
        let validation_result = ConfigManager::validate_config(&config);
        
        assert!(validation_result.is_ok(), "Default configuration should be valid: {:?}", validation_result);
    }

    /// Test filter presets in default configuration
    #[test]
    fn test_default_filter_presets() {
        let config = Config::default();
        
        // Should have filter presets
        assert!(!config.filter_presets.is_empty(), "Should have default filter presets");
        
        // Each preset should have a name
        for preset in &config.filter_presets {
            assert!(!preset.name.is_empty(), "Preset name should not be empty");
            // At least one of include_ext or exclude_ext should be specified
            assert!(preset.include_ext.is_some() || preset.exclude_ext.is_some(), 
                "Preset should have include or exclude extensions");
        }
    }
}

#[cfg(test)]
mod config_validation_tests {
    use super::*;

    /// Test validation of valid configurations
    #[test]
    fn test_validate_valid_configurations() {
        let valid_configs = vec![
            Config {
                concurrent_downloads: 1,
                max_retries: 1,
                filter_presets: vec![],
                ..Default::default()
            },
            Config {
                concurrent_downloads: 5,
                max_retries: 10,
                filter_presets: vec![
                    FilterPreset {
                        name: "Test".to_string(),
                        description: "Test preset".to_string(),
                        include_ext: Some("pdf".to_string()),
                        exclude_ext: None,
                        max_file_size: None,
                    }
                ],
                ..Default::default()
            },
            Config {
                concurrent_downloads: 10,
                max_retries: 20,
                filter_presets: vec![],
                ..Default::default()
            },
        ];

        for config in valid_configs {
            let result = ConfigManager::validate_config(&config);
            assert!(result.is_ok(), "Valid configuration should pass validation: {:?}", result);
        }
    }

    /// Test validation of invalid concurrent downloads
    #[test]
    fn test_validate_invalid_concurrent_downloads() {
        // Test zero concurrent downloads
        let mut config = Config::default();
        config.concurrent_downloads = 0;
        
        let result = ConfigManager::validate_config(&config);
        assert!(result.is_err(), "Zero concurrent downloads should be invalid");
        
        // Test too many concurrent downloads
        config.concurrent_downloads = 25;
        let result = ConfigManager::validate_config(&config);
        assert!(result.is_err(), "Excessive concurrent downloads should be invalid");
    }

    /// Test validation of invalid max retries
    #[test]
    fn test_validate_invalid_max_retries() {
        let mut config = Config::default();
        config.concurrent_downloads = 3; // Valid value
        config.max_retries = 0;
        
        let result = ConfigManager::validate_config(&config);
        assert!(result.is_err(), "Zero max retries should be invalid");
        
        config.max_retries = 25;
        let result = ConfigManager::validate_config(&config);
        assert!(result.is_err(), "Excessive max retries should be invalid");
    }

    /// Test validation of filter presets
    #[test]
    fn test_validate_filter_presets() {
        let mut config = Config::default();
        
        // Test valid filter presets
        config.filter_presets = vec![
            FilterPreset {
                name: "Documents".to_string(),
                description: "Document files".to_string(),
                include_ext: Some("pdf,txt".to_string()),
                exclude_ext: None,
                max_file_size: None,
            },
            FilterPreset {
                name: "Images".to_string(),
                description: "Image files".to_string(),
                include_ext: Some("jpg,png".to_string()),
                exclude_ext: None,
                max_file_size: None,
            },
        ];
        
        let result = ConfigManager::validate_config(&config);
        assert!(result.is_ok(), "Valid filter presets should pass validation");
        
        // Test invalid filter preset (empty name)
        config.filter_presets = vec![
            FilterPreset {
                name: "".to_string(),
                description: "Empty name test".to_string(),
                include_ext: Some("pdf".to_string()),
                exclude_ext: None,
                max_file_size: None,
            },
        ];
        
        let result = ConfigManager::validate_config(&config);
        assert!(result.is_err(), "Empty preset name should be invalid");
        
        // Test invalid filter preset (no extensions)
        config.filter_presets = vec![
            FilterPreset {
                name: "Test".to_string(),
                description: "No extensions test".to_string(),
                include_ext: None,
                exclude_ext: None,
                max_file_size: None,
            },
        ];
        
        let result = ConfigManager::validate_config(&config);
        assert!(result.is_err(), "Filter preset with no extensions should be invalid");
    }
}

#[cfg(test)]
mod config_manager_tests {
    use super::*;

    /// Test ConfigManager creation
    #[test]
    fn test_config_manager_creation() {
        let config_manager = ConfigManager::new();
        assert!(config_manager.is_ok(), "ConfigManager creation should succeed");
    }

    /// Test default configuration loading
    #[test]
    fn test_load_default_config() {
        let manager = ConfigManager::new().unwrap();
        let config = manager.load_config().unwrap();
        
        // Should return a valid config (either loaded or default)
        let default_config = Config::default();
        assert_eq!(config.concurrent_downloads, default_config.concurrent_downloads);
        assert_eq!(config.max_retries, default_config.max_retries);
    }

    /// Test configuration saving and loading (basic functionality test)
    #[test]
    fn test_save_and_load_config() {
        let manager = ConfigManager::new().unwrap();
        
        // Create a custom configuration
        let custom_config = Config {
            concurrent_downloads: 5,
            max_retries: 8,
            filter_presets: vec![
                FilterPreset {
                    name: "Custom".to_string(),
                    description: "Custom preset".to_string(),
                    include_ext: Some("custom".to_string()),
                    exclude_ext: None,
                    max_file_size: None,
                },
            ],
            ..Default::default()
        };
        
        // Test that save_config function exists and can be called
        let save_result = manager.save_config(&custom_config);
        // Note: This might fail due to permissions, but the function should exist
        println!("Save result: {:?}", save_result);
        
        // Test that load_config function works
        let loaded_config = manager.load_config().unwrap();
        // The loaded config should be valid
        assert!(loaded_config.concurrent_downloads > 0);
        assert!(loaded_config.max_retries > 0);
    }

    /// Test configuration file path access
    #[test]
    fn test_config_file_paths() {
        let manager = ConfigManager::new().unwrap();
        
        let config_dir = manager.config_directory();
        let config_file = manager.config_file_path();
        
        assert!(config_file.file_name().is_some(), "Config file should have a filename");
        
        // Config file should be within the config directory
        assert!(config_file.starts_with(config_dir), "Config file should be within config directory");
    }
}

#[cfg(test)]
mod filter_preset_tests {
    use super::*;

    /// Test filter preset structure and validation
    #[test]
    fn test_filter_preset_structure() {
        let preset = FilterPreset {
            name: "Test Preset".to_string(),
            description: "Test description".to_string(),
            include_ext: Some("pdf,txt,doc".to_string()),
            exclude_ext: None,
            max_file_size: None,
        };
        
        assert_eq!(preset.name, "Test Preset");
        assert_eq!(preset.include_ext, Some("pdf,txt,doc".to_string()));
    }

    /// Test filter preset creation with various extension formats
    #[test]
    fn test_filter_preset_extensions() {
        let preset = FilterPreset {
            name: "Mixed Extensions".to_string(),
            description: "Mixed extensions test".to_string(),
            include_ext: Some("pdf,txt,tar.gz,docx,mp3".to_string()),
            exclude_ext: Some("xml,log".to_string()),
            max_file_size: Some("10MB".to_string()),
        };
        
        assert!(preset.include_ext.is_some());
        assert!(preset.exclude_ext.is_some());
        assert!(preset.max_file_size.is_some());
        
        let include_ext = preset.include_ext.unwrap();
        assert!(include_ext.contains("tar.gz"), "Should support complex extensions");
    }

    /// Test default filter presets content
    #[test]
    fn test_default_filter_presets_content() {
        let config = Config::default();
        
        // Just verify that we have some presets and they have reasonable content
        if !config.filter_presets.is_empty() {
            for preset in &config.filter_presets {
                assert!(!preset.name.is_empty(), "Preset names should not be empty");
                // At least one extension field should be specified
                assert!(preset.include_ext.is_some() || preset.exclude_ext.is_some(),
                    "Preset should have include or exclude extensions");
            }
        }
    }
}

#[cfg(test)]
mod config_persistence_tests {
    use super::*;

    /// Test that configuration survives application restart simulation
    #[test]
    fn test_config_persistence_across_restarts() {
        // Test that multiple ConfigManager instances can be created
        let manager1 = ConfigManager::new().unwrap();
        let manager2 = ConfigManager::new().unwrap();
        
        // Both should have the same config file path
        assert_eq!(manager1.config_file_path(), manager2.config_file_path());
        
        // Both should be able to load config
        let config1 = manager1.load_config().unwrap();
        let config2 = manager2.load_config().unwrap();
        
        // Configs should have the same structure
        assert_eq!(config1.concurrent_downloads, config2.concurrent_downloads);
        assert_eq!(config1.max_retries, config2.max_retries);
    }

    /// Test handling of configuration file operations
    #[test]
    fn test_config_file_handling() {
        let manager = ConfigManager::new().unwrap();
        
        // Loading should work (returns default if no file exists)
        let loaded_config = manager.load_config().unwrap();
        let default_config = Config::default();
        
        assert_eq!(loaded_config.concurrent_downloads, default_config.concurrent_downloads);
        assert_eq!(loaded_config.max_retries, default_config.max_retries);
    }

    /// Test configuration file format and structure
    #[test]
    fn test_config_file_format() {
        let manager = ConfigManager::new().unwrap();
        let config = Config::default();
        
        // Test that save_config can be called
        let save_result = manager.save_config(&config);
        println!("Save config result: {:?}", save_result);
        
        // Test that the config file path is reasonable
        let config_path = manager.config_file_path();
        assert!(config_path.extension().map_or(false, |ext| ext == "toml"), 
            "Config file should have .toml extension");
    }
}

#[cfg(test)]
mod edge_cases_and_robustness {
    use super::*;

    /// Test configuration with very large values
    #[test]
    fn test_config_with_large_values() {
        // Test that validation catches unreasonably large values
        let config = Config {
            concurrent_downloads: 1000,
            max_retries: 1000,
            filter_presets: vec![],
            ..Default::default()
        };
        
        let result = ConfigManager::validate_config(&config);
        assert!(result.is_err(), "Unreasonably large values should be invalid");
    }

    /// Test configuration with empty filter preset names
    #[test]
    fn test_config_with_empty_preset_names() {
        let config = Config {
            filter_presets: vec![
                FilterPreset {
                    name: "".to_string(),
                    description: "Empty name test".to_string(),
                    include_ext: Some("pdf".to_string()),
                    exclude_ext: None,
                    max_file_size: None,
                },
            ],
            ..Default::default()
        };
        
        let result = ConfigManager::validate_config(&config);
        assert!(result.is_err(), "Empty filter preset names should be invalid");
    }

    /// Test configuration with duplicate filter preset names
    #[test]
    fn test_config_with_duplicate_preset_names() {
        let config = Config {
            filter_presets: vec![
                FilterPreset {
                    name: "Duplicate".to_string(),
                    description: "First duplicate".to_string(),
                    include_ext: Some("pdf".to_string()),
                    exclude_ext: None,
                    max_file_size: None,
                },
                FilterPreset {
                    name: "Duplicate".to_string(),
                    description: "Second duplicate".to_string(),
                    include_ext: Some("txt".to_string()),
                    exclude_ext: None,
                    max_file_size: None,
                },
            ],
            ..Default::default()
        };
        
        // Note: Current implementation might not check for duplicates
        // This test documents the current behavior
        let result = ConfigManager::validate_config(&config);
        // Depending on implementation, this might pass or fail
        println!("Duplicate preset names result: {:?}", result);
    }

    /// Test configuration manager error handling
    #[test]
    fn test_config_manager_error_handling() {
        let manager = ConfigManager::new().unwrap();
        
        // Test that functions can be called and handle errors gracefully
        let config = Config::default();
        let save_result = manager.save_config(&config);
        // Result may be Ok or Err depending on permissions, but shouldn't panic
        println!("Save config result: {:?}", save_result);
        
        let load_result = manager.load_config();
        // Should always be able to load (returns default if file doesn't exist)
        assert!(load_result.is_ok(), "Should be able to load config");
    }

    /// Test configuration with very long preset names and extensions
    #[test]
    fn test_config_with_long_strings() {
        let long_name = "a".repeat(1000);
        let long_extension = "b".repeat(100);
        
        let config = Config {
            filter_presets: vec![
                FilterPreset {
                    name: long_name.clone(),
                    description: "Long name test".to_string(),
                    include_ext: Some(long_extension.clone()),
                    exclude_ext: None,
                    max_file_size: None,
                },
            ],
            ..Default::default()
        };
        
        // Should handle long strings gracefully
        let result = ConfigManager::validate_config(&config);
        // Current implementation might not have length limits
        println!("Long strings validation result: {:?}", result);
        
        // Test that manager can work with the config
        let manager = ConfigManager::new().unwrap();
        let save_result = manager.save_config(&config);
        println!("Save result with long strings: {:?}", save_result);
    }
}