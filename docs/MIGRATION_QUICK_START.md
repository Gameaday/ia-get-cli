# IA Helper Migration - Quick Start Checklist

**Date**: October 8, 2025  
**Status**: Ready to Execute  
**Estimated Time**: 7.5 hours total

## ✅ Completed Preparation (2 hours)

- [x] Decided on new app name: "IA Helper"
- [x] Decided on new package name: com.gameaday.iahelper
- [x] Updated Play Store metadata with new branding
- [x] Created comprehensive migration guide (143 KB)
- [x] Created automated migration script (386 lines)
- [x] Created repository split summary document
- [x] Committed all preparation work to ia-get-cli
- [x] Pushed to GitHub

## 🚀 Next Steps (Execute Migration)

### Step 1: Create ia-helper Repository (30 minutes)

1. **Go to GitHub**: https://github.com/new
2. **Repository settings**:
   - Owner: Gameaday
   - Name: `ia-helper`
   - Description: "Your complete Internet Archive companion - download, search, and organize"
   - Public repository
   - ✅ Add README file (will be replaced)
   - ✅ Add .gitignore (Flutter template)
   - ✅ Add license (MIT - same as ia-get-cli)

3. **Clone to local machine**:
   ```powershell
   cd c:\Project
   git clone https://github.com/Gameaday/ia-helper.git
   ```

4. **Configure repository**:
   - Settings → General → Features: ✅ Issues, ✅ Discussions
   - Settings → Branches → Branch protection rules:
     - Branch name pattern: `main`
     - ✅ Require pull request reviews before merging
     - ✅ Require status checks to pass before merging
   - Settings → Topics: Add topics
     - `flutter`
     - `dart`
     - `android`
     - `internet-archive`
     - `material-design-3`
     - `mobile-app`

### Step 2: Run Migration Script (1 hour)

1. **Open PowerShell as Administrator**

2. **Navigate to ia-get-cli**:
   ```powershell
   cd c:\Project\ia-get-cli-github\ia-get-cli
   ```

3. **Review script** (optional):
   ```powershell
   notepad scripts\prepare-ia-helper-migration.ps1
   ```

4. **Run migration script**:
   ```powershell
   .\scripts\prepare-ia-helper-migration.ps1
   ```

5. **Review copied files**:
   ```powershell
   cd c:\Project\ia-helper
   dir
   ```

Expected output:
```
📁 lib/              ← Flutter app code
📁 android/          ← Android platform
📁 test/             ← Test files
📁 assets/           ← App assets
📁 docs/             ← Documentation
📁 .github/          ← CI/CD workflows
📄 pubspec.yaml      ← Flutter config
📄 README.md         ← Mobile-focused readme
📄 LICENSE           ← MIT license
📄 PRIVACY_POLICY.md ← Privacy policy
📄 CHANGELOG.md      ← Changelog
```

### Step 3: Update Flutter Code (2 hours)

#### A. Update pubspec.yaml

Location: `c:\Project\ia-helper\pubspec.yaml`

Changes needed:
```yaml
# OLD:
name: ia_get
description: Internet Archive download manager

# NEW:
name: ia_helper
description: Your Internet Archive companion app
repository: https://github.com/gameaday/ia-helper
```

#### B. Update Package Name

1. **Update android/app/build.gradle**:
   ```gradle
   defaultConfig {
       applicationId "com.gameaday.iahelper"  // Changed from iagetcli
   }
   ```

2. **Update android/app/src/main/AndroidManifest.xml**:
   ```xml
   <manifest package="com.gameaday.iahelper">  <!-- Changed -->
   ```

3. **Move MainActivity.kt**:
   ```powershell
   # Old location
   android/app/src/main/kotlin/com/gameaday/iagetcli/MainActivity.kt
   
   # Create new directory
   mkdir android\app\src\main\kotlin\com\gameaday\iahelper
   
   # Move file
   move android\app\src\main\kotlin\com\gameaday\iagetcli\MainActivity.kt `
        android\app\src\main\kotlin\com\gameaday\iahelper\MainActivity.kt
   
   # Delete old directory
   rmdir android\app\src\main\kotlin\com\gameaday\iagetcli
   ```

4. **Update MainActivity.kt package**:
   ```kotlin
   package com.gameaday.iahelper  // Changed from iagetcli
   ```

#### C. Update App Name String

File: `android/app/src/main/res/values/strings.xml`

```xml
<resources>
    <string name="app_name">IA Helper</string>  <!-- Changed from IA Get -->
</resources>
```

#### D. Update User-Facing Strings in Code

Run search and replace in all `.dart` files:

```powershell
cd c:\Project\ia-helper

# Search for "IA Get" references
findstr /s /i "IA Get" lib\*.dart

