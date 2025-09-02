//! Main entry point for ia-get CLI application

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use reqwest::Client;
use std::path::{Path, PathBuf};
use tokio::signal;

use ia_get::{
    cli::{Cli, Commands},
    command_handlers::{handle_config_command, handle_history_command},
    constants::get_user_agent,
    download_history::{DownloadHistoryManager, DownloadRecord, DownloadStatus, generate_download_id},
    enhanced_downloader::ArchiveDownloader,
    fetch_json_metadata,
    filters::{format_size, parse_size_string},
    interactive_menu::InteractiveMenu,
    metadata_storage::{ArchiveFile, DownloadConfig, DownloadSession, DownloadState},
};

use indicatif::{ProgressBar, ProgressStyle};

/// Entry point for the ia-get CLI application
#[tokio::main]
async fn main() -> Result<()> {
    // Set up signal handling for graceful shutdown
    tokio::spawn(async {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        println!("\n{} Download interrupted by user", "⚠️".yellow());
        std::process::exit(0);
    });

    // Parse CLI with configuration defaults applied
    let cli = match Cli::with_config_defaults() {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("{} Failed to load configuration: {}", "⚠️".yellow(), e);
            eprintln!("Using built-in defaults...");
            Cli::parse()
        }
    };

    // Validate CLI arguments
    if let Err(e) = cli.validate() {
        eprintln!("{} {}", "❌".red(), e);
        std::process::exit(1);
    }

    // Handle subcommands
    match cli.command {
        Some(Commands::Config { action }) => {
            handle_config_command(action).await?;
            return Ok(());
        }
        Some(Commands::History { action }) => {
            handle_history_command(action).await?;
            return Ok(());
        }
        Some(Commands::Download { url, output, include_ext, exclude_ext, max_file_size, compress }) => {
            // Handle download subcommand
            let output_dir = match output {
                Some(path) => PathBuf::from(path),
                None => {
                    let identifier = extract_identifier_from_url(&url)?;
                    let mut current = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
                    current.push(identifier);
                    current
                }
            };

            let include_formats: Vec<String> = include_ext
                .map(|ext| ext.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_default();

            let exclude_formats: Vec<String> = exclude_ext
                .map(|ext| ext.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_default();

            let max_file_size_bytes = max_file_size
                .and_then(|size| parse_size_string(&size).ok());

            return download_archive(
                &url,
                &output_dir,
                &include_formats,
                &exclude_formats,
                max_file_size_bytes,
                cli.concurrent_downloads,
                cli.dry_run,
                cli.verbose,
                compress || cli.compress,
                cli.decompress,
                &cli.decompress_formats,
            ).await;
        }
        None => {
            // Handle main CLI arguments or interactive mode
            if cli.is_interactive_mode() {
                // Enter interactive mode
                println!("{} Starting interactive mode...", "🚀".blue());
                let mut menu = InteractiveMenu::new()?;
                menu.run().await?;
                return Ok(());
            }

            // Handle direct URL argument
            if let Some(url) = cli.url.as_ref() {
                let identifier = extract_identifier_from_url(url)?;
                let output_dir = cli.output_path
                    .as_ref()
                    .map(PathBuf::from)
                    .unwrap_or_else(|| {
                        let mut current = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
                        current.push(identifier);
                        current
                    });

                let include_formats = cli.include_extensions();
                let exclude_formats = cli.exclude_extensions();
                let max_file_size_bytes = cli.max_file_size_bytes();

                return download_archive(
                    url,
                    &output_dir,
                    &include_formats,
                    &exclude_formats,
                    max_file_size_bytes,
                    cli.concurrent_downloads,
                    cli.dry_run,
                    cli.verbose,
                    cli.compress,
                    cli.decompress,
                    &cli.decompress_formats,
                ).await;
            } else {
                // No URL provided and not interactive mode
                eprintln!("{} No URL provided. Use --help for usage information.", "❌".red());
                eprintln!("Run without arguments to enter interactive mode.");
                std::process::exit(1);
            }
        }
    }
}

/// Extract identifier from URL or return the URL if it's already an identifier
fn extract_identifier_from_url(url: &str) -> Result<&str> {
    if url.contains("archive.org/details/") {
        url.split("archive.org/details/")
            .nth(1)
            .and_then(|s| s.split('/').next())
            .ok_or_else(|| anyhow::anyhow!("Invalid archive.org URL"))
    } else {
        // Assume it's already an identifier
        Ok(url)
    }
}

/// Main download function with full persistence support
#[allow(clippy::too_many_arguments)]
async fn download_archive(
    url: &str,
    output_dir: &Path,
    include_formats: &[String],
    exclude_formats: &[String],
    max_file_size: Option<u64>,
    concurrent_downloads: usize,
    dry_run: bool,
    _verbose: bool,
    enable_compression: bool,
    auto_decompress: bool,
    decompress_formats: &[String],
) -> Result<()> {
    let identifier = extract_identifier_from_url(url)?;
    let archive_url = if url.contains("archive.org") { 
        url.to_string() 
    } else { 
        format!("https://archive.org/details/{}", identifier) 
    };

    println!(
        "{} Initializing download for archive: {}",
        "🚀".blue(),
        identifier.bright_cyan().bold()
    );

    if dry_run {
        println!(
            "{} DRY RUN MODE - fetching metadata only",
            "🔍".yellow().bold()
        );
    }

    // Create HTTP client
    let client = Client::builder()
        .user_agent(get_user_agent())
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .context("Failed to create HTTP client")?;

    // Create session directory
    let session_dir = output_dir.join(".ia-get-sessions");

    // Initialize the archive downloader
    let downloader = ArchiveDownloader::new(
        client.clone(),
        concurrent_downloads,
        true, // verify_md5
        true, // preserve_mtime
        session_dir,
        enable_compression,
        auto_decompress,
    );

    // Fetch metadata
    let progress = ProgressBar::new_spinner();
    progress.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    println!("{} Fetching metadata for: {}", "📡".cyan(), identifier);

    let (metadata, _base_url) = fetch_json_metadata(&archive_url, &client, &progress)
        .await
        .context("Failed to fetch metadata using JSON API")?;

    progress.finish_and_clear();

    // Display metadata information
    println!("\n{} Archive Information:", "📊".blue().bold());
    println!("  Identifier: {}", identifier);
    println!("  Total files: {}", metadata.files.len());
    println!("  Archive size: {}", format_size(metadata.item_size));
    println!("  Server: {}", metadata.server);

    // Apply filtering based on CLI arguments
    let filtered_files = apply_file_filters(&metadata.files, include_formats, exclude_formats, max_file_size);

    if dry_run {
        display_dry_run_info(&filtered_files, &metadata.files, include_formats, exclude_formats, max_file_size);
        return Ok(());
    }

    if filtered_files.is_empty() {
        println!("{} No files match the specified filters", "⚠️".yellow());
        return Ok(());
    }

    // Initialize download history tracking
    let history_manager = DownloadHistoryManager::new()?;
    let download_id = generate_download_id(identifier);

    // Create download configuration
    let download_config = DownloadConfig {
        output_dir: output_dir.to_string_lossy().to_string(),
        max_concurrent: concurrent_downloads as u32,
        format_filters: include_formats.to_vec(),
        min_size: None,
        max_size: max_file_size,
        verify_md5: true,
        preserve_mtime: true,
        user_agent: get_user_agent(),
        enable_compression,
        auto_decompress,
        decompress_formats: decompress_formats.to_vec(),
    };

    // Record download start in history
    let mut download_record = DownloadRecord {
        id: download_id.clone(),
        archive_identifier: identifier.to_string(),
        original_url: archive_url.clone(),
        status: DownloadStatus::InProgress,
        started_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        completed_at: None,
        total_files: filtered_files.len(),
        completed_files: 0,
        failed_files: 0,
        bytes_downloaded: 0,
        settings: download_config.clone(),
        error_message: None,
        failed_file_list: vec![],
    };

    history_manager.add_download_record(download_record.clone())?;

    println!("\n{} Beginning file downloads...", "🔥".green().bold());
    println!("📋 {} files selected for download", filtered_files.len());

    // Get list of file names to download
    let requested_files: Vec<String> = filtered_files.iter().map(|f| f.name.clone()).collect();

    // Create progress bar
    let progress_bar = create_progress_bar(filtered_files.len());
    progress_bar.set_message("Initializing download session...".yellow().to_string());

    // Execute download
    match downloader
        .download_with_metadata(
            archive_url,
            identifier.to_string(),
            metadata,
            download_config,
            requested_files,
            &progress_bar,
        )
        .await
    {
        Ok(session) => {
            progress_bar.finish_with_message(
                "✅ Download completed successfully!"
                    .green()
                    .bold()
                    .to_string(),
            );

            // Update download record with completion
            let completed_files = session
                .file_status
                .values()
                .filter(|status| matches!(status.status, DownloadState::Completed))
                .count();

            let failed_files = session
                .file_status
                .values()
                .filter(|status| matches!(status.status, DownloadState::Failed))
                .count();

            let bytes_downloaded: u64 = session
                .file_status
                .values()
                .filter(|status| matches!(status.status, DownloadState::Completed))
                .map(|status| status.file_info.size.unwrap_or(0))
                .sum();

            let failed_file_list: Vec<String> = session
                .file_status
                .iter()
                .filter(|(_, status)| matches!(status.status, DownloadState::Failed))
                .map(|(name, _)| name.clone())
                .take(100) // Limit to prevent huge records
                .collect();

            download_record.status = if failed_files == 0 {
                DownloadStatus::Success
            } else if completed_files > 0 {
                DownloadStatus::PartialSuccess
            } else {
                DownloadStatus::Failed
            };

            download_record.completed_at = Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );
            download_record.completed_files = completed_files;
            download_record.failed_files = failed_files;
            download_record.bytes_downloaded = bytes_downloaded;
            download_record.failed_file_list = failed_file_list;

            history_manager.update_download_record(download_record)?;

            display_download_summary(&session, output_dir);

            if failed_files > 0 {
                println!(
                    "\n{} {} files failed to download",
                    "⚠️".yellow(),
                    failed_files
                );
                println!("💡 You can retry the download with the same command to resume");
                println!("📜 Check download history with: ia-get history list");
            }
        }
        Err(e) => {
            progress_bar.finish_with_message("❌ Download failed".red().bold().to_string());

            // Update download record with failure
            download_record.status = DownloadStatus::Failed;
            download_record.completed_at = Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );
            download_record.error_message = Some(e.to_string());

            history_manager.update_download_record(download_record)?;

            eprintln!("\n{} Download failed: {}", "❌".red().bold(), e);
            eprintln!("📜 Check download history with: ia-get history show {}", download_id);
            return Err(anyhow::Error::from(e));
        }
    }

    Ok(())
}

