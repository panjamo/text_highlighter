# Text Highlighter for Zed

A Zed extension that aims to replicate the functionality of VS Code's text-marker extension for highlighting and finding text patterns in your code.

> **IMPORTANT**: Due to current limitations in the Zed extension API (0.5.0), direct text highlighting from extensions is not possible. This extension stores highlight patterns but cannot apply visual highlights to the text automatically.

## Workaround for Highlighting

To see visual highlighting of your patterns:

1. Store patterns using the `/highlight` command
2. Press **Ctrl+F** (Windows/Linux) or **Cmd+F** (Mac) to open Zed's search
3. Type or paste the pattern you want to highlight
4. Zed will highlight all matches

## Features

- **Pattern Storage**: Store text patterns for quick access
- **Toggle Patterns**: Re-run the highlight command to remove stored patterns  
- **Multiple Colors**: Assigns from 8 different colors (ready for when API supports it)
- **Pattern Management**: Tracks case-sensitive, whole-word, and regex settings
- **Clear All**: Remove all stored highlight patterns

## Available Commands

All commands are available as slash commands in Zed:

### `/highlight [options] <pattern>`
Store the specified text pattern for highlighting.

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

**Note**: After running the command, use Zed's built-in search (Ctrl+F/Cmd+F) with the same pattern to see visual highlighting.

### `/next_highlight`
Intended to navigate to the next occurrence of highlighted text.
(Use F3 in Zed's search for now as a workaround)

### `/prev_highlight`  
Intended to navigate to the previous occurrence of highlighted text.
(Use Shift+F3 in Zed's search for now as a workaround)

### `/clear_highlights`
Clear all stored highlight patterns from memory.
(Use Esc in Zed's search to clear highlights as a workaround)

## Installation

> **IMPORTANT**: This extension currently has limited functionality due to Zed API restrictions. It stores patterns but cannot apply visual highlighting.

1. Clone or download this extension
2. Build the extension for WebAssembly target:
   ```bash
   rustup target add wasm32-wasip1
   cargo build --release --target wasm32-wasip1
   ```
3. Copy the compiled WASM file to the extension directory:
   ```bash
   copy target\wasm32-wasip1\release\high_lighter.wasm extension.wasm
   ```
4. Install the extension in Zed:
   - Open Zed's settings with `Cmd+,` (Mac) or `Ctrl+,` (Windows/Linux)
   - Navigate to the Extensions tab
   - Click "Add Extension" and browse to the extension directory
   - Select the directory containing `extension.toml` and `extension.wasm`
   
## API Limitations

The current Zed extension API (v0.5.0) does not provide:

1. Methods to apply text decorations or highlighting to editor content
2. Functions to visually mark or color text in the editor
3. Commands to modify the editor's visual display
4. Direct access to search functionality that could be used for highlighting

Until these capabilities are added to the Zed extension API, this extension can only:
- Store and manage highlight patterns in memory
- Provide feedback when patterns are added or removed
- Track pattern options (case sensitivity, whole word, regex)

## API Limitations & Workarounds

### Current Limitations

The Zed extension API (0.5.0) does not provide:

1. Direct access to editor text decoration or highlighting
2. Methods to apply visual styles to matched text patterns
3. Methods to execute built-in commands like search from extensions

### Workarounds

Since the extension cannot directly highlight text, use these workarounds:

| Extension Command | Manual Workaround |
|-------------------|------------------|
| `/highlight pattern` | Press Ctrl+F/Cmd+F, type the pattern |
| `/next_highlight` | Press F3 to find next match |
| `/prev_highlight` | Press Shift+F3 to find previous match |
| `/clear_highlights` | Press Esc to clear search/highlights |

The extension stores patterns for easy recall, but you must use Zed's search to see visual highlighting.

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
2. Open a text file in Zed (like test.txt containing "hallo welt")
3. Type `/highlight hallo` to store the pattern
4. The extension will confirm the pattern is stored
5. Press **Ctrl+F** (or **Cmd+F** on Mac) and type "hallo"
6. Zed will highlight all occurrences of "hallo" in the file
7. Use F3 and Shift+F3 to navigate between matches

This combination of the extension (for pattern storage) and Zed's built-in search (for visual highlighting) provides a workable solution until the API is enhanced.

## Architecture

The extension uses:
- **Slash Commands** for user interaction
- **RwLock** for thread-safe state management
- **HashMap** to store highlight patterns by category
- **Color cycling** through predefined highlight colors (ready for when API supports it)

Highlight patterns store:
- Pattern text
- Assigned color (for future highlighting)
- Case sensitivity setting
- Whole word setting
- Regex flag

### Implementation Limitations

The extension works by:
1. Storing highlight patterns in memory when `/highlight` is called
2. Supporting toggle behavior (calling highlight with the same pattern removes it)
3. Managing multiple highlight patterns with different colors
4. Providing detailed instructions for using Zed's built-in search as a workaround

**Workaround Workflow:**
1. Use `/highlight` to store patterns
2. Use Zed's Ctrl+F/Cmd+F search to actually see the highlighting
3. Use F3/Shift+F3 to navigate between matches

### Current Implementation vs. Future Goals

#### Current Implementation:
1. Stores highlight patterns in memory when `/highlight` is called
2. Supports toggle behavior (calling highlight with the same pattern removes it)
3. Manages multiple highlight patterns with different colors
4. Provides guidance on using Zed's search as a workaround

#### Future Goals (when API allows):
1. Direct visual highlighting without requiring manual search
2. Multiple highlight colors applied simultaneously
3. Navigation between different highlight patterns
4. Automatic persistence of highlights between sessions

Until Zed enhances its API with text decoration capabilities, use the workarounds described above.

## License

This extension follows the same licensing as the Zed editor project.