# Manual replacements needed in:
# - lib/screens/about_screen.dart
# - lib/screens/settings_screen.dart
# - lib/utils/constants.dart
# - Any screen titles or dialogs
```

Replace:
- "IA Get" → "IA Helper"
- Comments mentioning "IA Get" → "IA Helper"

#### E. Update CI/CD Workflow

File: `.github/workflows/flutter-ci.yml`

Changes:
```yaml
# OLD:
- name: Build APK
  run: |
    cd mobile/flutter
    flutter build apk --flavor production

# NEW:
- name: Build APK
  run: |
    flutter build apk --flavor production
```

Remove all `cd mobile/flutter` commands (we're already in root!).

### Step 4: Test Builds (1 hour)

```powershell
cd c:\Project\ia-helper

# Clean and get dependencies
flutter clean
flutter pub get

# Run analyzer
flutter analyze

# Build debug APK
flutter build apk --flavor development --debug

# Build release APK
flutter build apk --flavor production --release

# Build release AAB
flutter build appbundle --flavor production --release
```

Expected results:
- ✅ All builds complete successfully
- ✅ flutter analyze: 0 errors
- ✅ APK created: build/app/outputs/flutter-apk/app-production-release.apk
- ✅ AAB created: build/app/outputs/bundle/productionRelease/app-production-release.aab

### Step 5: Test on Device (1 hour)

1. **Install debug APK**:
   ```powershell
   flutter install --flavor development
   ```

2. **Run from IDE**:
   ```powershell
   flutter run --flavor development
   ```

3. **Manual testing checklist**:
   - [ ] App launches successfully
   - [ ] App name shows as "IA Helper" in launcher
   - [ ] Home screen loads
   - [ ] Search works
   - [ ] Can view item details
   - [ ] Download queue accessible
   - [ ] Can add downloads
   - [ ] Library displays correctly
   - [ ] Settings screen works
   - [ ] Dark mode toggles correctly
   - [ ] No crashes or errors

### Step 6: Commit to ia-helper (30 minutes)

```powershell
cd c:\Project\ia-helper

# Initialize git (if not already initialized)
git init
git branch -M main

# Stage all files
git add -A

# Create comprehensive first commit
git commit -m "feat: Initial commit - IA Helper mobile app

Migrated Flutter mobile app from ia-get-cli repository to dedicated ia-helper repository.

Changes from ia-get-cli:
- Rebranded from 'IA Get' to 'IA Helper'
- Updated package name: com.gameaday.iagetcli → com.gameaday.iahelper
- Updated all documentation with new branding
- Simplified directory structure (removed mobile/flutter/ prefix)
- Updated CI/CD for Flutter-only builds
- Updated all GitHub links to new repository

Features (carried over from ia-get-cli):
- ✅ Internet Archive search and browse
- ✅ Download queue with priority management
- ✅ Resumable downloads with auto-retry
- ✅ Material Design 3 UI with dark mode
- ✅ Offline library management
- ✅ Advanced search filters
- ✅ Download scheduling
- ✅ Network-aware downloads

App Status:
- Phase 4 complete (Download Queue Management)
- Phase 5 in progress (Play Store preparation)
- 0 compile errors, 0 warnings
- ~78% Material Design 3 compliant
- Ready for internal testing

Technical Details:
- Flutter 3.35.0
- Material Design 3
- SQLite database
- Product flavors: development, staging, production
- CI/CD: GitHub Actions

Migration documented in: docs/FLUTTER_APP_MIGRATION.md

Original repository: https://github.com/gameaday/ia-get-cli
New repository: https://github.com/gameaday/ia-helper"

# Push to GitHub
git remote add origin https://github.com/Gameaday/ia-helper.git
git push -u origin main
```

### Step 7: Update ia-get-cli (1 hour)

```powershell
cd c:\Project\ia-get-cli-github\ia-get-cli
```

#### A. Update README.md

Add section after project description:

```markdown
## 📱 Mobile App

The mobile companion app has moved to its own repository:

