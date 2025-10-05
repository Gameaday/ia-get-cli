#!/bin/bash
# Build script for Android cross-compilation (DEPRECATED)
# 
# NOTE: This script is DEPRECATED and not needed for the Flutter mobile app.
# The Flutter mobile app now uses a pure Dart implementation with no native dependencies.
# 
# For Flutter Android APK builds, use: ./scripts/build-mobile.sh
# 
# This script remains for reference purposes only.

set -e

error() {
    echo "❌ ERROR: This script is deprecated!"
    echo ""
    echo "The Flutter mobile app now uses a pure Dart implementation."
    echo "No native library compilation is needed."
    echo ""
    echo "To build the Flutter Android app, use:"
    echo "  ./scripts/build-mobile.sh --development"
    echo "  ./scripts/build-mobile.sh --production"
    echo ""
    exit 1
}

error