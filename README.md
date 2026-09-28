<h1 align="center">
  <img src="assets/ia-helper.png" width="256" height="256" alt="Internet Archive Helper">
  <br />
  IA Get - Internet Archive CLI
</h1>

<p align="center"><b>Command-line tool for downloading from Internet Archive</b></p>
<p align="center">
<img alt="GitHub Downloads" src="https://img.shields.io/github/downloads/Gameaday/ia-get-cli/total?logo=github&label=Downloads">
<img alt="CI Status" src="https://img.shields.io/github/actions/workflow/status/Gameaday/ia-get-cli/rust-ci.yml?branch=main&logo=github&label=CI">
<img alt="Rust" src="https://img.shields.io/badge/Rust-1.92%2B-orange?logo=rust">
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

**IA Get** is a high-performance command-line tool and Rust library for downloading from the Internet Archive:

- **⚡ Concurrent Downloads** - Parallel downloading with intelligent session management
- **🧠 Smart Resume** - Intelligent local file validation to resume large downloads instantly without API overhead
- **🛡️ API Compliance** - Rate-limiting, retry/backoff and a descriptive User-Agent respect Internet Archive guidelines
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

### Rust CLI & Library
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

**ia-get** is a cross-platform CLI and Rust library for downloading from the Internet Archive. Run it with no arguments for an interactive terminal UI, or pass an identifier/URL directly:

```shell
# Launch the interactive terminal UI
ia-get

# Download directly from the command line
ia-get https://archive.org/details/<identifier>

# Bare identifiers are accepted too
ia-get <identifier>

# Show help and available options
ia-get --help
```

## 🎯 Features

- 🔽 **Fast concurrent downloads** - parallel file downloads with configurable limits
- 🧠 **Smart resume** - byte-range resume of interrupted transfers, validated against local files
- 🗂️ **Directory structure** - preserves the original archive layout
- 🎯 **Advanced filtering** - by file format, size, and source type (original/derivative/metadata)
- 📊 **Progress tracking** - human-readable progress and statistics
- 🗜️ **Compression** - HTTP compression and automatic archive extraction
- 🔒 **Data integrity** - MD5 checksum verification
- 🛡️ **API compliance** - descriptive User-Agent, retry/backoff, and rate-limit handling
- 🔍 **Search & batch** - search archive.org and download many identifiers from a list

## 🚀 Advanced Usage

```shell
# Filter by file types and size
ia-get --include pdf,epub --max-size 100MB https://archive.org/details/books_archive

# Only original files
ia-get --original-only https://archive.org/details/software_archive

# Custom output directory
ia-get --output ./downloads https://archive.org/details/software_archive

# Search and batch download
ia-get search "vintage computers" --limit 20
ia-get batch identifiers.txt --output ./downloads --parallel 3
```

## 🛡️ Integrity Verification

All releases include SHA256 checksums:

```bash
curl -LO https://github.com/Gameaday/ia-get-cli/releases/latest/download/RELEASE_HASHES.txt
sha256sum -c RELEASE_HASHES.txt
```

## 🏗️ Development

```shell
cargo build --release     # optimized production build
cargo build               # fast development build
cargo test                # run the test suite
cargo clippy --all-targets -- -D warnings
cargo fmt

# Fast development profile
cargo build --profile fast-dev
```

**Requirements:** a recent stable Rust toolchain (see `rust-version` in `Cargo.toml`). No system/FFI dependencies are required for the CLI or library.

### Build Profiles
- **`dev`**: fast compilation for development
- **`fast-dev`**: minimal optimization for quick iteration
- **`release`**: maximum optimization for production

## 🧪 Quality Assurance

```shell
cargo test --all-targets
cargo fmt --check
cargo clippy --all-targets -- -D warnings
./scripts/validate-build.sh   # runs the above plus security/outdated audits
```

## CI/CD & Automated Builds 🔄

- **Platforms**: Linux (x86_64, musl, aarch64), Windows (x86_64, code-signed), macOS (Intel + Apple Silicon)
- **Workflows**: `.github/workflows/rust-ci.yml` (test + build), `.github/workflows/release.yml` (tagged releases)
- **Artifacts**: native binaries plus SHA256 checksums; signed Windows executables on tagged releases

## 🏗️ Architecture

ia-get is organised in layers, and the same core powers both the CLI and the library:

- `core` - archive metadata, download engine, session state
- `infrastructure` - HTTP client, Archive.org API client, configuration, persistence
- `utilities` - formatting, filters, compression, performance helpers
- `interface` - the CLI and the interactive terminal UI

The library is published as the `ia_get` crate; the `ia-get` binary is a thin CLI over it.

## 🌐 Community & Contributions

We welcome contributions from developers, researchers, and Internet Archive enthusiasts! Whether you want to:

- **🐛 Report bugs** or suggest improvements
- **💻 Contribute code** or documentation
- **🎨 Improve the user interface**
- **📚 Help with translations**

Check out our [Contributing Guidelines](CONTRIBUTING.md) to get started. Every contribution helps make Internet Archive content more accessible to everyone.
