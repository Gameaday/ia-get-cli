# UX Cleanup Quick Reference

**Sprint Goal:** Fix 7 critical UX issues before Phase 4  
**Estimated Time:** 5-7 days  
**Document:** See `PRE_PHASE_4_UX_CLEANUP_SPRINT.md` for detailed plans

---

## Quick Issue Summary

| # | Issue | Severity | Time | Status |
|---|-------|----------|------|--------|
| 1 | Download progress not updating | ⚠️ CRITICAL | 2-3h | 🔴 TODO |
| 2 | Two download screens confusion | ⚠️ CRITICAL | 3-4h | 🔴 TODO |
| 3 | No way to open downloaded files | ⚠️ CRITICAL | 2-3h | 🔴 TODO |
| 4 | Deep links not working | ⚠️ HIGH | 2-3h | 🔴 TODO |
| 5 | New settings not discoverable | ⚠️ MEDIUM | 2-3h | 🔴 TODO |
| 6 | No pinned archive indicator | ⚠️ LOW | 1-2h | 🔴 TODO |
| 7 | Inconsistent title placement | ⚠️ LOW | 2-3h | 🔴 TODO |

---

## Day 1-2: Critical Bugs (Must Fix First)

### Issue #1: Download Progress ⚠️ CRITICAL
**Problem:** Progress bar stuck at 0%, speed shows "-", frozen UI  
**Root Cause:** `notifyListeners()` not called or progress not tracked  
**Fix:** Add progress tracking in download loop, call `notifyListeners()` every 500ms  
**Files:** `background_download_service.dart`, `download_manager_widget.dart`

### Issue #2: Two Download Screens ⚠️ CRITICAL
**Problem:** Downloads button → empty screen, actual downloads elsewhere  
**Root Cause:** Two parallel systems (DownloadProvider vs BackgroundDownloadService)  
**Fix:** Change `useBackground` default to `true`, add "See All" button  
**Files:** `home_screen.dart`, `download_screen.dart`, `download_manager_widget.dart`

### Issue #3: Open Downloaded Files ⚠️ CRITICAL
**Problem:** Download completes, no way to access file  
**Root Cause:** No UI actions for completed downloads  
**Fix:** Add "Open" and "Show in Folder" buttons using `open_file` package  
**Files:** `download_screen.dart`, `notification_service.dart`

---

## Day 4: High Priority Features

### Issue #4: Deep Links ⚠️ HIGH
**Problem:** `ia-get://identifier/archive-name` links don't work, archive.org URLs not handled  
**Root Cause:** No intent filter or deep link handling  
**Fix:** Configure AndroidManifest.xml for both custom scheme AND archive.org URLs, create DeepLinkService  
**URLs to handle:**
- `ia-get://identifier/archive-name` (custom scheme)
- `https://archive.org/details/identifier` (details page)
- `https://archive.org/download/identifier` (download page)
**Files:** `AndroidManifest.xml`, `main.dart`, create `deep_link_service.dart`

---

## Day 4-5: Medium Priority Polish

### Issue #5: Settings Discoverability ⚠️ MEDIUM
**Problem:** Priority selector, bandwidth controls invisible  
**Root Cause:** No onboarding for new features  
**Fix:** Integrate "What's New" into welcome flow (show once on first launch), add tooltips, feature hints  
**Note:** Reusable component for future feature releases, NOT a persistent banner
**Files:** `main.dart`, create `whats_new_dialog.dart`, `settings_screen.dart`

---

## Day 5: Low Priority Polish

### Issue #6: Pinned Archive Indicator ⚠️ LOW
**Problem:** Can't tell which archives are pinned  
**Root Cause:** No visual indicator for pinned state  
**Fix:** Add amber badge, pinned section, styling  
**Files:** `archive_card_widget.dart`, `history_screen.dart`

### Issue #7: Title Consistency ⚠️ LOW
**Problem:** Some screens title in AppBar, some in body  
**Root Cause:** No UI style guide, inconsistent development  
**Fix:** Create StandardScreenLayout, refactor all screens  
**Files:** Create `standard_screen_layout.dart`, update all screens

---

## Implementation Checklist

