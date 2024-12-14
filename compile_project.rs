#!/bin/bash

# Exit on any error
set -e

# Define the workspace root directory
WORKSPACE_DIR=$(pwd)

echo "Starting build for Rust project in workspace: $WORKSPACE_DIR"

# Step 1: Clean any previous builds
echo "Cleaning previous build artifacts..."
cargo clean

# Step 2: Build the entire workspace
echo "Building the Rust workspace..."
cargo build --workspace --release

# Step 3: Run workspace tests
echo "Running tests..."
cargo test --workspace

# Step 4: Check the formatting
echo "Checking formatting..."
cargo fmt --check

# Step 5: Run clippy for linting
echo "Running Clippy linter..."
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo "Build completed successfully!"
