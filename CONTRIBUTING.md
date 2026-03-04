# Contributing to ia-get CLI

We welcome contributions from developers, researchers, and Internet Archive enthusiasts! Whether you're fixing bugs, adding features, improving documentation, or helping with testing, your help makes Internet Archive content more accessible to everyone.

## 🚀 Quick Start

1. **Fork the repository** on GitHub
2. **Clone your fork** locally
3. **Set up the development environment**
4. **Make your changes**
5. **Test thoroughly**
6. **Submit a pull request**

## 🛠️ Development Setup

### Prerequisites

**Required:**
- **Rust**: Latest stable (1.92.0 or higher)

### Building and Testing

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone the repository
git clone https://github.com/Gameaday/ia-get-cli.git
cd ia-get-cli

# Build CLI version (fastest for development)
cargo build --no-default-features --features cli

# Build with GUI support (optional)
cargo build --features gui

# Run tests
cargo test --no-default-features --features cli

# Check formatting and linting
cargo fmt --check
cargo clippy --no-default-features --features cli -- -D warnings
```

## 📝 Code Style Guidelines

### Rust Code
- Follow standard Rust conventions and idioms
- Use `cargo fmt` for consistent formatting
- Ensure `cargo clippy` passes without warnings
- Write comprehensive tests for new functionality
- Use meaningful variable and function names
- Add documentation comments for public APIs
- Use `thiserror` for library error types, `anyhow` for application error handling

## 🐛 Bug Reports

When reporting bugs, please include:

- **Description**: Clear description of the issue
- **Steps to reproduce**: Detailed steps to reproduce the problem
- **Expected behavior**: What you expected to happen
- **Actual behavior**: What actually happened
- **Environment**: OS, Rust version, architecture
- **Logs**: Include relevant error messages or logs

## ✨ Feature Requests

We love new ideas! When suggesting features:

- **Use case**: Explain why this feature would be useful
- **Description**: Detailed description of the proposed feature
- **Alternatives**: Describe any alternative solutions considered
- **Internet Archive alignment**: How does this support the Internet Archive mission?

## 🔧 Types of Contributions

### Code Contributions
- **Bug fixes**: Fix existing issues
- **New features**: Add functionality that benefits users
- **Performance improvements**: Optimize speed, memory usage, or download throughput
- **Code cleanup**: Refactor code for better maintainability
- **Test coverage**: Add missing tests

### Documentation Contributions
- **README updates**: Improve setup and usage instructions
- **Code comments**: Add or improve inline documentation
- **Guides**: Create tutorials or how-to guides in the `docs/` directory

## 🔍 Code Review Process

1. **Automated checks**: All PRs must pass automated tests and linting
2. **Manual review**: Core maintainers will review code for quality and fit
3. **Testing**: Changes should be tested thoroughly
4. **Documentation**: Update documentation if needed
5. **Feedback**: Address any review comments
6. **Approval**: PRs need approval from at least one maintainer

## 🌟 Recognition

Contributors are recognized in:
- **Release notes**: Major contributions mentioned in releases
- **Contributors section**: Listed in project documentation
- **Commit history**: Your commits become part of the project history

## 📞 Getting Help

Need help contributing?

- **GitHub Discussions**: Ask questions and get help
- **Issues**: Browse existing issues for contribution ideas

## 📄 License

By contributing to ia-get CLI, you agree that your contributions will be licensed under the same MIT license as the project.

## 🤝 Code of Conduct

We are committed to providing a welcoming and inclusive environment for all contributors. Please be:

- **Respectful**: Treat everyone with respect and professionalism
- **Inclusive**: Welcome contributors from all backgrounds
- **Constructive**: Provide helpful feedback and suggestions
- **Collaborative**: Work together towards common goals
- **Patient**: Remember that everyone is learning

## 🎯 Project Goals

Keep these goals in mind when contributing:

- **Accessibility**: Make Internet Archive content more accessible
- **Privacy**: Maintain user privacy and data protection
- **Performance**: Ensure fast, efficient downloads
- **Reliability**: Create stable, dependable software
- **Community**: Support the Internet Archive community

---

**Thank you for contributing to ia-get CLI!**

Your efforts help preserve and provide access to human knowledge for everyone. 📚✨