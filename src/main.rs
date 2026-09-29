//! Main entry point for ia-get CLI application

// Allow collapsible_if for now - can be refactored in separate PR
#![allow(clippy::collapsible_if)]

use anyhow::{Context, Result};
use colored::Colorize;
use std::path::PathBuf;
use tokio::signal;

use ia_get::{
    DownloadRequest, DownloadResult, DownloadService,
    core::session::DownloadState,
    core::session::sanitize_filename_for_filesystem,
    interface::cli::{
        analyze_archive_metadata, build_cli, display_api_health, get_source_types_from_matches,
    },
    utilities::filters::format_size,
};

/// Show an interactive menu when no arguments are provided
async fn show_interactive_menu() -> Result<()> {
    // Add debugging to see if we reach this function
    eprintln!("{} Launching interactive CLI menu...", "🔄".cyan());

    // Use the enhanced interactive CLI directly without creating a new runtime
    match ia_get::interface::interactive::launch_interactive_cli().await {
        Ok(()) => {
            eprintln!("{} Interactive CLI completed successfully", "✅".green());
            Ok(())
        }
        Err(e) => {
            eprintln!("{} Interactive CLI error: {}", "❌".red(), e);
            Err(anyhow::anyhow!("Interactive CLI error: {}", e))
        }
    }
}

