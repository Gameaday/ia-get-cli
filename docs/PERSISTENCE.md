# Persistence Features in ia-get

This document describes the persistence features implemented in ia-get, including configuration management and download history tracking.

## Overview

ia-get now provides comprehensive persistence capabilities:

1. **Configuration Persistence**: Store user preferences and default settings
2. **Download History Tracking**: Keep a permanent record of all download tasks
3. **Priority System**: Hierarchical setting precedence for CLI and GUI modes

## Configuration Files

### `ia-get.conf` (TOML format)

User configuration is stored in TOML format at:
- **Linux/macOS**: `~/.config/ia-get/config.toml`
- **Windows**: `%APPDATA%\ia-get\config.toml`

#### Configuration Options

```toml
# Download settings
concurrent_downloads = 5          # Number of concurrent downloads (1-16)
max_retries = 3                  # Maximum retry attempts
http_timeout = 30                # HTTP timeout in seconds

# Default options (override built-in defaults)
default_output_path = "/downloads"       # Default download directory
default_include_ext = "pdf,txt"          # Default include extensions
default_exclude_ext = "xml,log"          # Default exclude extensions
default_max_file_size = "100MB"          # Default maximum file size
default_resume = true                    # Resume downloads by default
default_verbose = false                  # Verbose output by default
default_log_hash_errors = true          # Log hash verification errors
default_dry_run = false                  # Dry run mode by default

# User agent override
user_agent_override = "MyBot/1.0"       # Custom user agent string

# Recent URLs (maintained automatically)
recent_urls = [
    "https://archive.org/details/example1",
    "https://archive.org/details/example2"
]
max_recent_urls = 10                     # Maximum recent URLs to keep

# Filter presets
[[filter_presets]]
name = "Documents"
description = "Common document formats"
include_ext = "pdf,doc,docx,txt,rtf,odt"
max_file_size = "100MB"

[[filter_presets]]
name = "Custom"
description = "My custom filter"
include_ext = "mp3,flac"
exclude_ext = "log"
max_file_size = "50MB"
```

### `ia-get-db.json` (JSON format)

Download history is stored in JSON format at:
- **Linux/macOS**: `~/.config/ia-get/ia-get-db.json`  
- **Windows**: `%APPDATA%\ia-get\ia-get-db.json`

#### Database Structure

```json
{
  "version": "1.0",
  "created_at": 1640995200,
  "last_updated": 1640995800,
  "records": {
    "archive_identifier_1640995200": {
      "id": "archive_identifier_1640995200",
      "archive_identifier": "example_archive",
      "original_url": "https://archive.org/details/example_archive",
      "status": "Success",
      "started_at": 1640995200,
      "completed_at": 1640995800,
      "total_files": 50,
      "completed_files": 50,
      "failed_files": 0,
      "bytes_downloaded": 104857600,
      "settings": {
        "output_dir": "/downloads/example_archive",
        "max_concurrent": 4,
        "format_filters": ["pdf", "txt"],
        "min_size": null,
        "max_size": 104857600,
        "verify_md5": true,
        "preserve_mtime": true,
        "user_agent": "ia-get/1.5.0",
        "enable_compression": true,
        "auto_decompress": false,
        "decompress_formats": []
      },
      "error_message": null,
      "failed_file_list": []
    }
  }
}
```

#### Download Record Fields

- `id`: Unique identifier for the download (format: `{identifier}_{timestamp}`)
- `archive_identifier`: Internet Archive identifier  
- `original_url`: URL that was requested
- `status`: Download status (`InProgress`, `Success`, `PartialSuccess`, `Failed`, `Cancelled`)
- `started_at`/`completed_at`: Unix timestamps
- `total_files`/`completed_files`/`failed_files`: File count statistics
- `bytes_downloaded`: Total bytes successfully downloaded
- `settings`: Complete download configuration used
- `error_message`: Error details if download failed
- `failed_file_list`: List of files that failed to download (limited to 100 entries)