### Day 1: Start Critical Bugs
- [ ] Issue #1: Add debug logging to `background_download_service.dart`
- [ ] Issue #1: Verify `notifyListeners()` called on progress updates
- [ ] Issue #1: Test with 10MB download
- [ ] Issue #1: Fix UI to read from correct source
- [ ] Issue #2: Change `useBackground` default to `true`
- [ ] Issue #2: Add "See All" button to DownloadManagerWidget

### Day 2: Finish Critical Bugs
- [ ] Issue #2: Test unified downloads screen
- [ ] Issue #3: Implement `_openDownloadedFile()` method
- [ ] Issue #3: Implement `_showInFolder()` method
- [ ] Issue #3: Add "Open" and "Show in Folder" buttons to UI
- [ ] Issue #3: Update notification with action buttons
- [ ] Run tests: `flutter test` (ensure 115 still passing)

### Day 3: Buffer / Catch-up
- [ ] Fix any issues from Day 1-2
- [ ] Manual testing of critical bugs
- [ ] Verify all 3 critical issues resolved

### Day 4: High Priority Features
- [ ] Issue #4: Add `uni_links` dependency
- [ ] Issue #4: Configure AndroidManifest.xml intent filter
- [ ] Issue #4: Create `deep_link_service.dart`
- [ ] Issue #4: Initialize in `main.dart`
- [ ] Issue #4: Test with ADB command
- [ ] Issue #5: Create "What's New" banner in settings

### Day 5: Medium & Low Priority
- [ ] Issue #5: Add tooltips to new settings
- [ ] Issue #5: Create `whats_new_screen.dart`
- [ ] Issue #6: Add pinned badges to archives
- [ ] Issue #6: Create pinned section in history
- [ ] Issue #7: Create `standard_screen_layout.dart`

### Day 6-7: Polish & Testing
- [ ] Issue #7: Refactor all screens to use StandardScreenLayout
- [ ] Comprehensive manual testing
- [ ] Run full test suite: `flutter test`
- [ ] Static analysis: `flutter analyze`
- [ ] Build release APK: `flutter build apk --release`
- [ ] Test on real device

### Day 7: Documentation
- [ ] Update CHANGELOG.md with v1.6.1
- [ ] Create completion summary document
- [ ] Update ROADMAP_ANALYSIS.md
- [ ] Ready for Phase 4!

---

## Quick Test Commands

```bash
# Run all tests
flutter test

# Static analysis
flutter analyze

# Build debug APK
flutter build apk --debug

# Build release APK
flutter build apk --release

# Install on device
adb install build/app/outputs/flutter-apk/app-release.apk

# Test deep link
adb shell am start -W -a android.intent.action.VIEW -d "ia-get://identifier/commute_test"

# View logs
adb logcat -s flutter
```

---

## Debug Commands

```dart
// Add to background_download_service.dart
void _debugProgress(String fileId) {
  print('[DEBUG] Progress for $fileId:');
  print('  progress: ${_activeDownloads[fileId]?.progress}');
  print('  bytes: ${_activeDownloads[fileId]?.bytesDownloaded}/${_activeDownloads[fileId]?.totalBytes}');
  print('  speed: ${_activeDownloads[fileId]?.speed}');
  print('  status: ${_activeDownloads[fileId]?.status}');
}
```

---

## Common Pitfalls

1. **Progress Updates:**
   - Forget to call `notifyListeners()` after updating state
   - Update wrong DownloadProgress instance (not in active map)
   - UI reads from stale cached object

2. **File Opening:**
   - Android 11+ requires storage permissions
   - File path may be null or invalid
   - `open_file` package doesn't handle all formats

3. **Deep Links:**
   - Intent filter must be in correct activity
   - App must handle cold start, warm start, hot link
   - Navigator key must be global

4. **Settings:**
   - SharedPreferences keys must be unique
   - Boolean defaults must match initial state
   - "What's New" banner must be dismissible

---

## Success Criteria

✅ **Must Have:**
- Download progress updates in real-time
- Single, unified downloads screen  
- Downloaded files can be opened
- Deep links work
- All 115 tests passing

✅ **Should Have:**
- New settings visible
- Pinned archives have indicator
- Consistent UI

---

## After Sprint Completion

1. User testing (1-2 days)
2. Release v1.6.1 (0.5 days)
3. Start Phase 4: Favorites & Collections

**See `PRE_PHASE_4_UX_CLEANUP_SPRINT.md` for full details.**
