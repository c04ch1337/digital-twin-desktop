#!/bin/bash
set -e

# Digital Twin Desktop Production Build Script

echo "Building Digital Twin Desktop for production..."

# Check if .env.local exists
if [ ! -f .env.local ]; then
    echo "Warning: .env.local file not found."
    echo "Using default configuration values."
fi

# Parse command line arguments
RELEASE_TYPE="release"
TARGET_PLATFORM="all"

# Process command line arguments
while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
        --debug)
            RELEASE_TYPE="debug"
            shift
            ;;
        --platform)
            TARGET_PLATFORM="$2"
            shift
            shift
            ;;
        --help)
            echo "Usage: ./scripts/build.sh [options]"
            echo ""
            echo "Options:"
            echo "  --debug                Build in debug mode"
            echo "  --platform <platform>  Build for specific platform (windows|macos|linux|all)"
            echo "  --help                 Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $key"
            echo "Use --help for usage information."
            exit 1
            ;;
    esac
done

# Validate platform
if [[ "$TARGET_PLATFORM" != "windows" && "$TARGET_PLATFORM" != "macos" && "$TARGET_PLATFORM" != "linux" && "$TARGET_PLATFORM" != "all" ]]; then
    echo "Error: Invalid platform '$TARGET_PLATFORM'"
    echo "Valid platforms: windows, macos, linux, all"
    exit 1
fi

# Build frontend
echo "Building frontend..."
cd ui
npm install
npm run build
cd ..

# Build Tauri application
echo "Building Tauri application ($RELEASE_TYPE mode) for platform: $TARGET_PLATFORM..."

BUILD_ARGS=""

# Set build arguments based on release type
if [[ "$RELEASE_TYPE" == "debug" ]]; then
    BUILD_ARGS="--debug"
fi

# Set build arguments based on target platform
if [[ "$TARGET_PLATFORM" != "all" ]]; then
    BUILD_ARGS="$BUILD_ARGS --target $TARGET_PLATFORM"
fi

# Execute the build
cargo tauri build $BUILD_ARGS

echo "Build complete!"

# Show output location
if [[ "$RELEASE_TYPE" == "debug" ]]; then
    echo "Debug build available in: target/debug/"
else
    echo "Release build available in: target/release/"
    
    # Show bundle location based on platform
    case $TARGET_PLATFORM in
        windows)
            echo "Windows installer: target/release/bundle/msi/"
            ;;
        macos)
            echo "macOS bundle: target/release/bundle/macos/"
            ;;
        linux)
            echo "Linux packages: target/release/bundle/deb/ and target/release/bundle/appimage/"
            ;;
        all)
            echo "Bundles available in: target/release/bundle/"
            ;;
    esac
fi

echo "To run the built application:"
if [[ "$RELEASE_TYPE" == "debug" ]]; then
    echo "  ./target/debug/digital-twin-desktop"
else
    echo "  ./target/release/digital-twin-desktop"
fi