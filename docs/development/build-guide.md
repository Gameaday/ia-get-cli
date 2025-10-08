# Build Guide

Complete guide for building ia-get CLI and mobile app locally and in CI/CD.

---

## 🦀 Rust CLI Build

### Prerequisites
- Rust toolchain (1.70+)
- Cargo (comes with Rust)

### Local Build

```bash
# Development build (debug)
cargo build

# Production build (optimized)
cargo build --release

# Run tests
cargo test

# Run linting
cargo clippy

# Format code
cargo fmt
```

### Build Output
- Debug: `target/debug/ia-get` or `ia-get.exe` (Windows)
- Release: `target/release/ia-get` or `ia-get.exe` (Windows)

---

## 📱 Flutter Mobile App Build

### Prerequisites
- Flutter SDK 3.35.0+
- Dart 3.8.0+
- Android Studio (for Android builds)
- Xcode (for iOS builds, macOS only)

### Setup

```bash
cd mobile/flutter

# Install dependencies
flutter pub get

# Verify setup
flutter doctor
```

### Local Build

```bash
# Development build (debug APK)
flutter build apk --debug

# Production build (release APK)
flutter build apk --release

# Build for specific ABI
flutter build apk --target-platform android-arm64

# Install on connected device
flutter install
```

### Run Tests

```bash
# All tests
flutter test

# Specific test file
flutter test test/services/download_service_test.dart

# With coverage
flutter test --coverage
```

### Analyze Code

```bash
# Static analysis
flutter analyze

# Format code
dart format .
```

---

## ⚙️ Android Build Configuration

### Gradle Settings (`android/gradle.properties`)

**Key Optimizations:**

```properties
# CRITICAL: Disable parallel builds for Flutter
org.gradle.parallel=false
# Flutter's build is serial; parallel causes conflicts (3x duplicate tree-shaking)

# Enable build caching for faster incremental builds
org.gradle.caching=true

# JVM memory allocation (4GB for large projects)
org.gradle.jvmargs=-Xmx4G -Xms512m -XX:MaxMetaspaceSize=1024m -XX:+UseG1GC
# Increased from 2GB to 4GB to prevent OOM during APK bundling

# Enable Gradle daemon
org.gradle.daemon=true

# Disable unused Android features
android.defaults.buildfeatures.aidl=false
android.defaults.buildfeatures.renderscript=false
android.defaults.buildfeatures.shaders=false

# Enable non-transitive R class (modern optimization)
android.nonTransitiveRClass=true
```

### Build Performance Tips

**Tree-Shaking:**
- Use `--no-tree-shake-icons` for faster builds (saves 1-2 minutes)
- Trade-off: APK is ~200KB larger
- For CI: Use `--no-tree-shake-icons`
- For production: Enable tree-shaking

**Why parallel=false?**
- Flutter build process is inherently serial
- Parallel Gradle causes duplicate work:
  - Tree-shaking runs 3x times
  - File watchers conflict
  - Dependency installation repeats
- **Result**: 60% slower builds with parallel enabled

**Memory Settings:**
- 4GB heap (`-Xmx4G`) prevents OOM during APK bundling
- 1GB metaspace for large dependency graphs
- G1GC for better garbage collection
- Tested on GitHub Actions (ubuntu-latest, 7GB RAM)

---

## 🔄 CI/CD Configuration

### Rust CI (`.github/workflows/rust-ci.yml`)

**Triggers:**
- Push to `main` branch
- Pull requests
- Git tags (for releases)

**Jobs:**
1. **Test**: Run `cargo test` on Linux
2. **Lint**: Run `cargo clippy` and `cargo fmt --check`
3. **Build**: Cross-compile for 6 platforms
   - Linux: x86_64, aarch64
   - Windows: x86_64
   - macOS: x86_64 (Intel), aarch64 (M1)

**Artifacts:** Binaries uploaded to release (on tag push)

### Flutter CI (`.github/workflows/flutter-ci.yml`)

**Triggers:**
- Push to `main` branch (mobile changes)
- Pull requests affecting `mobile/flutter/**`
- Git tags

**Jobs:**
1. **Analyze**: Run `flutter analyze`
2. **Test**: Run `flutter test` (144 tests, ~10 seconds)
3. **Build Android**: Build release APK
4. **Build Web**: Build web app and deploy to GitHub Pages

**Environment:**
- Runner: ubuntu-latest (7GB RAM)
- Flutter: 3.35.0
- Java: 17 (Zulu distribution)
- Gradle: 8.9.0

