# Contributing to FluxReel

Thank you for your interest in contributing to FluxReel! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful, inclusive, and constructive in all interactions.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/FluxReel.git`
3. Create a branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes: `make test`
6. Commit your changes: `git commit -m "Add feature: description"`
7. Push to your fork: `git push origin feature/your-feature-name`
8. Open a Pull Request

## Development Setup

### Rust Components

```bash
cd rust
cargo build
cargo test
```

### Python Components

```bash
cd python
pip install -e .
pytest tests/
```

## Coding Standards

- **Rust**: Follow `rustfmt` and `clippy` guidelines
- **Python**: Follow PEP 8, use `black` for formatting
- **Commits**: Use clear, descriptive commit messages
- **Documentation**: Update README and docstrings as needed

## Pull Request Process

1. Ensure all tests pass
2. Update documentation if needed
3. Add tests for new features
4. Ensure code follows style guidelines
5. Request review from maintainers

## Questions?

Open an issue or contact the maintainers.

