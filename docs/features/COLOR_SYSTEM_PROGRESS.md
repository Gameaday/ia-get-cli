# Color System Compliance - In Progress ⏳

**Status**: 🔄 In Progress - Fixing hardcoded Colors.* violations  
**Target**: Replace with ColorScheme values for proper theme support  
**Priority**: High-impact files first (video_preview, preview_dialog, pdf_preview)

---

## Audit Summary

### Total Violations Found
- **Total Files**: 15 files with hardcoded Colors.*
- **Total Violations**: ~120 hardcoded color references
- **High Priority**: 3 files (video_preview, preview_dialog, pdf_preview)
- **Medium Priority**: 4 files (file_list, image_preview, filter_controls, enhanced_progress)
- **Low Priority**: 8 files (minor violations, mostly theme-compliant)

---

## Color System Strategy

### Material Design 3 ColorScheme Properties

```dart
// Primary colors
colorScheme.primary          // Main brand color
colorScheme.onPrimary        // Text/icons on primary
colorScheme.primaryContainer // Muted primary background
colorScheme.onPrimaryContainer // Text on primary container

// Secondary colors
colorScheme.secondary
colorScheme.onSecondary
colorScheme.secondaryContainer
colorScheme.onSecondaryContainer

// Tertiary colors
colorScheme.tertiary
colorScheme.onTertiary
colorScheme.tertiaryContainer
colorScheme.onTertiaryContainer

// Error colors
colorScheme.error            // Error state (replaces Colors.red)
colorScheme.onError
colorScheme.errorContainer
colorScheme.onErrorContainer

// Surface colors
colorScheme.surface          // Card/dialog backgrounds
colorScheme.onSurface        // Text on surface
colorScheme.surfaceVariant   // Subtle backgrounds
colorScheme.onSurfaceVariant // Subtle text

// Outline colors
colorScheme.outline          // Borders (replaces Colors.grey)
colorScheme.outlineVariant   // Subtle borders

// Background colors
colorScheme.background       // App background (deprecated in MD3, use surface)
colorScheme.onBackground
```

---

## Replacement Mapping

### Common Color Replacements

| Hardcoded Color | MD3 Replacement | Use Case |
|----------------|-----------------|----------|
| `Colors.white` | `colorScheme.surface` or `onPrimary` | Backgrounds, text on dark |
| `Colors.black` | `colorScheme.onSurface` | Text, icons |
| `Colors.black87` | `colorScheme.onSurface` | Primary text |
| `Colors.black54` | `colorScheme.onSurfaceVariant` | Secondary text |
| `Colors.grey` | `colorScheme.outline` | Borders, dividers |
| `Colors.grey.shade600` | `colorScheme.onSurfaceVariant` | Subtle text |
| `Colors.red` | `colorScheme.error` | Error states, delete actions |
| `Colors.blue` | `colorScheme.primary` | Primary actions, links |
| `Colors.orange` | `colorScheme.tertiary` | Warning states |
| `Colors.green` | `colorScheme.secondary` or custom | Success states |

---

## Files by Priority

### 🔴 High Priority (Most Violations)

#### 1. **video_preview_widget.dart** (15 violations)
- Lines: 89-91, 95, 107, 114, 122, 160, 194, 201, 218, 225, 232, 242-243, 280, 287, 300, 306, 316, 322, 334, 348, 355
- Issues: Video controls, overlay colors, progress bar colors
- **Status**: ⏳ Pending

#### 2. **preview_dialog.dart** (21 violations)
- Lines: 209, 237, 356, 371, 377, 382, 461, 524, 533, 539, 551, 556, 582, 598, 616, 632, 647, 688, 706, 708, 716, 733, 766-767, 792, 797, 803, 914
- Issues: Dialog backgrounds, info cards, error states
- **Status**: ⏳ Pending

#### 3. **pdf_preview_widget.dart** (10 violations)
- Lines: 186, 192, 212-213, 224, 239, 245, 254, 279, 285
- Issues: Overlay controls, page indicators
- **Status**: ⏳ Pending

### 🟡 Medium Priority

#### 4. **file_list_widget.dart** (29 violations)
- Lines: Multiple instances of status colors, delete actions
- Issues: File type colors, status badges, delete buttons
- **Status**: ⏳ Pending

#### 5. **image_preview_widget.dart** (7 violations)
- Lines: 53, 92, 104, 117, 125, 133, 139
- Issues: Image viewer controls, overlays
- **Status**: ⏳ Pending

#### 6. **filter_controls_widget.dart** (3 violations)
- Lines: 48, 63, 81
- Issues: Filter badges, clear button
- **Status**: ⏳ Pending

#### 7. **enhanced_progress_card.dart** (7 violations)
- Lines: 44 (×2), 59, 65, 72, 78, 104, 136
- Issues: Progress indicators, status colors
- **Status**: ⏳ Pending

### 🟢 Low Priority (Minor Violations)

8. **download_manager_widget.dart** (1 violation)
9. **search_bar_widget.dart** (1 violation)
10. **animation_constants.dart** (1 violation)
11. **main.dart** (1 violation)

---

## Implementation Plan

### Phase 1: High-Impact Files (Current)
1. ✅ Update `video_preview_widget.dart` - Video player colors
2. ✅ Update `preview_dialog.dart` - Dialog and error colors
3. ✅ Update `pdf_preview_widget.dart` - PDF viewer colors

### Phase 2: Medium-Impact Files
4. Update `file_list_widget.dart` - File status colors
5. Update `image_preview_widget.dart` - Image viewer colors
6. Update `filter_controls_widget.dart` - Filter UI colors
7. Update `enhanced_progress_card.dart` - Progress indicators

### Phase 3: Low-Impact Files
8. Update remaining files with minor violations

### Phase 4: Verification
- Run `flutter analyze`
- Test light/dark theme switching
- Verify color consistency across app

---

## Current Progress

- ⏳ **Phase 1**: In Progress (0/3 files)
- ⏸️ **Phase 2**: Not Started (0/4 files)
- ⏸️ **Phase 3**: Not Started (0/8 files)
- ⏸️ **Phase 4**: Not Started

**Overall Progress**: 0% (0/15 files)

---

## Next Steps

1. Start with `video_preview_widget.dart` (most violations)
2. Continue to `preview_dialog.dart`
3. Complete `pdf_preview_widget.dart`
4. Verify with `flutter analyze`
5. Test theme switching

---

**Note**: This is a complex refactoring that requires careful attention to context. Each color replacement must consider its semantic meaning (error, warning, success, neutral) and its usage context (text, background, border, icon).
