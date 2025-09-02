//! Command handlers for configuration and history management
//!
//! Contains the implementation of CLI subcommands for managing configuration
//! and download history.

use crate::{
    cli::{ConfigAction, HistoryAction},
    config::{Config, ConfigManager},
    download_history::{DownloadHistoryManager, DownloadStatus},
    filters::format_size,
    Result,
};
use colored::Colorize;
use std::io::{self, Write};

/// Handle configuration management commands
pub async fn handle_config_command(action: ConfigAction) -> Result<()> {
    let config_manager = ConfigManager::new()?;

    match action {
        ConfigAction::Show => show_config(&config_manager).await,
        ConfigAction::Set { key, value } => set_config_value(&config_manager, &key, &value).await,
        ConfigAction::Get { key } => get_config_value(&config_manager, &key).await,
        ConfigAction::Reset => reset_config(&config_manager).await,
        ConfigAction::Path => show_config_path(&config_manager).await,
        ConfigAction::Edit => edit_config(&config_manager).await,
    }
}

/// Handle download history commands
pub async fn handle_history_command(action: HistoryAction) -> Result<()> {
    let history_manager = DownloadHistoryManager::new()?;

    match action {
        HistoryAction::List { limit, status } => {
            list_download_history(&history_manager, limit, status).await
        }
        HistoryAction::Stats => show_download_statistics(&history_manager).await,
        HistoryAction::Cleanup { days } => cleanup_download_history(&history_manager, days).await,
        HistoryAction::Path => show_history_path(&history_manager).await,
        HistoryAction::Show { id } => show_download_details(&history_manager, &id).await,
    }
}

/// Show current configuration
async fn show_config(config_manager: &ConfigManager) -> Result<()> {
    let config = config_manager.load_config()?;

    println!("{}", "📋 Current Configuration".bold().cyan());
    println!("{}", "=".repeat(50).dimmed());

    println!("{}: {}", "Output Directory".yellow(), format_option(&config.default_output_path));
    println!("{}: {}", "Concurrent Downloads".yellow(), config.concurrent_downloads.to_string().green());
    println!("{}: {}", "Max Retries".yellow(), config.max_retries.to_string().green());
    println!("{}: {}", "Include Extensions".yellow(), format_option(&config.default_include_ext));
    println!("{}: {}", "Exclude Extensions".yellow(), format_option(&config.default_exclude_ext));
    println!("{}: {}", "Max File Size".yellow(), format_option(&config.default_max_file_size));
    println!("{}: {}", "Resume by Default".yellow(), format_bool(config.default_resume));
    println!("{}: {}", "Verbose by Default".yellow(), format_bool(config.default_verbose));
    println!("{}: {}", "Log Hash Errors".yellow(), format_bool(config.default_log_hash_errors));
    println!("{}: {}", "Dry Run by Default".yellow(), format_bool(config.default_dry_run));
    println!("{}: {} seconds", "HTTP Timeout".yellow(), config.http_timeout.to_string().green());
    println!("{}: {}", "User Agent Override".yellow(), format_option(&config.user_agent_override));

    if !config.recent_urls.is_empty() {
        println!("\n{}: {} URLs", "Recent URLs".yellow(), config.recent_urls.len().to_string().green());
        for (i, url) in config.recent_urls.iter().enumerate().take(5) {
            println!("  {}. {}", (i + 1).to_string().dimmed(), url.bright_blue());
        }
        if config.recent_urls.len() > 5 {
            println!("  ... and {} more", (config.recent_urls.len() - 5).to_string().dimmed());
        }
    }

    if !config.filter_presets.is_empty() {
        println!("\n{}: {} presets", "Filter Presets".yellow(), config.filter_presets.len().to_string().green());
        for preset in &config.filter_presets {
            println!("  • {}: {}", preset.name.bright_green(), preset.description.dimmed());
        }
    }

    Ok(())
}

