//! Session management and storage
//!
//! Contains session tracking, metadata storage, and download state management.

pub use metadata_storage::*;

pub mod metadata_storage;

/// Progress update information for callbacks
#[derive(Debug, Clone)]
pub struct ProgressUpdate {
    pub current_file: String,
    pub completed_files: usize,
    pub total_files: usize,
    pub failed_files: usize,
    pub current_speed: f64,
    pub eta: String,
    pub status: String,
}

/// Progress update callback function type
pub type ProgressCallback = Box<dyn Fn(ProgressUpdate) + Send + Sync>;
