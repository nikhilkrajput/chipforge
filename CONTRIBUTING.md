# Contributing to ChipForge

First off, thank you for considering contributing to ChipForge! It's people like you that make ChipForge such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* Use a clear and descriptive title
* Describe the exact steps which reproduce the problem
* Provide specific examples to demonstrate the steps
* Describe the behavior you observed after following the steps
* Explain which behavior you expected to see instead and why
* Include logs, error messages, and screenshots if possible

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* A clear and descriptive title
* A detailed description of the proposed functionality
* Explain why this enhancement would be useful
* List any similar features in other tools

### Pull Requests

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. If you've changed APIs, update the documentation
4. Ensure the test suite passes
5. Make sure your code follows the existing style
6. Write a clear commit message

## Development Setup

### Prerequisites

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js for GUI development
# Install from https://nodejs.org/

# Python for AI features
# Install from https://python.org/
```

### Building from Source

```bash
# Clone the repository
git clone --recursive https://github.com/nikhilkrajput/chipforge.git
cd chipforge

# Build the project
cargo build

# Run tests
cargo test --workspace

# Run with logging
RUST_LOG=debug cargo run --bin chipforge
```

### Code Style

We use `rustfmt` and `clippy` to maintain code quality:

```bash
# Format code
cargo fmt --all

# Run clippy
cargo clippy --all --all-targets -- -D warnings
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run tests with output
cargo test --workspace -- --nocapture

# Run specific test
cargo test --package chipforge-synthesis test_parser

# Run benchmarks
cargo bench --workspace
```

### Documentation

```bash
# Build documentation
cargo doc --no-deps --workspace

# Build and open documentation
cargo doc --no-deps --workspace --open

# Build mdBook documentation
cd docs
mdbook build
mdbook serve
```

## Project Structure

```
chipforge/
├── core/              # Core EDA engines
│   ├── synthesis/     # Synthesis engine
│   ├── simulation/    # Simulation engine
│   ├── formal/        # Formal verification
│   └── common/        # Shared utilities
├── riscv/            # RISC-V ecosystem
├── gui/              # Desktop GUI
├── cli/              # Command-line tool
├── plugins/          # Plugin system
└── docs/             # Documentation
```

## Commit Messages

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types:
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that don't affect code meaning (formatting, etc.)
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `perf`: Performance improvement
- `test`: Adding missing tests or correcting existing tests
- `chore`: Changes to the build process or auxiliary tools

Examples:
```
feat(synthesis): add constant propagation pass
fix(parser): handle escaped identifiers correctly
docs(riscv): add SoC builder tutorial
```

## Release Process

1. Update version numbers in `Cargo.toml` files
2. Update `CHANGELOG.md`
3. Create a git tag: `git tag -a v0.1.0 -m "Release v0.1.0"`
4. Push tags: `git push origin --tags`
5. CI will automatically build and publish releases

## Getting Help

* Join our [Discord server](https://discord.gg/chipforge)
* Check the [documentation](https://docs.chipforge.org)
* Open an issue on GitHub

## Recognition

Contributors will be recognized in:
* The `CONTRIBUTORS.md` file
* Release notes
* Project website

Thank you for contributing to ChipForge! 🎉
