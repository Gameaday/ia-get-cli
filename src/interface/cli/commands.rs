//! CLI command handlers for configuration and history management

use crate::{
    Result,
    core::archive::AdvancedMetadataProcessor,
    error::IaGetError,
    infrastructure::{
        api::{EnhancedArchiveApiClient, get_archive_servers},
        config::{Config, ConfigManager},
        persistence::download_history::{DownloadHistory, TaskStatus, get_default_history_db_path},
    },
    utilities::common::{
        HTTP_TIMEOUT, MAX_CONCURRENT_CONNECTIONS, MIN_REQUEST_DELAY_MS, get_user_agent,
    },
    utilities::filters::format_size,
};
use anyhow::Context;
use colored::Colorize;
use std::io::{self, Write};

use super::types::{ConfigAction, HistoryAction};

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

/// Handle history commands
pub async fn handle_history_command(action: HistoryAction) -> Result<()> {
    let history_path = get_default_history_db_path()?;

    match action {
        HistoryAction::Show {
            limit,
            status,
            detailed,
        } => show_history(&history_path, limit, status.as_deref(), detailed).await,
        HistoryAction::Clear { force } => clear_history(&history_path, force).await,
        HistoryAction::Remove { id } => remove_history_entry(&history_path, &id).await,
        HistoryAction::Stats => show_history_stats(&history_path).await,
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

/// Show download history
async fn show_history(
    history_path: &std::path::Path,
    limit: usize,
    status_filter: Option<&str>,
    detailed: bool,
) -> Result<()> {
    let history = DownloadHistory::load_or_create(history_path)?;

    println!("{} Download History", "📚".blue().bold());
    println!();

    if history.entries.is_empty() {
        println!("{} No download history found", "ℹ️".blue());
        return Ok(());
    }

    // Filter by status if requested
    let entries: Vec<_> = if let Some(status_str) = status_filter {
        let target_status = match status_str.to_lowercase().as_str() {
            "success" => TaskStatus::Success,
            "failed" => TaskStatus::Failed("".to_string()),
            "in_progress" | "inprogress" => TaskStatus::InProgress,
            "cancelled" => TaskStatus::Cancelled,
            "paused" => TaskStatus::Paused,
            _ => {
                return Err(IaGetError::Config(format!(
                    "Invalid status filter: {}. Valid options: success, failed, in_progress, cancelled, paused",
                    status_str
                )));
            }
        };
        history.get_entries_by_status(&target_status)
    } else {
        history.get_recent_entries(limit)
    };

    if entries.is_empty() {
        println!("{} No entries match the specified criteria", "ℹ️".blue());
        return Ok(());
    }

    for (i, entry) in entries.iter().enumerate() {
        if i >= limit {
            break;
        }

        let status_display = match &entry.status {
            TaskStatus::Success => "✅ Success".green(),
            TaskStatus::Failed(msg) => format!("❌ Failed: {}", msg).red(),
            TaskStatus::InProgress => "🔄 In Progress".yellow(),
            TaskStatus::Cancelled => "⏹️ Cancelled".cyan(),
            TaskStatus::Paused => "⏸️ Paused".blue(),
        };

        println!(
            "{} {}",
            format!("{}.", i + 1).dimmed(),
            entry.archive_identifier.bright_green()
        );
        println!("    ID: {}", entry.id.cyan());
        println!("    Status: {}", status_display);
        println!(
            "    Started: {}",
            entry
                .started_at
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string()
                .dimmed()
        );

        if let Some(completed) = entry.completed_at {
            println!(
                "    Completed: {}",
                completed
                    .format("%Y-%m-%d %H:%M:%S UTC")
                    .to_string()
                    .dimmed()
            );
        }

        if detailed {
            println!("    Original input: {}", entry.original_input.blue());
            println!("    Output directory: {}", entry.output_directory.cyan());
            println!(
                "    Progress: {}/{} files ({:.1}%)",
                entry.completed_files,
                entry.total_files,
                entry.completion_percentage()
            );
            println!(
                "    Data downloaded: {}",
                format_size(entry.bytes_downloaded)
            );

            if entry.failed_files > 0 {
                println!("    Failed files: {}", entry.failed_files.to_string().red());
            }
        }

        println!();
    }

    if entries.len() == limit && history.entries.len() > limit {
        println!(
            "{} Showing {} of {} entries. Use --limit to see more.",
            "ℹ️".blue(),
            limit,
            history.entries.len()
        );
    }

    Ok(())
}

/// Clear download history
async fn clear_history(history_path: &std::path::Path, force: bool) -> Result<()> {
    if !force {
        print!(
            "{} This will clear ALL download history. Continue? [y/N]: ",
            "⚠️".yellow()
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
            println!("History clear cancelled.");
            return Ok(());
        }
    }

    let mut history = DownloadHistory::load_or_create(history_path)?;
    let count = history.entries.len();
    history.clear();
    history.save_to_file(history_path)?;

    println!("{} Cleared {} history entries", "✅".green(), count);

    Ok(())
}

/// Remove specific history entry
async fn remove_history_entry(history_path: &std::path::Path, id: &str) -> Result<()> {
    let mut history = DownloadHistory::load_or_create(history_path)?;

    if history.remove_entry(id) {
        history.save_to_file(history_path)?;
        println!("{} Removed history entry: {}", "✅".green(), id.cyan());
    } else {
        return Err(IaGetError::Config(format!(
            "History entry with ID '{}' not found",
            id
        )));
    }

    Ok(())
}

/// Show download history statistics
async fn show_history_stats(history_path: &std::path::Path) -> Result<()> {
    let history = DownloadHistory::load_or_create(history_path)?;

    println!("{} Download History Statistics", "📊".blue().bold());
    println!();

    if history.entries.is_empty() {
        println!("{} No download history found", "ℹ️".blue());
        return Ok(());
    }

    let stats = history.get_statistics();

    println!("{} Overall Statistics:", "📈".green());
    println!(
        "  Total downloads: {}",
        stats.total_downloads.to_string().cyan()
    );
    println!(
        "  Successful: {} ({:.1}%)",
        stats.successful_downloads.to_string().green(),
        (stats.successful_downloads as f32 / stats.total_downloads as f32) * 100.0
    );
    println!(
        "  Failed: {} ({:.1}%)",
        stats.failed_downloads.to_string().red(),
        (stats.failed_downloads as f32 / stats.total_downloads as f32) * 100.0
    );

    if stats.in_progress_downloads > 0 {
        println!(
            "  In progress: {}",
            stats.in_progress_downloads.to_string().yellow()
        );
    }

    if stats.cancelled_downloads > 0 {
        println!(
            "  Cancelled: {}",
            stats.cancelled_downloads.to_string().cyan()
        );
    }

    println!();
    println!("{} Data Transfer:", "💾".blue());
    println!(
        "  Total data downloaded: {}",
        format_size(stats.total_bytes_downloaded).bright_green()
    );
    println!(
        "  Total files downloaded: {}",
        stats.total_files_downloaded.to_string().cyan()
    );

    if stats.total_files_downloaded > 0 {
        let avg_file_size = stats.total_bytes_downloaded / stats.total_files_downloaded as u64;
        println!(
            "  Average file size: {}",
            format_size(avg_file_size).yellow()
        );
    }

    println!();
    println!("{} Database Information:", "🗃️".purple());
    println!("  Database version: {}", history.version.cyan());
    println!(
        "  Created: {}",
        history
            .created_at
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string()
            .dimmed()
    );
    println!(
        "  Last updated: {}",
        history
            .last_updated
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string()
            .dimmed()
    );
    println!("  Max entries: {}", history.max_entries.to_string().cyan());

    Ok(())
}

/// Helper function to format optional strings
fn format_option(opt: &Option<String>) -> colored::ColoredString {
    match opt {
        Some(value) => value.green(),
        None => "(not set)".dimmed(),
    }
}

/// Helper function to format boolean values
fn format_bool(value: bool) -> colored::ColoredString {
    if value {
        "enabled".green()
    } else {
        "disabled".red()
    }
}

/// Helper function to parse boolean values from strings
fn parse_bool(value: &str) -> Result<bool> {
    match value.to_lowercase().as_str() {
        "true" | "yes" | "on" | "1" | "enabled" => Ok(true),
        "false" | "no" | "off" | "0" | "disabled" => Ok(false),
        _ => Err(IaGetError::Config(format!(
            "Invalid boolean value: '{}'. Use true/false, yes/no, on/off, 1/0, or enabled/disabled",
            value
        ))),
    }
}

/// Display Archive.org API health and monitoring information
pub async fn display_api_health() -> anyhow::Result<()> {
    println!("{} Archive.org API Health Status", "🏥".blue().bold());
    println!();

    // Create a test API client
    let client = reqwest::Client::builder()
        .user_agent(get_user_agent())
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .context("Failed to create HTTP client")?;

    let mut api_client = EnhancedArchiveApiClient::new(client);

    // Test basic connectivity with official status endpoint
    println!("{} Testing Archive.org service status...", "🔗".cyan());
    match api_client.get_service_status().await {
        Ok(response) => {
            let status = response.status();
            println!("  ✅ Status endpoint successful (HTTP {})", status);

            // Try to parse the status response
            if let Ok(text) = response.text().await {
                if let Ok(status_data) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(status_msg) = status_data.get("status").and_then(|s| s.as_str()) {
                        println!("  📊 Service Status: {}", status_msg);
                    }
                }
            }
        }
        Err(e) => {
            println!("  ❌ Status check failed: {}", e);
        }
    }

    // Test metadata API
    println!("\n{} Testing Metadata API...", "📋".cyan());
    match api_client.get_metadata("nasa").await {
        Ok(response) => {
            println!(
                "  ✅ Metadata API successful (status: {})",
                response.status()
            );
        }
        Err(e) => {
            println!("  ❌ Metadata API failed: {}", e);
        }
    }

    // Test search API
    println!("\n{} Testing Search API...", "🔍".cyan());
    match api_client
        .search_items("collection:nasa", Some("identifier,title"), Some(1), None)
        .await
    {
        Ok(response) => {
            println!("  ✅ Search API successful (status: {})", response.status());

            // Parse search results to show functionality
            if let Ok(text) = response.text().await {
                if let Ok(search_data) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(num_found) =
                        search_data.get("response").and_then(|r| r.get("numFound"))
                    {
                        println!(
                            "  📊 Search returned {} total items in nasa collection",
                            num_found
                        );
                    }
                }
            }
        }
        Err(e) => {
            println!("  ❌ Search API failed: {}", e);
        }
    }

    // Display server list
    println!("\n{} Available Archive.org Servers:", "🌐".green().bold());
    let servers = get_archive_servers();
    for (i, server) in servers.iter().enumerate() {
        println!(
            "  {:<2} {}",
            format!("{}.", i + 1).dimmed(),
            server.bright_blue()
        );
    }

    // Test multiple requests to show rate limiting
    println!("\n{} Testing API rate limiting...", "⏱️".yellow());

    for i in 0..3 {
        // Use valid identifiers that exist
        let test_identifiers = ["mario", "luigi", "nasa"];
        let identifier = test_identifiers[i % test_identifiers.len()];

        match api_client.get_metadata(identifier).await {
            Ok(_) => {
                let stats = api_client.get_stats();
                println!(
                    "  Request {}: ✅ {} (Rate: {:.1} req/min)",
                    i + 1,
                    identifier,
                    stats.average_requests_per_minute
                );
            }
            Err(e) => {
                println!("  Request {}: ❌ {} - {}", i + 1, identifier, e);
            }
        }
    }

    // Display final statistics
    println!("\n{} API Session Statistics:", "📊".purple().bold());
    let final_stats = api_client.get_stats();
    println!("  {}", final_stats);

    // Health assessment
    println!("\n{} Health Assessment:", "🎯".bright_green().bold());
    if api_client.is_rate_healthy() {
        println!("  ✅ Request rate is healthy and Archive.org compliant");
    } else {
        println!("  ⚠️  Request rate is high - consider slowing down requests");
    }

    // Enhanced API capabilities
    println!(
        "\n{} Enhanced API Capabilities:",
        "⚡".bright_yellow().bold()
    );
    println!("  ✅ Metadata API - Item information and file listings");
    println!("  ✅ Search API - Finding items across collections");
    println!("  ✅ Tasks API - Monitoring upload/processing status");
    println!("  ✅ Collections API - Batch operations on collections");
    println!("  ✅ Status API - Real-time service health monitoring");

    println!(
        "\n{} Archive.org API Guidelines:",
        "📋".bright_cyan().bold()
    );
    println!("  • Keep concurrent connections ≤ 5 for respectful usage");
    println!("  • Include descriptive User-Agent with contact information");
    println!("  • Implement retry logic for transient failures");
    println!("  • Honor rate limiting (429) and retry-after headers");
    println!("  • Use appropriate timeouts for large file downloads");

    println!("\n{} Current Configuration:", "⚙️".bright_magenta().bold());
    println!("  User Agent: {}", get_user_agent().bright_green());
    println!("  Default Timeout: {} seconds", HTTP_TIMEOUT);
    println!("  Min Request Delay: {}ms", MIN_REQUEST_DELAY_MS);
    println!(
        "  Max Concurrent: {} connections",
        MAX_CONCURRENT_CONNECTIONS
    );

    Ok(())
}

