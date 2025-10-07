# App Organization: 4-Section Model

**Created:** October 7, 2025  
**Status:** ARCHITECTURAL PROPOSAL  
**Target:** Phase 4+ Implementation

## Overview

This document outlines the **4-Section Conceptual Model** for organizing the ia-get mobile app. This model provides clear boundaries between different areas of functionality and future-proofs the architecture for planned features.

---

## The 4 Sections

### 1. 📤 Upload Management
**Status:** Not yet implemented (planned for Phase 6+)  
**Purpose:** Manage uploading content TO Internet Archive

#### Features
- **Upload Queue**
  - Add files/folders to upload
  - Batch upload operations
  - Priority management for uploads
  
- **Active Uploads**
  - Real-time progress tracking
  - Pause/resume/cancel uploads
  - Upload speed limiting
  - Retry failed uploads

- **Upload History**
  - Previously uploaded items
  - Upload success/failure logs
  - Re-upload capability

- **Upload Settings**
  - Default metadata templates
  - Collection preferences
  - License selection
  - Privacy/access controls

#### Technical Considerations
- Similar architecture to download system
- Background upload service
- Progress tracking and notifications
- Internet Archive API authentication
- Metadata validation before upload
- Large file chunking/resumable uploads

---

### 2. 💾 Local Management
**Status:** Partially implemented, expanding in Phase 4  
**Purpose:** Manage content stored ON device

#### Current Features (v1.6)
- Downloaded files on device
- Pinned archives (quick access)
- Recently viewed history

#### Phase 4 Additions (Planned)
- **Favorites System**
  - Mark archives as favorites
  - Quick access to favorite items
  - Sync favorites across devices (future)

- **Collections/Playlists**
  - Create custom collections
  - Group related archives
  - Share collections
  - Import/export collections

- **Downloaded Files Management**
  - Browse all downloaded files
  - Sort by date, size, type
  - Search downloaded content
  - Bulk delete/organize
  - Storage usage overview

- **Offline Access**
  - Mark items for offline availability
  - Automatic download management
  - Cache management

#### Technical Architecture
```
LocalManagementService
├── FavoritesManager
│   ├── Add/remove favorites
│   ├── Sync to cloud (future)
│   └── Export/import
├── CollectionsManager
│   ├── CRUD operations
│   ├── Collection sharing
│   └── Collection metadata
├── DownloadedFilesManager
│   ├── File scanning
│   ├── Storage calculation
│   └── File organization
└── CacheManager
    ├── Cache policy
    ├── Automatic cleanup
    └── Storage limits
```

---

### 3. 📥 Download Management
**Status:** Currently implemented, needs UX fixes  
**Purpose:** Manage downloading content FROM Internet Archive

#### Current Features (v1.6)
- **Active Downloads**
  - Real-time progress tracking
  - Multiple concurrent downloads
  - Pause/resume/cancel
  - Priority management (High/Normal/Low)

- **Download Queue**
  - FIFO queue with priority
  - Max 3 concurrent downloads
  - Automatic queue processing

- **Completed Downloads**
  - Download history
  - File location tracking
  - Re-download capability

- **Download Settings**
  - Bandwidth throttling
  - Concurrent download limit
  - Save location preferences
  - Network type restrictions (WiFi only, etc.)