/// Apply file filters based on CLI arguments
fn apply_file_filters(
    files: &[ArchiveFile],
    include_formats: &[String],
    exclude_formats: &[String],
    max_file_size: Option<u64>,
) -> Vec<ArchiveFile> {
    files
        .iter()
        .filter(|file| {
            // Apply include format filter
            if !include_formats.is_empty() {
                let file_format = file.format.as_deref().unwrap_or("");
                let file_extension = std::path::Path::new(&file.name)
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("");

                let matches_include = include_formats.iter().any(|fmt| {
                    fmt.eq_ignore_ascii_case(file_format)
                        || fmt.eq_ignore_ascii_case(file_extension)
                });

                if !matches_include {
                    return false;
                }
            }

            // Apply exclude format filter
            if !exclude_formats.is_empty() {
                let file_format = file.format.as_deref().unwrap_or("");
                let file_extension = std::path::Path::new(&file.name)
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("");

                let matches_exclude = exclude_formats.iter().any(|fmt| {
                    fmt.eq_ignore_ascii_case(file_format)
                        || fmt.eq_ignore_ascii_case(file_extension)
                });

                if matches_exclude {
                    return false;
                }
            }

            // Apply size filter
            if let Some(max_size) = max_file_size {
                if file.size.unwrap_or(0) > max_size {
                    return false;
                }
            }

            true
        })
        .cloned()
        .collect()
}

