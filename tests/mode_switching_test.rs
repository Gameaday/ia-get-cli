//! Test interactive terminal UI (TUI) launch components
//!
//! These tests verify the CLI/TUI interface components are available without
//! requiring a display or an interactive terminal.

#[test]
fn test_interactive_cli_type_exists() {
    // Compile-time check that the interactive CLI type and API exist.
    use ia_get::interface::interactive::InteractiveCli;

    let _phantom: Option<InteractiveCli> = None;
}

#[test]
fn test_terminal_flush_functions_exist() {
    use std::io::{self, Write};

    assert!(io::stdout().flush().is_ok(), "stdout flush should work");
    assert!(io::stderr().flush().is_ok(), "stderr flush should work");
}

#[test]
fn test_config_manager_initializes() {
    let config_manager_result = ia_get::infrastructure::config::ConfigManager::new();
    assert!(
        config_manager_result.is_ok(),
        "ConfigManager should initialize"
    );
}
