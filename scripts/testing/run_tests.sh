#!/usr/bin/env bash
# Run all ChipForge tests

set -e

echo "Running ChipForge Tests..."
echo ""

# Run Rust tests
echo "→ Running Rust tests..."
cargo test --workspace --all-features

# Run clippy
echo ""
echo "→ Running clippy..."
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run rustfmt check
echo ""
echo "→ Checking formatting..."
cargo fmt --all -- --check

echo ""
echo "✓ All tests passed!"
