# Mobile App Migrated ✨# Mobile App - README



The **IA Helper** Flutter mobile app has been moved to its own dedicated repository:This directory contains the Flutter mobile application.



## 🔗 New Location## Overview



### [**github.com/gameaday/ia-helper**](https://github.com/gameaday/ia-helper)The mobile app uses a **pure Dart implementation** for accessing Internet Archive content. This provides:

- Cross-platform compatibility (Android, iOS, Web, Desktop)

---- Standard Flutter development workflow

- No native build dependencies

## 📱 Why the Move?- Fast build times without native compilation



The mobile app now lives in a separate repository for several great reasons:## Architecture



### ✅ Benefits```

Flutter/Dart App

1. **Faster Development**    ↓ (HTTP API)

   - No more waiting for Rust builds when working on FlutterInternet Archive JSON API

   - No more Flutter builds when working on Rust```

   - 50% faster CI/CD pipeline

### Key Design Principles

2. **Clear Product Identity**

   - **IA Get**: Command-line power tool for automation1. **Pure Dart Implementation**: All functionality is implemented in Dart, making it portable and easy to maintain.

   - **IA Helper**: Mobile companion app for browsing

   - Each has focused documentation and development2. **Standard Flutter Architecture**: Uses standard Flutter packages and follows Flutter best practices.



3. **Independent Releases**3. **API-First Design**: Direct communication with Internet Archive's JSON API using Dart's http package.

   - Mobile app can release on its own schedule

   - CLI tool can evolve independently## Building

   - No forced version synchronization

### Development Build

4. **Better Play Store Experience**```bash

   - Repository contains only mobile app codecd mobile/flutter

   - All documentation is mobile-focusedflutter pub get

   - Clearer for Google Play reviewersflutter run

```

5. **Smaller Repositories**

   - Mobile devs don't need Rust toolchain### Android APK Build

   - CLI devs don't need Flutter SDK```bash

   - Faster clones, focused development# From project root

./scripts/build-mobile.sh --development

---

# Or directly with Flutter

## 🆕 New Brandingcd mobile/flutter

flutter build apk

The mobile app has been rebranded from **"IA Get"** to **"IA Helper"** to better reflect its role as a complete Internet Archive companion app.```



### What Changed### Production Build

```bash

- **App Name**: IA Get → **IA Helper**# From project root

- **Package**: `com.gameaday.iagetcli` → `com.gameaday.iahelper`./scripts/build-mobile.sh --production --store-ready

- **Positioning**: "Downloader" → **"Internet Archive Companion"**

- **Repository**: ia-get-cli → **ia-helper**# Or build both APK and App Bundle

./scripts/build-mobile.sh --production --appbundle --store-ready

---```



## 🚀 Get Started with IA Helper## Testing



### DownloadRun Flutter tests from the mobile app directory:

- **Android APK**: [Coming to Google Play Store](#)```bash

- **Source Code**: https://github.com/gameaday/ia-helpercd mobile/flutter

flutter test

### Features```

- 📱 Beautiful Material Design 3 interface

- 🔍 Search 35+ million Internet Archive itemsRun analysis:

- 📥 Smart download queue with resume capability```bash

- 📚 Offline library managementcd mobile/flutter

- 🌙 Full dark mode supportflutter analyze

- 🔐 Privacy-first (no tracking, no ads)```



### Development## Documentation

```bash

git clone https://github.com/gameaday/ia-helper.gitSee the Flutter app's README for detailed information:

cd ia-helper- [Flutter App README](flutter/README.md) - Complete Flutter app documentation

flutter pub get- [Android Deployment Guide](../ANDROID_DEPLOYMENT_GUIDE.md) - Play Store submission guide

flutter run

```## Common Tasks



---### Update App Version

Update the version in `mobile/flutter/pubspec.yaml`:

## 📖 Documentation```yaml

version: 1.0.0+1

All mobile app documentation has moved to the new repository:```



- **Play Store Metadata**: [ia-helper/docs/PLAY_STORE_METADATA.md](https://github.com/gameaday/ia-helper/blob/main/docs/PLAY_STORE_METADATA.md)### Add New Features

- **Android Permissions**: [ia-helper/docs/ANDROID_PERMISSIONS.md](https://github.com/gameaday/ia-helper/blob/main/docs/ANDROID_PERMISSIONS.md)1. Create new Dart files in `lib/`

- **Migration Guide**: [ia-helper/docs/FLUTTER_APP_MIGRATION.md](https://github.com/gameaday/ia-helper/blob/main/docs/FLUTTER_APP_MIGRATION.md)2. Follow Flutter best practices and Material Design guidelines

- **Phase Plans**: [ia-helper/docs/features/](https://github.com/gameaday/ia-helper/tree/main/docs/features)3. Test changes with `flutter test`

4. Run analysis with `flutter analyze`

---

### Debug on Device

## 🔧 This Repository (ia-get-cli)```bash

cd mobile/flutter

This repository now focuses exclusively on the **Rust command-line tool**:flutter run -v  # Verbose output for debugging

```

- ⚡ High-performance concurrent downloads

- 🖼️ Optional desktop GUI (egui)## Flutter Integration

- ⌨️ Powerful CLI for automation

- 🗜️ HTTP compression supportThe Flutter app communicates directly with the Internet Archive API using standard HTTP requests. See `mobile/flutter/lib/services/` for the service implementations.

- 🎯 Advanced filtering

## Contributing

**[Back to main README →](../README.md)**

When making changes to the mobile app:

---1. Follow Flutter and Dart best practices

2. Use Material Design 3 components

## 💬 Questions?3. Test on multiple screen sizes

4. Run `flutter analyze` before committing

- **IA Helper Issues**: https://github.com/gameaday/ia-helper/issues5. Update documentation for significant changes

- **IA Get CLI Issues**: https://github.com/gameaday/ia-get-cli/issues

- **Email**: gameaday.project@gmail.com## Platform Support



---The app is built using Flutter and can target:

- **Android** - Primary platform (fully tested)

**Migration Date**: October 8, 2025  - **iOS** - Coming soon

**IA Helper**: https://github.com/gameaday/ia-helper  - **Web** - Experimental support

**IA Get CLI**: https://github.com/gameaday/ia-get-cli- **Desktop** - Windows, macOS, Linux (via Flutter Desktop)


## License

Same as the main ia-get project. See LICENSE in the root directory.
