//! Batch download operations for Internet Archive
//!
//! Supports downloading multiple archives from a file list with parallel processing,
//! progress tracking, and resume capabilities.

use crate::error::{IaGetError, Result};
use colored::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use tokio::sync::Semaphore;

/// Batch download configuration
pub struct BatchConfig {
    pub input_file: String,
    pub output_dir: Option<String>,
    pub parallel: usize,
    pub resume: bool,
    pub dry_run: bool,
}

/// Result of a single item in batch
#[derive(Debug)]
pub struct BatchItemResult {
    pub identifier: String,
    pub success: bool,
    pub error: Option<String>,
    pub files_downloaded: usize,
}

/// Execute batch download operation
pub async fn batch_download(config: BatchConfig) -> Result<Vec<BatchItemResult>> {
    println!("\n{}", "=".repeat(80).cyan());
    println!("{}", "Batch Download Operation".bright_cyan().bold());
    println!("{}", "=".repeat(80).cyan());

    // Read identifiers from file
    let identifiers = read_identifiers(&config.input_file)?;
    println!(
        "{} {} identifiers to process",
        "📋".bright_blue(),
        identifiers.len()
    );

    if config.dry_run {
        println!(
            "{} {}",
            "🔍".yellow(),
            "Dry run mode - no files will be downloaded".yellow()
        );
    }

    // Create semaphore for parallel processing
    let semaphore = Arc::new(Semaphore::new(config.parallel));
    let mut handles = vec![];

    println!(
        "\n{} Starting downloads with {} parallel workers...\n",
        "🚀".green(),
        config.parallel
    );

    // Spawn tasks for each identifier
    for (idx, identifier) in identifiers.iter().enumerate() {
        let sem = Arc::clone(&semaphore);
        let id = identifier.clone();
        let output = config.output_dir.clone();
        let resume = config.resume;
        let dry_run = config.dry_run;
        let total = identifiers.len();

        let handle = tokio::spawn(async move {
            let _permit = match sem.acquire().await {
                Ok(permit) => permit,
                Err(e) => {
                    return BatchItemResult {
                        identifier: id,
                        success: false,
                        error: Some(format!("Failed to acquire download slot: {}", e)),
                        files_downloaded: 0,
                    };
                }
            };

            println!(
                "{} [{}/{}] Processing: {}",
                "⬇️".cyan(),
                idx + 1,
                total,
                id.bright_white()
            );

            if dry_run {
                // Simulate download
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                BatchItemResult {
                    identifier: id,
                    success: true,
                    error: None,
                    files_downloaded: 0,
                }
            } else {
                // Actual download
                match download_single_archive(&id, output.as_deref(), resume).await {
                    Ok(count) => {
                        println!(
                            "{} [{}/{}] Completed: {} ({} files)",
                            "✅".green(),
                            idx + 1,
                            total,
                            id.bright_white(),
                            count
                        );
                        BatchItemResult {
                            identifier: id,
                            success: true,
                            error: None,
                            files_downloaded: count,
                        }
                    }
                    Err(e) => {
                        println!(
                            "{} [{}/{}] Failed: {} - {}",
                            "❌".red(),
                            idx + 1,
                            total,
                            id.bright_white(),
                            e.to_string().red()
                        );
                        BatchItemResult {
                            identifier: id,
                            success: false,
                            error: Some(e.to_string()),
                            files_downloaded: 0,
                        }
                    }
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all downloads to complete
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await?);
    }

    // Print summary
    print_batch_summary(&results);

    Ok(results)
}

/// Read identifiers from file
fn read_identifiers(file_path: &str) -> Result<Vec<String>> {
    let file = File::open(file_path)
        .map_err(|e| IaGetError::FileSystem(format!("Failed to open file {}: {}", file_path, e)))?;

    let reader = BufReader::new(file);
    let mut identifiers = Vec::new();

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Extract identifier from URL or use as-is
        let identifier = if line.starts_with("http") {
            crate::utilities::common::extract_identifier_from_url(line).map_err(|e| {
                IaGetError::UrlFormat(format!(
                    "Invalid URL on line {}: {} ({})",
                    line_num + 1,
                    line,
                    e
                ))
            })?
        } else {
            line.to_string()
        };

        identifiers.push(identifier);
    }

    Ok(identifiers)
}

/// Download a single archive via the unified download service, returning the
/// number of files that completed successfully.
async fn download_single_archive(
    identifier: &str,
    output_dir: Option<&str>,
    resume: bool,
) -> Result<usize> {
    use crate::core::download::{DownloadRequest, DownloadResult, DownloadService};
    use crate::core::session::{DownloadState, sanitize_filename_for_filesystem};

    let service = DownloadService::new()?;

    let resolved_output = output_dir.map(std::path::PathBuf::from).unwrap_or_else(|| {
        let mut current = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        current.push(sanitize_filename_for_filesystem(identifier));
        current
    });

    let request = DownloadRequest {
        identifier: identifier.to_string(),
        output_dir: resolved_output,
        resume,
        ..Default::default()
    };

    match service.download(request, None).await? {
        DownloadResult::Success(session, _stats, _dry_run) => {
            let completed = session
                .file_status
                .values()
                .filter(|status| matches!(status.status, DownloadState::Completed))
                .count();
            Ok(completed)
        }
        DownloadResult::Error(message) => {
            Err(IaGetError::Network(format!("Download failed: {}", message)))
        }
    }
}

/// Print summary of batch operation
fn print_batch_summary(results: &[BatchItemResult]) {
    println!("\n{}", "=".repeat(80).cyan());
    println!("{}", "Batch Download Summary".bright_cyan().bold());
    println!("{}", "=".repeat(80).cyan());

    let total = results.len();
    let successful = results.iter().filter(|r| r.success).count();
    let failed = results.iter().filter(|r| !r.success).count();
    let total_files: usize = results.iter().map(|r| r.files_downloaded).sum();

    println!("{} Total identifiers: {}", "📊".bright_blue(), total);
    println!(
        "{} Successful: {}",
        "✅".green(),
        successful.to_string().green()
    );
    println!("{} Failed: {}", "❌".red(), failed.to_string().red());
    println!(
        "{} Total files downloaded: {}",
        "📦".bright_blue(),
        total_files
    );

    if failed > 0 {
        println!("\n{} Failed identifiers:", "❌".red().bold());
        for result in results.iter().filter(|r| !r.success) {
            println!(
                "  {} - {}",
                result.identifier.yellow(),
                result.error.as_deref().unwrap_or("Unknown error").red()
            );
        }
    }

    println!();
}
