//! Persistence module for ia-get
//!
//! Handles persistent data including download history and task information.

pub mod download_history;

pub use download_history::{DownloadHistory, DownloadHistoryEntry, TaskStatus};
