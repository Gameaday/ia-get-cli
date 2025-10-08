# Repository Split Summary: IA Get → IA Helper

**Date**: October 8, 2025  
**Decision**: Split Flutter mobile app into dedicated repository  
**Status**: Planning Complete, Ready to Execute

## Overview

The IA Get project is being restructured to separate the Rust CLI tool and Flutter mobile app into two distinct repositories with independent identities.

### Before (Single Repository)
```
ia-get-cli/
├── src/                    (Rust CLI)
├── mobile/flutter/         (Flutter app)
├── docs/                   (Mixed documentation)
└── .github/workflows/      (Mixed CI/CD)
```

### After (Two Repositories)

#### ia-get-cli (Rust CLI Tool)
```
ia-get-cli/
├── src/                    (Rust CLI only)
├── docs/                   (CLI documentation)
├── .github/workflows/      (Rust CI/CD only)
└── README.md               (Links to ia-helper)
```

#### ia-helper (Flutter Mobile App)
```
ia-helper/
├── lib/                    (Flutter app code)
├── android/                (Android platform)
├── ios/                    (iOS platform)
├── docs/                   (Mobile documentation)
├── .github/workflows/      (Flutter CI/CD only)
└── README.md               (Mobile-focused)
```

## Branding Changes

### App Name
- **Old**: IA Get
- **New**: IA Helper
- **Rationale**: "Helper" better reflects full feature set (not just downloads)

### Package Name
- **Old**: `com.gameaday.iagetcli`
- **New**: `com.gameaday.iahelper`
- **Rationale**: Remove "cli" suffix, not relevant for mobile app

### Repository URLs
- **Old**: https://github.com/gameaday/ia-get-cli
- **New**: https://github.com/gameaday/ia-helper

### Play Store Title
- **Old**: "IA Get - Internet Archive Downloader"
- **New**: "IA Helper - Internet Archive Companion"
- **Rationale**: "Companion" emphasizes broader functionality

## Benefits of Split

### 1. Faster CI/CD
**Before**: Every commit builds both Rust and Flutter
- Rust changes trigger Flutter builds (unnecessary)
- Flutter changes trigger Rust builds (unnecessary)
- Total CI time: ~10-15 minutes per commit

**After**: Each repository builds only its own code
- Rust changes: ~3-5 minutes (Rust only)
- Flutter changes: ~5-7 minutes (Flutter only)
- **Time savings**: 40-50% faster feedback loop

### 2. Clearer Product Identity
**Before**: Confusion about what "ia-get" is
- CLI tool or mobile app?
- Mixed documentation
- Mixed issue tracker

**After**: Clear separation
- **ia-get**: Command-line power tool for automation
- **ia-helper**: Mobile companion app for browsing
- Each has focused documentation
- Each has targeted issue labels

### 3. Independent Versioning
**Before**: Version 1.6.0 applies to both
- CLI and app evolve together unnecessarily
- Breaking changes in one affect other

**After**: Independent releases
- **ia-get**: Can jump to 2.0.0 for Rust refactor
- **ia-helper**: Starts at 1.0.0 for Play Store
- Each follows its own release cycle

### 4. Play Store Compliance
**Before**: Confusing for Google reviewers
- Repository contains Rust code (irrelevant to Android app)
- Mixed documentation about CLI features
- Unclear what's being submitted

**After**: Crystal clear for reviewers
- Repository contains only mobile app code
- All documentation is mobile-focused
- No confusion about scope

### 5. Developer Experience
**Before**: Overhead for contributors
- Flutter devs must download Rust toolchain
- Rust devs must download Flutter SDK
- Larger repository, slower clones

**After**: Lightweight repositories
- Flutter devs only need Flutter
- Rust devs only need Rust
- Smaller repos, faster clones

## Documentation Updates

### Files Updated with New Branding

