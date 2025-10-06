# Phase 3: Advanced Media Support - PLAN

**Start Date**: October 6, 2025  
**Status**: 📋 **PLANNING**  
**Estimated Duration**: 3-4 days  
**Priority**: High

---

## 🎯 Phase 3 Objectives

Expand the Content Preview System to support audio, video, and archive formats - completing comprehensive media support for Internet Archive content.

---

## 📊 Current Status (Phase 1 + 2)

### ✅ Supported Formats (46 formats)
- **Text**: 39 formats (JSON, XML, Python, JS, etc.) with syntax highlighting
- **Images**: 7 formats (JPEG, PNG, GIF, WebP, BMP, TIFF, ICO) with zoom/pan
- **Markdown**: Rendered preview with formatting

### ✅ Existing Features
- Swipe navigation between files
- Smart caching (SQLite database)
- Large file handling (>5MB prompt)
- Offline availability indicators
- Share functionality
- Professional error handling

### ⏳ Format Gaps (Phase 3 Target)
- **Audio**: MP3, WAV, OGG, FLAC, M4A, AAC, OPUS
- **Video**: MP4, WebM, MKV, AVI, MOV
- **Archives**: ZIP, TAR, GZ, BZ2, XZ, RAR, 7Z
- **Documents**: PDF (high priority)

---

## 🎯 Phase 3 Tasks (4 Tasks)

### Task 1: PDF Document Preview 🔴 **HIGH PRIORITY**
**Estimated Time**: 1 day  
**Package**: `pdfx: ^2.7.0` or `flutter_pdfview: ^1.3.2`

**Features**:
- Render PDF pages with native quality
- Page navigation (previous/next, jump to page)
- Page indicator ("Page 3 of 24")
- Zoom support (pinch, double-tap)
- Rotation support
- Swipe between pages
- Performance optimization (render visible pages only)

**Considerations**:
- `pdfx`: More modern, null-safe, better performance
- `flutter_pdfview`: Mature, stable, larger community
- **Recommendation**: Use `pdfx` for modern API and performance

**Files to Create/Modify**:
- `lib/widgets/pdf_preview_widget.dart` (new)
- `lib/services/file_preview_service.dart` (update MIME detection)
- `lib/widgets/preview_dialog.dart` (add PDF case)
- `pubspec.yaml` (add pdfx dependency)

**Edge Cases**:
- Password-protected PDFs (show error with message)
- Corrupted PDFs (graceful error)
- Large PDFs (>100 pages - pagination)
- Memory management (dispose controllers)

---

### Task 2: Audio File Preview 🟠 **MEDIUM PRIORITY**
**Estimated Time**: 1 day  
**Package**: `just_audio: ^0.9.40`

**Supported Formats**:
- MP3, WAV, OGG, FLAC, M4A, AAC, OPUS, WMA
- Platform-dependent (iOS/Android codecs)

**Features**:
- Playback controls (play, pause, stop)
- Seek bar with current position
- Duration display (00:00 / 03:45)
- Volume control
- Loop toggle
- Playback speed (0.5x, 1x, 1.5x, 2x)
- Background playback support (optional)
- Waveform visualization (optional, advanced)

**UI Components**:
- Large play/pause button (center)
- Progress slider with timestamp labels
- Skip forward/backward buttons (±10s)
- Volume slider
- Metadata display (title, artist, album - if available)
- Cover art (if embedded in audio file)

**Files to Create/Modify**:
- `lib/widgets/audio_preview_widget.dart` (new)
- `lib/services/audio_service.dart` (new - audio state management)
- `lib/services/file_preview_service.dart` (update MIME detection)
- `lib/widgets/preview_dialog.dart` (add audio case)
- `pubspec.yaml` (add just_audio)

**Edge Cases**:
- Streaming vs. downloaded (show buffering indicator)
- Unsupported codecs (graceful error message)
- Corrupted audio files (error handling)
- Permission handling (audio playback notification)

---

### Task 3: Video File Preview 🟠 **MEDIUM PRIORITY**
**Estimated Time**: 1 day  
**Package**: `video_player: ^2.9.2` or `chewie: ^1.8.5`

**Supported Formats**:
- MP4, WebM, MKV, AVI, MOV, FLV
- Platform codecs (H.264, VP8, VP9)

