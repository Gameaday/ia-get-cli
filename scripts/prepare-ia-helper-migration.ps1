# IA Helper Repository Preparation Script
# This script prepares files for migration to the new ia-helper repository

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "IA Helper Migration Preparation" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$sourceRoot = "c:\Project\ia-get-cli-github\ia-get-cli"
$targetRoot = "c:\Project\ia-helper"

Write-Host "Source: $sourceRoot" -ForegroundColor Yellow
Write-Host "Target: $targetRoot" -ForegroundColor Green
Write-Host ""

# Check if target exists
if (Test-Path $targetRoot) {
    Write-Host "⚠️  Target directory already exists: $targetRoot" -ForegroundColor Red
    $response = Read-Host "Do you want to continue? This will overwrite files. (yes/no)"
    if ($response -ne "yes") {
        Write-Host "❌ Migration cancelled." -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "✅ Target directory does not exist. Will create: $targetRoot" -ForegroundColor Green
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Step 1: Create Directory Structure" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# Create target directory
New-Item -ItemType Directory -Force -Path $targetRoot | Out-Null
New-Item -ItemType Directory -Force -Path "$targetRoot\.github\workflows" | Out-Null
New-Item -ItemType Directory -Force -Path "$targetRoot\docs\features" | Out-Null

Write-Host "✅ Created base directory structure" -ForegroundColor Green

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Step 2: Copy Flutter App Files" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

$flutterSource = "$sourceRoot\mobile\flutter"

# Core Flutter files
$coreDirs = @("lib", "test", "android", "ios", "web", "assets")
foreach ($dir in $coreDirs) {
    $sourcePath = "$flutterSource\$dir"
    if (Test-Path $sourcePath) {
        Write-Host "📁 Copying $dir..." -ForegroundColor Cyan
        Copy-Item -Path $sourcePath -Destination $targetRoot -Recurse -Force
        Write-Host "   ✅ Copied $dir" -ForegroundColor Green
    } else {
        Write-Host "   ⚠️  $dir not found (might not exist yet)" -ForegroundColor Yellow
    }
}

# Flutter config files
$configFiles = @(
    "pubspec.yaml",
    "pubspec.lock",
    "analysis_options.yaml",
    ".gitignore",
    ".metadata",
    "build.yaml"
)

foreach ($file in $configFiles) {
    $sourcePath = "$flutterSource\$file"
    if (Test-Path $sourcePath) {
        Write-Host "📄 Copying $file..." -ForegroundColor Cyan
        Copy-Item -Path $sourcePath -Destination $targetRoot -Force
        Write-Host "   ✅ Copied $file" -ForegroundColor Green
    } else {
        Write-Host "   ⚠️  $file not found" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Step 3: Copy Documentation" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# Documentation files
$docFiles = @(
    "docs\PLAY_STORE_METADATA.md",
    "docs\ANDROID_PERMISSIONS.md",
    "docs\FLUTTER_APP_MIGRATION.md",
    "docs\features\PHASE_4_TASK_3_COMPLETE.md",
    "docs\features\PHASE_5_PLAN.md",
    "docs\features\PHASE_5_TASK_1_PROGRESS.md"
)

foreach ($file in $docFiles) {
    $sourcePath = "$sourceRoot\$file"
    $targetPath = "$targetRoot\$file"
    
    if (Test-Path $sourcePath) {
        Write-Host "📄 Copying $file..." -ForegroundColor Cyan
        $targetDir = Split-Path -Parent $targetPath
        New-Item -ItemType Directory -Force -Path $targetDir | Out-Null
        Copy-Item -Path $sourcePath -Destination $targetPath -Force
        Write-Host "   ✅ Copied $file" -ForegroundColor Green
    } else {
        Write-Host "   ⚠️  $file not found" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Step 4: Copy CI/CD and Configs" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# CI/CD workflow
$ciSource = "$sourceRoot\.github\workflows\flutter-ci.yml"
if (Test-Path $ciSource) {
    Write-Host "📄 Copying flutter-ci.yml..." -ForegroundColor Cyan
    Copy-Item -Path $ciSource -Destination "$targetRoot\.github\workflows\flutter-ci.yml" -Force
    Write-Host "   ✅ Copied CI workflow" -ForegroundColor Green
} else {
    Write-Host "   ⚠️  flutter-ci.yml not found" -ForegroundColor Yellow
}

# Copilot instructions
$copilotSource = "$sourceRoot\.github\copilot-instructions.md"
if (Test-Path $copilotSource) {
    Write-Host "📄 Copying copilot-instructions.md..." -ForegroundColor Cyan
    Copy-Item -Path $copilotSource -Destination "$targetRoot\.github\copilot-instructions.md" -Force
    Write-Host "   ✅ Copied Copilot instructions" -ForegroundColor Green
} else {
    Write-Host "   ⚠️  copilot-instructions.md not found" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Step 5: Copy Legal Files" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# Legal files
$legalFiles = @("LICENSE", "PRIVACY_POLICY.md")

foreach ($file in $legalFiles) {
    $sourcePath = "$sourceRoot\$file"
    if (Test-Path $sourcePath) {
        Write-Host "📄 Copying $file..." -ForegroundColor Cyan
        Copy-Item -Path $sourcePath -Destination "$targetRoot\$file" -Force
        Write-Host "   ✅ Copied $file" -ForegroundColor Green
    } else {
        Write-Host "   ⚠️  $file not found" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Step 6: Create New Repository Files" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# Create README.md
$readmeContent = @"
# IA Helper

[![Flutter CI](https://github.com/gameaday/ia-helper/actions/workflows/flutter-ci.yml/badge.svg)](https://github.com/gameaday/ia-helper/actions/workflows/flutter-ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**Your complete Internet Archive companion** - Download, search, and organize content from archive.org with a beautiful Material Design 3 interface.

<p align="center">
  <img src="docs/images/app-preview.png" alt="IA Helper Preview" width="800"/>
</p>

## 📱 About

IA Helper is a powerful mobile app for accessing the Internet Archive (archive.org), the world's largest digital library. Whether you're downloading historical documents, discovering classic media, or building your personal digital archive, IA Helper makes it effortless.

### ✨ Key Features

- 📥 **Smart Downloads** - Resume interrupted downloads, queue management, priority scheduling
- 🔍 **Advanced Search** - Search 35+ million items with powerful filters
- 📚 **Library Management** - Organize downloads, offline access, metadata viewer
- ⚡ **Lightning Fast** - Concurrent downloads with automatic retry
- 🎨 **Material Design 3** - Beautiful UI with full dark mode support
- 🔐 **Privacy First** - No tracking, no ads, local storage only

## 📥 Download

[<img src="https://play.google.com/intl/en_us/badges/static/images/badges/en_badge_web_generic.png" alt="Get it on Google Play" height="80">](https://play.google.com/store/apps/details?id=com.gameaday.iahelper)

*Coming soon to Google Play Store*

## 🏗️ Built With

- **Flutter** 3.35.0 - Cross-platform UI framework
- **Dart** - Programming language
- **Material Design 3** - Google's latest design system
- **SQLite** - Local database for downloads and metadata
- **HTTP/2** - Fast and efficient networking

## 🎨 Design

IA Helper follows Material Design 3 guidelines with ~98% compliance:
- ✅ MD3 color system with dynamic theming
- ✅ MD3 typography scale
- ✅ MD3 motion system (curves and durations)
- ✅ MD3 component library
- ✅ Adaptive layouts for phones and tablets
- ✅ Full dark mode support
- ✅ WCAG AA+ accessibility compliance

## 🚀 Getting Started

### Prerequisites

- Flutter 3.35.0 or higher
- Dart 3.5.0 or higher
- Android Studio or VS Code
- Android SDK (API 21+)

### Installation

``````bash
# Clone the repository
git clone https://github.com/gameaday/ia-helper.git
cd ia-helper

# Install dependencies
flutter pub get

# Run the app
flutter run
``````

### Building

``````bash
# Debug build
flutter build apk --flavor development

# Release build
flutter build apk --flavor production --release
flutter build appbundle --flavor production --release
``````

## 📖 Documentation

- [Play Store Metadata](docs/PLAY_STORE_METADATA.md) - App descriptions and store listing
- [Android Permissions](docs/ANDROID_PERMISSIONS.md) - Detailed permission explanations
- [Phase 5 Plan](docs/features/PHASE_5_PLAN.md) - Development roadmap
- [Migration Guide](docs/FLUTTER_APP_MIGRATION.md) - Migration from ia-get-cli

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and development process.

### Development Guidelines

- Follow Material Design 3 principles
- Write tests for new features
- Update documentation
- Use conventional commits
- Run ``flutter analyze`` before committing

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔐 Privacy

IA Helper is privacy-focused:
- ❌ No user tracking or analytics
- ❌ No ads or in-app purchases
- ❌ No data collection
- ✅ All data stored locally on your device
- ✅ No account required

Read our [Privacy Policy](PRIVACY_POLICY.md) for more details.

## 🙏 Acknowledgments

- [Internet Archive](https://archive.org) - For preserving the world's knowledge
- [Flutter](https://flutter.dev) - For the amazing framework
- [Material Design](https://m3.material.io) - For design guidelines

## 📧 Contact

- **Email**: gameaday.project@gmail.com
- **Issues**: [GitHub Issues](https://github.com/gameaday/ia-helper/issues)
- **Discussions**: [GitHub Discussions](https://github.com/gameaday/ia-helper/discussions)

## 🔗 Related Projects

- [ia-get CLI](https://github.com/gameaday/ia-get-cli) - Rust command-line tool for Internet Archive

---

**Not affiliated with Internet Archive**  
IA Helper is an independent third-party client and is not officially affiliated with or endorsed by the Internet Archive.

Made with ❤️ by the Gameaday team
"@

Write-Host "📄 Creating README.md..." -ForegroundColor Cyan
Set-Content -Path "$targetRoot\README.md" -Value $readmeContent -Encoding UTF8
Write-Host "   ✅ Created README.md" -ForegroundColor Green

# Create CHANGELOG.md
$changelogContent = @"
# Changelog

All notable changes to IA Helper will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-10-XX (Upcoming)

### Added
- 📥 Full download management with resume capability
- 🔍 Advanced search across 35+ million Internet Archive items
- 📚 Library organization with collections and tags
- ⚡ Smart download queue with priority management
- 🌙 Complete Material Design 3 UI with dark mode
- 🔐 Privacy-focused design (no tracking or ads)
- 📊 Download scheduling and network-aware pausing
- 🎨 Beautiful, accessible interface (WCAG AA+ compliant)
- 📱 Tablet-optimized layouts with master-detail views
- 🔄 Automatic retry with exponential backoff
- 💾 Offline access to downloaded content
- 🏷️ Rich metadata viewer for Internet Archive items

### Technical
- Built with Flutter 3.35.0
- Material Design 3 compliance (~98%)
- SQLite database for local storage
- Product flavors (development, staging, production)
- Comprehensive test coverage
- CI/CD with GitHub Actions

---

**Repository**: https://github.com/gameaday/ia-helper  
**Issues**: https://github.com/gameaday/ia-helper/issues
"@

Write-Host "📄 Creating CHANGELOG.md..." -ForegroundColor Cyan
Set-Content -Path "$targetRoot\CHANGELOG.md" -Value $changelogContent -Encoding UTF8
Write-Host "   ✅ Created CHANGELOG.md" -ForegroundColor Green

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Step 7: Generate Migration Summary" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# Count files
$libFiles = (Get-ChildItem -Path "$targetRoot\lib" -Recurse -File -ErrorAction SilentlyContinue).Count
$testFiles = (Get-ChildItem -Path "$targetRoot\test" -Recurse -File -ErrorAction SilentlyContinue).Count
$androidFiles = (Get-ChildItem -Path "$targetRoot\android" -Recurse -File -ErrorAction SilentlyContinue).Count
$docFiles = (Get-ChildItem -Path "$targetRoot\docs" -Recurse -File -ErrorAction SilentlyContinue).Count

Write-Host ""
Write-Host "📊 Migration Summary:" -ForegroundColor Green
Write-Host "   📁 lib files: $libFiles" -ForegroundColor White
Write-Host "   🧪 test files: $testFiles" -ForegroundColor White
Write-Host "   🤖 android files: $androidFiles" -ForegroundColor White
Write-Host "   📖 doc files: $docFiles" -ForegroundColor White

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "✅ Migration Preparation Complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "📁 Files copied to: $targetRoot" -ForegroundColor Yellow
Write-Host ""
Write-Host "Next Steps:" -ForegroundColor Cyan
Write-Host "1. Review copied files in $targetRoot" -ForegroundColor White
Write-Host "2. Update package name in Flutter app (see migration guide)" -ForegroundColor White
Write-Host "3. Update app strings ('IA Get' → 'IA Helper')" -ForegroundColor White
Write-Host "4. Update CI/CD workflow paths (remove mobile/flutter/)" -ForegroundColor White
Write-Host "5. Initialize git repository in $targetRoot" -ForegroundColor White
Write-Host "6. Create first commit" -ForegroundColor White
Write-Host "7. Push to github.com/gameaday/ia-helper" -ForegroundColor White
Write-Host ""
Write-Host "📖 See docs\FLUTTER_APP_MIGRATION.md for detailed instructions" -ForegroundColor Yellow
Write-Host ""
