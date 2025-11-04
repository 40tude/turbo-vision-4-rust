# Chapter 2 — Responding to Commands (Rust Edition)

**Version:** 0.2.11
**Updated:** 2025-11-04

In Chapter 1, you built a minimal Turbo Vision application. In this chapter, you'll learn how to make it **respond to user commands**, such as keystrokes or menu selections. You'll extend the tutorial application by adding an **About box** and a **display mode toggle**, and learn how to enable or disable menu items dynamically.

---

## Understanding Events and Commands

Turbo Vision applications are **event-driven**. The framework generates events to signal that something has happened — for example, the user pressed a key, clicked the mouse, or chose a menu command.

Your application's job is to **respond to those events**.

Events are represented by the `Event` struct and contain the following important fields:

```rust
pub struct Event {
    pub what: EventType,         // Kind of event (keyboard, mouse, command, etc.)
    pub key_code: u16,           // Key code for keyboard events
    pub command: u16,            // Command identifier for command events
    pub mouse: MouseEvent,       // Mouse coordinates and button state
    pub key_modifiers: KeyModifiers,  // Shift, Ctrl, Alt state
}

pub enum EventType {
    Nothing,        // No event or event was consumed
    Keyboard,       // Keyboard input
    MouseDown,      // Mouse button pressed
    MouseUp,        // Mouse button released
    MouseMove,      // Mouse movement
    MouseDrag,      // Mouse drag operation
    Command,        // High-level command
    Broadcast,      // Broadcast message to all views
}
```

**Key Update (v0.2.11):** Mouse events are now separate types (`MouseDown`, `MouseUp`, `MouseMove`, `MouseDrag`) instead of a single `Mouse` type. This provides finer control over mouse interactions.

Keyboard and mouse input are transformed by Turbo Vision into high-level commands such as `CM_QUIT`, `CM_NEW`, or `CM_ABOUT`.

---

## Step 3 — Responding to Commands

You'll now add your first custom command handler.

### Listing 2.1 — Handling Commands in a Custom Event Loop

```rust
// tutorial_02.rs
use turbo_vision::app::Application;
use turbo_vision::core::geometry::Rect;
use turbo_vision::core::command::{CM_QUIT, CM_ABOUT};
use turbo_vision::core::event::{EventType, KB_F10, KB_ALT_X};
use turbo_vision::views::msgbox::message_box_ok;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let mut app = Application::new()?;

    // Custom event loop with three-phase processing
    app.running = true;
    while app.running {
        // Draw the interface
        app.desktop.draw(&mut app.terminal);
        if let Some(ref mut menu_bar) = app.menu_bar {
            menu_bar.draw(&mut app.terminal);
        }
        if let Some(ref mut status_line) = app.status_line {
            status_line.draw(&mut app.terminal);
        }
        let _ = app.terminal.flush();

        // Poll for events
        if let Ok(Some(mut event)) = app.terminal.poll_event(Duration::from_millis(50)) {
            // Phase 1: PreProcess - Status line handles events first
            if let Some(ref mut status_line) = app.status_line {
                status_line.handle_event(&mut event);
            }

            // Special handling: Menu bar (F10 and Alt keys)
            if let Some(ref mut menu_bar) = app.menu_bar {
                menu_bar.handle_event(&mut event);

                // Check for cascading submenus
                if event.what == EventType::Keyboard || event.what == EventType::MouseUp {
                    if let Some(command) = menu_bar.check_cascading_submenu(&mut app.terminal) {
                        if command != 0 {
                            event = turbo_vision::core::event::Event::command(command);
                        }
                    }
                }
            }

            // Phase 2: Focused - Desktop (and its children) handle events
            app.desktop.handle_event(&mut event);

            // Application-level command handling
            if event.what == EventType::Command {
                match event.command {
                    CM_QUIT => {
                        app.running = false;
                    }
                    CM_ABOUT => {
                        show_about_box(&mut app);
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn show_about_box(app: &mut Application) {
    message_box_ok(
        app,
        "About",
        "  Turbo Vision Tutorial App 2.0\n\n  Rust Edition"
    );
}
```

