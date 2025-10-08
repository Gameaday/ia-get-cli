# Phase 4 Task 3: Download Queue Management - Implementation Plan

**Status**: 🚧 In Progress  
**Priority**: 🟠 High  
**Estimated Effort**: 10-12 hours  
**Timeline**: Week 2-3

---

## 📋 Overview

Implement comprehensive download queue management with resumable downloads, smart scheduling, and enhanced UI. This builds on the existing `BackgroundDownloadService` infrastructure.

### Goals
1. ✅ HTTP Range request support for resume capability
2. ✅ Partial download tracking in database
3. ✅ Auto-resume on network reconnect
4. ✅ Progress persistence across app restarts
5. ✅ Queue management UI with reordering
6. ✅ Smart scheduling (priority, network-aware, time-based)

---

## 🏗️ Current Architecture Analysis

### Existing Infrastructure ✅

**BackgroundDownloadService** (`lib/services/background_download_service.dart`):
- ✅ Active download tracking (`_activeDownloads`, `_completedDownloads`)
- ✅ Download queue (`_downloadQueue`)
- ✅ Concurrent download limit (`_maxConcurrentDownloads`)
- ✅ Retry logic (`_maxRetries`, `_retryTimer`)
- ✅ Progress tracking (`DownloadProgress` model)
- ✅ Notification integration
- ⚠️ **Missing**: Resume capability, ETag tracking, partial file handling

**DownloadProgress** (`lib/models/download_progress.dart`):
- ✅ Progress tracking (bytes, percentage, speed, ETA)
- ✅ Status enum (queued, downloading, paused, completed, error, cancelled)
- ✅ Retry count tracking
- ⚠️ **Missing**: ETag field, partial bytes field, resume offset field

**Database** (`lib/services/database_helper.dart`):
- ✅ SQLite for data persistence
- ⚠️ **Missing**: download_tasks table for resume state

### What Needs to be Built 🛠️

1. **ResumableDownloadService**: HTTP Range requests + partial file handling
2. **DownloadTask Model**: Enhanced model with resume state
3. **DownloadScheduler**: Priority + network-aware scheduling
4. **DownloadQueueScreen**: Visual queue management UI
5. **Database Schema**: New tables for resume state

---

## 📦 Part 1: Resumable Downloads Service (4-5 hours)

### 1.1 Create DownloadTask Model

**File**: `lib/models/download_task.dart` (NEW)

```dart
enum DownloadPriority { low, normal, high }
enum NetworkRequirement { any, wiFiOnly, unmetered }

class DownloadTask {
  final String id;
  final String identifier;
  final String url;
  final String savePath;
  final String fileName;
  
  // Resume state
  final int partialBytes;        // Bytes already downloaded
  final String? etag;            // Server ETag for validation
  final String? lastModified;    // Server last-modified header
  final int totalBytes;          // Total file size
  
  // Scheduling
  final DownloadPriority priority;
  final NetworkRequirement networkRequirement;
  final DateTime? scheduledTime; // Schedule for later
  
  // Metadata
  final DateTime createdAt;
  final DateTime? startedAt;
  final DateTime? completedAt;
  final int retryCount;
  final String? errorMessage;
  final DownloadStatus status;
  
  // Archive-specific
  final ArchiveMetadata? metadata;
  final List<String>? selectedFiles;
}
```

**Features**:
- Resume offset tracking (`partialBytes`)
- ETag validation for resume integrity
- Priority and network scheduling
- Full lifecycle timestamps
- Archive metadata association

---

### 1.2 Create ResumableDownloadService

**File**: `lib/services/resumable_download_service.dart` (NEW)

