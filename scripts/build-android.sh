#!/bin/bash
# Build script for Android cross-compilation

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}Building ia-get for Android targets...${NC}"

# Check for Android NDK
if [[ -z "$ANDROID_NDK_HOME" ]]; then
    echo -e "${RED}Error: ANDROID_NDK_HOME environment variable is not set${NC}"
    echo -e "${YELLOW}Please install Android NDK and set ANDROID_NDK_HOME${NC}"
    echo -e "${YELLOW}Example: export ANDROID_NDK_HOME=\$ANDROID_HOME/ndk/27.3.13750724${NC}"
    exit 1
fi

if [[ ! -d "$ANDROID_NDK_HOME" ]]; then
    echo -e "${RED}Error: Android NDK directory not found: $ANDROID_NDK_HOME${NC}"
    exit 1
fi

# Set Android API level (minimum supported version)
ANDROID_API_LEVEL=${ANDROID_API_LEVEL:-21}

# Detect host platform and architecture for NDK prebuilt toolchain
HOST_OS="$(uname -s)"
HOST_ARCH="$(uname -m)"
case "$HOST_OS" in
    Linux)
        if [[ "$HOST_ARCH" == "x86_64" ]]; then
            NDK_HOST="linux-x86_64"
        elif [[ "$HOST_ARCH" == "aarch64" || "$HOST_ARCH" == "arm64" ]]; then
            NDK_HOST="linux-arm64"
        else
            echo -e "${RED}Error: Unsupported Linux architecture: $HOST_ARCH${NC}"
            exit 1
        fi
        ;;
    Darwin)
        if [[ "$HOST_ARCH" == "x86_64" ]]; then
            NDK_HOST="darwin-x86_64"
        elif [[ "$HOST_ARCH" == "arm64" ]]; then
            NDK_HOST="darwin-arm64"
        else
            echo -e "${RED}Error: Unsupported macOS architecture: $HOST_ARCH${NC}"
            exit 1
        fi
        ;;
    *)
        echo -e "${RED}Error: Unsupported host OS: $HOST_OS${NC}"
        exit 1
        ;;
esac
NDK_BIN_DIR="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/$NDK_HOST/bin"

if [[ ! -d "$NDK_BIN_DIR" ]]; then
    echo -e "${RED}Error: NDK toolchain directory not found: $NDK_BIN_DIR${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Using Android NDK: $ANDROID_NDK_HOME${NC}"
echo -e "${GREEN}✓ Android API level: $ANDROID_API_LEVEL${NC}"

# Function to get compiler prefix for each target
get_compiler_prefix() {
    local target="$1"
    case "$target" in
        "aarch64")
            echo "aarch64-linux-android"
            ;;
        "armv7a")
            echo "armv7a-linux-androideabi"
            ;;
        "x86_64")
            echo "x86_64-linux-android"
            ;;
        "i686")
            echo "i686-linux-android"
            ;;
        *)
            echo "Unknown target: $target" >&2
            return 1
            ;;
    esac
}

# Function to get Rust target name from short target name
get_rust_target() {
    local target="$1"
    case "$target" in
        "aarch64")
            echo "aarch64-linux-android"
            ;;
        "armv7a")
            echo "armv7-linux-androideabi"
            ;;
        "x86_64")
            echo "x86_64-linux-android"
            ;;
        "i686")
            echo "i686-linux-android"
            ;;
        *)
            echo "Unknown target: $target" >&2
            return 1
            ;;
    esac
}

# Function to get Android ABI name from target name
get_android_abi() {
    local target="$1"
    case "$target" in
        "aarch64")
            echo "arm64-v8a"
            ;;
        "armv7a")
            echo "armeabi-v7a"
            ;;
        "x86_64")
            echo "x86_64"
            ;;
        "i686")
            echo "x86"
            ;;
        *)
            echo "Unknown target: $target" >&2
            return 1
            ;;
    esac
}

# Configure cross-compilation environment variables
for target in aarch64 armv7a x86_64 i686; do
    compiler_prefix=$(get_compiler_prefix "$target")
    rust_target=$(get_rust_target "$target")
    
    # Set CC, CXX, and AR variables
    export "CC_${rust_target//-/_}"="$NDK_BIN_DIR/${compiler_prefix}${ANDROID_API_LEVEL}-clang"
    export "CXX_${rust_target//-/_}"="$NDK_BIN_DIR/${compiler_prefix}${ANDROID_API_LEVEL}-clang++"
    export "AR_${rust_target//-/_}"="$NDK_BIN_DIR/llvm-ar"
    
    # Set Cargo linker variables
    rust_target_upper=$(echo "$rust_target" | tr '[:lower:]' '[:upper:]' | tr '-' '_')
    export "CARGO_TARGET_${rust_target_upper}_LINKER"="$NDK_BIN_DIR/${compiler_prefix}${ANDROID_API_LEVEL}-clang"
done

# Verify compilers exist
for target in aarch64 armv7a x86_64 i686; do
    compiler_prefix=$(get_compiler_prefix "$target")
    compiler="$NDK_BIN_DIR/${compiler_prefix}${ANDROID_API_LEVEL}-clang"
    
    if [[ ! -f "$compiler" ]]; then
        echo -e "${RED}Error: Compiler not found: $compiler${NC}"
        echo -e "${YELLOW}Available compilers in NDK:${NC}"
        ls -1 "$NDK_BIN_DIR"/*clang | head -10
        exit 1
    fi
done

echo -e "${GREEN}✓ All required NDK compilers found${NC}"

# Android targets to build for
TARGET_NAMES=(aarch64 armv7a x86_64 i686)

# Create output directory
mkdir -p target/android

# Build for each target
for target_name in "${TARGET_NAMES[@]}"; do
    target=$(get_rust_target "$target_name")
    echo -e "${BLUE}Building for ${target}...${NC}"
    
    # Check if target is installed
    if ! rustup target list --installed | grep -q "$target"; then
        echo -e "${BLUE}Installing target ${target}...${NC}"
        rustup target add "$target"
    fi
    
    # Build the library
    if cargo build --target "$target" --release --features ffi; then
        echo -e "${GREEN}✓ Successfully built for ${target}${NC}"
        
        # Copy library to organized output directory
        android_abi=$(get_android_abi "$target_name")
        mkdir -p "target/android/$android_abi"
        cp "target/${target}/release/libia_get.so" "target/android/$android_abi/"
    else
        echo -e "${RED}✗ Failed to build for ${target}${NC}"
        exit 1
    fi
done

echo -e "${GREEN}✓ All Android targets built successfully!${NC}"
echo -e "${BLUE}Libraries available in target/android/${NC}"

# Generate header file for FFI
echo -e "${BLUE}Generating C header file...${NC}"
if command -v cbindgen &> /dev/null; then
    cbindgen --config cbindgen.toml --crate ia-get --output target/android/ia_get.h
    echo -e "${GREEN}✓ Header file generated: target/android/ia_get.h${NC}"
else
    echo -e "${RED}⚠ cbindgen not found. Install with: cargo install cbindgen${NC}"
fi

echo -e "${GREEN}Build complete!${NC}"