#### Current Issues (Being Fixed)
- Progress tracking not updating (Issue #1)
- Two download screens confusion (Issue #2)
- No file opening capability (Issue #3)

#### Technical Architecture
```
BackgroundDownloadService (PRIMARY)
├── Active downloads tracking
├── Queue management
├── Progress updates
├── Bandwidth throttling
├── Priority scheduling
└── Notification service

DownloadProvider (DEPRECATED)
└── Being phased out in favor of BackgroundDownloadService
```

---

### 4. ⚙️ App Management
**Status:** Currently implemented, needs improvements  
**Purpose:** App-wide settings, configuration, and information

#### Current Features (v1.6)
- **Settings**
  - Download configuration
    - Bandwidth limits
    - Concurrent downloads
    - Default save location
    - Network preferences
  - Appearance
    - Theme (light/dark/auto)
    - Font size
    - Language (future)
  - Notifications
    - Download complete alerts
    - Upload complete alerts (future)
    - System notifications

- **Help & Documentation**
  - User guide
  - FAQ
  - Keyboard shortcuts (desktop)
  - Tips & tricks

- **About**
  - Version information
  - Credits
  - Privacy policy
  - Open source licenses
  - Report bug
  - Feature requests

#### Planned Improvements
- Deep link handling (Issue #4)
- Feature discoverability (Issue #5)
- What's New on version updates
- Onboarding for first-time users
- In-app tutorials
- Advanced debug options (for developers)

---

## Navigation Design

### Current Navigation (v1.6)
```
Top App Bar
├── [Back] (when applicable)
├── Title
└── Actions
    ├── Help
    ├── Downloads
    └── Settings (in some screens)

Bottom Area
└── DownloadManagerWidget (when downloads active)
```

**Issues:**
- No clear section separation
- Downloads hidden in widget
- No quick access to local library
- Upload section doesn't exist yet

### Proposed Navigation: Hybrid Model (RECOMMENDED)

#### Phase 1: Immediate (Post-UX Cleanup)
Bottom Navigation Bar with 3 tabs:
```
┌─────────────────────────────────────────────┐
│ [🔍 Browse] [📥 Downloads] [⚙️ Settings]   │
└─────────────────────────────────────────────┘
```

**Browse Tab:**
- Search
- Recently viewed
- Pinned archives
- Featured/Popular (future)

**Downloads Tab:**
- Active downloads
- Download queue
- Completed downloads
- Download settings quick access

**Settings Tab:**
- All app settings
- Help & documentation
- About

#### Phase 2: Phase 4 Implementation
Add Local Library tab:
```
┌──────────────────────────────────────────────────────┐
│ [🔍 Browse] [📥 Downloads] [💾 Local] [⚙️ Settings] │
└──────────────────────────────────────────────────────┘
```

**Local Tab:**
- Favorites
- Collections
- Downloaded files
- Pinned archives (moved from Browse)

#### Phase 3: Upload Feature (Phase 6+)
Two options:

**Option A: 5-Tab Bottom Nav (if uploads are heavily used)**
```
┌─────────────────────────────────────────────────────────────┐
│ [🔍] [📤 Upload] [📥 Download] [💾 Local] [⚙️ Settings]   │
└─────────────────────────────────────────────────────────────┘
```

**Option B: Drawer + 4-Tab (RECOMMENDED for scalability)**
```
≡ Menu (Drawer)
├── 📤 Uploads
│   ├── Upload Queue
│   ├── Active Uploads
│   └── Upload History
├── 🌟 Featured (future)
├── 📊 Statistics (future)
└── 🔧 Advanced Settings

Bottom Nav:
┌──────────────────────────────────────────────────────────┐
│ [🔍 Browse] [📥 Downloads] [💾 Local] [⚙️ More]         │
└──────────────────────────────────────────────────────────┘
```

**Why Option B is better:**
- Mobile UX best practice: 3-5 tabs maximum
- Drawer allows unlimited future expansion
- Uploads may be less frequently used than downloads
- "More" tab can include Settings + advanced features
- Cleaner, less cluttered interface

---

## Information Architecture

### Current Screen Hierarchy
```
App Entry
├── HomeScreen (Browse/Search)
│   ├── ArchiveDetailScreen
│   │   ├── FilePreviewScreen
│   │   └── Download actions
│   ├── FiltersScreen
│   │   └── AdvancedFiltersScreen
│   └── HistoryScreen
├── DownloadScreen (accessed via button)
└── SettingsScreen
    └── HelpScreen
```

### Proposed Screen Hierarchy (4-Section Model)

```
App Entry (Bottom Nav with 4 tabs)

┌─────────────────────────────────────────┐
│ 1️⃣ BROWSE TAB                          │
├─────────────────────────────────────────┤
│ ├── HomeScreen (search)                 │
│ ├── ArchiveDetailScreen                 │
│ │   ├── FilePreviewScreen               │
│ │   └── ShareArchiveScreen              │
│ ├── FiltersScreen                       │
│ │   └── AdvancedFiltersScreen           │
│ └── SearchHistoryScreen                 │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│ 2️⃣ DOWNLOADS TAB                        │
├─────────────────────────────────────────┤
│ ├── DownloadsScreen (unified)           │
│ │   ├── Active Downloads section        │
│ │   ├── Download Queue section          │
│ │   └── Completed Downloads section     │
│ ├── DownloadDetailScreen (per file)     │
│ └── DownloadSettingsScreen (quick)      │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│ 3️⃣ LOCAL TAB                            │
├─────────────────────────────────────────┤
│ ├── LocalLibraryScreen                  │
│ │   ├── Favorites section               │
│ │   ├── Collections section             │
│ │   ├── Downloaded Files section        │
│ │   └── Pinned Archives section         │
│ ├── FavoritesScreen (filtered view)     │
│ ├── CollectionDetailScreen              │
│ │   └── CollectionEditorScreen          │
│ ├── DownloadedFilesScreen               │
│ │   └── FileDetailScreen                │
│ └── StorageManagementScreen             │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│ 4️⃣ SETTINGS TAB (or "More")             │
├─────────────────────────────────────────┤
│ ├── SettingsScreen                      │
│ │   ├── DownloadSettingsScreen          │
│ │   ├── AppearanceSettingsScreen        │
│ │   ├── NotificationSettingsScreen      │
│ │   └── StorageSettingsScreen           │
│ ├── HelpScreen                          │
│ │   ├── UserGuideScreen                 │
│ │   ├── FAQScreen                       │
│ │   └── TutorialScreen                  │
│ ├── AboutScreen                         │
│ │   ├── LicensesScreen                  │
│ │   └── PrivacyPolicyScreen             │
│ └── DebugScreen (developer mode)        │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│ DRAWER (Future - Phase 6+)              │
├─────────────────────────────────────────┤
│ ├── 📤 Uploads Section                  │
│ │   ├── UploadQueueScreen               │
│ │   ├── ActiveUploadsScreen             │
│ │   ├── UploadHistoryScreen             │
│ │   └── UploadSettingsScreen            │
│ ├── 🌟 Featured/Discover                │
│ ├── 📊 Statistics & Analytics           │
│ └── 🔧 Advanced Settings                │
└─────────────────────────────────────────┘
```

---

## State Management by Section

### 1. Upload Management (Future)
```dart
// UploadProvider
class UploadProvider extends ChangeNotifier {
  Map<String, UploadProgress> _activeUploads = {};
  Map<String, UploadProgress> _completedUploads = {};
  
  // Queue management
  List<String> _uploadQueue = [];
  
  // Upload operations
  Future<void> startUpload(File file, ArchiveMetadata metadata);
  void pauseUpload(String uploadId);
  void resumeUpload(String uploadId);
  void cancelUpload(String uploadId);
  
  // Progress tracking
  Stream<UploadProgress> uploadProgressStream(String uploadId);
}

// UploadService
class UploadService {
  // Internet Archive API integration
  Future<void> uploadToArchive(File file, ArchiveMetadata metadata);
  Future<void> validateMetadata(ArchiveMetadata metadata);
  Future<String> getUploadToken();
}
```

### 2. Local Management
```dart
// LocalLibraryProvider
class LocalLibraryProvider extends ChangeNotifier {
  List<Archive> _favorites = [];
  List<Collection> _collections = [];
  Map<String, DownloadedFile> _downloadedFiles = {};
  
  // Favorites
  void addFavorite(Archive archive);
  void removeFavorite(String archiveId);
  bool isFavorite(String archiveId);
  
  // Collections
  Collection createCollection(String name);
  void addToCollection(String collectionId, Archive archive);
  void removeFromCollection(String collectionId, String archiveId);
  
  // Downloaded files
  List<DownloadedFile> getDownloadedFiles();
  StorageInfo getStorageInfo();
}

// Database schema
// favorites_table: id, archive_id, added_date
// collections_table: id, name, description, created_date
// collection_items_table: collection_id, archive_id, order
// downloaded_files_table: id, file_path, archive_id, file_name, size, downloaded_date
```

### 3. Download Management (Current)
```dart
// BackgroundDownloadService (already implemented)
class BackgroundDownloadService extends ChangeNotifier {
  Map<String, DownloadProgress> _activeDownloads = {};
  Map<String, DownloadProgress> _completedDownloads = {};
  List<DownloadTask> _downloadQueue = [];
  
  Future<void> startDownload(DownloadTask task);
  void pauseDownload(String downloadId);
  void resumeDownload(String downloadId);
  void cancelDownload(String downloadId);
}
```

### 4. App Management (Current)
```dart
// SettingsProvider
class SettingsProvider extends ChangeNotifier {
  // Download settings
  int maxConcurrentDownloads = 3;
  double? bandwidthLimit;
  String defaultSaveLocation;
  
  // App settings
  ThemeMode themeMode = ThemeMode.system;
  bool notificationsEnabled = true;
  
  // Methods
  Future<void> updateSetting(String key, dynamic value);
}
```

---

## Implementation Roadmap

### Phase 1: UX Cleanup (Current Sprint)
- Fix download progress tracking
- Unify download screens
- Add file opening capability
- Fix deep links
- Improve feature discoverability
- **Result:** Stable 3-section app (Browse, Downloads, Settings)

### Phase 2: Navigation Restructure (Phase 4, Week 1)
- Implement bottom navigation bar
- Separate Browse, Downloads, Settings into distinct tabs
- Move DownloadManagerWidget content to Downloads tab
- **Result:** Clear 3-tab navigation

### Phase 3: Local Management (Phase 4, Weeks 2-3)
- Add Favorites system
- Implement Collections
- Create Downloaded Files browser
- Add 4th tab: Local Library
- **Result:** 4-tab navigation (Browse, Downloads, Local, Settings)

### Phase 4: Upload Preparation (Phase 5-6)
- Research Internet Archive upload API
- Design upload UI/UX
- Implement upload queue architecture
- Add to drawer (not tab yet)
- **Result:** 4 tabs + drawer with uploads

### Phase 5: Full 4-Section Implementation (Phase 7+)
- Mature upload features
- Advanced analytics
- Cloud sync
- Consider 5th tab for uploads OR keep in drawer
- **Result:** Complete 4-section model

---

## Benefits of 4-Section Model

### 1. Clear Mental Model
Users understand where to find features:
- Want to browse/search? → Browse section
- Want to download? → Downloads section
- Want to manage local content? → Local section
- Want to configure app? → Settings section
- Want to upload? → Uploads section (future)

### 2. Scalability
Each section can grow independently:
- Add upload variants (batch, resumable, scheduled)
- Add local management features (tagging, notes)
- Add download scheduling
- Add advanced settings

### 3. Code Organization
Mirrors user-facing structure:
```
lib/
├── screens/
│   ├── browse/
│   ├── downloads/
│   ├── local/
│   ├── settings/
│   └── uploads/ (future)
├── services/
│   ├── archive_service.dart
│   ├── download_service.dart
│   ├── local_library_service.dart
│   └── upload_service.dart (future)
└── providers/
    ├── browse_provider.dart
    ├── download_provider.dart
    ├── local_library_provider.dart
    └── upload_provider.dart (future)
```

### 4. Testing & Maintenance
- Each section can be tested independently
- Feature teams can own specific sections
- Easier to debug issues (confined to section)
- Clear separation of concerns

### 5. Future-Proofing
- Upload features have dedicated space
- Analytics/statistics can be added
- Social features (sharing, collaboration) can be added
- Plugin architecture possible (community extensions)

---

## Potential Challenges

### 1. Navigation Depth
**Problem:** Too many nested screens  
**Solution:** Use modal sheets for quick actions, keep hierarchy max 3 levels deep

### 2. Feature Discovery
**Problem:** Users don't know features exist  
**Solution:** Onboarding tutorials, What's New dialogs, tooltips, empty state CTAs

### 3. State Management Complexity
**Problem:** Multiple providers competing for resources  
**Solution:** Clear data flow, single source of truth, proper dependency injection

### 4. Performance
**Problem:** Multiple sections loading simultaneously  
**Solution:** Lazy loading, tab-based lifecycle, efficient database queries

---

## Recommendations

1. **Immediate (Post-UX Cleanup):**
   - Implement 3-tab bottom navigation
   - Clearly separate Browse, Downloads, Settings

2. **Phase 4:**
   - Add Local Library as 4th tab
   - Move Favorites, Collections, Downloads to Local tab

3. **Phase 6+:**
   - Add Uploads to drawer (test usage patterns)
   - Consider promoting to 5th tab if heavily used
   - Keep Settings in "More" tab with advanced features

4. **Long-term:**
   - Monitor analytics on section usage
   - Adjust navigation based on user behavior
   - Consider platform-specific navigation (iOS vs Android)

---

## Conclusion

The **4-Section Conceptual Model** provides a solid foundation for the ia-get app's future growth. By organizing features into Upload, Local, Download, and App Management sections, we create a clear, scalable, and maintainable architecture that aligns with user mental models and supports future feature development.

**Next Steps:**
1. Complete UX Cleanup Sprint
2. Implement 3-tab navigation (Browse, Downloads, Settings)
3. Add Local Library tab in Phase 4
4. Plan Upload section architecture for Phase 6+

---

**Document Version:** 1.0  
**Last Updated:** October 7, 2025  
**Status:** Approved for implementation planning
