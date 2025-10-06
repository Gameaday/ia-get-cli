# Phase 4: Core Features & Foundation

## Overview
Phase 4 focuses on completing core download features, implementing proper data persistence with shared_preferences, and ensuring all essential functionality from Phases 1-3 is production-ready.
### Week 1: Favorites & Search
1. Task 1: Favorites & Collections System (3-4 hours)
2. Task 2: Advanced Search & Filters (4-5 hours)

### Week 2: Downloads & Settings
3. Task 3: Download Queue + Resume + Retry (6-8 hours)
4. Task 4: Settings & Customization + shared_preferences (3-4 hours)

### Week 3: Testing & Documentation
5. Comprehensive testing (2-3 hours)
6. Documentation updates (1-2 hours)

**Total Estimated Effort**: 16-21 hours (revised, statistics moved to Phase 6) 📋 Planning  
**Created**: 2025-10-06  
**Target Completion**: Q1 2025 (3-4 weeks)

## Phase Goals
1. **Favorites & Collections**: Let users organize and bookmark their favorite archives
2. **Advanced Search**: Enhanced search with filters, sorting, and saved searches  
3. **Download Queue + Resume + Retry**: Complete download management with reliability
4. **Settings & Customization**: Theme customization, data usage controls, shared_preferences integration
5. **~~Statistics & Insights~~**: ❌ MOVED to Phase 6 (post-release feature)

---

## Task Breakdown

### Task 1: Favorites & Collections System 🌟
**Priority**: High  
**Estimated Effort**: 3-4 hours

#### Features
- **Favorites List**: Star/bookmark favorite archives for quick access
- **Collections**: Create custom collections to organize archives by topic/category
- **Persistent Storage**: Store favorites using local database (shared_preferences or sqflite)
- **Quick Access**: Dedicated favorites tab/screen in app
- **Sync Indication**: Show which favorites are cached offline

#### Technical Approach
```dart
// New service: lib/services/favorites_service.dart
class FavoritesService {
  Future<void> addFavorite(String identifier);
  Future<void> removeFavorite(String identifier);
  Future<List<String>> getFavorites();
  Future<bool> isFavorite(String identifier);
  
  // Collections
  Future<void> createCollection(String name);
  Future<void> addToCollection(String collectionId, String identifier);
  Future<List<Collection>> getCollections();
}

// New model: lib/models/collection.dart
class Collection {
  String id;
  String name;
  List<String> identifiers;
  DateTime createdAt;
}
```

#### UI Components
- Star button in archive detail screen
- Favorites tab in home screen navigation
- Collection management screen
- Add to collection dialog

---

### Task 2: Advanced Search & Filters 🔍
**Priority**: High  
**Estimated Effort**: 4-5 hours

#### Features
- **Search History**: Auto-save recent searches for quick re-search
- **Search Suggestions**: Show popular/recent searches as suggestions
- **Saved Searches**: Save complex filter combinations with custom names
- **Sort Options**: Sort by date, size, relevance, downloads, etc.
- **Category Browser**: Browse by Internet Archive categories/collections
- **Advanced Filters UI**: Better filter UI with chips, date pickers, file type selectors

#### Technical Approach
```dart
// Enhanced service: lib/services/search_service.dart
class SearchService {
  // Search history
  Future<void> saveSearch(String query, Map<String, dynamic> filters);
  Future<List<SearchHistory>> getSearchHistory({int limit = 10});
  
  // Saved searches
  Future<void> saveFavoriteSearch(String name, String query, Map<String, dynamic> filters);
  Future<List<SavedSearch>> getSavedSearches();
  
  // Suggestions
  Future<List<String>> getSearchSuggestions(String partial);
}

// New models
class SearchHistory {
  String query;
  Map<String, dynamic> filters;
  DateTime timestamp;
}

class SavedSearch {
  String id;
  String name;
  String query;
  Map<String, dynamic> filters;
  DateTime createdAt;
}
```

#### UI Components
- Search history dropdown in search bar
- Saved searches management screen
- Enhanced filter drawer/bottom sheet
- Quick filter chips (e.g., "Videos only", "Last 7 days")
- Sort selector dialog

---

### Task 3: Download Queue Management 📥
**Priority**: Medium  
**Estimated Effort**: 3-4 hours

#### Features
- **Queue Reordering**: Drag-and-drop to reorder downloads in queue
- **Priority System**: Set high/normal/low priority for downloads
- **Scheduled Downloads**: Schedule downloads for specific times (e.g., overnight)
- **Download Presets**: Save preferred download settings (quality, format, filters)
- **Retry Failed Downloads**: Auto-retry with exponential backoff
- **Bandwidth Limiting**: Set max download speed to avoid network congestion