## Priority System

### CLI Mode Priority
1. **Command line arguments** (highest priority)
2. **Saved preferences** (config file)
3. **Built-in defaults** (lowest priority)

### GUI Mode Priority  
1. **One-time selected options** (highest priority)
2. **Saved preferences** (config file)
3. **Built-in defaults** (lowest priority)

### Example
If config file has `concurrent_downloads = 5` but user runs:
```bash
ia-get --concurrent-downloads 8 archive_id
```
The download will use 8 concurrent downloads (CLI argument takes precedence).

## Configuration Management Commands

### View Configuration
```bash
# Show all current configuration values
ia-get config show

# Get a specific configuration value
ia-get config get concurrent_downloads
```

### Modify Configuration
```bash
# Set configuration values
ia-get config set concurrent_downloads 8
ia-get config set default_output_path "/my/downloads"
ia-get config set include_ext "pdf,txt,doc"
ia-get config set verbose true

# Reset to defaults
ia-get config reset
```

### Configuration File Location
```bash
# Show where configuration files are stored
ia-get config path
```

## Download History Commands

### View History
```bash
# Show recent downloads (default: 10)
ia-get history list

# Show more records
ia-get history list --limit 20

# Filter by status
ia-get history list --status success
ia-get history list --status failed
```

### Download Statistics
```bash
# Show overall download statistics
ia-get history stats
```

### History Management
```bash
# Clean up old records (older than 90 days)
ia-get history cleanup --days 90

# Show specific download details
ia-get history show archive_identifier_1640995200

# Show database file location
ia-get history path
```

## Valid Configuration Keys

When using `ia-get config set <key> <value>`:

- `output_dir`, `output_directory`: Default download directory
- `concurrent_downloads`, `concurrent`: Number of concurrent downloads (1-16)
- `max_retries`, `retries`: Maximum retry attempts (0-20)
- `include_ext`, `include_extensions`: Default file extensions to include
- `exclude_ext`, `exclude_extensions`: Default file extensions to exclude
- `max_file_size`, `max_size`: Default maximum file size limit
- `resume`: Resume downloads by default (true/false)
- `verbose`: Verbose output by default (true/false)  
- `log_hash_errors`: Log hash verification errors (true/false)
- `dry_run`: Dry run mode by default (true/false)
- `http_timeout`, `timeout`: HTTP timeout in seconds (1-300)
- `user_agent`: Custom user agent string

### Boolean Values
For boolean settings, accepted values include:
- **True**: `true`, `yes`, `y`, `1`, `on`, `enable`, `enabled`
- **False**: `false`, `no`, `n`, `0`, `off`, `disable`, `disabled`

## Integration with Existing Features

### Automatic URL Tracking
Recently accessed archive URLs are automatically added to `recent_urls` in the configuration file.

### Session Resumption  
Download sessions (temporary state for resuming) are stored separately from the permanent history database.

### Filter Presets
Predefined filter configurations are available for common use cases and can be customized in the config file.

## File Locations Summary

| File | Purpose | Linux/macOS | Windows |
|------|---------|-------------|---------|
| `config.toml` | User preferences | `~/.config/ia-get/config.toml` | `%APPDATA%\ia-get\config.toml` |
| `ia-get-db.json` | Download history | `~/.config/ia-get/ia-get-db.json` | `%APPDATA%\ia-get\ia-get-db.json` |
| Session files | Resume data | `{output_dir}/.ia-get-sessions/` | `{output_dir}\.ia-get-sessions\` |

## Migration and Compatibility

- The database format includes a `version` field for future migrations
- Configuration is backward compatible with older versions
- Missing configuration keys use built-in defaults
- Invalid configuration values are rejected with helpful error messages

## Security and Privacy

- Configuration files are created with user-only read/write permissions where possible
- No sensitive information (passwords, tokens) is stored in configuration files
- Download URLs in history can be manually removed if needed
- The history database can be deleted entirely to clear all records