```dart
class ResumableDownloadService {
  final http.Client _client;
  final DatabaseHelper _db;
  
  /// Start or resume a download with HTTP Range support
  Future<void> downloadFile({
    required DownloadTask task,
    required Function(int downloaded, int total) onProgress,
    CancelToken? cancelToken,
  }) async {
    final file = File(task.savePath);
    int startByte = task.partialBytes;
    
    // Step 1: Verify partial download integrity
    if (startByte > 0) {
      final isValid = await _validatePartialDownload(file, task);
      if (!isValid) {
        // Server changed file, start fresh
        await file.delete();
        startByte = 0;
        await _updateTaskProgress(task.id, 0);
      }
    }
    
    // Step 2: Request with Range header
    final request = http.Request('GET', Uri.parse(task.url));
    if (startByte > 0) {
      request.headers['Range'] = 'bytes=$startByte-';
    }
    request.headers['If-Match'] = task.etag ?? '';
    
    // Step 3: Handle response (206 Partial or 200 OK)
    final response = await _client.send(request);
    
    if (response.statusCode == 206) {
      // Resume: Append to existing file
      await _streamToFile(
        response, 
        file, 
        mode: FileMode.append,
        startOffset: startByte,
        onProgress: onProgress,
        task: task,
      );
    } else if (response.statusCode == 200) {
      // Full download: Overwrite file
      await _streamToFile(
        response,
        file,
        mode: FileMode.write,
        startOffset: 0,
        onProgress: onProgress,
        task: task,
      );
    } else if (response.statusCode == 412) {
      // Precondition failed (ETag mismatch): Start fresh
      await file.delete();
      return downloadFile(
        task: task.copyWith(partialBytes: 0),
        onProgress: onProgress,
        cancelToken: cancelToken,
      );
    }
    
    // Step 4: Verify completion
    await _verifyDownload(file, task);
  }
  
  /// Validate partial download integrity using ETag
  Future<bool> _validatePartialDownload(File file, DownloadTask task) async {
    if (task.etag == null) return true; // No ETag, assume valid
    
    // HEAD request to get current ETag
    final response = await _client.head(Uri.parse(task.url));
    final serverEtag = response.headers['etag'];
    
    return serverEtag == task.etag;
  }
  
  /// Stream response to file with progress tracking
  Future<void> _streamToFile(
    http.StreamedResponse response,
    File file,
    FileMode mode,
    int startOffset,
    Function(int, int) onProgress,
    DownloadTask task,
  ) async {
    final sink = file.openWrite(mode: mode);
    int downloaded = startOffset;
    final total = task.totalBytes;
    
    await for (final chunk in response.stream) {
      sink.add(chunk);
      downloaded += chunk.length;
      
      // Update progress in database every 100KB
      if (downloaded % (100 * 1024) == 0) {
        await _updateTaskProgress(task.id, downloaded);
      }
      
      onProgress(downloaded, total);
    }
    
    await sink.close();
  }
  
  /// Update download progress in database
  Future<void> _updateTaskProgress(String taskId, int partialBytes) async {
    await _db.database.then((db) => db.update(
      'download_tasks',
      {'partial_bytes': partialBytes, 'updated_at': DateTime.now().toIso8601String()},
      where: 'id = ?',
      whereArgs: [taskId],
    ));
  }
}
```

**Key Features**:
- ✅ HTTP Range request support
- ✅ ETag validation for integrity
- ✅ Partial download resumption
- ✅ Progress persistence (every 100KB)
- ✅ Automatic retry on ETag mismatch
- ✅ Clean error handling

---

### 1.3 Database Schema Updates

**File**: `lib/services/database_helper.dart` (MODIFY)

```sql
CREATE TABLE IF NOT EXISTS download_tasks (
  id TEXT PRIMARY KEY,
  identifier TEXT NOT NULL,
  url TEXT NOT NULL,
  save_path TEXT NOT NULL,
  file_name TEXT NOT NULL,
  
  -- Resume state
  partial_bytes INTEGER NOT NULL DEFAULT 0,
  etag TEXT,
  last_modified TEXT,
  total_bytes INTEGER NOT NULL,
  
  -- Scheduling
  priority TEXT NOT NULL DEFAULT 'normal',
  network_requirement TEXT NOT NULL DEFAULT 'any',
  scheduled_time TEXT,
  
  -- Status
  status TEXT NOT NULL,
  retry_count INTEGER NOT NULL DEFAULT 0,
  error_message TEXT,
  
  -- Timestamps
  created_at TEXT NOT NULL,
  started_at TEXT,
  completed_at TEXT,
  updated_at TEXT NOT NULL,
  
  -- Archive metadata (JSON)
  metadata TEXT,
  selected_files TEXT
);

CREATE INDEX idx_download_tasks_status ON download_tasks(status);
CREATE INDEX idx_download_tasks_scheduled ON download_tasks(scheduled_time);
CREATE INDEX idx_download_tasks_priority ON download_tasks(priority);
```

