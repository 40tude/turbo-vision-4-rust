# Chapter 9 — Event-Driven Programming (Rust Edition)

**Version:** 0.2.11
**Updated:** 2025-11-04

This chapter explores event-driven programming in Turbo Vision, showing how the framework handles user input, commands, and inter-view communication. Understanding events is essential for creating responsive, interactive terminal applications.

**Prerequisites:** Chapters 7-8 (Architecture and Views)

---

## Table of Contents

1. [What is Event-Driven Programming?](#what-is-event-driven-programming)
2. [The Event System](#the-event-system)
3. [Event Types](#event-types)
4. [Event Routing](#event-routing)
5. [Three-Phase Event Processing](#three-phase-event-processing)
6. [Commands](#commands)
7. [Handling Events](#handling-events)
8. [Inter-View Communication](#inter-view-communication)
9. [Advanced Topics](#advanced-topics)

---

## What is Event-Driven Programming?

### Traditional vs. Event-Driven

**Traditional Program:**
```rust
// Traditional loop: constantly polling
loop {
    let key = read_keyboard();
    match key {
        'i' => invert_array(),
        'e' => edit_params(),
        'g' => graphic_display(),
        'q' => break,
        _ => {}
    }
}
```

**Event-Driven Program:**
```rust
// Turbo Vision: framework handles input, dispatches events
impl View for MyView {
    fn handle_event(&mut self, event: &mut Event) {
        match event.what {
            EventType::Command => {
                match event.command {
                    CM_INVERT => { invert_array(); event.clear(); }
                    CM_EDIT => { edit_params(); event.clear(); }
                    CM_GRAPH => { graphic_display(); event.clear(); }
                    CM_QUIT => { quit(); event.clear(); }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
```

### Key Differences

| Traditional | Event-Driven (Turbo Vision) |
|-------------|----------------------------|
| You read input | Framework reads input |
| You dispatch to handlers | Framework dispatches to views |
| Global input loop | Each view handles its events |
| Must filter irrelevant input | Only receive relevant events |

### Benefits

1. **Separation of Concerns** - Each view handles only its own events
2. **Automatic Routing** - Framework delivers events to the right view
3. **Built-in Behaviors** - Windows, buttons, menus work automatically
4. **Focus on Logic** - You define what to do, not how to get input

---

## The Event System

### The Event Structure

```rust
pub struct Event {
    pub what: EventType,        // Type of event
    pub key_code: u16,          // For keyboard events
    pub command: u16,           // For command events
    pub mouse: MouseEvent,      // For mouse events
}

pub struct MouseEvent {
    pub position: Point,        // Global screen coordinates
    pub buttons: u8,            // Which buttons pressed
    pub double_click: bool,     // Double-click flag
}
```

### The Main Event Loop

Inside `Application::run()`:

```rust
pub fn run(&mut self) {
    self.running = true;

    while self.running {
        // Get next event
        let mut event = self.get_event();

        // Route to views
        self.handle_event(&mut event);

        // Check for unhandled events
        if event.what != EventType::Nothing {
            self.event_error(&event);
        }

        // Draw changes
        self.draw();
        self.terminal.flush();
    }
}
```

**The Cycle:**
1. `get_event()` - Package input into event record
2. `handle_event()` - Route event to appropriate views
3. Views respond or transform events
4. Repeat until quit

---

## Event Types

### EventType Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Nothing,        // No event or consumed
    Keyboard,       // Key pressed
    MouseDown,      // Mouse button pressed
    MouseUp,        // Mouse button released
    MouseMove,      // Mouse moved
    MouseDrag,      // Dragging with button down
    Command,        // High-level command
    Broadcast,      // Message to all views
}
```

### Event Type Masks (from Pascal)

The original Turbo Vision used bit masks for event filtering:

```rust
// Event masks (for reference - Rust uses enum matching instead)
pub const EV_NOTHING: u16   = 0x0000;
pub const EV_MOUSE: u16     = 0x000F;  // All mouse events
pub const EV_KEYBOARD: u16  = 0x0010;
pub const EV_COMMAND: u16   = 0x0100;
pub const EV_BROADCAST: u16 = 0x0200;
```

**Rust Approach:**
```rust
// Pascal: if Event.What and evMouse <> 0 then ...
// Rust:
match event.what {
    EventType::MouseDown | EventType::MouseUp |
    EventType::MouseMove | EventType::MouseDrag => {
        // Handle any mouse event
    }
    _ => {}
}
```

### Mouse Events

#### MouseDown
Generated when user presses a mouse button:

```rust
if event.what == EventType::MouseDown {
    let pos = event.mouse.position;
    if self.bounds().contains(pos) {
        // Clicked inside this view
        if event.mouse.double_click {
            // Double-click!
        }
    }
}
```

#### MouseUp
Generated when user releases a mouse button:

```rust
if event.what == EventType::MouseUp {
    // Button release - complete click action
    self.complete_action();
    event.clear();
}
```

#### MouseMove
Generated when mouse moves without button pressed:

```rust
if event.what == EventType::MouseMove {
    // Update hover state, cursor shape, etc.
    self.update_hover(event.mouse.position);
}
```

#### MouseDrag
Generated when mouse moves with button held:

```rust
if event.what == EventType::MouseDrag {
    // Window being dragged
    let delta = Point {
        x: event.mouse.position.x - self.drag_start.x,
        y: event.mouse.position.y - self.drag_start.y,
    };
    self.move_by(delta);
}
```

### Keyboard Events

Generated when user presses a key:

```rust
impl View for InputLine {
    fn handle_event(&mut self, event: &mut Event) {
        if event.what == EventType::Keyboard {
            match event.key_code {
                KB_ENTER => {
                    self.finish_edit();
                    event.clear();
                }
                KB_ESC => {
                    self.cancel_edit();
                    event.clear();
                }
                KB_BACK => {
                    self.delete_char();
                    event.clear();
                }
                _ => {
                    if let Some(ch) = key_to_char(event.key_code) {
                        self.insert_char(ch);
                        event.clear();
                    }
                }
            }
        }
    }
}
```

### Command Events

High-level actions, typically generated by menus, buttons, or keyboard shortcuts:

```rust
impl View for Application {
    fn handle_event(&mut self, event: &mut Event) {
        if event.what == EventType::Command {
            match event.command {
                CM_QUIT => {
                    self.running = false;
                    event.clear();
                }
                CM_NEW => {
                    self.create_new_document();
                    event.clear();
                }
                CM_OPEN => {
                    self.open_document();
                    event.clear();
                }
                _ => {}
            }
        }
    }
}
```

### Broadcast Events

Messages sent to all views:

```rust
impl View for CounterView {
    fn handle_event(&mut self, event: &mut Event) {
        if event.what == EventType::Broadcast {
            match event.command {
                CM_RECORD_CHANGED => {
                    // Update display
                    self.refresh();
                    // Don't clear - let other views see it too
                }
                _ => {}
            }
        }
    }
}
```

### Nothing Events

An event marked as consumed:

```rust
impl Event {
    pub fn clear(&mut self) {
        self.what = EventType::Nothing;
    }
}

// Views should ignore Nothing events
if event.what == EventType::Nothing {
    return;
}
```

---

## Event Routing

Events are routed differently based on their type.

### Positional Routing (Mouse Events)

**Positional events** go to the view at the event's position:

```
Mouse click at (35, 10)
    ↓
Application finds Desktop at that position
    ↓
Desktop finds Window at that position
    ↓
Window finds Button at that position
    ↓
Button handles the click
```

```rust
impl Group {
    fn handle_mouse_event(&mut self, event: &mut Event) {
        let pos = event.mouse.position;

        // Search children in Z-order (front to back)
        for child in self.children.iter_mut().rev() {
            if child.bounds().contains(pos) {
                // Found the view at this position
                child.handle_event(event);
                if event.what == EventType::Nothing {
                    return;  // Handled
                }
            }
        }

        // No child handled it - handle locally
        self.handle_local_mouse(event);
    }
}
```

### Focused Routing (Keyboard & Commands)

**Focused events** go to the focused view, then propagate up if not handled:

```
Keyboard event
    ↓
Application → selected view (Desktop)
    ↓
Desktop → selected view (Window)
    ↓
Window → selected view (InputLine)
    ↓
InputLine handles or passes back up
```

```rust
impl Group {
    fn handle_focused_event(&mut self, event: &mut Event) {
        if let Some(idx) = self.focused_index {
            // Give to focused child
            self.children[idx].handle_event(event);

            if event.what == EventType::Nothing {
                return;  // Focused view handled it
            }
        }

        // Focused view didn't handle - try ourselves
        self.handle_local_event(event);
    }
}
```

### Broadcast Routing

**Broadcast events** are sent to all views:

```
Broadcast CM_RECORD_CHANGED
    ↓
Application
    ├→ StatusLine (updates)
    ├→ Desktop
    │   ├→ Window1
    │   │   ├→ CounterView (updates)
    │   │   └→ other views
    │   └→ Window2
    └→ MenuBar
```

```rust
impl Group {
    fn handle_broadcast(&mut self, event: &Event) {
        // Send to all children
        for child in &mut self.children {
            let mut event_copy = *event;
            child.handle_event(&mut event_copy);
            // Don't stop if cleared - broadcast goes to everyone
        }

        // Handle locally too
        self.handle_local_event(event);
    }
}
```

---

## Three-Phase Event Processing

Groups process events in three phases (matching Borland's TGroup):

### Phase Overview

```rust
impl Group {
    fn handle_event(&mut self, event: &mut Event) {
        // PHASE 1: PreProcess
        // Views with OF_PRE_PROCESS handle events first
        for child in &mut self.children {
            if child.options() & OF_PRE_PROCESS != 0 {
                child.handle_event(event);
                if event.what == EventType::Nothing {
                    return;  // Consumed
                }
            }
        }

        // PHASE 2: Focused
        // Normal event routing to focused view
        if let Some(idx) = self.focused_index {
            self.children[idx].handle_event(event);
            if event.what == EventType::Nothing {
                return;
            }
        }

        // PHASE 3: PostProcess
        // Views with OF_POST_PROCESS handle events last
        for child in &mut self.children {
            if child.options() & OF_POST_PROCESS != 0 {
                child.handle_event(event);
                if event.what == EventType::Nothing {
                    return;
                }
            }
        }
    }
}
```

### Phase 1: PreProcess

**Purpose:** Intercept events before the focused view sees them

**Example:** StatusLine uses PreProcess to handle F1 (help):

```rust
impl View for StatusLine {
    fn options(&self) -> u16 {
        OF_PRE_PROCESS  // See events first
    }

    fn handle_event(&mut self, event: &mut Event) {
        if event.what == EventType::Keyboard {
            if event.key_code == KB_F1 {
                // Show help - don't let focused view see F1
                self.show_help();
                event.clear();
                return;
            }
        }
        // Other events pass through
    }
}
```

### Phase 2: Focused

**Purpose:** Normal event processing for the focused view

```rust
// InputLine has focus - receives normal keyboard events
impl View for InputLine {
    fn handle_event(&mut self, event: &mut Event) {
        if event.what == EventType::Keyboard {
            if let Some(ch) = key_to_char(event.key_code) {
                self.insert_char(ch);
                event.clear();
            }
        }
    }
}
```

### Phase 3: PostProcess

**Purpose:** Handle events the focused view didn't consume

**Example:** Button handles Alt+letter in PostProcess:

```rust
impl View for Button {
    fn options(&self) -> u16 {
        OF_SELECTABLE | OF_POST_PROCESS  // See events after focused view
    }

    fn handle_event(&mut self, event: &mut Event) {
        if event.what == EventType::Keyboard {
            // Check for hotkey
            if event.key_code == self.hotkey {
                self.press();
                *event = Event::command(self.command_id);
                event.clear();
            }
        }
    }
}
```

### Why Three Phases?

**Problem:** Input line has focus, user wants to press Alt+O to activate OK button

**Without phases:**
- Input line sees Alt+O first
- Input line might consume it
- Button never sees the shortcut

**With phases:**
```
Alt+O pressed
    ↓
Phase 1: PreProcess (StatusLine) - doesn't handle Alt+O
    ↓
Phase 2: Focused (InputLine) - doesn't handle Alt+O
    ↓
Phase 3: PostProcess (Button) - handles Alt+O! ✓
```

### Pascal vs. Rust Comparison

| Pascal | Rust |
|--------|------|
| `ofPreProcess` flag | `OF_PRE_PROCESS` const |
| `ofPostProcess` flag | `OF_POST_PROCESS` const |
| `Owner^.Phase` field | Phase implicit in call order |
| Check phase in HandleEvent | Same - check in handle_event |

---

## Commands

Commands are high-level actions identified by numeric constants.

### Defining Commands

```rust
// Standard commands (0-99 reserved by framework)
pub const CM_QUIT: u16 = 1;
pub const CM_HELP: u16 = 2;
pub const CM_CUT: u16 = 20;
pub const CM_COPY: u16 = 21;
pub const CM_PASTE: u16 = 22;

// Application commands (100-255 can be disabled)
pub const CM_NEW: u16 = 100;
pub const CM_OPEN: u16 = 101;
pub const CM_SAVE: u16 = 102;
pub const CM_SAVE_AS: u16 = 103;

// Application commands (1000+ cannot be disabled)
pub const CM_RECORD_CHANGED: u16 = 1000;
pub const CM_REFRESH_VIEW: u16 = 1001;
```

### Command Ranges

| Range | Usage | Can Disable? |
|-------|-------|--------------|
| 0-99 | Framework reserved | Yes |
| 100-255 | App commands | Yes |
| 256-999 | Framework reserved | No |
| 1000+ | App commands | No |

### Generating Commands

#### From Buttons

```rust
let button = Button::new(
    Rect::new(10, 10, 20, 12),
    "~O~K",
    CM_OK,     // Generates this command when clicked
    true       // Default button
);
```

#### From Menu Items

```rust
let menu = Menu::new(vec![
    MenuItem::new("~N~ew", KB_CTRL_N, CM_NEW),
    MenuItem::new("~O~pen...", KB_F3, CM_OPEN),
    MenuItem::separator(),
    MenuItem::new("E~x~it", KB_ALT_X, CM_QUIT),
]);
```

#### From Status Line

```rust
let status_line = StatusLine::new(
    bounds,
    vec![
        StatusItem::new("~F1~ Help", KB_F1, CM_HELP),
        StatusItem::new("~Alt+X~ Exit", KB_ALT_X, CM_QUIT),
    ]
);
```

#### Manual Generation

```rust
impl View for MyView {
    fn handle_event(&mut self, event: &mut Event) {
        if event.what == EventType::MouseDown {
            if self.bounds().contains(event.mouse.position) {
                // Transform mouse event into command
                *event = Event::command(CM_MY_COMMAND);
            }
        }
    }
}
```

### Handling Commands

```rust
impl View for Application {
    fn handle_event(&mut self, event: &mut Event) {
        if event.what == EventType::Command {
            match event.command {
                CM_NEW => {
                    self.create_new();
                    event.clear();
                }
                CM_OPEN => {
                    if let Some(path) = self.show_open_dialog() {
                        self.open_file(path);
                    }
                    event.clear();
                }
                CM_QUIT => {
                    if self.confirm_quit() {
                        self.running = false;
                    }
                    event.clear();
                }
                _ => {}
            }
        }
    }
}
```

### Command Enabling/Disabling

Commands in range 0-255 can be enabled/disabled:

```rust
pub struct CommandSet {
    enabled: [bool; 256],
}

impl CommandSet {
    pub fn enable(&mut self, command: u16) {
        if command < 256 {
            self.enabled[command as usize] = true;
        }
    }

    pub fn disable(&mut self, command: u16) {
        if command < 256 {
            self.enabled[command as usize] = false;
        }
    }

    pub fn is_enabled(&self, command: u16) -> bool {
        if command < 256 {
            self.enabled[command as usize]
        } else {
            true  // Commands >= 256 always enabled
        }
    }
}
```

**Usage:**
```rust
impl Application {
    pub fn update_commands(&mut self) {
        if self.has_windows() {
            self.commands.enable(CM_CLOSE);
            self.commands.enable(CM_NEXT);
        } else {
            self.commands.disable(CM_CLOSE);
            self.commands.disable(CM_NEXT);
        }
    }
}
```

**Menu/StatusLine Integration:**
Views automatically gray out disabled commands in menus and status lines.

---

## Handling Events

### The HandleEvent Method

Every view implements `handle_event`:

```rust
pub trait View {
    fn handle_event(&mut self, event: &mut Event);
}
```

### Basic Pattern

```rust
impl View for MyView {
    fn handle_event(&mut self, event: &mut Event) {
        match event.what {
            EventType::Keyboard => {
                // Handle keyboard events
                if event.key_code == KB_ENTER {
                    self.do_action();
                    event.clear();
                }
            }
            EventType::MouseDown => {
                // Handle mouse clicks
                if self.bounds().contains(event.mouse.position) {
                    self.on_click(event.mouse.position);
                    event.clear();
                }
            }
            EventType::Command => {
                // Handle commands
                match event.command {
                    CM_MY_COMMAND => {
                        self.execute_command();
                        event.clear();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
```

### Clearing Events

**Critical:** Always clear events you handle:

```rust
if event.key_code == KB_ENTER {
    self.submit();
    event.clear();  // ← Must clear!
}
```

**What clear() does:**
```rust
impl Event {
    pub fn clear(&mut self) {
        self.what = EventType::Nothing;
    }
}
```

**Why clear?**
- Tells parent "I handled this event"
- Prevents duplicate handling
- Stops event propagation

**When NOT to clear:**
- Broadcasting information (let others see it too)
- Transforming event (changing what/command, not clearing)

### Event Transformation

Instead of clearing, sometimes you transform the event:

```rust
impl View for Button {
    fn handle_event(&mut self, event: &mut Event) {
        if event.what == EventType::MouseDown {
            if self.bounds().contains(event.mouse.position) {
                // Transform mouse click into command
                *event = Event::command(self.command_id);
                // Note: NOT cleared - parent will receive command
            }
        }
    }
}
```

### Calling Parent Behavior

In Pascal, you'd call inherited HandleEvent. In Rust with composition:

```rust
impl View for Window {
    fn handle_event(&mut self, event: &mut Event) {
        // Handle window-specific events first
        match event.what {
            EventType::Command => {
                if event.command == CM_CLOSE {
                    self.close();
                    event.clear();
                    return;
                }
            }
            _ => {}
        }

        // Delegate to interior group
        self.interior.handle_event(event);
    }
}
```

### Complete Example: Custom View

```rust
pub struct CalculatorView {
    bounds: Rect,
    display: String,
    state: StateFlags,
}

impl View for CalculatorView {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        // Draw calculator display
        let width = self.bounds.width() as usize;
        let mut buf = DrawBuffer::new(width);

        buf.move_char(0, ' ', colors::INPUT_NORMAL, width);
        buf.move_str(1, &self.display, colors::INPUT_NORMAL);

        write_line_to_terminal(
            terminal,
            self.bounds.a.x,
            self.bounds.a.y,
            &buf
        );
    }

    fn handle_event(&mut self, event: &mut Event) {
        match event.what {
            EventType::Keyboard => {
                match event.key_code {
                    // Digits
                    KB_0..=KB_9 => {
                        let digit = (event.key_code - KB_0) as u8;
                        self.add_digit(digit);
                        event.clear();
                    }
                    // Operators
                    KB_PLUS => {
                        self.operator('+');
                        event.clear();
                    }
                    KB_MINUS => {
                        self.operator('-');
                        event.clear();
                    }
                    KB_ENTER => {
                        self.calculate();
                        event.clear();
                    }
                    KB_ESC => {
                        self.clear();
                        event.clear();
                    }
                    _ => {}
                }
            }
            EventType::Command => {
                match event.command {
                    CM_CLEAR => {
                        self.clear();
                        event.clear();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn state(&self) -> StateFlags {
        self.state
    }

    fn set_state(&mut self, state: StateFlags) {
        self.state = state;
    }
}
```

---

## Inter-View Communication

Views need to communicate without having references to each other (Rust ownership prevents this).

### The Problem

**Pascal approach (not possible in Rust):**
```pascal
type
  TView = object
    Owner: PGroup;  // Raw pointer to parent
  end;

// Send message to parent
message(Owner, evBroadcast, cmUpdate, @Self);
```

**Rust problem:**
```rust
pub struct Child {
    parent: ???  // Can't store reference - would violate ownership!
}
```

### Solution 1: Event Transformation

Most common pattern - transform events to notify parent:

```rust
impl View for ScrollBar {
    fn handle_event(&mut self, event: &mut Event) {
        if event.what == EventType::MouseDown {
            if self.handle_thumb_click(event.mouse.position) {
                // Notify parent that scroll position changed
                *event = Event::broadcast(CM_SCROLLBAR_CHANGED);
                // Parent will receive this broadcast
            }
        }
    }
}

// Parent handles the broadcast
impl View for Scroller {
    fn handle_event(&mut self, event: &mut Event) {
        if event.what == EventType::Broadcast {
            if event.command == CM_SCROLLBAR_CHANGED {
                self.update_view();
                // Don't clear - let others see it
            }
        }
    }
}
```

### Solution 2: Shared State

For data-bound controls, use `Rc<RefCell<T>>`:

```rust
// Shared data
let name = Rc::new(RefCell::new(String::new()));

// Multiple views share the same data
let input = InputLine::new(bounds1, 50, name.clone());
let label = Label::new(bounds2, &format!("Name: {}", name.borrow()));

dialog.add(Box::new(input));
dialog.add(Box::new(label));

// When InputLine changes data, label can read it
```

### Solution 3: Application-Level State

Store shared state in Application:

```rust
pub struct MyApp {
    pub terminal: Terminal,
    pub desktop: Desktop,
    pub current_record: usize,
    pub total_records: usize,
}

impl MyApp {
    pub fn next_record(&mut self) {
        if self.current_record < self.total_records {
            self.current_record += 1;
            // Broadcast to all views
            let event = Event::broadcast(CM_RECORD_CHANGED);
            self.desktop.handle_event(&mut event);
        }
    }
}
```

### Solution 4: Return Values from Modal Dialogs

```rust
// Execute modal dialog - returns command that closed it
let result = confirm_dialog.execute(&mut app);

match result {
    CM_OK => {
        // User confirmed
        app.delete_file();
    }
    CM_CANCEL => {
        // User cancelled
    }
    _ => {}
}
```

### Broadcasting Messages

Send a message to all views:

```rust
impl Application {
    pub fn broadcast(&mut self, command: u16) {
        let mut event = Event::broadcast(command);

        // StatusLine receives it
        if let Some(ref mut status_line) = self.status_line {
            status_line.handle_event(&mut event);
        }

        // Desktop and all windows receive it
        self.desktop.handle_event(&mut event);

        // MenuBar receives it
        if let Some(ref mut menu_bar) = self.menu_bar {
            menu_bar.handle_event(&mut event);
        }
    }
}
```

### Finding Views

**Pascal approach:**
```pascal
// Broadcast to find if a view exists
AreYouThere := Message(Desktop, evBroadcast, cmFindWatchWindow, nil);
if AreYouThere = nil then
  CreateWatchWindow
else
  AreYouThereĜ.Select;
```

**Rust approach:**
```rust
// Store references in application
pub struct MyApp {
    watch_window: Option<Rc<RefCell<WatchWindow>>>,
}

impl MyApp {
    pub fn show_watch_window(&mut self) {
        if let Some(ref win) = self.watch_window {
            // Window exists - bring to front
            win.borrow_mut().select();
        } else {
            // Create new window
            let win = Rc::new(RefCell::new(WatchWindow::new()));
            self.desktop.add(Box::new(win.borrow_mut()));
            self.watch_window = Some(win);
        }
    }
}
```

### Communication Patterns Summary

| Pattern | Use When | Example |
|---------|----------|---------|
| Event Transformation | Child → Parent notification | ScrollBar → Scroller |
| Shared State | Multiple views share data | Dialog controls |
| Application State | Global application state | Current record number |
| Broadcast | Notify all views | Data changed |
| Return Value | Modal dialog result | Confirm dialog |

---

## Advanced Topics

### Custom Event Types

You can extend EventType for application-specific events:

```rust
// Define custom event types
pub const EV_USER: u16 = 0x1000;
pub const EV_TIMER: u16 = 0x2000;
pub const EV_NETWORK: u16 = 0x4000;

// Extend Event structure
pub struct ExtendedEvent {
    base: Event,
    timer_id: Option<u32>,
    network_data: Option<Vec<u8>>,
}
```

### Idle Time Processing

Handle background tasks when no events are pending:

```rust
impl Application {
    pub fn run(&mut self) {
        while self.running {
            let mut event = self.get_event_with_timeout(Duration::from_millis(50));

            if event.what == EventType::Nothing {
                // No event - do idle processing
                self.idle();
            } else {
                self.handle_event(&mut event);
            }

            self.draw();
            self.terminal.flush();
        }
    }

    fn idle(&mut self) {
        // Update clock display
        if let Some(ref mut clock) = self.clock_view {
            clock.update();
        }

        // Check for network messages
        self.check_network();

        // Auto-save
        if self.time_since_save() > Duration::from_secs(300) {
            self.auto_save();
        }
    }
}
```

### Event Filters

Pre-process events before routing:

```rust
impl Application {
    fn get_event(&mut self) -> Event {
        let mut event = self.terminal.poll_event();

        // Filter events
        event = self.filter_event(event);

        event
    }

    fn filter_event(&mut self, mut event: Event) -> Event {
        // Record to macro?
        if self.recording_macro {
            self.macro_buffer.push(event.clone());
        }

        // Replay macro?
        if self.playing_macro {
            if let Some(recorded) = self.macro_buffer.pop() {
                return recorded;
            }
        }

        // Keyboard shortcuts
        if event.what == EventType::Keyboard {
            if event.key_code == KB_CTRL_Q {
                // Ctrl+Q -> Quit command
                return Event::command(CM_QUIT);
            }
        }

        event
    }
}
```

### Modal Views

Creating custom modal views:

```rust
pub struct CustomModal {
    window: Window,
    result: u16,
}

impl CustomModal {
    pub fn execute(&mut self, app: &mut Application) -> u16 {
        self.window.set_state_flag(SF_MODAL, true);

        let mut running = true;
        let mut result = 0;

        while running {
            // Draw
            self.window.draw(&mut app.terminal);
            app.terminal.flush();

            // Get event
            if let Ok(Some(mut event)) = app.terminal.poll_event(None) {
                // Handle event
                self.window.handle_event(&mut event);

                // Check for closing commands
                if event.what == EventType::Command {
                    match event.command {
                        CM_OK | CM_CANCEL | CM_YES | CM_NO => {
                            result = event.command;
                            running = false;
                        }
                        _ => {}
                    }
                }
            }
        }

        self.window.set_state_flag(SF_MODAL, false);
        result
    }
}
```

### Event Priorities

Some events should be processed before others:

```rust
impl Application {
    fn handle_event(&mut self, event: &mut Event) {
        // Critical commands first
        if event.what == EventType::Command {
            match event.command {
                CM_QUIT => {
                    self.running = false;
                    event.clear();
                    return;
                }
                CM_SUSPEND => {
                    self.suspend();
                    event.clear();
                    return;
                }
                _ => {}
            }
        }

        // Then normal routing
        if let Some(ref mut status_line) = self.status_line {
            status_line.handle_event(event);
        }

        if event.what != EventType::Nothing {
            self.desktop.handle_event(event);
        }
    }
}
```

---

## Best Practices

### 1. Always Clear Handled Events

```rust
// ✓ Good
if event.key_code == KB_ENTER {
    self.submit();
    event.clear();  // Mark as handled
}

// ✗ Bad
if event.key_code == KB_ENTER {
    self.submit();
    // Forgot to clear - parent will also handle!
}
```

### 2. Check Event Type First

```rust
// ✓ Good - efficient
if event.what == EventType::Keyboard {
    match event.key_code {
        KB_ENTER => { /* ... */ }
        _ => {}
    }
}

// ✗ Bad - checks key_code on all event types
match event.key_code {
    KB_ENTER => { /* ... */ }
    _ => {}
}
```

### 3. Use Three-Phase Processing Appropriately

```rust
// ✓ PreProcess - intercept before focused view
impl View for StatusLine {
    fn options(&self) -> u16 {
        OF_PRE_PROCESS  // Handle F1 help before anyone else
    }
}

// ✓ PostProcess - handle after focused view
impl View for Button {
    fn options(&self) -> u16 {
        OF_POST_PROCESS  // Handle Alt+letter even when unfocused
    }
}
```

### 4. Don't Store Parent References

```rust
// ✗ Bad - violates ownership
pub struct Child {
    parent: &mut Parent,  // Won't compile!
}

// ✓ Good - use event transformation
impl View for Child {
    fn handle_event(&mut self, event: &mut Event) {
        *event = Event::broadcast(CM_CHILD_CHANGED);
        // Parent receives broadcast through call stack
    }
}
```

### 5. Use Commands for High-Level Actions

```rust
// ✓ Good - button generates command
let button = Button::new(bounds, "~O~K", CM_OK, true);

// ✗ Bad - button directly calls function
// (Not possible in Turbo Vision - this is why commands exist!)
```

### 6. Broadcasts Don't Block

```rust
// ✓ Good - don't clear broadcasts (let others see)
if event.what == EventType::Broadcast {
    match event.command {
        CM_UPDATE => {
            self.refresh();
            // Don't clear!
        }
        _ => {}
    }
}
```

---

## Pascal vs. Rust Comparison

| Concept | Pascal | Rust |
|---------|--------|------|
| **Event Record** | `TEvent = record` | `pub struct Event` |
| **Event Type** | `What: Word` | `what: EventType` (enum) |
| **Mouse Events** | `evMouse` mask | `MouseDown/Up/Move/Drag` variants |
| **Keyboard** | `evKeyDown` | `Keyboard` variant |
| **Commands** | `evCommand` | `Command` variant |
| **Clear Event** | `ClearEvent(Event)` | `event.clear()` |
| **Handle Events** | `procedure HandleEvent` | `fn handle_event` (trait method) |
| **Parent Pointer** | `Owner: PGroup` | Event transformation |
| **Message** | `Message(View, ...)` | `Event::broadcast()` |
| **Phase Check** | `Owner^.Phase` | Implicit in call order |
| **PreProcess** | `ofPreProcess` flag | `OF_PRE_PROCESS` const |
| **PostProcess** | `ofPostProcess` flag | `OF_POST_PROCESS` const |

---

## Complete Example: Event-Driven Calculator

```rust
use turbo_vision::prelude::*;

// Commands
const CM_DIGIT: u16 = 100;
const CM_OPERATOR: u16 = 101;
const CM_EQUALS: u16 = 102;
const CM_CLEAR: u16 = 103;

pub struct Calculator {
    window: Window,
    display: Rc<RefCell<String>>,
    accumulator: f64,
    operator: Option<char>,
}

impl Calculator {
    pub fn new() -> Self {
        let bounds = Rect::new(20, 5, 60, 18);
        let mut window = Window::new(bounds, "Calculator");

        let display = Rc::new(RefCell::new(String::from("0")));

        // Display
        let display_view = InputLine::new(
            Rect::new(2, 2, 36, 3),
            20,
            display.clone()
        );
        window.add(Box::new(display_view));

        // Number buttons
        let mut y = 4;
        for row in 0..3 {
            let mut x = 2;
            for col in 0..3 {
                let digit = row * 3 + col + 1;
                let button = Button::new(
                    Rect::new(x, y, x + 8, y + 2),
                    &digit.to_string(),
                    CM_DIGIT + digit as u16,
                    false
                );
                window.add(Box::new(button));
                x += 9;
            }
            y += 3;
        }

        // Zero button
        let zero = Button::new(
            Rect::new(2, 13, 19, 15),
            "0",
            CM_DIGIT,
            false
        );
        window.add(Box::new(zero));

        // Operator buttons
        let operators = vec![
            ("+", 29, 4),
            ("-", 29, 7),
            ("*", 29, 10),
            ("/", 29, 13),
        ];

        for (op, x, y) in operators {
            let button = Button::new(
                Rect::new(x, y, x + 8, y + 2),
                op,
                CM_OPERATOR,
                false
            );
            window.add(Box::new(button));
        }

        // Equals button
        let equals = Button::new(
            Rect::new(20, 13, 28, 15),
            "=",
            CM_EQUALS,
            false
        );
        window.add(Box::new(equals));

        // Clear button
        let clear = Button::new(
            Rect::new(2, 16, 19, 18),
            "Clear",
            CM_CLEAR,
            false
        );
        window.add(Box::new(clear));

        Self {
            window,
            display,
            accumulator: 0.0,
            operator: None,
        }
    }

    fn handle_digit(&mut self, digit: u8) {
        let mut display = self.display.borrow_mut();
        if *display == "0" {
            *display = digit.to_string();
        } else {
            display.push_str(&digit.to_string());
        }
    }

    fn handle_operator(&mut self, op: char) {
        let display = self.display.borrow();
        if let Ok(value) = display.parse::<f64>() {
            if let Some(prev_op) = self.operator {
                self.accumulator = match prev_op {
                    '+' => self.accumulator + value,
                    '-' => self.accumulator - value,
                    '*' => self.accumulator * value,
                    '/' => self.accumulator / value,
                    _ => value,
                };
            } else {
                self.accumulator = value;
            }
        }

        self.operator = Some(op);
        *self.display.borrow_mut() = String::from("0");
    }

    fn handle_equals(&mut self) {
        if let Some(op) = self.operator {
            let display = self.display.borrow();
            if let Ok(value) = display.parse::<f64>() {
                let result = match op {
                    '+' => self.accumulator + value,
                    '-' => self.accumulator - value,
                    '*' => self.accumulator * value,
                    '/' => self.accumulator / value,
                    _ => value,
                };

                *self.display.borrow_mut() = result.to_string();
                self.accumulator = result;
                self.operator = None;
            }
        }
    }

    fn handle_clear(&mut self) {
        *self.display.borrow_mut() = String::from("0");
        self.accumulator = 0.0;
        self.operator = None;
    }
}

impl View for Calculator {
    fn bounds(&self) -> Rect {
        self.window.bounds()
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.window.set_bounds(bounds);
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        self.window.draw(terminal);
    }

    fn handle_event(&mut self, event: &mut Event) {
        // Handle our commands
        if event.what == EventType::Command {
            match event.command {
                CM_DIGIT..=CM_DIGIT + 9 => {
                    let digit = (event.command - CM_DIGIT) as u8;
                    self.handle_digit(digit);
                    event.clear();
                    return;
                }
                CM_OPERATOR => {
                    // Operator stored in event somehow
                    // (Would need custom event data)
                    event.clear();
                    return;
                }
                CM_EQUALS => {
                    self.handle_equals();
                    event.clear();
                    return;
                }
                CM_CLEAR => {
                    self.handle_clear();
                    event.clear();
                    return;
                }
                _ => {}
            }
        }

        // Delegate to window
        self.window.handle_event(event);
    }

    fn state(&self) -> StateFlags {
        self.window.state()
    }

    fn set_state(&mut self, state: StateFlags) {
        self.window.set_state(state);
    }
}
```

---

## Summary

### Key Concepts

1. **Event-Driven** - Framework reads input, packages events, routes to views
2. **Event Types** - Mouse, Keyboard, Command, Broadcast, Nothing
3. **Event Routing** - Positional (mouse), Focused (keyboard), Broadcast (all)
4. **Three Phases** - PreProcess, Focused, PostProcess
5. **Commands** - High-level actions (numeric IDs)
6. **Clear Events** - Mark as handled with `event.clear()`
7. **Transform Events** - Change type/command to notify parent
8. **No Parent Pointers** - Use event transformation instead

### The Event Lifecycle

```
User Input
    ↓
Terminal captures (keyboard/mouse)
    ↓
Application.get_event() packages into Event
    ↓
Application.handle_event() routes:
    ├─→ Phase 1: PreProcess views (StatusLine)
    ├─→ Phase 2: Focused view
    └─→ Phase 3: PostProcess views (Buttons)
    ↓
View handles or transforms event
    ↓
Event cleared or propagated back up
    ↓
Draw changes, flush terminal
```

### Pascal → Rust Event Patterns

| Pascal Pattern | Rust Equivalent |
|----------------|-----------------|
| Check `Event.What and evMouse` | `match event.what { MouseDown \\| MouseUp => ...}` |
| `ClearEvent(Event)` | `event.clear()` |
| `Message(Owner, evBroadcast, ...)` | `*event = Event::broadcast(...)` |
| Check `Owner^.Phase` | Implicit in three-phase loop |
| `ofPreProcess` option | `OF_PRE_PROCESS` const |

---

## See Also

- **Chapter 7** - Architecture Overview
- **Chapter 8** - Views and Groups
- **Chapter 10** - Application Objects (upcoming)
- **docs/TURBOVISION-DESIGN.md** - Implementation details
- **src/core/event.rs** - Event definitions
- **examples/dialogs_demo.rs** - Event handling examples
- **examples/menu.rs** - Command handling

---

Event-driven programming is the heart of Turbo Vision. Master these concepts and you can build sophisticated, responsive terminal applications that handle complex user interactions with ease.
