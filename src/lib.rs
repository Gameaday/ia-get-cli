// Allow collapsible_if for now - can be refactored in separate PR
#![allow(clippy::collapsible_if)]

//! # ia-get
//!
//! A robust command-line tool for downloading files from the Internet Archive.
//!
//! ## Features
//!
//! - **Concurrent Downloads**: Fast parallel downloading with configurable concurrency limits
//! - **JSON API Integration**: Uses Internet Archive's modern JSON API for metadata
//! - **Session Management**: Resumable downloads with automatic session tracking
//! - **Compression Support**: Automatic decompression of common archive formats
//! - **Progress Tracking**: Real-time download progress and statistics
//! - **Error Handling**: Robust retry logic and comprehensive error reporting
//! - **Filtering**: Download specific files by format, size, or pattern
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use ia_get::{core::archive::fetch_json_metadata, core::download::ArchiveDownloader};
//! use reqwest::Client;
//! use indicatif::ProgressBar;
//! use std::path::PathBuf;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new();
//!     let progress = ProgressBar::new_spinner();
//!     
//!     // Fetch archive metadata
//!     let (metadata, _url) = fetch_json_metadata("identifier", &client, &progress, None).await?;
//!
//!     // Download with enhanced features
//!     let downloader = ArchiveDownloader::new(
//!         client, 4, true, true, PathBuf::from(".sessions"), false, false
//!     );
//!     // Use the downloader as needed...
//!     Ok(())
//! }
//! ```
//!
//! ## Architecture
//!
//! - [`metadata`]: JSON metadata fetching and parsing
//! - [`enhanced_downloader`]: Main download engine with session support
//! - [`metadata_storage`]: Session and file tracking structures
//! - [`compression`]: Automatic decompression utilities
//! - [`filters`]: File filtering and formatting utilities

// Organized module structure
pub mod core;
pub mod error;
pub mod infrastructure;
pub mod interface;
pub mod utilities;

// Re-export the error types for convenience
pub use error::{IaGetError, Result};

// Re-export commonly used functions from organized modules
pub use core::archive::{
    AdvancedMetadataProcessor, MetadataAnalysis, fetch_json_metadata, get_json_url,
    parse_archive_metadata,
};
pub use core::download::{DownloadRequest, DownloadResult, DownloadService};
pub use core::session::{
    ArchiveFile, ArchiveMetadata, DownloadConfig, DownloadSession, DownloadState, ProgressUpdate,
    sanitize_filename_for_filesystem,
};
pub use infrastructure::api::{
    ApiStats, ArchiveOrgApiClient, EnhancedArchiveApiClient, ItemDetails, ServiceStatus,
    validate_identifier,
};
pub use infrastructure::http::{
    ClientConfig, EnhancedHttpClient, HttpClientFactory, is_transient_error,
    is_transient_reqwest_error, is_url_accessible,
};
pub use interface::cli::{Cli, SourceType};
pub use utilities::common::{
    AdaptiveBufferManager, PerformanceMetrics, PerformanceMonitor, StringTruncate,
    construct_download_url, construct_metadata_url, extract_identifier_from_url, format_number,
    get_user_agent, is_archive_url, normalize_archive_identifier, validate_and_process_url,
};
pub use utilities::compression::*;
pub use utilities::filters::{
    FileFormats, FormatCategory, filter_files, format_size, parse_size_string,
};

// Legacy compatibility re-exports for external tests and examples
pub mod metadata {
    pub use crate::core::archive::*;
}

pub mod metadata_storage {
    pub use crate::core::session::*;
}

pub mod url_processing {
    pub use crate::utilities::common::*;
}

pub mod constants {
    pub use crate::utilities::common::*;
}

pub mod cli {
    pub use crate::interface::cli::*;
}

pub mod archive_metadata {
    pub use crate::core::archive::*;
}

pub mod filters {
    pub use crate::utilities::filters::*;
}

pub mod file_formats {
    pub use crate::utilities::filters::*;
}

pub mod progress {
    pub use crate::utilities::common::*;
}

pub mod compression {
    pub use crate::utilities::compression::*;
}