**[IA Helper](https://github.com/gameaday/ia-helper)** - Your complete Internet Archive companion

The mobile app provides a beautiful Material Design 3 interface for:
- 🔍 Searching the Internet Archive
- 📥 Downloading and managing files
- 📚 Organizing your digital library
- 📱 Offline access to downloaded content

[Download IA Helper on Google Play](#) *(coming soon)*

---

## 🖥️ CLI Tool (This Repository)

The `ia-get` command-line tool is perfect for:
```

#### B. Archive mobile/ directory

Option 1 (Recommended): Delete entirely
```powershell
git rm -r mobile/
```

Option 2: Keep as reference
```powershell
git mv mobile mobile.archived
```

Option 3: Just add notice
```powershell
# Create mobile/README.md
echo "# Mobile App Moved

The Flutter mobile app has been moved to: https://github.com/gameaday/ia-helper

This directory is kept for reference only." > mobile\README.md
```

#### C. Remove Flutter CI/CD

If any Flutter-specific workflows exist, remove them:
```powershell
# Check workflows
dir .github\workflows\

# Keep: rust-ci.yml or similar
# Remove: flutter-ci.yml (now in ia-helper)
```

#### D. Commit changes

```powershell
git add -A
git commit -m "refactor: Move Flutter app to dedicated ia-helper repository

The mobile app has been migrated to: https://github.com/gameaday/ia-helper

Changes:
- Updated README with link to ia-helper
- Archived/removed mobile/ directory
- Simplified CI/CD to Rust-only

Benefits:
- Faster CI (no cross-building)
- Clearer product separation
- Independent versioning
- Better Play Store submission

Migration documentation:
- docs/FLUTTER_APP_MIGRATION.md
- docs/REPOSITORY_SPLIT_SUMMARY.md

The ia-get CLI continues development here.
The IA Helper mobile app continues at ia-helper repository."

git push origin main
```

### Step 8: Verify Everything (30 minutes)

#### ia-helper Repository
- [ ] Repository accessible at https://github.com/gameaday/ia-helper
- [ ] CI/CD workflow runs and passes
- [ ] README displays correctly
- [ ] Documentation complete
- [ ] Issues enabled
- [ ] Topics configured

#### ia-get-cli Repository
- [ ] README links to ia-helper
- [ ] Mobile directory handled
- [ ] CI/CD focuses on Rust only
- [ ] No broken links

#### Builds
- [ ] ia-helper builds successfully
- [ ] ia-get-cli builds successfully (Rust)
- [ ] Both CI/CD pipelines green

## 📊 Progress Tracking

- [x] **Preparation** (2 hours) - 100% Complete
- [ ] **Repository Setup** (30 minutes) - 0% Complete
- [ ] **File Migration** (1 hour) - 0% Complete
- [ ] **Code Updates** (2 hours) - 0% Complete
- [ ] **Testing** (1 hour) - 0% Complete
- [ ] **Documentation** (1 hour) - 0% Complete
- [ ] **Overall** (7.5 hours) - **13% Complete**

## 🎯 Success Criteria

Migration is complete when ALL of these are true:

- [ ] ia-helper repository exists on GitHub
- [ ] Flutter app builds successfully from ia-helper
- [ ] App runs on Android device
- [ ] App shows "IA Helper" branding everywhere
- [ ] Package name is com.gameaday.iahelper
- [ ] All tests pass (flutter analyze)
- [ ] CI/CD workflow passes on GitHub Actions
- [ ] Documentation is complete and accurate
- [ ] ia-get-cli updated with links to ia-helper
- [ ] No broken links in either repository

## ⚠️ Common Issues & Solutions

### Issue 1: Build fails after package rename

**Error**: `Could not resolve all files for configuration ':app:debugRuntimeClasspath'`

**Solution**:
```powershell
flutter clean
rm -r android/.gradle
flutter pub get
flutter build apk
```

### Issue 2: MainActivity not found

**Error**: `MainActivity.kt: Unresolved reference: MainActivity`

**Solution**: Verify MainActivity.kt is in correct directory:
```powershell
android/app/src/main/kotlin/com/gameaday/iahelper/MainActivity.kt
```

And package declaration matches:
```kotlin
package com.gameaday.iahelper
```

### Issue 3: App name still shows "IA Get"

**Solution**: Check `android/app/src/main/res/values/strings.xml`:
```xml
<string name="app_name">IA Helper</string>
```

Then rebuild:
```powershell
flutter clean
flutter build apk
```

### Issue 4: CI/CD fails with "command not found"

**Solution**: Remove `cd mobile/flutter` from workflow YAML.

## 📞 Need Help?

- **Migration Guide**: See `docs/FLUTTER_APP_MIGRATION.md` for detailed instructions
- **Summary**: See `docs/REPOSITORY_SPLIT_SUMMARY.md` for overview
- **Issues**: Create issue in ia-get-cli or email gameaday.project@gmail.com

## 🎉 After Completion

Once migration is complete:

1. **Announce on social media** (if applicable)
2. **Update any external documentation**
3. **Continue with Phase 5** (Play Store preparation)
4. **Take screenshots** with "IA Helper" branding
5. **Submit to Play Store** when ready

---

**Created**: October 8, 2025  
**Status**: Ready to execute  
**Next Step**: Create ia-helper repository on GitHub  
**Estimated Time Remaining**: ~7.5 hours
