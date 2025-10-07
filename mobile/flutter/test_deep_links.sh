#!/bin/bash
# Deep Link Testing Script for Internet Archive Helper
# This script helps test deep link functionality on connected Android devices

echo "========================================"
echo "Internet Archive Helper - Deep Link Test"
echo "========================================"
echo ""

# Check if ADB is available
if ! command -v adb &> /dev/null; then
    echo "ERROR: ADB not found in PATH"
    echo "Please install Android SDK Platform Tools"
    exit 1
fi

# Get connected devices
DEVICES=$(adb devices | tail -n +2 | grep -w "device")
if [ -z "$DEVICES" ]; then
    echo "ERROR: No Android devices connected"
    echo "Please connect a device or start an emulator"
    exit 1
fi

echo "Connected device found!"
echo ""

# Package name
PACKAGE="com.gameaday.internet_archive_helper"

# Test URLs
declare -a TEST_NAMES=(
    "Custom scheme (iaget://)"
    "Archive.org details page"
    "Archive.org download page"
)

declare -a TEST_URLS=(
    "iaget://commute_test"
    "https://archive.org/details/commute_test"
    "https://archive.org/download/commute_test"
)

declare -a TEST_DESCRIPTIONS=(
    "Tests custom deep link scheme"
    "Tests handling of archive.org details URL"
    "Tests handling of archive.org download URL"
)

echo "Available test cases:"
for i in "${!TEST_NAMES[@]}"; do
    echo "  [$((i+1))] ${TEST_NAMES[$i]}"
    echo "      ${TEST_DESCRIPTIONS[$i]}"
done
echo "  [A] Run all tests"
echo "  [Q] Quit"
echo ""

read -p "Select test to run: " choice

if [ "$choice" = "Q" ] || [ "$choice" = "q" ]; then
    exit 0
fi

# Determine which tests to run
if [ "$choice" = "A" ] || [ "$choice" = "a" ]; then
    START_INDEX=0
    END_INDEX=${#TEST_URLS[@]}
else
    START_INDEX=$((choice-1))
    END_INDEX=$((choice))
    
    if [ $START_INDEX -lt 0 ] || [ $START_INDEX -ge ${#TEST_URLS[@]} ]; then
        echo "Invalid selection"
        exit 1
    fi
fi

echo ""
echo "Running tests..."
echo ""

for (( i=$START_INDEX; i<$END_INDEX; i++ )); do
    echo "Testing: ${TEST_NAMES[$i]}"
    echo "URL: ${TEST_URLS[$i]}"
    
    # Send deep link via ADB
    CMD="adb shell am start -W -a android.intent.action.VIEW -d '${TEST_URLS[$i]}'"
    echo "Command: $CMD"
    
    if adb shell am start -W -a android.intent.action.VIEW -d "${TEST_URLS[$i]}" &> /dev/null; then
        echo "✓ Command executed successfully"
        
        # Wait a moment for app to respond
        sleep 2
        
        # Check if app is in foreground
        FOREGROUND=$(adb shell "dumpsys activity activities | grep -E 'mResumedActivity|mFocusedActivity'")
        if echo "$FOREGROUND" | grep -q "$PACKAGE"; then
            echo "✓ App is now in foreground"
        else
            echo "⚠ App may not be in foreground"
            echo "  Check device to verify behavior"
        fi
    else
        echo "✗ Command failed"
    fi
    
    echo ""
    
    if [ $((END_INDEX - START_INDEX)) -gt 1 ]; then
        read -p "Press Enter to continue to next test..."
        echo ""
    fi
done

echo "========================================"
echo "Testing complete!"
echo ""
echo "Manual verification steps:"
echo "1. Check that the app opened"
echo "2. Verify the archive detail screen is shown"
echo "3. Confirm the correct archive loaded (commute_test)"
echo "========================================"
