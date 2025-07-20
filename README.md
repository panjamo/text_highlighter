# Text Highlighter for Zed

A Zed extension that aims to replicate the functionality of VS Code's text-marker extension, allowing you to highlight and navigate through text patterns in your code.

> **IMPORTANT**: Due to current limitations in the Zed extension API (0.5.0), the highlighting feature cannot directly modify the editor's visual display. This extension tracks highlight patterns internally but cannot apply visual highlights yet. We're waiting for future Zed API enhancements to fully implement this feature.

## Features

- **Text Highlighting**: Highlight any text pattern in your editor
- **Toggle Highlighting**: Re-run the highlight command to remove existing highlights  
- **Multiple Colors**: Automatically cycles through 8 different highlight colors
- **Navigation**: Jump to next/previous highlighted text
- **Pattern Options**: Support for case-sensitive, whole-word, and regex matching
- **Clear All**: Remove all highlights at once

## Available Commands

All commands are available as slash commands in Zed:

### `/highlight [options] <pattern>`
Toggle highlighting for the specified text pattern.

**Options:**
- `--case-sensitive`: Enable case-sensitive matching
- `--whole-word`: Match whole words only  
- `--regex`: Treat pattern as regular expression

**Examples:**
```
/highlight TODO
/highlight --case-sensitive Error
/highlight --regex \b(function|class)\b
```

### `/next_highlight`
Navigate to the next occurrence of any highlighted text.

### `/prev_highlight`  
Navigate to the previous occurrence of any highlighted text.

### `/clear_highlights`
Remove all text highlights from the current editor.

## Installation

> **NOTE**: This extension currently has limited functionality due to Zed API restrictions.

1. Clone or download this extension
2. Build the extension for WebAssembly target:
   ```bash
   rustup target add wasm32-wasip1
   cargo build --release --target wasm32-wasip1
   ```
3. Copy the compiled WASM file to the extension directory:
   ```bash
   cp target/wasm32-wasip1/release/high_lighter.wasm extension.wasm
   ```
4. Install the extension in Zed:
   - Open Zed's settings with `Cmd+,` (Mac) or `Ctrl+,` (Windows/Linux)
   - Navigate to the Extensions tab
   - Click "Add Extension" and browse to the extension directory
   - Select the directory containing `extension.toml` and `extension.wasm`

## API Limitations

Currently, the Zed extension API (0.5.0) does not provide:

1. Direct access to editor text decoration or highlighting
2. Methods to apply visual styles to matched text patterns
3. Methods to execute built-in commands like search from extensions

These limitations prevent this extension from applying visual highlights to the editor content. The extension will store highlight patterns internally, but cannot display them visually until Zed enhances its extension API.

For publishing to the official Zed extensions repository:
1. Fork the [zed-industries/extensions](https://github.com/zed-industries/extensions) repository
2. Add your extension as a Git submodule
3. Update the `extensions.toml` file
4. Open a pull request

## Development

This extension is written in Rust and compiled to WebAssembly. Key files:

- `src/lib.rs` - Main extension logic
- `extension.toml` - Extension metadata and slash command definitions
- `Cargo.toml` - Rust dependencies and build configuration

### Building
```bash
# Setup WebAssembly target (one-time)
rustup target add wasm32-wasip1

# Build for development
cargo build --target wasm32-wasip1

# Build for release
cargo build --release --target wasm32-wasip1

# Copy the WASM file to the extension directory
cp target/wasm32-wasip1/release/deps/high_lighter.wasm extension.wasm
```

### Testing
1. Install the extension in Zed using the instructions in the Installation section
2. Open a text file in Zed
3. Type `/highlight` followed by the text you want to highlight
4. Currently, the text will NOT be highlighted in the editor due to API limitations
5. The extension will store the pattern internally for future use
6. Commands `/next_highlight`, `/prev_highlight` and `/clear_highlights` are registered but have limited functionality

**Note**: Full functionality will be implemented when the Zed extension API provides the necessary capabilities.

## Architecture

The extension uses:
- **Slash Commands** for user interaction
- **RwLock** for thread-safe state management
- **HashMap** to store highlight patterns by category
- **Color cycling** through predefined highlight colors
- Internal pattern storage for future highlighting capabilities

Highlight patterns store:
- Pattern text
- Assigned color  
- Case sensitivity setting
- Whole word setting
- Regex flag

### Implementation Notes

The extension works by:
1. Storing highlight patterns in memory when `/highlight` is called
2. Supporting toggle behavior (calling highlight with the same pattern removes it)
3. Managing multiple highlight patterns with different colors
4. **Pending Zed API enhancements**: Visual highlighting of matched text is not yet possible

Currently, this extension tracks patterns internally but cannot apply visual highlights until Zed enhances its extension API with text decoration capabilities.

## License

This extension follows the same licensing as the Zed editor project.