/// Display information for dry run mode
fn display_dry_run_info(
    filtered_files: &[ArchiveFile],
    all_files: &[ArchiveFile],
    include_formats: &[String],
    exclude_formats: &[String],
    max_file_size: Option<u64>,
) {
    println!("\n{} Files in archive:", "📋".cyan().bold());

    // Show filtering info if filters are applied
    if !include_formats.is_empty() || !exclude_formats.is_empty() || max_file_size.is_some() {
        println!(
            "  {}: {} → {} files",
            "After filtering".yellow(),
            all_files.len(),
            filtered_files.len()
        );
        if !include_formats.is_empty() {
            println!(
                "  {}: {}",
                "Include formats".yellow(),
                include_formats.join(", ").cyan()
            );
        }
        if !exclude_formats.is_empty() {
            println!(
                "  {}: {}",
                "Exclude formats".yellow(),
                exclude_formats.join(", ").cyan()
            );
        }
        if let Some(max_size) = max_file_size {
            println!(
                "  {}: {}",
                "Size limit".yellow(),
                format_size(max_size).cyan()
            );
        }
        println!();
    }

    let display_files = if filtered_files.is_empty() {
        all_files
    } else {
        filtered_files
    };

    for (i, file) in display_files.iter().enumerate().take(10) {
        println!(
            "  {:<3} {} ({})",
            format!("{}.", i + 1).dimmed(),
            file.name.green(),
            format_size(file.size.unwrap_or(0)).cyan()
        );
    }
    if display_files.len() > 10 {
        println!("  ... and {} more files", display_files.len() - 10);
    }

    if filtered_files.is_empty() && (!include_formats.is_empty() || !exclude_formats.is_empty() || max_file_size.is_some()) {
        println!("\n{} No files match the specified filters", "⚠️".yellow());
    }

    println!("\n{} Use without --dry-run to download", "💡".yellow());
}

