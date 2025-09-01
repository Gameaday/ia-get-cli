//! Tests for GUI components

use crate::config::Config;
use crate::gui::download_controller::DownloadController;
use crate::gui::panels::{ConfigPanel, DownloadPanel, FiltersPanel};

#[test]
fn test_gui_panels_creation() {
    // Test that all GUI panels can be created successfully
    let config = Config::default();

    let _config_panel = ConfigPanel::new(config.clone());
    let _download_panel = DownloadPanel::new();
    let _filters_panel = FiltersPanel::new();
}

#[test]
fn test_download_controller_creation() {
    // Test that download controller can be created
    let config = Config::default();

    // This should succeed even in test environment
    let result = DownloadController::new(config);
    assert!(
        result.is_ok(),
        "Download controller should be created successfully"
    );
}

#[test]
fn test_filter_settings() {
    // Test filter panel functionality
    let filters_panel = FiltersPanel::new();

    // Test getting empty filter settings
    let (include, exclude, min_size, max_size) = filters_panel.get_filter_settings();
    assert!(include.is_empty());
    assert!(exclude.is_empty());
    assert!(min_size.is_empty());
    assert!(max_size.is_empty());
}
