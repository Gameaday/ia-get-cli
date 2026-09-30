# ia-get Codebase Audit

**Status:** Draft for review
**Scope:** Whole repository (`ia-get` Rust CLI + library)
**Method:** Static review of source, tests, docs, CI and packaging. The audit sandbox had no
Rust toolchain (no `cargo`/`rustc`), so `cargo build`, `cargo clippy` and `cargo test` could not be
executed here. All claims are backed by `file:line` evidence that can be re-verified locally with
`cargo clippy --all-targets --all-features`.

---

## 1. Executive summary

`ia-get` has a genuinely good **download core** (`ArchiveDownloader` + `DownloadService`):
concurrent file downloads, resumable byte-range downloads, multi-mirror fallback, MD5 validation,
and Internet-Archive-specific status handling. That core is worth protecting.

The problem is everything *around* the core. The repository is still carrying the residue of the
"add features + GUI, then split the GUI out" history:

| Symptom | Count |
| --- | --- |
| Overlapping download engines | **5** (only 1 is used in production) |
| Overlapping metadata modules | **3** (two define a struct both named `MetadataAnalysis`) |
| Overlapping config systems | **2** (they read/write *different files* and one deletes the other's file) |
| Overlapping interactive UIs | **2** (one is unreachable) |
| Orphaned source files (never compiled) | **2** |
| Shipped "feature" that is a stub | **1** (`batch` download) |

Against the stated product goals the gaps are significant:

- **Upload** — not implemented at all.
- **Torrents / seeding after download** — not implemented; `btih` (BitTorrent infohash) is *parsed*
  from metadata but never used, and `_archive.torrent` is treated as a generic file to fetch.
- **Library use** — the core is coupled to the terminal (`indicatif` progress bars, `colored`,
  `println!` inside `core::` and `infrastructure::`), and the crate is built as `rlib` only, so it
  is awkward to consume as a clean library or from other languages.

The single most important recommendation is to **pick one download engine, one metadata module, one
config system, one interactive UI, and delete the rest**, then close the goal gaps on top of that
smaller base. Most of the "improvements" here are deletions.

## 2. As-is architecture

The intended layering is well documented in `src/lib.rs` and is a good idea:

```
interface/        CLI (main.rs + clap), interactive menus, optional egui GUI
   |              src/interface/{cli,interactive,gui}
   v
core/             archive metadata, download engines, session state
   |              src/core/{archive,download,session}
   v
infrastructure/   HTTP client, Archive.org API client, config, persistence
   |              src/infrastructure/{api,http,config,persistence}
   v
utilities/        formatting, filters, compression, perf helpers
                  src/utilities/{common,filters,compression}
```

The dependency direction is *mostly* respected, but presentation concerns leak downward
(see §4.A4). Module LOC distribution (`wc -l src/**/*.rs`, ~18.7k lines total):

| Area | Approx. LOC | Notes |
| --- | --- | --- |
| `interface/` | ~6.3k | CLI 1.0k, interactive 2.9k, GUI 2.4k (feature-gated) |
| `core/` | ~4.4k | 3 download engines + 3 metadata modules + session |
| `infrastructure/` | ~2.0k | api, http, config, persistence |
| `utilities/` | ~1.6k | common, filters, compression |
| `main.rs` | 1.41k | clap builder + GUI launcher + menu launcher |
| `tests/` | ~3.2k | see §4.F |

## 3. Findings

Each finding lists **Evidence**, **Impact** and **Recommendation**.

### A. Gaps against the product goals

#### A1. Upload is not implemented (goal: download, **upload**, manage)
- **Evidence:** no upload code or API calls exist. `(?i)upload` only matches doc comments
  ("original uploaded files" in `src/interface/cli/main.rs:44`) and the metadata field
  `upload_date` (`src/core/archive/metadata_new.rs:59`). `src/infrastructure/api/archive_api.rs`
  only implements read operations (`get_metadata_json`, `search_items`, `search_collections`,
  `get_tasks`).
- **Impact:** the "upload" third of the product statement is entirely missing.
- **Recommendation:** decide scope. A respectful first step is **S3-style upload** to the
  `ia` item buckets (`https://s3.us.archive.org/<identifier>/<file>`, `Authorization: LOW <access>:<secret>`,
  `x-amz-auto-make-bucket`, `x-archive-meta-*` headers), behind an `upload` subcommand and a
  `client.upload()` library call. Add an `UploadRequest` type mirroring `DownloadRequest`. Keep
  credentials out of config in plaintext (see §4.C1).

#### A2. No torrent / seeding support (goal: usable when there are no seeded torrents, seed after)
- **Evidence:** `btih` is deserialized but unused (`src/core/session/metadata_storage.rs:66-67`,
  `src/core/archive/archive_metadata.rs:45`). `_archive.torrent` is downloaded as an ordinary
  metadata file (`src/core/download/enhanced_downloader.rs:931`). There is no `.torrent` creation,
  no magnet construction, no libtorrent/bittorrent client, and `magnet` only appears as a file
  *extension category* (`src/utilities/filters/file_formats.rs:359`).
- **Impact:** the differentiator (fetch huge, low-seed items and then let the user seed) is absent.
- **Recommendation:** add (a) a `--torrent` / `ia-get torrent <id>` path that fetches the item's
  `_archive.torrent` from the metadata server and writes it next to the files, (b) when a `btih`
  (or `_archive.torrent`) is present, print the `magnet:?xt=urn:btih:...` URI and the `.torrent`
  path so the user can seed immediately, and (c) optionally a `seed` mode that hands the completed
  directory to a torrent client (or documents seeding). At minimum, ensure the downloaded bitstream
  is byte-exact so seeds are valid (see §4.E3).

#### A3. Big-archive handling is decent; integrity is deliberately softened
- **Evidence:** resume via `Range` + `.tmp` file works (`src/core/download/enhanced_downloader.rs:744-757`,
  `:838-905`). But MD5 mismatches are *accepted* for a hardcoded set of metadata filenames
  (`:517-529`), and XML files pass on a loose size heuristic
  (`src/core/session/metadata_storage.rs:527-536`, tolerance up to 10%/100 bytes). Missing files are
  silently "skipped" when `verify_md5` is off but a file already exists (`:474-484`).
- **Impact:** for a tool whose purpose is archiving and seeding, silently accepting bad bytes is
  risky. `verify_md5` only applies when metadata *has* an md5; IA items frequently expose `sha1`
  too, which is unused.
- **Recommendation:** make integrity explicit and non-silent: prefer `md5`, fall back to `sha1`
  (`sha1` is already a dependency and a parsed field), log/return a distinct `IntegrityError`, and
  gate "accept mismatch" behind an explicit flag. Never treat a size mismatch as success without a
  warning the user can see in the final summary.

#### A4. The "library" is coupled to the terminal UI
- **Evidence:** `core::` and `infrastructure::` construct and drive `indicatif::ProgressBar`
  (`src/core/download/enhanced_downloader.rs:41,158-162`; `src/core/archive/metadata.rs` takes
  `&ProgressBar`), use `colored` (`src/core/download/download_service.rs:20,595-624`;
  `src/core/download/enhanced_downloader.rs:40`), and print with `println!`/`eprintln!`
  (`download_service.rs::display_download_summary`). `Cargo.toml` builds `crate-type = ["rlib"]`
  only.
- **Impact:** a library consumer cannot download without pulling terminal deps and getting stdout
  side effects; C/other-language embedding is impossible (no `cdylib`).
- **Recommendation:** introduce an event/progress abstraction in `core` (e.g. an
  `Observer`/`ProgressSink` trait or a `tokio::sync::mpsc` of `ProgressUpdate`) and move all
  `indicatif`/`colored` rendering into `interface::`. Add `cdylib` to `crate-type` and a small
  stable facade (`pub mod prelude`) if cross-language use is a goal.

### B. Dead code & duplication ("the Frankenstein")

#### B1. Five download engines; only one (+ its facade) is used
- **Evidence:** `src/core/download/` contains `enhanced_downloader.rs` (`ArchiveDownloader`),
  `concurrent_simple.rs` (`SimpleConcurrentDownloader`), `downloader.rs` (`download_files`),
  `downloads.rs` (`download_files_with_retries`), and `download_service.rs` (`DownloadService`).
  Production path is `main.rs` → `DownloadService::download` → `ArchiveDownloader`
  (`src/core/download/download_service.rs:402-407`). `SimpleConcurrentDownloader` is referenced only
  by `benches/download_performance.rs:60`; `downloader::download_files` /
  `download_files_with_retries` are referenced only by each other and by `lib.rs` re-exports.
- **Impact:** ~1.1k LOC of unexercised, divergent download logic that also carries the obsolete
  `batchlog.json` / `--hash` feature (`downloader.rs:449-513`). Bug fixes must be applied 2–3 times.
- **Recommendation:** delete `concurrent_simple.rs`, `downloader.rs`, `downloads.rs`; keep
  `enhanced_downloader.rs` (engine) + `download_service.rs` (facade). Re-point the benchmark at the
  real engine. Drop the `batchlog`/`--hash` plumbing or re-home it in the surviving engine.

#### B2. Three metadata modules, two with a struct literally named `MetadataAnalysis`
- **Evidence:** `archive_metadata.rs` (`JsonFile`/`JsonMetadata` + `parse_json_files`),
  `metadata.rs` (`get_json_url`, `fetch_json_metadata`, `AdvancedMetadataProcessor`, and a nested
  `MetadataAnalysis` at `:473-533`), and `metadata_new.rs` (a *different* `MetadataAnalysis` at
  `:32-68` plus `EnhancedMetadataProcessor`). All three are re-exported together by
  `src/core/archive/mod.rs`. Two separate sets of string-or-number deserializers exist
  (`archive_metadata.rs` and `session::metadata_storage`).
- **Impact:** confusing public API (two `MetadataAnalysis` types) and duplicated fetch/retry logic.
  `metadata_new.rs` (~1.0k LOC) is only reachable through `AdvancedMetadataProcessor`; its `enhanced`
  submodule is what tests import (`tests/enhanced_api_tests.rs:7`) yet it is not exported as
  `::enhanced`, so that test target is inconsistent with the module tree.
- **Recommendation:** collapse to one metadata module: one `ArchiveMetadata`/`ArchiveFile` model
  (already in `session::metadata_storage`), one fetch function, one analysis type. Delete
  `metadata_new.rs` and fold anything genuinely useful into `metadata.rs`.

#### B3. `lib.rs` is half legacy-compatibility shims
- **Evidence:** `src/lib.rs:98-145` defines twelve `pub mod X { pub use crate::... }` aliases
  (`metadata`, `metadata_storage`, `url_processing`, `constants`, `cli`, `archive_metadata`,
  `filters`, `file_formats`, `progress`, `concurrent_simple`, `enhanced_downloader`, `compression`)
  explicitly "for external tests and examples".
- **Impact:** doubles the public surface, invites new code to use old flat paths, and keeps the
  deleted modules alive.
- **Recommendation:** update the handful of internal call sites/tests to the real paths and delete
  the alias block. Ship a documented `prelude` instead.

#### B4. Two config systems that conflict on disk (real bug)
- **Evidence:** `ConfigManager` reads/writes `<config dir>/config.toml`
  (`src/infrastructure/config/main.rs:148,187,206`) and is used by the GUI
  (`src/interface/gui/app.rs:85`) and interactive CLI. `ConfigPersistence` prefers
  `<config dir>/ia-get.conf` and on `migrate()` **deletes** `config.toml`
  (`src/infrastructure/persistence/config_persistence.rs:250-313`); it backs the `ia-get config`
  subcommand (`src/interface/cli/main/commands.rs:41`). They use different types
  (`Config` vs `ConfigValue<T>`) and different defaults (e.g. `concurrent_downloads: 3` in
  `Config::default` vs the `DownloadRequest` default of 4).
- **Impact:** running `ia-get config set ...` can delete the file the GUI/interactive mode reads,
  and the two disagree on values. This is a data-loss footgun.
- **Recommendation:** keep exactly one config type and one on-disk file. Migrate `ConfigPersistence`
  onto `Config`/`ConfigManager` (or vice-versa) and remove the destructive migration. Add
  `#[serde(default)]` so unknown/missing keys are tolerated across versions.

#### B5. Three ways to edit configuration
- **Correction (later verified):** an earlier draft of this audit claimed `interactive_menu.rs`
  was dead. It is **not** — `interactive_cli.rs` calls `launch_config_menu()` for its
  "configure settings" option. So the config-editing surface is genuinely triplicated.
- **Evidence:** `interactive_cli.rs` (1 965 LOC, launched from `main.rs` with no args) delegates to
  `interactive_menu.rs` (885 LOC); the `ia-get config` subcommand is a third path.
- **Impact:** three divergent config editors (over two config backends, see §B4).
- **Recommendation:** after unifying config storage (§B4), collapse to one editor — keep the
  interactive menu and make the `config` subcommand delegate to it (or vice versa).

#### B6. Orphaned files that are never compiled
- **Evidence:** `src/interface/gui/tests.rs` exists but `src/interface/gui/mod.rs` declares only
  `app` and `panels` — no `mod tests`, so the file is ignored by the compiler (and it references
  `crate::config::Config` / `crate::download_service::DownloadService`, paths that no longer exist,
  so it would *not* compile if included). `src/bin/test_json_api.rs` is auto-discovered as an extra
  binary target and ships a debug "test program".
- **Impact:** dead/misleading files; `test_json_api` bloats `cargo build --all-targets`.
- **Recommendation:** delete both, or move the GUI test under the `gui` feature with correct paths.

#### B7. The `batch` subcommand is advertised but is a stub
- **Evidence:** `batch` is wired end-to-end (`src/main.rs:292-317,1126`) to
  `advanced_commands::batch_download`, whose per-item work is
  `download_single_archive` = `sleep(1s); Ok(5)` (`src/interface/cli/advanced_commands/batch.rs:188-200`).
  It always reports success and fabricates "5 files"; `--resume`/`--parallel` are no-ops.
- **Impact:** a user-visible command that lies.
- **Recommendation:** implement it against `DownloadService` (iterate identifiers, call the real
  service, honor `parallel` via a semaphore and `resume`), or remove it until then.

#### B8. Vestigial `build.rs` and FFI scaffolding
- **Evidence:** `build.rs` calls `generate_simplified_ffi_header()`, which only does anything if
  `CARGO_FEATURE_FFI` is set — but there is no `ffi` feature in `Cargo.toml`, and it references
  `src/interface/ffi_simple.rs` and `cbindgen_simple.toml`, neither of which exists. It also still
  prints Android/Flutter guidance although the mobile app moved out of this repo.
- **Impact:** noise; the only *useful* part is the Windows manifest embedding.
- **Recommendation:** reduce `build.rs` to the Windows manifest step (or drop it) and remove the
  dead FFI/Android guidance.

### C. CLI / UX correctness

#### C1. The main download command ignores the saved configuration
- **Evidence:** `src/main.rs:603` builds `DownloadRequest { .. }` directly from clap matches; it
  never constructs a `Config`/`ConfigManager`. `DownloadRequest::from_config`
  (`src/core/download/download_service.rs:86`) is used only by the interactive CLI and GUI
  (`src/interface/interactive/interactive_cli.rs`, `src/interface/gui/app.rs:225`).
- **Impact:** `ia-get config set default_output_path ...` / `concurrent_downloads ...` has no effect
  on `ia-get <url>`. Users must pass every flag every time; config is decorative for scripting.
- **Recommendation:** load `Config` in `main.rs`, then apply precedence
  **CLI flag > config > default** when building the request (or route through
  `DownloadRequest::from_config` + clap overrides).

#### C2. Status/health output is hardcoded and drifted from the real constants
- **Evidence:** `--api-health` prints literal values ("Default Timeout: 30 seconds",
  "Min Request Delay: 100ms", "Max Concurrent: 5") while the implementation uses
  `HTTP_TIMEOUT = 60`, `MIN_REQUEST_DELAY_MS = 2000`, `MAX_CONCURRENT_CONNECTIONS = 5`
  (`src/utilities/common/constants.rs:29-36`).
- **Impact:** misleading diagnostics; the printed "compliance" numbers are wrong by 20x.
- **Recommendation:** print the constants themselves so the display can't drift.

#### C3. Stale / duplicated User-Agent strings
- **Evidence:** canonical agent is `get_user_agent()` (`constants.rs:13-23`, e.g.
  `ia-get-cli/2.1.0 (...)`), but `constants.rs:26` hardcodes a `USER_AGENT` const at `1.5.0`
  (unused/legacy), and `search.rs:78-79` hardcodes an inline `ia-get-cli/1.6.0`.
- **Impact:** IA server logs see three different identities; the "respectful, contactable agent"
  goal is undermined.
- **Recommendation:** delete the `USER_AGENT` const and have every request (including
  `advanced_commands::search`, which currently builds its own `reqwest::Client`) use
  `get_user_agent()`.

#### C4. Duplicated identifier/URL parsing
- **Evidence:** `utilities::common::extract_identifier_from_url` exists
  (`src/utilities/common/url_processing.rs`) and is re-exported, yet
  `advanced_commands/batch.rs:178-186` defines its own private `extract_identifier_from_url`, and
  `core/archive/metadata.rs:84-95` has yet another inline parse in `get_json_url`.
- **Impact:** inconsistent edge-case handling for the same URLs.
- **Recommendation:** consolidate on one `url_processing` implementation used everywhere.

### D. Internet-Archive API compliance & politeness

#### D1. Rate limiting is applied to metadata only, not to downloads
- **Evidence:** the politeness delay + `Retry-After` handling live in
  `ArchiveOrgApiClient::make_request` (`src/infrastructure/api/archive_api.rs:33-66`). The actual
  file downloads bypass it entirely: `ArchiveDownloader` calls the raw `reqwest::Client`
  (`src/core/download/enhanced_downloader.rs:745-757`). The per-request delay is a fixed, global
  2 s (`MIN_REQUEST_DELAY_MS`), applied even to `archive.org` JSON endpoints where IA asks for
  *reasonable* concurrency rather than a 2 s floor.
- **Impact:** the busiest requests (large file GETs, fanned out across up to 5 mirrors × N
  concurrent files) are the least throttled; meanwhile metadata is slowed uniformly. This is the
  opposite of "respectful where it matters".
- **Recommendation:** centralize a single `RateLimiter`/`Backoff` used by *both* metadata and
  download paths, keyed per-host, honoring `Retry-After` and `X-Accept-Reduced-Priority`. Make the
  delay configurable and default to IA's guidance (bounded concurrency + backoff on 429/503) rather
  than a blanket 2 s.

#### D2. Retry/backoff logic is reimplemented 4+ times with different constants
- **Evidence:** `archive_api.rs` (2 s min delay), `http/network.rs` (30 s→600 s backoff, 5 retries),
  `enhanced_downloader.rs` (per-server backoff `min(2^attempt, 30)`, rate-limit `min(60, 2^attempt)`,
  3 resume attempts, 45 s stall timeout), `downloads.rs` (60 s→900 s). All differ.
- **Impact:** unpredictable behavior and hard-to-reason-about testing.
- **Recommendation:** one `RetryPolicy` type (max attempts, base, factor, cap, jitter, retryable
  predicate) used by every network call.

#### D3. "Built-in caching" claim does not match the code
- **Evidence:** README advertises "Built-in caching and rate-limiting" (`README.md:50`). The only
  cache is an in-memory `HashMap<String, CachedAnalysis>` inside `EnhancedMetadataProcessor`
  (`src/core/archive/metadata_new.rs:158-161`), and `ArchiveOrgApiClient` sends
  `Cache-Control: no-cache` (`archive_api.rs:51`). There is no on-disk metadata cache and no
  conditional-request (`ETag`/`If-Modified-Since`) usage.
- **Impact:** repeated runs re-fetch metadata; the marketing claim is inaccurate.
- **Recommendation:** either implement a real, opt-in on-disk metadata cache with conditional
  requests (respecting IA's `item_last_updated`), or fix the README.

### E. Robustness / panics

#### E1. `unwrap()`/`expect()` in library and spawned-task code
- **Evidence:** `src/infrastructure/http/http_client.rs:274` (`Default` calls `Self::new().expect(..)`);
  `src/core/download/concurrent_simple.rs:271` (`semaphore.acquire().await.unwrap()`) and
  `:73` in `batch.rs` (`.expect("Semaphore closed unexpectedly")`);
  `src/core/download/enhanced_downloader.rs:218,230` (`.expect(..)` on semaphore + progress pool).
- **Impact:** any of these panics aborts the whole run (and with `panic = "abort"` in release,
  there is no unwind to catch it). A single dropped channel shouldn't kill a 100 GB download.
- **Recommendation:** replace with error propagation (`?`/`map_err`) or recoverable handling;
  audit for `unwrap`/`expect`/slicing-by-index across `src/` and forbid them in `clippy.toml`
  (`disallowed-methods`) or an allowlist.

#### E2. Silent success on size mismatch
- **Evidence:** `enhanced_downloader.rs:923-949` accepts a downloaded-size mismatch for a hardcoded
  filename list; `metadata_storage.rs` XML validation tolerates 10%/100-byte deltas.
- **Impact:** corrupt/truncated files can be reported as complete (and would fail as seeds).
- **Recommendation:** see §A3 — surface integrity outcomes explicitly.

### F. Testing & CI

#### F1. Several tests are network-dependent and assert almost nothing
- **Evidence:** `tests/enhanced_api_tests.rs:27` calls `api_client.get_metadata("mario")` live and
  the test explicitly passes on error ("Network errors are acceptable in CI").
  `tests/api/network_tests.rs` and `tests/metadata_tests_simple.rs` also construct real clients.
- **Impact:** slow/flaky in CI and effectively asserting nothing; network tests run on every push.
- **Recommendation:** mark live-network tests `#[ignore]` (run them in a dedicated, opt-in CI job)
  and add a mock HTTP layer (e.g. `wiremock`/`httpmock`) for the metadata + download logic.

#### F2. Empty and duplicated test targets
- **Evidence:** `tests/integration_tests.rs` is 1 line (empty). `tests/api/metadata_tests.rs` (287)
  and `tests/metadata_tests_simple.rs` (333) overlap heavily. Grouping files
  (`tests/support_tests.rs` = `mod support;`, etc.) add indirection.
- **Impact:** inflated test count without proportional coverage of the download engine.
- **Recommendation:** delete the empty file; merge duplicates; add focused tests for
  resume/`Range`, multi-mirror fallback, md5/sha1 verification and config precedence (these are the
  behaviors that matter and are currently untested).

#### F3. CI/lint scope and README mismatch
- **Evidence:** `.github/workflows/rust-ci.yml` runs `cargo fmt --check`,
  `cargo clippy --no-default-features --features cli -- -D warnings`, and
  `cargo test --no-default-features --features cli`. There is **no** lint/test job for the `gui`
  feature, and no `--all-targets`. README's CI badge points at `ci.yml`, but the workflow file is
  `rust-ci.yml` (`README.md:10`).
- **Impact:** the GUI path and benchmarks can silently rot; the badge may never render.
- **Recommendation:** add a matrix/step for `--all-features` (or at least `gui`), use
  `--all-targets`, and fix the badge URL. If GUI is being removed (§G), delete the feature instead.

#### F4. Benchmarks depend on code slated for deletion
- **Evidence:** `benches/download_performance.rs:60-62` benchmarks `SimpleConcurrentDownloader`,
  which §B1 recommends deleting; `benches/performance_benchmarks.rs` exercises the perf helpers.
- **Impact:** removing dead code breaks `cargo bench` / `--all-targets`.
- **Recommendation:** re-point benchmarks at `ArchiveDownloader`/`DownloadService` when deleting.

### G. Documentation & repository drift

- **README still sells the old multi-interface story.** It advertises a "🖼️ Desktop GUI"
  (`README.md:51,138,159`), "smart auto-detection" of GUI, an entire "Flutter Mobile/Web" section
  (§"Flutter Mobile/Web Specific"), and references files that do not exist in the repo
  (`GUI_README.md`, `ANDROID_DEPLOYMENT_GUIDE.md`, `mobile/flutter`). It also claims
  "81+ tests" and a Rust badge of "1.70+" while `Cargo.toml` sets `rust-version = "1.92.0"` and
  `edition = "2024"`.
- **`docs/TODO.md`** is about code-signing/mobile phases (last updated Oct 2025) and references a
  Flutter future phase that no longer belongs here.
- **`docs/QUICK_REFERENCE.md`**, `docs/guides/*`, `docs/features/*` and root `CHANGELOG.md` still
  contain large Flutter/Material-Design-3 sections for the moved-out app.
- **Branding leftover:** `assets/ia-helper*.png/svg` are the *mobile* app's assets
  (`README.md:2` uses `assets/ia-helper.png` as the ia-get icon).
- **Recommendation:** decide the GUI's fate (keep behind a default-off feature *and* document it, or
  remove it entirely) and then do a documentation pass: remove Flutter/Android sections, fix badges
  and version claims, delete/replace stale TODOs, and remove or repurpose the `ia-helper` assets.

## 4. Recommended roadmap

### P0 — correctness & trust (small, high value)
1. **Fix the config conflict** (§B4): one config type, one file, remove the destructive migration.
2. **Make the CLI honor config** (§C1): flag > config > default precedence.
3. **Surface integrity** (§A3/§E2): use `sha1` when `md5` is absent; stop silently accepting
   mismatches; report failures in the summary.
4. **Kill panics in the hot path** (§E1): remove `unwrap`/`expect` from library/spawned code.
5. **Fix the lying `batch` command** (§B7): implement it against `DownloadService` or hide it.

### P1 — shrink the codebase (mostly deletions)
6. Delete duplicate engines `concurrent_simple.rs`, `downloader.rs`, `downloads.rs` (§B1);
   re-point benches (§F4).
7. Delete `metadata_new.rs`; unify metadata (§B2).
8. Delete the `lib.rs` legacy alias block after updating call sites (§B3).
9. Delete `interactive_menu.rs` (§B5), `src/bin/test_json_api.rs`, orphan `gui/tests.rs` (§B6).
10. Decide GUI: remove the `gui` feature + `interface/gui/*` **or** keep and add CI coverage
    (§F3/§G). Given the stated "CLI + library" goal, removal is the cleaner default.
11. Trim `build.rs` to the Windows manifest step (§B8).

### P2 — deliver the actual goals
12. **Torrents/seeding** (§A2): `--torrent`/`torrent` command, magnet output, byte-exact guarantee.
13. **Upload** (§A1): S3-style `upload` command + library API with safe credential handling.
14. **Library ergonomics** (§A4): progress/observer abstraction, move rendering to `interface`,
    add `cdylib` + `prelude` if cross-language use is wanted.
15. **One rate-limit/retry/backoff policy** shared by metadata + downloads (§D1/§D2); real
    opt-in metadata cache with conditional requests, or fix the README (§D3).
16. **Docs pass** (§G) and add `--all-targets`/`--all-features` CI (plus a `clippy.toml` to ban
    `unwrap`/`expect`) (§F1/§F3).

A useful sequencing trick: do **P0 item 1–2** and **P1 items 6–10** first, because they delete code
that would otherwise need to be updated by every later change.

---

## 5. What's worth keeping

- The **layered module intent** (`interface/core/infrastructure/utilities`) is sound; keep the
  boundaries and let the cleanup enforce them.
- `ArchiveDownloader` + `DownloadService` is a credible engine: multi-mirror fallback, byte-range
  resume, stall timeouts, IA status-code handling (`enhanced_downloader.rs`), and a single
  `DownloadRequest`/`DownloadResult`/`ProgressUpdate` vocabulary shared by CLI and GUI — exactly the
  right abstraction; it just needs to be the *only* engine.
- Session state (`DownloadSession` + `JsonFile`/`ArchiveFile` with `md5`/`sha1`/`btih`) is already
  rich enough to power resume, verification and torrent output.
- `DownloadHistory`/`ConfigPersistence` persistence and Windows path-length handling
  (`metadata_storage.rs::validate_path_length`) show good cross-platform attention.

---

## 6. Appendix — quick inventory

| Path | ~LOC | Status |
| --- | --- | --- |
| `src/main.rs` | 1414 | CLI dispatch + GUI/menu launchers (split candidate) |
| `src/interface/interactive/interactive_cli.rs` | 1965 | used (launched with no args) |
| `src/interface/interactive/interactive_menu.rs` | 885 | **dead** (no caller) |
| `src/core/download/enhanced_downloader.rs` | 1052 | **the** engine |
| `src/core/download/download_service.rs` | 627 | facade (keep) |
| `src/core/download/downloader.rs` | 515 | **dead** |
| `src/core/download/downloads.rs` | 58 | **dead** |
| `src/core/download/concurrent_simple.rs` | 338 | bench-only, **candidate to delete** |
| `src/core/archive/metadata_new.rs` | 1002 | **duplicate**, candidate to delete |
| `src/core/archive/metadata.rs` | 571 | keep (unify into this) |
| `src/core/archive/archive_metadata.rs` | 285 | duplicate model, fold in |
| `src/core/session/metadata_storage.rs` | 994 | session + models (keep) |
| `src/interface/gui/*` | ~2400 | feature-gated; decide fate |
| `src/interface/gui/tests.rs` | 38 | **orphan (never compiled)** |
| `src/bin/test_json_api.rs` | 79 | **debug binary** |
| `tests/integration_tests.rs` | 1 | **empty** |
| `docs/TODO.md` | 176 | **stale** |
| `Cargo.toml` | 114 | `rlib` only; `gui` optional; no `ffi` feature |

*Everything above is verifiable with `git`, `grep` and `cargo clippy --all-targets --all-features`
once a Rust toolchain is available.*

---

## 7. Progress log

Work completed against this audit (all verified with `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings` and `cargo test --all-targets` on Rust 1.98):

- **B1** removed the dead download engines (`concurrent_simple.rs`, `downloader.rs`, `downloads.rs`)
  and re-pointed the benchmark.
- **B2** removed the tests-only duplicate `metadata.rs::enhanced`; the production analysis module
  was renamed `analysis.rs` (was `metadata_new.rs`).
- **B3** deleted the legacy `lib.rs` alias modules and migrated every import/doc/test to canonical
  paths.
- **B4** deleted `ConfigPersistence`/`ConfigValue`/`ConfigSource`/`ConfigWithSources` (including
  the destructive `config.toml` -> `ia-get.conf` migration); the `config` subcommand and the TUI
  now share `Config`/`ConfigManager`.
- **B6** deleted `src/bin/test_json_api.rs` and the orphan `interface/gui/tests.rs`.
- **B7** the `batch` command now drives `DownloadService` per identifier instead of the 1-second
  sleep stub.
- **C1** the main download command now applies saved config (flag > config > default).
- **C2** `--api-health` prints the real constants.
- **E1** removed panics from the download hot path (semaphore/progress-pool `expect`, `SystemTime`
  and path `unwrap`s).
- **F3** CI clippy now lints `--all-targets`; the `gui` feature is gone so the GUI job is moot.
- **G** removed the desktop GUI entirely (per decision), rewrote the README for CLI+TUI, deleted the
  stale phase/history docs, and cleaned GUI/Flutter references from scripts and CI.

**Still open (P2, larger features):** torrent/magnet + seed-after handling (§A2), S3-style upload
(§A1), library ergonomics (observer abstraction + `cdylib`, §A4), a single shared rate-limit/retry
policy (§D1/§D2), and the metadata cache claim (§D3).

### Later: UI-agnostic progress reporting (A4)

- Added `core::progress` (`ProgressEvent`, `ProgressReporter`, `NoopReporter`,
  `SharedReporter`). The download engine, metadata fetch, download service,
  decompression and connectivity checks now emit events instead of driving
  `indicatif`/`colored`; the CLI and TUI supply reporters and the download
  summary moved out of `core` into the CLI. `core`/`infrastructure` no longer
  reference `indicatif` or `colored` (except `analysis.rs::display_analysis`,
  pending).

### Later: CLI restructure (maintainability)

- `main.rs` slimmed from 1,245 to ~560 lines; the clap tree moved to
  `interface::cli::definition`. The confusing `cli/main.rs` + `cli/main/` layout
  was renamed to `cli/types.rs` + `cli/commands/{config,history,analysis}.rs`.
  `build.rs` reduced to the Windows manifest step. Added
  `docs/architecture/overview.md`.