This example adds an `About` command handler. When the user chooses **About**, the application displays a modal dialog box using the standard library function `message_box_ok()`.

### Three-Phase Event Processing

**New in v0.1.9** - The event loop now implements proper three-phase processing matching Borland's TGroup::handleEvent():

1. **PreProcess Phase** - Views with `OF_PRE_PROCESS` flag (StatusLine)
2. **Focused Phase** - Currently focused view (Desktop and its children)
3. **PostProcess Phase** - Views with `OF_POST_PROCESS` flag (handled within Group)

**StatusLine PreProcess (v0.2.11):** StatusLine uses the `OF_PRE_PROCESS` flag to intercept events first, matching Borland's tstatusl.cc:33. This allows status line shortcuts to work before other views process the event.

### Key Differences from Pascal

In Pascal, you override the `HandleEvent` virtual method:

```pascal
procedure TTutorApp.HandleEvent(var Event: TEvent);
begin
  TApplication.HandleEvent(Event);  // Call parent first
  if Event.What = evCommand then begin
    // Handle commands
  end;
end;
```

In Rust, you implement command handling in your custom event loop:

```rust
// Application-level command handling
if event.what == EventType::Command {
    match event.command {
        CM_ABOUT => {
            show_about_box(&mut app);
        }
        _ => {}
    }
}
```

The Rust approach provides:
- **Explicit control flow** - You see exactly when and how events are processed
- **No inheritance required** - Use composition instead of class hierarchies
- **Type safety** - Pattern matching on commands catches unhandled cases
- **Flexible ordering** - Control whether views or application handles events first

---

## Step 4 — Customizing Menus and Status Lines

By default, the minimal Turbo Vision application displays an empty desktop. You'll now customize it to include a menu bar and status line with meaningful options.

### Listing 2.2 — Adding a Menu and Status Line

```rust
// tutorial_03.rs
use turbo_vision::app::Application;
use turbo_vision::core::geometry::Rect;
use turbo_vision::core::command::{CM_QUIT, CM_ABOUT};
use turbo_vision::core::event::{EventType, KB_F1, KB_F10, KB_ALT_X};
use turbo_vision::core::menu_data::{Menu, MenuItem};
use turbo_vision::views::menu_bar::{MenuBar, SubMenu};
use turbo_vision::views::status_line::{StatusLine, StatusItem};
use turbo_vision::views::msgbox::message_box_ok;
use std::time::Duration;

// Define custom commands
const CM_OPTIONS_VIDEO: u16 = 1502;

fn main() -> std::io::Result<()> {
    let mut app = Application::new()?;
    let (width, height) = app.terminal.size();

    // Initialize menu bar
    let menu_bar = create_menu_bar(width);
    app.set_menu_bar(menu_bar);

    // Initialize status line
    let status_line = create_status_line(height, width);
    app.set_status_line(status_line);

    // Custom event loop with three-phase processing
    app.running = true;
    while app.running {
        // Draw the interface
        app.desktop.draw(&mut app.terminal);
        if let Some(ref mut menu_bar) = app.menu_bar {
            menu_bar.draw(&mut app.terminal);
        }
        if let Some(ref mut status_line) = app.status_line {
            status_line.draw(&mut app.terminal);
        }
        let _ = app.terminal.flush();

        // Handle events
        if let Ok(Some(mut event)) = app.terminal.poll_event(Duration::from_millis(50)) {
            // Phase 1: PreProcess - Status line first
            if let Some(ref mut status_line) = app.status_line {
                status_line.handle_event(&mut event);
            }

            // Special: Menu bar handles F10 and Alt keys
            if let Some(ref mut menu_bar) = app.menu_bar {
                menu_bar.handle_event(&mut event);

                // Check for cascading submenus
                if event.what == EventType::Keyboard || event.what == EventType::MouseUp {
                    if let Some(command) = menu_bar.check_cascading_submenu(&mut app.terminal) {
                        if command != 0 {
                            event = turbo_vision::core::event::Event::command(command);
                        }
                    }
                }
            }

            // Phase 2: Focused - Desktop handles events
            app.desktop.handle_event(&mut event);

            // Application-level command handling
            if event.what == EventType::Command {
                match event.command {
                    CM_QUIT => {
                        app.running = false;
                    }
                    CM_ABOUT => {
                        show_about_box(&mut app);
                    }
                    CM_OPTIONS_VIDEO => {
                        // Will implement in next section
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn create_menu_bar(width: u16) -> MenuBar {
    let mut menu_bar = MenuBar::new(Rect::new(0, 0, width as i16, 1));

    // File menu
    let file_menu_items = vec![
        MenuItem::with_shortcut("E~x~it", CM_QUIT, KB_ALT_X, "Alt+X", 0),
    ];
    let file_menu = SubMenu::new("~F~ile", Menu::from_items(file_menu_items));
    menu_bar.add_submenu(file_menu);

    // Options menu
    let options_menu_items = vec![
        MenuItem::with_shortcut("~V~ideo Mode", CM_OPTIONS_VIDEO, 0, "", 0),
    ];
    let options_menu = SubMenu::new("~O~ptions", Menu::from_items(options_menu_items));
    menu_bar.add_submenu(options_menu);

    // Help menu
    let help_menu_items = vec![
        MenuItem::with_shortcut("~A~bout", CM_ABOUT, KB_F1, "F1", 0),
    ];
    let help_menu = SubMenu::new("~H~elp", Menu::from_items(help_menu_items));
    menu_bar.add_submenu(help_menu);

    menu_bar
}

fn create_status_line(height: u16, width: u16) -> StatusLine {
    StatusLine::new(
        Rect::new(0, height as i16 - 1, width as i16, height as i16),
        vec![
            StatusItem::new("~Alt+X~ Exit", KB_ALT_X, CM_QUIT),
        ],
    )
}

fn show_about_box(app: &mut Application) {
    message_box_ok(
        app,
        "About",
        "  Turbo Vision Tutorial App 2.0\n\n  Rust Edition"
    );
}
```

