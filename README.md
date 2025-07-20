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
2. Build the extension: `cargo build --release`
3. Follow [Zed's extension installation guide](https://zed.dev/docs/extensions) to install locally

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
cargo build
```

### Testing
Test the slash commands in Zed by typing `/highlight`, `/next_highlight`, etc. in the command palette.

## Architecture

The extension uses:
- **Slash Commands** for user interaction
- **RwLock** for thread-safe state management
- **HashMap** to store highlight patterns by category
- **Color cycling** through predefined highlight colors

Highlight patterns store:
- Pattern text
- Assigned color  
- Case sensitivity setting
- Whole word setting
- Regex flag

## License

This extension follows the same licensing as the Zed editor project.