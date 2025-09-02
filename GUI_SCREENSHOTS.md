# ia-get GUI Interface Mockups

**Note: These are ASCII art mockups showing the planned GUI interface design, not actual screenshots.**

## Main Window Layout

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ File   Help                                      ia-get - Internet Archive  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│ [Download] [Filters] [Settings] [History]                                  │
│ ─────────────────────────────────────────────────────────────────────────── │
│                                                                             │
│ 📥 Download from Archive.org                                               │
│                                                                             │
│ Archive Identifier/URL:  [_________________________________] 🔍            │
│ Output Directory:        [/home/user/Downloads/________] [Browse...]        │
│                                                                             │
│ ☐ Verbose output    ☐ Resume downloads    ☐ Dry run (preview only)        │
│                                                                             │
│ Concurrent Downloads: [●─────────] 4                                       │
│                                                                             │
│                    [Download]  [Cancel]                                    │
│                                                                             │
│ ─────────────────────────────────────────────────────────────────────────── │
│ 📊 Download Progress:                                                       │
│ [████████████████████████████████████████████████] 100% (15/15 files)     │
│ Current: example_file.pdf                                                  │
│ Speed: 5.2 MB/s  |  ETA: 00:02:45  |  Failed: 0                          │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│ 🔄 Downloading... archive.org/details/example-archive                      │
│                                                     ia-get v1.5.0          │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Filters Tab

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ 🔍 File Filters                                                            │
│                                                                             │
│ Quick Presets:                                                              │
│ [Documents] [Images] [Audio] [Video] [Software]                           │
│ Selected: Documents - Common document formats                              │
│                                                                             │
│ ─────────────────────────────────────────────────────────────────────────── │
│ Manual Filters:                                                             │
│                                                                             │
│ Include formats:  [pdf,txt,doc,docx,odt_______] (comma-separated)         │
│ Exclude formats:  [________________________] (comma-separated)             │
│ Min file size:    [1MB________________] (e.g., 1MB, 500KB)                │
│ Max file size:    [100MB______________] (e.g., 100MB, 2GB)                │
│                                                                             │
│                 [Apply Filters]  [Clear All Filters]                      │
│                                                                             │
│ ─────────────────────────────────────────────────────────────────────────── │
│ Current Filter Settings:                                                    │
│ Include: pdf,txt,doc,docx,odt                                             │
│ Max size: 100MB                                                           │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Settings Tab

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ ⚙️ Application Settings                                                     │
│                                                                             │
│ Download Settings                                                           │
│ ┌─────────────────────────────────────────────────────────────────────────┐ │
│ │ Max Concurrent Downloads: [●──────────] 8                              │ │
│ │ Max Retries:              [●──] 3                                      │ │
│ │ HTTP Timeout (seconds):   [●──────] 30                                 │ │
│ └─────────────────────────────────────────────────────────────────────────┘ │
│                                                                             │
│ Default Behavior                                                            │
│ ┌─────────────────────────────────────────────────────────────────────────┐ │
│ │ ☑ Resume downloads by default                                          │ │
│ │ ☐ Verbose output by default                                            │ │
│ │ ☐ Log hash errors by default                                           │ │
│ │ ☐ Dry run mode by default                                              │ │
│ └─────────────────────────────────────────────────────────────────────────┘ │
│                                                                             │
│ Advanced Settings                                                           │
│ ┌─────────────────────────────────────────────────────────────────────────┐ │
│ │ User Agent Override: [________________________________]                │ │
│ └─────────────────────────────────────────────────────────────────────────┘ │
│                                                                             │
│ Filter Presets                                                              │
│ ┌─────────────────────────────────────────────────────────────────────────┐ │
│ │ Documents - Common document formats                    [Remove]        │ │
│ │ Images - Image files only                              [Remove]        │ │
│ │ ─────────────────────────────────────────────────────────────────────── │ │
│ │ Add New Preset:                                                         │ │
│ │ Name: [Custom_____] Description: [My preset_______]                    │ │
│ │ Include: [pdf,txt___] Exclude: [___] Max size: [50MB_]  [Add Preset]  │ │
│ └─────────────────────────────────────────────────────────────────────────┘ │
│                                                                             │
│              [Save Configuration]  [Reset to Defaults]                     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## History Tab

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ 📚 Recent Downloads                                                         │
│                                                                             │
│ [📁] commute_test                                                      [❌] │
│ [📁] example-archive-2023                                             [❌] │
│ [📁] https://archive.org/details/software-collection                  [❌] │
│ [📁] vintage-computer-manuals                                         [❌] │
│ [📁] public-domain-books                                              [❌] │
│                                                                             │
│                                                                             │
│                           [Clear History]                                  │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Error Dialog

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│         ┌─────────────────── Error ────────────────────┐                   │
│         │                                              │                   │
│         │ ❌ Failed to connect to archive.org          │                   │
│         │                                              │                   │
│         │ Please check your internet connection and    │                   │
│         │ try again.                                   │                   │
│         │                                              │                   │
│         │                        [OK]                 │                   │
│         └──────────────────────────────────────────────┘                   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Features Illustrated

### 1. **Cross-Platform Design**
- Native look and feel on Windows, macOS, and Linux
- Consistent behavior across all platforms
- Proper window management and system integration

### 2. **User-Friendly Interface**
- Clear visual hierarchy with icons and sections
- Helpful tooltips and placeholder text
- Logical tab organization

### 3. **Real-Time Feedback**
- Progress bars with percentage and file counts
- Status messages in bottom bar
- Error dialogs with helpful suggestions

### 4. **Configuration Management**
- Visual sliders for numeric settings
- Checkboxes for boolean options
- Preset management for common configurations

### 5. **Modern Design Language**
- Clean, minimalist interface
- Consistent spacing and typography
- Accessible color scheme with dark theme support

This GUI provides the complete ia-get experience in a visual, user-friendly format while maintaining all the power and flexibility of the command-line version.