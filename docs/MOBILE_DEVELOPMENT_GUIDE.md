# Mobile Development Guide for ia-get

This guide provides detailed instructions for setting up and developing the ia-get mobile application using Flutter and Rust FFI.

## Prerequisites

### Development Environment

1. **Rust Toolchain**
   ```bash
   # Install Rust via rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Add Android targets
   rustup target add aarch64-linux-android
   rustup target add armv7-linux-androideabi
   rustup target add x86_64-linux-android
   rustup target add i686-linux-android
   ```

2. **Android Development**
   ```bash
   # Install Android Studio and SDK
   # Set ANDROID_HOME environment variable
   export ANDROID_HOME=$HOME/Android/Sdk
   export PATH=$PATH:$ANDROID_HOME/tools/bin:$ANDROID_HOME/platform-tools
   
   # Install Android NDK
   # NDK version 23c or later recommended
   export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/23.2.8568313
   ```

3. **Flutter Development**
   ```bash
   # Install Flutter
   git clone https://github.com/flutter/flutter.git -b stable
   export PATH=$PATH:`pwd`/flutter/bin
   
   # Verify installation
   flutter doctor
   ```

4. **Cargo NDK**
   ```bash
   # Install cargo-ndk for cross-compilation
   cargo install cargo-ndk
   ```

## Project Structure

```
ia-get-mobile/
├── rust/                     # Rust FFI library
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs           # FFI exports
│   │   └── mobile.rs        # Mobile-specific adaptations
│   └── build.rs             # Build configuration
├── flutter/                 # Flutter application  
│   ├── pubspec.yaml
│   ├── android/
│   ├── lib/
│   │   ├── main.dart
│   │   ├── models/          # Data models
│   │   ├── services/        # FFI service layer
│   │   ├── widgets/         # UI components
│   │   └── screens/         # App screens
│   └── test/
└── scripts/                 # Build and deployment scripts
    ├── build-rust.sh
    ├── build-android.sh
    └── deploy.sh
```

## Step-by-Step Setup

### 1. Create Rust FFI Library

```bash
# Create new Rust library for mobile
mkdir ia-get-mobile && cd ia-get-mobile
cargo new --lib rust
cd rust
```

**rust/Cargo.toml:**
```toml
[package]
name = "ia_get_mobile"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "staticlib"]

[dependencies]
ia-get = { path = "../../", features = ["ffi"] }
tokio = { version = "1.0", features = ["rt", "rt-multi-thread"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[target.'cfg(target_os = "android")'.dependencies]
jni = "0.20"
android_logger = "0.11"
```

**rust/src/lib.rs:**
```rust
//! Mobile FFI library for ia-get
//! Exposes core functionality for Flutter integration

// Re-export FFI interface from main crate
pub use ia_get::ffi::*;

// Android-specific initialization
#[cfg(target_os = "android")]
pub mod android {
    use android_logger::{Config, FilterBuilder};
    use jni::JNIEnv;
    use jni::objects::JClass;
    use log::LevelFilter;

    #[no_mangle]
    pub extern "C" fn Java_com_example_iaget_NativeLib_initializeLogging(
        _env: JNIEnv,
        _class: JClass,
    ) {
        android_logger::init_once(
            Config::default()
                .with_min_level(log::Level::Debug)
                .with_tag("ia-get")
                .with_filter(FilterBuilder::new().parse("debug,ia_get=debug").build()),
        );
    }
}
```

### 2. Create Flutter Application

```bash
# Create Flutter app
cd ../
flutter create flutter
cd flutter

# Add FFI dependency
flutter pub add ffi
flutter pub add path_provider
flutter pub add permission_handler
```

**flutter/pubspec.yaml additions:**
```yaml
dependencies:
  flutter:
    sdk: flutter
  ffi: ^2.0.1
  path_provider: ^2.0.11
  permission_handler: ^10.4.3
  http: ^0.13.5
  json_annotation: ^4.8.1

dev_dependencies:
  build_runner: ^2.4.6
  json_serializable: ^6.7.1
```