**Migration**:
- Add migration version tracking
- Create `_upgradeDatabase` method
- Handle existing downloads gracefully

---

## 🎨 Part 2: Queue Management UI (3-4 hours)

### 2.1 DownloadQueueScreen

**File**: `lib/screens/download_queue_screen.dart` (NEW)

**Features**:
- Reorderable list (drag-and-drop with `ReorderableListView`)
- Tab bar: Active (downloading), Queued, Completed, Failed
- Search and filter by status
- Bulk actions (pause all, resume all, clear completed)
- Queue statistics at top

**Layout**:
```
┌─────────────────────────────────────┐
│ 📥 Download Queue                   │
│                                     │
│ ┌─ Stats ─────────────────────────┐ │
│ │ 2 Active • 5 Queued • 10 Done  │ │
│ │ Speed: 2.5 MB/s • ETA: 10m     │ │
│ └─────────────────────────────────┘ │
│                                     │
│ [Active] [Queued] [Done] [Failed]  │
│                                     │
│ ┌─ Download Item ─────────────────┐ │
│ │ 📄 filename.zip                 │ │
│ │ ████████░░ 75% • 1.2 MB/s      │ │
│ │ [⏸] [❌] [⋯]                   │ │
│ └─────────────────────────────────┘ │
│                                     │
│ ┌─ Download Item (Queued) ────────┐ │
│ │ 📄 another.pdf                  │ │
│ │ Queued • Priority: High         │ │
│ │ [▶] [❌] [⋯]                   │ │
│ └─────────────────────────────────┘ │
└─────────────────────────────────────┘
```

---

### 2.2 DownloadQueueItem Widget

**File**: `lib/widgets/download_queue_item.dart` (NEW)

```dart
class DownloadQueueItem extends StatelessWidget {
  final DownloadTask task;
  final VoidCallback? onPause;
  final VoidCallback? onResume;
  final VoidCallback? onCancel;
  final VoidCallback? onRetry;
  final VoidCallback? onDelete;
  
  @override
  Widget build(BuildContext context) {
    return Card(
      child: ListTile(
        leading: _buildStatusIcon(),
        title: Text(task.fileName),
        subtitle: _buildSubtitle(),
        trailing: _buildActionButtons(),
      ),
    );
  }
  
  Widget _buildStatusIcon() {
    switch (task.status) {
      case DownloadStatus.downloading:
        return CircularProgressIndicator(value: task.progress);
      case DownloadStatus.queued:
        return Icon(Icons.schedule);
      case DownloadStatus.completed:
        return Icon(Icons.check_circle, color: Colors.green);
      case DownloadStatus.error:
        return Icon(Icons.error, color: Colors.red);
      default:
        return Icon(Icons.download);
    }
  }
  
  Widget _buildSubtitle() {
    if (task.status == DownloadStatus.downloading) {
      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          LinearProgressIndicator(value: task.progress),
          SizedBox(height: 4),
          Text('${_formatSpeed(task.speed)} • ${_formatETA(task.etaSeconds)}'),
        ],
      );
    } else if (task.status == DownloadStatus.queued) {
      return Text('Priority: ${task.priority.name} • Network: ${task.networkRequirement.name}');
    }
    return Text('Status: ${task.status.name}');
  }
}
```

---

### 2.3 QueueStatistics Widget

**File**: `lib/widgets/queue_statistics.dart` (NEW)

- Real-time statistics card
- Active/queued/completed counts
- Total speed (sum of all active downloads)
- Combined ETA
- Data usage today

---

## ⚙️ Part 3: Smart Scheduling (3-4 hours)

### 3.1 DownloadScheduler Service

**File**: `lib/services/download_scheduler.dart` (NEW)

