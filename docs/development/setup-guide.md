# Setup Guide

Get started with ia-get development (Rust CLI + Flutter mobile app).

---

## 🦀 Rust CLI Development Setup

### Prerequisites

**Required:**
- [Rust](https://rustup.rs/) 1.70+ (install via rustup)
- Git

**Optional:**
- [cargo-watch](https://github.com/watchexec/cargo-watch) for auto-recompilation

### Installation

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/Gameaday/ia-get-cli.git
cd ia-get-cli

# Build and run
cargo run -- --help

# Install locally
cargo install --path .
```

### Verify Setup

```bash
# Check Rust version
rustc --version  # Should be 1.70+

# Run tests
cargo test

# Run linting
cargo clippy
```

---

## 📱 Flutter Mobile App Development Setup

### Prerequisites

**Required:**
- [Flutter SDK](https://flutter.dev/docs/get-started/install) 3.35.0+
- [Dart SDK](https://dart.dev/get-dart) 3.8.0+ (comes with Flutter)
- Git

**For Android:**
- [Android Studio](https://developer.android.com/studio) or Android SDK
- Java JDK 17+ (Zulu recommended)
- Android SDK Platform-Tools
- Android SDK Build-Tools 34.0.0+

**For iOS (macOS only):**
- [Xcode](https://developer.apple.com/xcode/) 15+
- CocoaPods

### Installation

#### 1. Install Flutter

**macOS/Linux:**
```bash
# Download Flutter SDK
git clone https://github.com/flutter/flutter.git -b stable --depth 1
export PATH="$PATH:`pwd`/flutter/bin"

# Add to shell profile
echo 'export PATH="$PATH:$HOME/flutter/bin"' >> ~/.zshrc  # or ~/.bashrc
```

**Windows:**
1. Download [Flutter SDK](https://docs.flutter.dev/get-started/install/windows)
2. Extract to `C:\flutter`
3. Add `C:\flutter\bin` to PATH

#### 2. Verify Flutter Installation

```bash
# Check Flutter installation
flutter doctor

# Expected output:
[✓] Flutter (Channel stable, 3.35.0)
[✓] Android toolchain
[✓] Android Studio
[!] Xcode (optional, macOS only)
```

#### 3. Install Android Studio

1. Download [Android Studio](https://developer.android.com/studio)
2. Install with Android SDK
3. Open Android Studio → Settings → Plugins
4. Install "Flutter" and "Dart" plugins

#### 4. Setup Android SDK

```bash
# Install SDK components
sdkmanager "platform-tools" "platforms;android-34" "build-tools;34.0.0"

# Accept licenses
flutter doctor --android-licenses
```

#### 5. Clone and Setup Project

```bash
# Clone repository
git clone https://github.com/Gameaday/ia-get-cli.git
cd ia-get-cli/mobile/flutter

# Install dependencies
flutter pub get

# Verify setup
flutter doctor
```

### Device Setup

#### Android Physical Device

1. Enable Developer Options:
   - Settings → About Phone
   - Tap "Build Number" 7 times

2. Enable USB Debugging:
   - Settings → Developer Options
   - Enable "USB Debugging"

3. Connect via USB and verify:
```bash
adb devices
flutter devices
```

#### Android Emulator

```bash
# Create emulator
flutter emulators --create

# List emulators
flutter emulators

# Launch emulator
flutter emulators --launch <emulator_id>
```

#### iOS Simulator (macOS only)

```bash
# Open simulator
open -a Simulator

# Or launch via Flutter
flutter emulators --launch apple_ios_simulator
```

---

## 🔧 IDE Setup

### Visual Studio Code

**Required Extensions:**
- [Flutter](https://marketplace.visualstudio.com/items?itemName=Dart-Code.flutter)
- [Dart](https://marketplace.visualstudio.com/items?itemName=Dart-Code.dart-code)

**Recommended Extensions:**
- [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens)
- [GitLens](https://marketplace.visualstudio.com/items?itemName=eamodio.gitlens)

**Configuration:**

Create `.vscode/settings.json`:
```json
{
  "dart.flutterSdkPath": "/path/to/flutter",
  "dart.lineLength": 100,
  "editor.formatOnSave": true,
  "editor.codeActionsOnSave": {
    "source.fixAll": true
  },
  "[dart]": {
    "editor.rulers": [100],
    "editor.tabSize": 2
  }
}
```

**Launch Configuration** (`.vscode/launch.json`):
```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Flutter (mobile/flutter)",
      "request": "launch",
      "type": "dart",
      "program": "mobile/flutter/lib/main.dart"
    }
  ]
}
```

### Android Studio

1. Open `ia-get-cli/mobile/flutter` as Flutter project
2. Settings → Languages & Frameworks → Flutter
3. Set Flutter SDK path
4. Enable Dart support

**Recommended Plugins:**
- Flutter
- Dart
- Rainbow Brackets
- Key Promoter X

---

## 🏃 Running the App

### Flutter Mobile

```bash
cd mobile/flutter

