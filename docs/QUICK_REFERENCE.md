# Quick Reference - Local Development Setup

## Code Signing (Windows)

### Sign ia-get.exe
```powershell
# Build
cargo build --release --target x86_64-pc-windows-msvc

# Sign
powershell -ExecutionPolicy Bypass -File .\scripts\local-sign.ps1
```

### View GitHub Secrets
```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\show-github-secrets.ps1
```

## Flutter Android Builds

### Set Java 21 Environment
```powershell
. .\scripts\setup-flutter-env.ps1
```

### Build APK
```powershell
cd mobile/flutter
flutter build apk --release
```

## File Locations

### Local Secrets (gitignored)
- `.secrets/CODE_SIGNING_CERT.txt` - Base64 certificate
- `.secrets/CODE_SIGNING_PASSWORD.txt` - Password
- `.secrets/cert.pfx` - Binary certificate

### Scripts
- `scripts/local-sign.ps1` - Sign Windows executable
- `scripts/setup-flutter-env.ps1` - Configure Java 21 for Flutter
- `scripts/show-github-secrets.ps1` - Display GitHub secret values

## VS Code Issues

### Clear Cached Gradle Errors
1. Run: `. .\scripts\setup-flutter-env.ps1`
2. Reload window: `Ctrl+Shift+P` → "Developer: Reload Window"

The Java 21 environment is now properly configured.

## Security

✅ All secrets in `.secrets/` folder (gitignored)
✅ Additional patterns in `.gitignore`: `*.pfx`, `*cert-base64.txt`
✅ Desktop backups safe to delete after verification

⚠️ Never commit `.secrets/` folder or PFX files
⚠️ Keep GitHub secrets secure