/// Create a progress bar for download tracking
fn create_progress_bar(file_count: usize) -> ProgressBar {
    let pb = ProgressBar::new(file_count as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({msg})")
            .unwrap()
            .progress_chars("##-")
    );
    pb.set_message("Downloading...");
    pb
}

/// Display download summary after completion
fn display_download_summary(session: &DownloadSession, output_dir: &Path) {
    let completed_files = session
        .file_status
        .values()
        .filter(|status| {
            matches!(
                status.status,
                DownloadState::Completed
            )
        })
        .count();
    let total_files = session.file_status.len();
    let total_bytes: u64 = session
        .file_status
        .values()
        .filter(|status| {
            matches!(
                status.status,
                DownloadState::Completed
            )
        })
        .map(|status| status.file_info.size.unwrap_or(0))
        .sum();

    println!("\n{} Download Summary:", "📋".blue().bold());
    println!("  📂 Archive: {}", session.identifier);
    println!(
        "  📁 Output directory: {}",
        output_dir.display().to_string().bright_green()
    );
    println!("  📊 Files downloaded: {}/{}", completed_files, total_files);
    println!(
        "  💾 Total size: {}",
        format_size(total_bytes).bright_blue()
    );

    if completed_files < total_files {
        println!("\n{} Some files were not downloaded:", "⚠️".yellow());
        for (filename, status) in &session.file_status {
            if !matches!(
                status.status,
                DownloadState::Completed
            ) {
                println!("  • {} - {:?}", filename, status.status);
            }
        }
        println!(
            "\n💡 Use the same command to retry failed downloads"
        );
    }
}