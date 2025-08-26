//! Metadata storage module for ia-get
//!
//! Handles storing and retrieving complete Internet Archive JSON metadata
//! for download resumption and comprehensive file management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use crate::{Result, IaGetError};

/// Complete Internet Archive metadata response structure
/// Based on https://archive.org/developers/md-read.html
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArchiveMetadata {
    /// Timestamp when the item was created
    pub created: u64,
    /// Primary datanode server
    pub d1: String,
    /// Secondary datanode server  
    pub d2: String,
    /// Directory path on servers
    pub dir: String,
    /// Complete list of files in the archive
    pub files: Vec<ArchiveFile>,
    /// Total number of files
    pub files_count: u32,
    /// Last time the item was updated
    pub item_last_updated: u64,
    /// Total size of all files in bytes
    pub item_size: u64,
    /// Item metadata (title, description, etc.)
    pub metadata: serde_json::Value,
    /// Primary server for downloads
    pub server: String,
    /// Unique identifier for the record
    pub uniq: u64,
    /// List of servers that can serve the files
    pub workable_servers: Vec<String>,
    /// Optional reviews data
    #[serde(default)]
    pub reviews: Vec<serde_json::Value>,
}

/// Individual file entry from Internet Archive
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArchiveFile {
    /// File name
    pub name: String,
    /// Source type (original, metadata, etc.)
    pub source: String,
    /// File format
    pub format: Option<String>,
    /// Last modified time as Unix timestamp string
    #[serde(default, deserialize_with = "deserialize_string_to_u64_option")]
    pub mtime: Option<u64>,
    /// File size in bytes as string
    #[serde(default, deserialize_with = "deserialize_string_to_u64_option")]
    pub size: Option<u64>,
    /// MD5 hash of the file
    pub md5: Option<String>,
    /// CRC32 checksum
    pub crc32: Option<String>,
    /// SHA1 hash
    pub sha1: Option<String>,
    /// BitTorrent info hash
    pub btih: Option<String>,
    /// Summation type for checksums
    pub summation: Option<String>,
    /// Original file reference
    pub original: Option<String>,
    /// Rotation angle for images
    pub rotation: Option<u32>,
}

/// Download session metadata for resumption
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadSession {
    /// Original URL requested
    pub original_url: String,
    /// Archive identifier
    pub identifier: String,
    /// Complete archive metadata
    pub archive_metadata: ArchiveMetadata,
    /// Download configuration used
    pub download_config: DownloadConfig,
    /// Files that were requested for download
    pub requested_files: Vec<String>,
    /// Download status for each file
    pub file_status: HashMap<String, FileDownloadStatus>,
    /// Session start time
    pub session_start: u64,
    /// Last update time
    pub last_updated: u64,
}

/// Configuration used for downloads
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadConfig {
    /// Output directory
    pub output_dir: String,
    /// Maximum concurrent downloads
    pub max_concurrent: u32,
    /// File format filters
    pub format_filters: Vec<String>,
    /// Minimum file size
    pub min_size: Option<u64>,
    /// Maximum file size  
    pub max_size: Option<u64>,
    /// Whether to verify MD5 hashes
    pub verify_md5: bool,
    /// Whether to preserve file modification times
    pub preserve_mtime: bool,
    /// User agent string
    pub user_agent: String,
}

/// Status of an individual file download
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileDownloadStatus {
    /// File information from archive
    pub file_info: ArchiveFile,
    /// Download state
    pub status: DownloadState,
    /// Bytes downloaded so far
    pub bytes_downloaded: u64,
    /// Download start time
    pub started_at: Option<u64>,
    /// Download completion time
    pub completed_at: Option<u64>,
    /// Error message if download failed
    pub error_message: Option<String>,
    /// Number of retry attempts
    pub retry_count: u32,
    /// Server used for download
    pub server_used: Option<String>,
    /// Local file path
    pub local_path: String,
}

/// Download state enumeration
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DownloadState {
    /// Not yet started
    Pending,
    /// Currently downloading
    InProgress,
    /// Successfully completed
    Completed,
    /// Failed with error
    Failed,
    /// Paused/cancelled
    Paused,
    /// Skipped (e.g., already exists)
    Skipped,
}