```dart
class DownloadScheduler {
  final ResumableDownloadService _downloadService;
  final DatabaseHelper _db;
  final NetworkInfo _networkInfo;
  
  /// Process the download queue with smart scheduling
  Future<void> processQueue() async {
    // Step 1: Get eligible tasks
    final tasks = await _getEligibleTasks();
    
    // Step 2: Sort by priority and creation time
    tasks.sort((a, b) {
      if (a.priority != b.priority) {
        return _priorityValue(b.priority) - _priorityValue(a.priority);
      }
      return a.createdAt.compareTo(b.createdAt);
    });
    
    // Step 3: Check network constraints
    final currentNetwork = await _networkInfo.getNetworkType();
    final eligibleTasks = tasks.where((task) {
      return _checkNetworkRequirement(task, currentNetwork);
    }).toList();
    
    // Step 4: Start downloads up to concurrent limit
    final activeCount = await _getActiveDownloadCount();
    final maxConcurrent = _getMaxConcurrentDownloads();
    final slotsAvailable = maxConcurrent - activeCount;
    
    for (var i = 0; i < slotsAvailable && i < eligibleTasks.length; i++) {
      await _startDownload(eligibleTasks[i]);
    }
  }
  
  /// Get tasks that are ready to download
  Future<List<DownloadTask>> _getEligibleTasks() async {
    return await _db.getDownloadTasks(
      statuses: [DownloadStatus.queued],
      scheduledBefore: DateTime.now(),
    );
  }
  
  /// Check if network meets task requirements
  bool _checkNetworkRequirement(DownloadTask task, NetworkType network) {
    switch (task.networkRequirement) {
      case NetworkRequirement.wiFiOnly:
        return network == NetworkType.wifi;
      case NetworkRequirement.unmetered:
        return network == NetworkType.wifi || network == NetworkType.ethernet;
      case NetworkRequirement.any:
        return true;
    }
  }
  
  /// Priority scoring (higher is better)
  int _priorityValue(DownloadPriority priority) {
    switch (priority) {
      case DownloadPriority.high:
        return 3;
      case DownloadPriority.normal:
        return 2;
      case DownloadPriority.low:
        return 1;
    }
  }
  
  /// Bandwidth distribution algorithm
  void _distributeBandwidth(List<DownloadTask> activeTasks) {
    final totalBandwidth = _getBandwidthLimit();
    if (totalBandwidth == null) return; // No limit
    
    // Equal share for same priority, weighted by priority
    final highPriority = activeTasks.where((t) => t.priority == DownloadPriority.high).length;
    final normalPriority = activeTasks.where((t) => t.priority == DownloadPriority.normal).length;
    final lowPriority = activeTasks.where((t) => t.priority == DownloadPriority.low).length;
    
    // Weights: High=4, Normal=2, Low=1
    final totalWeight = (highPriority * 4) + (normalPriority * 2) + (lowPriority * 1);
    
    for (final task in activeTasks) {
      final weight = _priorityValue(task.priority);
      final allocatedBandwidth = (totalBandwidth * weight) / totalWeight;
      _setBandwidthLimit(task.id, allocatedBandwidth);
    }
  }
}
```

**Scheduling Algorithms**:
1. **Priority-based**: High > Normal > Low
2. **Size-based optimization**: Option to prioritize small files first
3. **Network-aware**: Wi-Fi only, unmetered only, any
4. **Time-based**: Schedule downloads for specific times
5. **Bandwidth distribution**: Fair sharing based on priority

---

### 3.2 Network Detection

**File**: `lib/utils/network_utils.dart` (ENHANCE)

```dart
enum NetworkType { wifi, ethernet, cellular, none }

class NetworkInfo {
  /// Get current network type
  Future<NetworkType> getNetworkType() async {
    final connectivityResult = await Connectivity().checkConnectivity();
    
    if (connectivityResult == ConnectivityResult.wifi) {
      return NetworkType.wifi;
    } else if (connectivityResult == ConnectivityResult.ethernet) {
      return NetworkType.ethernet;
    } else if (connectivityResult == ConnectivityResult.mobile) {
      return NetworkType.cellular;
    }
    return NetworkType.none;
  }
  
  /// Listen for network changes
  Stream<NetworkType> onNetworkChanged() {
    return Connectivity().onConnectivityChanged.map(_mapToNetworkType);
  }
}
```

