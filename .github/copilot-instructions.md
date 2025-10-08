# GitHub Copilot Instructions for ia-get

## Project Overview
This is a Rust CLI tool for downloading files from the Internet Archive, built with standard Cargo toolchain for simplicity and reliability.

## Development Guidelines

### Rust Standards
- Follow standard Rust conventions and idiomatic patterns
- Use `cargo fmt` and `cargo clippy` for code formatting and linting
- Prefer explicit error handling with `Result<T, E>` types
- Use `anyhow` or `thiserror` for error handling consistency

### Build System
- This project uses standard Cargo toolchain for all operations
- Use `cargo build` for development builds and `cargo build --release` for optimized builds
- Run tests with `cargo test` and linting with `cargo clippy`
- Maintain compatibility with standard Rust compilation targets
- **Always run `cargo fmt --check` and `cargo fmt` at the end of every PR to ensure consistent code formatting**

### Dependencies
- Keep dependencies minimal and well-justified
- Update Cargo.lock when adding new dependencies
- Prefer crates that are well-maintained and have good ecosystem support

### Code Structure
- Follow CLI best practices with clear subcommands and help text
- Use structured logging for better debugging
- Implement proper signal handling for long-running downloads
- Include comprehensive error messages for user-facing operations

### Testing
- Write unit tests for core functionality
- Include integration tests for CLI behavior
- Test cross-platform compatibility where relevant

### Documentation
- Update README.md for any new features or usage changes
- Include examples in help text and documentation
- Document any Internet Archive API specifics or limitations

### Documentation Organization
- **All documentation files MUST go in the `docs/` directory** with proper hierarchy and organization
- **EXCEPTION**: Only `PRIVACY_POLICY.md` stays at the repository root level
- Use subdirectories for organization:
  - `docs/features/` - Feature implementation documentation and completion reports
  - `docs/guides/` - User guides and how-to documents
  - `docs/architecture/` - System design, architecture decisions, and technical specs
  - `docs/development/` - Development workflows, setup guides, and contributing docs
- **Naming conventions**:
  - Use descriptive names: `cache-implementation.md` not `doc1.md`
  - Use lowercase with hyphens: `feature-name-guide.md`
  - Include completion status in feature docs: `FEATURE_NAME_COMPLETE.md`
- **Long-term management**:
  - Phase completion summaries → `docs/features/phase-N-complete.md`
  - Task completion reports → `docs/features/phase-N-task-M-complete.md`
  - Implementation plans → `docs/features/feature-name-plan.md`
  - Progress tracking → `docs/features/feature-name-progress.md`
- **DO NOT** create top-level documentation files except for standard files like README.md, CHANGELOG.md, CONTRIBUTING.md, LICENSE, and PRIVACY_POLICY.md

## Mobile App

**The Flutter mobile app has moved to its own repository:** [ia-helper](https://github.com/gameaday/ia-helper)

This repository (`ia-get-cli`) now focuses exclusively on the Rust command-line tool. For mobile app development, see the ia-helper repository.