### 3. FFI Service Layer

**flutter/lib/services/ia_get_service.dart:**
```dart
import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';
import 'package:path_provider/path_provider.dart';

class IaGetService {
  static late final DynamicLibrary _dylib;
  static bool _initialized = false;

  // FFI function signatures
  late final int Function() _init;
  late final int Function(Pointer<Utf8>, Pointer<NativeFunction>, Pointer<NativeFunction>, Pointer<Void>) _fetchMetadata;
  late final Pointer<Utf8> Function(Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>) _filterFiles;
  late final void Function(Pointer<Utf8>) _freeString;

  static Future<IaGetService> create() async {
    if (!_initialized) {
      await _initializeDylib();
      _initialized = true;
    }
    return IaGetService._();
  }

  IaGetService._() {
    // Bind FFI functions
    _init = _dylib.lookup<NativeFunction<Int32 Function()>>('ia_get_init').asFunction();
    _fetchMetadata = _dylib.lookup<NativeFunction<Int32 Function(Pointer<Utf8>, Pointer<NativeFunction>, Pointer<NativeFunction>, Pointer<Void>)>>('ia_get_fetch_metadata').asFunction();
    _filterFiles = _dylib.lookup<NativeFunction<Pointer<Utf8> Function(Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>, Pointer<Utf8>)>>('ia_get_filter_files').asFunction();
    _freeString = _dylib.lookup<NativeFunction<Void Function(Pointer<Utf8>)>>('ia_get_free_string').asFunction();

    // Initialize native library
    _init();
  }

  static Future<void> _initializeDylib() async {
    if (Platform.isAndroid) {
      _dylib = DynamicLibrary.open('libia_get_mobile.so');
    } else {
      throw UnsupportedError('Platform not supported yet');
    }
  }

  // High-level methods using FFI
  Future<ArchiveMetadata?> fetchArchiveMetadata(String identifier) async {
    // Implementation using _fetchMetadata FFI function
    // Returns parsed metadata
  }

  List<ArchiveFile> filterFiles(ArchiveMetadata metadata, FileFilter filter) {
    // Implementation using _filterFiles FFI function
    // Returns filtered file list
  }
}
```

### 4. Build Scripts

**scripts/build-rust.sh:**
```bash
#!/bin/bash
set -e

echo "Building Rust library for Android..."

cd rust

# Build for all Android architectures
for target in aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android; do
    echo "Building for $target..."
    cargo ndk -t $target build --release
done

echo "Copying libraries to Flutter project..."

# Copy libraries to Flutter android/app/src/main/jniLibs/
FLUTTER_JNI_LIBS="../flutter/android/app/src/main/jniLibs"
mkdir -p "$FLUTTER_JNI_LIBS"/{arm64-v8a,armeabi-v7a,x86_64,x86}

cp target/aarch64-linux-android/release/libia_get_mobile.so "$FLUTTER_JNI_LIBS/arm64-v8a/"
cp target/armv7-linux-androideabi/release/libia_get_mobile.so "$FLUTTER_JNI_LIBS/armeabi-v7a/"
cp target/x86_64-linux-android/release/libia_get_mobile.so "$FLUTTER_JNI_LIBS/x86_64/"
cp target/i686-linux-android/release/libia_get_mobile.so "$FLUTTER_JNI_LIBS/x86/"

echo "Rust library build complete!"
```

**scripts/build-android.sh:**
```bash
#!/bin/bash
set -e

echo "Building complete Android application..."

# Build Rust library first
./scripts/build-rust.sh

# Build Flutter app
cd flutter
flutter clean
flutter pub get
flutter build apk --release

echo "Android build complete!"
echo "APK location: flutter/build/app/outputs/flutter-apk/app-release.apk"
```

### 5. CI/CD Integration

