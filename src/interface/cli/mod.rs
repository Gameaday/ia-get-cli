//! Command-line interface
//!
//! - [`definition`]: the clap command/argument tree
//! - [`types`]: CLI types (`SourceType`, config/history actions)
//! - [`commands`]: command handlers (config, history, api-health, analyze)
//! - [`advanced_commands`]: search and batch operations

pub mod advanced_commands;
pub mod commands;
pub mod definition;
pub mod types;

pub use commands::{
    analyze_archive_metadata, display_api_health, handle_config_command, handle_history_command,
};
pub use definition::{build_cli, get_source_types_from_matches};
pub use types::*;
