# Downloads & Releases

Get the latest version of ia-get CLI for your platform.

## 🖥️ Rust CLI

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
- 🖼️ Desktop GUI (egui framework, optional)
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

```bash
git clone https://github.com/Gameaday/ia-get-cli.git
cd ia-get-cli
cargo build --release --no-default-features --features cli
```

---

## 📱 Mobile App

The mobile app has moved to its own repository: [ia-helper](https://github.com/gameaday/ia-helper)

---

## 📝 Changelog

See [CHANGELOG.md](CHANGELOG.md) for complete version history and release notes.

---

## 📜 License

MIT License - see [LICENSE](LICENSE) for details.

---

## ⚖️ Legal

This project is not affiliated with or endorsed by the Internet Archive. Use responsibly and comply with Internet Archive's Terms of Service.
