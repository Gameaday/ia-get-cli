# Downloads & Releases

Get the latest version of Internet Archive Helper for your platform.

## 📱 Flutter Mobile & Web

### Android
**Universal APK** - Works on all Android devices (phones and tablets)

```bash
# Latest stable release
https://github.com/Gameaday/ia-get-cli/releases/latest/download/app-release.apk

# File size: ~15 MB
# Architectures: arm64-v8a, armeabi-v7a, x86_64
# Android version: 5.0 (API 21) or higher
```

**Installation:**
1. Download APK file
2. Enable "Install from Unknown Sources" in Android Settings
3. Open APK to install

**Features:**
- 🎨 Material Design 3 (78% compliant)
- 📱 Responsive design (phone, tablet, desktop web)
- 🔗 Deep linking (archive.org URLs)
- 📥 Background downloads
- ♿ WCAG 2.1 Level AA accessibility

### Web (Desktop)
**Browser-based application** - No installation required

```bash
# Try online
https://gameaday.github.io/ia-get-cli

# Status: Experimental
# Recommended: Desktop browsers (Chrome, Firefox, Edge, Safari)
# Works best on: Tablets and desktops (840dp+)
```

**Download for self-hosting:**
```bash
# Get web bundle
https://github.com/Gameaday/ia-get-cli/releases/latest/download/web-bundle.tar.gz

# Extract and serve
tar xzf web-bundle.tar.gz
python3 -m http.server 8000 -d web
```

---

## 🖥️ Rust CLI & Server

### Quick Install

**Linux (x86_64):**
```bash
curl -L https://github.com/Gameaday/ia-get-cli/releases/latest/download/ia-get-linux-x86_64.tar.gz | tar xz
sudo mv ia-get /usr/local/bin/
```

**macOS (Apple Silicon):**
```bash
curl -L https://github.com/Gameaday/ia-get-cli/releases/latest/download/ia-get-macos-apple-silicon.tar.gz | tar xz
sudo mv ia-get /usr/local/bin/
```

**Windows:**
```powershell
# Download and extract ia-get-windows-x86_64.zip
# Add to PATH or run directly
```

### All Platforms

| Platform | Architecture | Download | Size | Notes |
|----------|-------------|----------|------|-------|
| **Linux** | x86_64 (glibc) | [tar.gz](https://github.com/Gameaday/ia-get-cli/releases/latest/download/ia-get-linux-x86_64.tar.gz) | ~8 MB | Most compatible |
| **Linux** | x86_64 (musl) | [tar.gz](https://github.com/Gameaday/ia-get-cli/releases/latest/download/ia-get-linux-x86_64-musl.tar.gz) | ~9 MB | Static, portable |
| **Linux** | ARM64 | [tar.gz](https://github.com/Gameaday/ia-get-cli/releases/latest/download/ia-get-linux-arm64.tar.gz) | ~8 MB | Raspberry Pi 4+ |
| **Windows** | x86_64 | [zip](https://github.com/Gameaday/ia-get-cli/releases/latest/download/ia-get-windows-x86_64.zip) | ~7 MB | Code-signed |
| **macOS** | Intel | [tar.gz](https://github.com/Gameaday/ia-get-cli/releases/latest/download/ia-get-macos-intel.tar.gz) | ~7 MB | Intel Macs |
| **macOS** | Apple Silicon | [tar.gz](https://github.com/Gameaday/ia-get-cli/releases/latest/download/ia-get-macos-apple-silicon.tar.gz) | ~7 MB | M1/M2/M3/M4 |

**Features:**
- ⚡ Concurrent downloads (configurable workers)
- 🗜️ HTTP compression & archive extraction
- 🖼️ Desktop GUI (egui framework)
- ⌨️ CLI for automation and scripts
- 📊 Real-time performance metrics

---

## 🔐 Verify Downloads

All releases include SHA256 checksums for integrity verification.

**Linux/macOS:**
```bash
# Download checksum file
curl -LO https://github.com/Gameaday/ia-get-cli/releases/latest/download/ia-get-linux-x86_64.tar.gz.sha256

# Verify
sha256sum -c ia-get-linux-x86_64.tar.gz.sha256
```

**Windows (PowerShell):**
```powershell
# Download and verify
$hash = (Get-FileHash ia-get-windows-x86_64.zip -Algorithm SHA256).Hash
$expected = Get-Content ia-get-windows-x86_64.zip.sha256
if ($hash -eq $expected) { Write-Host "✓ Verified" } else { Write-Host "✗ Failed" }
```

---

## 📋 Version Information

### Current Versions
- **Rust CLI/Server:** v1.6.0
- **Flutter Mobile/Web:** v1.7.0-beta (Material Design 3)

### Release Channels

**Stable** (Recommended)
- Production-ready releases
- Fully tested on all platforms
- Code-signed binaries (Windows)
- Available: [Latest Release](https://github.com/Gameaday/ia-get-cli/releases/latest)

**Development** (Testing)
- Latest features and fixes
- Built from main branch
- May have bugs
- Available: [Development Release](https://github.com/Gameaday/ia-get-cli/releases/tag/development)

---

## 🆘 Troubleshooting

### Windows SmartScreen Warning
Windows binaries are code-signed, but you may still see a SmartScreen warning on first run. Click "More info" → "Run anyway".

### Android Unknown Sources
Enable "Install from Unknown Sources" in Android Settings → Security → Unknown Sources (or Apps → Special Access → Install Unknown Apps).

### macOS Gatekeeper
```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine ia-get
```

### Linux Permissions
```bash
# Make executable
chmod +x ia-get
```

---

## 📦 Build from Source

### Rust CLI/Server
```bash
git clone https://github.com/Gameaday/ia-get-cli.git
cd ia-get-cli
cargo build --release --no-default-features --features cli
```

### Flutter Mobile/Web
```bash
cd mobile/flutter
flutter pub get
flutter build apk          # Android
flutter build web          # Web
```

See [Development Guide](docs/DEVELOPMENT.md) for detailed instructions.

---

## 📝 Changelog

See [CHANGELOG.md](CHANGELOG.md) for complete version history and release notes.

---

## 📜 License

MIT License - see [LICENSE](LICENSE) for details.

---

## ⚖️ Legal

This project is not affiliated with or endorsed by the Internet Archive. Use responsibly and comply with Internet Archive's Terms of Service.
