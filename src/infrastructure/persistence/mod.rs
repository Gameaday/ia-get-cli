//! Persistence module for ia-get
//! 
//! Handles all persistent data including download history, task information,
//! and configuration management with proper priority handling.

pub mod download_history;
pub mod config_persistence;

pub use download_history::{DownloadHistory, DownloadHistoryEntry, TaskStatus};
pub use config_persistence::{ConfigPersistence, ConfigSource, ConfigPriority};