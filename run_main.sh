#!/usr/bin/env bash
set -e  # Exit immediately on any error

# Define constants
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="$PROJECT_ROOT/build/artifacts"
VERSION_FILE="$PROJECT_ROOT/build/version.txt"

# Check if the build exists
if [[ ! -f "$VERSION_FILE" ]]; then
    echo "Error: No build version found. Please run './build.sh' first."
    exit 1
fi

# Get the latest version
VERSION=$(cat "$VERSION_FILE")
ARTIFACTS_DIR="$BUILD_DIR/$VERSION"

# Detect the main binary
MAIN_BINARY=$(find "$ARTIFACTS_DIR" -maxdepth 1 -type f -executable | head -n 1)

if [[ -z "$MAIN_BINARY" ]]; then
    echo "Error: No binary found in $ARTIFACTS_DIR"
    echo "Run './build.sh' to build the project."
    exit 1
fi

# Run the binary
echo "Launching project from $MAIN_BINARY..."
"$MAIN_BINARY" "$@"
