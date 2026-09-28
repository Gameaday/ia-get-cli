//! `ia-get config ...` command handlers.

use crate::{
    Result,
    error::IaGetError,
    infrastructure::config::{Config, ConfigManager},
};
use colored::Colorize;
use std::io::{self, Write};

use super::super::types::ConfigAction;
use super::{format_bool, format_option, parse_bool};

/// Valid configuration keys that can be set/unset
const VALID_CONFIG_KEYS: &[&str] = &[
    "default_output_path",
    "concurrent_downloads",
    "max_retries",
    "default_include_ext",
    "default_exclude_ext",
    "default_min_file_size",
    "default_max_file_size",
    "default_resume",
    "default_verbose",
    "default_log_hash_errors",
    "default_dry_run",
    "default_compress",
    "default_decompress",
    "http_timeout",
    "user_agent_override",
];

/// Handle configuration commands
pub async fn handle_config_command(action: ConfigAction) -> Result<()> {
    let manager = ConfigManager::new()?;

    match action {
        ConfigAction::Show => show_config(&manager).await,
        ConfigAction::Set { key, value } => set_config(&manager, &key, &value).await,
        ConfigAction::Unset { key } => unset_config(&manager, &key).await,
        ConfigAction::Location => show_config_location(&manager).await,
        ConfigAction::Reset => reset_config(&manager).await,
        ConfigAction::Validate => validate_config(&manager).await,
    }
}

/// Show current configuration
async fn show_config(manager: &ConfigManager) -> Result<()> {
    println!("{} Current Configuration", "📋".blue().bold());
    println!();

    let config = manager.load_config()?;

    // Show file location
    println!("{} Configuration File:", "📁".cyan());
    if manager.config_exists() {
        println!(
            "  Location: {}",
            manager.config_file_path().display().to_string().green()
        );
    } else {
        println!(
            "  Location: {} {}",
            manager.config_file_path().display().to_string().dimmed(),
            "(file does not exist)".yellow()
        );
    }
    println!();

    // Show download settings
    println!("{} Download Settings:", "⬇️".blue());
    println!(
        "  Default output path: {}",
        format_option(&config.default_output_path)
    );
    println!(
        "  Concurrent downloads: {}",
        config.concurrent_downloads.to_string().cyan()
    );
    println!("  Max retries: {}", config.max_retries.to_string().cyan());
    println!(
        "  HTTP timeout: {} seconds",
        config.http_timeout.to_string().cyan()
    );
    println!();

    // Show filter settings
    println!("{} Filter Settings:", "🔍".green());
    println!(
        "  Default include extensions: {}",
        format_option(&config.default_include_ext)
    );
    println!(
        "  Default exclude extensions: {}",
        format_option(&config.default_exclude_ext)
    );
    println!(
        "  Default min file size: {}",
        format_option(&config.default_min_file_size)
    );
    println!(
        "  Default max file size: {}",
        format_option(&config.default_max_file_size)
    );
    println!();

    // Show behavior settings
    println!("{} Default Behavior:", "⚙️".yellow());
    println!("  Resume downloads: {}", format_bool(config.default_resume));
    println!("  Verbose output: {}", format_bool(config.default_verbose));
    println!(
        "  Log hash errors: {}",
        format_bool(config.default_log_hash_errors)
    );
    println!("  Dry run mode: {}", format_bool(config.default_dry_run));
    println!(
        "  HTTP compression: {}",
        format_bool(config.default_compress)
    );
    println!(
        "  Auto decompress: {}",
        format_bool(config.default_decompress)
    );
    println!(
        "  Decompress formats: {}",
        format_option(&config.default_decompress_formats)
    );
    println!();

    // Show advanced settings
    println!("{} Advanced Settings:", "🔧".purple());
    println!(
        "  User agent override: {}",
        format_option(&config.user_agent_override)
    );
    println!(
        "  Max recent URLs: {}",
        config.max_recent_urls.to_string().cyan()
    );
    println!();

    // Show filter presets
    if !config.filter_presets.is_empty() {
        println!("{} Filter Presets:", "📝".magenta());
        for preset in &config.filter_presets {
            println!(
                "  • {} - {}",
                preset.name.bright_green(),
                preset.description.dimmed()
            );
            if let Some(ref include) = preset.include_ext {
                println!("    Include: {}", include.cyan());
            }
            if let Some(ref exclude) = preset.exclude_ext {
                println!("    Exclude: {}", exclude.red());
            }
            if let Some(ref max_size) = preset.max_file_size {
                println!("    Max size: {}", max_size.yellow());
            }
        }
        println!();
    }

    // Show recent URLs
    if !config.recent_urls.is_empty() {
        println!("{} Recent URLs:", "🕒".bright_blue());
        for (i, url) in config.recent_urls.iter().enumerate() {
            println!("  {}. {}", (i + 1).to_string().dimmed(), url.bright_blue());
        }
        println!();
    }

    println!(
        "{} Use 'ia-get config set <key> <value>' to modify settings",
        "💡".yellow()
    );
    println!(
        "{} Use 'ia-get config unset <key>' to reset to default",
        "💡".yellow()
    );

    Ok(())
}

