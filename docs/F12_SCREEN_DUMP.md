# Screen Dump Shortcuts Feature

## Overview

Global keyboard shortcuts have been added to capture screen dumps at any time during application execution. This makes debugging and visual inspection of your TUI application incredibly easy.

## Keyboard Shortcuts

Two shortcuts are available:

- **F12** - Dump entire screen to `screen-dump.txt`
- **F11** - Dump active window/dialog to `active-view-dump.txt`

Both shortcuts provide:
- **Visual Feedback**: Brief screen flash (color inversion) to confirm capture
- **Silent Operation**: Errors don't crash the app, just fail silently
- **Instant Capture**: Screen is captured immediately in its current state

## Usage

Simply press the shortcuts while your application is running:
- **F12** - Captures entire screen to `screen-dump.txt`
- **F11** - Captures active window/dialog to `active-view-dump.txt`

```rust
use turbo_vision::app::Application;

fn main() -> std::io::Result<()> {
    let mut app = Application::new()?;

    // ... set up your UI with menus, dialogs, etc. ...

    app.run();  // Press F12 anytime during execution!

    Ok(())
}
```

After pressing a shortcut, you can view the captured output:

```bash
cat screen-dump.txt           # View full screen dump
cat active-view-dump.txt      # View active window/dialog dump
less -R screen-dump.txt       # For scrollable viewing
```

## Implementation Details

### Key Codes

- **KB_F12**: `0x8600` - Full screen dump
- **KB_F11**: `0x8500` - Active view dump

### Event Handling

The screen dump handlers are implemented at the **lowest level** in `Terminal::poll_event()` and `Terminal::read_event()`:

1. **Lowest Level**: Handled in Terminal before any application code sees the event
2. **Universal**: Works in all event loops (Application, Dialog, custom loops)
3. **Visual Feedback**: Flashes the screen before capturing
4. **Silent Failure**: If the file cannot be written, the error is ignored (doesn't crash the app)
5. **Event Suppressed**: The shortcut keys never propagate to application code

```rust
// In Terminal::poll_event() and Terminal::read_event()
if key_code == KB_F12 {
    let _ = self.flash();
    let _ = self.dump_screen("screen-dump.txt");
    return Ok(None);  // or continue in read_event
}

if key_code == KB_F11 {
    let _ = self.flash();
    if let Some(bounds) = self.active_view_bounds {
        let _ = self.dump_region(..., "active-view-dump.txt");
    }
    return Ok(None);  // or continue in read_event
}
```

This means:
- **No hooks needed**: Works in `Application::run()`, `Dialog::execute()`, or any custom loop
- **Always available**: Can't be accidentally blocked by event handlers
- **Zero configuration**: Automatically works in all examples and user code

### Visual Flash Effect

The flash effect is implemented in `Terminal::flash()`:

1. Saves the current buffer
2. Inverts all foreground and background colors
3. Flushes the inverted screen
4. Waits 50ms
5. Restores the original buffer
6. Flushes the restored screen

This provides clear visual feedback that the screen was captured.

### Why These Shortcuts?

**F12 (Full Screen):**
- **Standard Shortcut**: F12 is commonly used for developer tools in web browsers
- **Low Conflict**: Unlikely to conflict with application-specific shortcuts
- **Easy to Remember**: Single key, no modifiers needed
- **Global Access**: Works everywhere in the application

**F11 (Active View):**
- **Complementary**: F11/F12 are adjacent function keys, easy to remember together
- **Context Aware**: Captures only the relevant window/dialog
- **Low Conflict**: F11 is rarely used in terminal applications
- **Automatic**: Application and Dialog automatically set active view bounds

## Benefits

1. **Zero Code Changes**: No need to add debugging code to your application
2. **Runtime Debugging**: Capture screen state at any moment during execution
3. **Non-Intrusive**: Doesn't interrupt application flow
4. **Always Available**: Works in dialogs, menus, or any other view
5. **Color Preserved**: Full ANSI color codes captured for accurate representation

## Use Cases

### Bug Reports
When users report visual bugs, ask them to press F12 and send you `screen-dump.txt`. You'll see exactly what they see.

### Visual Testing
Capture expected output during testing for visual regression comparisons.

### Documentation
Create screenshots of your TUI application by capturing and rendering the ANSI files.

### Development
Quickly inspect layout issues without adding temporary debugging code.

## Example

See `examples/f12_test.rs` for a complete working example:

```bash
cargo run --example f12_test
# Press F12 to dump entire screen -> screen-dump.txt
# Press F11 to dump dialog only -> active-view-dump.txt
# You'll see a brief flash effect for each
# Check the output files after closing
```

## Technical Notes

- **File Location**: Both files are created in the current working directory
- **Overwrite Behavior**: Each keypress overwrites the previous dump
- **Active View Tracking**: Application and Dialog automatically set bounds via `terminal.set_active_view_bounds()`
- **Thread Safety**: Safe to use from the main event loop
- **Performance**: Minimal overhead, only executes when shortcuts are pressed

## Related Features

- `Terminal::dump_screen(path)` - Programmatically dump the screen
- `Terminal::dump_region(x, y, w, h, path)` - Dump a specific region
- `View::dump_to_file(terminal, path)` - Dump a specific view
- See `docs/ansi-dump.md` for complete ANSI dump documentation

## Files Modified

- `src/core/event.rs` - Added KB_F11 and KB_F12 constants
- `src/terminal/mod.rs` - Added `flash()` method, `active_view_bounds` tracking, and F11/F12 handlers in `poll_event()` and `read_event()`
- `src/app/application.rs` - Added `update_active_view_bounds()` to track topmost window
- `src/views/dialog.rs` - Set dialog bounds as active view in `execute()`
- `src/views/desktop.rs` - Added `child_count()` and `child_at()` methods
- `examples/f12_test.rs` - Example demonstrating both shortcuts
- `docs/ansi-dump.md` - Updated with both shortcuts and flash effect documentation
- `docs/F12_SCREEN_DUMP.md` - This file
- `README.md` - Mentioned shortcuts in features and quick start
