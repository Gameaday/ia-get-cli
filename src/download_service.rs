//! Unified download service that provides a single API for both CLI and GUI
//!
//! This module provides a unified interface for downloading files from the Internet Archive,
//! ensuring both CLI and GUI use exactly the same API and business logic.

use crate::{
    config::Config,
    constants::get_user_agent,
    enhanced_downloader::ArchiveDownloader,
    fetch_json_metadata,
    filters::{format_size, parse_size_string},
    metadata_storage::{ArchiveFile, DownloadConfig, DownloadSession},
    IaGetError, Result,
};
use reqwest::Client;
use std::path::PathBuf;

/// Unified download request parameters used by both CLI and GUI
#[derive(Debug, Clone)]
pub struct DownloadRequest {
    /// Archive identifier or URL
    pub identifier: String,
    /// Output directory for downloaded files
    pub output_dir: PathBuf,
    /// Formats to include (empty means include all)
    pub include_formats: Vec<String>,
    /// Formats to exclude
    pub exclude_formats: Vec<String>,
    /// Minimum file size filter (empty string means no minimum)
    pub min_file_size: String,
    /// Maximum file size filter (None means no maximum)
    pub max_file_size: Option<String>,
    /// Number of concurrent downloads (1-16)
    pub concurrent_downloads: usize,
    /// Enable HTTP compression during downloads
    pub enable_compression: bool,
    /// Automatically decompress downloaded files
    pub auto_decompress: bool,
    /// Compression formats to auto-decompress
    pub decompress_formats: Vec<String>,
    /// Dry run mode (preview only, don't download)
    pub dry_run: bool,
    /// Verify MD5 checksums
    pub verify_md5: bool,
    /// Preserve file modification times
    pub preserve_mtime: bool,
    /// Enable verbose output
    pub verbose: bool,
    /// Enable resume capability
    pub resume: bool,
}

impl Default for DownloadRequest {
    fn default() -> Self {
        Self {
            identifier: String::new(),
            output_dir: std::env::current_dir().unwrap_or_default(),
            include_formats: Vec::new(),
            exclude_formats: Vec::new(),
            min_file_size: String::new(),
            max_file_size: None,
            concurrent_downloads: 4,
            enable_compression: true,
            auto_decompress: false,
            decompress_formats: Vec::new(),
            dry_run: false,
            verify_md5: true,
            preserve_mtime: true,
            verbose: false,
            resume: true,
        }
    }
}

impl DownloadRequest {
    /// Create a new download request from a Config object and identifier
    pub fn from_config(config: &Config, identifier: String, output_dir: PathBuf) -> Self {
        Self {
            identifier,
            output_dir,
            concurrent_downloads: config.concurrent_downloads,
            enable_compression: config.default_compress,
            auto_decompress: config.default_decompress,
            decompress_formats: config.default_decompress_formats
                .as_ref()
                .map(|formats| {
                    formats
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                })
                .unwrap_or_default(),
            dry_run: config.default_dry_run,
            verbose: config.default_verbose,
            resume: config.default_resume,
            ..Default::default()
        }
    }

    /// Parse size strings into u64 values
    pub fn get_parsed_sizes(&self) -> Result<(Option<u64>, Option<u64>)> {
        let min_size = if self.min_file_size.is_empty() {
            None
        } else {
            Some(parse_size_string(&self.min_file_size)?)
        };

        let max_size = self.max_file_size
            .as_ref()
            .map(|s| parse_size_string(s))
            .transpose()?;

        Ok((min_size, max_size))
    }
}

/// Progress update callback function type
pub type ProgressCallback = Box<dyn Fn(ProgressUpdate) + Send + Sync>;

/// Progress update information
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

/// Download operation result
#[derive(Debug)]
pub enum DownloadResult {
    Success(Box<DownloadSession>),
    Error(String),
}

/// Unified download service that both CLI and GUI use
pub struct DownloadService {
    client: Client,
}

