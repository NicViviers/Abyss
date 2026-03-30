#!/bin/bash
set -e

# Define paths
PROJECT_ROOT=~/IdeaProjects/Abyss
KERNEL_DIR=$PROJECT_ROOT/kernel
BUILD_DIR=$PROJECT_ROOT/build

export PICO_COMPILER=pico_arm_cortex_m33_gcc

# Check if the user wants a clean build (pass "clean" as argument)
if [ "$1" == "clean" ]; then
    echo "Cleaning build directory..."
    rm -rf "$BUILD_DIR"
fi

# 1. Configure CMake (Only if build dir doesn't exist)
if [ ! -d "$BUILD_DIR" ]; then
    echo "Configuring CMake..."
    mkdir -p "$BUILD_DIR"
    cd "$BUILD_DIR"
    cmake -DPICO_BOARD=pico2 ..
else
    # If CMakeLists.txt changed, make will handle it, but we cd in.
    cd "$BUILD_DIR"
fi

# 2. Build Rust (Cargo handles incremental builds automatically)
echo "Building Rust Kernel..."
cd "$KERNEL_DIR"
cargo build --release --target thumbv8m.main-none-eabi

# 3. Final Link/Build (Make only recompiles changed C files)
echo "Linking and Flashing..."
cd "$BUILD_DIR"
make abyss -j$(nproc)

# 4. Flash UF2
# Only try to load if the file actually exists (make might fail)
if [ -f "examples/abyss/abyss.uf2" ]; then
    sudo picotool load -x examples/abyss/abyss.uf2
else
    echo "Build failed, UF2 not found."
    exit 1
fi