**Add to .github/workflows/ci.yml:**
```yaml
  android-build:
    name: Android Build
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    
    steps:
    - name: Checkout repository
      uses: actions/checkout@v5

    - name: Setup Java
      uses: actions/setup-java@v3
      with:
        distribution: 'zulu'
        java-version: '17'

    - name: Setup Android SDK
      uses: android-actions/setup-android@v2

    - name: Setup Flutter
      uses: subosito/flutter-action@v2
      with:
        flutter-version: '3.16.0'

    - name: Setup Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        targets: aarch64-linux-android,armv7-linux-androideabi,x86_64-linux-android,i686-linux-android

    - name: Install cargo-ndk
      run: cargo install cargo-ndk

    - name: Cache dependencies
      uses: actions/cache@v4
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target/
        key: ${{ runner.os }}-android-${{ hashFiles('**/Cargo.lock') }}

    - name: Build Android APK
      run: |
        chmod +x scripts/build-android.sh
        ./scripts/build-android.sh

    - name: Upload APK artifact
      uses: actions/upload-artifact@v4
      with:
        name: ia-get-android-apk
        path: flutter/build/app/outputs/flutter-apk/app-release.apk
        retention-days: 30
```

## Development Workflow

### 1. Local Development

```bash
# Terminal 1: Start Android emulator
emulator -avd Pixel_7_API_33

# Terminal 2: Build and run
./scripts/build-rust.sh
cd flutter
flutter run
```

### 2. Hot Reload Support

For UI changes:
```bash
# In flutter directory
flutter run
# Press 'r' for hot reload, 'R' for hot restart
```

For Rust changes:
```bash
# Rebuild Rust library
./scripts/build-rust.sh
# Hot restart Flutter app (press 'R')
```

### 3. Testing

```bash
# Run Rust tests
cd rust && cargo test

# Run Flutter tests  
cd flutter && flutter test

# Integration tests on device
flutter test integration_test/
```

### 4. Debugging

**Android logs:**
```bash
# View native logs
adb logcat -s "ia-get"

# View Flutter logs
flutter logs
```

**Debugging FFI:**
```rust
// Add to Rust code
#[cfg(debug_assertions)]
eprintln!("Debug: {}", value);
```

## Best Practices

### Memory Management
- Always free strings returned by FFI functions
- Use `malloc.free()` for allocated pointers
- Implement proper cleanup in Flutter widgets

### Error Handling
- Check for null pointers before dereferencing
- Implement proper error codes and messages
- Use Result types in Rust and handle them in Flutter

### Performance
- Minimize FFI calls in hot code paths
- Batch operations when possible
- Use async operations for I/O intensive tasks

### Security
- Validate all input at FFI boundaries
- Handle permissions properly on Android
- Use secure storage for sensitive data

## Troubleshooting

### Common Issues

1. **Library not found:**
   ```
   Solution: Ensure libraries are in correct jniLibs directories
   Check: android/app/src/main/jniLibs/{arch}/lib*.so
   ```

2. **FFI function not found:**
   ```
   Solution: Verify function names match exactly
   Check: Use nm -D library.so to list exported symbols
   ```

3. **Build failures:**
   ```
   Solution: Clean and rebuild everything
   Commands: cargo clean && flutter clean
   ```

4. **Permission errors:**
   ```
   Solution: Add required permissions to android/app/src/main/AndroidManifest.xml
   ```

### Performance Optimization

1. **Reduce FFI overhead:**
   - Batch multiple operations
   - Use shared memory for large data transfers
   - Implement object handles instead of serializing data

2. **Memory usage:**
   - Profile memory usage with Android Studio
   - Implement lazy loading for large archives
   - Use pagination for file lists

3. **Battery optimization:**
   - Implement proper background task handling
   - Use WorkManager for downloads
   - Respect Android battery optimization settings

This guide provides a complete foundation for developing the ia-get mobile application. The modular architecture ensures maintainability while maximizing code reuse from the existing Rust codebase.