<h1 align="center">
  <img src="assets/ia-helper.png" width="256" height="256" alt="Internet Archive Helper">
  <br />
  IA Get - Internet Archive CLI
</h1>

<p align="center"><b>Command-line tool for downloading from Internet Archive</b></p>
<p align="center">
<img alt="GitHub Downloads" src="https://img.shields.io/github/downloads/Gameaday/ia-get-cli/total?logo=github&label=Downloads">
<img alt="CI Status" src="https://img.shields.io/github/actions/workflow/status/Gameaday/ia-get-cli/ci.yml?branch=main&logo=github&label=CI">
<img alt="Rust" src="https://img.shields.io/badge/Rust-1.70+-orange?logo=rust">
</p>

<p align="center">Built with ❤️ for the Internet Archive community</p>

> **Note**: This is an unofficial, community-developed project and is not affiliated with or endorsed by the Internet Archive.

---

## � Looking for the Mobile App?

The **Flutter mobile app** has moved to its own repository for better development workflow:

### [**IA Helper** - Mobile Companion App](https://github.com/gameaday/ia-helper)

<p align="center">
  <a href="https://github.com/gameaday/ia-helper">
    <img src="https://img.shields.io/badge/Android-Coming%20Soon-green?logo=android&style=for-the-badge" alt="Android App" />
  </a>
</p>

**Mobile App Features:**
- � Beautiful Material Design 3 interface
- 🔍 Search 35+ million Internet Archive items
- 📥 Smart download queue with resume capability
- � Offline library management
- 🌙 Full dark mode support
- 🔐 Privacy-first (no tracking, no ads)

