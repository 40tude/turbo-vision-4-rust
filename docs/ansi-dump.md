# ANSI Dump Feature

The ANSI dump feature allows you to export terminal buffers to text files with ANSI color codes for debugging purposes. This is useful for:

- Debugging UI layout issues
- Capturing screenshots of terminal applications
- Testing color schemes
- Creating visual regression tests
- Documenting UI behavior

## Quick Start: Keyboard Shortcuts

The easiest way to capture a screen dump is to use one of these keyboard shortcuts at any time while your application is running:

- **F12** - Quick screen dump
- **ESC + Shift + S** - Alternative shortcut (useful on systems where F12 is reserved)

Both shortcuts will:
1. Flash the screen briefly (visual feedback)
2. Save the current screen to `screen-dump.txt`

```rust
use turbo_vision::app::Application;

let mut app = Application::new()?;
// ... set up your UI ...
app.run();  // Press F12 or ESC+Shift+S during execution
```

## Usage

### Dumping the Entire Screen

```rust
use turbo_vision::terminal::Terminal;

let terminal = Terminal::init()?;
// ... draw your UI ...
terminal.dump_screen("debug_screen.ans")?;
```

### Dumping a Specific View

All views implement the `dump_to_file` method from the `View` trait:

```rust
use turbo_vision::prelude::*;
use turbo_vision::views::{dialog::Dialog, View};

let mut terminal = Terminal::init()?;
let mut dialog = Dialog::new(Rect::new(10, 5, 50, 15), "Test Dialog");
// ... configure and draw the dialog ...
dialog.draw(&mut terminal);
dialog.dump_to_file(&terminal, "debug_dialog.ans")?;
```

### Dumping a Region

To dump a specific rectangular region:

```rust
terminal.dump_region(x, y, width, height, "region.ans")?;
```

## Viewing Dumped Files

### On Unix/Linux/macOS

```bash
# View directly in terminal
cat debug_screen.ans

# Scrollable viewing
less -R debug_screen.ans

# View in a pager with color support
bat debug_screen.ans
```

### On Windows

```powershell
# PowerShell with ANSI support (Windows 10+)
Get-Content debug_screen.ans

# Or use Windows Terminal
type debug_screen.ans
```

### In Text Editors

Most modern text editors support ANSI escape codes:
- VS Code (with appropriate extensions)
- Sublime Text
- vim/neovim
- emacs

## Example

See `examples/dump_demo.rs` for a complete working example:

```bash
cargo run --example dump_demo
```

This will create three files:
- `screen_dump.ans` - The entire terminal screen
- `dialog_dump.ans` - Just the dialog region
- `buttons_dump.ans` - Just the buttons

## File Format

The generated files use standard ANSI escape sequences:
- `\x1b[XXm` for foreground colors (30-37, 90-97)
- `\x1b[XXm` for background colors (40-47, 100-107)
- `\x1b[0m` to reset colors at end of each line

The color codes correspond to the standard 16-color terminal palette used by Turbo Vision.

## API Reference

### Global Shortcuts

- **F12** - Automatically dumps the screen to `screen-dump.txt`
  - Works at any time during application execution
  - Handled at the application level (before any other event handlers)
  - Shows a brief flash effect for visual feedback
  - Silently fails if file cannot be written (doesn't crash app)

- **ESC + Shift + S** - Alternative screen dump shortcut
  - Same functionality as F12
  - Useful on systems where F12 is captured by the OS or terminal
  - Also shows flash effect
  - Processed through ESC sequence tracker

### Terminal Methods

- `dump_screen(&self, path: &str) -> io::Result<()>`
  - Dumps the entire terminal buffer to a file

- `dump_region(&self, x: u16, y: u16, width: u16, height: u16, path: &str) -> io::Result<()>`
  - Dumps a rectangular region to a file

- `buffer(&self) -> &[Vec<Cell>]`
  - Returns a reference to the internal buffer for custom dumping

- `flash(&mut self) -> io::Result<()>`
  - Flashes the screen by inverting all colors for 50ms
  - Used for visual feedback when capturing screen dumps
  - Can also be called manually for other notifications

### View Trait Method

- `dump_to_file(&self, terminal: &Terminal, path: &str) -> io::Result<()>`
  - Dumps the view's region (including shadow if present) to a file

### Core Functions

The `core::ansi_dump` module provides lower-level functions:

- `dump_buffer_to_file(buffer, width, height, path)`
- `dump_buffer(writer, buffer, width, height)`
- `dump_buffer_region(writer, buffer, x, y, width, height)`

These are useful if you need custom dumping behavior or want to write to something other than a file.