**Features**:
- Full-screen video playback
- Play/pause controls
- Seek bar with thumbnail preview (optional)
- Duration and current time display
- Volume control
- Playback speed control
- Fullscreen toggle
- Quality selection (if multiple sources)
- Subtitle support (if available)

**UI Components**:
- Video surface (aspect ratio preserved)
- Overlay controls (auto-hide after 3s)
- Bottom control bar (play, seek, time, fullscreen)
- Loading indicator (buffering)
- Error overlay (if playback fails)

**Considerations**:
- `video_player`: Low-level, requires custom UI
- `chewie`: High-level wrapper with built-in controls
- **Recommendation**: Use `chewie` for faster development

**Files to Create/Modify**:
- `lib/widgets/video_preview_widget.dart` (new)
- `lib/services/video_service.dart` (new - playback state)
- `lib/services/file_preview_service.dart` (update MIME detection)
- `lib/widgets/preview_dialog.dart` (add video case)
- `pubspec.yaml` (add chewie, video_player)

**Edge Cases**:
- Large video files (>50MB - streaming vs. download)
- Unsupported codecs (fallback to download)
- Network buffering (show progress)
- Background playback (pause when minimized)
- Memory management (dispose on exit)

---

### Task 4: Archive File Preview 🟡 **MEDIUM-LOW PRIORITY**
**Estimated Time**: 1 day  
**Package**: `archive: ^4.0.7` (already included!)

**Supported Formats**:
- ZIP, TAR, GZ, BZ2, XZ, 7Z
- Nested archives (archive within archive)

**Features**:
- List archive contents (file tree)
- File metadata (name, size, modified date)
- Directory structure (expandable tree)
- File count and total size summary
- Search/filter files within archive
- Preview individual files (text, image, etc.)
- Extract single file (to temp, then preview)
- Extract all files (confirm prompt)
- Progress indicator for large archives

**UI Components**:
- Tree view or flat list (toggle)
- File icons (folder, document, image, etc.)
- Sort options (name, size, date)
- Header with archive summary
- Action buttons (Extract All, Close)
- Context menu per file (Preview, Extract)

**Files to Create/Modify**:
- `lib/widgets/archive_preview_widget.dart` (new)
- `lib/services/archive_service.dart` (new - extraction logic)
- `lib/models/archive_entry.dart` (new - data model)
- `lib/services/file_preview_service.dart` (update MIME detection)
- `lib/widgets/preview_dialog.dart` (add archive case)

**Edge Cases**:
- Password-protected archives (error message)
- Corrupted archives (partial extraction)
- Large archives (>1000 files - pagination/virtual scrolling)
- Nested archives (recursive extraction limit)
- Memory management (stream extraction)

---

## 📦 Dependencies to Add

```yaml
dependencies:
  # PDF rendering
  pdfx: ^2.7.0  # Modern PDF viewer with zoom and navigation
  
  # Audio playback
  just_audio: ^0.9.40  # Cross-platform audio player
  
  # Video playback
  chewie: ^1.8.5  # Video player with controls
  video_player: ^2.9.2  # Core video player (chewie dependency)
  
  # Archive is already included!
  archive: ^4.0.7  # ZIP, TAR, etc. support ✅
```

**Total New Dependencies**: 3 packages  
**Already Included**: 1 package (archive)

---

## 🎯 Success Criteria

### Functionality
- [x] PDF documents render with page navigation
- [x] Audio files play with controls
- [x] Video files play with controls
- [x] Archive contents display and extract
- [x] All formats integrate with swipe navigation
- [x] All formats integrate with caching system
- [x] All formats integrate with sharing system

### Quality
- [x] 0 compilation errors
- [x] 0 warnings
- [x] <30 info suggestions (linter style)
- [x] All edge cases handled gracefully
- [x] Professional error messages

### Performance
- [x] PDF renders smoothly (60 FPS)
- [x] Audio playback is seamless
- [x] Video playback is smooth (30+ FPS)
- [x] Archive listing is responsive (<1s for <1000 files)
- [x] Memory usage is acceptable (<100MB for typical files)

### Documentation
- [x] Each task documented (phase-3-task-N-complete.md)
- [x] Phase 3 summary created (phase-3-complete.md)
- [x] Code comments added for complex logic
- [x] README updated with new format support

---

## 🚧 Implementation Order

### Recommended Sequence
1. **PDF Preview** (Day 1)
   - High priority, high value
   - Simpler than audio/video (no playback state)
   - Tests widget integration patterns