/// Set a configuration value
async fn set_config(manager: &ConfigManager, key: &str, value: &str) -> Result<()> {
    let mut config = manager.load_config().unwrap_or_default();

    match key {
        "default_output_path" => {
            config.default_output_path = if value.is_empty() {
                None
            } else {
                Some(value.to_string())
            };
        }
        "concurrent_downloads" => {
            let val: usize = value.parse().map_err(|_| {
                IaGetError::Config("concurrent_downloads must be a number".to_string())
            })?;
            if val == 0 || val > 20 {
                return Err(IaGetError::Config(
                    "concurrent_downloads must be between 1 and 20".to_string(),
                ));
            }
            config.concurrent_downloads = val;
        }
        "max_retries" => {
            let val: usize = value
                .parse()
                .map_err(|_| IaGetError::Config("max_retries must be a number".to_string()))?;
            if val > 50 {
                return Err(IaGetError::Config(
                    "max_retries must be 50 or less".to_string(),
                ));
            }
            config.max_retries = val;
        }
        "default_include_ext" => {
            config.default_include_ext = if value.is_empty() {
                None
            } else {
                Some(value.to_string())
            };
        }
        "default_exclude_ext" => {
            config.default_exclude_ext = if value.is_empty() {
                None
            } else {
                Some(value.to_string())
            };
        }
        "default_min_file_size" => {
            config.default_min_file_size = if value.is_empty() {
                None
            } else {
                Some(value.to_string())
            };
        }
        "default_max_file_size" => {
            config.default_max_file_size = if value.is_empty() {
                None
            } else {
                Some(value.to_string())
            };
        }
        "default_resume" => {
            config.default_resume = parse_bool(value)?;
        }
        "default_verbose" => {
            config.default_verbose = parse_bool(value)?;
        }
        "default_log_hash_errors" => {
            config.default_log_hash_errors = parse_bool(value)?;
        }
        "default_dry_run" => {
            config.default_dry_run = parse_bool(value)?;
        }
        "default_compress" => {
            config.default_compress = parse_bool(value)?;
        }
        "default_decompress" => {
            config.default_decompress = parse_bool(value)?;
        }
        "http_timeout" => {
            let val: u64 = value
                .parse()
                .map_err(|_| IaGetError::Config("http_timeout must be a number".to_string()))?;
            if !(5..=600).contains(&val) {
                return Err(IaGetError::Config(
                    "http_timeout must be between 5 and 600 seconds".to_string(),
                ));
            }
            config.http_timeout = val;
        }
        "user_agent_override" => {
            config.user_agent_override = if value.is_empty() {
                None
            } else {
                Some(value.to_string())
            };
        }
        _ => {
            return Err(IaGetError::Config(format!(
                "Unknown configuration key: '{}'.\n\n{} Valid keys:\n  {}\n\n{} Use 'ia-get config show' to see current values",
                key.bright_red(),
                "💡".bright_yellow(),
                VALID_CONFIG_KEYS.join(", ").bright_cyan(),
                "💡".bright_yellow()
            )));
        }
    }

    manager.save_config(&config)?;

    println!(
        "{} Configuration updated: {} = {}",
        "✅".green(),
        key.bright_cyan(),
        value.bright_green()
    );
    println!(
        "{} Configuration saved to: {}",
        "💾".blue(),
        manager.config_file_path().display()
    );

    Ok(())
}

