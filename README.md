# Turbo Vision - Rust TUI Library

![Turbo Vision Logo](https://raw.githubusercontent.com/aovestdipaperino/turbo-vision-4-rust/main/logo.png)

## WARNING: WORK IN PROGRESS

A Rust implementation of the classic Borland Turbo Vision text user interface framework.

## Features

- **Complete UI Component Set**: Windows, dialogs, buttons, input fields, menus, status bars, scrollbars
- **Z-Order Management**: Click any non-modal window to bring it to the front
- **Modal Dialog Support**: Modal dialogs block interaction with background windows
- **Borland-Accurate Styling**: Menu borders and shadows match original Borland Turbo Vision
- **Scrollable Views**: Built-in scrollbar support with keyboard navigation
- **Text Viewer**: Ready-to-use scrollable text viewer with line numbers
- **Event-Driven Architecture**: Keyboard and command-based event routing
- **Mouse Support**: Full mouse support for buttons, menus, status bar, and dialog close buttons
- **Window Dragging**: Drag windows in all directions with proper redrawing
- **Flexible Layout System**: Geometry primitives with absolute and relative positioning
- **Color Support**: 16-color palette with attribute system
- **Cross-Platform**: Built on crossterm for wide terminal compatibility
- **Modal Dialogs**: Built-in support for modal dialog execution
- **Focus Management**: Tab navigation and keyboard shortcuts
- **ANSI Dump**: Debug UI by dumping screen/views to ANSI text files (F12 for full screen, F11 for active view, with flash effect)

## Quick Start

```rust
use turbo_vision::prelude::*;
use turbo_vision::app::Application;
use turbo_vision::views::{
    dialog::Dialog,
    button::Button,
    static_text::StaticText,
};

fn main() -> std::io::Result<()> {
    let mut app = Application::new()?;

    // Create a simple dialog
    let mut dialog = Dialog::new(
        Rect::new(20, 8, 60, 16),
        "Hello World"
    );

    let text = StaticText::new(
        Rect::new(22, 10, 58, 12),
        "Welcome to Turbo Vision!"
    );
    dialog.add(Box::new(text));

    let button = Button::new(
        Rect::new(35, 13, 45, 15),
        "  OK  ",
        CM_OK,
        true
    );
    dialog.add(Box::new(button));

    dialog.execute(&mut app.terminal);
    Ok(())
}
```

**Tip**: Press F12 at any time to capture full screen to `screen-dump.txt`, or F11 to capture active window/dialog to `active-view-dump.txt` - both with a visual flash effect for debugging!

## Module Overview

- **core**: Fundamental types (geometry, events, drawing, colors)
- **terminal**: Terminal I/O abstraction layer
- **views**: UI components (dialogs, buttons, menus, etc.)
- **app**: Application framework and event loop

## Documentation

See the [examples](examples) for a complete simple examples.

## Status

Currently implements:
- ✅ Core drawing and event system
- ✅ Dialog boxes with frames and close buttons
- ✅ Buttons with keyboard shortcuts
- ✅ Static text labels (with centered text support)
- ✅ Input fields
- ✅ Menu bar with dropdowns
- ✅ Status line
- ✅ Desktop manager
- ✅ Scrollbars (vertical and horizontal)
- ✅ Scroller base class for scrollable views
- ✅ Indicator (position display)
- ✅ Text viewer with scrolling
- ✅ CheckBoxes
- ✅ RadioButtons
- ✅ ListBoxes
- ✅ Memo (multi-line text editor)
- ✅ Mouse support (buttons, menus, status bar, close buttons, hover effects, listbox clicks)
- ✅ File Dialog (fully functional with mouse/keyboard support and directory navigation)
- ✅ ANSI Dump for debugging (dump screen/views to text files with colors)
- ❌ Full text editor with search/replace (basic editing available)

## License

MIT License - see [LICENSE](LICENSE) file for details.
