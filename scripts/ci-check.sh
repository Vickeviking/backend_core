#!/bin/bash
set -e

echo "Running format check..."
cargo fmt --all -- --check

echo "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "Running tests..."
cargo test --all-targets

echo "All checks passed!"
