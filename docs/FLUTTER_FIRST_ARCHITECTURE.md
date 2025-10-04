# Flutter-First Architecture: Reimagining ia-get for Mobile

## Executive Summary

This document explores alternative architectures for ia-get that prioritize Flutter/Android as a first-party platform rather than treating it as a Rust CLI project with mobile FFI bindings. The goal is to reduce complexity, eliminate race conditions, and provide a clearer single source of truth while maintaining code quality.

## Current Architecture Challenges

### Rust FFI + Flutter Approach (Current)

**Structure:**
```
┌─────────────────────────────────────┐
│     Rust CLI (Primary)              │
│     - Core logic in Rust            │
│     - FFI layer for mobile          │
│     - Complex state management      │
└─────────────────────────────────────┘
              ↓ FFI
┌─────────────────────────────────────┐
│     Flutter (Secondary)             │
│     - Dart UI                       │
│     - FFI bindings                  │
│     - State synchronization         │
└─────────────────────────────────────┘
```

**Pain Points:**
- ❌ **Build Complexity**: Managing two build systems (Cargo + Flutter)
- ❌ **Race Conditions**: State synchronization between Rust and Dart
- ❌ **FFI Maintenance**: 14+ FFI functions requiring manual binding updates
- ❌ **Debugging Difficulty**: Issues span language boundaries
- ❌ **Development Friction**: Changes require updates in both ecosystems
- ❌ **Memory Management**: Manual handling across FFI boundary

## Alternative Architectures

### Option 1: Pure Flutter/Dart (Mobile-First) ✅ **RECOMMENDED**

**Code Reuse:** 0% (rewrite) | **Complexity:** Low | **Maintenance:** Low

**Structure:**
```
┌─────────────────────────────────────┐
│     Flutter/Dart (Primary)          │
│     - Pure Dart implementation      │
│     - Native platform channels      │
│     - Single language, single build │
└─────────────────────────────────────┘
```

**Approach:**
- Rewrite core logic in pure Dart
- Use Dart's excellent async/await for downloads
- Leverage Flutter's cross-platform capabilities
- Keep CLI as separate Rust project if needed

**Pros:**
- ✅ No FFI complexity or race conditions
- ✅ Single build system (Flutter)
- ✅ Easier debugging and testing
- ✅ Better IDE support and tooling
- ✅ Native Flutter state management
- ✅ Hot reload for rapid development
- ✅ Excellent async/await primitives in Dart
- ✅ Rich ecosystem of Dart packages

**Cons:**
- ⚠️ Complete rewrite of core logic
- ⚠️ 0% code sharing with Rust CLI
- ⚠️ Dart may be slower than Rust for heavy processing
- ⚠️ Need to maintain two separate codebases

**Implementation Path:**
1. Create pure Dart HTTP client for Internet Archive API
2. Implement download manager with Dart's Isolates
3. Add file filtering and compression in Dart
4. Build Flutter UI on top of Dart services
5. Keep Rust CLI as separate project for desktop users

**Key Dart Packages:**
```yaml
dependencies:
  dio: ^5.9.0              # Advanced HTTP client
  path_provider: ^2.1.5    # File system access
  archive: ^3.6.0          # Compression support
  flutter_isolate: ^2.0.4  # Background processing
  hive: ^2.2.3            # Local storage
```

### Option 2: HTTP Bridge Architecture 🔄

**Code Reuse:** 90% | **Complexity:** Medium | **Maintenance:** Medium

**Structure:**
```
┌─────────────────────────────────────┐
│     Flutter App                     │
│     - Pure Dart UI                  │
│     - HTTP client                   │
└─────────────────────────────────────┘
              ↓ HTTP/REST
┌─────────────────────────────────────┐
│     Rust HTTP Service               │
│     - Local HTTP server             │
│     - Core download logic           │
│     - RESTful API                   │
└─────────────────────────────────────┘
```

**Approach:**
- Run Rust as embedded HTTP service
- Flutter communicates via localhost REST API
- Clean separation of concerns
- No FFI complexity

**Pros:**
- ✅ Clean separation (HTTP boundary)
- ✅ 90% code reuse from Rust
- ✅ No FFI bindings needed
- ✅ Easy debugging (HTTP traffic inspection)
- ✅ Language agnostic interface
- ✅ Can test Rust service independently

**Cons:**
- ⚠️ HTTP overhead for local communication
- ⚠️ Process management complexity
- ⚠️ Additional dependency (HTTP server)
- ⚠️ Battery impact from running separate service

**Implementation Path:**
1. Create minimal Actix/Axum HTTP server in Rust
2. Expose download operations as REST endpoints
3. Flutter app connects to localhost:PORT
4. Use WebSocket for real-time progress updates
5. Package Rust binary with Flutter app

### Option 3: Platform Channels (Flutter Standard) 🎯

