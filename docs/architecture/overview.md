# ia-get Architecture

This document describes the intended structure and the main code paths, so that
changes land in the right layer.

## Goals

- A cross-platform **CLI** and a reusable **library** (`ia_get`).
- Download from the Internet Archive first (compliantly and respectfully), used
  where ordinary web downloads are impractical (large items, no seeded torrents).
- **One code path per concern** — avoid parallel implementations that drift.

## Layers

```
binary        src/main.rs              thin entry point: dispatch + download orchestration
interface/    everything user-facing
              cli/                     definition.rs (clap tree), types.rs, commands/*, advanced_commands/*
              interactive/             terminal UI (interactive_cli + config menu)
core/         domain logic
              archive/                 metadata.rs (fetch/parse), analysis.rs (AdvancedMetadataProcessor)
              download/                enhanced_downloader.rs (engine), download_service.rs (facade)
              session/                 metadata_storage.rs (models + session persistence)
infrastructure/  I/O and integration
              api/                     archive_api.rs (ArchiveOrgApiClient, EnhancedArchiveApiClient)
              http/                    http_client.rs (pooled client), network.rs (connectivity)
              config/                  main.rs (Config + ConfigManager)
              persistence/             download_history.rs
utilities/    reusable helpers
              common/                  constants, url_processing, utils, progress, performance
              filters/                 file_formats, format_help, filter_files
              compression/             decompression
```

Dependency direction is one-way: `interface → core → infrastructure → utilities`.

`interface/cli` layout:

| File | Responsibility |
| --- | --- |
| `definition.rs` | the clap command/argument tree (`build_cli`) and arg helpers |
| `types.rs` | `SourceType`, `ConfigAction`, `HistoryAction` |
| `commands/config.rs` | `ia-get config ...` |
| `commands/history.rs` | `ia-get history ...` |
| `commands/analysis.rs` | `--api-health`, `--analyze-metadata` |
| `advanced_commands/` | `search`, `batch` |

## Main data flows

### Download (the core path)

1. `src/main.rs` builds a `DownloadRequest`, applying **flag > config > default**
   precedence.
2. `DownloadService::download` (`core/download/download_service.rs`) fetches
   metadata via `fetch_json_metadata` (`core/archive/metadata.rs`), applies
   filters, and hands off to the engine.
3. `ArchiveDownloader::download_with_metadata`
   (`core/download/enhanced_downloader.rs`) schedules files under a semaphore.
   Per file: try mirrors in order, resume with an HTTP `Range` into a `.tmp`
   file, optionally verify MD5, then atomically move into place (with optional
   decompression).
4. File/session state lives in `DownloadSession`
   (`core/session/metadata_storage.rs`) and is saved as JSON for resume.

### Configuration

`ConfigManager` (`infrastructure/config/main.rs`) reads/writes `config.toml` in
the platform config directory. Both `ia-get config ...` and the TUI use it — there
is a single config type (`Config`) and a single file.

### Metadata

`fetch_json_metadata` parses responses into `ArchiveMetadata` (the session model).
`AdvancedMetadataProcessor` (`core/archive/analysis.rs`) powers `--analyze-metadata`.

## Conventions

- **Layering**: user-facing rendering belongs in `interface`; `core` and
  `infrastructure` should not print or depend on terminal UI. *(Target state — see
  below.)*
- **One model per concept**: `ArchiveFile` / `ArchiveMetadata` are the canonical
  types; don't introduce a parallel struct for the same data.
- **Errors**: prefer the crate `Result`/`IaGetError` (`src/error.rs`). The binary
  may use `anyhow` at the top level.
- **Paths**: use canonical `crate::core::…` / `crate::infrastructure::…` paths;
  no crate-root aliases.
- **One implementation per concern** (one download engine, one config store, one
  metadata parser).

## Testing & quality gates

```shell
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

Unit tests live inline (`#[cfg(test)]`) with focused integration tests under `tests/`.

## Progress reporting (UI-agnostic)

Long-running operations in `core`/`infrastructure` report by emitting
[`ProgressEvent`]s to a [`ProgressReporter`] (`core/progress.rs`):

- the download engine, metadata fetch, download service, decompression and
  connectivity checks emit events — they no longer touch the terminal;
- the CLI renders them via `interface::progress::IndicatifReporter`;
- the interactive TUI renders them via its own reporter;
- library users pass their own reporter, or `NoopReporter` to stay silent.

[`ProgressEvent`]: ../src/core/progress.rs
[`ProgressReporter`]: ../src/core/progress.rs

## Known gaps / next steps

- `analysis.rs::display_analysis` still prints with `colored`; it should return a
  formatted string that the CLI prints.
- `core`/`infrastructure` still contain a few plain `eprintln!` diagnostics
  (disk-space and history warnings); these could become `Message` events.
- Retry/backoff/rate-limiting is implemented in more than one place
  (`http/network.rs`, `api/archive_api.rs`, the download engine) — target: a
  single `RetryPolicy` + limiter.
- Torrent/seed-after-download and S3-style upload are not implemented yet.
