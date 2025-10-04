# Simplified FFI Implementation Plan

## Overview
Implementing the Simplified FFI (Hybrid) approach to keep Rust as computational core while moving all state management to Dart.

## Current Status
- [x] Documentation cleanup complete
- [ ] Phase 1: Redesign Rust core (2-3 weeks)
- [ ] Phase 2: Simplify FFI layer (1-2 weeks)
- [ ] Phase 3: Update Flutter integration (2-3 weeks)
- [ ] Phase 4: Deprecate old FFI (1 week)

## Phase 1: Redesign Rust Core

### Goals
- Separate core computation logic from state management
- Create stateless, pure functions
- Add synchronous wrappers for FFI use
- Maintain backward compatibility with CLI

### Tasks

#### 1.1 Create Stateless Core Module Structure
```
src/
├── core/
│   ├── stateless/          # NEW: Pure computation functions
│   │   ├── mod.rs
│   │   ├── metadata.rs     # Stateless metadata fetching
│   │   ├── download.rs     # Stateless download operations
│   │   ├── compression.rs  # Stateless compression/decompression
│   │   └── validation.rs   # Stateless checksum validation
│   ├── archive/            # Existing
│   ├── download/           # Existing
│   └── session/            # Existing (will be CLI-only)
```

#### 1.2 Implement Stateless Metadata Module
- [x] Plan created
- [ ] Create `src/core/stateless/mod.rs`
- [ ] Create `src/core/stateless/metadata.rs`
  - [ ] `fetch_metadata_sync()` - Synchronous metadata fetching
  - [ ] `fetch_metadata_async()` - Async version for CLI
  - [ ] Return JSON strings for easy FFI transfer

#### 1.3 Implement Stateless Download Module
- [ ] Create `src/core/stateless/download.rs`
  - [ ] `download_file_sync()` - Blocking download with progress callback
  - [ ] `download_file_async()` - Async version for CLI
  - [ ] Support resume from offset
  - [ ] Simple progress callback interface

#### 1.4 Implement Stateless Compression Module
- [ ] Create `src/core/stateless/compression.rs`
  - [ ] `decompress_archive()` - Extract archives
  - [ ] `compress_file()` - Create archives
  - [ ] Auto-detect format from extension
  - [ ] Return list of extracted files as JSON

#### 1.5 Implement Stateless Validation Module
- [ ] Create `src/core/stateless/validation.rs`
  - [ ] `validate_checksum()` - Verify file integrity
  - [ ] Support MD5, SHA1, SHA256
  - [ ] Simple bool return for FFI

### Testing Strategy
- Unit tests for each stateless function
- Integration tests comparing with existing functionality
- Ensure CLI continues to work during transition

## Phase 2: Simplify FFI Layer

### Goals
- Reduce from 14 to 5 core FFI functions
- Remove all state management from FFI
- Simple request-response pattern

### New FFI Functions (Target: 5)

1. **`ia_get_fetch_metadata(identifier: *const c_char) -> *mut c_char`**
   - Fetches metadata, returns JSON string
   - Caller must free returned string

2. **`ia_get_download_file(url, path, callback, user_data) -> IaGetResult`**
   - Downloads file with progress callback
   - Blocking operation (caller uses isolates)

3. **`ia_get_decompress_file(archive_path, output_dir) -> *mut c_char`**
   - Decompresses archive
   - Returns JSON array of extracted files

4. **`ia_get_validate_checksum(file_path, expected_hash, hash_type) -> c_int`**
   - Validates file checksum
   - Returns 1 (match), 0 (no match), -1 (error)

5. **`ia_get_last_error() -> *const c_char`**
   - Returns last error message
   - Thread-local storage

6. **`ia_get_free_string(s: *mut c_char)`**
   - Frees strings returned by library

### Tasks
- [ ] Create new FFI module: `src/interface/ffi_simple.rs`
- [ ] Implement 5 core FFI functions
- [ ] Add error handling with thread-local storage
- [ ] Generate C header with cbindgen
- [ ] Update build configuration

## Phase 3: Update Flutter Integration

### Goals
- Move all state management to Dart
- Use Isolates for blocking Rust calls
- Simplified FFI bindings

### Tasks
- [ ] Update Flutter FFI bindings to new interface
- [ ] Implement state management in Dart
  - [ ] DownloadProvider with local state
  - [ ] Progress tracking in Dart
  - [ ] Session management in Dart
- [ ] Use Isolates for blocking operations
- [ ] Test thoroughly on Android

## Phase 4: Deprecate Old FFI

### Tasks
- [ ] Mark old FFI functions as deprecated
- [ ] Add deprecation warnings
- [ ] Update documentation
- [ ] Create migration guide for any external users
- [ ] Remove old FFI in next major version

## Success Criteria

### Phase 1
- ✅ All stateless functions implemented
- ✅ Unit tests passing
- ✅ CLI still works with existing code
- ✅ No breaking changes to external API

### Phase 2
- ✅ FFI reduced to 5 functions
- ✅ No state in FFI layer
- ✅ C header generated
- ✅ Example test program works

### Phase 3
- ✅ Flutter app uses new FFI
- ✅ All state in Dart
- ✅ No race conditions
- ✅ Android app works correctly

### Phase 4
- ✅ Old FFI marked deprecated
- ✅ Documentation updated
- ✅ Migration guide available

## Timeline

- **Phase 1**: 2-3 weeks (Current)
- **Phase 2**: 1-2 weeks
- **Phase 3**: 2-3 weeks
- **Phase 4**: 1 week

**Total**: 6-9 weeks (1.5-2 months)

## Current Focus: Phase 1.2 - Stateless Metadata Module

Starting with the metadata module as it's the foundation for everything else.
