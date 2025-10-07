# Pre-Phase 4 UX Cleanup Sprint

**Created:** 2025-01-XX  
**Status:** ACTIVE - BLOCKING PHASE 4  
**Estimated Time:** 5-7 hours implementation + 2-3 hours testing = ~1 week sprint

## Executive Summary

User testing revealed 7 critical UX issues that prevent effective use of the app. These must be fixed before implementing Phase 4 features (Favorites & Collections). This document provides detailed analysis and specific fix plans for each issue.

### Priority Classification
- **CRITICAL** (3 issues): Block basic functionality, must fix first
- **HIGH** (1 issue): Important feature completely broken
- **MEDIUM** (1 issue): New features invisible to users
- **LOW** (2 issues): Polish items for better UX

---

## Future App Architecture: 4-Section Model

Based on user feedback, the app should be organized into **4 major conceptual sections**:

### 1. 📤 Upload Management (Future - Phase 6+)
*Not yet implemented*
- Upload files to Internet Archive
- Manage upload queue
- Track upload progress
- Upload history
- Batch upload operations

### 2. 💾 Local Management (Partially implemented, Phase 4 expansion)
*Current features:*
- Downloaded files on device
- Pinned archives
- Recently viewed (History)

*Phase 4 additions:*
- Favorites system
- Collections/Playlists
- Local file organization
- Storage management
- Offline access

### 3. 📥 Download Management (Currently implemented)
*Existing features:*
- Active downloads
- Completed downloads
- Download queue
- Download settings (bandwidth, priority)
- Background download support