/// Custom deserializer for string numbers to u64 Option with default support
fn deserialize_string_to_u64_option<'de, D>(deserializer: D) -> std::result::Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, Visitor};
    use std::fmt;

    struct StringToU64Visitor;

    impl<'de> Visitor<'de> for StringToU64Visitor {
        type Value = Option<u64>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string, number, or null that can be converted to u64")
        }

        fn visit_str<E>(self, value: &str) -> std::result::Result<Option<u64>, E>
        where
            E: de::Error,
        {
            if value.is_empty() {
                Ok(None)
            } else {
                value.parse::<u64>().map(Some).map_err(de::Error::custom)
            }
        }

        fn visit_u64<E>(self, value: u64) -> std::result::Result<Option<u64>, E>
        where
            E: de::Error,
        {
            Ok(Some(value))
        }

        fn visit_i64<E>(self, value: i64) -> std::result::Result<Option<u64>, E>
        where
            E: de::Error,
        {
            if value >= 0 {
                Ok(Some(value as u64))
            } else {
                Err(de::Error::custom("negative number cannot be converted to u64"))
            }
        }

        fn visit_none<E>(self) -> std::result::Result<Option<u64>, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> std::result::Result<Option<u64>, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_any(StringToU64Visitor)
}

impl DownloadSession {
    /// Create a new download session
    pub fn new(
        original_url: String,
        identifier: String,
        archive_metadata: ArchiveMetadata,
        download_config: DownloadConfig,
        requested_files: Vec<String>,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut file_status = HashMap::new();
        for file_name in &requested_files {
            if let Some(file_info) = archive_metadata.files.iter().find(|f| f.name == *file_name) {
                let local_path = format!("{}/{}", download_config.output_dir, file_name);
                file_status.insert(file_name.clone(), FileDownloadStatus {
                    file_info: file_info.clone(),
                    status: DownloadState::Pending,
                    bytes_downloaded: 0,
                    started_at: None,
                    completed_at: None,
                    error_message: None,
                    retry_count: 0,
                    server_used: None,
                    local_path,
                });
            }
        }

        Self {
            original_url,
            identifier,
            archive_metadata,
            download_config,
            requested_files,
            file_status,
            session_start: now,
            last_updated: now,
        }
    }

    /// Save session to disk
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| IaGetError::JsonParsing(format!("Failed to serialize session: {}", e)))?;
        
        std::fs::write(path, json)
            .map_err(|e| IaGetError::FileSystem(format!("Failed to write session file: {}", e)))?;
        
        Ok(())
    }

    /// Load session from disk
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| IaGetError::FileSystem(format!("Failed to read session file: {}", e)))?;
        
        serde_json::from_str(&content)
            .map_err(|e| IaGetError::JsonParsing(format!("Failed to parse session file: {}", e)))
    }

    /// Update file status
    pub fn update_file_status(&mut self, file_name: &str, status: DownloadState) {
        if let Some(file_status) = self.file_status.get_mut(file_name) {
            file_status.status = status;
            self.last_updated = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
    }

    /// Get files that still need to be downloaded
    pub fn get_pending_files(&self) -> Vec<&str> {
        self.file_status
            .iter()
            .filter(|(_, status)| matches!(status.status, DownloadState::Pending | DownloadState::Failed))
            .map(|(name, _)| name.as_str())
            .collect()
    }

    /// Get download progress summary
    pub fn get_progress_summary(&self) -> DownloadProgress {
        let mut completed = 0;
        let mut failed = 0;
        let mut in_progress = 0;
        let mut total_bytes = 0;
        let mut downloaded_bytes = 0;

        for status in self.file_status.values() {
            match status.status {
                DownloadState::Completed => completed += 1,
                DownloadState::Failed => failed += 1,
                DownloadState::InProgress => in_progress += 1,
                _ => {}
            }
            
            if let Some(size) = status.file_info.size {
                total_bytes += size;
            }
            downloaded_bytes += status.bytes_downloaded;
        }

        DownloadProgress {
            total_files: self.file_status.len(),
            completed_files: completed,
            failed_files: failed,
            in_progress_files: in_progress,
            total_bytes,
            downloaded_bytes,
        }
    }
}