**Code Reuse:** 50% | **Complexity:** Medium-Low | **Maintenance:** Low

**Structure:**
```
┌─────────────────────────────────────┐
│     Flutter App (Dart)              │
│     - Core logic in Dart            │
│     - Platform channels for native  │
└─────────────────────────────────────┘
              ↓ Method Channels
┌─────────────────────────────────────┐
│     Kotlin/Java (Android)           │
│     - Native Android features only  │
│     - Storage, notifications, etc.  │
└─────────────────────────────────────┘
```

**Approach:**
- Implement core logic in Dart
- Use platform channels ONLY for platform-specific features
- No heavy computation in native layer
- Standard Flutter pattern

**Pros:**
- ✅ Standard Flutter approach
- ✅ Minimal native code needed
- ✅ Easy to maintain
- ✅ Good Flutter ecosystem support
- ✅ Native features when needed

**Cons:**
- ⚠️ Requires Dart rewrite of core logic
- ⚠️ Less code sharing with CLI

### Option 4: Shared Library with Simpler Interface 🔧

**Code Reuse:** 80% | **Complexity:** Medium | **Maintenance:** Medium

**Structure:**
```
┌─────────────────────────────────────┐
│     Rust Core Library               │
│     - Simplified, focused API       │
│     - 3-5 key functions only        │
└─────────────────────────────────────┘
              ↓ Minimal FFI
┌─────────────────────────────────────┐
│     Flutter App                     │
│     - Business logic in Dart        │
│     - Calls Rust for heavy lifting  │
└─────────────────────────────────────┘
```

**Approach:**
- Drastically simplify FFI interface (reduce from 14 to 3-5 functions)
- Move state management entirely to Dart side
- Rust provides pure computation functions only
- No sessions or state in Rust layer

**Pros:**
- ✅ High code reuse (80%)
- ✅ Much simpler than current FFI
- ✅ State management in one place (Dart)
- ✅ Leverage Rust for performance-critical parts

**Cons:**
- ⚠️ Still requires FFI (but much simpler)
- ⚠️ Need to redesign current architecture
- ⚠️ Two build systems remain

**Simplified FFI Functions:**
```rust
// Only 3 essential functions
pub extern "C" fn ia_fetch_metadata(url: *const c_char) -> *mut c_char;
pub extern "C" fn ia_download_file(url: *const c_char, path: *const c_char) -> i32;
pub extern "C" fn ia_compress_file(path: *const c_char) -> i32;
```

## Comparison Matrix

| Approach | Code Reuse | Complexity | Maintenance | Development Speed | Performance |
|----------|-----------|------------|-------------|-------------------|-------------|
| **Pure Flutter/Dart** | 0% | ★★★★★ Low | ★★★★★ Low | ★★★★★ Fast | ★★★★☆ Good |
| **HTTP Bridge** | 90% | ★★★☆☆ Medium | ★★★☆☆ Medium | ★★★☆☆ Medium | ★★★★☆ Good |
| **Platform Channels** | 50% | ★★★★☆ Low-Med | ★★★★★ Low | ★★★★☆ Fast | ★★★★☆ Good |
| **Simplified FFI** | 80% | ★★★☆☆ Medium | ★★★☆☆ Medium | ★★★☆☆ Medium | ★★★★★ Excellent |
| **Current FFI** | 85% | ★☆☆☆☆ High | ★☆☆☆☆ High | ★★☆☆☆ Slow | ★★★★★ Excellent |

## Recommended Approach: Pure Flutter/Dart

### Rationale

The **Pure Flutter/Dart** approach is recommended because:

1. **Eliminates Root Cause**: Removes all FFI complexity, race conditions, and build system integration issues
2. **Single Source of Truth**: All application logic in one language and ecosystem
3. **Faster Development**: Hot reload, better debugging, unified tooling
4. **Better Long-term Maintainability**: Standard Flutter patterns, easier onboarding
5. **Mobile-First Design**: Optimized for the mobile use case from the ground up

### Addressing Concerns

**Q: What about performance?**
- A: Dart's performance is sufficient for HTTP I/O operations
- Dart's async/await is excellent for concurrent downloads
- Most time is spent in I/O, not computation
- Can optimize critical paths if needed

**Q: What about the CLI?**
- A: Keep Rust CLI as a separate project
- Different use cases: CLI for power users, Flutter for mobile
- No need for 100% feature parity
- Focus each tool on its strengths

**Q: What about code duplication?**
- A: Accept some duplication for architectural clarity
- Better than fighting impedance mismatch between ecosystems
- Can share high-level algorithms/patterns in documentation

### Migration Strategy

#### Phase 1: Prototype (2-3 weeks)
- [ ] Create new Flutter project with pure Dart implementation
- [ ] Implement basic metadata fetching
- [ ] Build simple download manager
- [ ] Validate approach with real-world testing

