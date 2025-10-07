# ia-get Roadmap Analysis: From Download Helper to Full Archive Manager

**Document Purpose**: Bridge the gap between initial brainstorming (BRAINSTORMING_IMPROVEMENTS.md), current implementation status (Phase 1-3 complete), and future vision (Phase 4+) with focus on becoming a comprehensive Internet Archive management tool.

**Created**: 2025-10-06  
**Last Updated**: 2025-10-07 (v1.7.0 Released - Consolidated Documentation)  
**Status**: 🎯 Active Planning - v1.7.0 Complete (~98% MD3 Compliance), Phase 4 Ready

---

## 📚 Documentation Navigation

**This is the comprehensive strategic roadmap.** For specific phase details, see:

- **Historical Reference**: [`mobile/flutter/docs/PHASES_1-3_COMPLETE.md`](../../mobile/flutter/docs/PHASES_1-3_COMPLETE.md) - Consolidated completion summary for Phases 1-3, Day 6, and v1.7.0 Material Design 3 Excellence
- **Current/Upcoming**: [`mobile/flutter/docs/PHASE_4-5_PLAN.md`](../../mobile/flutter/docs/PHASE_4-5_PLAN.md) - Detailed plans for Phase 4 (User Experience & Data Management) and Phase 5 (Polish & Play Store Release)
- **Immediate Next Steps**: [`mobile/flutter/docs/POST_V1.7.0_ROADMAP.md`](../../mobile/flutter/docs/POST_V1.7.0_ROADMAP.md) - Quick reference for what to do after v1.7.0

**Note**: Individual phase completion documents (phase-2-task-1-complete.md, etc.) have been consolidated into the documents above for better organization and reduced documentation sprawl.

---

## 📊 Current State Assessment

### ✅ What We've Built (Phases 1-3 + Day 6 Enhancements)

#### Phase 1: Basic UI Foundation
- ✅ Search and browse Internet Archive
- ✅ View archive metadata and details
- ✅ Basic file list display
- ✅ Download functionality (single/batch)
- ✅ Settings screen foundation
- ✅ Navigation structure

#### Phase 2: Content Preview System (46 formats)
- ✅ Text preview (TXT, MD, CSV, JSON, XML, etc.) - 15 formats
- ✅ Image preview (JPG, PNG, GIF, WebP, SVG, etc.) - 12 formats
- ✅ Basic audio preview - 8 formats
- ✅ Basic video preview - 11 formats
- ✅ Metadata caching system
- ✅ Offline caching infrastructure

#### Phase 3: Advanced Media Support (28 additional formats = 74 total)
- ✅ Task 1: Enhanced PDF preview with zoom/navigation (pdfx)
- ✅ Task 2: Advanced audio player with controls (just_audio)
- ✅ Task 3: Professional video player (chewie/video_player)
- ✅ Task 4: Archive preview system (ZIP, TAR, GZIP, 7Z, etc.) - 15+ formats
- ✅ Interactive file trees for archives
- ✅ Archive extraction capabilities

#### Day 6: UI Integration & API Compliance (RECENTLY COMPLETED ✅)
- ✅ **Phase 1: Bandwidth Management UI**
  - Bandwidth throttle with token bucket algorithm
  - Real-time speed display and limiting controls
  - Pause/resume functionality
  - Burst allowance for better performance
  
- ✅ **Phase 2: Download Priority Controls**
  - Priority queue system (High/Normal/Low)
  - Priority-based download scheduling
  - Visual priority indicators
  - Priority adjustment UI
  
- ✅ **Phase 3: Enhanced Progress Display**
  - File-level progress tracking
  - Speed indicators (MB/s, KB/s)
  - ETA calculations
  - Multi-file progress aggregation
  
- ✅ **Phase 4: Rate Limit Display**
  - Real-time rate limit status
  - Active/queued request counts
  - Rate limit warning indicators
  - Server retry-after tracking
  
- ✅ **Phase 5: Cache Statistics**
  - Cache hit/miss tracking
  - Cache size monitoring
  - Cache efficiency metrics
  - Purge controls
  
- ✅ **Phase 6: Error Handling UI**
  - Detailed error messages
  - Retry functionality
  - Error categorization (network, server, client)
  - User-friendly error descriptions

#### API Compliance Infrastructure (RECENTLY COMPLETED ✅)
- ✅ **Proper Rate Limiting**
  - Semaphore-based concurrency control (max 3-5 concurrent)
  - Configurable rate limiter with queue management
  - Global rate limiter instance for app-wide coordination
  - Statistics tracking (active, queued, capacity)

  
- ✅ **User-Agent Headers**
  - Proper identification with app version and contact
  - Dynamic Flutter/Dart version detection
  - Compliant format: `InternetArchiveHelper/1.6.0 (contact) Flutter/3.35.5 Dart/3.8.0`
  
- ✅ **Exponential Backoff Retry**
  - Automatic retry on transient errors (429, 503, 502, 504)
  - Exponential backoff: 1s → 2s → 4s → 8s → 60s
  - Max 5 retry attempts with configurable delays
  - Respects Retry-After header (both seconds and HTTP-date formats)
  
- ✅ **Request Throttling**
  - Minimum 150ms delay between API requests
  - Configurable delay per rate limiter instance
  - Prevents API overwhelming
  
- ✅ **Bandwidth Throttling**
  - Token bucket algorithm for smooth limiting
  - Configurable bytes per second
  - Burst allowance for better UX
  - Pause/resume controls
  
- ✅ **ETag Support**
  - Conditional GET requests (If-None-Match)
  - 304 Not Modified support
  - Cache validation
  - Bandwidth savings
  
- ✅ **Comprehensive Testing**
  - 115 tests covering all HTTP client functionality
  - Rate limiting tests
  - Retry logic tests
  - HTTP date parsing tests
  - Error handling tests

### 🔍 What's in BRAINSTORMING_IMPROVEMENTS.md But Not Yet Implemented

#### High-Value Features NOT Yet Implemented

##### 1. **Archive.org API Compliance** ✅ COMPLETED!
**Status**: Fully implemented as of Day 6
- ✅ Configurable rate limiting (max 3-5 concurrent)
- ✅ Proper User-Agent headers with dynamic versioning
- ✅ Exponential backoff on 429/503 errors
- ✅ Request delay between API calls (150-200ms)
- ✅ Bandwidth throttling with token bucket
- ✅ ETag support for conditional requests
- ✅ Retry-After header parsing (seconds and HTTP-date)
- ⚠️ Staggered download starts (partially - can be enhanced)

**Priority**: ✅ **COMPLETE** - Foundation solidified
**Next Steps**: Monitor and optimize based on real-world usage

##### 2. **Download Resume/Recovery** ⚠️ HIGH VALUE
**Status**: Partially implemented
- ⚠️ Retry functionality exists (just implemented)
- ❌ HTTP Range request support
- ❌ Partial download tracking
- ❌ Automatic resume on network failure
- ❌ Progress persistence across app restarts
- ⚠️ ETag verification available but not used for file integrity

**Priority**: 🟠 High - Next major feature to implement
**Implementation Phase**: Phase 4 Task 3 (Download Queue Management)

##### 3. **Parallel Downloads** ✅ IMPLEMENTED!
**Status**: Fully implemented
- ✅ Mobile: Background download service supports concurrency
- ✅ Configurable max concurrent (UI slider + settings)
- ✅ Priority-based scheduling (High/Normal/Low)
- ✅ Smart queue management
- ⚠️ CLI: Sequential downloads only (enhancement opportunity)
- ⚠️ Staggered start times (can be enhanced further)

**Priority**: ✅ **COMPLETE** for mobile
**Next Steps**: 
- Consider adding parallel downloads to CLI
- Fine-tune staggered start algorithm

##### 4. **Search Functionality** ⚠️ CORE FEATURE
**Status**: Basic implementation, needs major enhancement
- ✅ Basic search API integration
- ❌ Advanced search with field queries
- ❌ Date range filters
- ❌ Media type filters
- ❌ Sort options (relevance, date, downloads, size)
- ❌ Search history
- ❌ Saved searches
- ❌ Search suggestions

**Priority**: 🟠 High - Core discovery feature
**Implementation Phase**: Phase 4 Task 2 (Advanced Search)

##### 5. **Smart Offline Mode** ⚠️ MOBILE-CRITICAL
**Status**: Foundation exists, needs enhancement
- ✅ Metadata caching infrastructure
- ✅ Basic offline metadata viewing
- ❌ Offline queue management
- ❌ Sync scheduling (Wi-Fi only, off-peak)
- ❌ Offline indicator for cached items
- ❌ Background sync service
- ❌ Conflict resolution

