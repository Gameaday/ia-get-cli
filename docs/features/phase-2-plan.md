# Phase 2: Enhanced UI & Polish - Implementation Plan

**Status**: In Progress 🚧  
**Started**: October 6, 2025  
**Goal**: Deliver a polished, professional preview experience with navigation and enhanced features

---

## 📋 Phase 2 Tasks

### Completed in Phase 1 ✅
1. ✅ Full-screen preview dialog (PreviewDialog)
2. ✅ Zoom/pan for images (photo_view package)
3. ✅ Syntax highlighting for code (flutter_highlight package)
4. ✅ Markdown rendering (flutter_markdown package)

### New Tasks for Phase 2

#### 1. Swipe Navigation Between Files 🎯
**Priority**: HIGH  
**Goal**: Allow users to swipe left/right to navigate between files

**Implementation**:
- Use `PageView` widget to enable swipe gestures
- Load adjacent file previews on demand
- Show file indicator (e.g., "2 of 5")
- Preload next/previous file for smooth transitions

**Files to Create/Modify**:
- Modify `preview_dialog.dart` to use `PageView`
- Add navigation indicators

**Package**: Built-in Flutter `PageView` (zero dependencies)

---

#### 2. Download Prompt for Large Files 🎯
**Priority**: HIGH  
**Goal**: Gracefully handle files too large for preview

**Implementation**:
- Detect large files (>5MB) in `FilePreviewService`
- Show custom dialog with file size and options
- Options: "Download First" or "Cancel"
- Show progress indicator during download

**Files to Create/Modify**:
- Add `LargeFileDialog` widget
- Integrate with download service

**Package**: No additional dependencies needed

---

#### 3. Offline Availability Indicators 🎯
**Priority**: MEDIUM  
**Goal**: Show which files are cached offline

**Implementation**:
- Check cache status in file list
- Show badge/icon for cached files
- Different icon for "available offline"
- Tooltip: "Cached X ago"

**Files to Create/Modify**:
- Modify `file_list_widget.dart` to show cache badges
- Add cache status checks

**Package**: No additional dependencies needed

---

#### 4. Share Preview Feature 🎯
**Priority**: MEDIUM  
**Goal**: Allow users to share previewed content

**Implementation**:
- Add share button to preview dialog AppBar
- Use `share_plus` package for cross-platform sharing
- Share options:
  - Text: Share as plain text
  - Image: Share image file
- Include file name and metadata

**Files to Create/Modify**:
- Modify `preview_dialog.dart` to add share button
- Add share logic for text and images

**Package**: `share_plus: ^10.1.2` (official Flutter plugin)

---

#### 5. Enhanced Text Preview Features 🎯
**Priority**: MEDIUM  
**Goal**: Add power-user features to text preview

**Features**:
- Line numbers (toggleable)
- Font size controls (zoom in/out)
- Word wrap toggle
- Search functionality (Ctrl+F)

**Files to Create/Modify**:
- Enhance `text_preview_widget.dart` with toolbar
- Add search bar widget
- Add settings for text display

**Package**: No additional dependencies needed

---

#### 6. Enhanced Image Preview Features 🎯
**Priority**: LOW  
**Goal**: Add convenience features to image preview

**Features**:
- Image info overlay (dimensions, format, size)
- Rotation controls
- Brightness/contrast adjustment
- Save to gallery

**Files to Create/Modify**:
- Enhance `image_preview_widget.dart` with controls
- Add image manipulation options

**Package**: `image_gallery_saver: ^2.0.3` (for save to gallery)

---

#### 7. Preview Settings Screen 🎯
**Priority**: LOW  
**Goal**: Allow users to customize preview behavior

**Settings**:
- Auto-cache threshold (1MB, 5MB, 10MB)
- Preview quality (low, medium, high)
- Default zoom level
- Theme (light/dark/auto)
- Cache management (clear cache, cache size)