# Run in debug mode (hot reload enabled)
flutter run

# Run in release mode
flutter run --release

# Run on specific device
flutter run -d <device-id>

# Run with verbose output
flutter run -v
```

### Rust CLI

```bash
# Run with arguments
cargo run -- search "public domain"

# Run in release mode (faster)
cargo run --release -- download <identifier>

# Install and run
cargo install --path .
ia-get --help
```

---

## 🧪 Verify Setup

### Flutter

```bash
cd mobile/flutter

# Run tests
flutter test

# Analyze code
flutter analyze

# Build APK
flutter build apk --debug
```

**Expected:**
- ✅ 144/144 tests passing
- ✅ No analysis issues
- ✅ APK builds successfully

### Rust

```bash
# Run tests
cargo test

# Run benchmarks
cargo bench

# Check linting
cargo clippy
```

**Expected:**
- ✅ All tests passing
- ✅ No clippy warnings

---

## 🐛 Troubleshooting

### Flutter Doctor Issues

**Issue:** "Android licenses not accepted"
```bash
flutter doctor --android-licenses
```

**Issue:** "cmdline-tools not installed"
```bash
sdkmanager "cmdline-tools;latest"
```

**Issue:** "Flutter SDK not found"
```bash
# Verify PATH
echo $PATH | grep flutter

# Reinstall Flutter
flutter upgrade
```

### Build Issues

**Issue:** Gradle OOM error
```bash
# Increase heap in gradle.properties
org.gradle.jvmargs=-Xmx4G
```

**Issue:** "Could not find package"
```bash
flutter pub get
flutter clean
flutter pub get
```

**Issue:** Device not detected
```bash
# Android
adb kill-server
adb start-server
flutter devices

# iOS
xcrun simctl list
```

### Common Errors

**Error:** "Waiting for another flutter command to release the startup lock"
```bash
rm -rf ~/.flutter-repo-cache
flutter clean
```

**Error:** "Gradle daemon failed to start"
```bash
cd mobile/flutter/android
./gradlew --stop
./gradlew clean
```

**Error:** "Version conflict" in dependencies
```bash
flutter pub upgrade
flutter pub outdated
```

---

## 📦 Dependencies

### Rust CLI

**Core:**
- `clap` - CLI argument parsing
- `reqwest` - HTTP client
- `tokio` - Async runtime
- `serde` - Serialization

**See:** `Cargo.toml` for full list

### Flutter Mobile

**Core:**
- `provider` - State management
- `dio` - HTTP client
- `sqflite` - SQLite database
- `path_provider` - File paths

**Media:**
- `pdfx` - PDF rendering
- `just_audio` - Audio playback
- `video_player` + `chewie` - Video playback
- `archive` - Archive extraction

**See:** `mobile/flutter/pubspec.yaml` for full list

---

## 🔄 Keeping Updated

### Update Flutter

```bash
flutter upgrade
flutter pub upgrade
```

### Update Rust

```bash
rustup update
cargo update
```

### Update Dependencies

```bash
# Flutter
cd mobile/flutter
flutter pub upgrade

# Rust
cargo update
```

---

## 📚 Next Steps

- Read [Build Guide](build-guide.md) for compilation details
- Read [Testing Guide](testing-guide.md) for running tests
- Read [Architecture](../architecture/mobile-app-architecture.md) for design patterns
- Check [Roadmap](../mobile/roadmap.md) for planned features

---

## 🤝 Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for:
- Code style guidelines
- Commit message format
- Pull request process
- Development workflow

---

**Last Updated:** October 8, 2025  
**Flutter Version:** 3.35.0  
**Dart Version:** 3.8.0  
**Rust Version:** 1.70+
