# Deep Link Support

The Internet Archive Helper mobile app fully supports deep links, allowing you to open Internet Archive content directly from other apps, web browsers, or shared links.

## Supported URL Formats

The app recognizes and handles the following URL formats:

### 1. Internet Archive URLs

- **Details page**: `https://archive.org/details/[identifier]`
  - Example: `https://archive.org/details/commute_test`
  
- **Download page**: `https://archive.org/download/[identifier]`
  - Example: `https://archive.org/download/nasa_images`
  
- **Metadata API**: `https://archive.org/metadata/[identifier]`
  - Example: `https://archive.org/metadata/commute_test`

### 2. Custom App Scheme

- **Direct identifier**: `iaget://[identifier]`
  - Example: `iaget://commute_test`

## How It Works

1. **App Launch**: When you tap a supported link while the app is closed, the app will launch and automatically load the archive metadata
2. **Running App**: When you tap a supported link while the app is running, it will navigate to the archive detail screen
3. **Initial Link**: The app checks for deep links on startup and processes them automatically

## Using Deep Links

### From a Web Browser

Simply tap any Internet Archive details, download, or metadata URL. Your device will offer to open the link in Internet Archive Helper.

### From Other Apps

Any app that shares Internet Archive links (email, messaging apps, social media) can open them directly in Internet Archive Helper.

### Creating Custom Links

Developers can create custom deep links using the `iaget://` scheme:

```
iaget://commute_test
```

This will open the archive with identifier `commute_test` in the app.

## Configuration

Deep link support is automatically enabled when the app is installed. No additional configuration is required.

### Android Configuration

The app registers the following intent filters in `AndroidManifest.xml`:

- **HTTPS URLs**: `archive.org` domain with `/details/`, `/download/`, and `/metadata/` paths
- **Custom Scheme**: `iaget://` for direct archive access

### iOS Configuration (Future)

iOS support will use Universal Links with associated domains for `archive.org`.

## Troubleshooting

### Link Doesn't Open the App

1. **Check URL format**: Ensure the URL matches one of the supported formats
2. **Reinstall the app**: Sometimes the system needs to re-register the app's intent filters
3. **Clear default handlers**: In Android Settings → Apps → Internet Archive Helper → Open by default, clear any conflicting handlers

### App Opens but Shows Error

1. **Check internet connection**: Deep links require network access to fetch archive metadata
2. **Verify identifier**: Ensure the archive identifier exists on archive.org
3. **Check logs**: Look for deep link errors in the app's debug output

## Technical Details

### Implementation

Deep link handling is implemented in `/mobile/flutter/lib/services/deep_link_service.dart`:

- Uses the `app_links` package for cross-platform deep link support
- Initializes on app startup with timeout protection
- Extracts archive identifiers from various URL formats
- Integrates with `ArchiveService` to fetch and display metadata

### Error Handling

- **Timeout protection**: Initial link retrieval times out after 3 seconds to prevent hanging
- **Invalid URLs**: Gracefully handles malformed URLs without crashing
- **Missing identifiers**: Shows appropriate error messages when archive is not found

## Examples

### Opening an Archive from Chrome

1. Browse to `https://archive.org/details/commute_test` in Chrome
2. Tap the link
3. Select "Internet Archive Helper" from the app chooser
4. The app opens and displays the archive details

### Sharing an Archive

1. In any app, share an Internet Archive URL
2. Select "Internet Archive Helper" from the share menu
3. The app opens and loads the archive

### Creating a QR Code

Generate a QR code with the URL `iaget://commute_test` to provide quick access to archives in physical spaces or printed materials.