### Running the Program

When you run this version of the program:

- A **menu bar** appears with three main menus: *File*, *Options*, and *Help*.
- Choosing **Help → About** (or pressing F1) triggers the About dialog.
- The **status line** shows `Alt+X Exit` at the bottom.
- The menus respond to hot keys (Alt+F for File, Alt+O for Options, Alt+H for Help).

### Understanding the Menu Creation

**Current API (v0.2.11):**

Menus are created using `Menu::from_items()` and `SubMenu::new()`:

```rust
// Create menu items
let file_menu_items = vec![
    MenuItem::with_shortcut("E~x~it", CM_QUIT, KB_ALT_X, "Alt+X", 0),
];

// Create submenu from items
let file_menu = SubMenu::new("~F~ile", Menu::from_items(file_menu_items));

// Add to menu bar
menu_bar.add_submenu(file_menu);
```

**Comparison with Pascal:**

| Pascal | Rust (v0.2.11) |
|--------|----------------|
| `NewSubMenu('~F~ile', hcNoContext, NewMenu(...))` | `SubMenu::new("~F~ile", Menu::from_items(vec![...]))` |
| `NewItem('E~x~it', 'Alt+X', kbAltX, cmQuit, hcNoContext, nil)` | `MenuItem::with_shortcut("E~x~it", CM_QUIT, KB_ALT_X, "Alt+X", 0)` |
| Nested constructors | Builder pattern with vectors |

The Rust approach is more readable and follows Rust idioms for building complex structures.

---

## Step 5 — Adding Display Mode Toggle

Let's make the "Video Mode" menu item toggle between different display modes. While the original Turbo Vision used `SetSnowFilter` for CGA snow protection, modern terminal applications can toggle other display features.

We'll implement a color mode toggle that switches between full color and monochrome display.

### Listing 2.3 — Adding a Display Toggle Command

First, add state to track the current mode:

