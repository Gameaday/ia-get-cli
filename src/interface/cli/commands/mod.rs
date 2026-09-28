//! Command handlers for the ia-get CLI, split by concern.
//!
//! - [`config`]: `ia-get config ...`
//! - [`history`]: `ia-get history ...`
//! - [`analysis`]: `--api-health` and `--analyze-metadata`

pub mod analysis;
pub mod config;
pub mod history;

pub use analysis::{analyze_archive_metadata, display_api_health};
pub use config::handle_config_command;
pub use history::handle_history_command;

use crate::{Result, error::IaGetError};
use colored::Colorize;

/// Helper function to format optional strings
pub(crate) fn format_option(opt: &Option<String>) -> colored::ColoredString {
    match opt {
        Some(value) => value.green(),
        None => "(not set)".dimmed(),
    }
}

/// Helper function to format boolean values
pub(crate) fn format_bool(value: bool) -> colored::ColoredString {
    if value {
        "enabled".green()
    } else {
        "disabled".red()
    }
}

/// Helper function to parse boolean values from strings
pub(crate) fn parse_bool(value: &str) -> Result<bool> {
    match value.to_lowercase().as_str() {
        "true" | "yes" | "on" | "1" | "enabled" => Ok(true),
        "false" | "no" | "off" | "0" | "disabled" => Ok(false),
        _ => Err(IaGetError::Config(format!(
            "Invalid boolean value: '{}'. Use true/false, yes/no, on/off, 1/0, or enabled/disabled",
            value
        ))),
    }
}