**Priority**: 🟡 Medium-High - Mobile user expectation
**Implementation Phase**: Phase 4 Task 3 (Offline Improvements)

##### 6. **Advanced Filtering UI** ⚠️ UX IMPROVEMENT
**Status**: Basic filters exist, needs rich UI
- ✅ Basic file type filtering (current implementation)
- ❌ Visual filter builder with chips
- ❌ Size range sliders
- ❌ Date range pickers
- ❌ Regex/pattern filters
- ❌ Filter presets (save/load)
- ❌ Filter combinations (AND/OR logic)

**Priority**: 🟡 Medium - UX enhancement
**Implementation Phase**: Phase 4 Task 2 (Advanced Search & Filters)

##### 7. **Batch Operations** ⚠️ POWER USER FEATURE
**Status**: Minimal implementation
- ⚠️ Batch download exists (select multiple files)
- ❌ Multi-select UI for batch operations
- ❌ Bulk actions (delete, share, export)
- ❌ Selection state management
- ❌ Batch operation progress tracking

**Priority**: 🟡 Medium - Power user productivity
**Implementation Phase**: Phase 5+ (Power User Features)

##### 8. **Rich Media Enhancements** ⚠️ POLISH
**Status**: Good foundation, room for polish
- ✅ Basic zoom/pan for images
- ✅ Video playback controls
- ✅ Audio player
- ❌ Image rotation/editing
- ❌ Audio waveform visualization
- ❌ Video thumbnail generation
- ❌ Subtitle support for videos
- ❌ Audio equalizer

**Priority**: 🟢 Low-Medium - Nice to have polish
**Implementation Phase**: Phase 6+ (Polish & Enhancement)

##### 9. **Performance Features** ⚠️ SCALABILITY
**Status**: Basic optimization done
- ✅ Virtual scrolling (ListView.builder)
- ✅ Basic image caching
- ❌ Disk-based image cache with LRU
- ❌ Thumbnail pre-generation
- ❌ Background isolate processing
- ❌ Memory usage optimization
- ❌ Large file handling (streaming)

**Priority**: 🟡 Medium - Important at scale
**Implementation Phase**: Ongoing (performance sprints)

##### 10. **CLI Advanced Features** ⚠️ POWER USERS
**Status**: Basic CLI complete
- ✅ Download command
- ✅ Metadata viewing
- ✅ File filtering
- ❌ Interactive REPL mode
- ❌ Shell completion (bash/zsh/fish/pwsh)
- ❌ Multiple output formats (JSON/YAML/table)
- ❌ Progress bar enhancements
- ❌ Configuration profiles
- ❌ Dry run mode

**Priority**: 🟢 Low-Medium - CLI power users
**Implementation Phase**: Phase 5+ (CLI Enhancements)

---

## 🎯 The Vision: Full Internet Archive Helper

### Current Positioning
**ia-get Today**: A mobile-first download manager with excellent preview capabilities for 74+ file formats.

**Target Positioning**: A comprehensive Internet Archive companion app for both **downloading** and **uploading** content, with collection management, metadata editing, and community features.

### Strategic Direction

#### Year 1: Master Downloads (Current Focus)
**Goal**: Become the best way to download and manage Internet Archive content on mobile

##### Q1-Q2 2025 (Current - Phases 4-5)
- ✅ Complete Phase 3 (Advanced Media) - DONE
- 🎯 Phase 4: User Experience & Data Management (Q1)
  - Favorites & Collections
  - Advanced Search & Filters
  - Download Queue Management
  - Settings & Customization
  - Statistics & Insights
- 🎯 API Compliance Sprint (Q1) - **URGENT**
  - Proper rate limiting
  - Request throttling
  - User-Agent compliance
  - Exponential backoff
- 🎯 Phase 5: Power User Features (Q2)
  - Batch operations polish
  - CLI enhancements
  - Scripting support
  - Advanced filtering

##### Q3-Q4 2025 (Phases 6-7)
- 🎯 Phase 6: Enterprise Features
  - Multi-account support
  - Team collections (shared favorites)
  - Export/import functionality
  - Advanced statistics
  - Scheduled downloads
- 🎯 Phase 7: Community Features
  - Share collections publicly
  - Discover popular collections
  - User ratings/reviews
  - Follow other users
  - Activity feed

#### Year 2: Add Upload Capabilities (2026)
**Goal**: Expand from download-only to full bidirectional archive management

##### Q1-Q2 2026 (Phases 8-9)
- 🎯 Phase 8: Upload Foundation
  - Internet Archive account authentication (OAuth)
  - Upload API integration
  - Single file upload UI
  - Metadata entry forms
  - Upload progress tracking
  - Error handling & validation
  
- 🎯 Phase 9: Bulk Upload System
  - Multi-file upload
  - Folder upload with structure preservation
  - Metadata templates
  - CSV metadata import
  - Upload queue management
  - Background upload service
  - Resume interrupted uploads

##### Q3-Q4 2026 (Phases 10-11)
- 🎯 Phase 10: Content Creation Tools
  - Photo/video capture in-app
  - Audio recording
  - Document scanning (OCR)
  - Metadata extraction (EXIF, ID3, etc.)
  - Auto-tagging with AI
  - Collection builder for new uploads
  
- 🎯 Phase 11: Upload Management
  - Draft items (save before publish)
  - Edit existing items
  - Manage uploaded collections
  - Analytics for uploaded content
  - Item moderation tools
  - Version control for items

#### Year 3: Ecosystem Integration (2027+)
**Goal**: Become a platform for Internet Archive power users and creators

##### Q1-Q2 2027 (Phases 12-13)
- 🎯 Phase 12: Creator Tools
  - Creator dashboard
  - Upload analytics & insights
  - Item performance tracking
  - Monetization integration (if IA supports)
  - Content recommendations for creators
  
- 🎯 Phase 13: Advanced Collaboration
  - Collaborative collections
  - Multi-user uploads
  - Permission management
  - Review/approval workflows
  - Organization accounts

##### Q3-Q4 2027 (Phases 14-15)
- 🎯 Phase 14: API Platform
  - Public API for ia-get features
  - Webhook support
  - Third-party integrations
  - Plugin system
  - Automation workflows
  
- 🎯 Phase 15: AI/ML Features
  - Smart content recommendations
  - Auto-categorization of downloads
  - Duplicate detection
  - Content quality analysis
  - Accessibility improvements (auto-captions, alt-text)

---

## 🏗️ Architecture Evolution

### Current Architecture (Phase 1-3)
```
[Flutter Mobile App]
    ├── UI Layer (screens, widgets)
    ├── Services Layer
    │   ├── internet_archive_api.dart (API client)
    │   ├── archive_service.dart (business logic)
    │   ├── metadata_cache.dart (caching)
    │   ├── background_download_service.dart
    │   └── file_preview_service.dart
    ├── Models Layer
    │   ├── archive_metadata.dart
    │   ├── file_info.dart
    │   └── download_state.dart
    └── Utils Layer

[Rust CLI] (separate, minimal mobile integration)
    ├── archive_api.rs
    ├── downloader.rs
    ├── metadata.rs
    └── cli.rs
```

### Target Architecture (Phase 8+: Upload Support)
```
[Flutter Mobile App]
    ├── UI Layer
    │   ├── Download Screens (existing)
    │   ├── Upload Screens (new)
    │   ├── Content Creation Tools (new)
    │   └── Shared Components
    │
    ├── Services Layer
    │   ├── Download Services (existing)
    │   │   ├── internet_archive_api.dart
    │   │   ├── download_manager.dart
    │   │   └── cache_manager.dart
    │   │
    │   ├── Upload Services (new)
    │   │   ├── upload_api.dart
    │   │   ├── upload_manager.dart
    │   │   ├── metadata_builder.dart
    │   │   └── content_validator.dart
    │   │
    │   ├── Authentication (new)
    │   │   ├── ia_auth_service.dart (OAuth)
    │   │   └── session_manager.dart
    │   │
    │   └── Sync Services
    │       ├── bidirectional_sync.dart
    │       └── conflict_resolver.dart
    │
    ├── Models Layer
    │   ├── Download Models (existing)
    │   ├── Upload Models (new)
    │   │   ├── upload_item.dart
    │   │   ├── upload_metadata.dart
    │   │   └── upload_state.dart
    │   └── Shared Models
    │
    ├── Storage Layer (enhanced)
    │   ├── Local Database (sqflite)
    │   │   ├── Downloads table
    │   │   ├── Uploads table
    │   │   ├── Favorites table
    │   │   ├── Collections table
    │   │   └── Cache table
    │   └── File Storage
    │       ├── Downloaded files
    │       ├── Upload queue
    │       └── Draft items
    │
    └── Background Services
        ├── Download worker (existing)
        ├── Upload worker (new)
        └── Sync worker (new)

[Rust CLI] (enhanced for upload support)
    ├── Download Module (existing)
    ├── Upload Module (new)
    │   ├── upload_api.rs
    │   ├── multipart_upload.rs
    │   └── metadata_builder.rs
    ├── Auth Module (new)
    └── Sync Module (new)
```

