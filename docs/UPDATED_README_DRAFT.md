<h1 align="center">
  <img src="assets/ia-helper.png" width="256" height="256" alt="Internet Archive Helper">
  <br />
  Internet Archive Helper
</h1>

<p align="center"><b>Comprehensive tools for accessing Internet Archive content</b></p>
<p align="center">
<img alt="GitHub Downloads" src="https://img.shields.io/github/downloads/Gameaday/ia-get-cli/total?logo=github&label=Downloads">
<img alt="CI Status" src="https://img.shields.io/github/actions/workflow/status/Gameaday/ia-get-cli/ci.yml?branch=main&logo=github&label=CI">
<img alt="Material Design 3" src="https://img.shields.io/badge/Material%20Design%203-78%25-blue?logo=material-design">
</p>

<p align="center">Built with ❤️ for the Internet Archive community</p>

> **Note**: This is an unofficial, community-developed project and is not affiliated with or endorsed by the Internet Archive.

---

## 🚀 Two Powerful Implementations

### 📱 Flutter Mobile/Web (Cross-Platform GUI)
**Pure Dart implementation** for modern user experience:

- **🎨 Material Design 3** - Modern, responsive UI (78% compliant)
- **📱 Mobile** - Optimized for Android phones with touch interface
- **🖥️ Tablet** - Master-detail layouts, two-column views (600dp+)
- **🌐 Desktop Web** - Full-featured browser experience (840dp+)
- **🔄 Responsive Design** - Adaptive layouts for all screen sizes
- **♿ Accessible** - Screen reader support, dynamic type, semantic labels
- **📥 Background Downloads** - Continue downloads when app is closed
- **🔗 Deep Links** - Open archive.org links directly in app

**Platform Support:**
- ✅ Android (phones/tablets)
- ✅ Web (desktop browsers)
- 🔜 iOS (coming soon)
- 🔜 Windows/macOS/Linux desktop

### 🖥️ Rust CLI/Server (High-Performance Automation)
**Zero-cost Rust implementation** for servers and power users:

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

### Flutter Mobile/Web
- **🤖 Android APK**: [Latest Release](https://github.com/Gameaday/ia-get-cli/releases/latest)
- **🌐 Web App**: [Try Online](https://gameaday.github.io/ia-get-cli) *(experimental)*

### Rust CLI/Server
- **🐧 Linux**: [Latest Release](https://github.com/Gameaday/ia-get-cli/releases/latest)
- **🪟 Windows**: [Latest Release](https://github.com/Gameaday/ia-get-cli/releases/latest) *(code-signed)*
- **🍎 macOS**: [Latest Release](https://github.com/Gameaday/ia-get-cli/releases/latest)

### 🔐 Security & Trust
- **Windows binaries are code-signed** to prevent SmartScreen warnings
- **SHA256 checksums** provided for all releases
- **Automated security audits** on every commit

---

## ⚡ Quick Start

### Rust CLI
```shell
# Auto-detect best mode (GUI if available, menu otherwise)
ia-get

# Download directly from command line
ia-get https://archive.org/details/<identifier>

# Show help and available options
ia-get --help
```

### Flutter Mobile/Web
1. Download APK or visit web app
2. Enter Internet Archive URL or identifier
3. Browse files and start downloading
4. Enjoy responsive design on any device

---

## 🎯 Key Features

### Core Capabilities (All Platforms)
- 🔽 **Fast Concurrent Downloads** - Parallel downloading with configurable limits
- 🌳 **Directory Structure** - Preserves original archive organization  
- 🔄 **Smart Resume** - Automatically recovers from interruptions
- 🎯 **Advanced Filtering** - Filter by file type, size, patterns
- 📊 **Progress Tracking** - Real-time speed and ETA information
- 🔒 **Data Integrity** - MD5/SHA256 checksum verification

### Flutter Mobile/Web Specific
- 🎨 **Material Design 3** - Modern, accessible UI (78% compliant)
- 📱 **Responsive Design** - Optimized for phones, tablets, desktop browsers
  - **Mobile** (< 600dp): Phone-optimized layouts
  - **Tablet** (600-839dp): Master-detail, two-column views
  - **Desktop** (840dp+): Constrained width, multi-pane layouts
- 🔗 **Deep Links** - Open archive.org URLs directly in app
- 📥 **Background Downloads** - Continue when app is closed
- 🌐 **Offline Mode** - Access cached metadata without internet
- ♿ **Accessibility** - Screen reader support, dynamic type, WCAG 2.1 Level AA

### Rust CLI/Server Specific
- 🗜️ **Compression** - HTTP compression and automatic archive extraction
- 🖼️ **Desktop GUI** - Native egui interface with visual controls
- ⌨️ **CLI Mode** - Powerful command-line for automation
- 📈 **Performance** - Zero-cost abstractions, minimal overhead
- 🔧 **Scripting** - Batch operations and integration support

---

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

---

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

## 🏗️ Architecture & Implementation

### Two Independent, Optimized Implementations

**Rust CLI/Server (High Performance):**
- ⚡ **Zero-cost abstractions** - Minimal overhead for maximum speed
- 🔄 **Modern JSON APIs** - Clean communication with Internet Archive
- 🧪 **Comprehensive testing** - 81+ tests ensuring reliability
- 🔒 **Memory safe** - No unsafe code (minimal unsafe blocks)
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

## 🌐 Community & Contributions

We welcome contributions from developers, researchers, and Internet Archive enthusiasts! Whether you want to:

- **🐛 Report bugs** or suggest improvements
- **💻 Contribute code** or documentation
- **🎨 Improve the user interface**
- **📚 Help with translations**

Check out our [Contributing Guidelines](CONTRIBUTING.md) to get started. Every contribution helps make Internet Archive content more accessible to everyone.

---

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## ⚖️ Legal Disclaimer

This project is an independent, community-developed tool and is not affiliated with, endorsed by, or sponsored by the Internet Archive. "Internet Archive" and related trademarks are the property of Internet Archive. This tool is provided for personal and educational use only. Users are responsible for ensuring their use complies with Internet Archive's Terms of Service and applicable laws.