/// Progress summary for downloads
#[derive(Debug)]
pub struct DownloadProgress {
    pub total_files: usize,
    pub completed_files: usize,
    pub failed_files: usize,
    pub in_progress_files: usize,
    pub total_bytes: u64,
    pub downloaded_bytes: u64,
}

impl ArchiveFile {
    /// Get the download URL for this file using the specified server
    pub fn get_download_url(&self, server: &str, dir: &str) -> String {
        format!("https://{}{}/{}", server, dir, self.name)
    }

    /// Check if this file matches the given format filters
    pub fn matches_format_filter(&self, filters: &[String]) -> bool {
        if filters.is_empty() {
            return true;
        }

        if let Some(format) = &self.format {
            filters.iter().any(|filter| {
                format.to_lowercase().contains(&filter.to_lowercase())
            })
        } else {
            false
        }
    }

    /// Check if this file meets size requirements
    pub fn meets_size_requirements(&self, min_size: Option<u64>, max_size: Option<u64>) -> bool {
        if let Some(size) = self.size {
            if let Some(min) = min_size {
                if size < min {
                    return false;
                }
            }
            if let Some(max) = max_size {
                if size > max {
                    return false;
                }
            }
        }
        true
    }

    /// Validate MD5 hash of a local file
    pub fn validate_md5<P: AsRef<Path>>(&self, file_path: P) -> Result<bool> {
        if let Some(expected_md5) = &self.md5 {
            let actual_md5 = crate::utils::calculate_md5(file_path)?;
            Ok(actual_md5.to_lowercase() == expected_md5.to_lowercase())
        } else {
            Ok(true) // No MD5 to validate
        }
    }

    /// Set the modification time of a local file to match the archive
    pub fn set_file_mtime<P: AsRef<Path>>(&self, file_path: P) -> Result<()> {
        if let Some(mtime) = self.mtime {
            use std::time::UNIX_EPOCH;
            
            let _modified_time = UNIX_EPOCH + std::time::Duration::from_secs(mtime);
            let metadata = std::fs::metadata(&file_path)
                .map_err(|e| IaGetError::FileSystem(format!("Failed to get file metadata: {}", e)))?;
            
            let permissions = metadata.permissions();
            std::fs::set_permissions(&file_path, permissions)
                .map_err(|e| IaGetError::FileSystem(format!("Failed to set permissions: {}", e)))?;
            
            // Note: Setting file times on Windows requires additional handling
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                let atime = metadata.atime();
                use std::process::Command;
                
                let mtime_str = mtime.to_string();
                Command::new("touch")
                    .args(&["-t", &format!("{}", mtime), file_path.as_ref().to_str().unwrap()])
                    .output()
                    .map_err(|e| IaGetError::FileSystem(format!("Failed to set mtime: {}", e)))?;
            }
            
            #[cfg(windows)]
            {
                // On Windows, we'll use the filetime crate if available, or skip mtime setting
                eprintln!("Warning: Setting file modification time not implemented on Windows");
            }
        }
        Ok(())
    }
}

/// Generate a session file name based on identifier and timestamp
pub fn generate_session_filename(identifier: &str) -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("ia-get-session-{}-{}.json", identifier, timestamp)
}

/// Find the most recent session file for an identifier
pub fn find_latest_session_file(identifier: &str, session_dir: &str) -> Result<Option<String>> {
    let session_pattern = format!("ia-get-session-{}-", identifier);
    
    let entries = std::fs::read_dir(session_dir)
        .map_err(|e| IaGetError::FileSystem(format!("Failed to read session directory: {}", e)))?;
    
    let mut session_files = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| IaGetError::FileSystem(format!("Failed to read directory entry: {}", e)))?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        
        if file_name_str.starts_with(&session_pattern) && file_name_str.ends_with(".json") {
            session_files.push(entry.path());
        }
    }
    
    // Sort by modification time, newest first
    session_files.sort_by_key(|path| {
        std::fs::metadata(path)
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });
    
    Ok(session_files.last().map(|p| p.to_string_lossy().to_string()))
}