---

## 📋 Current Status: API Compliance Complete! 🎉

### ✅ API Compliance Sprint - COMPLETED!

**Achievement**: Successfully implemented all critical API compliance features over Day 6 development session (~4-5 hours of focused work).

#### What We Built:

1. ✅ **Rate Limiting Infrastructure**
   - File: `lib/services/rate_limiter.dart`
   - Semaphore-based concurrency control
   - Configurable max concurrent (default: 3)
   - Min delay between requests (default: 150ms)
   - Queue management and statistics

2. ✅ **Enhanced HTTP Client**
   - File: `lib/services/ia_http_client.dart`
   - Proper User-Agent with dynamic versioning
   - Exponential backoff retry (1s→2s→4s→8s→60s)
   - Retry-After header parsing (seconds + HTTP-date RFC 7231)
   - ETag support for conditional GET requests
   - Comprehensive error handling and categorization
   - 115 passing tests

3. ✅ **Bandwidth Throttling**
   - File: `lib/services/bandwidth_throttle.dart`
   - Token bucket algorithm
   - Configurable bytes/second limits
   - Burst allowance for UX
   - Pause/resume functionality
   - Real-time statistics

4. ✅ **UI Integration**
   - Rate limit status display
   - Bandwidth controls with sliders
   - Priority queue UI
   - Progress tracking enhancements
   - Cache statistics dashboard
   - Error handling improvements

#### Testing Results:
- ✅ All 115 tests passing
- ✅ flutter analyze: No issues
- ✅ HTTP date parsing validated
- ✅ Invalid input handling verified
- ✅ Retry logic tested thoroughly

---

## 🎯 Next Steps: Phase 4 Implementation

**NOW THAT API COMPLIANCE IS COMPLETE**, we can confidently move forward with Phase 4 features.

### Phase 4: User Experience & Data Management

**Goal**: Make ia-get a powerful, user-friendly archive management tool with excellent data organization.

**Estimated Duration**: 3-4 weeks  
**Status**: Ready to begin

#### Phase 4 Tasks Overview

##### Task 1: Favorites & Collections System (Week 1)
**Priority**: 🟠 High  
**Effort**: 6-8 hours

**Features to Implement**:
1. **Favorites Infrastructure** (2-3 hours)
   - Database schema for favorites table
   - Favorites service with CRUD operations
   - Star/unstar functionality
   - Sync across app sessions

2. **Collections System** (3-4 hours)
   - Collections database schema
   - Create/rename/delete collections
   - Add/remove items from collections
   - Collection viewing UI
   - Smart collections (auto-populated by rules)

3. **UI Components** (2-3 hours)
   - Star button on archive cards
   - Favorites screen with grid/list view
   - Collection management screen
   - Collection picker dialog
   - Batch add to collection

**Files to Create/Modify**:
```
lib/
  models/
    favorite.dart (new)
    collection.dart (new)
  services/
    favorites_service.dart (new)
    collections_service.dart (new)
  screens/
    favorites_screen.dart (new)
    collections_screen.dart (new)
  widgets/
    favorite_button.dart (new)
    collection_picker.dart (new)
  database/
    schema_v2.dart (upgrade from v1)
```

**Database Schema**:
```sql
-- Favorites table
CREATE TABLE favorites (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  identifier TEXT UNIQUE NOT NULL,
  title TEXT,
  mediatype TEXT,
  added_at INTEGER NOT NULL,
  metadata_json TEXT  -- Cache full metadata
);

-- Collections table
CREATE TABLE collections (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  description TEXT,
  icon TEXT,  -- Material icon name
  color INTEGER,  -- Color value
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  is_smart INTEGER DEFAULT 0,  -- Smart collection flag
  smart_rules_json TEXT  -- Rules for smart collections
);

-- Collection items junction table
CREATE TABLE collection_items (
  collection_id INTEGER NOT NULL,
  identifier TEXT NOT NULL,
  added_at INTEGER NOT NULL,
  PRIMARY KEY (collection_id, identifier),
  FOREIGN KEY (collection_id) REFERENCES collections(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX idx_favorites_mediatype ON favorites(mediatype);
CREATE INDEX idx_favorites_added_at ON favorites(added_at DESC);
CREATE INDEX idx_collection_items_identifier ON collection_items(identifier);
```

##### Task 2: Advanced Search & Filters (Week 1-2)
**Priority**: 🟠 High  
**Effort**: 8-10 hours

**Features to Implement**:
1. **Enhanced Search API Integration** (3-4 hours)
   - Advanced query builder
   - Field-specific search (title, creator, subject, etc.)
   - Boolean operators (AND, OR, NOT)
   - Date range filters
   - Mediatype filters
   - Sort options (relevance, date, downloads, week)

2. **Search UI Enhancements** (3-4 hours)
   - Advanced search dialog
   - Filter chips UI
   - Date range picker
   - Mediatype selector
   - Sort dropdown
   - Clear filters button

3. **Search History & Saved Searches** (2-3 hours)
   - Recent searches persistence
   - Save search with name
   - Quick access to saved searches
   - Search suggestions based on history

**Files to Create/Modify**:
```
lib/
  services/
    advanced_search_service.dart (new)
    search_history_service.dart (new)
  models/
    search_query.dart (new)
    search_filter.dart (new)
    saved_search.dart (new)
  screens/
    advanced_search_screen.dart (new)
  widgets/
    search_filters_widget.dart (new)
    filter_chips.dart (new)
    date_range_picker.dart (new)
    mediatype_selector.dart (new)
```

**Advanced Search Query Builder**:
```dart
class SearchQueryBuilder {
  String? query;
  Map<String, String> fieldQueries = {}; // title:value, creator:value
  List<String> mediatypes = [];
  DateRange? dateRange;
  SortOption sortBy = SortOption.relevance;
  
  String build() {
    final parts = <String>[];
    
    // Main query
    if (query != null && query!.isNotEmpty) {
      parts.add(query!);
    }
    
    // Field queries
    fieldQueries.forEach((field, value) {
      parts.add('$field:($value)');
    });
    
    // Mediatype filter
    if (mediatypes.isNotEmpty) {
      final types = mediatypes.map((t) => 'mediatype:$t').join(' OR ');
      parts.add('($types)');
    }
    
    // Date range
    if (dateRange != null) {
      parts.add('date:[${dateRange!.start} TO ${dateRange!.end}]');
    }
    
    return parts.join(' AND ');
  }
}
```

##### Task 3: Download Queue Management (Week 2-3)
**Priority**: 🟠 High  
**Effort**: 10-12 hours

**Features to Implement**:
1. **Download Resume/Recovery** (4-5 hours)
   - HTTP Range request support
   - Partial download tracking in database
   - Auto-resume on network reconnect
   - Progress persistence across restarts
   - File verification with ETag/checksums

2. **Queue Management UI** (3-4 hours)
   - Download queue screen
   - Reorder downloads (drag and drop)
   - Pause/resume individual downloads
   - Cancel with option to keep partial
   - Queue statistics dashboard

3. **Smart Scheduling** (3-4 hours)
   - Priority-based scheduling (already started)
   - Size-based optimization (small files first option)
   - Network-aware scheduling (Wi-Fi only option)
   - Time-based scheduling (off-peak downloads)
   - Bandwidth distribution among active downloads

**Files to Create/Modify**:
```
lib/
  services/
    resumable_download_service.dart (new)
    download_scheduler.dart (new)
  models/
    download_task.dart (enhance existing)
    download_queue.dart (new)
  screens/
    download_queue_screen.dart (new)
  widgets/
    download_queue_item.dart (new)
    queue_statistics.dart (new)
```