**Files to Create/Modify**:
- Create `preview_settings_screen.dart`
- Use `shared_preferences` for persistence
- Add settings button to main app

**Package**: Already have `shared_preferences: ^2.5.3`

---

#### 8. Loading & Error State Polish 🎯
**Priority**: MEDIUM  
**Goal**: Make loading and error states more user-friendly

**Improvements**:
- Animated loading indicators
- Progress percentage for downloads
- Better error messages with suggestions
- Quick actions (retry, report issue)

**Files to Create/Modify**:
- Enhance `preview_dialog.dart` loading/error states
- Add custom error widgets
- Add animations

**Package**: `flutter_spinkit: ^5.2.2` (already installed)

---

## 🎯 Recommended Implementation Order

### Week 1: Core Navigation & UX (HIGH Priority)
1. **Swipe Navigation** - Most requested feature, enables efficient browsing
2. **Download Prompt** - Critical for handling large files gracefully
3. **Loading/Error Polish** - Improves perceived performance

### Week 2: Convenience Features (MEDIUM Priority)
4. **Share Preview** - High user value, low implementation cost
5. **Offline Indicators** - Improves cache awareness
6. **Enhanced Text Preview** - Power user features

### Week 3: Advanced & Settings (LOW Priority)
7. **Enhanced Image Preview** - Nice-to-have features
8. **Preview Settings** - Configuration and customization

---

## 📦 New Dependencies Needed

```yaml
dependencies:
  # Phase 2 additions
  share_plus: ^10.1.2           # Cross-platform sharing
  image_gallery_saver: ^2.0.3   # Save images to gallery (optional)
```

**Note**: Most Phase 2 features use existing packages or built-in Flutter widgets!

---

## 🎨 UI/UX Improvements

### Navigation Pattern
```
┌─────────────────────────────────┐
│ ← file1.txt    (2/5)    Close ↗ │  ← AppBar with navigation
├─────────────────────────────────┤
│                                 │
│   [Swipe left/right to          │
│    navigate between files]      │
│                                 │
│         Preview Content         │
│                                 │
├─────────────────────────────────┤
│  ◀ Previous    Next ▶           │  ← Bottom navigation (optional)
└─────────────────────────────────┘
```

### Large File Dialog
```
┌─────────────────────────────────┐
│   ⚠️  Large File                │
│                                 │
│   video.mp4 (25.3 MB)          │
│                                 │
│   This file is too large       │
│   to preview directly.         │
│                                 │
│   [ Download First ]           │
│   [     Cancel     ]           │
└─────────────────────────────────┘
```

### Cache Badge
```
File List Item:
┌─────────────────────────────────┐
│ ☑ document.pdf    📄 [CACHED]  │  ← Badge shows cache status
│   1.2 MB                        │
└─────────────────────────────────┘
```

---

## 🎯 Success Criteria

### Phase 2 Complete When:
- ✅ Users can swipe between files in preview
- ✅ Large files show appropriate prompts
- ✅ Cached files are visually indicated
- ✅ Text and images can be shared
- ✅ Loading states are polished
- ✅ Error messages are helpful
- ⏳ Optional: Enhanced text/image features
- ⏳ Optional: Settings screen

---

## 📝 Notes

### Package Selection Philosophy
Continuing Phase 1's "package-first" approach:
- Use `share_plus` (official plugin) instead of custom sharing
- Use built-in `PageView` instead of custom gesture handling
- Use existing `flutter_spinkit` for animations
- Minimize new dependencies

### Testing Strategy
For each feature:
1. Test on iOS and Android
2. Test with various file types
3. Test offline scenarios
4. Test with large file collections
5. Test gesture interactions

### Performance Considerations
- Lazy load adjacent previews (don't load all files)
- Dispose of unused preview widgets
- Monitor memory usage during navigation
- Cache preview states for quick navigation

---

**Next Step**: Implement Task 1 (Swipe Navigation) 🚀
