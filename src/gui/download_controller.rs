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

/// Download request parameters
#[derive(Debug, Clone)]
pub struct DownloadRequest {
    pub identifier: String,
    pub output_dir: PathBuf,
    pub include_formats: Vec<String>,
    pub exclude_formats: Vec<String>,
    pub min_file_size: String,
    pub max_file_size: Option<String>,
    pub dry_run: bool,
    pub decompress_formats: Vec<String>,
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
        request: DownloadRequest,
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

        // Apply file filters
        let max_size = request.max_file_size
            .as_ref()
            .and_then(|s| parse_size_string(s).ok());
        let min_size = if request.min_file_size.is_empty() {
            None
        } else {
            parse_size_string(&request.min_file_size).ok()
        };

        let filtered_files = self.apply_file_filters(
            &metadata.files, 
            &request.include_formats, 
            &request.exclude_formats,
            min_size,
            max_size
        );

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

        if request.dry_run {
            // For dry run, just return the file list information
            return Ok(DownloadResult::Success(Box::new(DownloadSession {
                original_url: archive_url.clone(),
                identifier: request.identifier.clone(),
                archive_metadata: metadata,
                download_config: DownloadConfig {
                    output_dir: request.output_dir.to_string_lossy().to_string(),
                    max_concurrent: self.config.concurrent_downloads as u32,
                    format_filters: request.include_formats,
                    min_size,
                    max_size,
                    verify_md5: true,
                    preserve_mtime: true,
                    user_agent: get_user_agent(),
                    enable_compression: self.config.default_compress,
                    auto_decompress: self.config.default_decompress,
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
            max_concurrent: self.config.concurrent_downloads as u32,
            format_filters: request.include_formats.clone(),
            min_size,
            max_size,
            verify_md5: true,
            preserve_mtime: true,
            user_agent: get_user_agent(),
            enable_compression: self.config.default_compress,
            auto_decompress: self.config.default_decompress,
            decompress_formats: request.decompress_formats.clone(),
        };

        // Create session directory
        let session_dir = request.output_dir.join(".ia-get-sessions");

        // Initialize the archive downloader
        let downloader = ArchiveDownloader::new(
            self.client.clone(),
            self.config.concurrent_downloads,
            true, // verify_md5
            true, // preserve_mtime
            session_dir,
            self.config.default_compress,
            self.config.default_decompress,
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
}
