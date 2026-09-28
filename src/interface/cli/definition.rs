//! clap command and argument definitions for the ia-get CLI.

use clap::{Arg, ArgAction, ArgMatches, Command};

use super::types::SourceType;

/// Extract source types from CLI matches
pub fn get_source_types_from_matches(matches: &ArgMatches) -> Vec<SourceType> {
    // Handle convenience flags first
    if matches.get_flag("original-only") {
        return vec![SourceType::Original];
    }

    let mut types = vec![SourceType::Original]; // Always include originals by default

    if matches.get_flag("include-derivatives") {
        types.push(SourceType::Derivative);
    }

    if matches.get_flag("include-metadata") {
        types.push(SourceType::Metadata);
    }

    // Handle explicit source-types argument if provided
    if let Some(source_types) = matches.get_many::<String>("source-types") {
        let mut parsed_types = Vec::new();
        for type_str in source_types {
            match type_str.to_lowercase().as_str() {
                "original" => parsed_types.push(SourceType::Original),
                "derivative" => parsed_types.push(SourceType::Derivative),
                "metadata" => parsed_types.push(SourceType::Metadata),
                _ => {} // Ignore invalid types
            }
        }
        if !parsed_types.is_empty() {
            return parsed_types;
        }
    }

    types
}

/// Build the CLI interface
pub fn build_cli() -> Command {
    Command::new("ia-get")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Download files from the Internet Archive")
        .long_about("A CLI tool for downloading files from the Internet Archive with comprehensive metadata support, resume functionality, and progress tracking.")
        .arg(
            Arg::new("identifier")
                .help("Internet Archive identifier")
                .required(false)  // Make identifier optional since subcommands might not need it
                .value_name("IDENTIFIER")
                .index(1)
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output directory")
                .value_name("DIR")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .help("Show what would be downloaded without actually downloading")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("concurrent")
                .short('c')
                .long("concurrent")
                .help("Number of concurrent downloads (1-16)")
                .value_name("NUM")
                .default_value("4")
        )
        .arg(
            Arg::new("include")
                .short('i')
                .long("include")
                .help("Include only files with these formats (can be used multiple times)")
                .value_name("FORMAT")
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("include-formats")
                .long("include-formats")
                .help("Include files by format category (documents,images,audio,video,software,data,web,archives,metadata)")
                .value_name("CATEGORIES")
                .value_delimiter(',')
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("exclude-formats")
                .long("exclude-formats")
                .help("Exclude files by format category")
                .value_name("CATEGORIES")
                .value_delimiter(',')
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("list-formats")
                .long("list-formats")
                .help("List available file format categories and exit")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("list-formats-detailed")
                .long("list-formats-detailed")
                .help("List detailed file format information and exit")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("max-size")
                .long("max-size")
                .help("Maximum file size to download (e.g., 100MB, 1GB)")
                .value_name("SIZE")
        )
        .arg(
            Arg::new("no-compress")
                .long("no-compress")
                .help("Disable HTTP compression during downloads (compression is enabled by default)")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("decompress")
                .long("decompress")
                .help("Automatically decompress downloaded files")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("decompress-formats")
                .long("decompress-formats")
                .help("Compression formats to auto-decompress (comma-separated: gzip,bzip2,xz,tar)")
                .value_name("FORMATS")
                .value_delimiter(',')
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("source-types")
                .long("source-types")
                .help("Source types to include (original, derivative, metadata)")
                .value_name("TYPES")
                .value_delimiter(',')
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("original-only")
                .long("original-only")
                .help("Download only original files")
                .action(ArgAction::SetTrue)
                .conflicts_with("source-types")
        )
        .arg(
            Arg::new("include-derivatives")
                .long("include-derivatives")
                .help("Include derivative files in addition to originals")
                .action(ArgAction::SetTrue)
                .conflicts_with("source-types")
        )
        .arg(
            Arg::new("include-metadata")
                .long("include-metadata")
                .help("Include metadata files in addition to originals")
                .action(ArgAction::SetTrue)
                .conflicts_with("source-types")
        )
        .arg(
            Arg::new("api-health")
                .long("api-health")
                .help("Display Archive.org API health and monitoring information")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("analyze-metadata")
                .long("analyze-metadata")
                .help("Display enhanced metadata analysis for the specified archive")
                .action(ArgAction::SetTrue)
        )
        // Add subcommands for advanced features, configuration and history management
        .subcommand(
            Command::new("search")
                .about("Search Internet Archive")
                .long_about("Search the Internet Archive with advanced filtering options")
                .arg(
                    Arg::new("query")
                        .help("Search query (e.g., 'vintage computers', 'nasa missions')")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("limit")
                        .short('l')
                        .long("limit")
                        .help("Maximum number of results to return")
                        .value_name("NUM")
                        .default_value("20")
                )
                .arg(
                    Arg::new("sort")
                        .short('s')
                        .long("sort")
                        .help("Sort results by field (downloads, date, title)")
                        .value_name("FIELD")
                        .value_parser(["downloads", "date", "title", "views"])
                )
                .arg(
                    Arg::new("mediatype")
                        .short('m')
                        .long("mediatype")
                        .help("Filter by media type (movies, audio, texts, software, etc.)")
                        .value_name("TYPE")
                )
                .arg(
                    Arg::new("year")
                        .short('y')
                        .long("year")
                        .help("Filter by year or year range (e.g., '1970', '1970-1980')")
                        .value_name("YEAR")
                )
                .arg(
                    Arg::new("creator")
                        .short('c')
                        .long("creator")
                        .help("Filter by creator name")
                        .value_name("NAME")
                )
                .arg(
                    Arg::new("subject")
                        .long("subject")
                        .help("Filter by subject/topic")
                        .value_name("SUBJECT")
                )
        )
        .subcommand(
            Command::new("batch")
                .about("Batch download multiple archives")
                .long_about("Download multiple archives from a file containing identifiers or URLs, one per line")
                .arg(
                    Arg::new("file")
                        .help("File containing archive identifiers or URLs (one per line)")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Output directory for all downloads")
                        .value_name("DIR")
                )
                .arg(
                    Arg::new("parallel")
                        .short('p')
                        .long("parallel")
                        .help("Number of parallel downloads (1-10)")
                        .value_name("NUM")
                        .default_value("3")
                )
                .arg(
                    Arg::new("resume")
                        .short('r')
                        .long("resume")
                        .help("Resume interrupted batch operations")
                        .action(ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Show detailed progress for each download")
                        .action(ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("config")
                .about("Configuration and preference management")
                .subcommand(
                    Command::new("show")
                        .about("Show current configuration and preferences")
                )
                .subcommand(
                    Command::new("set")
                        .about("Set a configuration value")
                        .long_about("Set a configuration value. Use 'ia-get config show' to see available keys.")
                        .arg(Arg::new("key")
                            .help("Configuration key to set (e.g., concurrent_downloads, default_output_path)")
                            .required(true))
                        .arg(Arg::new("value")
                            .help("Value to set (e.g., 8, /path/to/downloads)")
                            .required(true))
                )
                .subcommand(
                    Command::new("unset")
                        .about("Remove a configuration value (reset to default)")
                        .long_about("Reset a configuration value to its default. Use 'ia-get config show' to see available keys.")
                        .arg(Arg::new("key")
                            .help("Configuration key to reset (e.g., concurrent_downloads, default_output_path)")
                            .required(true))
                )
                .subcommand(
                    Command::new("location")
                        .about("Show location of configuration file")
                )
                .subcommand(
                    Command::new("reset")
                        .about("Reset all configuration to defaults")
                )
                .subcommand(
                    Command::new("validate")
                        .about("Validate current configuration")
                )
        )
        .subcommand(
            Command::new("history")
                .about("Download history management")
                .long_about("Manage download history database. View past downloads, clear history, and get statistics.")
                .subcommand(
                    Command::new("show")
                        .about("Show download history")
                        .long_about("Display download history with optional filtering and details.")
                        .arg(
                            Arg::new("limit")
                                .short('l')
                                .long("limit")
                                .help("Number of recent entries to show (default: 10)")
                                .default_value("10")
                        )
                        .arg(
                            Arg::new("status")
                                .short('s')
                                .long("status")
                                .help("Filter entries by status: success, failed, in_progress, cancelled, paused")
                                .value_parser(["success", "failed", "in_progress", "cancelled", "paused"])
                        )
                        .arg(
                            Arg::new("detailed")
                                .short('d')
                                .long("detailed")
                                .help("Show detailed information including file counts and error messages")
                                .action(ArgAction::SetTrue)
                        )
                )
                .subcommand(
                    Command::new("clear")
                        .about("Clear download history")
                        .long_about("Remove all download history entries. Use --force to skip confirmation.")
                        .arg(
                            Arg::new("force")
                                .short('f')
                                .long("force")
                                .help("Clear all entries without confirmation prompt")
                                .action(ArgAction::SetTrue)
                        )
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove specific entry by ID")
                        .long_about("Remove a specific download history entry by its ID. Use 'ia-get history show' to see entry IDs.")
                        .arg(Arg::new("id")
                            .help("Entry ID to remove (shown in 'ia-get history show' output)")
                            .required(true))
                )
                .subcommand(
                    Command::new("stats")
                        .about("Show statistics about downloads")
                        .long_about("Display comprehensive statistics about download history including success rates and totals.")
                )
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        let cmd = build_cli();

        // Test basic usage
        let matches = cmd
            .clone()
            .try_get_matches_from(vec!["ia-get", "test-archive"])
            .unwrap();
        assert_eq!(
            matches.get_one::<String>("identifier").unwrap(),
            "test-archive"
        );

        // Test with options
        let matches = cmd
            .try_get_matches_from(vec![
                "ia-get",
                "test-archive",
                "--verbose",
                "--concurrent",
                "8",
                "--include",
                "pdf",
                "--include",
                "txt",
            ])
            .unwrap();

        assert_eq!(
            matches.get_one::<String>("identifier").unwrap(),
            "test-archive"
        );
        assert!(matches.get_flag("verbose"));
        assert_eq!(matches.get_one::<String>("concurrent").unwrap(), "8");

        let includes: Vec<_> = matches.get_many::<String>("include").unwrap().collect();
        assert_eq!(includes, vec!["pdf", "txt"]);
    }
}
