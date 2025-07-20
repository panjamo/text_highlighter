# Text Highlighter for Zed

A Zed extension that replicates the functionality of VS Code's text-marker extension, allowing you to highlight and navigate through text patterns in your code.

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

1. Clone or download this extension
2. Build the extension for WebAssembly target:
   ```bash
   rustup target add wasm32-wasip1
   cargo build --release --target wasm32-wasip1
   ```
3. Copy the compiled WASM file to the extension directory:
   ```bash
   cp target/wasm32-wasip1/release/deps/high_lighter.wasm extension.wasm
   ```
4. Install the extension in Zed:
   - Open Zed's settings with `Cmd+,` (Mac) or `Ctrl+,` (Windows/Linux)
   - Navigate to the Extensions tab
   - Click "Add Extension" and browse to the extension directory
   - Select the directory containing `extension.toml` and `extension.wasm`

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
4. The text should be highlighted in the editor
5. Test navigation with `/next_highlight` and `/prev_highlight`
6. Clear highlights with `/clear_highlights`

## Architecture

The extension uses:
- **Slash Commands** for user interaction
- **RwLock** for thread-safe state management
- **HashMap** to store highlight patterns by category
- **Color cycling** through predefined highlight colors
- **Zed's search API** to apply highlights in the editor

Highlight patterns store:
- Pattern text
- Assigned color  
- Case sensitivity setting
- Whole word setting
- Regex flag

### Implementation Notes

The extension works by:
1. Storing highlight patterns in memory when `/highlight` is called
2. Using Zed's built-in search functionality to visually highlight matches
3. Supporting toggle behavior (calling highlight with the same pattern removes it)
4. Managing multiple highlight patterns with different colors

## License

This extension follows the same licensing as the Zed editor project.