#### Phase 2: Core Features (4-6 weeks)
- [ ] Implement full download engine with resume capability
- [ ] Add file filtering and compression
- [ ] Build comprehensive UI
- [ ] Add local storage and state management

#### Phase 3: Polish (2-3 weeks)
- [ ] Add background download service
- [ ] Implement notifications
- [ ] Add deep linking
- [ ] Comprehensive testing

#### Phase 4: Deprecation (Gradual)
- [ ] Mark FFI implementation as deprecated
- [ ] Maintain CLI as separate project
- [ ] Focus mobile development on pure Flutter

## Implementation Details

### Core Dart Services

```dart
// lib/services/archive_api_service.dart
class ArchiveApiService {
  final Dio _dio;
  
  Future<ArchiveMetadata> fetchMetadata(String identifier) async {
    // Pure Dart HTTP request
    final response = await _dio.get(
      'https://archive.org/metadata/$identifier'
    );
    return ArchiveMetadata.fromJson(response.data);
  }
}

// lib/services/download_service.dart
class DownloadService {
  final _isolates = <DownloadIsolate>[];
  
  Future<void> downloadFile(FileInfo file, String path) async {
    // Use Dart Isolates for concurrent downloads
    final isolate = await DownloadIsolate.spawn(file, path);
    _isolates.add(isolate);
    
    // Pure Dart download with progress tracking
    await isolate.download(onProgress: (progress) {
      // Update UI via streams
      _progressController.add(progress);
    });
  }
}

// lib/services/compression_service.dart
class CompressionService {
  Future<void> decompressFile(String path) async {
    // Use Dart 'archive' package
    final bytes = await File(path).readAsBytes();
    final archive = ZipDecoder().decodeBytes(bytes);
    // Extract files...
  }
}
```

### Project Structure

```
mobile/flutter/
├── lib/
│   ├── main.dart
│   ├── models/              # Data models
│   │   ├── archive_metadata.dart
│   │   ├── file_info.dart
│   │   └── download_state.dart
│   ├── services/            # Business logic (pure Dart)
│   │   ├── archive_api_service.dart
│   │   ├── download_service.dart
│   │   ├── compression_service.dart
│   │   ├── storage_service.dart
│   │   └── notification_service.dart
│   ├── providers/           # State management
│   │   ├── archive_provider.dart
│   │   └── download_provider.dart
│   ├── screens/             # UI screens
│   │   ├── home_screen.dart
│   │   ├── archive_detail_screen.dart
│   │   └── downloads_screen.dart
│   └── widgets/             # Reusable widgets
│       ├── file_list_widget.dart
│       └── download_progress_widget.dart
├── test/                    # Comprehensive tests
└── pubspec.yaml
```

## Alternative Consideration: Keep Both Separate

### Two Independent Projects

**Option:** Maintain CLI and Mobile as completely separate projects

**Structure:**
```
ia-get-cli/           # Rust CLI (existing)
├── src/
├── Cargo.toml
└── README.md

ia-get-mobile/        # Flutter app (new repo)
├── lib/
├── pubspec.yaml
└── README.md
```

**Pros:**
- ✅ Complete independence
- ✅ No build system conflicts
- ✅ Optimized for each use case
- ✅ Easier to reason about

**Cons:**
- ⚠️ Duplicate functionality
- ⚠️ Separate maintenance
- ⚠️ Different features may diverge

## Conclusion

The **Pure Flutter/Dart approach** is the best solution for making Android a first-party platform:

### Benefits:
1. ✅ **Eliminates complexity** - No FFI, no race conditions
2. ✅ **Single source of truth** - All mobile logic in Dart
3. ✅ **Faster development** - Standard Flutter patterns
4. ✅ **Better debugging** - Single language/toolchain
5. ✅ **Easier maintenance** - No impedance mismatch

### Trade-offs:
- ⚠️ Requires rewriting core logic in Dart (one-time cost)
- ⚠️ CLI and mobile become separate projects
- ⚠️ No code sharing between platforms

### Why This Is Worth It:
- The current FFI approach has reached its complexity limit
- Fighting impedance mismatch between Rust and Flutter is not sustainable
- Better to have two simple, maintainable codebases than one complex, fragile integration
- Mobile-first design will result in better UX
- Development velocity will dramatically improve

## Next Steps

### Immediate Actions:
1. **Approve Direction**: Decide on pure Flutter/Dart approach
2. **Create Prototype**: 2-3 week spike to validate approach
3. **Evaluate Results**: Compare complexity vs current implementation
4. **Plan Migration**: If successful, plan full implementation

### Long-term Vision:
- **Mobile App**: Pure Flutter/Dart, optimized for mobile UX
- **CLI Tool**: Keep Rust implementation, optimized for power users
- **Web App**: Could add Flutter Web in future
- **Desktop GUI**: Could use Flutter Desktop instead of Rust GUI

This separation allows each platform to excel in its domain rather than trying to share code at all costs.