/// Unset a configuration value (reset to default)
async fn unset_config(manager: &ConfigManager, key: &str) -> Result<()> {
    let mut config = manager.load_config().unwrap_or_default();
    let default_config = Config::default();

    match key {
        "default_output_path" => config.default_output_path = default_config.default_output_path,
        "concurrent_downloads" => config.concurrent_downloads = default_config.concurrent_downloads,
        "max_retries" => config.max_retries = default_config.max_retries,
        "default_include_ext" => config.default_include_ext = default_config.default_include_ext,
        "default_exclude_ext" => config.default_exclude_ext = default_config.default_exclude_ext,
        "default_min_file_size" => {
            config.default_min_file_size = default_config.default_min_file_size
        }
        "default_max_file_size" => {
            config.default_max_file_size = default_config.default_max_file_size
        }
        "default_resume" => config.default_resume = default_config.default_resume,
        "default_verbose" => config.default_verbose = default_config.default_verbose,
        "default_log_hash_errors" => {
            config.default_log_hash_errors = default_config.default_log_hash_errors
        }
        "default_dry_run" => config.default_dry_run = default_config.default_dry_run,
        "default_compress" => config.default_compress = default_config.default_compress,
        "default_decompress" => config.default_decompress = default_config.default_decompress,
        "http_timeout" => config.http_timeout = default_config.http_timeout,
        "user_agent_override" => config.user_agent_override = default_config.user_agent_override,
        _ => {
            return Err(IaGetError::Config(format!(
                "Unknown configuration key: '{}'.\n\n{} Valid keys:\n  {}\n\n{} Use 'ia-get config show' to see current values",
                key.bright_red(),
                "💡".bright_yellow(),
                VALID_CONFIG_KEYS.join(", ").bright_cyan(),
                "💡".bright_yellow()
            )));
        }
    }

    manager.save_config(&config)?;

    println!(
        "{} Configuration key '{}' reset to default",
        "✅".green(),
        key.bright_cyan()
    );

    Ok(())
}

/// Show configuration file location
async fn show_config_location(manager: &ConfigManager) -> Result<()> {
    println!("{} Configuration File Location", "📁".blue().bold());
    println!();

    println!(
        "Primary config file: {}",
        manager
            .config_file_path()
            .display()
            .to_string()
            .bright_green()
    );
    println!(
        "Config directory: {}",
        manager.config_directory().display().to_string().cyan()
    );

    if manager.config_exists() {
        println!("Status: {}", "File exists".green());
    } else {
        println!(
            "Status: {} (will be created when settings are saved)",
            "File does not exist".yellow()
        );
    }

    Ok(())
}

/// Reset all configuration to defaults
async fn reset_config(manager: &ConfigManager) -> Result<()> {
    print!(
        "{} This will reset ALL configuration to defaults. Continue? [y/N]: ",
        "⚠️".yellow()
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
        println!("Configuration reset cancelled.");
        return Ok(());
    }

    let default_config = Config::default();
    manager.save_config(&default_config)?;

    println!("{} All configuration reset to defaults", "✅".green());

    Ok(())
}

/// Validate configuration
async fn validate_config(manager: &ConfigManager) -> Result<()> {
    println!("{} Validating Configuration", "🔍".blue().bold());
    println!();

    match manager.load_config() {
        Ok(config) => {
            println!("{} Configuration file is valid", "✅".green());

            // Validate specific values
            let mut warnings = Vec::new();

            if config.concurrent_downloads > 10 {
                warnings.push("concurrent_downloads is quite high, consider reducing for Archive.org compatibility".to_string());
            }

            if config.http_timeout < 10 {
                warnings.push(
                    "http_timeout is quite low, may cause timeouts for large files".to_string(),
                );
            }

            if config.max_retries > 10 {
                warnings.push(
                    "max_retries is quite high, failed downloads may take a long time".to_string(),
                );
            }

            if warnings.is_empty() {
                println!("{} No validation warnings", "✅".green());
            } else {
                println!("{} Validation warnings:", "⚠️".yellow());
                for warning in warnings {
                    println!("  • {}", warning.yellow());
                }
            }
        }
        Err(e) => {
            println!("{} Configuration validation failed: {}", "❌".red(), e);
            return Err(e);
        }
    }

    Ok(())
}
