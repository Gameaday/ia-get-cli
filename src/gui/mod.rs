//! GUI module for ia-get
//!
//! Provides a graphical user interface wrapper around the core ia-get functionality
//! using egui for cross-platform compatibility.

pub mod app;
pub mod download_controller;
pub mod panels;

pub use app::IaGetApp;
pub use download_controller::{DownloadController, DownloadResult, ProgressUpdate};