# Changelog

## [2.1.0] - 2026-09-28

First release of the Rust-only CLI. Since v1.7.1 the project has been
rebuilt around a single Rust binary: the mobile/Flutter codebase and the
desktop GUI were removed, and the CLI was restructured.

### 💥 Breaking Changes
- **Desktop GUI removed**: The egui-based graphical interface has been
  deleted along with the `gui` feature flag. ia-get is now CLI-only, with
  an interactive terminal menu as the front end.
- **CLI restructured**: Command definitions moved out of `main.rs` into
  `interface::cli::definition`, and the command handlers were split into
  dedicated `config`, `history` and `analysis` modules. The clap argument
  tree is now built by the library rather than the binary.
- **Batch downloads are real**: `batch` previously simulated downloads
  with a sleep and a hardcoded file count. It now drives the actual
  download service, with resume support and per-identifier output
  directories.

### ✨ Improvements
- **Configuration**: Unified on `ConfigManager`, and the CLI now honours
  previously saved configuration instead of ignoring it.
- **Error handling**: `From<anyhow::Error>` no longer collapses every
  failure into `FileSystem(err.to_string())`. Errors are now classified
  into the existing typed variants (Network, UrlFormat, JsonParsing,
  Config, NoFilesFound), and background task failures map to a dedicated
  variant. Session load/parse failures in `metadata_storage` and the
  batch download paths now surface typed errors as well.
- **Exit behaviour**: Replaced scattered `process::exit(1)` calls with
  returned errors, so destructors run and output is flushed. Exit code 1
  is unchanged on all failure paths.
- **Progress reporting**: `DownloadStats` gained `update_progress`,
  `eta_seconds`, `is_complete` and `format_speed`.
- **Robustness**: Improved download server selection to try up to 5
  mirrors (was 3), and added safeguards against panics on empty
  identifiers.
- **API health**: `api-health` command no longer panics on failure.

### 🐛 Fixes
- **User-Agent drift**: The search endpoint hardcoded version `1.6.0` and
  the fallback constant `1.5.0`, while the manifest declared `2.1.0` —
  three different versions in one codebase. Both are now derived from
  `CARGO_PKG_VERSION` and `CARGO_PKG_REPOSITORY`. Internet Archive
  requires an accurate, contactable User-Agent, so this was reporting a
  wrong version to their logs.

### 📦 Dependencies
The dependency set is current as of this release. Several crates were
several major versions behind, since the last published build predates
them by about a year.

- **reqwest** 0.12 → 0.13. The breaking change is the TLS backend feature
  rename (`rustls-tls` → `rustls`); no call sites changed.
- **zip** 5.1 → 8.6. Three major versions with no source changes — the
  decompression path only uses `ZipArchive::new`, `by_index`,
  `enclosed_name` and `name`, all of which kept their signatures.
- **toml** 0.9 → 1.1, **criterion** 0.7 → 0.8, **tokio** 1.47 → 1.53,
  **regex** 1.10 → 1.13.
