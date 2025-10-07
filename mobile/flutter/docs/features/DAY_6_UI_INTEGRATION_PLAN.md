# Day 6: UI Integration & Enhanced Download Management

**Target Duration**: 2-3 hours  
**Status**: 🚧 Planning Phase

## Overview

Integrate Days 1-5 components (RateLimiter, IAHttpClient, BandwidthThrottle, ETag caching, Priority downloads) into the Flutter UI, providing users with rich visual feedback and control over downloads.

## Current State Assessment

### ✅ What We Have
- `DownloadProvider` - State management for downloads
- `DownloadManagerWidget` - Basic download UI
- `download_controls_widget.dart` - Download controls
- `download_statistics_widget.dart` - Statistics display
- `BackgroundDownloadService` - Background download handling
- Basic progress tracking

### ⚠️ What's Missing
- No integration with BandwidthThrottle (Day 3)
- No bandwidth limit controls in UI
- No priority download toggle
- No rate limit status display
- No ETag cache statistics
- No retry/backoff visualization

## Objectives

### 1. Bandwidth Management UI
**Goal**: Let users control and monitor bandwidth usage

**Features**:
- [ ] Bandwidth limit selector (presets: 256KB/s, 512KB/s, 1MB/s, 5MB/s, 10MB/s, Unlimited)
- [ ] Real-time bandwidth usage display
- [ ] Per-download and global bandwidth graphs
- [ ] Pause/resume bandwidth throttling
- [ ] Show throttle events (when downloads are slowed)

**Files to Modify**:
- `lib/widgets/download_controls_widget.dart` - Add bandwidth controls
- `lib/widgets/download_statistics_widget.dart` - Show bandwidth stats
- `lib/providers/download_provider.dart` - Integrate BandwidthThrottle

### 2. Priority Download Control
**Goal**: Let users mark downloads as reduced priority

**Features**:
- [ ] Toggle for "Good Citizen Mode" (reduced priority)
- [ ] Auto-detect and show when files are auto-reduced (>= 50MB)
- [ ] Per-download priority override
- [ ] Global default priority setting
- [ ] Visual indicator for reduced priority downloads

**Files to Modify**:
- `lib/widgets/download_manager_widget.dart` - Add priority indicators
- `lib/screens/settings_screen.dart` - Add priority settings
- `lib/providers/download_provider.dart` - Pass priority to API

### 3. Enhanced Progress Tracking
**Goal**: Show detailed download progress with network info

**Features**:
- [ ] Real-time download speed (bytes/sec)
- [ ] Estimated time remaining (ETA)
- [ ] Network health indicator (rate limit status)
- [ ] Retry attempt counter
- [ ] ETag cache hit/miss indicator
- [ ] Smooth progress animations

**Files to Create/Modify**:
- `lib/widgets/download_progress_card.dart` - Enhanced progress display
- `lib/models/download_statistics.dart` - Statistics model
- `lib/providers/download_provider.dart` - Calculate stats

### 4. Rate Limit Status Display
**Goal**: Show rate limiter health to users

**Features**:
- [ ] Rate limit status badge (healthy/warning/throttled)
- [ ] Current requests/minute counter
- [ ] Rate limit threshold indicator
- [ ] Cool-down timer when throttled
- [ ] Historical rate limit graph

**Files to Create**:
- `lib/widgets/rate_limit_indicator.dart` - Rate limit status widget
- `lib/services/rate_limit_monitor.dart` - Monitor service

### 5. Cache Statistics
**Goal**: Show ETag cache performance

**Features**:
- [ ] Cache hit rate percentage
- [ ] Bandwidth saved via 304 Not Modified
- [ ] Cached items count
- [ ] Cache clear button
- [ ] Per-item cache status

**Files to Modify**:
- `lib/screens/settings_screen.dart` - Add cache stats section
- `lib/services/archive_service.dart` - Expose cache stats

### 6. Error Handling UI
**Goal**: User-friendly error messages with retry options

**Features**:
- [ ] Categorized error types (network, rate limit, server, etc.)
- [ ] Suggested actions for each error type
- [ ] One-tap retry with exponential backoff visualization
- [ ] Error history log
- [ ] Smart error recovery

**Files to Create**:
- `lib/widgets/error_dialog.dart` - Enhanced error dialogs
- `lib/utils/error_handler.dart` - Error categorization

## Implementation Plan

### Phase 1: Bandwidth Management (45 min)
1. Create `BandwidthManagerProvider` to wrap BandwidthThrottle
2. Add bandwidth controls to `download_controls_widget.dart`
3. Update `download_statistics_widget.dart` with bandwidth display
4. Integrate into `DownloadProvider`

### Phase 2: Priority Controls (30 min)
1. Add priority parameter to download methods
2. Create priority toggle in download UI
3. Add settings for default priority
4. Show visual indicators for reduced priority

### Phase 3: Enhanced Progress (45 min)
1. Create `DownloadStatistics` model
2. Build `download_progress_card.dart` widget
3. Calculate ETA and speed in provider
4. Add smooth progress animations

### Phase 4: Rate Limit Display (30 min)
1. Create `RateLimitIndicator` widget
2. Expose RateLimiter stats from IAHttpClient
3. Add to download manager header
4. Show cool-down timer

### Phase 5: Cache Stats (20 min)
1. Add cache statistics methods to MetadataCache
2. Create cache stats section in settings
3. Add cache clear functionality
4. Show bandwidth saved