2. **Audio Preview** (Day 2)
   - Medium complexity
   - Introduces playback controls
   - Tests state management patterns

3. **Video Preview** (Day 3)
   - Similar to audio but more complex
   - Reuses control patterns from audio
   - Tests performance with larger files

4. **Archive Preview** (Day 4)
   - Most complex (tree view, extraction)
   - Benefits from lessons learned in tasks 1-3
   - Ties everything together (preview files within archives)

---

## 📊 Format Coverage After Phase 3

### Before Phase 3: 46 formats
- Text: 39 formats
- Images: 7 formats

### After Phase 3: 70+ formats
- **Text**: 39 formats ✅
- **Images**: 7 formats ✅
- **PDF**: 1 format (NEW)
- **Audio**: 8+ formats (NEW)
- **Video**: 8+ formats (NEW)
- **Archives**: 7+ formats (NEW)

**Total**: 70+ formats supported! 🎉

---

## 🔄 Integration with Existing Features

### Swipe Navigation
- All new previews integrate with PageView
- File indicator updates for all types
- Adjacent file preloading works across formats

### Caching System
- Audio: Cache metadata (duration, bitrate)
- Video: Cache first frame thumbnail
- PDF: Cache page count, first page thumbnail
- Archives: Cache file list and structure

### Sharing
- PDF: Share as file (native share)
- Audio: Share file or streaming link
- Video: Share file or streaming link
- Archive: Share entire archive or extracted files

### Error Handling
- All formats use context-aware error messages
- Each format has specific troubleshooting tips
- Technical details in collapsible ExpansionTile

---

## 🎓 Learning Objectives

### Technical Skills
1. Media playback state management
2. Native platform integration (codecs, permissions)
3. Memory optimization for large files
4. Streaming vs. download strategies
5. Tree view and list virtualization

### Package Integration
1. PDF rendering engines
2. Audio players (just_audio ecosystem)
3. Video players (chewie + video_player)
4. Archive extraction (already familiar with archive package)

---

## 📝 Documentation Plan

### Per-Task Documentation
- `docs/features/phase-3-task-1-complete.md` - PDF preview
- `docs/features/phase-3-task-2-complete.md` - Audio preview
- `docs/features/phase-3-task-3-complete.md` - Video preview
- `docs/features/phase-3-task-4-complete.md` - Archive preview

### Phase Summary
- `docs/features/phase-3-complete.md` - Overall summary
- Update `docs/features/phase-2-complete.md` with Phase 3 link
- Update main `README.md` with new format support

---

## 🚀 Next Steps

1. **Review and Approve Plan** (5 minutes)
   - User reviews this plan
   - Confirms task priorities
   - Approves dependency choices

2. **Begin Task 1: PDF Preview** (Day 1)
   - Add pdfx dependency
   - Create PDF preview widget
   - Integrate with preview dialog
   - Test with various PDF files
   - Document completion

3. **Continue Through Tasks 2-4** (Days 2-4)
   - Follow recommended sequence
   - Test each task thoroughly
   - Document as you go

4. **Phase 3 Completion** (End of Day 4)
   - Run comprehensive tests
   - Create phase summary
   - Update main documentation
   - Celebrate! 🎉

---

## 💡 Alternative: Phased Approach

If 4 tasks feel overwhelming, consider:

### Phase 3A: Documents (1-2 days)
- Task 1: PDF Preview
- Task 4: Archive Preview

### Phase 3B: Media (2-3 days)
- Task 2: Audio Preview
- Task 3: Video Preview

**Benefit**: Faster incremental value, easier testing

---

## ✅ Ready to Begin?

**Prerequisites Complete**:
- ✅ Phase 2 finished (5/5 tasks)
- ✅ Pre-Phase 3 cleanup complete
- ✅ 0 errors, 0 warnings
- ✅ Build system modernized
- ✅ Dependencies optimized

**Current Status**:
- 📋 Plan created and ready for review
- 🎯 Tasks prioritized and estimated
- 📦 Dependencies researched and selected
- 📚 Documentation structure planned

**Awaiting**: User approval to begin Phase 3 Task 1 (PDF Preview) 🚀

---

**Plan Created**: October 6, 2025  
**Status**: Awaiting approval  
**Estimated Completion**: October 9-10, 2025 (4 days)
