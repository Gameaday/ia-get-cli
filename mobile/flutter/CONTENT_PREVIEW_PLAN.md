# Content Preview System - Implementation Plan

## Overview
Implement a universal file preview system for the ia-get Flutter mobile app, enabling users to preview files before downloading. This feature will support text, images, audio, and video with intelligent caching.

## Priority
**User-specified priority #2** (after offline metadata caching)

## Goals

### Primary Goals
1. **Preview Without Download**: View file content without full download
2. **Multi-Format Support**: Text, images, audio, video
3. **Smart Caching**: Cache small previews, prompt for large files
4. **Seamless Integration**: Works with existing offline caching system
5. **Offline Support**: Cached previews available offline

### User Experience Goals
- Quick preview access from file list
- Smooth navigation between files
- Clear indicators for cached vs. uncached previews
- Download prompt for large files (>5MB)
- Offline availability for cached previews

## Supported File Types

### Phase 1: Text & Images (MVP)
- **Text**: `.txt`, `.md`, `.json`, `.xml`, `.csv`, `.log`
- **Images**: `.jpg`, `.jpeg`, `.png`, `.gif`, `.webp`, `.bmp`

### Phase 2: Audio & Video
- **Audio**: `.mp3`, `.wav`, `.ogg`, `.m4a`, `.flac`
- **Video**: `.mp4`, `.webm`, `.avi`, `.mov` (thumbnails only)

### Phase 3: Documents
- **PDF**: First page preview or text extraction
- **Archive**: Directory listing without extraction

## Architecture

### Components

#### 1. FilePreviewService (Core)
```dart
class FilePreviewService {
  // Preview generation
  Future<FilePreview> generatePreview(ArchiveFile file);
  
  // Cache management
  Future<FilePreview?> getCachedPreview(String identifier, String fileName);
  Future<void> cachePreview(FilePreview preview);
  Future<void> deleteCachedPreview(String identifier, String fileName);
  
  // Smart download decision
  bool shouldDownloadFirst(int fileSize);
  
  // Preview availability
  Future<bool> isPreviewCached(String identifier, String fileName);
  Future<bool> canPreview(String format);
}
```

#### 2. Preview Models
```dart
class FilePreview {
  final String identifier;
  final String fileName;
  final PreviewType type;
  final dynamic content; // String for text, Uint8List for images, etc.
  final DateTime cachedAt;
  final int fileSize;
}

enum PreviewType {
  text,
  image,
  audioWaveform,
  videoThumbnail,
  unavailable,
}
```

#### 3. Preview Widgets
- `TextPreviewWidget` - Text file viewer
- `ImagePreviewWidget` - Image viewer with zoom/pan
- `AudioPreviewWidget` - Audio player with waveform
- `VideoPreviewWidget` - Video player with controls
- `PreviewDialogWidget` - Full-screen preview dialog

#### 4. Database Extension
```sql
CREATE TABLE preview_cache (
  identifier TEXT NOT NULL,
  file_name TEXT NOT NULL,
  preview_type TEXT NOT NULL,
  preview_data BLOB,
  text_content TEXT, -- For text previews
  cached_at INTEGER NOT NULL,
  file_size INTEGER NOT NULL,
  PRIMARY KEY (identifier, file_name)
);

CREATE INDEX idx_preview_cache_identifier ON preview_cache(identifier);
CREATE INDEX idx_preview_cache_cached_at ON preview_cache(cached_at DESC);
```

## Implementation Phases

### Phase 1: Foundation (Week 1)
**Goal**: Basic text and image preview

**Tasks**:
1. ✅ Create `FilePreviewService` with basic API
2. ✅ Extend database with `preview_cache` table
3. ✅ Create `FilePreview` model
4. ✅ Implement text preview generation
5. ✅ Implement image preview generation
6. ✅ Create `TextPreviewWidget`
7. ✅ Create `ImagePreviewWidget`
8. ✅ Add preview button to file list
9. ✅ Integrate with caching system

**Deliverables**:
- Text files previewable
- Images previewable
- Basic caching working
- UI integrated

### Phase 2: Enhanced UI (Week 2)
**Goal**: Polished preview experience