*Needs fixes:*
- Progress tracking (Issue #1)
- Unified screens (Issue #2)
- File opening (Issue #3)

### 4. ⚙️ App Management (Currently implemented)
*Existing features:*
- Settings/Preferences
- Help/Documentation
- About/Version info
- Theme/Appearance
- Download configuration

*Needs improvements:*
- Deep links (Issue #4)
- Feature discoverability (Issue #5)
- UI consistency (Issue #7)

### Navigation Design Considerations

**Option A: Bottom Navigation Bar**
```
[🔍 Browse] [📥 Downloads] [💾 Local] [⚙️ Settings]
```
- Pro: Quick access to all sections
- Con: Limited to 4-5 tabs maximum
- Con: Upload section would need to be added later

**Option B: Drawer Navigation**
```
≡ Menu
├── 🔍 Search & Browse (Home)
├── 📤 Uploads (Future)
│   ├── Active Uploads
│   ├── Upload Queue
│   └── Upload History
├── 💾 Local Library
│   ├── Downloaded Files
│   ├── Favorites
│   ├── Collections
│   └── Pinned Archives
├── 📥 Downloads
│   ├── Active Downloads
│   ├── Download Queue
│   └── Completed Downloads
└── ⚙️ Settings
    ├── Download Settings
    ├── Upload Settings (Future)
    ├── Storage Settings
    └── About/Help
```
- Pro: Scales well with future features
- Pro: Clear hierarchy and organization
- Con: Requires extra tap to access sections

**Option C: Hybrid Approach (RECOMMENDED)**
- Bottom Nav: `[🔍 Browse] [📥 Downloads] [💾 Local] [⚙️ More]`
- Drawer: Advanced features, future Upload section, Collections
- Pro: Best of both worlds
- Pro: Room for growth (Upload can be added to drawer first, moved to tab later)
- Pro: Most common actions (Browse, Downloads, Local) always accessible

**Recommendation for Phase 4+:**
Implement Option C (Hybrid). This allows the app to grow naturally:
1. **Current (v1.6):** Browse, Downloads, Settings (3 tabs + drawer)
2. **Phase 4:** Browse, Downloads, Local, Settings (4 tabs)
3. **Phase 6+:** Browse, Downloads, Local, More (Uploads in drawer or 5th tab)

This 4-section model future-proofs the architecture and provides clear conceptual boundaries for features.

---

## Architecture Understanding

### Current Download System Architecture

The app has **TWO PARALLEL DOWNLOAD SYSTEMS** that are not properly unified:

1. **DownloadProvider** (legacy system)
   - Located: `lib/providers/download_provider.dart`
   - Used by: `DownloadScreen` (when `useBackground = false`)
   - Purpose: Original download implementation

2. **BackgroundDownloadService** (new system)
   - Located: `lib/services/background_download_service.dart`
   - Used by: `DownloadManagerWidget` (bottom widget on home screen)
   - Purpose: Enhanced download with background support, prioritization, API compliance

### Navigation Flow

```
HomeScreen (main entry)
├── Top App Bar
│   ├── Help Button → HelpScreen
│   └── Downloads Button → DownloadScreen(useBackground=false) [EMPTY!]
│
└── Bottom Widget Area
    └── DownloadManagerWidget → Shows BackgroundDownloadService downloads
        └── Only visible when hasActiveDownloads = true
```

**THE PROBLEM:** 
- User clicks "Downloads" button → Goes to empty DownloadScreen
- Actual downloads show in DownloadManagerWidget at bottom of home screen
- User doesn't realize downloads are happening in a different location
- No way to access completed downloads from UI

---

## Issue #1: Download Progress Not Updating ⚠️ CRITICAL

### Symptoms
- Progress bar frozen at 0%
- Speed shows "-" (never updates)
- Downloaded shows "0 B" (never updates)
- Session time may show app uptime instead of download time
- Downloads DO complete successfully (confirmed by user testing)

### Root Cause Analysis

**Hypothesis 1:** Event listener not properly attached to BackgroundDownloadService
- `DownloadManagerWidget` uses `Consumer<BackgroundDownloadService>`
- Should automatically rebuild when service calls `notifyListeners()`
- Need to verify `notifyListeners()` is called on progress updates

**Hypothesis 2:** Progress model fields not being updated
- `DownloadProgress` model has fields: `progress`, `bytesDownloaded`, `totalBytes`, `speed`
- Need to trace where these are set during download

**Hypothesis 3:** UI reads from wrong source
- Widget may be reading from stale/cached progress object
- Need to ensure UI always reads from latest service state

### Files to Investigate

1. `lib/services/background_download_service.dart` (lines 150-350)
   - Look for `_updateProgress()` method
   - Verify `notifyListeners()` is called after updates
   - Check if progress tracking is properly implemented

2. `lib/widgets/download_manager_widget.dart` (lines 200-400)
   - Verify progress display code reads from correct source
   - Check calculations for speed, progress percentage

3. `lib/models/download_progress.dart`
   - Verify all fields are properly defined
   - Check if `copyWith()` method works correctly

### Fix Plan

**Step 1:** Add debug logging to trace progress updates
```dart
// In background_download_service.dart
void _updateDownloadProgress(String fileId, {
  double? progress,
  int? bytesDownloaded,
  int? totalBytes,
  double? speed,
}) {
  print('[DEBUG] Updating progress for $fileId: '
        'progress=$progress, bytes=$bytesDownloaded/$totalBytes, speed=$speed');
  
  final current = _activeDownloads[fileId];
  if (current == null) return;
  
  _activeDownloads[fileId] = current.copyWith(
    progress: progress,
    bytesDownloaded: bytesDownloaded,
    totalBytes: totalBytes,
    speed: speed,
    lastUpdated: DateTime.now(),
  );
  
  print('[DEBUG] After update: ${_activeDownloads[fileId]}');
  notifyListeners(); // CRITICAL: Must be called!
}
```

**Step 2:** Verify download loop updates progress regularly
```dart
// In background_download_service.dart
Future<void> _downloadFile(DownloadProgress progress) async {
  // ... download setup ...
  
  int bytesDownloaded = 0;
  final stopwatch = Stopwatch()..start();
  
  await for (final chunk in response.stream) {
    bytesDownloaded += chunk.length;
    sink.add(chunk);
    
    // Update progress every 500ms or 1MB
    if (stopwatch.elapsedMilliseconds > 500 || bytesDownloaded % 1048576 == 0) {
      final elapsed = stopwatch.elapsedMilliseconds / 1000.0;
      final speed = elapsed > 0 ? bytesDownloaded / elapsed : 0.0;
      
      _updateDownloadProgress(
        progress.fileId,
        progress: bytesDownloaded / totalBytes,
        bytesDownloaded: bytesDownloaded,
        totalBytes: totalBytes,
        speed: speed,
      );
      
      stopwatch.reset();
      stopwatch.start();
    }
  }
}
```

**Step 3:** Fix UI to properly display progress
```dart
// In download_manager_widget.dart
Widget _buildProgressIndicator(DownloadProgress download) {
  final progress = (download.progress ?? 0.0).clamp(0.0, 1.0);
  final speed = download.speed ?? 0.0;
  final downloaded = download.bytesDownloaded ?? 0;
  final total = download.totalBytes ?? 0;
  
  print('[DEBUG UI] Rendering progress: $progress, speed: $speed, $downloaded/$total bytes');
  
  return Column(
    children: [
      LinearProgressIndicator(value: progress),
      Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text('${FileUtils.formatBytes(downloaded)} / ${FileUtils.formatBytes(total)}'),
          Text(speed > 0 ? '${FileUtils.formatSpeed(speed)}' : '-'),
        ],
      ),
    ],
  );
}
```

**Step 4:** Test with real download
- Start download of small file (e.g., 10MB)
- Watch debug console for progress updates
- Verify UI updates every 500ms
- Verify final state shows 100% completion

### Testing Checklist
- [ ] Debug logs show progress updates during download
- [ ] Progress bar moves smoothly from 0% to 100%
- [ ] Speed shows reasonable values (not "-" or "0")
- [ ] Downloaded/Total bytes update continuously
- [ ] Session time shows download duration (not app uptime)
- [ ] Multiple simultaneous downloads all update correctly
- [ ] Pausing and resuming preserves progress state

### Estimated Time: 2-3 hours

---

## Issue #2: Two Download Screens Confusion ⚠️ CRITICAL

### Symptoms
- Clicking "Downloads" button in app bar shows empty screen
- In-progress downloads only visible in `DownloadManagerWidget` at bottom of home screen
- User doesn't know where to find their downloads
- No intuitive way to see download history

### Root Cause

**Architecture Problem:** Two separate download systems not unified:

1. **DownloadScreen** (`useBackground=false`)
   - Navigated to via app bar button
   - Uses `DownloadProvider`
   - Shows "No downloads yet" because provider is empty
   - Location: `lib/screens/download_screen.dart`

2. **DownloadManagerWidget**
   - Fixed widget at bottom of HomeScreen
   - Uses `BackgroundDownloadService`
   - Actually shows real downloads
   - Only visible when `hasActiveDownloads = true`
   - Collapses when no active downloads

### Fix Plan

**Option 1: Use Background Service Everywhere (RECOMMENDED)**

Remove the dual-mode architecture and consolidate on `BackgroundDownloadService`:

**Step 1:** Update HomeScreen navigation to use background service
```dart
// In home_screen.dart line 125
IconButton(
  icon: const Icon(Icons.download_rounded),
  onPressed: () {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (_) => const DownloadScreen(useBackground: true), // Changed!
        settings: const RouteSettings(name: DownloadScreen.routeName),
      ),
    );
  },
),
```

**Step 2:** Deprecate DownloadProvider mode
```dart
// In download_screen.dart
class DownloadScreen extends StatefulWidget {
  const DownloadScreen({super.key, this.useBackground = true}); // Changed default!

  /// When true, show downloads from BackgroundDownloadService (REQUIRED)
  /// Legacy DownloadProvider support maintained for backwards compatibility only
  @deprecated('DownloadProvider mode is deprecated. Use BackgroundDownloadService.')
  final bool useBackground;
```

**Step 3:** Add "See All Downloads" navigation to DownloadManagerWidget
```dart
// In download_manager_widget.dart _buildHeader()
Widget _buildHeaderActions(...) {
  return Row(
    mainAxisSize: MainAxisSize.min,
    children: [
      // NEW: "See All" button
      TextButton.icon(
        icon: const Icon(Icons.open_in_full, size: 16),
        label: const Text('See All'),
        onPressed: () {
          Navigator.push(
            context,
            MaterialPageRoute(
              builder: (_) => const DownloadScreen(useBackground: true),
            ),
          );
        },
      ),
      // Existing minimize button...
    ],
  );
}
```

**Step 4:** Enhance DownloadScreen to show completed downloads with actions
```dart
// In download_screen.dart _buildBackgroundView()
Widget _buildCompletedSection(BackgroundDownloadService service) {
  return Column(
    children: [
      const Text('Completed Downloads'),
      ...service.completedDownloads.values.map((progress) {
        return Card(
          child: ListTile(
            leading: Icon(Icons.check_circle, color: Colors.green),
            title: Text(progress.fileName),
            subtitle: Text('${FileUtils.formatBytes(progress.totalBytes!)} • ${progress.completedAt}'),
            trailing: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                // NEW: Open file button
                IconButton(
                  icon: Icon(Icons.open_in_new),
                  onPressed: () => _openDownloadedFile(progress),
                ),
                // NEW: Show in folder button  
                IconButton(
                  icon: Icon(Icons.folder_open),
                  onPressed: () => _showInFolder(progress),
                ),
                // Existing delete button
                IconButton(
                  icon: Icon(Icons.delete_outline),
                  onPressed: () => service.removeCompletedDownload(progress.fileId),
                ),
              ],
            ),
          ),
        );
      }),
    ],
  );
}
```

**Option 2: Merge Systems (MORE WORK, NOT RECOMMENDED)**

Completely remove `DownloadProvider` and migrate all functionality to `BackgroundDownloadService`. This would require:
- Removing `lib/providers/download_provider.dart`
- Updating all references throughout codebase
- Extensive testing to ensure no regressions

### Testing Checklist
- [ ] Clicking "Downloads" button shows actual downloads
- [ ] Active downloads visible in both home widget and full screen
- [ ] Completed downloads accessible from downloads screen
- [ ] "See All" button in home widget navigates correctly
- [ ] No confusion about where downloads are
- [ ] History of completed downloads persists across app restarts

### Estimated Time: 3-4 hours

---

## Issue #3: No Way to Open Downloaded Files ⚠️ CRITICAL

### Symptoms
- Download completes successfully
- No visual indicator in app that file is ready
- No button to open file
- No way to navigate to file location in system
- User must manually find file in device storage

### Root Cause

UI doesn't provide actions for completed downloads. Files are saved successfully but inaccessible from app.

### Fix Plan

**Step 1:** Add file opening capability

Already have dependency in `pubspec.yaml`:
```yaml
dependencies:
  open_file: ^3.3.2  # Already installed!
```

**Step 2:** Implement file opening methods
```dart
// In download_screen.dart
import 'package:open_file/open_file.dart';
import 'package:path/path.dart' as path;

Future<void> _openDownloadedFile(DownloadProgress progress) async {
  try {
    final filePath = progress.savePath;
    if (filePath == null || !File(filePath).existsSync()) {
      _showError('File not found: $filePath');
      return;
    }

    final result = await OpenFile.open(filePath);
    
    if (result.type != ResultType.done) {
      _showError('Could not open file: ${result.message}');
    }
  } catch (e) {
    _showError('Error opening file: $e');
  }
}

Future<void> _showInFolder(DownloadProgress progress) async {
  try {
    final filePath = progress.savePath;
    if (filePath == null || !File(filePath).existsSync()) {
      _showError('File not found: $filePath');
      return;
    }

    // On Android, open file manager at directory
    final directory = path.dirname(filePath);
    
    if (Platform.isAndroid) {
      // Use Android intent to show in file manager
      await OpenFile.open(directory);
    } else {
      // On other platforms, open directory
      await OpenFile.open(directory);
    }
  } catch (e) {
    _showError('Error showing folder: $e');
  }
}

void _showError(String message) {
  ScaffoldMessenger.of(context).showSnackBar(
    SnackBar(
      content: Text(message),
      backgroundColor: Colors.red,
    ),
  );
}
```

**Step 3:** Add completion notification with action buttons
```dart
// In background_download_service.dart
Future<void> _onDownloadComplete(DownloadProgress progress) async {
  // Mark as complete
  _completedDownloads[progress.fileId] = progress.copyWith(
    status: DownloadStatus.complete,
    completedAt: DateTime.now(),
    progress: 1.0,
  );
  
  _activeDownloads.remove(progress.fileId);
  notifyListeners();
  
  // Show notification with action buttons
  await NotificationService.showDownloadComplete(
    fileName: progress.fileName,
    filePath: progress.savePath!,
    actions: [
      NotificationAction(
        id: 'open',
        title: 'Open',
        onPressed: () => OpenFile.open(progress.savePath!),
      ),
      NotificationAction(
        id: 'show_in_folder',
        title: 'Show in Folder',
        onPressed: () => OpenFile.open(path.dirname(progress.savePath!)),
      ),
    ],
  );
}
```

**Step 4:** Update DownloadManagerWidget to show completion state
```dart
// In download_manager_widget.dart
Widget _buildDownloadItem(DownloadProgress download) {
  final isComplete = download.status == DownloadStatus.complete;
  
  return Card(
    color: isComplete ? Colors.green.shade50 : null,
    child: ListTile(
      leading: Icon(
        isComplete ? Icons.check_circle : Icons.downloading,
        color: isComplete ? Colors.green : Colors.blue,
      ),
      title: Text(download.fileName),
      subtitle: isComplete
          ? Row(
              children: [
                Icon(Icons.done, size: 16, color: Colors.green),
                SizedBox(width: 4),
                Text('Download complete • ${FileUtils.formatBytes(download.totalBytes!)}'),
              ],
            )
          : _buildProgressSubtitle(download),
      trailing: isComplete
          ? Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                IconButton(
                  icon: Icon(Icons.open_in_new),
                  tooltip: 'Open file',
                  onPressed: () => _openFile(download.savePath!),
                ),
                IconButton(
                  icon: Icon(Icons.folder_open),
                  tooltip: 'Show in folder',
                  onPressed: () => _showInFolder(download.savePath!),
                ),
              ],
            )
          : _buildDownloadActions(download),
    ),
  );
}
```

**Step 5:** Add visual feedback for completion
```dart
// Add animation when download completes
void _onDownloadStatusChanged(DownloadProgress progress) {
  if (progress.status == DownloadStatus.complete) {
    // Show brief celebratory animation
    setState(() {
      _justCompleted.add(progress.fileId);
    });
    
    // Clear after 3 seconds
    Future.delayed(Duration(seconds: 3), () {
      setState(() {
        _justCompleted.remove(progress.fileId);
      });
    });
  }
}
```

### Testing Checklist
- [ ] Completed download shows clear visual indicator (green checkmark)
- [ ] "Open" button successfully launches file in appropriate app
- [ ] "Show in Folder" opens file manager at download location
- [ ] Notification appears with action buttons when download completes
- [ ] Actions work from notification (not just in-app)
- [ ] Error messages shown if file not found or can't open
- [ ] Works for all file types (PDFs, images, videos, etc.)
- [ ] Handles permission errors gracefully (Android 11+ storage access)

### Estimated Time: 2-3 hours

---

## Issue #4: Deep Links Not Working ⚠️ HIGH

### Symptoms
- Feature completely non-functional
- Should allow opening app via `ia-get://identifier/archive-name` links
- Should also handle Internet Archive URLs:
  - `https://archive.org/download/identifier` → Navigate to archive detail/download page
  - `https://archive.org/details/identifier` → Navigate to archive detail page
- Currently does nothing when clicked

### Root Cause Analysis

Need to implement deep link handling for both custom scheme and archive.org URLs:

**Files to Check:**
1. `android/app/src/main/AndroidManifest.xml` - Intent filters
2. `lib/main.dart` - App initialization and route handling
3. Any deep link handling service/utility

### Investigation Plan

**Step 1:** Check if intent filters are configured
```xml
<!-- In AndroidManifest.xml -->
<intent-filter>
    <action android:name="android.intent.action.VIEW" />
    <category android:name="android.intent.category.DEFAULT" />
    <category android:name="android.intent.category.BROWSABLE" />
    <data
        android:scheme="ia-get"
        android:host="identifier" />
</intent-filter>
```

**Step 2:** Check if app handles incoming links
```dart
// In main.dart or dedicated service
import 'package:uni_links/uni_links.dart';

class DeepLinkService {
  static Future<void> initialize() async {
    // Handle app launch from link
    try {
      final initialLink = await getInitialLink();
      if (initialLink != null) {
        _handleDeepLink(initialLink);
      }
    } catch (e) {
      print('Error getting initial link: $e');
    }
    
    // Handle links while app is running
    linkStream.listen((String? link) {
      if (link != null) {
        _handleDeepLink(link);
      }
    });
  }
  
  static void _handleDeepLink(String link) {
    // Parse: ia-get://identifier/archive-name
    final uri = Uri.parse(link);
    if (uri.scheme == 'ia-get' && uri.host == 'identifier') {
      final identifier = uri.pathSegments.firstOrNull;
      if (identifier != null) {
        // Navigate to archive detail
        navigatorKey.currentState?.pushNamed(
          ArchiveDetailScreen.routeName,
          arguments: identifier,
        );
      }
    }
  }
}
```

### Fix Plan

**Step 1:** Add deep link dependency (if not present)
```yaml
# In pubspec.yaml
dependencies:
  uni_links: ^0.5.1
```

**Step 2:** Configure Android manifest for both custom scheme and archive.org URLs
```xml
<!-- In android/app/src/main/AndroidManifest.xml -->
<activity
    android:name=".MainActivity"
    android:launchMode="singleTask">
    <!-- Existing configuration -->
    
    <!-- Custom scheme deep link (ia-get://) -->
    <intent-filter android:autoVerify="true">
        <action android:name="android.intent.action.VIEW" />
        <category android:name="android.intent.category.DEFAULT" />
        <category android:name="android.intent.category.BROWSABLE" />
        <data
            android:scheme="ia-get"
            android:host="identifier" />
    </intent-filter>
    
    <!-- Archive.org URL handling (https://archive.org/...) -->
    <intent-filter android:autoVerify="true">
        <action android:name="android.intent.action.VIEW" />
        <category android:name="android.intent.category.DEFAULT" />
        <category android:name="android.intent.category.BROWSABLE" />
        <data
            android:scheme="https"
            android:host="archive.org"
            android:pathPrefix="/details" />
        <data
            android:scheme="https"
            android:host="archive.org"
            android:pathPrefix="/download" />
    </intent-filter>
</activity>
```

**Step 3:** Create deep link service
```dart
// Create: lib/services/deep_link_service.dart
import 'dart:async';
import 'package:uni_links/uni_links.dart';
import 'package:flutter/services.dart';

class DeepLinkService {
  static StreamSubscription? _linkSubscription;
  
  static Future<void> initialize(Function(String) onLinkReceived) async {
    // Handle initial link (app was closed)
    try {
      final initialLink = await getInitialLink();
      if (initialLink != null) {
        onLinkReceived(initialLink);
      }
    } on PlatformException {
      // Handle exception
    }
    
    // Handle links while app is running
    _linkSubscription = linkStream.listen(
      (String? link) {
        if (link != null) {
          onLinkReceived(link);
        }
      },
      onError: (err) {
        print('Deep link error: $err');
      },
    );
  }
  
  static void dispose() {
    _linkSubscription?.cancel();
  }
  
  static String? parseArchiveIdentifier(String link) {
    try {
      final uri = Uri.parse(link);
      
      // Handle custom scheme: ia-get://identifier/archive-name
      if (uri.scheme == 'ia-get' && uri.host == 'identifier') {
        return uri.pathSegments.firstOrNull;
      }
      
      // Handle archive.org URLs: https://archive.org/details/identifier
      // or https://archive.org/download/identifier
      if (uri.scheme == 'https' && uri.host == 'archive.org') {
        if (uri.pathSegments.length >= 2 && 
            (uri.pathSegments[0] == 'details' || uri.pathSegments[0] == 'download')) {
          return uri.pathSegments[1]; // The identifier
        }
      }
    } catch (e) {
      print('Error parsing deep link: $e');
    }
    return null;
  }
}
```

**Step 4:** Initialize in main.dart
```dart
// In main.dart
void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // Initialize services
  await NotificationService.initialize();
  
  // Initialize deep link handling
  DeepLinkService.initialize((link) {
    final identifier = DeepLinkService.parseArchiveIdentifier(link);
    if (identifier != null) {
      // Navigate to archive detail screen
      navigatorKey.currentState?.pushNamed(
        ArchiveDetailScreen.routeName,
        arguments: identifier,
      );
    }
  });
  
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      navigatorKey: navigatorKey, // Global navigator key
      routes: {
        '/': (context) => const HomeScreen(),
        ArchiveDetailScreen.routeName: (context) {
          final identifier = ModalRoute.of(context)!.settings.arguments as String;
          return ArchiveDetailScreen(identifier: identifier);
        },
        // ... other routes
      },
    );
  }
}

// Global navigator key
final GlobalKey<NavigatorState> navigatorKey = GlobalKey<NavigatorState>();
```

**Step 5:** Add testing utility
```dart
// Create test deep link widget
class DeepLinkTestWidget extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        ElevatedButton(
          child: Text('Test Deep Link: commute_test'),
          onPressed: () {
            DeepLinkService._handleDeepLink('ia-get://identifier/commute_test');
          },
        ),
      ],
    );
  }
}
```

### Testing Checklist
- [ ] Custom scheme works: `ia-get://identifier/archive-name`
- [ ] Archive.org details URL works: `https://archive.org/details/commute_test`
- [ ] Archive.org download URL works: `https://archive.org/download/commute_test`
- [ ] Correct archive detail screen is shown
- [ ] Works when app is closed (cold start)
- [ ] Works when app is in background (warm start)
- [ ] Works when app is already open (hot link)
- [ ] Invalid links handled gracefully (show error, don't crash)
- [ ] Test commands:
  - `adb shell am start -W -a android.intent.action.VIEW -d "ia-get://identifier/commute_test"`
  - `adb shell am start -W -a android.intent.action.VIEW -d "https://archive.org/details/commute_test"`
  - `adb shell am start -W -a android.intent.action.VIEW -d "https://archive.org/download/commute_test"`

### Estimated Time: 2-3 hours

---

## Issue #5: New Settings Not Discoverable ⚠️ MEDIUM

### Symptoms
- Priority selector and other new settings hidden
- Users don't know these features exist
- No onboarding or tooltips explaining new functionality

### Root Cause

New features added without UI discoverability:
- Priority selector (for download prioritization)
- Bandwidth controls (enhanced settings)
- Rate limit status indicator

### Fix Plan

**Step 1:** Integrate "What's New" into welcome/onboarding flow

Instead of a dismissible banner, integrate new feature announcements into the existing welcome flow. Show once on first launch of new version, reusable for future releases.
```dart
// In main.dart - Check version on app startup
void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // Check if this is first launch of new version
  final prefs = await SharedPreferences.getInstance();
  final lastVersion = prefs.getString('last_app_version');
  const currentVersion = '1.6.0'; // From pubspec.yaml
  
  if (lastVersion != currentVersion) {
    // Show what's new on first launch of new version
    prefs.setBool('show_whats_new', true);
    prefs.setString('last_app_version', currentVersion);
  }
  
  runApp(const MyApp());
}

// In home_screen.dart - Show what's new dialog after initialization
@override
void initState() {
  super.initState();
  
  WidgetsBinding.instance.addPostFrameCallback((_) async {
    final prefs = await SharedPreferences.getInstance();
    final showWhatsNew = prefs.getBool('show_whats_new') ?? false;
    
    if (showWhatsNew && mounted) {
      // Clear flag
      prefs.setBool('show_whats_new', false);
      
      // Show what's new dialog
      showDialog(
        context: context,
        builder: (context) => WhatsNewDialog(),
      );
    }
  });
}

// Create: lib/widgets/whats_new_dialog.dart
class WhatsNewDialog extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Row(
        children: [
          Icon(Icons.celebration, color: Colors.blue),
          SizedBox(width: 8),
          Text('What\'s New in v1.6'),
        ],
      ),
      content: SingleChildScrollView(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            _buildFeatureItem(
              icon: Icons.priority_high,
              title: 'Download Prioritization',
              description: 'Set priority for downloads: High, Normal, or Low',
            ),
            SizedBox(height: 12),
            _buildFeatureItem(
              icon: Icons.speed,
              title: 'Bandwidth Controls',
              description: 'Limit download speed to save data',
            ),
            SizedBox(height: 12),
            _buildFeatureItem(
              icon: Icons.timer,
              title: 'Rate Limit Indicator',
              description: 'See when API rate limiting is active',
            ),
            SizedBox(height: 16),
            Text(
              'Tap "Learn More" to see detailed explanations.',
              style: TextStyle(fontSize: 12, color: Colors.grey),
            ),
          ],
        ),
      ),
      actions: [
        TextButton(
          child: Text('Learn More'),
          onPressed: () {
            Navigator.pop(context);
            Navigator.push(
              context,
              MaterialPageRoute(builder: (_) => WhatsNewScreen()),
            );
          },
        ),
        ElevatedButton(
          child: Text('Got it!'),
          onPressed: () => Navigator.pop(context),
        ),
      ],
    );
  }

  Widget _buildFeatureItem({
    required IconData icon,
    required String title,
    required String description,
  }) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Icon(icon, color: Colors.blue, size: 24),
        SizedBox(width: 12),
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                title,
                style: TextStyle(
                  fontWeight: FontWeight.w600,
                  fontSize: 14,
                ),
              ),
              SizedBox(height: 4),
              Text(
                description,
                style: TextStyle(
                  fontSize: 12,
                  color: Colors.grey.shade700,
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }
}
```

**Step 2:** Add tooltips to new settings
```dart
// In settings_screen.dart _buildDownloadSettings()
ListTile(
  leading: Icon(Icons.priority_high),
  title: Row(
    children: [
      Text('Download Priority'),
      SizedBox(width: 8),
      Tooltip(
        message: 'Set priority for downloads. High priority downloads '
                'start first and get more bandwidth.',
        child: Icon(Icons.info_outline, size: 16, color: Colors.grey),
      ),
    ],
  ),
  subtitle: Text('Default: Normal'),
  trailing: DropdownButton<DownloadPriority>(
    value: _currentPriority,
    items: [
      DropdownMenuItem(value: DownloadPriority.high, child: Text('High')),
      DropdownMenuItem(value: DownloadPriority.normal, child: Text('Normal')),
      DropdownMenuItem(value: DownloadPriority.low, child: Text('Low')),
    ],
    onChanged: (value) {
      setState(() {
        _currentPriority = value!;
      });
    },
  ),
),
```

**Step 3:** Add feature discovery hints in archive detail screen
```dart
// In archive_detail_screen.dart
// Show hint on first download
void _startDownload(ArchiveFile file) async {
  // Check if user has seen priority hint
  final seenPriorityHint = prefs.getBool('seen_priority_hint') ?? false;
  
  if (!seenPriorityHint) {
    // Show dialog explaining priority feature
    final priority = await showDialog<DownloadPriority>(
      context: context,
      builder: (context) => AlertDialog(
        title: Row(
          children: [
            Icon(Icons.lightbulb, color: Colors.amber),
            SizedBox(width: 8),
            Text('New Feature: Priority'),
          ],
        ),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('You can now set priority for downloads!'),
            SizedBox(height: 12),
            Text('• High: Starts immediately, max speed'),
            Text('• Normal: Standard queue, balanced'),
            Text('• Low: Background, minimal bandwidth'),
            SizedBox(height: 12),
            Text('Change anytime in Settings.'),
          ],
        ),
        actions: [
          TextButton(
            child: Text('Got it'),
            onPressed: () {
              prefs.setBool('seen_priority_hint', true);
              Navigator.pop(context, DownloadPriority.normal);
            },
          ),
        ],
      ),
    );
  }
  
  // Start download with selected priority
  _downloadService.startDownload(file, priority: priority ?? DownloadPriority.normal);
}
```

**Step 4:** Create "What's New" screen
```dart
// Create: lib/screens/whats_new_screen.dart
class WhatsNewScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('What\'s New')),
      body: ListView(
        padding: EdgeInsets.all(16),
        children: [
          _buildFeatureCard(
            icon: Icons.priority_high,
            title: 'Download Prioritization',
            description: 'Control which downloads start first and how much '
                        'bandwidth they get. Perfect for managing multiple downloads.',
            example: 'Tap "Priority" when starting a download to choose High, Normal, or Low.',
          ),
          _buildFeatureCard(
            icon: Icons.speed,
            title: 'Bandwidth Throttling',
            description: 'Limit download speed to conserve data or prevent network congestion.',
            example: 'Go to Settings → Downloads → Bandwidth Limit to set a maximum speed.',
          ),
          _buildFeatureCard(
            icon: Icons.timer,
            title: 'Rate Limit Indicator',
            description: 'See when Internet Archive API rate limiting is active. '
                        'The app automatically handles this for you.',
            example: 'Look for the rate limit badge in the downloads screen.',
          ),
        ],
      ),
    );
  }

  Widget _buildFeatureCard({
    required IconData icon,
    required String title,
    required String description,
    required String example,
  }) {
    return Card(
      margin: EdgeInsets.only(bottom: 16),
      child: Padding(
        padding: EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(icon, size: 32, color: Colors.blue),
                SizedBox(width: 12),
                Expanded(
                  child: Text(
                    title,
                    style: TextStyle(
                      fontSize: 18,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
              ],
            ),
            SizedBox(height: 12),
            Text(description),
            SizedBox(height: 8),
            Container(
              padding: EdgeInsets.all(8),
              decoration: BoxDecoration(
                color: Colors.grey.shade100,
                borderRadius: BorderRadius.circular(8),
              ),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Icon(Icons.tips_and_updates, size: 16, color: Colors.blue),
                  SizedBox(width: 8),
                  Expanded(
                    child: Text(
                      example,
                      style: TextStyle(fontSize: 12, fontStyle: FontStyle.italic),
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
```

### Testing Checklist
- [ ] "What's New" banner appears on first launch of v1.6
- [ ] Banner can be dismissed and stays dismissed
- [ ] Tooltips appear when hovering/pressing info icons
- [ ] Feature discovery hint shows on first download
- [ ] "Learn More" button navigates to detailed explanation
- [ ] All new features are properly documented in UI
- [ ] Users can easily find priority selector
- [ ] Users can easily find bandwidth controls

### Estimated Time: 2-3 hours

---

## Issue #6: No Pinned Archive Visual Indicator ⚠️ LOW

### Symptoms
- Users can pin archives (feature exists)
- No visual difference between pinned and unpinned items
- Can't tell at a glance which archives are pinned

### Root Cause

Pinning functionality implemented but no UI indicator added.

### Fix Plan

**Step 1:** Add pinned badge to archive cards
```dart
// In archive_card_widget.dart (or wherever archives are displayed)
Widget _buildArchiveCard(Archive archive) {
  return Card(
    child: Stack(
      children: [
        // Existing card content
        ListTile(
          leading: _buildThumbnail(archive),
          title: Text(archive.title),
          subtitle: Text(archive.description),
          onTap: () => _navigateToDetail(archive),
        ),
        
        // NEW: Pinned indicator badge
        if (archive.isPinned)
          Positioned(
            top: 8,
            right: 8,
            child: Container(
              padding: EdgeInsets.symmetric(horizontal: 8, vertical: 4),
              decoration: BoxDecoration(
                color: Colors.amber,
                borderRadius: BorderRadius.circular(12),
                boxShadow: [
                  BoxShadow(
                    color: Colors.black26,
                    blurRadius: 4,
                    offset: Offset(0, 2),
                  ),
                ],
              ),
              child: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Icon(Icons.push_pin, size: 14, color: Colors.white),
                  SizedBox(width: 4),
                  Text(
                    'PINNED',
                    style: TextStyle(
                      fontSize: 10,
                      fontWeight: FontWeight.bold,
                      color: Colors.white,
                    ),
                  ),
                ],
              ),
            ),
          ),
      ],
    ),
  );
}
```

**Step 2:** Add pinned section in history/recent screens
```dart
// In history_screen.dart
Widget build(BuildContext context) {
  return ListView(
    children: [
      // NEW: Pinned archives section
      if (_pinnedArchives.isNotEmpty) ...[
        Padding(
          padding: EdgeInsets.all(16),
          child: Row(
            children: [
              Icon(Icons.push_pin, color: Colors.amber),
              SizedBox(width: 8),
              Text(
                'Pinned Archives',
                style: TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ],
          ),
        ),
        ..._pinnedArchives.map((archive) => _buildArchiveCard(archive)),
        Divider(height: 32, thickness: 2),
      ],
      
      // Existing recent archives
      Padding(
        padding: EdgeInsets.all(16),
        child: Text(
          'Recent Archives',
          style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
        ),
      ),
      ..._recentArchives.map((archive) => _buildArchiveCard(archive)),
    ],
  );
}
```

**Step 3:** Add pin/unpin animation
```dart
// Animate pin toggle
void _togglePin(Archive archive) {
  setState(() {
    archive.isPinned = !archive.isPinned;
  });
  
  // Save to database
  _archiveService.updateArchive(archive);
  
  // Show feedback
  ScaffoldMessenger.of(context).showSnackBar(
    SnackBar(
      content: Row(
        children: [
          Icon(
            archive.isPinned ? Icons.push_pin : Icons.push_pin_outlined,
            color: Colors.white,
          ),
          SizedBox(width: 8),
          Text(archive.isPinned ? 'Pinned' : 'Unpinned'),
        ],
      ),
      duration: Duration(seconds: 2),
      action: SnackBarAction(
        label: 'UNDO',
        onPressed: () => _togglePin(archive), // Undo
      ),
    ),
  );
}
```

**Step 4:** Style pinned items differently
```dart
// Subtle background color for pinned items
Widget _buildArchiveCard(Archive archive) {
  return Card(
    color: archive.isPinned 
        ? Colors.amber.shade50 
        : null,
    elevation: archive.isPinned ? 4 : 1,
    // ... rest of card
  );
}
```

### Testing Checklist
- [ ] Pinned archives show amber badge
- [ ] Badge position doesn't cover important content
- [ ] Pinned section appears at top of history
- [ ] Pin toggle shows visual feedback (snackbar)
- [ ] Undo works correctly
- [ ] Pinned state persists across app restarts
- [ ] Badge is clearly visible in both light and dark themes

### Estimated Time: 1-2 hours

---

## Issue #7: Inconsistent Title Placement ⚠️ LOW

### Symptoms
- Some screens have title in app bar
- Some screens have title in body
- Inconsistent padding and sizing
- Unprofessional appearance

### Root Cause

No UI style guide followed during rapid development. Different developers or implementation phases used different patterns.

### Fix Plan

**Step 1:** Establish standard layout pattern
```dart
// Create: lib/widgets/standard_screen_layout.dart
class StandardScreenLayout extends StatelessWidget {
  final String title;
  final List<Widget>? actions;
  final Widget body;
  final Widget? floatingActionButton;
  final bool showBackButton;

  const StandardScreenLayout({
    required this.title,
    required this.body,
    this.actions,
    this.floatingActionButton,
    this.showBackButton = true,
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(title),
        automaticallyImplyLeading: showBackButton,
        actions: actions,
      ),
      body: body,
      floatingActionButton: floatingActionButton,
    );
  }
}
```

**Step 2:** Audit all screens and standardize

Create checklist of all screens:
- [ ] HomeScreen - ✅ Already standardized
- [ ] ArchiveDetailScreen - Needs update
- [ ] DownloadScreen - Needs update
- [ ] HistoryScreen - Needs update
- [ ] SettingsScreen - Needs update
- [ ] FiltersScreen - Needs update
- [ ] AdvancedFiltersScreen - Needs update
- [ ] HelpScreen - Needs update
- [ ] FilePreviewScreen - Needs update

**Step 3:** Refactor each screen

Example for ArchiveDetailScreen:
```dart
// BEFORE
class ArchiveDetailScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Column(
        children: [
          // Custom header with title
          Container(
            padding: EdgeInsets.all(16),
            child: Text(
              'Archive Details',
              style: TextStyle(fontSize: 24),
            ),
          ),
          // Content
        ],
      ),
    );
  }
}

// AFTER
class ArchiveDetailScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return StandardScreenLayout(
      title: 'Archive Details',
      actions: [
        IconButton(
          icon: Icon(Icons.share),
          onPressed: _shareArchive,
        ),
      ],
      body: _buildContent(),
    );
  }
}
```

**Step 4:** Create UI style guide document
```markdown
# UI Style Guide

## Screen Layout
- All screens use `StandardScreenLayout` widget
- Title always in AppBar, never in body
- Actions in AppBar trailing area
- Back button automatic (except HomeScreen)

## Typography
- Screen title: Default AppBar style
- Section headers: 18px, bold
- Body text: 14px, regular
- Captions: 12px, grey

## Spacing
- Screen padding: 16px
- Section spacing: 24px
- Item spacing: 12px
- Compact spacing: 8px

## Colors
- Primary: Material Blue
- Success: Green.shade600
- Warning: Amber.shade700
- Error: Red.shade700
- Info: Blue.shade500
```

### Testing Checklist
- [ ] All screens use StandardScreenLayout
- [ ] Titles always in AppBar
- [ ] Consistent padding across all screens
- [ ] Consistent typography sizes
- [ ] No custom header implementations
- [ ] Visual consistency verified in both light and dark themes
- [ ] Navigation feels cohesive

### Estimated Time: 2-3 hours

---

## Implementation Order

### Day 1-2: Critical Bug Fixes
1. ✅ **Issue #1: Download Progress** (2-3 hours)
   - Fix progress tracking
   - Debug event listeners
   - Verify UI updates

2. ✅ **Issue #2: Two Download Screens** (3-4 hours)
   - Consolidate on BackgroundDownloadService
   - Update navigation
   - Add "See All" button

### Day 2-3: Critical UX Improvements
3. ✅ **Issue #3: Open Downloaded Files** (2-3 hours)
   - Implement file opening
   - Add "Show in Folder"
   - Completion notifications

### Day 4: High Priority Features
4. ✅ **Issue #4: Deep Links** (2-3 hours)
   - Configure intent filters
   - Implement deep link service
   - Test with real links

### Day 4-5: Medium Priority Polish
5. ✅ **Issue #5: Settings Discoverability** (2-3 hours)
   - "What's New" banner
   - Feature tooltips
   - Onboarding hints

### Day 5: Low Priority Polish
6. ✅ **Issue #6: Pinned Indicators** (1-2 hours)
   - Add visual badges
   - Pinned section
   - Styling

7. ✅ **Issue #7: Title Consistency** (2-3 hours)
   - Create StandardScreenLayout
   - Refactor all screens
   - Style guide

---

## Testing Strategy

### Automated Testing
- Run existing 115 tests after each fix
- Ensure no regressions
- Add new tests for fixed issues

### Manual Testing Checklist

**Download Progress:**
- [ ] Start small download (10MB), verify progress updates
- [ ] Start large download (100MB+), verify sustained progress
- [ ] Pause and resume, verify progress preserved
- [ ] Multiple simultaneous downloads all update correctly

**Download Screens:**
- [ ] Navigate to downloads from app bar
- [ ] See actual in-progress downloads
- [ ] See completed downloads with actions
- [ ] "See All" button from home widget works

**File Opening:**
- [ ] Complete download shows green checkmark
- [ ] "Open" button launches correct app
- [ ] "Show in Folder" opens file manager
- [ ] Works for PDFs, images, videos

**Deep Links:**
- [ ] `adb shell am start -W -a android.intent.action.VIEW -d "ia-get://identifier/commute_test"`
- [ ] App launches and navigates to correct archive
- [ ] Works from cold start, warm start, hot link

**Settings Discoverability:**
- [ ] First launch shows "What's New" banner
- [ ] Can dismiss and stays dismissed
- [ ] Priority selector visible and functional
- [ ] Tooltips show on info icons

**Pinned Indicators:**
- [ ] Pin an archive, see amber badge
- [ ] Pinned section at top of history
- [ ] Unpin removes badge and moves to recent

**Title Consistency:**
- [ ] All screens have title in AppBar
- [ ] No custom headers in body
- [ ] Consistent spacing and typography

### Regression Testing
After all fixes:
- [ ] Run full test suite: `flutter test`
- [ ] Static analysis: `flutter analyze`
- [ ] Build APK: `flutter build apk --release`
- [ ] Install and test on real device
- [ ] Test all 74 file format downloads
- [ ] Verify API compliance features still work

---

## Documentation Updates

After completing all fixes, update:

1. **CHANGELOG.md** - Add v1.6.1 section with bug fixes
2. **README.md** - Update screenshots if UI changed
3. **docs/features/PHASE_4_KICKOFF.md** - Mark UX fixes complete
4. **Create: PRE_PHASE_4_UX_CLEANUP_COMPLETE.md** - Summary document

---

## Success Criteria

### Must Have (Before Phase 4)
- ✅ Download progress updates in real-time
- ✅ Single, unified downloads screen
- ✅ Downloaded files can be opened from app
- ✅ Deep links work for archive navigation
- ✅ All 115 tests passing
- ✅ No regressions in existing functionality

### Should Have
- ✅ New settings visible and documented
- ✅ Pinned archives have visual indicator
- ✅ Consistent UI across all screens

### Quality Gates
- ✅ Flutter analyze shows no errors
- ✅ No crashes during user testing
- ✅ All critical user complaints addressed
- ✅ App feels polished and professional

---

## Next Steps After Completion

1. **Comprehensive User Testing** (1-2 days)
   - Test all 7 fixes thoroughly
   - Get feedback from original bug reporter
   - Verify no new issues introduced

2. **Update Documentation** (0.5 days)
   - Write completion summary
   - Update roadmap
   - Create lessons learned document

3. **Release v1.6.1** (0.5 days)
   - Build release APK
   - Create GitHub release
   - Update app store listing

4. **Start Phase 4** (After v1.6.1 release)
   - Task 1: Favorites & Collections
   - Task 2: Advanced Search
   - Task 3: Download Queue
   - And more...

---

## Estimated Total Time

| Category | Hours |
|----------|-------|
| Critical Bug Fixes (Issues 1-3) | 7-10 hours |
| High Priority (Issue 4) | 2-3 hours |
| Medium Priority (Issue 5) | 2-3 hours |
| Low Priority (Issues 6-7) | 3-5 hours |
| Testing | 2-3 hours |
| Documentation | 1-2 hours |
| **TOTAL** | **17-26 hours** |

**Sprint Duration:** 1 week (5 working days)  
**Daily Average:** 3.4-5.2 hours per day  
**Realistic with buffer:** 7-10 days

---

## Contact & Support

If any issues arise during implementation:
1. Check debug logs for error details
2. Review related test failures
3. Consult Flutter/Dart documentation
4. Ask for help in team chat

**Priority:** HIGH - These fixes are BLOCKING Phase 4 development

---

## Appendix: File Locations

### Key Files for Each Issue

**Issue #1 (Progress):**
- `lib/services/background_download_service.dart`
- `lib/widgets/download_manager_widget.dart`
- `lib/models/download_progress.dart`

**Issue #2 (Two Screens):**
- `lib/screens/download_screen.dart`
- `lib/screens/home_screen.dart`
- `lib/widgets/download_manager_widget.dart`

**Issue #3 (Open Files):**
- `lib/screens/download_screen.dart`
- `lib/services/notification_service.dart`
- `pubspec.yaml` (open_file dependency)

**Issue #4 (Deep Links):**
- `android/app/src/main/AndroidManifest.xml`
- `lib/main.dart`
- `lib/services/deep_link_service.dart` (new file)

**Issue #5 (Discoverability):**
- `lib/screens/settings_screen.dart`
- `lib/screens/whats_new_screen.dart` (new file)
- `lib/screens/archive_detail_screen.dart`

**Issue #6 (Pinned):**
- `lib/widgets/archive_card_widget.dart` (or similar)
- `lib/screens/history_screen.dart`

**Issue #7 (Titles):**
- `lib/widgets/standard_screen_layout.dart` (new file)
- All screen files in `lib/screens/`

---

**End of Pre-Phase 4 UX Cleanup Sprint Document**
