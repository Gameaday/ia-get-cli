//! `--api-health` and `--analyze-metadata` command handlers.

use crate::{
    core::archive::AdvancedMetadataProcessor,
    infrastructure::api::{EnhancedArchiveApiClient, get_archive_servers},
    utilities::common::{
        HTTP_TIMEOUT, MAX_CONCURRENT_CONNECTIONS, MIN_REQUEST_DELAY_MS, get_user_agent,
    },
    utilities::filters::format_size,
};
use anyhow::Context;
use colored::Colorize;

/// Display Archive.org API health and monitoring information
pub async fn display_api_health() -> anyhow::Result<()> {
    println!("{} Archive.org API Health Status", "🏥".blue().bold());
    println!();

    // Create a test API client
    let client = reqwest::Client::builder()
        .user_agent(get_user_agent())
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .context("Failed to create HTTP client")?;

    let mut api_client = EnhancedArchiveApiClient::new(client);

    // Test basic connectivity with official status endpoint
    println!("{} Testing Archive.org service status...", "🔗".cyan());
    match api_client.get_service_status().await {
        Ok(response) => {
            let status = response.status();
            println!("  ✅ Status endpoint successful (HTTP {})", status);

            // Try to parse the status response
            if let Ok(text) = response.text().await {
                if let Ok(status_data) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(status_msg) = status_data.get("status").and_then(|s| s.as_str()) {
                        println!("  📊 Service Status: {}", status_msg);
                    }
                }
            }
        }
        Err(e) => {
            println!("  ❌ Status check failed: {}", e);
        }
    }

    // Test metadata API
    println!("\n{} Testing Metadata API...", "📋".cyan());
    match api_client.get_metadata("nasa").await {
        Ok(response) => {
            println!(
                "  ✅ Metadata API successful (status: {})",
                response.status()
            );
        }
        Err(e) => {
            println!("  ❌ Metadata API failed: {}", e);
        }
    }

    // Test search API
    println!("\n{} Testing Search API...", "🔍".cyan());
    match api_client
        .search_items("collection:nasa", Some("identifier,title"), Some(1), None)
        .await
    {
        Ok(response) => {
            println!("  ✅ Search API successful (status: {})", response.status());

            // Parse search results to show functionality
            if let Ok(text) = response.text().await {
                if let Ok(search_data) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(num_found) =
                        search_data.get("response").and_then(|r| r.get("numFound"))
                    {
                        println!(
                            "  📊 Search returned {} total items in nasa collection",
                            num_found
                        );
                    }
                }
            }
        }
        Err(e) => {
            println!("  ❌ Search API failed: {}", e);
        }
    }

    // Display server list
    println!("\n{} Available Archive.org Servers:", "🌐".green().bold());
    let servers = get_archive_servers();
    for (i, server) in servers.iter().enumerate() {
        println!(
            "  {:<2} {}",
            format!("{}.", i + 1).dimmed(),
            server.bright_blue()
        );
    }

    // Test multiple requests to show rate limiting
    println!("\n{} Testing API rate limiting...", "⏱️".yellow());

    for i in 0..3 {
        // Use valid identifiers that exist
        let test_identifiers = ["mario", "luigi", "nasa"];
        let identifier = test_identifiers[i % test_identifiers.len()];

        match api_client.get_metadata(identifier).await {
            Ok(_) => {
                let stats = api_client.get_stats();
                println!(
                    "  Request {}: ✅ {} (Rate: {:.1} req/min)",
                    i + 1,
                    identifier,
                    stats.average_requests_per_minute
                );
            }
            Err(e) => {
                println!("  Request {}: ❌ {} - {}", i + 1, identifier, e);
            }
        }
    }

    // Display final statistics
    println!("\n{} API Session Statistics:", "📊".purple().bold());
    let final_stats = api_client.get_stats();
    println!("  {}", final_stats);

    // Health assessment
    println!("\n{} Health Assessment:", "🎯".bright_green().bold());
    if api_client.is_rate_healthy() {
        println!("  ✅ Request rate is healthy and Archive.org compliant");
    } else {
        println!("  ⚠️  Request rate is high - consider slowing down requests");
    }

    // Enhanced API capabilities
    println!(
        "\n{} Enhanced API Capabilities:",
        "⚡".bright_yellow().bold()
    );
    println!("  ✅ Metadata API - Item information and file listings");
    println!("  ✅ Search API - Finding items across collections");
    println!("  ✅ Tasks API - Monitoring upload/processing status");
    println!("  ✅ Collections API - Batch operations on collections");
    println!("  ✅ Status API - Real-time service health monitoring");

    println!(
        "\n{} Archive.org API Guidelines:",
        "📋".bright_cyan().bold()
    );
    println!("  • Keep concurrent connections ≤ 5 for respectful usage");
    println!("  • Include descriptive User-Agent with contact information");
    println!("  • Implement retry logic for transient failures");
    println!("  • Honor rate limiting (429) and retry-after headers");
    println!("  • Use appropriate timeouts for large file downloads");

    println!("\n{} Current Configuration:", "⚙️".bright_magenta().bold());
    println!("  User Agent: {}", get_user_agent().bright_green());
    println!("  Default Timeout: {} seconds", HTTP_TIMEOUT);
    println!("  Min Request Delay: {}ms", MIN_REQUEST_DELAY_MS);
    println!(
        "  Max Concurrent: {} connections",
        MAX_CONCURRENT_CONNECTIONS
    );

    Ok(())
}

