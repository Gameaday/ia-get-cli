//! Bridge module to connect GUI with core download functionality

use crate::{
    config::Config,
    constants::get_user_agent,
    enhanced_downloader::ArchiveDownloader,
    fetch_json_metadata,
    filters::parse_size_string,
    metadata_storage::{ArchiveFile, DownloadConfig, DownloadSession},
    IaGetError, Result,
};
use reqwest::Client;
use std::path::PathBuf;
use tokio::sync::mpsc;

/// Progress update message from download operation
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

/// GUI download controller
pub struct DownloadController {
    client: Client,
    config: Config,
}

impl DownloadController {
    /// Create a new download controller
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::builder()
            .user_agent(get_user_agent())
            .timeout(std::time::Duration::from_secs(config.http_timeout))
            .build()
            .map_err(|e| IaGetError::Network(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { client, config })
    }

    /// Start a download operation with progress updates
    pub async fn start_download(
        &self,
        identifier: String,
        output_dir: PathBuf,
        include_formats: Vec<String>,
        max_file_size: Option<String>,
        dry_run: bool,
        progress_tx: mpsc::UnboundedSender<ProgressUpdate>,
    ) -> Result<DownloadResult> {
        // Send initial status
        let _ = progress_tx.send(ProgressUpdate {
            current_file: String::new(),
            completed_files: 0,
            total_files: 0,
            failed_files: 0,
            current_speed: 0.0,
            eta: String::new(),
            status: "Fetching metadata...".to_string(),
        });

        // Construct archive URL
        let archive_url = if identifier.starts_with("http") {
            identifier.clone()
        } else {
            format!("https://archive.org/details/{}", identifier)
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

        // Apply file filters
        let max_size = max_file_size
            .as_ref()
            .and_then(|s| parse_size_string(s).ok());

        let filtered_files = self.apply_file_filters(&metadata.files, &include_formats, max_size);

        if filtered_files.is_empty() {
            return Ok(DownloadResult::Error(
                "No files match the specified filters".to_string(),
            ));
        }

        // Send file count update
        let _ = progress_tx.send(ProgressUpdate {
            current_file: String::new(),
            completed_files: 0,
            total_files: filtered_files.len(),
            failed_files: 0,
            current_speed: 0.0,
            eta: String::new(),
            status: format!("Found {} files to download", filtered_files.len()),
        });

        if dry_run {
            // For dry run, just return the file list information
            return Ok(DownloadResult::Success(Box::new(DownloadSession {
                original_url: archive_url.clone(),
                identifier: identifier.clone(),
                archive_metadata: metadata,
                download_config: DownloadConfig {
                    output_dir: output_dir.to_string_lossy().to_string(),
                    max_concurrent: self.config.concurrent_downloads as u32,
                    format_filters: include_formats,
                    min_size: None,
                    max_size,
                    verify_md5: true,
                    preserve_mtime: true,
                    user_agent: get_user_agent(),
                    enable_compression: true,
                    auto_decompress: false,
                    decompress_formats: vec![],
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
            output_dir: output_dir.to_string_lossy().to_string(),
            max_concurrent: self.config.concurrent_downloads as u32,
            format_filters: include_formats.clone(),
            min_size: None,
            max_size,
            verify_md5: true,
            preserve_mtime: true,
            user_agent: get_user_agent(),
            enable_compression: true,
            auto_decompress: false,
            decompress_formats: vec![],
        };

        // Create session directory
        let session_dir = output_dir.join(".ia-get-sessions");

        // Initialize the archive downloader
        let downloader = ArchiveDownloader::new(
            self.client.clone(),
            self.config.concurrent_downloads,
            true, // verify_md5
            true, // preserve_mtime
            session_dir,
            true,  // enable_compression
            false, // auto_decompress
        );

        // Get list of file names to download
        let requested_files: Vec<String> = filtered_files.iter().map(|f| f.name.clone()).collect();

        // Send download start status
        let _ = progress_tx.send(ProgressUpdate {
            current_file: String::new(),
            completed_files: 0,
            total_files: filtered_files.len(),
            failed_files: 0,
            current_speed: 0.0,
            eta: String::new(),
            status: "Starting download...".to_string(),
        });

        // Create a simple progress bar for the download operation
        let progress_bar = indicatif::ProgressBar::new(filtered_files.len() as u64);

        // Execute download
        match downloader
            .download_with_metadata(
                archive_url,
                identifier,
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
        max_file_size: Option<u64>,
    ) -> Vec<ArchiveFile> {
        files
            .iter()
            .filter(|file| {
                // Apply format filter
                if !include_formats.is_empty() {
                    let file_format = file.format.as_deref().unwrap_or("");
                    let file_extension = std::path::Path::new(&file.name)
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .unwrap_or("");

                    let matches_format = include_formats.iter().any(|fmt| {
                        fmt.eq_ignore_ascii_case(file_format)
                            || fmt.eq_ignore_ascii_case(file_extension)
                    });

                    if !matches_format {
                        return false;
                    }
                }

                // Apply size filter
                if let Some(max_size) = max_file_size {
                    if file.size.unwrap_or(0) > max_size {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect()
    }
}