```rust
// tutorial_04.rs
use turbo_vision::app::Application;
use turbo_vision::core::geometry::Rect;
use turbo_vision::core::command::{CM_QUIT, CM_ABOUT};
use turbo_vision::core::event::{EventType, KB_F1, KB_ALT_X};
use turbo_vision::core::menu_data::{Menu, MenuItem};
use turbo_vision::views::menu_bar::{MenuBar, SubMenu};
use turbo_vision::views::status_line::{StatusLine, StatusItem};
use turbo_vision::views::msgbox::message_box_ok;
use std::time::Duration;

// Define custom commands
const CM_OPTIONS_VIDEO: u16 = 1502;

// Application state
struct AppState {
    use_colors: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            use_colors: true,
        }
    }

    fn toggle_colors(&mut self) {
        self.use_colors = !self.use_colors;
    }
}

fn main() -> std::io::Result<()> {
    let mut app = Application::new()?;
    let (width, height) = app.terminal.size();
    let mut state = AppState::new();

    // Initialize menu bar
    let menu_bar = create_menu_bar(width);
    app.set_menu_bar(menu_bar);

    // Initialize status line
    let status_line = create_status_line(height, width);
    app.set_status_line(status_line);

    // Custom event loop with three-phase processing
    app.running = true;
    while app.running {
        // Draw the interface
        app.desktop.draw(&mut app.terminal);
        if let Some(ref mut menu_bar) = app.menu_bar {
            menu_bar.draw(&mut app.terminal);
        }
        if let Some(ref mut status_line) = app.status_line {
            status_line.draw(&mut app.terminal);
        }
        let _ = app.terminal.flush();

        // Handle events
        if let Ok(Some(mut event)) = app.terminal.poll_event(Duration::from_millis(50)) {
            // Phase 1: PreProcess - Status line first
            if let Some(ref mut status_line) = app.status_line {
                status_line.handle_event(&mut event);
            }

            // Special: Menu bar handles F10 and Alt keys
            if let Some(ref mut menu_bar) = app.menu_bar {
                menu_bar.handle_event(&mut event);

                // Check for cascading submenus
                if event.what == EventType::Keyboard || event.what == EventType::MouseUp {
                    if let Some(command) = menu_bar.check_cascading_submenu(&mut app.terminal) {
                        if command != 0 {
                            event = turbo_vision::core::event::Event::command(command);
                        }
                    }
                }
            }

            // Phase 2: Focused - Desktop handles events
            app.desktop.handle_event(&mut event);

            // Application-level command handling
            if event.what == EventType::Command {
                match event.command {
                    CM_QUIT => {
                        app.running = false;
                    }
                    CM_ABOUT => {
                        show_about_box(&mut app);
                    }
                    CM_OPTIONS_VIDEO => {
                        // Toggle color mode
                        state.toggle_colors();
                        show_mode_changed_dialog(&mut app, &state);
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn show_mode_changed_dialog(app: &mut Application, state: &AppState) {
    let mode_text = if state.use_colors {
        "Color mode enabled"
    } else {
        "Monochrome mode enabled"
    };

    message_box_ok(app, "Display Mode", mode_text);
}

fn create_menu_bar(width: u16) -> MenuBar {
    let mut menu_bar = MenuBar::new(Rect::new(0, 0, width as i16, 1));

    // File menu
    let file_menu_items = vec![
        MenuItem::with_shortcut("E~x~it", CM_QUIT, KB_ALT_X, "Alt+X", 0),
    ];
    let file_menu = SubMenu::new("~F~ile", Menu::from_items(file_menu_items));
    menu_bar.add_submenu(file_menu);

    // Options menu
    let options_menu_items = vec![
        MenuItem::with_shortcut("~V~ideo Mode", CM_OPTIONS_VIDEO, 0, "", 0),
    ];
    let options_menu = SubMenu::new("~O~ptions", Menu::from_items(options_menu_items));
    menu_bar.add_submenu(options_menu);

    // Help menu
    let help_menu_items = vec![
        MenuItem::with_shortcut("~A~bout", CM_ABOUT, KB_F1, "F1", 0),
    ];
    let help_menu = SubMenu::new("~H~elp", Menu::from_items(help_menu_items));
    menu_bar.add_submenu(help_menu);

    menu_bar
}

fn create_status_line(height: u16, width: u16) -> StatusLine {
    StatusLine::new(
        Rect::new(0, height as i16 - 1, width as i16, height as i16),
        vec![
            StatusItem::new("~Alt+X~ Exit", KB_ALT_X, CM_QUIT),
        ],
    )
}

fn show_about_box(app: &mut Application) {
    message_box_ok(
        app,
        "About",
        "  Turbo Vision Tutorial App 2.0\n\n  Rust Edition"
    );
}
```

