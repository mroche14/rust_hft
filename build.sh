#!/usr/bin/env bash
set -e  # Exit immediately on any error
set -o pipefail  # Ensure errors in pipelines are propagated

# Constants
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="$SCRIPT_DIR/build"  # Output folder for artifacts
LOG_DIR="$BUILD_DIR/logs"      # Logs for build steps
VERSION_FILE="$BUILD_DIR/version.txt"  # File to track current version

# Helper function: Print a section header
print_section() {
  echo
  echo "========================================"
  echo "$1"
  echo "========================================"
}

# Step 1: Create necessary folders
print_section "Setting up build environment"
mkdir -p "$BUILD_DIR" "$LOG_DIR"

# Step 2: Detect or increment version
if [[ -f "$VERSION_FILE" ]]; then
  CURRENT_VERSION=$(cat "$VERSION_FILE")
else
  CURRENT_VERSION="0.0.1"
fi

# Increment version for this build (e.g., bump minor version)
IFS='.' read -r major minor patch <<<"$CURRENT_VERSION"
NEW_VERSION="$major.$((minor + 1)).$patch"
echo "$NEW_VERSION" > "$VERSION_FILE"

echo "Current Version: $CURRENT_VERSION"
echo "New Version: $NEW_VERSION"

# Step 3: Clean up old artifacts
print_section "Cleaning up previous builds"
rm -rf "$BUILD_DIR/artifacts"
mkdir -p "$BUILD_DIR/artifacts"

# Step 4: Build the workspace
print_section "Building the project (all crates)"
cargo build --release 2>&1 | tee "$LOG_DIR/build.log"

# Step 5: Detect and Copy Build Artifacts
print_section "Organizing build artifacts"
ARTIFACTS_DIR="$BUILD_DIR/artifacts/$NEW_VERSION"
mkdir -p "$ARTIFACTS_DIR"

# Detect the main binary in the target directory
MAIN_BINARY=$(find "$SCRIPT_DIR/target/release/" -maxdepth 1 -type f -executable | grep -v '\.so' | head -n 1)

if [[ -z "$MAIN_BINARY" ]]; then
  echo "Error: No executable binary found in target/release/. Ensure you have a binary crate with src/main.rs."
  exit 1
fi

# Copy the binary and other artifacts
cp "$MAIN_BINARY" "$ARTIFACTS_DIR/"
cp -r "$SCRIPT_DIR/target/release/deps" "$ARTIFACTS_DIR/"  # Copy dependencies, if any

echo "Build artifacts stored in: $ARTIFACTS_DIR"

# Step 6: Generate metadata
print_section "Generating build metadata"
METADATA_FILE="$ARTIFACTS_DIR/build_metadata.txt"
{
  echo "Build Date: $(date)"
  echo "Git Commit: $(git rev-parse HEAD 2>/dev/null || echo 'Not a Git repo')"
  echo "Version: $NEW_VERSION"
} > "$METADATA_FILE"

cat "$METADATA_FILE"

# Step 7: Run tests
print_section "Running tests (all crates, including doc-tests)"
if ! cargo test --release 2>&1 | tee "$LOG_DIR/test.log"; then
  echo "Some tests failed! Check logs in $LOG_DIR."
  echo "Hint: Run 'cargo test --doc' to debug doc-tests."
  exit 1
fi

# Step 8: Success message
print_section "Build and tests completed successfully"
echo "Build artifacts and metadata are available in: $ARTIFACTS_DIR"
