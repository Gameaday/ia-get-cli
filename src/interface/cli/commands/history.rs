//! `ia-get history ...` command handlers.

use crate::{
    Result,
    error::IaGetError,
    infrastructure::persistence::download_history::{
        DownloadHistory, TaskStatus, get_default_history_db_path,
    },
    utilities::filters::format_size,
};
use colored::Colorize;
use std::io::{self, Write};

use super::super::types::HistoryAction;

/// Handle history commands
pub async fn handle_history_command(action: HistoryAction) -> Result<()> {
    let history_path = get_default_history_db_path()?;

    match action {
        HistoryAction::Show {
            limit,
            status,
            detailed,
        } => show_history(&history_path, limit, status.as_deref(), detailed).await,
        HistoryAction::Clear { force } => clear_history(&history_path, force).await,
        HistoryAction::Remove { id } => remove_history_entry(&history_path, &id).await,
        HistoryAction::Stats => show_history_stats(&history_path).await,
    }
}

/// Show download history
async fn show_history(
    history_path: &std::path::Path,
    limit: usize,
    status_filter: Option<&str>,
    detailed: bool,
) -> Result<()> {
    let history = DownloadHistory::load_or_create(history_path)?;

    println!("{} Download History", "📚".blue().bold());
    println!();

    if history.entries.is_empty() {
        println!("{} No download history found", "ℹ️".blue());
        return Ok(());
    }

    // Filter by status if requested
    let entries: Vec<_> = if let Some(status_str) = status_filter {
        let target_status = match status_str.to_lowercase().as_str() {
            "success" => TaskStatus::Success,
            "failed" => TaskStatus::Failed("".to_string()),
            "in_progress" | "inprogress" => TaskStatus::InProgress,
            "cancelled" => TaskStatus::Cancelled,
            "paused" => TaskStatus::Paused,
            _ => {
                return Err(IaGetError::Config(format!(
                    "Invalid status filter: {}. Valid options: success, failed, in_progress, cancelled, paused",
                    status_str
                )));
            }
        };
        history.get_entries_by_status(&target_status)
    } else {
        history.get_recent_entries(limit)
    };

    if entries.is_empty() {
        println!("{} No entries match the specified criteria", "ℹ️".blue());
        return Ok(());
    }

    for (i, entry) in entries.iter().enumerate() {
        if i >= limit {
            break;
        }

        let status_display = match &entry.status {
            TaskStatus::Success => "✅ Success".green(),
            TaskStatus::Failed(msg) => format!("❌ Failed: {}", msg).red(),
            TaskStatus::InProgress => "🔄 In Progress".yellow(),
            TaskStatus::Cancelled => "⏹️ Cancelled".cyan(),
            TaskStatus::Paused => "⏸️ Paused".blue(),
        };

        println!(
            "{} {}",
            format!("{}.", i + 1).dimmed(),
            entry.archive_identifier.bright_green()
        );
        println!("    ID: {}", entry.id.cyan());
        println!("    Status: {}", status_display);
        println!(
            "    Started: {}",
            entry
                .started_at
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string()
                .dimmed()
        );

        if let Some(completed) = entry.completed_at {
            println!(
                "    Completed: {}",
                completed
                    .format("%Y-%m-%d %H:%M:%S UTC")
                    .to_string()
                    .dimmed()
            );
        }

        if detailed {
            println!("    Original input: {}", entry.original_input.blue());
            println!("    Output directory: {}", entry.output_directory.cyan());
            println!(
                "    Progress: {}/{} files ({:.1}%)",
                entry.completed_files,
                entry.total_files,
                entry.completion_percentage()
            );
            println!(
                "    Data downloaded: {}",
                format_size(entry.bytes_downloaded)
            );

            if entry.failed_files > 0 {
                println!("    Failed files: {}", entry.failed_files.to_string().red());
            }
        }

        println!();
    }

    if entries.len() == limit && history.entries.len() > limit {
        println!(
            "{} Showing {} of {} entries. Use --limit to see more.",
            "ℹ️".blue(),
            limit,
            history.entries.len()
        );
    }

    Ok(())
}

/// Clear download history
async fn clear_history(history_path: &std::path::Path, force: bool) -> Result<()> {
    if !force {
        print!(
            "{} This will clear ALL download history. Continue? [y/N]: ",
            "⚠️".yellow()
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !matches!(input.trim().to_lowercase().as_str(), "y" | "yes") {
            println!("History clear cancelled.");
            return Ok(());
        }
    }

    let mut history = DownloadHistory::load_or_create(history_path)?;
    let count = history.entries.len();
    history.clear();
    history.save_to_file(history_path)?;

    println!("{} Cleared {} history entries", "✅".green(), count);

    Ok(())
}

/// Remove specific history entry
async fn remove_history_entry(history_path: &std::path::Path, id: &str) -> Result<()> {
    let mut history = DownloadHistory::load_or_create(history_path)?;

    if history.remove_entry(id) {
        history.save_to_file(history_path)?;
        println!("{} Removed history entry: {}", "✅".green(), id.cyan());
    } else {
        return Err(IaGetError::Config(format!(
            "History entry with ID '{}' not found",
            id
        )));
    }

    Ok(())
}

/// Show download history statistics
async fn show_history_stats(history_path: &std::path::Path) -> Result<()> {
    let history = DownloadHistory::load_or_create(history_path)?;

    println!("{} Download History Statistics", "📊".blue().bold());
    println!();

    if history.entries.is_empty() {
        println!("{} No download history found", "ℹ️".blue());
        return Ok(());
    }

    let stats = history.get_statistics();

    println!("{} Overall Statistics:", "📈".green());
    println!(
        "  Total downloads: {}",
        stats.total_downloads.to_string().cyan()
    );
    println!(
        "  Successful: {} ({:.1}%)",
        stats.successful_downloads.to_string().green(),
        (stats.successful_downloads as f32 / stats.total_downloads as f32) * 100.0
    );
    println!(
        "  Failed: {} ({:.1}%)",
        stats.failed_downloads.to_string().red(),
        (stats.failed_downloads as f32 / stats.total_downloads as f32) * 100.0
    );

    if stats.in_progress_downloads > 0 {
        println!(
            "  In progress: {}",
            stats.in_progress_downloads.to_string().yellow()
        );
    }

    if stats.cancelled_downloads > 0 {
        println!(
            "  Cancelled: {}",
            stats.cancelled_downloads.to_string().cyan()
        );
    }

    println!();
    println!("{} Data Transfer:", "💾".blue());
    println!(
        "  Total data downloaded: {}",
        format_size(stats.total_bytes_downloaded).bright_green()
    );
    println!(
        "  Total files downloaded: {}",
        stats.total_files_downloaded.to_string().cyan()
    );

    if stats.total_files_downloaded > 0 {
        let avg_file_size = stats.total_bytes_downloaded / stats.total_files_downloaded as u64;
        println!(
            "  Average file size: {}",
            format_size(avg_file_size).yellow()
        );
    }

    println!();
    println!("{} Database Information:", "🗃️".purple());
    println!("  Database version: {}", history.version.cyan());
    println!(
        "  Created: {}",
        history
            .created_at
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string()
            .dimmed()
    );
    println!(
        "  Last updated: {}",
        history
            .last_updated
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string()
            .dimmed()
    );
    println!("  Max entries: {}", history.max_entries.to_string().cyan());

    Ok(())
}
