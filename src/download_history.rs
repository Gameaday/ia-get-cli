//! Download history database for ia-get
//!
//! Provides persistent tracking of download tasks, their status, and settings used.
//! Data is stored in JSON format as `ia-get-db.json` in the configuration directory.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::{config::ConfigManager, error::IaGetError, metadata_storage::DownloadConfig, Result};

/// Database entry for a completed or attempted download
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRecord {
    /// Unique identifier for this download record
    pub id: String,
    /// Internet Archive identifier
    pub archive_identifier: String,
    /// Original URL requested
    pub original_url: String,
    /// Download status
    pub status: DownloadStatus,
    /// Timestamp when download was started (Unix seconds)
    pub started_at: u64,
    /// Timestamp when download completed/failed (Unix seconds)
    pub completed_at: Option<u64>,
    /// Total files requested
    pub total_files: usize,
    /// Files successfully downloaded
    pub completed_files: usize,
    /// Files that failed to download
    pub failed_files: usize,
    /// Total bytes downloaded
    pub bytes_downloaded: u64,
    /// Download configuration/settings used
    pub settings: DownloadConfig,
    /// Error message if failed
    pub error_message: Option<String>,
    /// List of files that failed (up to 100 entries to prevent huge records)
    pub failed_file_list: Vec<String>,
}

/// Status of a download task
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DownloadStatus {
    /// Download is currently in progress
    InProgress,
    /// Download completed successfully (all files downloaded)
    Success,
    /// Download completed with some failures
    PartialSuccess,
    /// Download failed completely
    Failed,
    /// Download was cancelled by user
    Cancelled,
}

/// Download history database containing all download records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadDatabase {
    /// Version of the database format
    pub version: String,
    /// When the database was created
    pub created_at: u64,
    /// When the database was last updated
    pub last_updated: u64,
    /// All download records, keyed by record ID
    pub records: HashMap<String, DownloadRecord>,
}

impl Default for DownloadDatabase {
    fn default() -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            version: "1.0".to_string(),
            created_at: now,
            last_updated: now,
            records: HashMap::new(),
        }
    }
}

/// Manages the download history database
pub struct DownloadHistoryManager {
    db_file: PathBuf,
}

impl DownloadHistoryManager {
    /// Create a new download history manager
    pub fn new() -> Result<Self> {
        let config_dir = ConfigManager::get_config_directory()?;
        let db_file = config_dir.join("ia-get-db.json");

        Ok(Self { db_file })
    }

    /// Load the download database from disk
    pub fn load_database(&self) -> Result<DownloadDatabase> {
        if self.db_file.exists() {
            let content = fs::read_to_string(&self.db_file).map_err(|e| {
                IaGetError::Config(format!("Failed to read download database: {}", e))
            })?;

            serde_json::from_str(&content).map_err(|e| {
                IaGetError::Config(format!("Failed to parse download database: {}", e))
            })
        } else {
            // Create default database
            let db = DownloadDatabase::default();
            self.save_database(&db)?;
            Ok(db)
        }
    }

    /// Save the download database to disk
    pub fn save_database(&self, database: &DownloadDatabase) -> Result<()> {
        let content = serde_json::to_string_pretty(database).map_err(|e| {
            IaGetError::Config(format!("Failed to serialize download database: {}", e))
        })?;

        fs::write(&self.db_file, content).map_err(|e| {
            IaGetError::Config(format!("Failed to write download database: {}", e))
        })?;

        Ok(())
    }

    /// Add a new download record to the database
    pub fn add_download_record(&self, record: DownloadRecord) -> Result<()> {
        let mut db = self.load_database()?;
        db.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        db.records.insert(record.id.clone(), record);
        self.save_database(&db)
    }

    /// Update an existing download record
    pub fn update_download_record(&self, record: DownloadRecord) -> Result<()> {
        let mut db = self.load_database()?;
        db.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        db.records.insert(record.id.clone(), record);
        self.save_database(&db)
    }

    /// Get a download record by ID
    pub fn get_download_record(&self, id: &str) -> Result<Option<DownloadRecord>> {
        let db = self.load_database()?;
        Ok(db.records.get(id).cloned())
    }

    /// Get all download records, optionally filtered by status
    pub fn get_download_records(&self, status_filter: Option<DownloadStatus>) -> Result<Vec<DownloadRecord>> {
        let db = self.load_database()?;
        let mut records: Vec<DownloadRecord> = db.records.into_values().collect();
        
        if let Some(status) = status_filter {
            records.retain(|r| r.status == status);
        }
        
        // Sort by start time, newest first
        records.sort_by(|a, b| b.started_at.cmp(&a.started_at));
        
        Ok(records)
    }