### Phase 6: Error UI (30 min)
1. Create error categorization utility
2. Build enhanced error dialogs
3. Add retry with backoff visualization
4. Implement error history

## Technical Architecture

### State Flow
```
User Action (UI)
    ↓
DownloadProvider (State Management)
    ↓
ArchiveService (Orchestration)
    ↓
InternetArchiveApi (HTTP Layer)
    ↓
┌─────────────────┬────────────────────┬──────────────────┐
│                 │                    │                  │
IAHttpClient    MetadataCache    BandwidthThrottle
│                 │                    │
RateLimiter     ETag Storage     Token Bucket
(Day 1)         (Day 4)          (Day 3)
```

### UI Component Hierarchy
```
DownloadScreen
├── DownloadManagerWidget
│   ├── BandwidthControls
│   ├── RateLimitIndicator
│   └── DownloadProgressCard (per download)
│       ├── ProgressBar
│       ├── SpeedIndicator
│       ├── ETADisplay
│       ├── PriorityBadge
│       └── ActionButtons
└── DownloadStatisticsWidget
    ├── BandwidthGraph
    ├── CacheStats
    └── ErrorHistory
```

## Models to Create/Enhance

### 1. DownloadStatistics
```dart
class DownloadStatistics {
  final int bytesDownloaded;
  final int totalBytes;
  final double bytesPerSecond;
  final Duration estimatedTimeRemaining;
  final int retryAttempts;
  final bool isThrottled;
  final bool isReducedPriority;
  final bool cacheHit;
  final RateLimitStatus rateLimitStatus;
}
```

### 2. RateLimitStatus
```dart
enum RateLimitStatus {
  healthy,      // Under 50% of limit
  warning,      // 50-80% of limit
  throttled,    // 80-100% of limit
  cooldown,     // Temporarily blocked
}
```

### 3. BandwidthPreset
```dart
enum BandwidthPreset {
  kb256(256 * 1024),
  kb512(512 * 1024),
  mb1(1024 * 1024),
  mb5(5 * 1024 * 1024),
  mb10(10 * 1024 * 1024),
  unlimited(0);
  
  final int bytesPerSecond;
}
```

## UI Mockups (Conceptual)

### Enhanced Progress Card
```
┌────────────────────────────────────────────────┐
│ 📄 commute_test.mp3                    [⏸] [✖]│
│ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ │
│ 45.2 MB / 68.5 MB (66%)                        │
│                                                 │
│ 🚀 1.2 MB/s  •  ⏱️ 19s left  •  🔄 Retry 0/3  │
│ 🌐 Rate: Healthy  •  ⚡ Priority: Normal       │
│ 💾 Cache: Miss  •  🔧 Throttled: No           │
└────────────────────────────────────────────────┘
```

### Bandwidth Controls
```
┌────────────────────────────────────────────────┐
│ Bandwidth Limit                                 │
│ ┌──┬──┬──┬──┬──┬────┐                         │
│ │256K│512K│1M│5M│10M│∞│  [Currently: 1 MB/s]  │
│ └──┴──┴──┴──┴──┴────┘                         │
│                                                 │
│ Current Usage: ━━━━━━━━━━━━━━━ 0.8 / 1.0 MB/s│
│ Active Downloads: 2  •  Queued: 1              │
└────────────────────────────────────────────────┘
```

### Rate Limit Indicator
```
┌────────────────────────────────────────────────┐
│ 🟢 Rate Limit: Healthy                         │
│ 18 / 30 requests per minute (60%)              │
│ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ │
└────────────────────────────────────────────────┘
```

## Testing Strategy

### Unit Tests
- [ ] DownloadStatistics calculation
- [ ] Bandwidth preset conversions
- [ ] Rate limit status detection
- [ ] ETA calculation accuracy

### Widget Tests
- [ ] BandwidthControls interaction
- [ ] Progress card rendering
- [ ] Priority toggle functionality
- [ ] Error dialog display

### Integration Tests
- [ ] Full download flow with bandwidth limiting
- [ ] Priority download end-to-end
- [ ] Rate limit recovery flow
- [ ] Cache hit/miss scenarios

## Success Criteria

### Performance
- [ ] UI remains responsive during downloads
- [ ] Progress updates at 60 FPS
- [ ] No memory leaks during long downloads
- [ ] Bandwidth limiting accurate within 5%

### Usability
- [ ] Intuitive bandwidth controls
- [ ] Clear error messages with actions
- [ ] Progress info updates in real-time
- [ ] Settings persist across restarts

### Quality
- [ ] All new widgets tested
- [ ] 0 analyzer warnings
- [ ] Maintains 100% test pass rate
- [ ] Smooth animations

## Timeline

### Session 1 (1.5 hours)
- Implement bandwidth management UI
- Add priority controls
- Basic progress enhancements

### Session 2 (1 hour)
- Rate limit indicator
- Cache statistics
- Error handling UI

### Session 3 (30 min)
- Testing and polish
- Documentation
- Final integration

## References

- Days 1-5 completion documents
- Flutter Material Design 3 guidelines
- Internet Archive API best practices
- BandwidthThrottle API from Day 3
- IAHttpClient API from Day 2

## Next Steps

1. Start with Phase 1 (Bandwidth Management)
2. Create bandwidth control widgets
3. Integrate BandwidthThrottle into provider
4. Test bandwidth limiting in UI

**Ready to begin implementation!** 🚀
