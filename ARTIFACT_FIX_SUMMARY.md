# Automated Development Build APK Artifact Fix

## Issue Summary
The automated development build was not uploading APK artifacts to the development release, even though the build passed successfully. The APK files were being created but not included in the final release artifacts.

## Root Cause Analysis

### Problem 1: Artifact Name Pattern Mismatch
**Location**: `.github/workflows/ci.yml` line 636

The workflow was downloading artifacts with pattern:
```yaml
pattern: ia-get-*  # Only matches artifacts starting with "ia-get-"
```

However, the Flutter mobile artifact was named:
```yaml
name: flutter-mobile-dev  # Does NOT match "ia-get-*" pattern
```

**Result**: The Flutter mobile artifacts (including APK files) were never downloaded by the release job, so they couldn't be included in the development release.

### Problem 2: Hash File Name Mismatch  
**Location**: `.github/workflows/ci.yml` line 655

The workflow was looking for hash file:
```bash
android-dev-hash.txt  # Old/incorrect name
```

But the actual hash file created was:
```bash
flutter-dev-hash.txt  # Actual name from line 605
```

**Result**: Hash information for Flutter artifacts was not being included in release notes.

## Solution Implemented

### Fix 1: Updated Artifact Download Pattern
Changed line 636 in `.github/workflows/ci.yml`:

**Before:**
```yaml
pattern: ia-get-*  # Download both GUI and Android artifacts
```

**After:**
```yaml
pattern: '{ia-get-*,flutter-mobile-dev}'  # Download CLI/GUI and Flutter mobile artifacts
```

This pattern now matches:
- `ia-get-*` - All Rust CLI and GUI artifacts (Linux, Windows, macOS)
- `flutter-mobile-dev` - Flutter mobile development artifacts

### Fix 2: Updated Hash File Reference
Changed line 655 in `.github/workflows/ci.yml`:

**Before:**
```bash
for hash_file in ../release-artifacts/*/hashes_*_gui.txt ../release-artifacts/*/android-dev-hash.txt; do
```

**After:**
```bash
for hash_file in ../release-artifacts/*/hashes_*_gui.txt ../release-artifacts/*/flutter-dev-hash.txt; do
```

## Impact

### ✅ Fixed
- APK files from development builds are now included in development releases
- AAB (App Bundle) files are also properly included  
- Hash information for Flutter artifacts is now in release notes
- Development release properly includes all platform artifacts

### 🔒 No Breaking Changes
- Production builds continue to work as before
- Staging builds unaffected
- CLI and GUI artifact handling unchanged
- Only affects artifact collection in the release job

## Testing Verification

The fix has been validated to ensure:
1. ✅ Artifact pattern correctly matches both CLI/GUI and Flutter mobile artifacts
2. ✅ Hash file reference matches the actual created file
3. ✅ YAML syntax is valid
4. ✅ No breaking changes to existing builds

## Future Considerations

To prevent similar issues in the future:
1. Consider using a more consistent naming convention for all artifacts
2. Add automated tests to verify artifact patterns match actual artifact names
3. Document artifact naming conventions in CI/CD documentation

## Related Files
- `.github/workflows/ci.yml` - Main CI workflow (lines 636, 655)
- `.github/workflows/ci.yml` - Flutter artifact creation (line 613)
- `.github/workflows/ci.yml` - Hash file creation (line 605)
