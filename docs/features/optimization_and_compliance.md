# Optimization and Compliance Features

## Overview
This document details the optimizations implemented to reduce load on the Internet Archive API and ensure compliance with their usage guidelines.

## Features Implemented

### 1. Metadata Caching (`src/core/archive/metadata.rs`)
To reduce redundant API calls for metadata (which is static for completed items):
- **Mechanism**: JSON responses from the metadata API are cached to disk.
- **Location**: `.ia-get-sessions/{identifier}_metadata.json` (or similar, inside the session/cache directory).
- **TTL (Time To Live)**: 24 hours. If the cached file is younger than 24 hours, it is used instead of making a network request.
- **Compliance**: This significantly reduces the "read" load on the Archive's servers, especially when resuming or restarting complex downloads multiple times.

### 2. Local File Pre-Validation (`src/core/download/enhanced_downloader.rs`)
To avoid re-checking every file against the remote server when resuming:
- **Mechanism**: When a download session is initialized (or resumed), the local filesystem is checked immediately.
- **Logic**: For each file in the manifest:
  - If the file exists locally AND
  - The local file size matches the metadata size
  - Then the file is marked as `Completed` instantly.
- **Benefit**: 
  - Zero network requests for files that are already successfully downloaded.
  - Skips the "HEAD" request overhead for potentially thousands of files.
  - Dramatically speeds up resume operations ("smart resume").

## Compliance Benefits
- **Reduced Request Rate**: By caching metadata and skipping checks for existing files, the tool generates far fewer HTTP requests per session.
- **Bandwidth Conservation**: No re-downloading or re-verifying headers for content that is already intact.
- **Respectful Retries**: By checking local state first, the tool avoids "hammering" the API during retry loops.

## Usage
These features are enabled by default. 
- The metadata cache is automatic.
- The smart resume is part of the standard `download` command workflow.
