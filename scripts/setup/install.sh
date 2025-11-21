#!/usr/bin/env bash
# ChipForge Installation Script

set -e

echo "====================================="
echo "ChipForge Installation"
echo "====================================="
echo ""

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo "Rust is not installed. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "✓ Rust is already installed"
fi

# Check Rust version
RUST_VERSION=$(rustc --version | awk '{print $2}')
echo "Rust version: $RUST_VERSION"

# Build and install ChipForge
echo ""
echo "Building ChipForge..."
cargo build --release

echo ""
echo "Installing ChipForge CLI..."
cargo install --path cli --force

echo ""
echo "====================================="
echo "✓ Installation Complete!"
echo "====================================="
echo ""
echo "Run 'chipforge --version' to verify"
echo "Run 'chipforge --help' to get started"