/// Set a configuration value
async fn set_config_value(config_manager: &ConfigManager, key: &str, value: &str) -> Result<()> {
    let mut config = config_manager.load_config()?;

    match key {
        "output_dir" | "output_directory" => {
            config.default_output_path = if value.is_empty() { None } else { Some(value.to_string()) };
            println!("{} Output directory set to: {}", "✅".green(), format_option(&config.default_output_path));
        }
        "concurrent_downloads" | "concurrent" => {
            let concurrent: usize = value.parse().map_err(|_| {
                crate::error::IaGetError::Config("Invalid number for concurrent downloads".to_string())
            })?;
            if concurrent == 0 || concurrent > 16 {
                return Err(crate::error::IaGetError::Config("Concurrent downloads must be between 1 and 16".to_string()));
            }
            config.concurrent_downloads = concurrent;
            println!("{} Concurrent downloads set to: {}", "✅".green(), concurrent.to_string().cyan());
        }
        "max_retries" | "retries" => {
            let retries: usize = value.parse().map_err(|_| {
                crate::error::IaGetError::Config("Invalid number for max retries".to_string())
            })?;
            if retries > 20 {
                return Err(crate::error::IaGetError::Config("Max retries cannot exceed 20".to_string()));
            }
            config.max_retries = retries;
            println!("{} Max retries set to: {}", "✅".green(), retries.to_string().cyan());
        }
        "include_ext" | "include_extensions" => {
            config.default_include_ext = if value.is_empty() { None } else { Some(value.to_string()) };
            println!("{} Include extensions set to: {}", "✅".green(), format_option(&config.default_include_ext));
        }
        "exclude_ext" | "exclude_extensions" => {
            config.default_exclude_ext = if value.is_empty() { None } else { Some(value.to_string()) };
            println!("{} Exclude extensions set to: {}", "✅".green(), format_option(&config.default_exclude_ext));
        }
        "max_file_size" | "max_size" => {
            config.default_max_file_size = if value.is_empty() { None } else { Some(value.to_string()) };
            println!("{} Max file size set to: {}", "✅".green(), format_option(&config.default_max_file_size));
        }
        "resume" => {
            let resume = parse_bool(value)?;
            config.default_resume = resume;
            println!("{} Resume by default set to: {}", "✅".green(), format_bool(resume));
        }
        "verbose" => {
            let verbose = parse_bool(value)?;
            config.default_verbose = verbose;
            println!("{} Verbose by default set to: {}", "✅".green(), format_bool(verbose));
        }
        "log_hash_errors" => {
            let log_errors = parse_bool(value)?;
            config.default_log_hash_errors = log_errors;
            println!("{} Log hash errors set to: {}", "✅".green(), format_bool(log_errors));
        }
        "dry_run" => {
            let dry_run = parse_bool(value)?;
            config.default_dry_run = dry_run;
            println!("{} Dry run by default set to: {}", "✅".green(), format_bool(dry_run));
        }
        "http_timeout" | "timeout" => {
            let timeout: u64 = value.parse().map_err(|_| {
                crate::error::IaGetError::Config("Invalid number for HTTP timeout".to_string())
            })?;
            if timeout == 0 || timeout > 300 {
                return Err(crate::error::IaGetError::Config("HTTP timeout must be between 1 and 300 seconds".to_string()));
            }
            config.http_timeout = timeout;
            println!("{} HTTP timeout set to: {} seconds", "✅".green(), timeout.to_string().cyan());
        }
        "user_agent" => {
            config.user_agent_override = if value.is_empty() { None } else { Some(value.to_string()) };
            println!("{} User agent override set to: {}", "✅".green(), format_option(&config.user_agent_override));
        }
        _ => {
            println!("{} Unknown configuration key: {}", "❌".red(), key.red());
            println!("Valid keys: output_dir, concurrent_downloads, max_retries, include_ext, exclude_ext,");
            println!("           max_file_size, resume, verbose, log_hash_errors, dry_run, http_timeout, user_agent");
            return Ok(());
        }
    }

    config_manager.save_config(&config)?;
    println!("{} Configuration saved successfully", "💾".green());

    Ok(())
}

