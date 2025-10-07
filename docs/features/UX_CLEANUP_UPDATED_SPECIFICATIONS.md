# UX Cleanup Sprint - Updated Specifications

**Date:** October 7, 2025  
**Status:** Ready to implement  
**Based on:** User feedback and architectural planning

## Key Updates from User Feedback

### ✅ Downloads Solution Approved
The recommended solution (unify on BackgroundDownloadService) has been approved. Will proceed with:
- Change `useBackground` default to `true`
- Add "See All" button to DownloadManagerWidget
- Deprecate DownloadProvider

### ✅ Deep Links Enhanced
Deep links will now handle THREE URL patterns:
1. **Custom scheme:** `ia-get://identifier/archive-name`
2. **Archive.org details:** `https://archive.org/details/identifier`
3. **Archive.org downloads:** `https://archive.org/download/identifier`

This makes the app a full handler for Internet Archive URLs!

**AndroidManifest.xml changes:**
```xml
<!-- Custom scheme -->
<intent-filter android:autoVerify="true">
    <data android:scheme="ia-get" android:host="identifier" />
</intent-filter>

<!-- Archive.org URLs -->
<intent-filter android:autoVerify="true">
    <data android:scheme="https" android:host="archive.org" android:pathPrefix="/details" />
    <data android:scheme="https" android:host="archive.org" android:pathPrefix="/download" />
</intent-filter>
```

### ✅ What's New Redesigned
**CHANGED:** No longer a dismissible banner in Settings.

**NEW APPROACH:**
- Integrate into welcome/onboarding flow
- Show ONCE on first launch of new version
- Version detection via SharedPreferences
- Reusable component for future releases

**Implementation:**
```dart
// In main.dart - check version
final lastVersion = prefs.getString('last_app_version');
if (lastVersion != currentVersion) {
  prefs.setBool('show_whats_new', true);
}

// In home_screen.dart - show dialog after init
if (showWhatsNew) {
  showDialog(context: context, builder: (_) => WhatsNewDialog());
}
```

### ✅ 4-Section Architecture Proposed
User proposed organizing app into **4 major conceptual sections**:

1. **📤 Upload Management** (Future - Phase 6+)
   - Upload files to Internet Archive
   - Upload queue and progress
   - Upload history

2. **💾 Local Management** (Phase 4 expansion)
   - Downloaded files
   - Favorites system
   - Collections/playlists
   - Pinned archives

3. **📥 Download Management** (Current - needs fixes)
   - Active downloads
   - Download queue
   - Completed downloads

4. **⚙️ App Management** (Current)
   - Settings
   - Help/About
   - Configuration

**Navigation Recommendation:** Hybrid approach
- Bottom Nav: `[🔍 Browse] [📥 Downloads] [💾 Local] [⚙️ More]`
- Drawer: Future features (Uploads, Analytics, Advanced Settings)

This is an EXCELLENT organizational model that provides:
- Clear mental model for users
- Scalability for future features
- Clean code organization
- Easy testing and maintenance

## Updated Implementation Priority

### Critical (Days 1-2) - UNCHANGED
1. Fix download progress updates
2. Unify two download screens
3. Add file opening capability

### High Priority (Day 4) - ENHANCED
4. **Fix deep links** - Now handles 3 URL patterns instead of 1

### Medium Priority (Day 4-5) - REDESIGNED
5. **Feature discoverability** - Now uses onboarding dialog instead of banner

### Low Priority (Day 5) - UNCHANGED
6. Add pinned archive indicators
7. Standardize title placement

## New Documents Created

1. **`docs/features/PRE_PHASE_4_UX_CLEANUP_SPRINT.md`** (Updated)
   - Enhanced deep link specification (3 URL patterns)
   - Redesigned What's New implementation (dialog vs banner)
   - Added 4-section architecture overview

2. **`docs/features/UX_CLEANUP_QUICK_REFERENCE.md`** (Updated)
   - Updated Issue #4 with archive.org URL handling
   - Updated Issue #5 with onboarding approach

3. **`docs/architecture/APP_ORGANIZATION_4_SECTION_MODEL.md`** (NEW - 20KB)
   - Comprehensive 4-section architecture document
   - Navigation design considerations
   - Information architecture
   - State management by section
   - Implementation roadmap
   - Benefits and challenges analysis

## Architecture Document Highlights

The new architecture document provides:

### Section Breakdown
- **Upload:** Future feature (Phase 6+), prepared architecture
- **Local:** Favorites, Collections, Downloads browser (Phase 4)
- **Download:** Current implementation, being fixed
- **Settings:** App configuration and management

### Navigation Evolution
```
Current (v1.6):
- Home with hidden downloads widget
- Scattered settings

Phase 1 (Post-cleanup):
[🔍 Browse] [📥 Downloads] [⚙️ Settings]

Phase 2 (Phase 4):
[🔍 Browse] [📥 Downloads] [💾 Local] [⚙️ Settings]

Phase 3 (Phase 6+):
[🔍 Browse] [📥 Downloads] [💾 Local] [⚙️ More]
+ Drawer with Uploads
```