impl DownloadService {
    /// Create a new download service
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent(get_user_agent())
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| IaGetError::Network(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { client })
    }

    /// Execute a download request
    pub async fn download(
        &self,
        request: DownloadRequest,
        progress_callback: Option<ProgressCallback>,
    ) -> Result<DownloadResult> {
        // Send initial status
        if let Some(ref callback) = progress_callback {
            callback(ProgressUpdate {
                current_file: String::new(),
                completed_files: 0,
                total_files: 0,
                failed_files: 0,
                current_speed: 0.0,
                eta: String::new(),
                status: "Fetching metadata...".to_string(),
            });
        }

        // Construct archive URL
        let archive_url = if request.identifier.starts_with("http") {
            request.identifier.clone()
        } else {
            format!("https://archive.org/details/{}", request.identifier)
        };

        // Fetch metadata
        let progress = indicatif::ProgressBar::new_spinner();
        let (metadata, _base_url) =
            match fetch_json_metadata(&archive_url, &self.client, &progress).await {
                Ok(result) => result,
                Err(e) => {
                    return Ok(DownloadResult::Error(format!(
                        "Failed to fetch metadata: {}",
                        e
                    )));
                }
            };

        // Parse file size filters
        let (min_size, max_size) = request.get_parsed_sizes()?;

        // Apply file filters
        let filtered_files = self.apply_file_filters(
            &metadata.files,
            &request.include_formats,
            &request.exclude_formats,
            min_size,
            max_size,
        );

        if filtered_files.is_empty() {
            return Ok(DownloadResult::Error(
                "No files match the specified filters".to_string(),
            ));
        }

        // Send file count update
        if let Some(ref callback) = progress_callback {
            callback(ProgressUpdate {
                current_file: String::new(),
                completed_files: 0,
                total_files: filtered_files.len(),
                failed_files: 0,
                current_speed: 0.0,
                eta: String::new(),
                status: format!("Found {} files to download", filtered_files.len()),
            });
        }

        if request.dry_run {
            // For dry run, just return the file list information
            return Ok(DownloadResult::Success(Box::new(DownloadSession {
                original_url: archive_url.clone(),
                identifier: request.identifier.clone(),
                archive_metadata: metadata,
                download_config: DownloadConfig {
                    output_dir: request.output_dir.to_string_lossy().to_string(),
                    max_concurrent: request.concurrent_downloads as u32,
                    format_filters: request.include_formats,
                    min_size,
                    max_size,
                    verify_md5: request.verify_md5,
                    preserve_mtime: request.preserve_mtime,
                    user_agent: get_user_agent(),
                    enable_compression: request.enable_compression,
                    auto_decompress: request.auto_decompress,
                    decompress_formats: request.decompress_formats.clone(),
                },
                requested_files: filtered_files.iter().map(|f| f.name.clone()).collect(),
                file_status: std::collections::HashMap::new(),
                session_start: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                last_updated: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            })));
        }

        // Create download configuration
        let download_config = DownloadConfig {
            output_dir: request.output_dir.to_string_lossy().to_string(),
            max_concurrent: request.concurrent_downloads as u32,
            format_filters: request.include_formats.clone(),
            min_size,
            max_size,
            verify_md5: request.verify_md5,
            preserve_mtime: request.preserve_mtime,
            user_agent: get_user_agent(),
            enable_compression: request.enable_compression,
            auto_decompress: request.auto_decompress,
            decompress_formats: request.decompress_formats.clone(),
        };

        // Create session directory
        let session_dir = request.output_dir.join(".ia-get-sessions");

        // Initialize the archive downloader
        let downloader = ArchiveDownloader::new(
            self.client.clone(),
            request.concurrent_downloads,
            request.verify_md5,
            request.preserve_mtime,
            session_dir,
            request.enable_compression,
            request.auto_decompress,
        );

        // Get list of file names to download
        let requested_files: Vec<String> = filtered_files.iter().map(|f| f.name.clone()).collect();

        // Send download start status
        if let Some(ref callback) = progress_callback {
            callback(ProgressUpdate {
                current_file: String::new(),
                completed_files: 0,
                total_files: filtered_files.len(),
                failed_files: 0,
                current_speed: 0.0,
                eta: String::new(),
                status: "Starting download...".to_string(),
            });
        }

        // Create a simple progress bar for the download operation
        let progress_bar = indicatif::ProgressBar::new(filtered_files.len() as u64);

        // Execute download
        match downloader
            .download_with_metadata(
                archive_url,
                request.identifier,
                metadata,
                download_config,
                requested_files,
                &progress_bar,
            )
            .await
        {
            Ok(session) => Ok(DownloadResult::Success(Box::new(session))),
            Err(e) => Ok(DownloadResult::Error(format!("Download failed: {}", e))),
        }
    }

    /// Apply file filters to the list of archive files
    fn apply_file_filters(
        &self,
        files: &[ArchiveFile],
        include_formats: &[String],
        exclude_formats: &[String],
        min_file_size: Option<u64>,
        max_file_size: Option<u64>,
    ) -> Vec<ArchiveFile> {
        files
            .iter()
            .filter(|file| {
                // Apply include format filter
                if !include_formats.is_empty() {
                    let file_format = file.format.as_deref().unwrap_or("");
                    let file_extension = std::path::Path::new(&file.name)
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .unwrap_or("");

                    let matches_include = include_formats.iter().any(|fmt| {
                        fmt.eq_ignore_ascii_case(file_format)
                            || fmt.eq_ignore_ascii_case(file_extension)
                    });

                    if !matches_include {
                        return false;
                    }
                }

                // Apply exclude format filter
                if !exclude_formats.is_empty() {
                    let file_format = file.format.as_deref().unwrap_or("");
                    let file_extension = std::path::Path::new(&file.name)
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .unwrap_or("");

                    let matches_exclude = exclude_formats.iter().any(|fmt| {
                        fmt.eq_ignore_ascii_case(file_format)
                            || fmt.eq_ignore_ascii_case(file_extension)
                    });

                    if matches_exclude {
                        return false;
                    }
                }

                let file_size = file.size.unwrap_or(0);

                // Apply min size filter
                if let Some(min_size) = min_file_size {
                    if file_size < min_size {
                        return false;
                    }
                }

                // Apply max size filter
                if let Some(max_size) = max_file_size {
                    if file_size > max_size {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect()
    }

    /// Display download summary for CLI usage
    pub fn display_download_summary(session: &DownloadSession, request: &DownloadRequest) {
        use colored::Colorize;
        
        let completed_files = session
            .file_status
            .values()
            .filter(|status| {
                matches!(
                    status.status,
                    crate::metadata_storage::DownloadState::Completed
                )
            })
            .count();
        let total_files = session.file_status.len();
        let total_bytes: u64 = session
            .file_status
            .values()
            .filter(|status| {
                matches!(
                    status.status,
                    crate::metadata_storage::DownloadState::Completed
                )
            })
            .map(|status| status.file_info.size.unwrap_or(0))
            .sum();

        println!("\n{} Download Summary:", "📋".blue().bold());
        println!("  📂 Archive: {}", session.identifier);
        println!(
            "  📁 Output directory: {}",
            request.output_dir.display().to_string().bright_green()
        );
        println!("  📊 Files downloaded: {}/{}", completed_files, total_files);
        println!(
            "  💾 Total size: {}",
            format_size(total_bytes).bright_blue()
        );

        if completed_files < total_files {
            println!("\n{} Some files were not downloaded:", "⚠️".yellow());
            for (filename, status) in &session.file_status {
                if !matches!(
                    status.status,
                    crate::metadata_storage::DownloadState::Completed
                ) {
                    println!("  • {} - {:?}", filename, status.status);
                }
            }
            println!(
                "\n💡 Use {} to retry failed downloads",
                "--resume".bright_blue()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_request_default() {
        let request = DownloadRequest::default();
        assert_eq!(request.concurrent_downloads, 4);
        assert!(request.enable_compression);
        assert!(!request.auto_decompress);
        assert!(request.verify_md5);
        assert!(request.preserve_mtime);
        assert!(!request.dry_run);
    }

    #[test]
    fn test_download_request_from_config() {
        let config = Config {
            concurrent_downloads: 8,
            default_compress: false,
            default_decompress: true,
            default_dry_run: true,
            default_verbose: true,
            default_resume: false,
            ..Default::default()
        };

        let request = DownloadRequest::from_config(
            &config,
            "test-archive".to_string(),
            PathBuf::from("/tmp/test"),
        );

        assert_eq!(request.identifier, "test-archive");
        assert_eq!(request.output_dir, PathBuf::from("/tmp/test"));
        assert_eq!(request.concurrent_downloads, 8);
        assert!(!request.enable_compression);
        assert!(request.auto_decompress);
        assert!(request.dry_run);
        assert!(request.verbose);
        assert!(!request.resume);
    }

    #[test]
    fn test_parse_sizes() {
        let mut request = DownloadRequest::default();
        request.min_file_size = "1MB".to_string();
        request.max_file_size = Some("100MB".to_string());

        let (min_size, max_size) = request.get_parsed_sizes().unwrap();
        assert_eq!(min_size, Some(1_048_576)); // 1 MB in binary (1024^2)
        assert_eq!(max_size, Some(104_857_600)); // 100 MB in binary (100 * 1024^2)
    }
}