---

## 🧪 Part 4: Testing & Integration (2-3 hours)

### 4.1 Unit Tests

**File**: `test/services/resumable_download_service_test.dart` (NEW)

```dart
group('ResumableDownloadService', () {
  test('should resume download with Range header', () async {
    // Arrange: Mock HTTP client with 206 Partial Content
    // Act: Start download with partial_bytes > 0
    // Assert: Range header sent, file appended
  });
  
  test('should restart on ETag mismatch', () async {
    // Arrange: Mock HTTP client with different ETag
    // Act: Validate partial download
    // Assert: File deleted, download restarted
  });
  
  test('should persist progress every 100KB', () async {
    // Arrange: Mock database
    // Act: Download 500KB file
    // Assert: Database updated 5 times
  });
});
```

---

### 4.2 Integration Testing Checklist

- [ ] **Resume after interruption**: Kill app mid-download, verify resume on restart
- [ ] **ETag validation**: Change file on server, verify fresh download
- [ ] **Network change**: Switch from Wi-Fi to cellular, verify pause/resume
- [ ] **Queue reordering**: Drag items, verify order persists
- [ ] **Priority scheduling**: Add high-priority item, verify it jumps queue
- [ ] **Bandwidth limiting**: Set limit, verify speed stays within bounds
- [ ] **Scheduled downloads**: Schedule for future time, verify execution
- [ ] **Concurrent limit**: Set limit to 2, verify only 2 active at once
- [ ] **Partial file cleanup**: Cancel download, verify partial file removed
- [ ] **Progress persistence**: Force close app, verify progress on restart

---

## 📊 Success Metrics

### Functional Requirements ✅
- [ ] Downloads resume after app restart (progress within 1%)
- [ ] ETag validation prevents corrupted resumes
- [ ] Queue reordering persists across sessions
- [ ] Priority scheduling works (high-priority items start first)
- [ ] Network-aware downloads respect Wi-Fi only setting
- [ ] Scheduled downloads execute at correct time (±5 minutes)
- [ ] Bandwidth distribution is fair (within 10% variance)

### Performance Requirements ✅
- [ ] Resume latency < 2 seconds
- [ ] Queue processing < 1 second for 100 items
- [ ] Database updates < 100ms per write
- [ ] UI remains responsive during queue operations (60 FPS)

### UX Requirements ✅
- [ ] Visual feedback for all actions (< 200ms)
- [ ] Clear error messages for failures
- [ ] Progress animations smooth (MD3 compliant)
- [ ] Accessible (TalkBack compatible)
- [ ] Dark mode support (100% compatible)

---

## 📝 Implementation Timeline

### Day 1 (4-5 hours): Resumable Downloads
- [x] Create DownloadTask model
- [ ] Implement ResumableDownloadService
- [ ] Add database schema
- [ ] Write unit tests for resume logic

### Day 2 (3-4 hours): Queue UI
- [ ] Build DownloadQueueScreen
- [ ] Create DownloadQueueItem widget
- [ ] Add QueueStatistics widget
- [ ] Implement drag-and-drop reordering

### Day 3 (3-4 hours): Smart Scheduling
- [ ] Implement DownloadScheduler
- [ ] Add network detection
- [ ] Implement bandwidth distribution
- [ ] Add scheduled download support

### Day 4 (1-2 hours): Testing & Polish
- [ ] Run integration tests
- [ ] Fix any bugs discovered
- [ ] Update documentation
- [ ] Create completion report

---

## 🎯 Next Actions

1. **Immediate**: Create DownloadTask model
2. **Then**: Implement ResumableDownloadService with HTTP Range support
3. **Then**: Add database migration for download_tasks table
4. **Then**: Build DownloadQueueScreen UI
5. **Finally**: Implement DownloadScheduler

Ready to start? Let's begin with the DownloadTask model!
