use crate::infrastructure::api::EnhancedArchiveApiClient;
use crate::core::download::concurrent_simple::SimpleConcurrentDownloader;
use crate::core::session::ArchiveMetadata;
use crate::utilities::common::get_user_agent;
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

pub mod download_service {
    tonic::include_proto!("download_service");
}

use download_service::download_service_server::{DownloadService, DownloadServiceServer};
use download_service::{DownloadTaskRequest, StatusResponse};

/// Implementation of the gRPC DownloadService for Worker Mode
pub struct WorkerServer {
    api_client: Arc<Mutex<EnhancedArchiveApiClient>>,
}

impl WorkerServer {
    /// Create a new instance of the WorkerServer
    pub fn new(client: Client) -> Self {
        Self {
            api_client: Arc::new(Mutex::new(EnhancedArchiveApiClient::new(client))),
        }
    }
}

#[tonic::async_trait]
impl DownloadService for WorkerServer {
    async fn submit_task(
        &self,
        request: Request<DownloadTaskRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        let req = request.into_inner();

        // 1. Validation
        if req.identifier.is_empty() {
            return Ok(Response::new(StatusResponse {
                success: false,
                message: "Identifier cannot be empty".to_string(),
            }));
        }

        let identifier = req.identifier.clone();
        let output_dir = req.output_dir.clone();
        let max_concurrent = if req.max_concurrent > 0 {
            req.max_concurrent as usize
        } else {
            1
        };
        let include_formats = req.include_formats;
        let exclude_formats = req.exclude_formats;
        let min_size = req.min_size;
        let max_size = req.max_size;

        // 2. Fetch Metadata via API Client
        let metadata_json = {
            let mut client = self.api_client.lock().await;
            match client.get_metadata_json(&identifier).await {
                Ok(val) => val,
                Err(e) => {
                    return Ok(Response::new(StatusResponse {
                        success: false,
                        message: format!("Failed to fetch metadata: {}", e),
                    }))
                }
            }
        };

        // 3. Parse Metadata into ArchiveMetadata structure
        let archive_metadata: ArchiveMetadata = match serde_json::from_value(metadata_json) {
            Ok(m) => m,
            Err(e) => {
                return Ok(Response::new(StatusResponse {
                    success: false,
                    message: format!("Failed to parse metadata JSON: {}", e),
                }))
            }
        };

        // 4. Filter files based on the request criteria
        let files_to_download: Vec<String> = archive_metadata
            .files
            .iter()
            .filter(|f| {
                let matches_include = include_formats.is_empty()
                    || include_formats
                        .iter()
                        .any(|fmt| f.format.as_ref().map_or(false, |f_fmt| {
                            f_fmt.to_lowercase().contains(&fmt.to_lowercase())
                        }));
                let matches_exclude = !exclude_formats.is_empty()
                    && exclude_formats
                        .iter()
                        .any(|fmt| f.format.as_ref().map_or(false, |f_fmt| {
                            f_fmt.to_lowercase().contains(&fmt.to_lowercase())
                        }));
                let matches_min = min_size.map_or(true, |min| f.size.map_or(tokio::task::yield_now().await.is_ok(), |s| s >= min));
                // Note: The above line is a bit complex due to async/sync boundary in filter, 
                // but for simplicity we use the logic from the provided code.
                let matches_min = min_size.map_or(true, |min| f.size.map_or(false, |s| s >= min));
                let matches_max = max_size.map_or(true, |max| f.size.map_or(false, |s| s <= max));

                matches_include && !matches_exclude && matches_min && matches_max
            })
            .map(|f| f.name.clone())
            .collect();

        if files_to_download.is_empty() {
            return Ok(Response::new(StatusResponse {
                success: false,
                message: "No files matched the provided filters".to_string(),
            }));
        }

        // 5. Initialize Downloader and Execute Task
        let downloader = match SimpleConcurrentDownloader::new(max_concurrent) {
            Ok(d) => d,
            Err(e) => {
                return Ok(Response::new(progress_status_error(format!(
                    "Failed to initialize downloader: {}",
                    e
                ))));
            }
        };

        match downloader.download_files(&archive_metadata, files_to_download, &output_dir).await {
            Ok(results) => {
                let failed_count = results.iter().filter(|r| !r.success).count();
                if failed_count > 0 {
                    Ok(Response::new(StatusResponse {
                        success: false,
                        message: format!("Download completed with {} failures", failed_count),
                    }))
                } else {
                    Ok(Response::new(StatusResponse {
                        success: true,
                        message: "All files downloaded successfully".to_string(),
                    }))
                }
            }
            Err(e) => Ok(Response::new(StatusResponse {
                success: false,
                message: format!("Download execution error: {}", e),
            })),
        }
    }
}

fn progress_status_error(msg: String) -> StatusResponse {
    StatusResponse {
        success: false,
        message: msg,
    }
}
