# ia-get Documentation

This directory contains technical documentation for the ia-get Rust CLI tool.

> **Note:** The Flutter mobile app has moved to [ia-helper](https://github.com/gameaday/ia-helper)

## 📚 Documentation

### Reference
- **[Quick Reference](QUICK_REFERENCE.md)** - Common development workflows and security setup
- **[Format Categories](guides/format-categories.md)** - File format filtering and presets
- **[Migration Complete](MIGRATION_COMPLETE.md)** - Historical record of repository split
- **[TODO](TODO.md)** - Current work items

---

## 📁 Key Files

```
docs/
├── README.md                    # This file
├── QUICK_REFERENCE.md          # Development quick reference
├── MIGRATION_COMPLETE.md       # Repository split history
├── TODO.md                     # Current tasks
└── guides/
    └── format-categories.md    # File format filtering guide
```
├── README.md                    # This file
├── QUICK_REFERENCE.md          # Common commands
├── TODO.md                     # Work tracking
---

## 🎯 Project Status (October 2025)

**ia-get CLI**: ✅ Stable (v2.0.0)
- Rust-based Internet Archive downloader
- High-performance concurrent downloads with resume support
- Metadata caching, compression support, advanced filtering
- CLI-first design with optional GUI mode

**Flutter Mobile App**: Moved to [ia-helper repository](https://github.com/gameaday/ia-helper)

---

## 🤝 Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for development guidelines.

For Rust development:
- Follow standard Rust conventions
- Run `cargo fmt` and `cargo clippy` before committing
- Write tests for new functionality
- Update documentation for API changes

## 📜 License

See [LICENSE](../LICENSE) for details (MIT License).