**Resumable Download Implementation**:
```dart
class ResumableDownloadService {
  /// Start or resume a download
  Future<void> downloadFile({
    required String url,
    required String savePath,
    required String identifier,
    String? etag,
    ProgressCallback? onProgress,
  }) async {
    final file = File(savePath);
    int startByte = 0;
    
    // Check if partial download exists
    if (await file.exists()) {
      startByte = await file.length();
      
      // Verify ETag if available
      if (etag != null) {
        final headResponse = await _client.head(Uri.parse(url));
        final serverEtag = headResponse.headers['etag'];
        
        if (serverEtag != etag) {
          // File changed on server, start fresh
          await file.delete();
          startByte = 0;
        }
      }
    }
    
    // Request with Range header
    final request = http.Request('GET', Uri.parse(url));
    if (startByte > 0) {
      request.headers['Range'] = 'bytes=$startByte-';
    }
    
    final response = await _client.send(request);
    
    // Handle 206 Partial Content or 200 OK
    if (response.statusCode == 206 || response.statusCode == 200) {
      final mode = response.statusCode == 206 
        ? FileMode.append 
        : FileMode.write;
        
      await _saveStreamToFile(
        response,
        file,
        mode: mode,
        startByte: startByte,
        onProgress: onProgress,
      );
    }
  }
}
```

##### Task 4: Settings & Customization (Week 3)
**Priority**: 🟡 Medium-High  
**Effort**: 6-8 hours

**Features to Implement**:
1. **Download Settings Enhancements** (2-3 hours)
   - Default download location picker
   - Auto-organize by mediatype
   - File naming templates
   - Duplicate handling options
   - Auto-delete after X days

2. **Performance Settings** (2-3 hours)
   - Cache size limit
   - Image quality preferences
   - Preload settings
   - Background sync schedule
   - Data saver mode

3. **Appearance Settings** (2-3 hours)
   - Dark/light/system theme
   - Grid vs list view default
   - Compact view option
   - Font size scaling
   - Color scheme customization

**Files to Modify**:
```
lib/
  screens/
    settings_screen.dart (enhance existing)
  widgets/
    settings_section.dart (new)
    theme_picker.dart (new)
  services/
    theme_service.dart (new)
    settings_service.dart (enhance)
```

##### Task 5: Statistics & Insights (Week 4)
**Priority**: 🟡 Medium  
**Effort**: 4-6 hours

**Features to Implement**:
1. **Download Statistics** (2-3 hours)
   - Total downloads count
   - Total data downloaded
   - Download history chart
   - Top downloaded mediatypes
   - Bandwidth usage over time

2. **Usage Insights** (2-3 hours)
   - Most viewed archives
   - Search trends
   - Storage usage breakdown
   - Time spent in app
   - Feature usage analytics (local only)

**Files to Create**:
```
lib/
  services/
    statistics_service.dart (new)
    analytics_service.dart (new)
  models/
    download_statistics.dart (new)
    usage_insights.dart (new)
  screens/
    statistics_screen.dart (new)
  widgets/
    statistics_chart.dart (new)
    insights_card.dart (new)
```

---

## 🗓️ Updated Implementation Timeline

### ✅ Completed (October 2025)
- Phases 1-3: Foundation + Media Support (74 formats)
- Day 6: API Compliance + UI Integration

### 🎯 Q4 2025 (October - December)

#### Week 1-2 (Current)
- **Task 1**: Favorites & Collections (6-8 hours)
- **Task 2**: Advanced Search (start, 4-5 hours)

#### Week 3-4
- **Task 2**: Advanced Search (complete, 4-5 hours)
- **Task 3**: Download Queue (start, 6 hours)

#### Week 5-6
- **Task 3**: Download Queue (complete, 6 hours)
- **Task 4**: Settings & Customization (6-8 hours)

#### Week 7-8
- **Task 5**: Statistics & Insights (4-6 hours)
- **Testing & Bug Fixes**
- **Documentation Updates**
- **Phase 4 Completion Report**

### 🎯 Q1 2026 (January - March)
- **Phase 5**: Power User Features
  - Batch operations polish
  - CLI enhancements (shell completion, REPL mode)
  - Export/import functionality
  - Scripting support

### 🎯 Q2 2026 (April - June)
- **Phase 6**: Enterprise Features
  - Multi-account support
  - Team collections
  - Advanced statistics
  - Scheduled downloads

### 🎯 Q3-Q4 2026
- **Phase 7**: Community Features
- **Phase 8**: Upload Foundation (Q4 start)
        return _client.get(
          Uri.parse(url),
          headers: {
            'User-Agent': userAgent,
            ...?headers,
          },
        );
      });
      
      return response;
    } finally {
      _rateLimiter.release();
    }
  }
  
  Future<T> _retryWithBackoff<T>(Future<T> Function() operation) async {
    int attempt = 0;
    Duration delay = const Duration(seconds: 1);
    
    while (true) {
      try {
        return await operation();
      } on http.ClientException catch (e) {
        if (e.message.contains('429') || e.message.contains('503')) {
          attempt++;
          if (attempt >= _retryPolicy.maxAttempts) rethrow;
          
          await Future.delayed(delay);
          delay *= 2; // Exponential backoff
          delay = Duration(
            milliseconds: min(delay.inMilliseconds, 60000) // Max 60s
          );
        } else {
          rethrow;
        }
      }
    }
  }
}
```

#### Task 3: Bandwidth Throttling (2 hours)
```dart
// lib/services/bandwidth_throttle.dart
class BandwidthThrottle {
  final int? maxBytesPerSecond;
  final TokenBucket _bucket;
  
  BandwidthThrottle({this.maxBytesPerSecond}) 
    : _bucket = TokenBucket(
        capacity: maxBytesPerSecond ?? int.maxFinite,
        refillRate: maxBytesPerSecond ?? int.maxFinite,
      );
  
  Future<void> waitForTokens(int bytes) async {
    if (maxBytesPerSecond == null) return;
    await _bucket.acquire(bytes);
  }
  
  Stream<List<int>> throttleStream(Stream<List<int>> source) async* {
    await for (final chunk in source) {
      await waitForTokens(chunk.length);
      yield chunk;
    }
  }
}
```

#### Task 4: ETag Caching (2 hours)
```dart
// Enhanced metadata_cache.dart
class MetadataCache {
  Future<ArchiveMetadata?> getIfNotModified(
    String identifier,
    String? etag,
  ) async {
    if (etag == null) return null;
    
    final cached = await _getCached(identifier);
    if (cached == null) return null;
    
    if (cached.etag == etag && !cached.isExpired) {
      return cached.metadata;
    }
    
    return null;
  }
  
  Future<void> saveWithETag(
    String identifier,
    ArchiveMetadata metadata,
    String? etag,
  ) async {
    await _storage.save(CachedMetadata(
      identifier: identifier,
      metadata: metadata,
      etag: etag,
      cachedAt: DateTime.now(),
    ));
  }
}
```

#### Task 5: Download Orchestration (2-3 hours)
```dart
// lib/services/download_orchestrator.dart
class DownloadOrchestrator {
  final int maxConcurrent = 3;
  final Duration staggerDelay = const Duration(milliseconds: 500);
  
  Future<void> downloadBatch(List<FileInfo> files) async {
    final queue = Queue<FileInfo>.from(files);
    final active = <Future>[];
    
    while (queue.isNotEmpty || active.isNotEmpty) {
      // Start new downloads up to limit
      while (active.length < maxConcurrent && queue.isNotEmpty) {
        final file = queue.removeFirst();
        
        // Stagger starts
        if (active.isNotEmpty) {
          await Future.delayed(staggerDelay);
        }
        
        final future = _downloadFile(file);
        active.add(future);
        
        // Remove completed downloads
        future.then((_) => active.remove(future));
      }
      
      // Wait for at least one to complete
      if (active.isNotEmpty) {
        await Future.any(active);
      }
    }
  }
}
```

#### Task 6: Monitoring & Metrics (1-2 hours)
```dart
// lib/services/api_metrics.dart
class APIMetrics {
  int _totalRequests = 0;
  int _failedRequests = 0;
  int _rateLimitHits = 0;
  Duration _totalWaitTime = Duration.zero;
  
  void recordRequest() => _totalRequests++;
  void recordFailure() => _failedRequests++;
  void recordRateLimit() => _rateLimitHits++;
  void recordWaitTime(Duration duration) => _totalWaitTime += duration;
  
