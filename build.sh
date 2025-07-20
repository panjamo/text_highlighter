#!/bin/bash

# Build script for Zed Text Highlighter Extension

echo "Building Text Highlighter extension for Zed..."

# Ensure wasm32-wasip1 target is installed
if ! rustup target list --installed | grep -q "wasm32-wasip1"; then
    echo "Installing wasm32-wasip1 target..."
    rustup target add wasm32-wasip1
fi

# Build the extension
echo "Compiling to WebAssembly..."
cargo build --target wasm32-wasip1 --release

# Copy the wasm file to the expected location
echo "Copying WebAssembly file..."
cp target/wasm32-wasip1/release/high_lighter.wasm extension.wasm

echo "Build complete! Files ready:"
echo "  - extension.toml (manifest)"
echo "  - extension.wasm (compiled extension)"
echo ""
echo "To install in Zed:"
echo "  1. Open Zed"
echo "  2. Press Cmd+Shift+P (Ctrl+Shift+P on Windows/Linux)"
echo "  3. Type 'install dev extension'"
echo "  4. Select this directory"
echo ""
echo "The slash commands should then be available:"
echo "  /highlight <pattern>    - Toggle highlighting"
echo "  /next_highlight         - Go to next highlight"
echo "  /prev_highlight         - Go to previous highlight"
echo "  /clear_highlights       - Clear all highlights"