### Code Organization
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
```

## What's Different from Original Plan

| Aspect | Original Plan | Updated Plan |
|--------|---------------|--------------|
| **Deep Links** | Custom scheme only | Custom scheme + archive.org URLs |
| **What's New** | Dismissible banner in Settings | One-time dialog on version update |
| **Architecture** | Ad-hoc organization | Planned 4-section model |
| **Navigation** | Unclear future path | Clear evolution: 3→4 tabs + drawer |
| **Upload Features** | Not discussed | Planned architecture (Phase 6+) |

## Benefits of Changes

### Deep Link Enhancement
- **User Value:** Users can click archive.org links and open in app
- **Discoverability:** App appears as option when viewing IA content
- **Market Position:** Becomes THE mobile client for Internet Archive

### Onboarding Dialog
- **Less Intrusive:** No permanent banner cluttering UI
- **Better UX:** Shows once, then stays hidden
- **Reusable:** Easy to add features in future versions
- **Cleaner Code:** Single dialog component vs persistent state

### 4-Section Model
- **Clear Structure:** Users know where everything is
- **Scalability:** Room for uploads without redesign
- **Maintainability:** Clean separation of concerns
- **Future-Proof:** Architecture supports years of growth

## Implementation Notes

### Deep Links Testing
```bash
# Custom scheme
adb shell am start -W -a android.intent.action.VIEW -d "ia-get://identifier/commute_test"

# Archive.org details
adb shell am start -W -a android.intent.action.VIEW -d "https://archive.org/details/commute_test"

# Archive.org download
adb shell am start -W -a android.intent.action.VIEW -d "https://archive.org/download/commute_test"
```

### Version Detection
```dart
// In pubspec.yaml
version: 1.6.0+10  // Semantic version + build number

// In code
import 'package:package_info_plus/package_info_plus.dart';

final packageInfo = await PackageInfo.fromPlatform();
final currentVersion = packageInfo.version; // "1.6.0"
```

### What's New Timing
- Trigger: App version changes (compare stored vs current)
- Frequency: Once per version
- Timing: After HomeScreen initialization (post-frame callback)
- Dismissal: Automatic flag clear after showing

## Phase 4 Integration

After UX cleanup, Phase 4 will implement:

1. **Week 1: Navigation Restructure**
   - Add bottom navigation bar
   - Separate into clear tabs

2. **Weeks 2-3: Local Management**
   - Favorites system
   - Collections
   - Downloads browser
   - Add 4th "Local" tab

3. **Week 4: Polish**
   - Analytics
   - Performance optimization
   - Documentation

The 4-section model will guide all Phase 4 development.

## Questions Addressed

### Q: "What's the best way to organize the app?"
**A:** 4-section model (Upload, Local, Download, App Management) with hybrid navigation (tabs + drawer).

### Q: "Should What's New be a persistent banner?"
**A:** No, integrate into onboarding, show once per version update.

### Q: "Should deep links only work for custom scheme?"
**A:** No, also handle archive.org URLs to become default IA mobile client.

## Next Steps

1. **Immediate:** Start implementing UX fixes (Issues 1-3)
2. **Day 4:** Implement enhanced deep links (3 URL patterns)
3. **Day 5:** Implement onboarding dialog for What's New
4. **Day 6-7:** Complete remaining fixes and test
5. **Post-Sprint:** Begin Phase 4 with 4-section model in mind

## Success Criteria (Updated)

### Must Have
- ✅ Download progress updates in real-time
- ✅ Single, unified downloads screen
- ✅ Downloaded files can be opened from app
- ✅ Deep links work for ia-get:// AND archive.org URLs
- ✅ All 115 tests passing

### Should Have
- ✅ What's New dialog on version updates (reusable component)
- ✅ Pinned archives have visual indicator
- ✅ Consistent UI across all screens

### Nice to Have
- ✅ Architecture document for 4-section model
- ✅ Clear roadmap for Phase 4+ navigation
- ✅ Plan for Upload features (Phase 6+)

---

## Conclusion

User feedback has significantly improved the UX cleanup plan:

1. **Deep links are more powerful** - Handle archive.org URLs
2. **What's New is less intrusive** - One-time dialog vs persistent banner
3. **Architecture is future-proof** - 4-section model guides all development

The changes maintain the same implementation timeline while delivering better long-term value. The 4-section model in particular provides years of structural guidance for the project.

**Ready to implement!** 🚀

---

**Documents Reference:**
- Main plan: `docs/features/PRE_PHASE_4_UX_CLEANUP_SPRINT.md`
- Quick reference: `docs/features/UX_CLEANUP_QUICK_REFERENCE.md`
- Architecture: `docs/architecture/APP_ORGANIZATION_4_SECTION_MODEL.md`
- This summary: `docs/features/UX_CLEANUP_UPDATED_SPECIFICATIONS.md`