#### Technical Approach
```dart
// Enhanced service: lib/services/download_queue_service.dart
class DownloadQueueService {
  Future<void> reorderQueue(String downloadId, int newPosition);
  Future<void> setPriority(String downloadId, DownloadPriority priority);
  Future<void> scheduleDownload(String identifier, DateTime scheduledTime);
  Future<void> retryFailed({int maxAttempts = 3});
  Future<void> setBandwidthLimit(int bytesPerSecond);
  
  // Presets
  Future<void> savePreset(String name, DownloadSettings settings);
  Future<List<DownloadPreset>> getPresets();
}

enum DownloadPriority { high, normal, low }

class DownloadPreset {
  String id;
  String name;
  DownloadSettings settings;
}
```

#### UI Components
- Reorderable download list with drag handles
- Priority badge/indicator on download items
- Schedule picker dialog
- Presets management screen
- Bandwidth limiter slider in settings

---

### Task 4: App Settings & Customization ⚙️
**Priority**: Medium  
**Estimated Effort**: 3-4 hours

#### Features
- **Theme Options**: Light/Dark/System auto theme
- **Color Schemes**: Multiple color themes (Blue, Green, Purple, etc.)
- **Download Settings**: Default download location, auto-download metadata
- **Cache Management**: Set cache size limits, auto-purge settings
- **Data Usage Controls**: Wi-Fi only downloads, low-data mode
- **Notification Preferences**: Customize notification sounds, vibration
- **Language Support**: Add localization support (prepare for future translations)

#### Technical Approach
```dart
// Enhanced service: lib/services/settings_service.dart
class SettingsService extends ChangeNotifier {
  // Theme
  ThemeMode themeMode;
  ColorScheme colorScheme;
  
  // Downloads
  String defaultDownloadPath;
  bool autoDownloadMetadata;
  bool wifiOnlyDownloads;
  
  // Cache
  int maxCacheSizeMB;
  int autoPurgeDays;
  
  // Notifications
  bool enableNotifications;
  String notificationSound;
  bool vibrate;
  
  Future<void> saveSettings();
  Future<void> loadSettings();
  Future<void> resetToDefaults();
}
```

#### UI Components
- Enhanced settings screen with sections
- Theme selector with preview
- Storage usage charts/indicators
- Data usage monitor
- Reset settings confirmation dialog

---

### ~~Task 5: Statistics & Insights~~ ❌ REMOVED
**Status**: Moved to Phase 6 (post-release feature)  
**Reason**: Not critical for initial Play Store release. Focus on core functionality, polish, and stability first.

---

## Dependencies

### New Packages
```yaml
dependencies:
  # For local database (favorites, collections)
  sqflite: ^2.3.0
  path_provider: ^2.1.1
  
  # For simple settings/preferences (CRITICAL)
  shared_preferences: ^2.2.2
  
  # For date/time pickers
  intl: ^0.19.0
  
  # For drag-and-drop reordering
  reorderable_grid_view: ^2.2.8
```

### Existing Packages to Leverage
- `shared_preferences`: For simple settings storage
- `provider`: For state management
- `http`: For API calls

---

## Implementation Order

### Week 1: Favorites & Search
1. Task 1: Favorites & Collections System (3-4 hours)
2. Task 2: Advanced Search & Filters (4-5 hours)

### Week 2: Downloads & Settings
3. Task 3: Download Queue Management (3-4 hours)
4. Task 4: App Settings & Customization (3-4 hours)

### Week 3: Statistics & Polish
5. Task 5: Statistics & Insights (2-3 hours)
6. Testing, bug fixes, and documentation (2-3 hours)

**Total Estimated Effort**: 16-21 hours (revised from 20-27h, removed statistics)

---

## Testing Strategy

### Unit Tests
- Test favorites service CRUD operations
- Test search history and saved searches
- Test download queue reordering and priority
- Test settings persistence

### Widget Tests
- Test favorites UI interaction
- Test search suggestions dropdown
- Test download reordering drag-and-drop
- Test theme switching

### Integration Tests
- Test end-to-end favorite archive workflow
- Test saved search execution
- Test scheduled download triggering
- Test statistics calculation accuracy

---

## Success Criteria

### Task 1 (Favorites)
- ✅ User can star/unstar archives
- ✅ User can create and manage collections
- ✅ Favorites persist across app restarts
- ✅ Favorites screen shows all bookmarked archives