#### docs/PLAY_STORE_METADATA.md
- [x] App name: "IA Get" → "IA Helper"
- [x] Package name: com.gameaday.iahelper
- [x] Short description: Updated to emphasize "companion" role
- [x] Full description: Updated all "IA Get" references
- [x] GitHub links: ia-get-cli → ia-helper
- [x] Play Store title: "Internet Archive Companion"

#### docs/FLUTTER_APP_MIGRATION.md
- [x] Complete migration guide created
- [x] Step-by-step instructions
- [x] Code change examples
- [x] Testing checklist
- [x] Rollback plan
- [x] Timeline and estimates

### Files to Update (In Flutter App)

#### pubspec.yaml
- [ ] name: ia_get → ia_helper
- [ ] description: Update to reflect new branding
- [ ] repository: Update to ia-helper URL

#### android/app/build.gradle
- [ ] applicationId: com.gameaday.iahelper

#### android/app/src/main/AndroidManifest.xml
- [ ] package: com.gameaday.iahelper
- [ ] Update deep link schemes

#### android/app/src/main/res/values/strings.xml
- [ ] app_name: "IA Helper"

#### lib/**/*.dart (All Dart files)
- [ ] Search/replace: "IA Get" → "IA Helper"
- [ ] Update user-facing strings
- [ ] Update comments and documentation

## CI/CD Changes

### Before (flutter-ci.yml in ia-get-cli)
```yaml
- name: Build APK
  run: |
    cd mobile/flutter
    flutter build apk --flavor production
```

### After (flutter-ci.yml in ia-helper)
```yaml
- name: Build APK
  run: |
    flutter build apk --flavor production
```

**Simplification**: No more `cd mobile/flutter` - we're already in the root!

### Workflow Adjustments Needed
- [ ] Remove `cd mobile/flutter` from all commands
- [ ] Update artifact paths (no mobile/flutter/ prefix)
- [ ] Keep product flavors (development, staging, production)
- [ ] Keep checksum generation
- [ ] Keep APK and AAB uploads

## Migration Process

### Phase 1: Preparation (DONE ✅)
- [x] Create migration guide
- [x] Update Play Store metadata
- [x] Create migration script
- [x] Commit changes to ia-get-cli

### Phase 2: Create ia-helper Repository (TODO)
- [ ] Create repository on GitHub
- [ ] Initialize with README, LICENSE, .gitignore
- [ ] Set up branch protection
- [ ] Configure GitHub Actions secrets

### Phase 3: Copy Files (TODO)
- [ ] Run preparation script
- [ ] Verify all files copied
- [ ] Review directory structure
- [ ] Test builds locally

### Phase 4: Update Code (TODO)
- [ ] Update package name throughout
- [ ] Update app name strings
- [ ] Update CI/CD workflows
- [ ] Update documentation links
- [ ] Test on device

### Phase 5: First Commit (TODO)
- [ ] Initialize git in ia-helper
- [ ] Create comprehensive first commit
- [ ] Push to GitHub
- [ ] Verify CI/CD runs

### Phase 6: Update ia-get-cli (TODO)
- [ ] Update README with link to ia-helper
- [ ] Archive or remove mobile/ directory
- [ ] Remove Flutter CI/CD workflows
- [ ] Create announcement issue

## Timeline

| Phase | Duration | Status |
|-------|----------|--------|
| Preparation | 2 hours | ✅ Complete |
| Repository setup | 30 minutes | ⏳ Pending |
| File migration | 1 hour | ⏳ Pending |
| Code updates | 2 hours | ⏳ Pending |
| Testing | 1 hour | ⏳ Pending |
| Documentation | 1 hour | ⏳ Pending |
| **Total** | **7.5 hours** | **13% Complete** |

## Testing Plan

### Build Verification
- [ ] `flutter clean && flutter pub get`
- [ ] `flutter analyze` (0 errors expected)
- [ ] `flutter build apk --flavor development`
- [ ] `flutter build apk --flavor production`
- [ ] `flutter build appbundle --flavor production`