**Tasks**:
1. ✅ Full-screen preview dialog
2. ✅ Swipe navigation between files
3. ✅ Zoom/pan for images
4. ✅ Syntax highlighting for code
5. ✅ Markdown rendering
6. ✅ Download prompt for large files
7. ✅ Offline availability indicators
8. ✅ Share preview feature

**Deliverables**:
- Professional preview UI
- Navigation working
- Large file handling
- Offline support

### Phase 3: Audio & Video (Week 3)
**Goal**: Multimedia preview

**Tasks**:
1. ✅ Audio player implementation
2. ✅ Waveform visualization
3. ✅ Video thumbnail generation
4. ✅ Video player (for small files)
5. ✅ Playback controls
6. ✅ Background audio support

**Deliverables**:
- Audio playback working
- Video thumbnails generated
- Multimedia controls

### Phase 4: Advanced Features (Week 4)
**Goal**: Polish and optimization

**Tasks**:
1. ✅ PDF preview (first page)
2. ✅ Archive preview (directory listing)
3. ✅ Preview cache management in settings
4. ✅ Batch cache clearing
5. ✅ Performance optimization
6. ✅ Comprehensive testing

**Deliverables**:
- All file types supported
- Settings integration
- Production-ready

## Technical Decisions

### Smart Download Threshold
- **<1MB**: Always cache preview
- **1-5MB**: Cache with user confirmation
- **>5MB**: Require download first

### Cache Strategy
- **Text**: Store in database as TEXT
- **Images**: Store in database as BLOB (compressed)
- **Audio/Video**: Store thumbnails/waveforms only
- **Large Files**: Don't cache, stream on demand

### Image Handling
- **Thumbnail Size**: 800x800px max
- **Compression**: JPEG quality 85%
- **Format**: Convert all to JPEG for consistency
- **Memory**: Dispose images after use

### Text Handling
- **Max Length**: 1MB text content
- **Encoding**: UTF-8 detection
- **Line Limit**: 10,000 lines displayed
- **Truncation**: Show first/last with "..." indicator

## UI/UX Design

### File List Integration
```
┌─────────────────────────────────────┐
│ Files                                │
├─────────────────────────────────────┤
│ 📄 document.pdf         👁️ ⬇️      │
│    1.2 MB                            │
│                                      │
│ 🖼️  image.jpg          👁️ ⬇️      │
│    350 KB                            │
│                                      │
│ 📝 readme.txt          👁️ ⬇️      │
│    15 KB               [Cached]      │
└─────────────────────────────────────┘
```

### Preview Dialog
```
┌─────────────────────────────────────┐
│ ← readme.txt          [•][•][•] →  │
├─────────────────────────────────────┤
│                                      │
│  # Internet Archive Project         │
│                                      │
│  This archive contains...           │
│                                      │
│                                      │
│                                      │
│                                      │
├─────────────────────────────────────┤
│ 📥 Download  📤 Share  ❌ Close     │
└─────────────────────────────────────┘
```

### Image Preview
```
┌─────────────────────────────────────┐
│ ← photo.jpg          [•][•][•] →   │
├─────────────────────────────────────┤
│                                      │
│         [      IMAGE      ]          │
│         [    CONTENT      ]          │
│         [      HERE       ]          │
│                                      │
│                                      │
├─────────────────────────────────────┤
│ 🔍+ 🔍- 📥 Download 📤 Share ❌     │
└─────────────────────────────────────┘
```

## Dependencies

### Required Packages
```yaml
dependencies:
  # Image handling
  cached_network_image: ^3.3.1
  photo_view: ^0.15.0  # Zoom/pan
  
  # Text/syntax highlighting
  flutter_highlight: ^0.7.0
  flutter_markdown: ^0.7.3
  
  # Audio/video
  audioplayers: ^6.0.0
  video_player: ^2.9.1
  
  # Utilities
  mime: ^1.0.5  # MIME type detection
  path: ^1.9.0  # File path utilities
  
  # Already have
  sqflite: ^2.4.1
  http: ^1.2.2
```

## Database Schema

