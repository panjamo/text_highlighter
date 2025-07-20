Summary of the Fix

I've implemented the best possible solution given the current API limitations:

### What I've Fixed

1. **Better Error Messages**: The extension now clearly explains the API limitations when commands are used

2. **Practical Workaround Instructions**: I've added detailed guidance on how to use Zed's built-in search as a workaround:
   - After running `/highlight hallo`, use Ctrl+F (or Cmd+F) and type "hallo"
   - Zed will highlight matches with its built-in search functionality
   - Use F3/Shift+F3 to navigate between matches

3. **Comprehensive Documentation**: Updated README with:
   - Clear explanation of current limitations
   - Step-by-step workaround instructions
   - Comparison of current vs. future functionality

4. **Cleaner Code**: Fixed unused variable warnings and improved code clarity

### How to Use the Extension Now

1. Run `/highlight hallo` to store the pattern
2. The extension will confirm the pattern is stored
3. Press Ctrl+F (or Cmd+F on Mac) and type "hallo" 
4. Zed will highlight all occurrences in the editor
5. Use F3 and Shift+F3 to navigate between matches

### Why It Can't Directly Highlight Text

The Zed extension API (version 0.5.0) simply doesn't provide any methods or functions that allow extensions to:
- Apply visual decorations to text
- Highlight portions of text
- Execute built-in commands that could achieve highlighting

This is a fundamental limitation of the current API, not an issue with the extension code.

When Zed enhances its API to support text decorations in the future, the extension is structured to easily implement direct highlighting functionality.