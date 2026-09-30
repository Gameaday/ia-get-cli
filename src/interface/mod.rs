//! User interface layer
//!
//! This module contains all user interface components: the command-line
//! interface and the interactive terminal UI (TUI).

pub mod cli;
pub mod interactive;
pub mod progress;

// Re-export commonly used interface types
pub use cli::*;
pub use interactive::*;
pub use progress::IndicatifReporter;