### Task 2 (Search)
- ✅ Search history auto-saves and displays suggestions
- ✅ User can save complex searches with custom names
- ✅ Sort options work correctly (date, size, relevance)
- ✅ Advanced filters UI is intuitive and responsive

### Task 3 (Queue)
- ✅ User can reorder download queue via drag-and-drop
- ✅ Priority system affects download order
- ✅ Scheduled downloads execute at specified time
- ✅ Failed downloads auto-retry with backoff

### Task 4 (Settings)
- ✅ Theme changes apply immediately
- ✅ Wi-Fi only mode prevents cellular downloads
- ✅ Cache limits are enforced
- ✅ Settings persist across app restarts

### ~~Task 5 (Statistics)~~ - REMOVED
- Moved to Phase 6 (post-release enhancement)

---

## Known Challenges

### Technical Challenges
1. **Drag-and-Drop Performance**: Large download queues may have performance issues
   - Solution: Virtualize list, limit simultaneous animations
   
2. **Scheduled Downloads**: Requires background task scheduling
   - Solution: Use `workmanager` plugin for Android/iOS background tasks
   
3. **Chart Rendering**: Complex charts may impact performance on low-end devices
   - Solution: Lazy-load charts, provide simplified view option
   
4. **Database Performance**: Querying large history/favorites tables
   - Solution: Add indexes, implement pagination

### UX Challenges
1. **Feature Discoverability**: New features may be hidden
   - Solution: Add onboarding tour, in-app tips, tutorial screens
   
2. **Settings Complexity**: Too many settings can be overwhelming
   - Solution: Group settings logically, provide good defaults
   
3. **Storage Management**: Users may not understand cache usage
   - Solution: Add visual storage indicators, clear explanations

---

## Future Enhancements (Phase 5+)

These features are out of scope for Phase 4 but could be considered later:

### Social Features
- Share archives with friends
- Public collections/playlists
- Comment on archives

### Advanced Media Features
- Playlist creation for audio/video
- Annotations and notes on documents
- OCR for scanned documents

### Cloud Sync
- Sync favorites/collections across devices
- Cloud backup of settings
- Collaborative collections

### AI Features
- Smart recommendations based on usage
- Auto-categorization of archives
- Search result relevance learning

---

## Documentation Requirements

### User Documentation
- Update README with Phase 4 features
- Create user guide for favorites and collections
- Document search tips and advanced filters
- Explain download queue management

### Developer Documentation
- API documentation for new services
- Database schema documentation
- Testing guide for Phase 4 features
- Migration guide from Phase 3 to Phase 4

### Completion Documentation
- `phase-4-task-1-complete.md` (Favorites)
- `phase-4-task-2-complete.md` (Search)
- `phase-4-task-3-complete.md` (Queue)
- `phase-4-task-4-complete.md` (Settings)
- `phase-4-task-5-complete.md` (Statistics)
- `phase-4-complete.md` (Overall summary)

---

## Risk Assessment

### Low Risk ✅
- Favorites system (straightforward CRUD)
- Settings UI (standard patterns)
- Basic statistics (simple calculations)

### Medium Risk ⚠️
- Advanced search (complex filter logic)
- Download queue reordering (state management complexity)
- Chart rendering (performance concerns)

### High Risk 🔴
- Scheduled downloads (background tasks, platform-specific)
- Bandwidth limiting (network layer modification)
- Database migrations (data integrity)

---

## Questions for User

Before starting Phase 4 implementation, please confirm:

1. **Priority**: Are all 5 tasks desired, or should we focus on subset?
2. **Scope**: Any specific features to add/remove?
3. **Timeline**: What's the target completion date?
4. **Design**: Do you have UI mockups or design preferences?
5. **Localization**: Should we prepare for multi-language support now?

---

## Notes

- Phase 4 completes core download functionality from Phases 1-3
- All features designed to work offline-first
- Minimal new dependencies to keep app size manageable
- **shared_preferences integration is CRITICAL** - proper data persistence
- Focus on essential features for Play Store release
- Maintains consistency with existing Internet Archive branding
- Code follows Flutter best practices and project conventions
- Statistics moved to Phase 6 (post-release)

---

**Next Steps**: 
1. Get user approval on Phase 4 plan
2. Prioritize tasks based on user feedback
3. Begin implementation with Task 1 (Favorites)
4. Update main README with Phase 4 roadmap
5. Create initial task completion templates

**Contact**: Ready to begin implementation upon approval! 🚀