This example toggles the display mode whenever the user selects **Options → Video Mode**.

### Pascal vs. Rust: State Management

In Pascal, application state is stored in fields of the application object:

```pascal
TTutorApp = object(TApplication)
  UseColors: Boolean;
end;
```

In Rust, we use a separate state struct:

```rust
struct AppState {
    use_colors: bool,
}
```

This separation provides:
- **Clear ownership** - State is independent from the UI framework
- **Easy testing** - State logic can be tested without a terminal
- **Flexibility** - Multiple state structs for different concerns
- **Type safety** - Compiler ensures state is properly initialized

---

## Enabling and Disabling Commands

Turbo Vision allows you to dynamically enable or disable menu items. This is useful when certain actions are unavailable (for example, *Paste* when the clipboard is empty, or *Save* when there are no changes).

### Example — Dynamic Menu Items

```rust
use turbo_vision::core::menu_data::{Menu, MenuItem};
use turbo_vision::views::menu_bar::{MenuBar, SubMenu};
use turbo_vision::core::command::{CM_PASTE, CM_SAVE};

fn create_dynamic_menu_bar(width: u16, has_clipboard: bool, has_changes: bool) -> MenuBar {
    let mut menu_bar = MenuBar::new(Rect::new(0, 0, width as i16, 1));

    let mut edit_menu_items = Vec::new();

    // Paste is only enabled when clipboard has content
    if has_clipboard {
        edit_menu_items.push(MenuItem::with_shortcut("~P~aste", CM_PASTE, 0, "", 0));
    } else {
        // Create disabled menu item (would need to be implemented)
        edit_menu_items.push(MenuItem::with_shortcut("~P~aste", CM_PASTE, 0, "", 0));
    }

    // Save is only enabled when there are unsaved changes
    if has_changes {
        edit_menu_items.push(MenuItem::with_shortcut("~S~ave", CM_SAVE, 0, "", 0));
    } else {
        edit_menu_items.push(MenuItem::with_shortcut("~S~ave", CM_SAVE, 0, "", 0));
    }

    let edit_menu = SubMenu::new("~E~dit", Menu::from_items(edit_menu_items));
    menu_bar.add_submenu(edit_menu);

    menu_bar
}
```

### Rebuilding Menus on State Change

When application state changes, you can rebuild the menu bar:

```rust
// After clipboard content changes
let menu_bar = create_dynamic_menu_bar(width, has_clipboard, app_state.modified);
app.set_menu_bar(menu_bar);
```

### More Elegant Approach: Command Set System

**New in v0.1.8** - Turbo Vision includes a Command Set system for automatic button enable/disable:

```rust
use turbo_vision::core::command_set;

// Disable commands initially
command_set::disable_command(CM_PASTE);  // No clipboard content
command_set::disable_command(CM_UNDO);   // Nothing to undo

// ... in event loop, app.idle() broadcasts changes ...

// User copies text
clipboard::set_text("Hello");
command_set::enable_command(CM_PASTE);  // Paste button automatically enables!

// User performs action
perform_action();
command_set::enable_command(CM_UNDO);   // Undo button automatically enables!
```

The command set system provides automatic button enable/disable based on application state. See `docs/TURBOVISION-DESIGN.md#command-set-system` for details.

---

## Event Flow and Processing Order

Understanding how events flow through your application is crucial:

```rust
// 1. Poll for events from terminal
if let Ok(Some(mut event)) = app.terminal.poll_event(Duration::from_millis(50)) {

    // 2. Phase 1: PreProcess - Status line first (OF_PRE_PROCESS)
    if let Some(ref mut status_line) = app.status_line {
        status_line.handle_event(&mut event);
    }

    // 3. Special: Menu bar handles F10 and Alt keys
    if let Some(ref mut menu_bar) = app.menu_bar {
        menu_bar.handle_event(&mut event);

        // Check for cascading submenus
        if event.what == EventType::Keyboard || event.what == EventType::MouseUp {
            if let Some(command) = menu_bar.check_cascading_submenu(&mut app.terminal) {
                if command != 0 {
                    event = turbo_vision::core::event::Event::command(command);
                }
            }
        }
    }

    // 4. Phase 2: Focused - Desktop and windows handle event
    app.desktop.handle_event(&mut event);

    // 5. Phase 3: PostProcess happens inside Group
    // (Buttons with OF_POST_PROCESS intercept their hotkeys)

    // 6. Application handles any remaining command events
    if event.what == EventType::Command {
        match event.command {
            // Handle application-level commands
        }
    }
}
```

Views can **consume** events by calling `event.clear()`, which sets `event.what = EventType::Nothing`. This prevents the event from being processed by subsequent handlers.

**Important:** The three-phase event processing (PreProcess → Focused → PostProcess) matches Borland's TGroup::handleEvent() exactly. This allows views like StatusLine to intercept events first, and buttons to intercept hotkeys even when not focused.

---

## Standard Library Dialog Functions

**New in v0.2.11** - Turbo Vision includes standard library dialog functions for common UI patterns:

```rust
use turbo_vision::views::msgbox::{message_box_ok, message_box_error, confirmation_box};

// Simple message
message_box_ok(&mut app, "About", "Turbo Vision 2.0");

// Error message
message_box_error(&mut app, "Error", "File not found");

// Confirmation (returns CM_YES, CM_NO, or CM_CANCEL)
let result = confirmation_box(&mut app, "Save changes?");
if result == CM_YES {
    save_file();
}
```

Additional dialogs available:
- `search_box()` - Search text input
- `search_replace_box()` - Find and replace
- `goto_line_box()` - Go to line number

See `src/views/msgbox.rs` for all available dialog functions.

---

## Summary

In this chapter, you learned:

- How Turbo Vision events and commands work in Rust
- How to handle commands in a custom event loop with three-phase processing
- How to create menus and status lines using `Menu::from_items()` and vectors
- How to manage application state separately from the UI framework
- How to enable or disable menu items dynamically based on application state
- How events flow through the view hierarchy with PreProcess → Focused → PostProcess
- How to use standard library dialog functions

### Key Differences from Pascal

| Pascal | Rust (v0.2.11) |
|--------|----------------|
| Override `HandleEvent` method | Custom event loop with pattern matching |
| `ClearEvent(Event)` | `event.clear()` |
| `MessageBox()` function | `message_box_ok()` from standard library |
| Object fields for state | Separate `AppState` struct |
| `NewMenu` / `NewSubMenu` | `Menu::from_items()` / `SubMenu::new()` |
| Virtual method dispatch | Explicit control flow |
| `evMouse` | `MouseDown`, `MouseUp`, `MouseMove`, `MouseDrag` |
| Two-phase event handling | Three-phase: PreProcess → Focused → PostProcess |

### Best Practices

1. **Separate state from UI** - Use dedicated state structs
2. **Handle commands in one place** - Centralize command processing in the main event loop
3. **Always clear handled events** - Call `event.clear()` after processing
4. **Order matters** - StatusLine (PreProcess) → MenuBar → Desktop (Focused) → Application
5. **Use pattern matching** - Rust's `match` catches unhandled commands at compile time
6. **Use standard dialogs** - Leverage `message_box_ok()`, `confirmation_box()`, etc.
7. **Check cascading submenus** - Always call `check_cascading_submenu()` after menu_bar.handle_event()

---

## See Also

- **Chapter 1** - Stepping into Turbo Vision (basics, minimal application)
- **docs/TURBOVISION-DESIGN.md** - Complete architecture documentation
- **docs/TURBOVISION-DESIGN.md#event-system-architecture** - Three-phase event processing details
- **docs/TURBOVISION-DESIGN.md#command-set-system** - Automatic command enable/disable
- **examples/README.md** - Guide to all 16 examples

---

In the next chapter, you'll learn how to **add windows** to your Turbo Vision application, create text editors, and work with multiple views simultaneously.