[**Download IA Helper →**](https://github.com/gameaday/ia-helper)

---

## 🖥️ Rust CLI Tool (This Repository)

**IA Get** is a high-performance command-line tool for downloading from Internet Archive:

- **⚡ Concurrent Downloads** - Parallel downloading with intelligent session management
- **🖼️ Desktop GUI** - Native desktop interface (egui framework)
- **⌨️ CLI Mode** - Powerful command-line for automation and scripts
- **🗜️ Compression** - HTTP compression and automatic archive extraction
- **🎯 Advanced Filtering** - Filter by file type, size, patterns
- **📊 Performance** - Zero-cost abstractions, minimal overhead

**Platform Support:**
- ✅ Linux (x86_64, ARM, musl)
- ✅ Windows (x86_64, code-signed)
- ✅ macOS (Intel + Apple Silicon)

---

## 📥 Quick Download

<div align="center">

### ️ Rust CLI & Desktop GUI
[🐧 Linux](https://github.com/Gameaday/ia-get-cli/releases/latest) | [🪟 Windows](https://github.com/Gameaday/ia-get-cli/releases/latest) | [🍎 macOS](https://github.com/Gameaday/ia-get-cli/releases/latest)

**📋 [Complete Downloads & Installation Guide →](DOWNLOADS.md)**

### 📱 Mobile App
Looking for the mobile app? Check out **[IA Helper](https://github.com/gameaday/ia-helper)**

</div>

### 🔐 Security & Trust
- **Windows binaries are code-signed** to prevent SmartScreen warnings
- **SHA256 checksums** provided for all releases
- **Automated security audits** on every commit

---

## ⚡ Quick Start

**IA Get** provides both CLI and GUI interfaces with smart auto-detection:

```shell
# Auto-detect best mode (GUI if available, menu otherwise)
ia-get

# Download directly from command line
ia-get https://archive.org/details/<identifier>

# Show help and available options
ia-get --help
```

**Smart Interface Detection**: Automatically chooses the best interface - GUI when display is available, falls back to interactive menu or CLI mode based on your environment.

## 🎯 Features

### 🏗️ Architecture

**Two Independent Implementations:**

| Feature | Flutter Mobile/Web | Rust CLI/Server |
|---------|-------------------|-----------------|
| **Purpose** | Cross-platform GUI | High-performance automation |
| **Platforms** | Android, Web, iOS (soon), Desktop (soon) | Linux, Windows, macOS |
| **UI** | Material Design 3, Responsive | egui (desktop GUI) + CLI |
| **Downloads** | Background, pause/resume | Concurrent, session management |
| **Target Users** | Mobile users, casual browsing | Power users, automation, servers |

Both implementations provide feature parity for core functionality.

---

### Core Functionality (All Platforms)
- 🔽 **Fast Concurrent Downloads** - Parallel downloading with configurable limits
- 🌳 **Directory Structure** - Preserves original archive organization  
- 🔄 **Smart Resume** - Automatically recovers from interruptions
- 🎯 **Advanced Filtering** - Filter by file type, size, patterns
- � **Progress Tracking** - Real-time speed and ETA information
- 🔒 **Data Integrity** - MD5/SHA256 checksum verification

### Flutter Mobile/Web Specific
- 🎨 **Material Design 3** - Modern, accessible UI (78% compliant)
- 📱 **Responsive Design** - Optimized for phones, tablets, desktop browsers
- 🔗 **Deep Links** - Open archive.org URLs directly in app
- 📥 **Background Downloads** - Continue when app is closed
- 🌐 **Offline Mode** - Access cached metadata without internet
- ♿ **Accessibility** - Screen reader support, dynamic type

### Rust CLI/Server Specific
- 🗜️ **Compression** - HTTP compression and automatic archive extraction
- 🖼️ **Desktop GUI** - Native egui interface with visual controls
- ⌨️ **CLI Mode** - Powerful command-line for automation
- 📈 **Performance** - Zero-cost abstractions, minimal overhead
- 🔧 **Scripting** - Batch operations and integration support

## 🚀 Advanced Usage

```shell
# Concurrent downloads with compression
ia-get --compress --decompress https://archive.org/details/your_archive

# Filter by file types
ia-get --include-ext pdf,epub https://archive.org/details/books_archive

# Limit file sizes  
ia-get --max-file-size 100MB https://archive.org/details/data_archive

# Specify output directory
ia-get --output ./downloads https://archive.org/details/software_archive
```

### GUI Features
The GUI provides smart detection, easy archive input with validation, visual file filtering, real-time progress tracking, settings management, and download history. See [GUI_README.md](GUI_README.md) for detailed documentation.

## 🛡️ Integrity Verification

All releases include SHA256 checksums for security verification:

```bash
# Download and verify (example for Linux x86_64)
curl -LO https://github.com/Gameaday/ia-get-cli/releases/latest/download/RELEASE_HASHES.txt
sha256sum -c RELEASE_HASHES.txt
```

## 🗜️ Compression & Decompression

```bash
# Enable compression and auto-decompression
ia-get --compress --decompress https://archive.org/details/your_archive

# Decompress specific formats only
ia-get --decompress --decompress-formats gzip,bzip2 https://archive.org/details/your_archive
```

Supports gzip, bzip2, xz, tar, and combined formats. See [docs/COMPRESSION.md](docs/COMPRESSION.md) for details.

## 🏗️ Development

### Rust CLI/Server Development

```shell
# Standard Cargo workflow
cargo build --release        # Optimized production build
cargo build                  # Fast development build
cargo test                   # Run test suite
cargo clippy                 # Linting
cargo fmt                    # Code formatting

# Build CLI only (60% faster)
cargo build --no-default-features --features cli

# Build with desktop GUI
cargo build --features gui

# Fast development profile
cargo build --profile fast-dev --no-default-features --features cli
```

**Requirements:**
- Rust 1.70+ (standard toolchain)
- No external dependencies for CLI
- egui for desktop GUI mode

---

### Flutter Mobile/Web Development

The Flutter app uses **pure Dart** - no native compilation or NDK required!

```shell
# Navigate to Flutter project
cd mobile/flutter

# Get dependencies
flutter pub get

# Run on device (auto-detects connected device/emulator)
flutter run

# Build for specific platforms
flutter build apk           # Android APK
flutter build web           # Web bundle
flutter build ios           # iOS (coming soon)
flutter build windows       # Windows desktop (coming soon)
flutter build macos         # macOS desktop (coming soon)
flutter build linux         # Linux desktop (coming soon)
```

**Requirements:**
- Flutter 3.35.5+ / Dart 3.8.0+
- No Android NDK needed
- No Rust toolchain needed
- Standard Flutter development workflow

**Current Platform Status:**
- ✅ **Android** - Full support (phones and tablets)
- ✅ **Web** - Desktop browser support
- 🔜 **iOS** - Coming soon (easy to add with pure Dart)
- 🔜 **Windows/macOS/Linux Desktop** - Coming soon

**Responsive Design:**
- 📱 **Mobile** (< 600dp) - Phone-optimized layouts
- 🖥️ **Tablet** (600-839dp) - Master-detail, two-column layouts
- 💻 **Desktop** (840dp+) - Constrained width, multi-pane views

See [mobile/flutter/README.md](mobile/flutter/README.md) for detailed documentation.
- ✅ Standard Flutter build process
- ✅ Faster builds (no native compilation)
- ✅ Easy debugging with Flutter DevTools
- ✅ Works on all Flutter platforms

For Android deployment and Play Store submission, see **[ANDROID_DEPLOYMENT_GUIDE.md](ANDROID_DEPLOYMENT_GUIDE.md)**.

### 🔧 Troubleshooting Build Issues

**Having Flutter/Dart SDK version conflicts?** Run our quick-fix script:

```shell
./scripts/fix-flutter-deps.sh
```

**Common Issues:**
- **Flutter version errors**: Ensure Flutter 3.35.0+ is installed (includes Dart 3.8.0+)
- **Dependency conflicts**: Run `flutter clean` and `flutter pub get` in `mobile/flutter/`
- **Build failures**: See **[TROUBLESHOOTING.md](TROUBLESHOOTING.md)** for comprehensive solutions

**Required Versions:**
- Flutter: 3.35.0 or higher
- Dart: 3.8.0 or higher (included with Flutter 3.35.0+)
- Rust: Latest stable (1.75.0+)

### Build Profiles
- **`dev`**: Fast compilation for development
- **`fast-dev`**: Minimal optimization for quick iteration
- **`release`**: Maximum optimization for production

### 📊 Build Optimization

The project includes significant build time optimizations:

- **Feature Gates**: CLI and GUI components separated for faster compilation
- **Build Profiles**: Multiple profiles optimized for different use cases  
- **CLI-only builds**: ~60-70% faster than full builds
- **Development iteration**: Additional 10-20% improvement with fast-dev profile

```shell
# Measure build performance
./scripts/build-benchmark.sh

# See full development guide  
cat docs/DEVELOPMENT.md
```

## 🧪 Quality Assurance

```shell
# Run tests (CLI only - fastest)
cargo test --no-default-features --features cli

# Check formatting and linting
cargo fmt --check
cargo clippy --no-default-features --features cli -- -D warnings

# Test CI locally
./scripts/test-ci.sh
```

**Test Coverage**: 81+ tests passing across unit, integration, and API validation.

## CI/CD & Automated Builds 🔄

### Separate Build Pipelines

**Rust CLI/Server:**
- **Platforms**: Linux (x86_64, ARM, musl), Windows (x86_64), macOS (Intel + Apple Silicon)
- **Artifacts**: Native binaries, code-signed Windows executables
- **Workflow**: `.github/workflows/ci.yml`
- **Toolchain**: Standard Cargo, no cross-compilation dependencies

**Flutter Mobile/Web:**
- **Platforms**: Android APK, Web bundle
- **Artifacts**: Universal APK, web deployment files
- **Workflow**: `.github/workflows/flutter-ci.yml` (coming soon)
- **Toolchain**: Standard Flutter SDK, no native dependencies

### Release Artifacts

**Every Commit** (Development Release):
- All Rust binaries for desktop platforms
- Android APK (debug build)
- SHA256 checksums

**Tagged Releases** (Production):
- Optimized Rust binaries (all platforms)
- Signed Android APK (release build)
- Web deployment bundle
- Comprehensive documentation

### Quality Assurance

```shell
# Rust: Run tests, formatting, linting
cargo test --no-default-features --features cli
cargo fmt --check
cargo clippy -- -D warnings

# Flutter: Run tests and analysis
cd mobile/flutter
flutter test
flutter analyze
```

**Test Coverage:**
- Rust: 81+ unit and integration tests
- Flutter: Comprehensive widget and unit tests
- Both: Zero linting issues required for merge

---
- **Android**: APK for all architectures (arm64-v8a, armeabi-v7a, x86_64)
- **iOS**: Coming soon (pure Dart makes this easy)
- **Web**: Experimental support
- **Desktop**: Windows, macOS, Linux via Flutter

## 🏗️ Architecture & Implementation

### Two Independent, Optimized Implementations

**Rust CLI/Server (High Performance):**
- ⚡ **Zero-cost abstractions** - Minimal overhead for maximum speed
- 🔄 **Modern JSON APIs** - Clean communication with Internet Archive
- 🧪 **Comprehensive testing** - 81+ tests ensuring reliability
- � **Memory safe** - No unsafe code (minimal unsafe blocks)
- 📦 **Standard toolchain** - Just Cargo, no special setup

**Flutter Mobile/Web (Cross-Platform GUI):**
- 🎨 **Pure Dart** - No native dependencies or FFI complexity
- 📱 **Responsive design** - Adaptive layouts for all screen sizes
- 🚀 **Hot reload** - Fast iteration with Flutter DevTools
- ♿ **Accessible** - Screen reader support, dynamic type
- 🔧 **Standard workflow** - Just Flutter SDK, no Android NDK

**Why Two Implementations?**
- Each optimized for its target platform and use case
- Rust: Maximum performance for servers and automation
- Flutter: Best UX for mobile and casual users
- No compromises - both implementations excel at their goals

---
- **🔄 Modern HTTP Client**: Direct API communication using Dart's http package
- **✅ Easy Testing**: Standard Flutter testing framework with no native complications

**Architecture Benefits:**
- ✅ Clear separation of concerns
- ✅ Independent optimization for each platform
- ✅ Simplified development and maintenance
- ✅ Both implementations maintain full feature parity

- **📦 Professional CI/CD**: Automated builds and testing across all supported platforms
- **🎯 Cross-Platform Excellence**: Native performance on desktop, mobile, and embedded systems

**Built for the future** with forward-compatible design and modern development practices.

## 🌐 Community & Contributions

We welcome contributions from developers, researchers, and Internet Archive enthusiasts! Whether you want to:

- **🐛 Report bugs** or suggest improvements
- **💻 Contribute code** or documentation
- **🎨 Improve the user interface**
- **📚 Help with translations**

Check out our [Contributing Guidelines](CONTRIBUTING.md) to get started. Every contribution helps make Internet Archive content more accessible to everyone.
