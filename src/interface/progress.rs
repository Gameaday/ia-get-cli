//! `indicatif`-backed [`ProgressReporter`] for the CLI/TUI.
//!
//! This is the only place that turns [`ProgressEvent`]s into terminal output.

use crate::core::progress::{ProgressEvent, ProgressReporter};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

/// Renders progress events as `indicatif` progress bars.
pub struct IndicatifReporter {
    multi: MultiProgress,
    main: ProgressBar,
    active: Mutex<HashMap<String, ProgressBar>>,
}

impl IndicatifReporter {
    /// Create a new reporter with a main summary bar and per-file bars.
    pub fn new() -> Self {
        let multi = MultiProgress::new();
        let main = multi.add(ProgressBar::new(0));
        main.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>3}/{len:>3} files {msg}",
                )
                .expect("valid main progress template")
                .progress_chars("█▉▊▋▌▍▎▏ "),
        );

        Self {
            multi,
            main,
            active: Mutex::new(HashMap::new()),
        }
    }

    fn lock_active(&self) -> MutexGuard<'_, HashMap<String, ProgressBar>> {
        // Recover from a poisoned lock rather than panicking mid-download.
        self.active.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn file_bar(&self, name: &str, size: Option<u64>) -> ProgressBar {
        let bar = self.multi.add(ProgressBar::new(size.unwrap_or(0)));
        bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} {msg:30.30} [{bar:25.cyan/blue}] {bytes:>8}/{total_bytes:>8} {eta:>8}",
                )
                .expect("valid file progress template")
                .progress_chars("█▉▊▋▌▍▎▏ "),
        );
        bar.set_message(name.chars().take(30).collect::<String>());
        bar
    }
}

impl Default for IndicatifReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressReporter for IndicatifReporter {
    fn report(&self, event: ProgressEvent) {
        match event {
            ProgressEvent::SessionStarted {
                total_files,
                total_bytes,
            } => {
                self.main.set_length(total_files as u64);
                self.main.set_position(0);
                self.main.set_message(format!("{total_bytes} bytes total"));
            }
            ProgressEvent::FileStarted { name, size } => {
                let bar = self.file_bar(&name, size);
                self.lock_active().insert(name, bar);
            }
            ProgressEvent::FileProgress {
                name,
                downloaded,
                total,
            } => {
                if let Some(bar) = self.lock_active().get(&name) {
                    if let Some(total) = total {
                        bar.set_length(total);
                    }
                    bar.set_position(downloaded);
                }
            }
            ProgressEvent::FileCompleted {
                name,
                bytes,
                skipped,
            } => {
                if let Some(bar) = self.lock_active().remove(&name) {
                    let msg = if skipped {
                        format!("✓ {name} (already present)")
                    } else {
                        format!("✓ {name} ({bytes} bytes)")
                    };
                    bar.finish_with_message(msg);
                }
                self.main.inc(1);
            }
            ProgressEvent::FileFailed { name, error } => {
                if let Some(bar) = self.lock_active().remove(&name) {
                    bar.abandon_with_message(format!("✘ {name}: {error}"));
                } else {
                    eprintln!("✘ {name}: {error}");
                }
                self.main.inc(1);
            }
            ProgressEvent::Message(message) => {
                self.main.set_message(message);
            }
            ProgressEvent::SessionCompleted { completed, failed } => {
                self.main.finish_with_message(if failed == 0 {
                    format!("✓ Successfully downloaded {completed} files")
                } else {
                    format!("⚠ Completed {completed} files, {failed} failed")
                });
            }
        }
    }
}