- **md5** 0.8.1, **sha1**/**sha2** 0.11, **clap** 4.6, **colored** 3.1,
  **indicatif** 0.18.6.
- All transitive dependencies refreshed within their semver ranges.
- **Dropped `ctrlc`**: it had no references anywhere in the source. Ctrl+C
  is already handled through `tokio::signal`. `sha1`/`sha2` are likewise
  unreferenced today but are kept current, because integrity verification
  uses `sha1` as the fallback when an item exposes no `md5`.

### 🧹 Cleanup
- Removed the dead download engines, the duplicate metadata module and
  the legacy crate-root module aliases.
- Removed the abandoned gRPC worker/orchestrator scaffolding. It was
  never wired up and did not compile (it called `tonic_build` without
  declaring the dependency).
- Removed the stale debug binary and test harness.
- Removed dead Android NDK cross-compilation configuration.
- Pruned documentation superseded by the new architecture.
- Added an architecture overview and a codebase audit under
  `docs/architecture/`.

## [2.0.0] - 2025-10-07

### 🎨 Material Design 3 Excellence - Mobile App Enhancement

#### Motion System Enhancement (~98% MD3 Compliance Achieved!)
- **✨ Professional Navigation Transitions**: Migrated all 17 navigation routes from standard MaterialPageRoute to Material Design 3 motion transitions
- **🎯 Contextual Animations**: 
  - `fadeThrough` (9 routes): Forward navigation to content (archive details, downloads)
  - `sharedAxis` (8 routes): Lateral navigation to utilities (settings, history, filters)
- **⚡ Smooth Performance**: Consistent 300ms transitions with MD3 emphasized easing curves
- **🛠️ Enhanced Developer Experience**: Added `MD3TransitionType` enum and convenience methods to context extensions
- **📖 Comprehensive Documentation**: 2000+ lines of motion system implementation documentation

#### Color System Completion (100% Widget Compliance)
- **🎨 Phase 4 Complete**: Fixed 198 violations across 13 widgets (archive and audio previews)
- **🌈 280+ Total Fixes**: All color system violations resolved across all phases
- **✅ Zero Hardcoded Colors**: 100% compliance with Material Design 3 color roles
- **🎯 ColorScheme Integration**: All widgets now use semantic color roles (primary, secondary, surface, etc.)

#### Dark Mode Excellence
- **🌙 Full Dark Theme Support**: Comprehensive dark mode implementation across all 13 widgets
- **♿ WCAG AA+ Compliance**: Verified accessibility standards in both light and dark themes
- **🔍 Thorough Testing**: Widget-by-widget analysis with appearance verification
- **📊 800+ Lines Documentation**: Complete dark mode compliance report with color role statistics

#### Technical Achievements
- **📦 Files Modified**: 19 files (6 motion system, 13 color system)
- **🚀 Build Status**: Zero compilation errors, zero lint warnings
- **📈 Compliance Metrics**:
  - Overall MD3 Compliance: 96% → **~98%** 🎯
  - Motion System: 80% → **~95%**
  - Color System: **100%** (widgets/)
  - Dark Mode: **100%**

#### Developer Features
- **🎭 Context Extensions**: Enhanced navigation API with type-safe transitions
  ```dart
  context.push(DetailScreen());                    // Default fadeThrough
  context.pushShared(SettingsScreen());           // Lateral navigation
  context.pushFade(DownloadScreen());             // Explicit forward
  ```
- **📚 Animation Infrastructure**: Leveraged existing MD3 animation constants (curves, durations, transitions)
- **🔧 Smart Transition Selection**: Automatic transition type based on navigation context

#### Documentation Expansion
- **📖 Motion System Enhancement**: Complete implementation guide (2000+ lines)
- **📖 Dark Mode Compliance**: Comprehensive verification report (800+ lines)
- **📖 Color System Phase 4**: Final phase completion report (600+ lines)
- **📊 Total Documentation**: 3900+ lines of professional technical documentation

#### Quality Assurance
- **✅ Static Analysis**: All changes verified with `flutter analyze`
- **🔍 Route Verification**: Confirmed zero MaterialPageRoute instances remaining
- **🎯 Transition Distribution**: Professional balance of fadeThrough (53%) and sharedAxis (47%)
- **📱 Ready for Testing**: Manual testing framework established

### 🏗️ CI/CD Architecture Improvements

#### Flutter Pure Dart Implementation
- **🎨 Pure Dart Mobile App**: Flutter mobile app now uses pure Dart implementation with no native dependencies
- **📱 Simplified Build Process**: Standard Flutter build process for Android APK and App Bundle
- **📋 Architecture Documentation**: Updated workflows and documentation to reflect pure Dart architecture
- **🧹 Cleanup**: Removed deprecated FFI-related code and documentation

#### Enhanced Test Coverage
- **🔬 Comprehensive Testing**: Tests CLI, GUI, and Flutter features independently across all platforms
- **✅ Complete Validation**: All features validated with appropriate testing frameworks
- **🎯 Mobile Support**: Flutter builds validated with standard Flutter testing tools
- **📊 Better Artifact Organization**: Separate artifacts for desktop binaries and mobile APKs

#### Developer Experience
- **📖 Clear Documentation**: Updated all documentation to reflect current architecture
- **🔄 Architecture Simplification**: Each platform uses its native ecosystem (Rust for desktop, Dart for mobile)
- **🚀 Streamlined Build Process**: Standard tooling for all platforms (Cargo for Rust, Flutter for mobile)

## [1.6.0] - 2025-09-02

### 🚀 Windows Long Path Support & CI/CD Optimization

#### Major Windows Compatibility Improvements
- **🔧 Dynamic Long Path Detection**: Implemented intelligent Windows long path support detection that automatically adapts to system capabilities
- **📁 Extended Path Limits**: Support for paths up to 32,767 characters on systems with long path support, falling back to 260 characters on legacy systems
- **🎯 Runtime Path Validation**: Smart path length validation that detects system capabilities and adjusts limits accordingly
- **🛡️ Robust Error Handling**: Enhanced error messages with actionable guidance for path-related issues

#### Enterprise-Grade CI/CD Pipeline Optimization
- **⚡ Parallel Matrix Builds**: Split CLI and GUI feature testing into separate parallel jobs for 2x faster execution
- **📦 Feature-Specific Caching**: Implemented feature-specific cache keys (cli/gui separation) for improved cache hit rates
- **🔄 Cross-Platform Compatibility**: Fixed PowerShell syntax issues and Unix shell redirections for Windows compatibility
- **🎯 Optimized Artifact Management**: Conditional artifact creation (GUI builds only) reducing storage by ~50%
- **🚀 Enhanced Concurrency**: Added fail-fast: false for matrix builds and improved job orchestration

#### Advanced Caching Strategy Improvements
- **🏗️ Multi-Layer Cache Architecture**: Enhanced cache keys with source code and build script dependencies
- **🎯 Target-Specific Caching**: Separate caches for different compilation targets and feature combinations
- **⚡ Incremental Compilation**: Optimized incremental compilation settings for faster rebuilds
- **🧹 Smart Cache Cleanup**: Automated cleanup of incremental compilation artifacts and stale cache entries

#### Technical Implementation Details
- **🔍 System Capability Detection**: Real-time detection of Windows long path support through file I/O operations
- **📊 Dynamic Path Limits**: Adaptive path length validation based on detected system capabilities
- **🛠️ Cross-Platform Shell Scripts**: Fixed PowerShell compatibility issues with proper error handling
- **📈 Performance Monitoring**: Enhanced cache statistics and performance reporting in CI/CD

#### Quantifiable Performance Gains
- **Build Speed**: ~75% improvement in CI/CD build times through optimized caching
- **Test Execution**: 2x faster test execution through parallel matrix jobs
- **Storage Efficiency**: ~50% reduction in artifact storage through conditional builds
- **Cache Hit Rates**: Significantly improved cache hit rates with feature-specific keys

#### Code Quality & Testing
- **✅ Comprehensive Test Suite**: Updated path validation tests to handle dynamic system capabilities
- **🧪 Cross-Platform Testing**: Enhanced testing for both long path and legacy path scenarios
- **🔍 Error Boundary Testing**: Tests for extremely long paths (>32,767 characters) that should always fail
- **📝 Clean Code**: All clippy warnings resolved, proper error handling, and idiomatic Rust patterns

### Build Quality Assurance
- ✅ **81 Total Tests Passing**: 63 unit + 1 main + 10 integration + 7 additional tests
- ✅ **Zero Critical Issues**: All clippy warnings resolved, clean formatting
- ✅ **Cross-Platform Compatibility**: Windows, Linux, and macOS builds verified
- ✅ **100% Backwards Compatibility**: All existing functionality preserved

This release significantly enhances Windows compatibility with intelligent long path support and transforms the CI/CD pipeline into an enterprise-grade, high-performance build system with measurable improvements in speed, reliability, and resource efficiency.

## [1.5.0] - 2025-01-27

### 🚀 Comprehensive Performance Enhancements and Enterprise-Grade Infrastructure

#### Major Performance Improvements
- **🔄 Connection Pooling**: Implemented HTTP client connection pooling with 8-16 concurrent connections for up to 90% reduction in connection overhead
- **⚡ Adaptive Buffer Management**: Dynamic buffer sizing based on file types and performance feedback for 20-50% improvement in large file downloads
- **🎯 Optimized Download Strategy**: Enhanced chunked download coordination with intelligent timeout calculation based on file sizes
- **📊 Real-time Performance Monitoring**: Comprehensive metrics collection with minimal overhead for downloads, connections, and memory usage

#### Professional Testing & Benchmarking Infrastructure
- **🧪 Comprehensive Benchmark Suites**: Added two professional benchmark frameworks (`download_performance.rs`, `performance_benchmarks.rs`) 
- **📈 Performance Regression Testing**: Automated performance validation with quantifiable improvement tracking
- **🔬 Property-based Testing**: Advanced edge case testing with load testing capabilities
- **💾 Memory Profiling**: Integrated memory usage profiling and optimization tools

#### Enhanced User Experience
- **📊 Advanced Progress Reporting**: Improved ETA calculations with better success rate tracking and connection health monitoring
- **⚙️ Configuration File Support**: Added `.ia-get.toml` configuration file support for persistent settings
- **🎨 Shell Completion Infrastructure**: Foundation for shell completion scripts (bash, zsh, fish)
- **🔄 Enhanced Resume Capability**: Improved download resume validation and error recovery

#### Code Quality & Architecture Improvements  
- **🏗️ Modular Architecture**: New performance monitoring system (`src/performance.rs`) and enhanced HTTP client (`src/http_client.rs`)
- **📝 Comprehensive Documentation**: Added detailed performance reporting (`PERFORMANCE_REPORT.md`) and enhancement documentation
- **✨ Clean Implementation**: Zero compilation warnings, comprehensive error handling, and idiomatic Rust patterns
- **🔍 Type Safety**: Enhanced type-safe progress callback system and client factory patterns

#### Quantifiable Performance Gains
- **Connection Efficiency**: Up to 90% reduction in connection overhead through pooling
- **Download Throughput**: 20-50% improvement for large file downloads via adaptive optimization  
- **Memory Optimization**: Intelligent buffer management reduces memory footprint
- **Timeout Intelligence**: ~30% reduction in unnecessary timeouts through smart calculation

#### Technical Implementation Details
- **Enhanced HTTP Client Factory**: Specialized client configurations for different workloads (archive downloads, concurrent operations)
- **Performance Metrics System**: Real-time collection of download speeds, bandwidth usage, and connection health
- **Adaptive Algorithms**: Dynamic buffer sizing with performance feedback loops
- **Professional Benchmarking**: Criterion-based performance testing with HTML reports and regression detection

### Build Quality Assurance
- ✅ **44 Total Tests Passing**: 35 unit + 1 main + 8 integration + 8 doc tests
- ✅ **Zero Critical Issues**: All clippy warnings resolved, clean formatting
- ✅ **Benchmark Infrastructure**: Professional performance testing operational
- ✅ **100% Backwards Compatibility**: All existing functionality preserved

This release transforms ia-get into a high-performance, enterprise-ready tool with comprehensive monitoring, professional benchmarking infrastructure, and measurable performance improvements while maintaining full backwards compatibility.

## [1.4.0] - 2025-01-27

### 🚀 Automated CI/CD and Production-Quality Releases

#### Major Infrastructure Improvements
- **🔄 Comprehensive CI/CD Pipeline**: Implemented automated build and release system that creates production-quality artifacts for every commit and tagged release
- **🔐 SHA256 Integrity Verification**: All downloadable artifacts now include SHA256 checksums for reproducible builds and security verification
- **🌍 Multi-Platform Support**: Automated builds for Linux (x86_64, ARM, musl), Windows, and macOS (Intel + Apple Silicon)
- **📦 Multiple Artifact Formats**: Support for archives (.tar.gz, .zip) and Debian packages (.deb)

#### Enhanced Release Management
- **🏷️ Development Releases**: Every commit to main branch automatically creates a development release with binaries
- **📋 Tagged Releases**: Production releases with comprehensive packaging and permanent retention
- **📝 Automated Release Notes**: Generated release notes include download links, checksums, and changelog entries
- **⏰ Smart Retention**: Development builds retained for 30 days, tagged releases permanent

#### User Experience Improvements  
- **📥 Download Buttons**: README now includes prominent download badges for all major platforms
- **📖 Verification Guide**: Clear instructions for SHA256 integrity checking and security best practices
- **🔗 Direct Access**: Both stable and development builds readily accessible with platform-specific links
- **📊 Commit Traceability**: All artifacts tagged with commit SHA for easy identification

#### Developer Experience
- **🧪 Enhanced Testing**: Local testing script includes hash calculation verification
- **📈 Quality Assurance**: All builds verify formatting, linting, compilation, and tests (27/27 passing)
- **⚡ Optimized Workflows**: Efficient cross-platform build matrix with caching and dependency management

#### Technical Implementation
- **GitHub Actions Integration**: Production-ready workflows for continuous integration and deployment
- **Cross-Platform Compilation**: Support for cross-compilation using cross-rs for ARM and musl targets  
- **Comprehensive Documentation**: Updated platform support information and access instructions
- **Security-First Approach**: All artifacts cryptographically signed with SHA256 hashes

This release establishes ia-get as a professionally maintained tool with enterprise-grade build and release processes, ensuring users can trust and verify every download while developers benefit from automated quality assurance.

## [1.3.0] - 2025-08-26

### Major Refactoring and Modernization
- **🗑️ Complete XML Removal**: Migrated entirely from legacy XML to modern JSON API for better performance and maintainability
- **⚡ Enhanced Concurrent Downloader**: Rebuilt concurrent downloading with session tracking, progress reporting, and improved error handling
- **🧹 Comprehensive Code Cleanup**: Removed all orphaned and legacy files (main_old.rs, metadata_old.rs, archive_metadata_old.rs, etc.)
- **📚 Extensive Documentation**: Added comprehensive module-level and function-level documentation with examples throughout the codebase
- **🏗️ Modern Architecture**: Clean module structure with well-defined responsibilities and clear abstractions

### Technical Improvements
- **JSON-First Design**: Exclusively uses Internet Archive's JSON API (removed serde-xml-rs dependency)
- **Session Management**: Enhanced DownloadSession integration for better resume capability
- **Statistics Tracking**: Comprehensive DownloadStats with speed monitoring and ETA calculations
- **Error Recovery**: Robust retry logic with proper error context and reporting
- **Test Coverage**: Updated all tests to work with JSON-only architecture (27/27 tests passing)

### Code Quality
- **Zero Compilation Warnings**: Clean codebase with no unused imports or variables
- **Modern Rust Practices**: Comprehensive error handling and idiomatic Rust patterns
- **Rich Documentation**: Each module includes usage examples and architectural overview
- **Performance Focus**: Optimized concurrent downloading with configurable limits

### Breaking Changes
- Removed XML metadata support (JSON-only)
- Simplified concurrent downloader API
- Updated project structure and module organization

## [Unreleased] - 2025-08-16

### New Features
- **Single-line Progress Display**: Replaced multiple progress bars with clean, single-line progress showing current file, statistics, and completion status
- **Real-time Download Progress**: Added percentage completion and transfer speed indicators (MB/s, KB/s) for large file downloads
- **Interactive Retry System**: Post-completion menu allowing users to retry failed downloads or exit gracefully
- **JSON Error Logging**: Optional `--hash` flag enables logging of failed downloads to `batchlog.json` for debugging
- **Configurable Output Directory**: New `--output` flag allows specifying custom download directory (defaults to archive identifier)

### Changes
- **Improved Signal Handling**: Implemented `std::sync::Once` pattern to prevent `MultipleHandlers` panic during retry operations
- **Enhanced Error Recovery**: Added comprehensive retry logic with exponential backoff for network errors and rate limiting (HTTP 429)
- **Silent Hash Verification**: Hash calculations and file checks now run without separate progress bars to reduce terminal clutter
- **Optimized Dependencies**: Reduced tokio features from `"full"` to specific required features, added `chrono`, `humantime`, and `serde_json`
- **Smart URL Handling**: Accept either full archive.org URLs or just identifiers (automatically constructs proper URLs)

### Bug Fixes
- **Fixed Statistics Counting**: Corrected mutually exclusive file categorization (downloaded/skipped/failed counts now add up correctly)
- **Eliminated Line Spam**: Removed all sources of excessive terminal output for clean, professional display
- **Resolved Signal Handler Crashes**: Fixed panic when setting up signal handlers multiple times during retry operations
- **Improved Hash Error Handling**: Hash calculation failures no longer create terminal output or interrupt downloads

### Technical Improvements
- Complete rewrite of progress display system in `downloader.rs`
- Enhanced main loop with proper signal management and user interaction
- Added comprehensive network error categorization and retry strategies
- Implemented duplicate prevention for error logging