    /// Get recent download records (last N records)
    pub fn get_recent_downloads(&self, limit: usize) -> Result<Vec<DownloadRecord>> {
        let mut records = self.get_download_records(None)?;
        records.truncate(limit);
        Ok(records)
    }

    /// Get download statistics
    pub fn get_statistics(&self) -> Result<DownloadStatistics> {
        let db = self.load_database()?;
        let records: Vec<_> = db.records.values().collect();

        let total_downloads = records.len();
        let successful_downloads = records.iter().filter(|r| r.status == DownloadStatus::Success).count();
        let failed_downloads = records.iter().filter(|r| r.status == DownloadStatus::Failed).count();
        let partial_downloads = records.iter().filter(|r| r.status == DownloadStatus::PartialSuccess).count();
        let in_progress_downloads = records.iter().filter(|r| r.status == DownloadStatus::InProgress).count();
        
        let total_files_downloaded: usize = records.iter().map(|r| r.completed_files).sum();
        let total_bytes_downloaded: u64 = records.iter().map(|r| r.bytes_downloaded).sum();

        Ok(DownloadStatistics {
            total_downloads,
            successful_downloads,
            failed_downloads,
            partial_downloads,
            in_progress_downloads,
            total_files_downloaded,
            total_bytes_downloaded,
        })
    }

    /// Remove old download records (older than specified days)
    pub fn cleanup_old_records(&self, days: u32) -> Result<usize> {
        let cutoff_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - (days as u64 * 24 * 60 * 60);

        let mut db = self.load_database()?;
        let initial_count = db.records.len();
        
        db.records.retain(|_, record| record.started_at > cutoff_time);
        
        let removed_count = initial_count - db.records.len();
        
        if removed_count > 0 {
            db.last_updated = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            self.save_database(&db)?;
        }

        Ok(removed_count)
    }

    /// Get the path to the database file
    pub fn get_database_path(&self) -> &PathBuf {
        &self.db_file
    }
}

/// Statistics about download history
#[derive(Debug)]
pub struct DownloadStatistics {
    pub total_downloads: usize,
    pub successful_downloads: usize,
    pub failed_downloads: usize,
    pub partial_downloads: usize,
    pub in_progress_downloads: usize,
    pub total_files_downloaded: usize,
    pub total_bytes_downloaded: u64,
}

/// Generate a unique ID for a download record
pub fn generate_download_id(archive_identifier: &str) -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    format!("{}_{}", archive_identifier, timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_database_creation() {
        let db = DownloadDatabase::default();
        assert_eq!(db.version, "1.0");
        assert!(db.records.is_empty());
        assert!(db.created_at > 0);
    }

    #[test]
    fn test_download_record_creation() {
        let config = DownloadConfig {
            output_dir: "/tmp".to_string(),
            max_concurrent: 4,
            format_filters: vec![],
            min_size: None,
            max_size: None,
            verify_md5: true,
            preserve_mtime: true,
            user_agent: "test".to_string(),
            enable_compression: false,
            auto_decompress: false,
            decompress_formats: vec![],
        };

        let record = DownloadRecord {
            id: "test_123".to_string(),
            archive_identifier: "test_archive".to_string(),
            original_url: "https://archive.org/details/test_archive".to_string(),
            status: DownloadStatus::Success,
            started_at: 1000,
            completed_at: Some(2000),
            total_files: 10,
            completed_files: 10,
            failed_files: 0,
            bytes_downloaded: 1024 * 1024,
            settings: config,
            error_message: None,
            failed_file_list: vec![],
        };

        assert_eq!(record.status, DownloadStatus::Success);
        assert_eq!(record.total_files, 10);
    }

    #[test]
    fn test_generate_download_id() {
        let id1 = generate_download_id("testarchive");
        // Sleep a small amount to ensure timestamp difference
        std::thread::sleep(std::time::Duration::from_millis(1));
        let id2 = generate_download_id("testarchive");
        
        // IDs should start with the archive identifier
        assert!(id1.starts_with("testarchive_"));
        assert!(id2.starts_with("testarchive_"));
        
        // Verify format: identifier_timestamp
        let parts1: Vec<&str> = id1.split('_').collect();
        let parts2: Vec<&str> = id2.split('_').collect();
        assert_eq!(parts1.len(), 2);
        assert_eq!(parts2.len(), 2);
        assert_eq!(parts1[0], "testarchive");
        assert_eq!(parts2[0], "testarchive");
        
        // Timestamps should be numeric
        assert!(parts1[1].parse::<u64>().is_ok());
        assert!(parts2[1].parse::<u64>().is_ok());
    }
}