/// Analyze and display enhanced metadata for an archive
pub async fn analyze_archive_metadata(identifier: &str) -> anyhow::Result<()> {
    println!("{} Enhanced Metadata Analysis", "🔍".blue().bold());
    println!("Archive: {}", identifier.bright_green());
    println!();

    // Create HTTP client
    let client = reqwest::Client::builder()
        .user_agent(get_user_agent())
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .context("Failed to create HTTP client")?;

    // Create enhanced API client
    let api_client = EnhancedArchiveApiClient::new(client.clone());

    // Create advanced metadata processor
    let mut processor = AdvancedMetadataProcessor::new(api_client);

    // Perform comprehensive analysis
    match processor.analyze_metadata(identifier).await {
        Ok(analysis) => {
            // Display the comprehensive analysis
            processor.display_analysis(&analysis);

            // Show additional insights
            if analysis.completeness_score < 60.0 {
                println!("{}", "💡 Suggestions for improvement:".yellow().bold());
                if !analysis.quality_indicators.has_description {
                    println!("  • Add a detailed description to improve discoverability");
                }
                if !analysis.quality_indicators.has_creator {
                    println!("  • Specify the creator or author information");
                }
                if analysis.quality_indicators.files_have_checksums < 80.0 {
                    println!(
                        "  • Consider adding checksums to more files for integrity verification"
                    );
                }
                println!();
            }

            // Show technical details for advanced users
            if std::env::var("IA_GET_VERBOSE").is_ok() {
                println!("{}", "🔧 Technical Details:".dimmed().bold());
                println!("  Size Distribution:");
                println!(
                    "    Small files (< 1MB): {}",
                    analysis.size_distribution.small_files
                );
                println!(
                    "    Medium files (1MB-100MB): {}",
                    analysis.size_distribution.medium_files
                );
                println!(
                    "    Large files (100MB-1GB): {}",
                    analysis.size_distribution.large_files
                );
                println!(
                    "    Huge files (> 1GB): {}",
                    analysis.size_distribution.huge_files
                );
                println!(
                    "    Average size: {}",
                    format_size(analysis.size_distribution.average_size)
                );
                println!(
                    "    Median size: {}",
                    format_size(analysis.size_distribution.median_size)
                );

                if !analysis.largest_files.is_empty() {
                    println!("  Largest Files:");
                    for (i, file) in analysis.largest_files.iter().take(5).enumerate() {
                        let truncated_name = if file.name.len() > 50 {
                            format!("{}...", &file.name[..47])
                        } else {
                            file.name.clone()
                        };
                        println!(
                            "    {}. {} ({})",
                            i + 1,
                            truncated_name,
                            format_size(file.size)
                        );
                    }
                }
                println!();
            }

            println!(
                "\n{} Use this analysis to make informed download decisions!",
                "💡".bright_yellow()
            );
        }
        Err(e) => {
            eprintln!(
                "{} {} {}",
                "❌".red(),
                "Failed to analyze metadata:".red().bold(),
                e
            );
            std::process::exit(1);
        }
    }

    Ok(())
}