**Build Command:**
```yaml
- name: Build APK
  working-directory: mobile/flutter
  run: flutter build apk --release --no-tree-shake-icons --no-pub
  env:
    GRADLE_OPTS: -Xmx4g -Xms512m -XX:MaxMetaspaceSize=1024m -XX:+UseG1GC -Dorg.gradle.daemon=true -Dorg.gradle.parallel=false -Dorg.gradle.caching=true
    JAVA_OPTS: -Xmx4g -Xms512m
```

**Caching Strategy:**
```yaml
# Gradle dependencies cache
~/.gradle/caches
~/.gradle/wrapper
mobile/flutter/android/.gradle

# Flutter build cache
mobile/flutter/build
```

### CI Test Strategy

**Unit Tests (VM):** 144 tests
- Services (download, search, settings)
- Models (progress, metadata)
- Utilities (formatting, validation)
- **Execution:** flutter_test VM (~10 seconds)

**Database Tests:** 39 tests (skipped in CI)
- Files renamed to `.skip` extension
- Require SQLite (sqflite package)
- **Execution:** On device/emulator only

**Why Skip Database Tests?**
- `sqflite` requires platform-specific native code
- CI VM doesn't have Android/iOS runtime
- Tests pass on physical devices
- Alternative: Use in-memory mocked database

---

## 🐛 Common Build Issues

### Gradle OOM Error

**Error:** `Java heap space` during APK build

**Solution:**
1. Increase heap in `gradle.properties`:
   ```properties
   org.gradle.jvmargs=-Xmx4G -Xms512m -XX:MaxMetaspaceSize=1024m
   ```
2. Set env vars in CI:
   ```yaml
   GRADLE_OPTS: -Xmx4g
   JAVA_OPTS: -Xmx4g
   ```

### Duplicate Tree-Shaking

**Error:** "Already watching path" or 3x tree-shaking logs

**Solution:** Disable parallel builds:
```properties
org.gradle.parallel=false
```

### Slow CI Builds (8+ minutes)

**Solutions:**
1. Add Gradle caching (see CI config above)
2. Use `--no-tree-shake-icons` in CI
3. Add Flutter build caching
4. Result: ~3-4 minute builds

### Test Failures in CI

**Error:** Database tests fail in VM

**Solution:** Rename files to `.skip`:
```bash
mv test/services/search_history_service_test.dart \
   test/services/search_history_service_test.dart.skip
```

---

## 📦 Release Builds

### Creating a Release

```bash
# Tag the release
git tag v2.0.0
git push origin v2.0.0

# GitHub Actions automatically:
# 1. Builds CLI for 6 platforms
# 2. Builds Android APK
# 3. Builds web app and deploys to GitHub Pages
# 4. Creates GitHub release with artifacts
# 5. Generates SHA256 checksums
```

### Artifact Naming

**Rust CLI:**
- `ia-get-{version}-x86_64-unknown-linux-gnu` (Linux x64)
- `ia-get-{version}-aarch64-unknown-linux-gnu` (Linux ARM64)
- `ia-get-{version}-x86_64-pc-windows-msvc.exe` (Windows)
- `ia-get-{version}-x86_64-apple-darwin` (macOS Intel)
- `ia-get-{version}-aarch64-apple-darwin` (macOS M1)

**Flutter Mobile:**
- `ia-get-mobile-{version}.apk` (Android)
- `ia-get-web-{version}.zip` (Web bundle)

### Verification

```bash
# Verify checksums
sha256sum -c checksums.txt

# Test binary
./ia-get-{version}-{platform} --version
```

---

## 🔧 Development Workflow

### Typical Development Cycle

```bash
# 1. Make code changes
vim lib/services/download_service.dart

# 2. Run tests
flutter test test/services/download_service_test.dart

# 3. Analyze code
flutter analyze

# 4. Format code
dart format .

# 5. Test on device
flutter run

# 6. Build release
flutter build apk --release
```

### Pre-Commit Checklist

- [ ] Tests pass: `flutter test`
- [ ] No analysis issues: `flutter analyze`
- [ ] Code formatted: `dart format .`
- [ ] Builds successfully: `flutter build apk`
- [ ] Manual testing on device

---

## 📚 Related Documentation

- [Setup Guide](setup-guide.md) - Development environment setup
- [Testing Guide](testing-guide.md) - Comprehensive testing information
- [Architecture](../architecture/mobile-app-architecture.md) - Design patterns

---

**Last Updated:** October 8, 2025  
**Gradle Heap:** 4GB (optimized for CI)  
**Build Time:** ~3-4 minutes (CI with caching)
