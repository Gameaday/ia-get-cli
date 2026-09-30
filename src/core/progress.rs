//! UI-agnostic progress reporting.
//!
//! Long-running operations in `core` and `infrastructure` describe their progress
//! by emitting [`ProgressEvent`]s to a [`ProgressReporter`]. This keeps the
//! download engine and metadata code free of terminal concerns: the CLI/TUI
//! provide concrete reporters (for example an `indicatif` bar), library users can
//! supply their own, and [`NoopReporter`] silences everything.

use std::sync::Arc;

/// An event describing the progress of a long-running operation.
#[derive(Debug, Clone)]
pub enum ProgressEvent {
    /// A download session is starting.
    SessionStarted {
        /// Total number of files that will be processed.
        total_files: usize,
        /// Total bytes across those files, when known.
        total_bytes: u64,
    },
    /// A file is about to be processed.
    FileStarted {
        /// File name.
        name: String,
        /// Expected size in bytes, when known.
        size: Option<u64>,
    },
    /// Bytes downloaded so far for the current file.
    FileProgress {
        /// File name.
        name: String,
        /// Bytes downloaded so far.
        downloaded: u64,
        /// Total bytes expected, when known.
        total: Option<u64>,
    },
    /// A file finished successfully.
    FileCompleted {
        /// File name.
        name: String,
        /// Bytes written.
        bytes: u64,
        /// Whether the file was skipped (already present and valid).
        skipped: bool,
    },
    /// A file failed.
    FileFailed {
        /// File name.
        name: String,
        /// Human-readable error.
        error: String,
    },
    /// A free-form status message.
    Message(String),
    /// The session finished.
    SessionCompleted {
        /// Number of files completed successfully.
        completed: usize,
        /// Number of files that failed.
        failed: usize,
    },
}

/// Receives [`ProgressEvent`]s from long-running operations.
///
/// Implementations should be cheap and non-blocking: the download engine calls
/// `report` from hot paths. All implementations must be `Send + Sync` because
/// events are emitted from spawned tasks.
pub trait ProgressReporter: Send + Sync {
    /// Report a progress event.
    fn report(&self, event: ProgressEvent);

    /// Convenience for reporting a [`ProgressEvent::Message`].
    fn message(&self, message: String) {
        self.report(ProgressEvent::Message(message));
    }
}

/// A shared, type-erased reporter.
pub type SharedReporter = Arc<dyn ProgressReporter>;

/// A reporter that ignores every event.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopReporter;

impl ProgressReporter for NoopReporter {
    fn report(&self, _event: ProgressEvent) {}
}
