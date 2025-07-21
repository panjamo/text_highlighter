# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Zed text highlighter extension that replicates VS Code's text-marker functionality. Due to Zed API limitations (v0.5.0), the extension currently stores highlight patterns but cannot apply visual highlights directly - users must use Zed's built-in search (Ctrl+F/Cmd+F) as a workaround.

## Build Commands

### Building the Extension
```bash
# Use the build script (recommended)
bash build.sh

# Manual build commands
rustup target add wasm32-wasip1
cargo build --target wasm32-wasip1 --release
cp target/wasm32-wasip1/release/high_lighter.wasm extension.wasm

# Build LSP server
cd highlight-lsp && cargo build --release && cd ..
```

### Testing
- Install as dev extension in Zed: `Ctrl+Shift+P` → "Extensions: Install Dev Extension"
- Test commands: `/highlight pattern`, `/next_highlight`, `/prev_highlight`, `/clear_highlights`
- Use Ctrl+F/Cmd+F to see actual highlighting via Zed's search

## Architecture

### Dual Component System
The project consists of two main components:

1. **Zed Extension** (`src/lib.rs`)
   - WASM-compiled Rust extension using `zed_extension_api`
   - Implements slash commands for pattern management
   - Thread-safe state management with `RwLock<HashMap>`
   - Color cycling system (8 predefined colors)

2. **LSP Server** (`highlight-lsp/`)
   - Standalone Language Server Protocol implementation
   - Built with `tower-lsp` and `tokio`
   - Intended for future direct highlighting when API allows

### Key Data Structures
- `HighlightPattern`: Stores pattern text, color, and options (case-sensitive, whole-word, regex)
- `HighLighterState`: Thread-safe HashMap storing patterns by category with color cycling
- Pattern storage supports toggle behavior (add/remove same pattern)

### Extension Integration
- `extension.toml`: Defines slash commands and LSP server configuration
- LSP server binary is copied to extension root during build
- Extension references LSP via `./highlight-lsp` command

### API Limitations
Current Zed extension API (v0.5.0) lacks:
- Text decoration/highlighting methods
- Visual styling capabilities
- Direct editor manipulation

This necessitates the workaround workflow:
1. `/highlight pattern` → stores pattern
2. `Ctrl+F` + type pattern → see visual highlights
3. `F3`/`Shift+F3` → navigate matches

### File Structure
- `src/lib.rs` - Main extension logic with slash command handlers
- `extension.toml` - Extension manifest with command definitions and LSP config
- `highlight-lsp/src/main.rs` - LSP server implementation
- `build.sh` - Build script that compiles both WASM extension and LSP binary
- `README.md` - Comprehensive documentation with API limitations and workarounds