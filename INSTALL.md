# Installation Guide for Text Highlighter Extension

## Prerequisites
- Zed editor installed
- This extension built with `cargo build --target wasm32-wasip1 --release`

## Installation Steps

### Method 1: Dev Extension (Recommended for testing)
1. Open Zed
2. Press `Ctrl+Shift+P` (Windows/Linux) or `Cmd+Shift+P` (macOS)
3. Type "Extensions: Install Dev Extension" and select it
4. Navigate to this directory (`high_lighter`)
5. Select the directory
6. **Restart Zed** (important!)

### Method 2: Manual Installation
1. Copy this entire directory to Zed's extensions directory:
   - **Windows**: `%APPDATA%\Zed\extensions\`
   - **macOS**: `~/Library/Application Support/Zed/extensions/`
   - **Linux**: `~/.config/zed/extensions/`
2. Restart Zed

## Verification
After installation and restart, the following slash commands should be available:

- `/highlight <pattern>` - Toggle highlighting for text
- `/next_highlight` - Navigate to next highlight
- `/prev_highlight` - Navigate to previous highlight  
- `/clear_highlights` - Clear all highlights

## Testing
1. Open any text file in Zed
2. Type `/highlight` in the command palette
3. Enter some text to highlight (e.g., "function")
4. The text should be highlighted in the editor

## Troubleshooting

### Commands Don't Appear
- Ensure Zed is fully restarted
- Check Extensions panel in Zed settings
- Verify `extension.wasm` file exists and is ~500KB+

### Extension Doesn't Load
- Check Zed's console for errors
- Ensure `extension.toml` has correct syntax
- Verify WebAssembly file was built correctly

### Build Issues
- Ensure `wasm32-wasip1` target is installed: `rustup target add wasm32-wasip1`
- Use `./build.sh` script if on Unix systems
- On Windows: `cargo build --target wasm32-wasip1 --release && copy target\\wasm32-wasip1\\release\\high_lighter.wasm extension.wasm`

## Support
If the extension doesn't work, check:
1. Zed version compatibility
2. Extension file permissions
3. Zed's extension logs/console output