  Map<String, dynamic> getMetrics() => {
    'total_requests': _totalRequests,
    'failed_requests': _failedRequests,
    'rate_limit_hits': _rateLimitHits,
    'total_wait_time_ms': _totalWaitTime.inMilliseconds,
    'success_rate': _totalRequests > 0 
      ? (_totalRequests - _failedRequests) / _totalRequests
      : 0.0,
  };
}
```

### Success Criteria for API Compliance Sprint
- [ ] All HTTP requests include proper User-Agent header
- [ ] Rate limiting enforced (max 3-5 concurrent downloads)
- [ ] Minimum 150ms delay between metadata API calls
- [ ] Exponential backoff on 429/503 errors (1s → 2s → 4s → 8s → max 60s)
- [ ] Bandwidth throttling available in settings (optional, user-configured)
- [ ] Download starts staggered by 500ms minimum
- [ ] ETag support for conditional metadata requests
- [ ] API metrics tracked and logged
- [ ] Settings UI for configuring rate limits
- [ ] Documentation updated with API compliance details

---

## 📊 Phase 4 Integration with Brainstorming

### Overlap Analysis: Phase 4 vs BRAINSTORMING_IMPROVEMENTS.md

#### ✅ Good Alignment (Phase 4 covers these brainstormed items)

1. **Favorites & Collections** ✅
   - Brainstorming: Not explicitly mentioned
   - Phase 4: Task 1 - Full implementation
   - **Gap**: Brainstorming focused more on external features; Phase 4 correctly identifies this as UX foundation

2. **Advanced Search** ✅
   - Brainstorming: Section "Search Functionality" (pg 10)
   - Phase 4: Task 2 - Comprehensive implementation
   - **Alignment**: Good match! Both include:
     - Search history
     - Saved searches
     - Advanced filters
     - Sort options

3. **Download Queue Management** ⚠️ Partial
   - Brainstorming: Covered in "Parallel Downloads", "Download Resume", "Retry Logic"
   - Phase 4: Task 3 - Includes queue reordering, priority, scheduling
   - **Gap**: Phase 4 needs to add:
     - Download resume (from brainstorming)
     - Retry with exponential backoff (from brainstorming)
     - Parallel downloads configuration (from brainstorming)

4. **Settings & Customization** ✅
   - Brainstorming: "Theme Customization" section
   - Phase 4: Task 4 - Full theme, cache, data controls
   - **Alignment**: Excellent! Phase 4 expands on brainstorming

5. **Statistics & Insights** ✅
   - Brainstorming: Not explicitly covered
   - Phase 4: Task 5 - New addition
   - **Gap**: Brainstorming missed this UX feature; Phase 4 adds value

#### ⚠️ Missing from Phase 4 (Should add to Phase 5+)

1. **Download Resume** ❌
   - **In Brainstorming**: Yes (HTTP Range requests, section B)
   - **In Phase 4**: No (should be in Task 3)
   - **Action**: Add to Phase 4 Task 3 or make Phase 5 Task 1

2. **Retry Logic with Backoff** ❌
   - **In Brainstorming**: Yes (exponential backoff, section B)
   - **In Phase 4**: No
   - **Action**: Add to API Compliance Sprint + Phase 4 Task 3

3. **Bandwidth Throttling** ❌
   - **In Brainstorming**: Yes (token bucket, section C)
   - **In Phase 4**: No (only mentioned in settings)
   - **Action**: Add to API Compliance Sprint

4. **Virtual Scrolling** ❌
   - **In Brainstorming**: Yes (performance optimization)
   - **In Phase 4**: No
   - **Action**: Already implemented in Phase 2! Just needs docs

5. **Rich Media Preview Enhancements** ❌
   - **In Brainstorming**: Yes (zoom/pan/rotate, waveforms)
   - **In Phase 4**: No
   - **Action**: Phase 6+ (polish features)

6. **Offline Mode Improvements** ⚠️
   - **In Brainstorming**: Yes (full offline support, sync)
   - **In Phase 4**: Mentioned in Task 3 but not detailed
   - **Action**: Expand Phase 4 Task 3 or make Phase 5 focus

7. **Batch Operations UI** ❌
   - **In Brainstorming**: Yes (multi-select, bulk actions)
   - **In Phase 4**: No
   - **Action**: Phase 5 (Power User Features)

8. **CLI Improvements** ❌
   - **In Brainstorming**: Yes (REPL, shell completion, etc.)
   - **In Phase 4**: No (mobile-focused)
   - **Action**: Phase 5+ (Separate CLI track)

---

## 🎯 Revised Priority Matrix

### URGENT (Before Phase 4)
1. **API Compliance Sprint** (1 week)
   - Rate limiting
   - User-Agent headers
   - Exponential backoff
   - Request throttling
   - Staggered starts

### HIGH PRIORITY (Phase 4 - Q1 2025)
1. **Task 1**: Favorites & Collections (3-4h)
2. **Task 2**: Advanced Search & Filters (4-5h)
3. **Task 3**: Download Queue + Resume + Retry (6-8h) ⬅️ EXPANDED
   - Queue reordering & priority
   - HTTP Range request resume
   - Exponential backoff retry
   - Scheduled downloads
   - Bandwidth limiting UI
4. **Task 4**: Settings & Customization (3-4h)
5. **Task 5**: Statistics & Insights (2-3h)

**Total Phase 4**: 20-27 hours (revised from 17-23h)

### MEDIUM PRIORITY (Phase 5 - Q2 2025)
1. **Enhanced Offline Mode**
   - Sync scheduling
   - Background sync service
   - Conflict resolution
   - Offline indicators
   
2. **Batch Operations UI**
   - Multi-select with checkboxes
   - Bulk actions (delete, share, export)
   - Selection state management
   
3. **CLI Enhancements**
   - Interactive REPL mode
   - Shell completions
   - Output format options
   - Configuration profiles
   
4. **Performance Optimizations**
   - Disk-based image cache
   - Background isolate processing
   - Memory optimization
   - Large file streaming

### LOW PRIORITY (Phase 6+ - Q3-Q4 2025)
1. **Rich Media Polish**
   - Image rotation/editing
   - Audio waveform visualization
   - Video subtitles
   - Advanced zoom controls
   
2. **Smart Features**
   - AI recommendations
   - Auto-categorization
   - Duplicate detection
   
3. **Social/Community Features**
   - Share collections
   - Follow users
   - Ratings/reviews

---

## 🚀 The Upload Journey: Detailed Roadmap

### Phase 8: Upload Foundation (Q1 2026) - 6-8 weeks

#### Week 1-2: Authentication & API Setup
**Goal**: Establish secure authentication with Internet Archive

**Tasks**:
1. Internet Archive OAuth integration
   - Register app with IA (get client ID/secret)
   - Implement OAuth 2.0 flow
   - Secure token storage (flutter_secure_storage)
   - Token refresh mechanism
   - Session management

2. Upload API research
   - Study IA upload API documentation
   - Test API endpoints with Postman/curl
   - Understand S3-compatible upload format
   - Research metadata requirements
   - Identify file size limits and constraints

**Deliverables**:
- `lib/services/ia_auth_service.dart` - OAuth implementation
- `lib/services/ia_upload_api.dart` - Upload API client
- Authentication UI (login/logout screens)
- API documentation summary

#### Week 3-4: Single File Upload MVP
**Goal**: Users can upload a single file with minimal metadata

**Tasks**:
1. File picker integration
   - Use `file_picker` package
   - Support images, videos, audio, documents
   - File validation (size, type)
   - Preview selected file

2. Basic metadata form
   - Title (required)
   - Description (optional)
   - Media type selector
   - Tags (comma-separated)
   - License selector

3. Upload service implementation
   - Chunked upload for large files
   - Progress tracking
   - Error handling
   - Success/failure states

4. Upload UI
   - File selection screen
   - Metadata entry form
   - Upload progress screen
   - Success confirmation

**Deliverables**:
- `lib/screens/upload_screen.dart`
- `lib/services/upload_service.dart`
- `lib/models/upload_item.dart`
- Basic upload workflow (end-to-end)

#### Week 5-6: Upload Queue & Background Processing
**Goal**: Support queued uploads with background processing

**Tasks**:
1. Upload queue database
   - Sqflite schema for upload queue
   - CRUD operations
   - State management (pending/uploading/complete/failed)

2. Background upload service
   - Use `workmanager` for background tasks
   - Handle app termination
   - Resume interrupted uploads
   - Retry failed uploads

3. Upload manager UI
   - Queue list view
   - Upload status indicators
   - Pause/resume/cancel controls
   - Retry failed uploads

4. Notifications
   - Upload progress notifications
   - Completion notifications
   - Error notifications with actions

**Deliverables**:
- `lib/services/upload_queue_service.dart`
- `lib/services/background_upload_worker.dart`
- Upload management screen
- Background processing documentation

#### Week 7-8: Testing & Polish
**Goal**: Stable, production-ready upload feature

**Tasks**:
1. Comprehensive testing
   - Unit tests for upload service
   - Integration tests for queue
   - UI tests for upload flow
   - Test various file types/sizes

2. Error handling improvements
   - Network error recovery
   - Authentication error handling
   - Validation error messages
   - Rate limit handling

3. UX polish
   - Loading states
   - Error states with helpful messages
   - Success animations
   - Accessibility improvements

4. Documentation
   - User guide for uploads
   - API documentation
   - Developer guide
   - Known limitations

**Deliverables**:
- Test suite (80%+ coverage)
- Polished upload UI
- Complete documentation
- Phase 8 completion report

**Success Criteria**:
- [ ] User can authenticate with Internet Archive
- [ ] User can select and upload a single file
- [ ] Basic metadata can be added (title, description, tags)
- [ ] Upload progress is visible
- [ ] Uploads continue in background
- [ ] Failed uploads can be retried
- [ ] User receives notifications for upload status
- [ ] Documentation is complete

---

### Phase 9: Bulk Upload System (Q2 2026) - 6-8 weeks

#### Week 1-2: Multi-File Selection & Upload
**Goal**: Users can select and upload multiple files at once

**Tasks**:
1. Multi-file picker
   - Select multiple files simultaneously
   - Select entire folders (with structure)
   - Show file count and total size
   - Preview file list before upload

2. Batch metadata entry
   - Apply metadata to all files
   - Individual metadata override
   - Metadata templates
   - CSV import for bulk metadata

3. Batch upload orchestration
   - Upload files in sequence or parallel
   - Smart scheduling based on size
   - Pause/resume entire batch
   - Cancel individual or all files

**Deliverables**:
- Multi-file selection UI
- Batch metadata entry screen
- Batch upload service
- CSV metadata import tool

#### Week 3-4: Folder Upload with Structure
**Goal**: Preserve folder structure when uploading directories

**Tasks**:
1. Directory traversal
   - Recursively scan folders
   - Preserve folder hierarchy
   - Handle nested structures
   - Exclude hidden/system files

2. Structure preservation
   - Map local paths to IA structure
   - Create collection hierarchy
   - Maintain relationships
   - Document folder mappings

3. Folder upload UI
   - Visual folder tree
   - Select/deselect folders
   - Show total file count/size
   - Preview structure before upload

**Deliverables**:
- Folder traversal service
- Structure preservation logic
- Folder upload UI
- Documentation on structure mapping

#### Week 5-6: Advanced Metadata Management
**Goal**: Rich metadata editing and templates

**Tasks**:
1. Metadata templates
   - Create reusable templates
   - Template categories (video, audio, book, etc.)
   - Save/load templates
   - Share templates (export/import)

2. Advanced metadata editor
   - All IA metadata fields
   - Field validation
   - Auto-fill suggestions
   - Metadata preview

3. Metadata extraction
   - EXIF data from images
   - ID3 tags from audio
   - Video metadata
   - Document properties
   - Auto-populate form

**Deliverables**:
- Metadata template system
- Advanced metadata editor UI
- Metadata extraction service
- Template library

#### Week 7-8: Upload Queue Management
**Goal**: Professional-grade upload queue with advanced controls

**Tasks**:
1. Queue operations
   - Reorder queue (drag-and-drop)
   - Priority system (high/normal/low)
   - Schedule uploads (time-based)
   - Pause/resume/cancel queue

2. Queue persistence
   - Save queue state
   - Resume after app restart
   - Handle app updates
   - Queue backup/restore

3. Upload monitoring
   - Real-time progress
   - Speed/ETA calculation
   - Error tracking
   - Success rate statistics

4. Queue UI enhancements
   - Grouped by status
   - Filter/sort options
   - Batch operations
   - Visual priority indicators

**Deliverables**:
- Advanced queue service
- Queue management UI
- Persistence layer
- Monitoring dashboard

**Success Criteria**:
- [ ] User can select multiple files/folders
- [ ] Folder structure is preserved on upload
- [ ] Metadata can be applied in bulk
- [ ] CSV metadata import works
- [ ] Metadata templates can be created and reused
- [ ] EXIF/ID3 metadata auto-extracted
- [ ] Upload queue can be reordered
- [ ] Uploads can be scheduled
- [ ] Queue persists across app restarts

---

### Phase 10: Content Creation Tools (Q3 2026) - 6-8 weeks

#### Week 1-2: In-App Media Capture
**Goal**: Capture photos/videos directly in app for upload

**Tasks**:
1. Camera integration
   - Photo capture
   - Video recording
   - Camera controls (flash, flip, etc.)
   - Preview before save

2. Audio recording
   - Microphone access
   - Recording controls
   - Audio format selection
   - Waveform visualization

3. Media library
   - Store captured media locally
   - Organize by date/type
   - Delete/edit options
   - Preview media

**Deliverables**:
- Camera screen with capture
- Audio recorder screen
- Media library screen
- Captured media storage service

#### Week 2-3: Document Scanning & OCR
**Goal**: Scan physical documents for digital archiving

**Tasks**:
1. Document scanner
   - Camera-based scanning
   - Edge detection
   - Perspective correction
   - Multi-page scanning

2. OCR integration
   - Text extraction from images
   - Multiple language support
   - Searchable PDF generation
   - Text overlay preservation

3. Document processor
   - Enhance image quality
   - Remove shadows/noise
   - Adjust contrast
   - Generate clean output

**Deliverables**:
- Document scanner UI
- OCR service
- Image enhancement processor
- Scanned document library

#### Week 4-5: Metadata Extraction & Auto-Tagging
**Goal**: Intelligent metadata extraction and suggestions

**Tasks**:
1. Metadata extractors
   - Image EXIF (location, camera, date)
   - Audio ID3 (artist, album, year)
   - Video metadata (duration, codec, resolution)
   - Document properties (author, title)

2. AI auto-tagging
   - Image recognition (objects, scenes)
   - Audio genre detection
   - Video content analysis
   - Suggest relevant tags

3. Metadata enrichment
   - Lookup external databases
   - Geographic data from GPS
   - Timestamp formatting
   - Auto-generate descriptions

**Deliverables**:
- Metadata extraction library
- AI tagging service (integration with ML Kit or similar)
- Metadata enrichment pipeline
- Auto-tag UI with suggestions

#### Week 6-8: Collection Builder
**Goal**: Create and organize collections for new uploads

**Tasks**:
1. Collection creator
   - Name and describe collection
   - Add multiple items
   - Reorder items
   - Preview collection structure

2. Collection templates
   - Pre-made templates (photo album, music album, book series)
   - Template customization
   - Template library

3. Collection preview
   - Visual layout preview
   - Metadata completeness check
   - Size/quota estimation
   - Validation before upload

4. Collection upload
   - Upload entire collection
   - Maintain relationships
   - Progress tracking
   - Error handling

**Deliverables**:
- Collection builder UI
- Collection template system
- Collection preview screen
- Collection upload service

**Success Criteria**:
- [ ] User can capture photos/videos in-app
- [ ] User can record audio
- [ ] Documents can be scanned with OCR
- [ ] Metadata auto-extracted from media
- [ ] AI suggests relevant tags
- [ ] Collections can be created and organized
- [ ] Collection templates available
- [ ] Entire collections can be uploaded

---

### Phase 11: Upload Management (Q4 2026) - 6-8 weeks

#### Week 1-2: Draft Items System
**Goal**: Save items as drafts before publishing

**Tasks**:
1. Draft storage
   - Local database for drafts
   - Draft metadata
   - Draft file references
   - Draft state management

2. Draft editor
   - Edit draft metadata
   - Add/remove files
   - Preview draft
   - Validation checks

3. Draft management
   - List all drafts
   - Search/filter drafts
   - Delete drafts
   - Publish drafts

**Deliverables**:
- Draft storage service
- Draft editor UI
- Draft management screen
- Draft-to-upload conversion

#### Week 3-4: Edit Existing Items
**Goal**: Modify metadata of already-uploaded items

**Tasks**:
1. Item retrieval
   - Fetch user's uploaded items
   - Display item list
   - Search user's items
   - Filter by date/type

2. Metadata editor
   - Load existing metadata
   - Edit all fields
   - Validation
   - Save changes

3. Update service
   - Update metadata via API
   - Handle version conflicts
   - Verify changes applied
   - Error recovery

**Deliverables**:
- Item retrieval service
- Metadata edit screen
- Update API integration
- Edit history tracking

#### Week 5-6: Upload Analytics
**Goal**: Provide insights into uploaded content

**Tasks**:
1. Analytics dashboard
   - Total uploads
   - Storage used
   - Views/downloads per item
   - Popular items

2. Item-level analytics
   - View count over time
   - Download statistics
   - Geographic distribution
   - Referral sources

3. Trend analysis
   - Upload trends
   - Popular tags
   - Best-performing content
   - Growth metrics

**Deliverables**:
- Analytics dashboard UI
- Analytics API integration
- Charts and visualizations
- Analytics data caching

#### Week 7-8: Version Control & Moderation
**Goal**: Manage item versions and moderate content

**Tasks**:
1. Version management
   - Upload new versions of items
   - View version history
   - Revert to previous versions
   - Compare versions

2. Moderation tools
   - Flag inappropriate content
   - Appeal flags
   - Visibility controls
   - Privacy settings

3. Item health monitoring
   - Check for broken files
   - Verify checksums
   - Monitor access restrictions
   - Health reports

**Deliverables**:
- Version management service
- Version history UI
- Moderation tools UI
- Health monitoring dashboard

**Success Criteria**:
- [ ] User can save items as drafts
- [ ] Drafts can be edited before publishing
- [ ] User can edit existing uploaded items
- [ ] Analytics show views/downloads for uploads
- [ ] Upload trends are visualized
- [ ] New versions of items can be uploaded
- [ ] Content moderation tools available
- [ ] Item health can be monitored

---

## 🎨 UI/UX Design Principles for Upload Features

### Design Philosophy
1. **Mirror Download UX**: Upload UI should feel like "reverse download"
2. **Progressive Disclosure**: Start simple, reveal complexity as needed
3. **Offline-First**: Draft items work offline, upload when connected
4. **Visual Feedback**: Clear progress, status, and outcome
5. **Error Recovery**: Always provide path forward from errors

### Key UI Patterns

#### Upload Flow Pattern
```
Select Files → Add Metadata → Review → Upload → Track → Complete
     ↓             ↓            ↓        ↓        ↓        ↓
  [Picker]    [Form/AI]    [Preview]  [Queue]  [Progress] [Success]