/// Entry point for the ia-get CLI application  
#[tokio::main]
async fn main() -> Result<()> {
    // Set up signal handling for graceful shutdown
    tokio::spawn(async {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        println!("\n{} Download interrupted by user", "⚠️".yellow());
        std::process::exit(0);
    });

    // Parse command line arguments
    let matches = build_cli().try_get_matches();

    // Handle parsing errors gracefully
    let matches = match matches {
        Ok(matches) => matches,
        Err(e) => {
            // Check if this is a "missing arguments" error and we have no args at all
            let args: Vec<String> = std::env::args().collect();
            if args.len() == 1 {
                // No arguments provided - use smart detection
                println!(
                    "{} No arguments provided, detecting best interface mode...",
                    "🚀".bright_blue()
                );

                return show_interactive_menu().await;
            } else {
                // Other parsing errors, show them normally
                e.exit();
            }
        }
    };

    // Check for subcommands first
    match matches.subcommand() {
        Some(("search", search_matches)) => {
            use ia_get::interface::cli::advanced_commands;

            let query = search_matches
                .get_one::<String>("query")
                .expect("Query argument is required")
                .clone();
            let limit = search_matches
                .get_one::<String>("limit")
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(20);
            let sort = search_matches.get_one::<String>("sort").map(|s| s.as_str());
            let mediatype = search_matches
                .get_one::<String>("mediatype")
                .map(|s| s.as_str());
            let year = search_matches.get_one::<String>("year").map(|s| s.as_str());

            println!("{} Searching Internet Archive...", "🔍".cyan().bold());

            // Call with correct argument order: query, mediatype, year, sort, limit
            match advanced_commands::search_archive(&query, mediatype, year, sort, limit).await {
                Ok(results) => {
                    advanced_commands::display_search_results(&results);
                }
                Err(e) => {
                    return Err(anyhow::anyhow!("Search failed: {}", e));
                }
            }
            return Ok(());
        }
        Some(("batch", batch_matches)) => {
            use ia_get::interface::cli::advanced_commands;

            let file_path = batch_matches
                .get_one::<String>("file")
                .expect("File argument is required")
                .clone();
            let output_dir = batch_matches.get_one::<String>("output").cloned();
            let parallel = batch_matches
                .get_one::<String>("parallel")
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(3);
            let resume = batch_matches.get_flag("resume");
            let verbose = batch_matches.get_flag("verbose");

            println!("{} Starting batch download...", "📦".cyan().bold());

            let config = advanced_commands::BatchConfig {
                input_file: file_path,
                parallel,
                resume,
                output_dir,
                dry_run: false,
            };

            match advanced_commands::batch_download(config).await {
                Ok(results) => {
                    println!("\n{} Batch download completed!", "✅".green().bold());

                    let successful = results.iter().filter(|r| r.success).count();
                    let failed = results.iter().filter(|r| !r.success).count();

                    println!("Successful: {}", successful);
                    println!("Failed: {}", failed);

                    if failed > 0 && verbose {
                        println!("\n{} Failed downloads:", "⚠️".yellow());
                        for item in results.iter().filter(|r| !r.success) {
                            if let Some(error) = &item.error {
                                println!("  • {}: {}", item.identifier, error);
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(anyhow::anyhow!("Batch download failed: {}", e));
                }
            }
            return Ok(());
        }
        Some(("config", config_matches)) => {
            use ia_get::interface::cli::commands;
            match config_matches.subcommand() {
                Some(("show", _)) => {
                    commands::handle_config_command(ia_get::interface::cli::ConfigAction::Show)
                        .await?;
                }
                Some(("set", set_matches)) => {
                    let key = set_matches
                        .get_one::<String>("key")
                        .expect("Key argument is required")
                        .clone();
                    let value = set_matches
                        .get_one::<String>("value")
                        .expect("Value argument is required")
                        .clone();
                    commands::handle_config_command(ia_get::interface::cli::ConfigAction::Set {
                        key,
                        value,
                    })
                    .await?;
                }
                Some(("unset", unset_matches)) => {
                    let key = unset_matches
                        .get_one::<String>("key")
                        .expect("Key argument is required")
                        .clone();
                    commands::handle_config_command(ia_get::interface::cli::ConfigAction::Unset {
                        key,
                    })
                    .await?;
                }
                Some(("location", _)) => {
                    commands::handle_config_command(ia_get::interface::cli::ConfigAction::Location)
                        .await?;
                }
                Some(("reset", _)) => {
                    commands::handle_config_command(ia_get::interface::cli::ConfigAction::Reset)
                        .await?;
                }
                Some(("validate", _)) => {
                    commands::handle_config_command(ia_get::interface::cli::ConfigAction::Validate)
                        .await?;
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "No config subcommand specified. Use 'ia-get config --help' for available options."
                    ));
                }
            }
            return Ok(());
        }
        Some(("history", history_matches)) => {
            use ia_get::interface::cli::commands;
            match history_matches.subcommand() {
                Some(("show", show_matches)) => {
                    let limit = show_matches
                        .get_one::<String>("limit")
                        .expect("Limit argument required (has default)")
                        .parse()
                        .unwrap_or(10);
                    let status = show_matches.get_one::<String>("status").cloned();
                    let detailed = show_matches.get_flag("detailed");
                    commands::handle_history_command(ia_get::interface::cli::HistoryAction::Show {
                        limit,
                        status,
                        detailed,
                    })
                    .await?;
                }
                Some(("clear", clear_matches)) => {
                    let force = clear_matches.get_flag("force");
                    commands::handle_history_command(
                        ia_get::interface::cli::HistoryAction::Clear { force },
                    )
                    .await?;
                }
                Some(("remove", remove_matches)) => {
                    let id = remove_matches
                        .get_one::<String>("id")
                        .expect("ID argument is required")
                        .clone();
                    commands::handle_history_command(
                        ia_get::interface::cli::HistoryAction::Remove { id },
                    )
                    .await?;
                }
                Some(("stats", _)) => {
                    commands::handle_history_command(ia_get::interface::cli::HistoryAction::Stats)
                        .await?;
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "No history subcommand specified. Use 'ia-get history --help' for available options."
                    ));
                }
            }
            return Ok(());
        }
        _ => {
            // Continue with regular download processing
        }
    }

    // Check for API health command first
    if matches.get_flag("api-health") {
        display_api_health().await?;
        return Ok(());
    }

    // Check for metadata analysis command
    if matches.get_flag("analyze-metadata") {
        let raw_identifier = matches.get_one::<String>("identifier").ok_or_else(|| {
            anyhow::anyhow!("Archive identifier is required for metadata analysis")
        })?;

        let identifier = ia_get::utilities::common::normalize_archive_identifier(raw_identifier)
            .context("Failed to normalize archive identifier")?;

        analyze_archive_metadata(&identifier).await?;
        return Ok(());
    }

    // Check for format listing commands
    if matches.get_flag("list-formats") {
        ia_get::utilities::filters::list_format_categories();
        return Ok(());
    }

    if matches.get_flag("list-formats-detailed") {
        ia_get::utilities::filters::show_complete_format_help();
        return Ok(());
    }

    // Extract arguments - identifier is only required for download operations
    let raw_identifier = matches.get_one::<String>("identifier");

    // Check if we have an identifier when we need one
    if raw_identifier.is_none() {
        // If no identifier and no flags/subcommands that don't need it, launch smart mode detection
        if !matches.get_flag("api-health")
            && !matches.get_flag("list-formats")
            && !matches.get_flag("list-formats-detailed")
            && !matches.get_flag("analyze-metadata")
            && matches.subcommand().is_none()
        {
            // No arguments provided that require identifier - use smart detection for best interface mode
            println!(
                "{} No archive identifier provided, launching interactive mode...",
                "🚀".bright_blue()
            );

            return show_interactive_menu().await;
        }
        return Ok(()); // This shouldn't be reached due to subcommand handling above
    }

    let raw_identifier =
        raw_identifier.expect("Identifier should have been verified by interactive check logic");

    // Normalize the identifier - extract just the identifier portion if it's a URL
    let identifier = ia_get::utilities::common::normalize_archive_identifier(raw_identifier)
        .context("Failed to normalize archive identifier")?;

    // Load saved configuration (if any). `config_exists` avoids creating a file on
    // a plain download invocation.
    let config = ia_get::infrastructure::config::ConfigManager::new()
        .ok()
        .filter(|manager| manager.config_exists())
        .and_then(|manager| manager.load_config().ok())
        .unwrap_or_default();

    // Precedence for the output directory: --output > config > current dir/<identifier>
    let output_dir = match matches.get_one::<String>("output") {
        Some(output) => PathBuf::from(output),
        None => {
            let base = config
                .default_output_path
                .as_ref()
                .map(PathBuf::from)
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
            // Sanitize the identifier when using it as a directory name to prevent Windows path issues
            base.join(sanitize_filename_for_filesystem(&identifier))
        }
    };

    let verbose = matches.get_flag("verbose") || config.default_verbose;
    let dry_run = matches.get_flag("dry-run") || config.default_dry_run;

    let concurrent_downloads = matches
        .get_one::<String>("concurrent")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(config.concurrent_downloads)
        .clamp(1, 16); // Cap at 16 concurrent downloads

    let mut include_formats = matches
        .get_many::<String>("include")
        .map(|values| values.map(|s| s.to_string()).collect::<Vec<_>>())
        .unwrap_or_default();

    // Add formats from format categories
    if let Some(format_categories) = matches.get_many::<String>("include-formats") {
        use ia_get::utilities::filters::{FileFormats, FormatCategory};
        let file_formats = FileFormats::new();

        for category_name in format_categories {
            let category_name_lower = category_name.to_lowercase();
            for category in FormatCategory::all() {
                if category.display_name().to_lowercase() == category_name_lower {
                    include_formats.extend(file_formats.get_formats(&category));
                    break;
                }
            }
        }
    }

    let mut exclude_formats = Vec::new();

    // Add exclude formats from format categories
    if let Some(exclude_format_categories) = matches.get_many::<String>("exclude-formats") {
        use ia_get::utilities::filters::{FileFormats, FormatCategory};
        let file_formats = FileFormats::new();

        for category_name in exclude_format_categories {
            let category_name_lower = category_name.to_lowercase();
            for category in FormatCategory::all() {
                if category.display_name().to_lowercase() == category_name_lower {
                    exclude_formats.extend(file_formats.get_formats(&category));
                    break;
                }
            }
        }
    }

    let max_file_size = matches
        .get_one::<String>("max-size")
        .map(|s| s.to_string())
        .or_else(|| config.default_max_file_size.clone());

    // Compression settings: --no-compress overrides config; otherwise use the config default
    let enable_compression = if matches.get_flag("no-compress") {
        false
    } else {
        config.default_compress
    };
    let auto_decompress = matches.get_flag("decompress") || config.default_decompress;
    let decompress_formats: Vec<String> = matches
        .get_many::<String>("decompress-formats")
        .map(|values| values.map(|s| s.to_string()).collect::<Vec<_>>())
        .unwrap_or_else(|| {
            config
                .default_decompress_formats
                .as_ref()
                .map(|formats| {
                    formats
                        .split(',')
                        .map(|f| f.trim().to_string())
                        .filter(|f| !f.is_empty())
                        .collect()
                })
                .unwrap_or_default()
        });

    // Create unified download request
    let request = DownloadRequest {
        identifier: identifier.clone(),
        output_dir: output_dir.clone(),
        include_formats,
        exclude_formats,              // Now we support exclude formats
        min_file_size: String::new(), // CLI doesn't support min size yet, but unified API does
        max_file_size,
        concurrent_downloads,
        enable_compression,
        auto_decompress,
        decompress_formats,
        dry_run,
        verify_md5: true,
        preserve_mtime: true,
        verbose,
        resume: true,
        source_types: get_source_types_from_matches(&matches),
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

    // Create download service
    let service = DownloadService::new().context("Failed to create download service")?;

    // Execute download using unified API
    match service.download(request.clone(), None).await {
        Ok(DownloadResult::Success(session, api_stats, _is_dry_run)) => {
            if !dry_run {
                println!("\n{} Download completed successfully!", "✅".green().bold());
                println!(
                    "📁 Output directory: {}",
                    output_dir.display().to_string().bright_green()
                );
                DownloadService::display_download_summary(&session, &request);

                // Display Archive.org API statistics
                if let Some(stats) = api_stats {
                    println!("\n{} Archive.org API Usage:", "📊".blue().bold());
                    println!("  {}", stats);
                    if verbose {
                        println!(
                            "  Session healthy: {}",
                            if stats.average_requests_per_minute < 30.0 {
                                "✅ Yes"
                            } else {
                                "⚠️ High rate"
                            }
                        );
                    }
                }

                // Provide next steps if session has failed files
                let failed_files: Vec<_> = session
                    .file_status
                    .values()
                    .filter(|status| matches!(status.status, DownloadState::Failed))
                    .collect();

                if !failed_files.is_empty() {
                    println!(
                        "\n{} {} files failed to download",
                        "⚠️".yellow(),
                        failed_files.len()
                    );
                    println!("💡 You can retry the download with the same command to resume");
                }
            } else {
                // Display dry run results
                println!("\n{} Archive Information:", "📊".blue().bold());
                println!("  Identifier: {}", session.identifier);
                println!("  Total files: {}", session.archive_metadata.files.len());
                println!(
                    "  Archive size: {}",
                    format_size(session.archive_metadata.item_size)
                );
                println!("  Server: {}", session.archive_metadata.server);
                println!(
                    "  Available servers: {}",
                    session.archive_metadata.workable_servers.join(", ")
                );
                println!("  Directory: {}", session.archive_metadata.dir);

                println!("\n{} Files selected for download:", "📋".cyan().bold());
                println!("  Selected: {} files", session.requested_files.len());

                for (i, filename) in session.requested_files.iter().enumerate().take(10) {
                    println!(
                        "  {:<3} {}",
                        format!("{}.", i + 1).dimmed(),
                        filename.green()
                    );
                }
                if session.requested_files.len() > 10 {
                    println!(
                        "  ... and {} more files",
                        session.requested_files.len() - 10
                    );
                }

                println!("\n{} Use without --dry-run to download", "💡".yellow());

                // Display Archive.org API statistics for dry run too
                if let Some(stats) = api_stats {
                    println!("\n{} Archive.org API Usage:", "📊".blue().bold());
                    println!("  {}", stats);
                }
            }
        }
        Ok(DownloadResult::Error(error)) => {
            return Err(anyhow::anyhow!("Error: {}", error));
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Error: {}", e));
        }
    }

    Ok(())
}