/// Analyze and display enhanced metadata for an archive
pub async fn analyze_archive_metadata(identifier: &str) -> anyhow::Result<()> {
    println!("{} Enhanced Metadata Analysis", "🔍".blue().bold());
    println!("Archive: {}", identifier.bright_green());
    println!();

    // Create HTTP client
    let client = reqwest::Client::builder()
        .user_agent(get_user_agent())
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .context("Failed to create HTTP client")?;

    // Create enhanced API client
    let api_client = EnhancedArchiveApiClient::new(client.clone());

    // Create advanced metadata processor
    let mut processor = AdvancedMetadataProcessor::new(api_client);

    // Perform comprehensive analysis
    match processor.analyze_metadata(identifier).await {
        Ok(analysis) => {
            // Display the comprehensive analysis
            processor.display_analysis(&analysis);

            // Show additional insights
            if analysis.completeness_score < 60.0 {
                println!("{}", "💡 Suggestions for improvement:".yellow().bold());
                if !analysis.quality_indicators.has_description {
                    println!("  • Add a detailed description to improve discoverability");
                }
                if !analysis.quality_indicators.has_creator {
                    println!("  • Specify the creator or author information");
                }
                if analysis.quality_indicators.files_have_checksums < 80.0 {
                    println!(
                        "  • Consider adding checksums to more files for integrity verification"
                    );
                }
                println!();
            }

            // Show technical details for advanced users
            if std::env::var("IA_GET_VERBOSE").is_ok() {
                println!("{}", "🔧 Technical Details:".dimmed().bold());
                println!("  Size Distribution:");
                println!(
                    "    Small files (< 1MB): {}",
                    analysis.size_distribution.small_files
                );
                println!(
                    "    Medium files (1MB-100MB): {}",
                    analysis.size_distribution.medium_files
                );
                println!(
                    "    Large files (100MB-1GB): {}",
                    analysis.size_distribution.large_files
                );
                println!(
                    "    Huge files (> 1GB): {}",
                    analysis.size_distribution.huge_files
                );
                println!(
                    "    Average size: {}",
                    format_size(analysis.size_distribution.average_size)
                );
                println!(
                    "    Median size: {}",
                    format_size(analysis.size_distribution.median_size)
                );

                if !analysis.largest_files.is_empty() {
                    println!("  Largest Files:");
                    for (i, file) in analysis.largest_files.iter().take(5).enumerate() {
                        let truncated_name = if file.name.len() > 50 {
                            format!("{}...", &file.name[..47])
                        } else {
                            file.name.clone()
                        };
                        println!(
                            "    {}. {} ({})",
                            i + 1,
                            truncated_name,
                            format_size(file.size)
                        );
                    }
                }
                println!();
            }

            println!(
                "\n{} Use this analysis to make informed download decisions!",
                "💡".bright_yellow()
            );
        }
        Err(e) => {
            eprintln!(
                "{} {} {}",
                "❌".red(),
                "Failed to analyze metadata:".red().bold(),
                e
            );
            std::process::exit(1);
        }
    }

    Ok(())
}
