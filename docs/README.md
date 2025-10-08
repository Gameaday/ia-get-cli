# ia-get Documentation

This directory contains all technical documentation for the ia-get project (Rust CLI + Flutter mobile app).

## 📚 Quick Navigation

### Getting Started
- **[Setup Guide](development/setup-guide.md)** - Installation and development environment setup
- **[Build Guide](development/build-guide.md)** - Building CLI and mobile app (includes CI/CD)
- **[Testing Guide](development/testing-guide.md)** - Running tests and quality checks

### Mobile App
- **[Implementation Status](mobile/implementation-status.md)** - What's built, what's in progress
- **[Roadmap](mobile/roadmap.md)** - Phase 4-5 plans and future features
- **[Play Store Deployment](mobile/play-store-guide.md)** - Publishing to Google Play

### Architecture
- **[Mobile App Architecture](architecture/mobile-app-architecture.md)** - Flutter app design and patterns
- **[Rust CLI Architecture](architecture/cli-architecture.md)** - Core download engine design

### Reference
- **[Quick Reference](QUICK_REFERENCE.md)** - Common commands and workflows
- **[TODO](TODO.md)** - Current work items

---

## 📁 Directory Structure

```
docs/
├── README.md                    # This file
├── QUICK_REFERENCE.md          # Common commands
├── TODO.md                     # Work tracking
│
├── architecture/               # System design
│   ├── mobile-app-architecture.md
│   └── cli-architecture.md
│
├── development/               # Dev guides
│   ├── setup-guide.md
│   ├── build-guide.md
│   └── testing-guide.md
│
└── mobile/                    # Mobile-specific
    ├── implementation-status.md
    ├── roadmap.md
    └── play-store-guide.md
```

---

## 🎯 Project Status (October 2025)

**Rust CLI**: ✅ Stable (v1.6.0+)
- Internet Archive downloads with resume support
- Metadata caching, compression, performance optimized

**Flutter Mobile App**: 🚧 In Development
- **Phase 1-3**: ✅ Complete (search, preview 74+ formats, MD3 UI)
- **Phase 4**: 🚧 In Progress (favorites, queue management, analytics)
  - Task 1: Not started
  - Task 2: ✅ Complete (unit tests)
  - Task 3: 🚧 In progress (download queue)
- **Phase 5**: 📋 Planned (advanced features, v2.0 release)

---

## 🤝 Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for development guidelines.

## 📜 License

See [LICENSE](../LICENSE) for details.