```

#### Queue Management Pattern
```
┌─────────────────────────────────────┐
│ Upload Queue (3 active, 5 pending)  │
├─────────────────────────────────────┤
│ 📤 Uploading: vacation_photos/      │
│    └─ [████████░░░░] 67% • 2.1MB/s  │
│                                      │
│ 📤 Uploading: concert_audio.mp3     │
│    └─ [█████░░░░░░░] 45% • 1.5MB/s  │
│                                      │
│ 📤 Uploading: document_scan.pdf     │
│    └─ [██████████░░] 89% • 800KB/s  │
│                                      │
│ ⏸  Paused: old_movies/              │
│ ⏱  Scheduled: book_scans/ (tonight) │
│ ⏱  Pending: family_reunion/         │
│ ⏱  Pending: research_data/          │
│ ⏱  Pending: software_archive/       │
└─────────────────────────────────────┘
```

#### Metadata Entry Pattern
```
┌─────────────────────────────────────┐
│ Upload: 15 files selected            │
├─────────────────────────────────────┤
│                                      │
│ Title: *                             │
│ └─ [vacation_photos_2025        ]   │
│                                      │
│ Description:                         │
│ └─ [Photos from summer vacation ]   │
│     [to Hawaii               ]       │
│                                      │
│ Tags: vacation, hawaii, 2025         │
│ 💡 Suggested: beach, ocean, travel   │
│                                      │
│ Media Type: ● Images                 │
│                                      │
│ License: [Creative Commons BY v]    │
│                                      │
│ 🤖 AI Detected: Beach scenes, sunset │
│    📍 Location: Maui, Hawaii         │
│    📅 Date: July 15-22, 2025         │
│                                      │
│ [Apply to All] [Individual Metadata]│
└─────────────────────────────────────┘
```

### Mobile-First Considerations
- **Thumb-Friendly**: All actions reachable with thumb
- **Swipe Gestures**: Swipe to pause/cancel/retry
- **Haptic Feedback**: Tactile response for key actions
- **Landscape Support**: Support both orientations
- **Split-Screen**: Work with upload in background

---

## 🔧 Technical Considerations for Upload

### API Integration Challenges

#### 1. Internet Archive Upload API
**Current Knowledge** (needs verification):
- IA uses S3-compatible API for uploads
- Authentication via OAuth or API keys
- Metadata sent as JSON or form data
- Supports multipart uploads for large files
- Rate limiting and quotas apply

**Research Needed**:
- [ ] Official IA upload API documentation
- [ ] Authentication methods (OAuth vs API key)
- [ ] Metadata schema and requirements
- [ ] File size limits and restrictions
- [ ] Rate limiting specifics for uploads
- [ ] How to create collections via API
- [ ] Version control support
- [ ] Analytics API availability

#### 2. Large File Handling
**Challenges**:
- Mobile bandwidth constraints
- Limited device storage
- Background upload restrictions (iOS/Android)
- Progress persistence across app restarts

**Solutions**:
- Chunked uploads (5MB chunks)
- Resume capability with range headers
- Background tasks with WorkManager
- Local queue persistence
- Compression for text/documents
- Bandwidth throttling options

#### 3. Metadata Complexity
**Challenges**:
- IA has extensive metadata schema
- Different media types need different metadata
- Metadata validation requirements
- Handling of custom fields

**Solutions**:
- Template system for common patterns
- Dynamic forms based on media type
- Client-side validation
- Graceful handling of optional fields
- AI-assisted metadata generation

### Platform-Specific Issues

#### iOS
- Background upload limitations
- File access permissions (Photos, Documents)
- App Store review for camera/microphone
- Storage limitations

#### Android
- Scoped storage (Android 10+)
- Background service restrictions
- WorkManager battery optimization
- Various manufacturer limitations

### Performance Optimization

#### Upload Performance
- **Parallel uploads**: 2-3 concurrent (respect IA limits)
- **Compression**: GZIP for text, optional for media
- **Resumable uploads**: Save chunk progress
- **Bandwidth adaptation**: Detect and adapt to network speed

#### Storage Management
- **Queue size limits**: Max 100 pending uploads
- **Auto-cleanup**: Remove completed uploads after 7 days
- **Cache management**: Clear draft previews periodically
- **Quota warnings**: Alert when storage low

---

## 📚 Documentation Requirements

### User Documentation
1. **Upload Guide**
   - How to upload single file
   - How to upload folders
   - How to use metadata templates
   - How to manage upload queue
   - Troubleshooting common issues

2. **Metadata Guide**
   - Understanding IA metadata
   - Required vs optional fields
   - Choosing the right license
   - Using tags effectively
   - Collections best practices

3. **Content Creation Guide**
   - Using in-app camera
   - Document scanning tips
   - Audio recording best practices
   - OCR accuracy tips

### Developer Documentation
1. **Upload Architecture**
   - Service layer design
   - Queue management system
   - Background worker implementation
   - State management approach

2. **API Integration**
   - IA upload API reference
   - Authentication flow
   - Error handling strategies
   - Rate limiting compliance

3. **Testing Guide**
   - Unit test patterns
   - Integration test scenarios
   - UI test automation
   - Performance testing

---

## 🎯 Success Metrics

### Download Phase (Year 1)
- **App Installs**: 10,000+ downloads
- **Active Users**: 1,000+ monthly active users
- **Content Downloaded**: 1TB+ total downloads
- **User Retention**: 40%+ after 30 days
- **App Rating**: 4.5+ stars
- **Crash-Free Rate**: 99.5%+

### Upload Phase (Year 2)
- **Upload Adoption**: 20%+ of users try upload
- **Content Uploaded**: 100GB+ user-generated content
- **Upload Success Rate**: 95%+ successful uploads
- **Upload Frequency**: 50%+ of uploaders return
- **Collection Creation**: 1,000+ collections created

### Ecosystem Phase (Year 3)
- **API Usage**: 100+ third-party integrations
- **Community Features**: 5,000+ public collections shared
- **Creator Adoption**: 500+ active content creators
- **Platform Impact**: Mentioned in IA blog/documentation

---

## 💡 Innovation Opportunities

### Unique Features (Not in other IA apps)
1. **AI-Powered Metadata**: Auto-tagging with ML Kit
2. **Smart Collections**: AI-suggested groupings
3. **Social Discovery**: Follow creators, share collections
4. **Content Remix**: Create derivative works with attribution
5. **Collaborative Archives**: Multi-user uploads
6. **Blockchain Verification**: Optional content verification
7. **Offline-First Everything**: Full offline capability
8. **Cross-Platform Sync**: Desktop + mobile
9. **Developer API**: Let others build on ia-get
10. **Archive Health Monitoring**: Verify file integrity over time

### Competitive Advantages
- **Best Mobile Experience**: Better than official IA app
- **Upload + Download**: One-stop archive management
- **Offline Capability**: Work anywhere
- **Beautiful UI**: Material 3 design
- **Open Source**: Community-driven development
- **Privacy-First**: No tracking, no ads
- **Fast**: Optimized for performance
- **Accessible**: WCAG compliance

---

## 🚨 Risks & Mitigation

### Technical Risks
1. **API Changes**: IA changes upload API
   - *Mitigation*: Abstract API layer, version pinning
   
2. **Rate Limiting**: Hit IA rate limits
   - *Mitigation*: Implement respectful defaults, user education
   
3. **Large File Handling**: OOM crashes on uploads
   - *Mitigation*: Chunked uploads, streaming, memory profiling
   
4. **Background Upload Reliability**: OS kills background tasks
   - *Mitigation*: Queue persistence, auto-resume, user notifications

### Product Risks
1. **Low Upload Adoption**: Users don't upload
   - *Mitigation*: Onboarding, tutorials, showcase examples
   
2. **Storage Costs**: Too much local storage used
   - *Mitigation*: Configurable limits, auto-cleanup
   
3. **Complexity**: Too many features confuse users
   - *Mitigation*: Progressive disclosure, simple defaults
   
4. **Competition**: Official IA app improves
   - *Mitigation*: Focus on unique features, community

### Legal/Compliance Risks
1. **Content Violations**: Users upload illegal content
   - *Mitigation*: Terms of service, reporting tools, moderation
   
2. **Copyright Issues**: Unlicensed uploads
   - *Mitigation*: License education, DMCA compliance
   
3. **Privacy Concerns**: Metadata leaks location/identity
   - *Mitigation*: Privacy controls, metadata stripping options
   
4. **Terms Compliance**: Violate IA terms
   - *Mitigation*: Careful API usage, rate limiting, user education

---

## 📝 Actionable Next Steps

### Immediate (This Week - Week 1)
1. ✅ ~~API Compliance Sprint~~ **COMPLETED!**
2. ✅ ~~Day 6 UI Integration~~ **COMPLETED!**
3. 🎯 **START Phase 4 Task 1: Favorites & Collections** (HIGH PRIORITY)
   - Day 1-2: Database schema + Favorites service (3-4 hours)
   - Day 3-4: Collections system (3-4 hours)
   - Day 5: UI components + testing (2-3 hours)

### Short Term (Weeks 2-3)
4. 🎯 **Phase 4 Task 2: Advanced Search** (HIGH PRIORITY)
   - Week 2: Search query builder + API integration (4-5 hours)
   - Week 2: Advanced search UI (3-4 hours)
   - Week 3: Search history & saved searches (2-3 hours)

### Medium Term (Weeks 4-6)
5. 🎯 **Phase 4 Task 3: Download Queue Management** (HIGH PRIORITY)
   - Week 4: Resume/recovery implementation (4-5 hours)
   - Week 5: Queue UI + smart scheduling (6-7 hours)
   - Week 6: Testing + optimization (2-3 hours)

6. 🎯 **Phase 4 Task 4-5: Settings & Statistics** (MEDIUM PRIORITY)
   - Week 6-7: Settings enhancements (6-8 hours)
   - Week 7-8: Statistics dashboard (4-6 hours)

### Long Term (Q1-Q2 2026)
7. 🎯 Phase 5: Power User Features
8. 🎯 Phase 6: Enterprise Features
9. 🎯 Research Upload API (prepare for Phase 8)

---

## 🎯 Success Metrics

### Phase 4 Completion Criteria

**Favorites & Collections**:
- [ ] Users can star/unstar archives
- [ ] Create and manage collections
- [ ] Add/remove items from collections
- [ ] View favorites in dedicated screen
- [ ] Collections persist across sessions

**Advanced Search**:
- [ ] Field-specific search works
- [ ] Date range filtering functional
- [ ] Mediatype filtering works
- [ ] Sort options implemented
- [ ] Search history saved
- [ ] Saved searches accessible

**Download Queue**:
- [ ] Downloads resume after interruption
- [ ] Progress persists across app restarts
- [ ] Queue reordering works
- [ ] Priority scheduling functional
- [ ] Network-aware downloading works

**Settings & Stats**:
- [ ] All settings persist correctly
- [ ] Theme changes apply immediately
- [ ] Download statistics accurate
- [ ] Usage insights generated
- [ ] Performance optimizations measurable

---

## 🎉 Conclusion & Vision

### Current Achievement 🏆

ia-get has evolved from a simple download tool to a **comprehensive Internet Archive client** with:
- ✅ 74 file format support
- ✅ Professional media playback
- ✅ Archive.org API compliance
- ✅ Advanced download management
- ✅ Excellent offline capabilities
- ✅ 115+ passing tests

### Immediate Focus (Phase 4)

**Goal**: Transform ia-get from a download tool into a **content management system** for Internet Archive materials.

**Key Differentiators**:
1. **Organization**: Favorites, collections, and smart categorization
2. **Discovery**: Advanced search with comprehensive filtering
3. **Reliability**: Resume downloads, smart queue management
4. **Personalization**: Rich settings and usage insights
5. **Performance**: Optimized caching and bandwidth management

### Future Vision (2026+)

**Year 1 Complete**: Best-in-class download and management experience  
**Year 2 Goal**: Add upload capabilities for bidirectional archive management  
**Year 3 Vision**: Become the platform for Internet Archive power users and creators

### Vision Statement

> **"ia-get empowers everyone to preserve and share human knowledge through an intuitive, powerful, and respectful mobile experience for the Internet Archive."**

### Strategic Positioning

- **Today**: Mobile-first Internet Archive download manager with excellent preview capabilities
- **6 Months**: Comprehensive archive management tool with favorites, collections, and advanced search
- **12 Months**: Feature-complete download experience with enterprise-grade reliability
- **24 Months**: Bidirectional platform supporting both download and upload workflows
- **36 Months**: Community-driven ecosystem for archive creators and curators

---

## 📊 Progress Dashboard

### Overall Progress
- ✅ **Phases 1-3**: Complete (Foundation + 74 formats)
- ✅ **API Compliance**: Complete (Rate limiting, throttling, retry logic)
- 🎯 **Phase 4**: Ready to start (0% complete)
- ⏳ **Phase 5-7**: Planned (future quarters)
- 📋 **Phase 8+**: Designed (upload capabilities)

### Feature Completion by Category
- **Core Downloads**: 95% (resume/recovery remaining)
- **Media Preview**: 100% (74 formats supported)
- **API Compliance**: 100% ✅
- **UI/UX Polish**: 85% (favorites, advanced search pending)
- **Performance**: 90% (excellent caching, room for optimization)
- **Testing**: 90% (115 tests, expanding with new features)

### Technical Health
- ✅ Test Coverage: Excellent (115 tests)
- ✅ Code Quality: Clean (flutter analyze: no issues)
- ✅ Architecture: Solid (clear separation of concerns)
- ✅ Documentation: Good (comprehensive roadmaps)
- ✅ API Compliance: Complete (respectful usage patterns)

---

## 🔄 Document Maintenance

**Document Status**: ✅ Up to date (October 6, 2025)  
**Next Review**: After Phase 4 Task 1 completion (Favorites & Collections)  
**Update Frequency**: After each major phase/task completion  
**Owner**: Development Team  
**Stakeholders**: Users, Internet Archive Community, Contributors

### Recent Updates
- **October 6, 2025**: Added Day 6 UI Integration completion
- **October 6, 2025**: Marked API Compliance as complete
- **October 6, 2025**: Added detailed Phase 4 implementation plans
- **October 6, 2025**: Updated success metrics and progress dashboard

### Next Document Updates
- After Task 1 (Favorites): Update completion status, add lessons learned
- After Task 2 (Search): Document search performance metrics
- After Task 3 (Queue): Document resume/recovery success rates
- After Phase 4: Create Phase 4 completion report, plan Phase 5 kickoff

---

**Ready to build!** 🚀 Let's make ia-get the best Internet Archive companion app!