/// Get a configuration value
async fn get_config_value(config_manager: &ConfigManager, key: &str) -> Result<()> {
    let config = config_manager.load_config()?;

    let value = match key {
        "output_dir" | "output_directory" => format_option(&config.default_output_path),
        "concurrent_downloads" | "concurrent" => config.concurrent_downloads.to_string(),
        "max_retries" | "retries" => config.max_retries.to_string(),
        "include_ext" | "include_extensions" => format_option(&config.default_include_ext),
        "exclude_ext" | "exclude_extensions" => format_option(&config.default_exclude_ext),
        "max_file_size" | "max_size" => format_option(&config.default_max_file_size),
        "resume" => format_bool(config.default_resume),
        "verbose" => format_bool(config.default_verbose),
        "log_hash_errors" => format_bool(config.default_log_hash_errors),
        "dry_run" => format_bool(config.default_dry_run),
        "http_timeout" | "timeout" => format!("{} seconds", config.http_timeout),
        "user_agent" => format_option(&config.user_agent_override),
        _ => {
            println!("{} Unknown configuration key: {}", "❌".red(), key.red());
            return Ok(());
        }
    };

    println!("{}: {}", key.yellow(), value.cyan());
    Ok(())
}

/// Reset configuration to defaults
async fn reset_config(config_manager: &ConfigManager) -> Result<()> {
    print!("Are you sure you want to reset all configuration to defaults? [y/N]: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes" {
        let default_config = Config::default();
        config_manager.save_config(&default_config)?;
        println!("{} Configuration reset to defaults", "✅".green());
    } else {
        println!("Reset cancelled");
    }

    Ok(())
}

/// Show configuration file path
async fn show_config_path(config_manager: &ConfigManager) -> Result<()> {
    println!("{}", "📁 Configuration File Location".bold().cyan());
    println!("{}", "=".repeat(35).dimmed());
    println!("File: {}", config_manager.config_file_path().display().to_string().bright_green());
    println!("Directory: {}", config_manager.config_directory().display().to_string().bright_blue());
    Ok(())
}

/// Edit configuration interactively
async fn edit_config(config_manager: &ConfigManager) -> Result<()> {
    println!("{}", "⚙️  Interactive Configuration Editor".bold().cyan());
    println!("{}", "=".repeat(40).dimmed());

    // This would open an interactive menu similar to the existing interactive_menu.rs
    // For now, we'll just show the current config and suggest using 'config set'
    show_config(config_manager).await?;
    
    println!("\n{}", "💡 Interactive editing not yet implemented".yellow());
    println!("Use 'ia-get config set <key> <value>' to change individual settings");
    println!("Use 'ia-get config show' to view current configuration");

    Ok(())
}

/// List download history
async fn list_download_history(
    history_manager: &DownloadHistoryManager,
    limit: usize,
    status_filter: Option<String>,
) -> Result<()> {
    let status = if let Some(status_str) = status_filter {
        match status_str.to_lowercase().as_str() {
            "success" => Some(DownloadStatus::Success),
            "failed" => Some(DownloadStatus::Failed),
            "partial" => Some(DownloadStatus::PartialSuccess),
            "in-progress" | "inprogress" => Some(DownloadStatus::InProgress),
            "cancelled" => Some(DownloadStatus::Cancelled),
            _ => {
                println!("{} Unknown status filter: {}", "❌".red(), status_str.red());
                println!("Valid statuses: success, failed, partial, in-progress, cancelled");
                return Ok(());
            }
        }
    } else {
        None
    };

    let records = if status.is_some() {
        history_manager.get_download_records(status)?
    } else {
        history_manager.get_recent_downloads(limit)?
    };

    if records.is_empty() {
        println!("{} No download records found", "📭".yellow());
        return Ok(());
    }

    println!("{}", "📜 Download History".bold().cyan());
    println!("{}", "=".repeat(50).dimmed());

    for (i, record) in records.iter().enumerate() {
        if i >= limit {
            break;
        }

        let status_icon = match record.status {
            DownloadStatus::Success => "✅",
            DownloadStatus::Failed => "❌",
            DownloadStatus::PartialSuccess => "⚠️",
            DownloadStatus::InProgress => "🔄",
            DownloadStatus::Cancelled => "🚫",
        };

        let status_color = match record.status {
            DownloadStatus::Success => record.status.to_string().green(),
            DownloadStatus::Failed => record.status.to_string().red(),
            DownloadStatus::PartialSuccess => record.status.to_string().yellow(),
            DownloadStatus::InProgress => record.status.to_string().blue(),
            DownloadStatus::Cancelled => record.status.to_string().bright_black(),
        };

        println!("\n{} {}", status_icon, record.archive_identifier.bright_cyan().bold());
        println!("   ID: {}", record.id.dimmed());
        println!("   Status: {}", status_color);
        println!("   Files: {}/{} ({} failed)", 
                record.completed_files.to_string().green(),
                record.total_files,
                record.failed_files.to_string().red());
        println!("   Size: {}", format_size(record.bytes_downloaded).bright_blue());
        
        let started = chrono::DateTime::from_timestamp(record.started_at as i64, 0)
            .unwrap_or_default()
            .format("%Y-%m-%d %H:%M:%S");
        println!("   Started: {}", started.to_string().dimmed());

        if let Some(completed_at) = record.completed_at {
            let completed = chrono::DateTime::from_timestamp(completed_at as i64, 0)
                .unwrap_or_default()
                .format("%Y-%m-%d %H:%M:%S");
            let duration = completed_at - record.started_at;
            println!("   Completed: {} ({}s)", completed.to_string().dimmed(), duration);
        }

        if let Some(ref error) = record.error_message {
            println!("   Error: {}", error.red());
        }
    }

    if records.len() > limit {
        println!("\n... and {} more records", (records.len() - limit).to_string().dimmed());
        println!("Use '--limit {}' to see more", (limit + 10).to_string().cyan());
    }

    Ok(())
}

/// Show download statistics
async fn show_download_statistics(history_manager: &DownloadHistoryManager) -> Result<()> {
    let stats = history_manager.get_statistics()?;

    println!("{}", "📊 Download Statistics".bold().cyan());
    println!("{}", "=".repeat(30).dimmed());

    println!("{}: {}", "Total Downloads".yellow(), stats.total_downloads.to_string().bright_white());
    println!("{}: {}", "Successful".yellow(), stats.successful_downloads.to_string().green());
    println!("{}: {}", "Failed".yellow(), stats.failed_downloads.to_string().red());
    println!("{}: {}", "Partial Success".yellow(), stats.partial_downloads.to_string().yellow());
    println!("{}: {}", "In Progress".yellow(), stats.in_progress_downloads.to_string().blue());

    if stats.total_downloads > 0 {
        let success_rate = (stats.successful_downloads as f64 / stats.total_downloads as f64) * 100.0;
        println!("{}: {:.1}%", "Success Rate".yellow(), success_rate.to_string().green());
    }

    println!("{}: {}", "Total Files Downloaded".yellow(), stats.total_files_downloaded.to_string().bright_white());
    println!("{}: {}", "Total Data Downloaded".yellow(), format_size(stats.total_bytes_downloaded).bright_blue());

    Ok(())
}

/// Clean up old download records
async fn cleanup_download_history(history_manager: &DownloadHistoryManager, days: u32) -> Result<()> {
    print!("Remove download records older than {} days? [y/N]: ", days);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes" {
        let removed_count = history_manager.cleanup_old_records(days)?;
        println!("{} Removed {} old download records", "✅".green(), removed_count.to_string().cyan());
    } else {
        println!("Cleanup cancelled");
    }

    Ok(())
}

