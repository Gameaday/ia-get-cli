# Mobile App - README

This directory contains the Flutter mobile application.

## Overview

The mobile app uses a **pure Dart implementation** for accessing Internet Archive content. This provides:
- Cross-platform compatibility (Android, iOS, Web, Desktop)
- Standard Flutter development workflow
- No native build dependencies
- Fast build times without native compilation

## Architecture

```
Flutter/Dart App
    ↓ (HTTP API)
Internet Archive JSON API
```

### Key Design Principles

1. **Pure Dart Implementation**: All functionality is implemented in Dart, making it portable and easy to maintain.

2. **Standard Flutter Architecture**: Uses standard Flutter packages and follows Flutter best practices.

3. **API-First Design**: Direct communication with Internet Archive's JSON API using Dart's http package.

## Building

### Development Build
```bash
cd mobile/flutter
flutter pub get
flutter run
```

### Android APK Build
```bash
# From project root
./scripts/build-mobile.sh --development

# Or directly with Flutter
cd mobile/flutter
flutter build apk
```

### Production Build
```bash
# From project root
./scripts/build-mobile.sh --production --store-ready

# Or build both APK and App Bundle
./scripts/build-mobile.sh --production --appbundle --store-ready
```

## Testing

Run Flutter tests from the mobile app directory:
```bash
cd mobile/flutter
flutter test
```

Run analysis:
```bash
cd mobile/flutter
flutter analyze
```

## Documentation

See the Flutter app's README for detailed information:
- [Flutter App README](flutter/README.md) - Complete Flutter app documentation
- [Android Deployment Guide](../ANDROID_DEPLOYMENT_GUIDE.md) - Play Store submission guide

## Common Tasks

### Update App Version
Update the version in `mobile/flutter/pubspec.yaml`:
```yaml
version: 1.0.0+1
```

### Add New Features
1. Create new Dart files in `lib/`
2. Follow Flutter best practices and Material Design guidelines
3. Test changes with `flutter test`
4. Run analysis with `flutter analyze`

### Debug on Device
```bash
cd mobile/flutter
flutter run -v  # Verbose output for debugging
```

## Flutter Integration

The Flutter app communicates directly with the Internet Archive API using standard HTTP requests. See `mobile/flutter/lib/services/` for the service implementations.

## Contributing

When making changes to the mobile app:
1. Follow Flutter and Dart best practices
2. Use Material Design 3 components
3. Test on multiple screen sizes
4. Run `flutter analyze` before committing
5. Update documentation for significant changes

## Platform Support

The app is built using Flutter and can target:
- **Android** - Primary platform (fully tested)
- **iOS** - Coming soon
- **Web** - Experimental support
- **Desktop** - Windows, macOS, Linux (via Flutter Desktop)

## License

Same as the main ia-get project. See LICENSE in the root directory.