### preview_cache Table
```sql
CREATE TABLE preview_cache (
  -- Composite key
  identifier TEXT NOT NULL,
  file_name TEXT NOT NULL,
  
  -- Preview metadata
  preview_type TEXT NOT NULL,  -- 'text', 'image', 'audio', 'video'
  cached_at INTEGER NOT NULL,
  file_size INTEGER NOT NULL,
  
  -- Preview content (one will be non-null)
  text_content TEXT,           -- For text previews
  preview_data BLOB,           -- For binary previews (images, thumbnails)
  
  -- Constraints
  PRIMARY KEY (identifier, file_name)
);

-- Indexes
CREATE INDEX idx_preview_cache_identifier 
  ON preview_cache(identifier);
CREATE INDEX idx_preview_cache_cached_at 
  ON preview_cache(cached_at DESC);
CREATE INDEX idx_preview_cache_type 
  ON preview_cache(preview_type);
```

## Integration with Existing System

### Metadata Cache Integration
- Preview cache shares database with metadata cache
- Use same `DatabaseHelper` singleton
- Coordinate cache management in settings
- Combined cache statistics

### LocalArchiveStorage Integration
- Downloaded files don't need preview caching
- Check local storage before generating preview
- Prioritize local files over cached previews

### Settings Integration
- Add "Preview Cache" section to settings
- Show preview cache size
- Clear preview cache option
- Configure preview size limits
- Toggle auto-preview for types

## Performance Considerations

### Memory Management
- Dispose images after preview closed
- Limit concurrent preview generations
- Stream large text files instead of loading all
- Use pagination for long documents

### Network Efficiency
- Range requests for partial downloads
- HTTP caching headers
- Parallel preview generation (max 3)
- Cancel pending requests on navigation

### Storage Management
- Limit preview cache size (default: 100MB)
- LRU eviction for old previews
- Compress images before caching
- Don't cache very large files

## Testing Strategy

### Unit Tests
- FilePreviewService methods
- Preview generation logic
- Cache operations
- Size threshold decisions

### Widget Tests
- TextPreviewWidget rendering
- ImagePreviewWidget zoom/pan
- Navigation between previews
- Dialog interactions

### Integration Tests
- Full preview workflow
- Cache persistence
- Offline preview access
- Settings integration

### Manual Tests
- Various file types
- Large file handling
- Network error scenarios
- UI responsiveness

## Success Criteria

### Functional Requirements
- ✅ Text files preview correctly
- ✅ Images display and zoom
- ✅ Large files prompt download
- ✅ Previews cache properly
- ✅ Offline previews work
- ✅ Settings allow management

### Performance Requirements
- ✅ Preview loads in <500ms (cached)
- ✅ Preview loads in <3s (network)
- ✅ Smooth zoom/pan (60fps)
- ✅ No memory leaks
- ✅ Cache size stays within limits

### UX Requirements
- ✅ Intuitive navigation
- ✅ Clear status indicators
- ✅ Helpful error messages
- ✅ Consistent design
- ✅ Accessible controls

## Risks & Mitigation

### Risk 1: Large File Memory Issues
**Mitigation**: Strict size limits, streaming, compression

### Risk 2: Format Detection Errors
**Mitigation**: MIME type + extension check, fallback to download

### Risk 3: Cache Bloat
**Mitigation**: Size limits, LRU eviction, user control

### Risk 4: Network Failures
**Mitigation**: Retry logic, cached fallback, clear errors

## Timeline

### Week 1: Foundation
- Days 1-2: Service & database
- Days 3-4: Text preview
- Days 5-7: Image preview

### Week 2: Polish
- Days 1-3: Full-screen UI
- Days 4-5: Navigation
- Days 6-7: Large file handling

### Week 3: Multimedia (Optional)
- Days 1-3: Audio
- Days 4-5: Video
- Days 6-7: Testing

### Week 4: Finalization (Optional)
- Days 1-2: PDF & archives
- Days 3-4: Settings
- Days 5-7: Testing & docs

## Getting Started

### Step 1: Create Plan Document ✅
This document serves as the implementation plan.

### Step 2: Foundation Setup
1. Create `FilePreviewService`
2. Extend `DatabaseHelper` with preview table
3. Create `FilePreview` model
4. Add dependencies to `pubspec.yaml`

### Step 3: MVP Implementation
1. Implement text preview
2. Implement image preview
3. Add preview buttons to UI
4. Test basic workflow

Ready to begin? Let's start with Phase 1: Foundation! 🚀
