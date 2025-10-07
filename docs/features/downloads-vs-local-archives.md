# Downloads Page vs Local Archives

The Internet Archive Helper app has distinct features for tracking downloads and managing local archives.

## Downloads Page

**Location**: Main menu → Downloads icon (top-right)

**Purpose**: Track **active** and **recently completed** downloads in real-time

**Features**:
- View currently downloading files with progress bars
- Monitor download speed, ETA, and bandwidth usage
- See queued downloads waiting to start
- Control download priority (high/normal/low)
- Cancel or pause downloads
- Access download settings (bandwidth limits, concurrent downloads)

**What You'll See**:
- **Active Downloads**: Files currently being downloaded with live progress
- **Completed Downloads**: Recently finished downloads that can be opened or cleared
- **Download Controls**: Bandwidth presets, speed display, ETA calculations

**Use Cases**:
- Monitor ongoing downloads
- Adjust bandwidth limits
- Change download priorities
- View download history (recent session only)

## Local Archives (Offline Content)

**Location**: Built into search suggestions and archive detail pages

**Purpose**: Track **permanently saved** archives available offline

**Features**:
- Green "Downloaded" badges on archive cards
- Check mark indicators on archive icons
- "Previously Downloaded" labels on detail pages
- Access downloaded files through device file manager

**What You'll See**:
- Visual indicators on archives you've downloaded before
- Quick identification of locally available content
- No need for internet to identify what's downloaded

**Use Cases**:
- Quickly identify archives you've already downloaded
- Know which content is available offline
- Avoid re-downloading the same content

## Key Differences

| Feature | Downloads Page | Local Archives |
|---------|---------------|----------------|
| **Purpose** | Track active downloads | Show downloaded content |
| **Scope** | Current session + recent | Permanent until deleted |
| **Access** | Top-right download icon | Integrated into archive cards |
| **Actions** | Cancel, control, monitor | View indicators only |
| **Persistence** | Clears when completed | Stays until manually removed |

## Why Two Systems?

### Downloads Page
- **Real-time monitoring**: See what's happening right now
- **Active control**: Pause, cancel, prioritize ongoing transfers
- **Performance tuning**: Adjust bandwidth and concurrent downloads
- **Session history**: Quick access to recently downloaded files

### Local Archives
- **Quick reference**: Instantly see what you've downloaded before
- **Offline awareness**: Know what's available without internet
- **Persistent tracking**: Doesn't disappear when downloads complete
- **Visual cues**: Green badges and check marks throughout the app

## How They Work Together

1. **Start Download**: Use archive detail page to begin download
2. **Monitor Progress**: Switch to Downloads page to watch progress
3. **Download Completes**: File shows in "Completed Downloads"
4. **Clear from Downloads**: Remove from downloads page when done
5. **Archive Marked**: Archive now shows "Downloaded" badge permanently
6. **Find Later**: Search for same archive, see green indicator
7. **Access Files**: Open from device Downloads folder

## Finding Downloaded Files

Downloaded files are stored in your device's Downloads folder:

```
/storage/emulated/0/Download/ia-get/[archive-identifier]/
```

You can access them through:
- Any file manager app
- Downloads page → "Open folder" button
- Device's built-in Files app

## Settings

Both systems are configured through the Settings menu:

### Downloads Settings
- **Download Location**: Where files are saved
- **Concurrent Downloads**: How many files at once
- **Auto-decompress**: Extract archives automatically
- **Verify Checksums**: Ensure file integrity

### Cache Settings
- **Offline Cache**: Store metadata for offline access
- **Pin Archives**: Keep metadata permanently
- **Sync Frequency**: How often to update metadata

## Best Practices

### For Active Downloading
1. Use **Downloads page** to monitor and control
2. Adjust bandwidth limits if download is too slow/fast
3. Change priorities if multiple downloads are queued
4. Clear completed downloads to keep the list clean

### For Finding Downloaded Content
1. Look for **green badges** in search results
2. Check **"Previously Downloaded"** label on detail pages
3. Use device file manager to browse downloaded files
4. Don't re-download unless you need to update the content

## Troubleshooting

### "Can't find my downloaded files"
- Check Downloads page → Open folder button
- Navigate to `/Download/ia-get/[identifier]/` in file manager
- Ensure download actually completed (check Downloads page)

### "Download shows as complete but no badge"
- Badge appears after app restart or cache refresh
- Manually refresh the archive metadata
- Check if files actually exist in download folder

### "Downloaded badge shows but files are gone"
- Files may have been deleted from storage
- Use device file manager to verify
- Re-download if needed

## Future Enhancements

Planned features for local archive management:

- **Local Archives Page**: Dedicated page to browse all downloaded content
- **Archive Manager**: Edit, delete, or organize local archives
- **Storage Statistics**: View total storage used by downloads
- **Quick Access**: Recently accessed archives list
- **Offline Search**: Search through downloaded archive metadata

These features are documented in the roadmap and will be added in future updates.
