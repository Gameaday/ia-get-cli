//! User interface layer
//!
//! This module contains all user interface components including CLI, GUI, interactive interfaces,
//! and FFI bindings for mobile platforms.

pub mod cli;
// Temporarily disabled - being replaced by ffi_simple in Phase 2
// #[cfg(feature = "ffi")]
// pub mod ffi;
#[cfg(feature = "ffi")]
pub mod ffi_simple; // NEW: Simplified FFI interface
#[cfg(feature = "gui")]
pub mod gui;
pub mod interactive;

// Re-export commonly used interface types
pub use cli::*;
// Temporarily disabled - being replaced by ffi_simple in Phase 2
// #[cfg(feature = "ffi")]
// pub use ffi::*;
#[cfg(feature = "gui")]
pub use gui::*;
pub use interactive::*;