### Functionality Testing
- [ ] App launches successfully
- [ ] App name shows as "IA Helper"
- [ ] Search works correctly
- [ ] Download queue functional
- [ ] Library displays downloads
- [ ] Settings persist
- [ ] Dark mode toggles
- [ ] All screens accessible

### Platform Testing
- [ ] Android 5.0 (API 21) - minimum
- [ ] Android 14 (API 34) - target
- [ ] Phone in portrait
- [ ] Tablet in landscape

## Rollback Plan

If critical issues arise:

1. **Keep ia-get-cli mobile code intact** until ia-helper proven stable
2. **Parallel development** possible during transition
3. **Rollback steps**:
   - Continue development in ia-get-cli/mobile/flutter
   - Archive ia-helper repository
   - Revert branding changes if needed

## Success Criteria

Migration is successful when:

- ✅ ia-helper repository exists and builds
- ✅ All tests pass
- ✅ App runs on physical device
- ✅ All branding shows "IA Helper"
- ✅ Package name is com.gameaday.iahelper
- ✅ CI/CD workflows pass
- ✅ Documentation is complete
- ✅ ia-get-cli updated appropriately

## Post-Migration Tasks

### Immediate (Day 1)
- [ ] Announce migration in ia-get-cli
- [ ] Update external documentation
- [ ] Create GitHub topics for ia-helper
- [ ] Set up issue templates

### Short Term (Week 1)
- [ ] Complete Phase 5 (Play Store prep)
- [ ] Take screenshots with new branding
- [ ] Update feature graphic
- [ ] Prepare for Play Store submission

### Long Term (Month 1)
- [ ] Submit to Play Store
- [ ] Monitor user feedback
- [ ] Plan next release
- [ ] Consider iOS version

## Communication

### Internal Team
- Migration documented in this file
- All changes tracked in git commits
- Clear rollback procedure available

### External Users
- Announcement in ia-get-cli README
- GitHub issue explaining migration
- Links to new repository
- Redirect old mobile documentation

### Google Play Store
- New app submission as "IA Helper"
- Fresh app entry (not an update)
- Package name: com.gameaday.iahelper
- All metadata reflects new branding

## Files Created/Updated

### New Files Created
1. `docs/FLUTTER_APP_MIGRATION.md` (143 KB) - Complete migration guide
2. `scripts/prepare-ia-helper-migration.ps1` (386 lines) - Automation script
3. `docs/REPOSITORY_SPLIT_SUMMARY.md` (this file) - High-level overview

### Files Updated
1. `docs/PLAY_STORE_METADATA.md` - All branding to "IA Helper"
2. `docs/features/PHASE_5_TASK_1_PROGRESS.md` - Created

### Commits Made
1. `feat(migration): Rebrand to IA Helper and prepare for repository split` (3f58d40)

## Resources

### Documentation
- [Flutter App Migration Guide](FLUTTER_APP_MIGRATION.md) - Detailed instructions
- [Play Store Metadata](PLAY_STORE_METADATA.md) - Updated with new branding
- [Android Permissions](ANDROID_PERMISSIONS.md) - Needs app name update

### Scripts
- `scripts/prepare-ia-helper-migration.ps1` - Copies files to new repo location

### Repositories
- **Original**: https://github.com/gameaday/ia-get-cli
- **New**: https://github.com/gameaday/ia-helper (to be created)

## Questions & Answers

**Q: Why not keep everything in one repository?**  
A: Faster CI/CD, clearer identity, independent releases, better Play Store submission.

**Q: Will this break existing users?**  
A: No existing users yet. Perfect time for this change before Play Store launch.

**Q: Can we still share code between repositories?**  
A: They don't share code. CLI is Rust, mobile is Dart/Flutter.

**Q: What about git history?**  
A: Fresh start for ia-helper. If history needed, use git subtree (complex).

**Q: Timeline to complete?**  
A: ~7.5 hours total. Can be done in 1-2 days.

---

**Status**: Planning Complete ✅  
**Next Step**: Create ia-helper repository  
**Estimated Completion**: October 9-10, 2025  
**Contact**: gameaday.project@gmail.com