/// Show download history database path
async fn show_history_path(history_manager: &DownloadHistoryManager) -> Result<()> {
    println!("{}", "📁 Download History Database Location".bold().cyan());
    println!("{}", "=".repeat(45).dimmed());
    println!("File: {}", history_manager.get_database_path().display().to_string().bright_green());
    Ok(())
}

/// Show detailed information about a specific download
async fn show_download_details(history_manager: &DownloadHistoryManager, id: &str) -> Result<()> {
    if let Some(record) = history_manager.get_download_record(id)? {
        println!("{}", "📋 Download Details".bold().cyan());
        println!("{}", "=".repeat(30).dimmed());
        
        println!("{}: {}", "ID".yellow(), record.id.bright_white());
        println!("{}: {}", "Archive".yellow(), record.archive_identifier.bright_cyan());
        println!("{}: {}", "URL".yellow(), record.original_url.bright_blue());
        
        let status_color = match record.status {
            DownloadStatus::Success => record.status.to_string().green(),
            DownloadStatus::Failed => record.status.to_string().red(),
            DownloadStatus::PartialSuccess => record.status.to_string().yellow(),
            DownloadStatus::InProgress => record.status.to_string().blue(),
            DownloadStatus::Cancelled => record.status.to_string().bright_black(),
        };
        println!("{}: {}", "Status".yellow(), status_color);
        
        println!("{}: {}/{} completed, {} failed", "Files".yellow(), 
                record.completed_files.to_string().green(),
                record.total_files,
                record.failed_files.to_string().red());
        println!("{}: {}", "Data Downloaded".yellow(), format_size(record.bytes_downloaded).bright_blue());
        
        let started = chrono::DateTime::from_timestamp(record.started_at as i64, 0)
            .unwrap_or_default()
            .format("%Y-%m-%d %H:%M:%S UTC");
        println!("{}: {}", "Started".yellow(), started.to_string().dimmed());

        if let Some(completed_at) = record.completed_at {
            let completed = chrono::DateTime::from_timestamp(completed_at as i64, 0)
                .unwrap_or_default()
                .format("%Y-%m-%d %H:%M:%S UTC");
            let duration = completed_at - record.started_at;
            println!("{}: {} (duration: {}s)", "Completed".yellow(), completed.to_string().dimmed(), duration);
        }

        if let Some(ref error) = record.error_message {
            println!("{}: {}", "Error".yellow(), error.red());
        }

        println!("\n{}", "Download Settings:".bold().yellow());
        println!("  Output: {}", record.settings.output_dir.cyan());
        println!("  Concurrent: {}", record.settings.max_concurrent.to_string().cyan());
        println!("  Verify MD5: {}", format_bool(record.settings.verify_md5));
        println!("  Compression: {}", format_bool(record.settings.enable_compression));
        println!("  Auto-decompress: {}", format_bool(record.settings.auto_decompress));
        
        if !record.settings.format_filters.is_empty() {
            println!("  Format filters: {}", record.settings.format_filters.join(", ").cyan());
        }

        if !record.failed_file_list.is_empty() {
            println!("\n{}", "Failed Files:".bold().red());
            for file in record.failed_file_list.iter().take(10) {
                println!("  • {}", file.red());
            }
            if record.failed_file_list.len() > 10 {
                println!("  ... and {} more", (record.failed_file_list.len() - 10).to_string().dimmed());
            }
        }

    } else {
        println!("{} Download record not found: {}", "❌".red(), id.red());
    }

    Ok(())
}

/// Helper function to format optional strings
fn format_option(opt: &Option<String>) -> String {
    match opt {
        Some(value) => value.clone().green().to_string(),
        None => "(not set)".dimmed().to_string(),
    }
}

/// Helper function to format boolean values
fn format_bool(value: bool) -> String {
    if value {
        "Yes".green().to_string()
    } else {
        "No".red().to_string()
    }
}

/// Helper function to parse boolean from string
fn parse_bool(value: &str) -> Result<bool> {
    match value.to_lowercase().as_str() {
        "true" | "yes" | "y" | "1" | "on" | "enable" | "enabled" => Ok(true),
        "false" | "no" | "n" | "0" | "off" | "disable" | "disabled" => Ok(false),
        _ => Err(crate::error::IaGetError::Config(
            format!("Invalid boolean value: '{}'. Use true/false, yes/no, or 1/0", value)
        )),
    }
}

impl std::fmt::Display for DownloadStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloadStatus::InProgress => write!(f, "In Progress"),
            DownloadStatus::Success => write!(f, "Success"),
            DownloadStatus::PartialSuccess => write!(f, "Partial Success"),
            DownloadStatus::Failed => write!(f, "Failed"),
            DownloadStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}