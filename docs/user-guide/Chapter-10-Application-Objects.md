# Chapter 10 — Application Objects (Rust Edition)

**Version:** 0.2.11
**Updated:** 2025-11-04

This chapter explores the Application object, the heart of every Turbo Vision program. The Application manages the entire screen, coordinates all views, handles the event loop, and provides the framework for your terminal application.

**Prerequisites:** Chapters 7-9 (Architecture, Views, Events)

---

## Table of Contents

1. [Understanding Application Objects](#understanding-application-objects)
2. [The Application Lifecycle](#the-application-lifecycle)
3. [Creating Your Application](#creating-your-application)
4. [Customizing the Desktop](#customizing-the-desktop)
5. [Customizing the Status Line](#customizing-the-status-line)
6. [Customizing the Menu Bar](#customizing-the-menu-bar)
7. [Using Idle Time](#using-idle-time)
8. [Context-Sensitive Help](#context-sensitive-help)

---

## Understanding Application Objects

### Three Roles

The Application object plays three critical roles:

#### 1. Application is a View

The Application occupies the **entire screen**:

```rust
pub struct Application {
    bounds: Rect,       // Always (0, 0, width, height)
    terminal: Terminal,
    // ...
}

impl View for Application {
    fn bounds(&self) -> Rect {
        // Entire screen
        Rect::new(0, 0, self.terminal.width(), self.terminal.height())
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        // Draw all children (menu bar, desktop, status line)
    }
}
```

While the application itself isn't visible, it owns and manages the entire screen real estate.

#### 2. Application is a Group

The Application contains three key subviews:

```
┌────────────────────────────────────────────┐
│ MenuBar (optional)                         │ ← Top line
├────────────────────────────────────────────┤
│                                            │
│                                            │
│           Desktop                          │ ← Middle area
│                                            │
│                                            │
├────────────────────────────────────────────┤
│ StatusLine (optional)                      │ ← Bottom line
└────────────────────────────────────────────┘
```

```rust
pub struct Application {
    pub terminal: Terminal,
    pub desktop: Desktop,
    pub menu_bar: Option<MenuBar>,
    pub status_line: Option<StatusLine>,
    pub running: bool,
}
```

**Ownership Chain:**
- Application owns Desktop
- Desktop owns Windows
- Windows own Dialogs/Controls
- Every view in the program ultimately belongs to Application

#### 3. Application is Modal

Most of the time, Application is the **modal view** — it receives all events first:

```rust
while self.running {
    let event = self.get_event();
    self.handle_event(event);  // Application handles or delegates
}
```

**Exception:** When a dialog is executed, the dialog becomes modal temporarily.

### Pascal vs. Rust Comparison

| Pascal | Rust |
|--------|------|
| `TApplication = object(TProgram)` | `impl Application` (no inheritance) |
| `Init` constructor | `Application::new()` |
| `Run` method | `app.run()` |
| `Done` destructor | Automatic `Drop` |
| Global `Application` variable | Pass `&mut app` as parameter |
| `InitDesktop` virtual method | Builder pattern or constructor |
| `InitMenuBar` virtual method | Builder pattern or constructor |
| `InitStatusLine` virtual method | Builder pattern or constructor |

---

## The Application Lifecycle

### The Three-Statement Main

In Pascal, every Turbo Vision program had this structure:

```pascal
var MyApp: TApplication;
begin
  MyApp.Init;     // Constructor
  MyApp.Run;      // Main loop
  MyApp.Done;     // Destructor
end.
```

**Rust equivalent:**

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = Application::new()?;  // Init
    app.run()?;                         // Run
    Ok(())                              // Done automatic (Drop)
}
```

### Application::new()

Creates and initializes the application:

```rust
impl Application {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize terminal
        let mut terminal = Terminal::new()?;
        let (width, height) = terminal.size()?;

        // Create desktop (middle area)
        let desktop = Desktop::new(Rect::new(
            0,
            1,              // Below menu bar
            width,
            height - 1,     // Above status line
        ));

        // Create menu bar (top line)
        let menu_bar = Some(Self::init_menu_bar(width));

        // Create status line (bottom line)
        let status_line = Some(Self::init_status_line(width, height));

        Ok(Self {
            terminal,
            desktop,
            menu_bar,
            status_line,
            running: false,
        })
    }
}
```

**What happens:**
1. Terminal initialized (raw mode, mouse support)
2. Desktop created for middle area
3. Menu bar created for top line
4. Status line created for bottom line
5. Ready to run!

### Application::run()

The main event loop:

```rust
impl Application {
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.running = true;

        while self.running {
            // Draw everything
            self.draw();
            self.terminal.flush()?;

            // Get next event
            let mut event = self.get_event()?;

            // Route event
            self.handle_event(&mut event);

            // Check for unhandled events
            if event.what != EventType::Nothing {
                self.event_error(&event);
            }
        }

        Ok(())
    }

    fn get_event(&mut self) -> Result<Event, Box<dyn std::error::Error>> {
        // Check for pending events
        if let Some(event) = self.pending_event.take() {
            return Ok(event);
        }

        // Poll terminal for new event
        match self.terminal.poll_event(Some(Duration::from_millis(50)))? {
            Some(event) => Ok(event),
            None => {
                // No event - call idle
                self.idle();
                Ok(Event::nothing())
            }
        }
    }
}
```

**The Loop:**
1. Draw all views
2. Flush to screen
3. Get next event (keyboard, mouse, command)
4. Route event to views
5. Handle idle time if no events
6. Repeat until `CM_QUIT` received

### Automatic Cleanup

In Pascal, you explicitly called `Done`:

```pascal
MyApp.Done;  // Free menu bar, status line, desktop, terminal
```

**In Rust, `Drop` handles cleanup automatically:**

```rust
impl Drop for Application {
    fn drop(&mut self) {
        // Terminal restoration happens automatically
        // All owned views (desktop, menu_bar, status_line) dropped
        // No manual cleanup needed!
    }
}
```

---

## Creating Your Application

### Minimal Application

```rust
use turbo_vision::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create application
    let mut app = Application::new()?;

    // Run
    app.run()?;

    Ok(())
}
```

This creates an application with empty desktop, no menu bar, no status line.

### Application with Menu Bar

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = Application::new()?;

    // Add menu bar
    let (width, _) = app.terminal.size()?;
    let menu_bar = MenuBar::new(
        Rect::new(0, 0, width, 1),
        vec![
            MenuBarItem::submenu(
                "~F~ile",
                vec![
                    MenuItem::new("~N~ew", KB_CTRL_N, CM_NEW),
                    MenuItem::new("~O~pen...", KB_F3, CM_OPEN),
                    MenuItem::separator(),
                    MenuItem::new("E~x~it", KB_ALT_X, CM_QUIT),
                ]
            ),
            MenuBarItem::submenu(
                "~H~elp",
                vec![
                    MenuItem::new("~A~bout", KB_NONE, CM_ABOUT),
                ]
            ),
        ]
    );
    app.menu_bar = Some(menu_bar);

    app.run()?;
    Ok(())
}
```

### Application with Status Line

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = Application::new()?;

    // Add status line
    let (width, height) = app.terminal.size()?;
    let status_line = StatusLine::new(
        Rect::new(0, height - 1, width, height),
        vec![
            StatusItem::new("~F1~ Help", KB_F1, CM_HELP),
            StatusItem::new("~Alt+X~ Exit", KB_ALT_X, CM_QUIT),
        ]
    );
    app.status_line = Some(status_line);

    app.run()?;
    Ok(())
}
```

### Custom Application Struct

For complex applications, create a custom type:

```rust
pub struct MyApp {
    app: Application,
    // Application-specific state
    current_document: Option<PathBuf>,
    modified: bool,
}

impl MyApp {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let app = Application::new()?;

        Ok(Self {
            app,
            current_document: None,
            modified: false,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize windows
        self.create_initial_windows();

        // Run application
        while self.app.running {
            self.app.draw();
            self.app.terminal.flush()?;

            let mut event = self.app.get_event()?;
            self.handle_event(&mut event);

            if event.what != EventType::Nothing {
                self.app.handle_event(&mut event);
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: &mut Event) {
        if event.what == EventType::Command {
            match event.command {
                CM_NEW => {
                    self.new_document();
                    event.clear();
                }
                CM_OPEN => {
                    if let Some(path) = self.show_open_dialog() {
                        self.open_document(path);
                    }
                    event.clear();
                }
                CM_SAVE => {
                    self.save_document();
                    event.clear();
                }
                _ => {}
            }
        }
    }

    fn create_initial_windows(&mut self) {
        // Add windows to desktop
    }

    fn new_document(&mut self) {
        self.current_document = None;
        self.modified = false;
        // Create editor window
    }

    fn open_document(&mut self, path: PathBuf) {
        // Load file
        self.current_document = Some(path);
        self.modified = false;
    }

    fn save_document(&mut self) {
        // Save file
        self.modified = false;
    }

    fn show_open_dialog(&mut self) -> Option<PathBuf> {
        // Show file dialog
        None
    }
}
```

### Builder Pattern

For flexibility, use a builder:

```rust
pub struct ApplicationBuilder {
    title: String,
    with_menu_bar: bool,
    with_status_line: bool,
    menu_items: Vec<MenuBarItem>,
    status_items: Vec<StatusItem>,
}

impl ApplicationBuilder {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            with_menu_bar: true,
            with_status_line: true,
            menu_items: Vec::new(),
            status_items: Self::default_status_items(),
        }
    }

    pub fn menu_bar(mut self, items: Vec<MenuBarItem>) -> Self {
        self.with_menu_bar = true;
        self.menu_items = items;
        self
    }

    pub fn status_line(mut self, items: Vec<StatusItem>) -> Self {
        self.with_status_line = true;
        self.status_items = items;
        self
    }

    pub fn no_menu_bar(mut self) -> Self {
        self.with_menu_bar = false;
        self
    }

    pub fn no_status_line(mut self) -> Self {
        self.with_status_line = false;
        self
    }

    pub fn build(self) -> Result<Application, Box<dyn std::error::Error>> {
        let mut app = Application::new()?;

        if !self.with_menu_bar {
            app.menu_bar = None;
            // Adjust desktop to use top line
            let mut bounds = app.desktop.bounds();
            bounds.a.y = 0;
            app.desktop.set_bounds(bounds);
        } else if !self.menu_items.is_empty() {
            // Custom menu bar
            let (width, _) = app.terminal.size()?;
            app.menu_bar = Some(MenuBar::new(
                Rect::new(0, 0, width, 1),
                self.menu_items
            ));
        }

        if !self.with_status_line {
            app.status_line = None;
            // Adjust desktop to use bottom line
            let mut bounds = app.desktop.bounds();
            bounds.b.y += 1;
            app.desktop.set_bounds(bounds);
        } else {
            // Custom status line
            let (width, height) = app.terminal.size()?;
            app.status_line = Some(StatusLine::new(
                Rect::new(0, height - 1, width, height),
                self.status_items
            ));
        }

        Ok(app)
    }

    fn default_status_items() -> Vec<StatusItem> {
        vec![
            StatusItem::new("~F1~ Help", KB_F1, CM_HELP),
            StatusItem::new("~Alt+X~ Exit", KB_ALT_X, CM_QUIT),
        ]
    }
}

// Usage:
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = ApplicationBuilder::new("My App")
        .menu_bar(vec![
            MenuBarItem::submenu("~F~ile", vec![
                MenuItem::new("~Q~uit", KB_ALT_X, CM_QUIT),
            ]),
        ])
        .build()?;

    app.run()?;
    Ok(())
}
```

---

## Customizing the Desktop

### Desktop Basics

The Desktop is the main workspace where windows live:

```rust
pub struct Desktop {
    group: Group,
    background: Background,
}

impl Desktop {
    pub fn new(bounds: Rect) -> Self {
        let background = Background::new(bounds, '░');
        let group = Group::new(bounds);

        Self { group, background }
    }

    pub fn add(&mut self, window: Box<dyn View>) {
        self.group.add(window);
    }
}
```

### Inserting Windows

**Direct insertion:**
```rust
let window = Window::new(Rect::new(10, 5, 70, 20), "My Window");
app.desktop.add(Box::new(window));
```

**With validation (Pascal's InsertWindow):**
```rust
impl Application {
    pub fn insert_window(&mut self, window: Box<dyn View>) -> bool {
        // Validate window constructed successfully
        if !self.valid_view(&window) {
            return false;
        }

        // Insert into desktop
        self.desktop.add(window);
        true
    }

    fn valid_view(&self, _view: &Box<dyn View>) -> bool {
        // Check if view is valid
        // (In Rust, construction usually can't fail silently)
        true
    }
}
```

### Executing Modal Dialogs

```rust
impl Application {
    pub fn execute_dialog(&mut self, mut dialog: Dialog) -> u16 {
        // Make modal
        dialog.set_state_flag(SF_MODAL, true);

        let mut running = true;
        let mut result = 0;

        while running {
            // Draw dialog on top of everything
            self.draw();
            dialog.draw(&mut self.terminal);
            self.terminal.flush().ok();

            // Get event
            if let Ok(Some(mut event)) = self.terminal.poll_event(None) {
                // Dialog handles event first
                dialog.handle_event(&mut event);

                // Check for closing command
                if event.what == EventType::Command {
                    match event.command {
                        CM_OK | CM_CANCEL | CM_YES | CM_NO => {
                            result = event.command;
                            running = false;
                        }
                        _ => {}
                    }
                }

                // Application handles remaining events
                if event.what != EventType::Nothing {
                    self.handle_event(&mut event);
                }
            }
        }

        dialog.set_state_flag(SF_MODAL, false);
        result
    }
}

// Usage:
let dialog = Dialog::new(Rect::new(20, 8, 60, 16), "Confirm");
// ... add controls ...

let result = app.execute_dialog(dialog);
if result == CM_OK {
    // User confirmed
}
```

### Arranging Windows

#### Tiling Windows

Arrange windows in non-overlapping tiles:

```rust
impl Desktop {
    pub fn tile(&mut self) {
        let bounds = self.bounds();
        let tileable: Vec<_> = self.group.children
            .iter()
            .filter(|w| w.options() & OF_TILEABLE != 0)
            .collect();

        let count = tileable.len();
        if count == 0 {
            return;
        }

        // Tile vertically
        let height_per_window = bounds.height() / count as i16;

        for (i, window) in tileable.iter().enumerate() {
            let y = bounds.a.y + (i as i16 * height_per_window);
            let new_bounds = Rect::new(
                bounds.a.x,
                y,
                bounds.b.x,
                y + height_per_window
            );
            window.set_bounds(new_bounds);
        }
    }
}
```

#### Cascading Windows

Arrange windows in overlapping cascade:

```rust
impl Desktop {
    pub fn cascade(&mut self) {
        let bounds = self.bounds();
        let tileable: Vec<_> = self.group.children
            .iter()
            .filter(|w| w.options() & OF_TILEABLE != 0)
            .collect();

        let count = tileable.len();
        if count == 0 {
            return;
        }

        let mut x = bounds.a.x;
        let mut y = bounds.a.y;

        for window in tileable {
            let new_bounds = Rect::new(
                x,
                y,
                bounds.b.x - (count as i16 - 1),
                bounds.b.y - (count as i16 - 1)
            );
            window.set_bounds(new_bounds);

            // Offset next window
            x += 1;
            y += 1;
        }
    }
}
```

### Custom Desktop Background

#### Simple Character Background

```rust
impl Application {
    pub fn set_background_char(&mut self, ch: char) {
        self.desktop.background.set_pattern(ch);
    }
}

// Usage:
app.set_background_char('▒');  // Different pattern
app.set_background_char(' ');  // Blank background
app.set_background_char('█');  // Solid background
```

#### Custom Background View

```rust
pub struct CustomBackground {
    bounds: Rect,
    state: StateFlags,
}

impl CustomBackground {
    pub fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            state: SF_VISIBLE,
        }
    }
}

impl View for CustomBackground {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        let width = self.bounds.width() as usize;
        let height = self.bounds.height() as usize;

        // Draw repeating pattern
        let pattern = "Turbo Vision ";

        for row in 0..height {
            let mut buf = DrawBuffer::new(width);

            // Repeat pattern across row
            let mut pos = 0;
            while pos < width {
                let remaining = width - pos;
                let chunk = &pattern[..pattern.len().min(remaining)];
                buf.move_str(pos, chunk, colors::DESKTOP_BACKGROUND);
                pos += chunk.len();
            }

            write_line_to_terminal(
                terminal,
                self.bounds.a.x,
                self.bounds.a.y + row as i16,
                &buf
            );
        }
    }

    fn handle_event(&mut self, _event: &mut Event) {
        // Background doesn't handle events
    }

    fn state(&self) -> StateFlags {
        self.state
    }

    fn set_state(&mut self, state: StateFlags) {
        self.state = state;
    }
}

// Usage:
app.desktop.background = CustomBackground::new(app.desktop.bounds());
```

---

## Customizing the Status Line

### Status Line Basics

The status line shows available commands at the bottom:

```
┌──────────────────────────────────────────────────────┐
│ ~F1~ Help   ~Alt+X~ Exit   ~F5~ Zoom   ~F6~ Next   │
└──────────────────────────────────────────────────────┘
```

### Creating a Status Line

```rust
let (width, height) = app.terminal.size()?;

let status_line = StatusLine::new(
    Rect::new(0, height - 1, width, height),
    vec![
        StatusItem::new("~F1~ Help", KB_F1, CM_HELP),
        StatusItem::new("~F10~ Menu", KB_F10, CM_MENU),
        StatusItem::new("~Alt+X~ Exit", KB_ALT_X, CM_QUIT),
    ]
);

app.status_line = Some(status_line);
```

### Context-Sensitive Status Lines

Show different items based on help context:

```rust
pub struct ContextStatusLine {
    bounds: Rect,
    items_by_context: HashMap<u16, Vec<StatusItem>>,
    default_items: Vec<StatusItem>,
    current_context: u16,
    state: StateFlags,
}

impl ContextStatusLine {
    pub fn new(bounds: Rect) -> Self {
        let default_items = vec![
            StatusItem::new("~F1~ Help", KB_F1, CM_HELP),
            StatusItem::new("~Alt+X~ Exit", KB_ALT_X, CM_QUIT),
        ];

        let mut items_by_context = HashMap::new();

        // Context 1000: Editor windows
        items_by_context.insert(1000, vec![
            StatusItem::new("~F2~ Save", KB_F2, CM_SAVE),
            StatusItem::new("~F3~ Open", KB_F3, CM_OPEN),
            StatusItem::new("~Alt+X~ Exit", KB_ALT_X, CM_QUIT),
        ]);

        // Context 2000: File dialogs
        items_by_context.insert(2000, vec![
            StatusItem::new("~Enter~ Select", KB_ENTER, CM_OK),
            StatusItem::new("~Esc~ Cancel", KB_ESC, CM_CANCEL),
        ]);

        Self {
            bounds,
            items_by_context,
            default_items,
            current_context: 0,
            state: SF_VISIBLE,
        }
    }

    pub fn set_context(&mut self, context: u16) {
        self.current_context = context;
    }

    fn get_items(&self) -> &[StatusItem] {
        self.items_by_context
            .get(&self.current_context)
            .map(|v| v.as_slice())
            .unwrap_or(&self.default_items)
    }
}

impl View for ContextStatusLine {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        let width = self.bounds.width() as usize;
        let mut buf = DrawBuffer::new(width);

        // Fill with background
        buf.move_char(0, ' ', colors::STATUS_LINE, width);

        // Draw items for current context
        let items = self.get_items();
        let mut x = 1;

        for item in items {
            if x + item.text.len() + 2 >= width {
                break;  // No more room
            }

            buf.move_str_with_shortcut(
                x,
                &item.text,
                colors::STATUS_LINE,
                colors::STATUS_LINE_SHORTCUT
            );

            x += item.text.len() + 3;  // +3 for spacing
        }

        write_line_to_terminal(
            terminal,
            self.bounds.a.x,
            self.bounds.a.y,
            &buf
        );
    }

    fn handle_event(&mut self, event: &mut Event) {
        // Check if any status item matches this key
        if event.what == EventType::Keyboard {
            for item in self.get_items() {
                if event.key_code == item.key_code {
                    *event = Event::command(item.command);
                    return;
                }
            }
        }
    }

    fn state(&self) -> StateFlags {
        self.state
    }

    fn set_state(&mut self, state: StateFlags) {
        self.state = state;
    }

    fn options(&self) -> u16 {
        OF_PRE_PROCESS  // See events before focused view
    }
}
```

### Status Line Hints

Show dynamic hints based on help context:

```rust
pub struct HintStatusLine {
    base: ContextStatusLine,
    hints: HashMap<u16, String>,
}

impl HintStatusLine {
    pub fn new(bounds: Rect) -> Self {
        let base = ContextStatusLine::new(bounds);

        let mut hints = HashMap::new();
        hints.insert(1000, "Edit document".to_string());
        hints.insert(2000, "Select a file to open".to_string());
        hints.insert(3000, "Enter search text".to_string());

        Self { base, hints }
    }

    fn get_hint(&self) -> &str {
        self.hints
            .get(&self.base.current_context)
            .map(|s| s.as_str())
            .unwrap_or("")
    }
}

impl View for HintStatusLine {
    fn bounds(&self) -> Rect {
        self.base.bounds()
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.base.set_bounds(bounds);
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        // Draw base items
        self.base.draw(terminal);

        // Draw hint on right side
        let hint = self.get_hint();
        if !hint.is_empty() {
            let width = self.bounds().width() as usize;
            let hint_x = width.saturating_sub(hint.len() + 2);

            let mut buf = DrawBuffer::new(hint.len());
            buf.move_str(0, hint, colors::STATUS_LINE_HINT);

            write_line_to_terminal(
                terminal,
                self.bounds().a.x + hint_x as i16,
                self.bounds().a.y,
                &buf
            );
        }
    }

    fn handle_event(&mut self, event: &mut Event) {
        self.base.handle_event(event);
    }

    fn state(&self) -> StateFlags {
        self.base.state()
    }

    fn set_state(&mut self, state: StateFlags) {
        self.base.set_state(state);
    }

    fn options(&self) -> u16 {
        self.base.options()
    }
}
```

---

## Customizing the Menu Bar

### Menu Bar Basics

The menu bar shows top-level menus:

```
┌────────────────────────────────────────┐
│ File  Edit  Search  Window  Help      │
└────────────────────────────────────────┘
```

### Creating a Menu Bar

```rust
let menu_bar = MenuBar::new(
    Rect::new(0, 0, width, 1),
    vec![
        MenuBarItem::submenu(
            "~F~ile",
            vec![
                MenuItem::new("~N~ew", KB_CTRL_N, CM_NEW),
                MenuItem::new("~O~pen...", KB_F3, CM_OPEN),
                MenuItem::new("~S~ave", KB_F2, CM_SAVE),
                MenuItem::separator(),
                MenuItem::new("E~x~it", KB_ALT_X, CM_QUIT),
            ]
        ),
        MenuBarItem::submenu(
            "~E~dit",
            vec![
                MenuItem::new("~C~ut", KB_CTRL_X, CM_CUT),
                MenuItem::new("C~o~py", KB_CTRL_C, CM_COPY),
                MenuItem::new("~P~aste", KB_CTRL_V, CM_PASTE),
            ]
        ),
        MenuBarItem::submenu(
            "~H~elp",
            vec![
                MenuItem::new("~A~bout", KB_NONE, CM_ABOUT),
            ]
        ),
    ]
);

app.menu_bar = Some(menu_bar);
```

### Nested Submenus

```rust
MenuBarItem::submenu(
    "~F~ile",
    vec![
        MenuItem::new("~N~ew", KB_CTRL_N, CM_NEW),
        MenuItem::submenu(
            "~R~ecent Files",
            vec![
                MenuItem::new("document1.txt", KB_NONE, CM_RECENT_1),
                MenuItem::new("document2.txt", KB_NONE, CM_RECENT_2),
                MenuItem::new("document3.txt", KB_NONE, CM_RECENT_3),
            ]
        ),
        MenuItem::separator(),
        MenuItem::new("E~x~it", KB_ALT_X, CM_QUIT),
    ]
)
```

### Dynamic Menus

Update menu items at runtime:

```rust
impl MyApp {
    pub fn update_file_menu(&mut self) {
        if let Some(ref mut menu_bar) = self.app.menu_bar {
            // Find File menu
            if let Some(file_menu) = menu_bar.find_menu("File") {
                // Add recent files
                file_menu.clear_recent();
                for path in &self.recent_files {
                    file_menu.add_item(MenuItem::new(
                        path.display().to_string(),
                        KB_NONE,
                        CM_RECENT_BASE + file_menu.item_count() as u16
                    ));
                }
            }
        }
    }

    pub fn update_edit_menu(&mut self) {
        if let Some(ref mut menu_bar) = self.app.menu_bar {
            if let Some(edit_menu) = menu_bar.find_menu("Edit") {
                // Enable/disable based on state
                edit_menu.enable_item(CM_CUT, self.has_selection());
                edit_menu.enable_item(CM_COPY, self.has_selection());
                edit_menu.enable_item(CM_PASTE, self.can_paste());
            }
        }
    }
}
```

---

## Using Idle Time

### The Idle Method

When no events are pending, Application calls `idle()`:

```rust
impl Application {
    fn get_event(&mut self) -> Result<Event, Box<dyn std::error::Error>> {
        match self.terminal.poll_event(Some(Duration::from_millis(50)))? {
            Some(event) => Ok(event),
            None => {
                // No event - do idle processing
                self.idle();
                Ok(Event::nothing())
            }
        }
    }

    fn idle(&mut self) {
        // Override in your application
    }
}
```

### Clock Example

Display a continuously updating clock:

```rust
pub struct ClockView {
    bounds: Rect,
    state: StateFlags,
    last_time: String,
}

impl ClockView {
    pub fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            state: SF_VISIBLE,
            last_time: String::new(),
        }
    }

    pub fn update(&mut self) {
        use chrono::Local;
        let now = Local::now();
        let time = now.format("%H:%M:%S").to_string();

        if time != self.last_time {
            self.last_time = time;
            // Mark for redraw
        }
    }
}

impl View for ClockView {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        let width = self.bounds.width() as usize;
        let mut buf = DrawBuffer::new(width);

        buf.move_char(0, ' ', colors::STATUS_LINE, width);
        buf.move_str(1, &self.last_time, colors::STATUS_LINE);

        write_line_to_terminal(
            terminal,
            self.bounds.a.x,
            self.bounds.a.y,
            &buf
        );
    }

    fn handle_event(&mut self, _event: &mut Event) {}

    fn state(&self) -> StateFlags {
        self.state
    }

    fn set_state(&mut self, state: StateFlags) {
        self.state = state;
    }
}

// In application:
pub struct MyApp {
    app: Application,
    clock: ClockView,
}

impl MyApp {
    fn idle(&mut self) {
        // Update clock
        self.clock.update();

        // Do other idle tasks
        self.auto_save_check();
        self.check_network();
    }

    fn auto_save_check(&mut self) {
        // Auto-save every 5 minutes
        if self.time_since_save() > Duration::from_secs(300) {
            self.auto_save();
        }
    }

    fn check_network(&mut self) {
        // Poll for network messages
    }
}
```

### Heap Monitor Example

Show available memory:

```rust
pub struct HeapView {
    bounds: Rect,
    state: StateFlags,
}

impl HeapView {
    pub fn update(&mut self) {
        // In Rust, memory info from system
        // (Unlike Pascal's direct heap access)
    }
}

impl View for HeapView {
    // ... implementation

    fn draw(&mut self, terminal: &mut Terminal) {
        // Get memory info
        use sysinfo::{System, SystemExt};
        let mut sys = System::new_all();
        sys.refresh_memory();

        let used = sys.used_memory();
        let total = sys.total_memory();
        let text = format!("Mem: {} / {} MB", used / 1024, total / 1024);

        let mut buf = DrawBuffer::new(self.bounds.width() as usize);
        buf.move_str(0, &text, colors::FRAME_PASSIVE);

        write_line_to_terminal(
            terminal,
            self.bounds.a.x,
            self.bounds.a.y,
            &buf
        );
    }
}
```

### Idle Processing Guidelines

**Do:**
- Update displays (clock, status)
- Check for background events (network, files)
- Auto-save periodically
- Animate background elements

**Don't:**
- Do heavy computation (will make UI sluggish)
- Block for I/O
- Allocate lots of memory
- Take more than a few milliseconds

---

## Context-Sensitive Help

### Help Context System

Every view can have a help context number:

```rust
pub trait View {
    fn help_context(&self) -> u16 {
        0  // Default: no context
    }

    fn set_help_context(&mut self, _context: u16) {
        // Override if view stores help context
    }
}
```

### Setting Help Contexts

```rust
// Define help contexts
pub const HC_NO_CONTEXT: u16 = 0;
pub const HC_FILE_MENU: u16 = 1000;
pub const HC_EDIT_MENU: u16 = 1001;
pub const HC_EDITOR_WINDOW: u16 = 2000;
pub const HC_FILE_DIALOG: u16 = 3000;

// Assign to views
let mut window = Window::new(bounds, "Editor");
window.set_help_context(HC_EDITOR_WINDOW);

let mut dialog = Dialog::new(bounds, "Open File");
dialog.set_help_context(HC_FILE_DIALOG);
```

### Getting Current Help Context

```rust
impl Application {
    pub fn get_help_context(&self) -> u16 {
        // Get focused view's help context
        if let Some(focused) = self.desktop.get_focused_view() {
            focused.help_context()
        } else {
            HC_NO_CONTEXT
        }
    }
}
```

### Help View Example

```rust
pub struct HelpView {
    window: Window,
    help_text: HashMap<u16, String>,
}

impl HelpView {
    pub fn new(context: u16) -> Self {
        let mut help_text = HashMap::new();

        help_text.insert(
            HC_FILE_MENU,
            "File Menu:\n\n\
             New    - Create a new document\n\
             Open   - Open an existing file\n\
             Save   - Save the current file\n\
             Exit   - Close the application".to_string()
        );

        help_text.insert(
            HC_EDITOR_WINDOW,
            "Editor Window:\n\n\
             Type to edit text\n\
             Ctrl+S to save\n\
             Ctrl+Q to close window".to_string()
        );

        let bounds = Rect::new(10, 5, 70, 20);
        let window = Window::new(bounds, "Help");

        Self { window, help_text }
    }

    pub fn show_help(&mut self, context: u16) {
        if let Some(text) = self.help_text.get(&context) {
            // Display help text in window
            self.window.set_title(&format!("Help - Context {}", context));
            // ... add StaticText with help text
        }
    }
}

// Usage:
impl MyApp {
    fn handle_event(&mut self, event: &mut Event) {
        if event.what == EventType::Command && event.command == CM_HELP {
            let context = self.app.get_help_context();
            let mut help = HelpView::new(context);
            help.show_help(context);

            self.app.execute_dialog(help.window);
            event.clear();
        }
    }
}
```

### Integration with Status Line

Status line changes based on help context:

```rust
// Already shown in "Context-Sensitive Status Lines" section
// The context_status_line updates automatically when focused view changes
```

---

## Complete Example: Full Application

```rust
use turbo_vision::prelude::*;
use std::path::PathBuf;

// Commands
pub const CM_NEW: u16 = 100;
pub const CM_OPEN: u16 = 101;
pub const CM_SAVE: u16 = 102;
pub const CM_ABOUT: u16 = 200;

// Help contexts
pub const HC_EDITOR: u16 = 1000;
pub const HC_FILE_DIALOG: u16 = 2000;

pub struct MyApplication {
    terminal: Terminal,
    desktop: Desktop,
    menu_bar: Option<MenuBar>,
    status_line: Option<HintStatusLine>,
    clock: ClockView,
    running: bool,
    current_file: Option<PathBuf>,
}

impl MyApplication {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut terminal = Terminal::new()?;
        let (width, height) = terminal.size()?;

        // Create desktop
        let desktop = Desktop::new(Rect::new(0, 1, width, height - 1));

        // Create menu bar
        let menu_bar = Some(Self::create_menu_bar(width));

        // Create status line
        let status_line = Some(Self::create_status_line(width, height));

        // Create clock
        let clock = ClockView::new(Rect::new(width - 12, height - 1, width - 2, height));

        Ok(Self {
            terminal,
            desktop,
            menu_bar,
            status_line,
            clock,
            running: false,
            current_file: None,
        })
    }

    fn create_menu_bar(width: i16) -> MenuBar {
        MenuBar::new(
            Rect::new(0, 0, width, 1),
            vec![
                MenuBarItem::submenu(
                    "~F~ile",
                    vec![
                        MenuItem::new("~N~ew", KB_CTRL_N, CM_NEW),
                        MenuItem::new("~O~pen...", KB_F3, CM_OPEN),
                        MenuItem::new("~S~ave", KB_F2, CM_SAVE),
                        MenuItem::separator(),
                        MenuItem::new("E~x~it", KB_ALT_X, CM_QUIT),
                    ]
                ),
                MenuBarItem::submenu(
                    "~H~elp",
                    vec![
                        MenuItem::new("~A~bout", KB_NONE, CM_ABOUT),
                    ]
                ),
            ]
        )
    }

    fn create_status_line(width: i16, height: i16) -> HintStatusLine {
        HintStatusLine::new(Rect::new(0, height - 1, width, height))
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.running = true;

        // Create initial window
        self.create_editor_window();

        while self.running {
            // Draw
            self.draw()?;

            // Get event
            let mut event = self.get_event()?;

            // Handle
            self.handle_event(&mut event);
        }

        Ok(())
    }

    fn draw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Clear screen
        self.terminal.clear()?;

        // Draw menu bar
        if let Some(ref mut menu_bar) = self.menu_bar {
            menu_bar.draw(&mut self.terminal);
        }

        // Draw desktop
        self.desktop.draw(&mut self.terminal);

        // Draw status line
        if let Some(ref mut status_line) = self.status_line {
            status_line.draw(&mut self.terminal);
        }

        // Draw clock
        self.clock.draw(&mut self.terminal);

        // Flush
        self.terminal.flush()?;

        Ok(())
    }

    fn get_event(&mut self) -> Result<Event, Box<dyn std::error::Error>> {
        match self.terminal.poll_event(Some(Duration::from_millis(50)))? {
            Some(event) => Ok(event),
            None => {
                self.idle();
                Ok(Event::nothing())
            }
        }
    }

    fn handle_event(&mut self, event: &mut Event) {
        // Menu bar first
        if let Some(ref mut menu_bar) = self.menu_bar {
            menu_bar.handle_event(event);
            if event.what == EventType::Nothing {
                return;
            }
        }

        // Status line
        if let Some(ref mut status_line) = self.status_line {
            status_line.handle_event(event);
            if event.what == EventType::Nothing {
                return;
            }
        }

        // Handle commands
        if event.what == EventType::Command {
            match event.command {
                CM_QUIT => {
                    self.running = false;
                    event.clear();
                    return;
                }
                CM_NEW => {
                    self.new_document();
                    event.clear();
                    return;
                }
                CM_OPEN => {
                    self.open_document();
                    event.clear();
                    return;
                }
                CM_SAVE => {
                    self.save_document();
                    event.clear();
                    return;
                }
                CM_ABOUT => {
                    self.show_about();
                    event.clear();
                    return;
                }
                _ => {}
            }
        }

        // Desktop handles rest
        self.desktop.handle_event(event);
    }

    fn idle(&mut self) {
        // Update clock
        self.clock.update();
    }

    fn create_editor_window(&mut self) {
        let bounds = Rect::new(5, 3, 75, 22);
        let mut window = Window::new(bounds, "Untitled");
        window.set_help_context(HC_EDITOR);

        self.desktop.add(Box::new(window));
    }

    fn new_document(&mut self) {
        self.current_file = None;
        self.create_editor_window();
    }

    fn open_document(&mut self) {
        // Show file dialog
        // ...
    }

    fn save_document(&mut self) {
        // Save current file
        // ...
    }

    fn show_about(&mut self) {
        let dialog = Dialog::new(
            Rect::new(20, 8, 60, 14),
            "About"
        );
        // ... add controls ...

        // Show modally
        // self.execute_dialog(dialog);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = MyApplication::new()?;
    app.run()?;
    Ok(())
}
```

---

## Best Practices

### 1. Initialize in Correct Order

```rust
// ✓ Good - terminal first
let terminal = Terminal::new()?;
let (width, height) = terminal.size()?;
let desktop = Desktop::new(Rect::new(0, 1, width, height - 1));

// ✗ Bad - using hardcoded size
let desktop = Desktop::new(Rect::new(0, 1, 80, 24));
```

### 2. Clean Event Handling

```rust
// ✓ Good - clear handled events
if event.command == CM_SAVE {
    self.save();
    event.clear();  // Mark as handled
}

// ✗ Bad - forgot to clear
if event.command == CM_SAVE {
    self.save();
    // Parent will also see event!
}
```

### 3. Use Builder Pattern

```rust
// ✓ Good - flexible, readable
let app = ApplicationBuilder::new("My App")
    .menu_bar(file_menu)
    .status_line(status_items)
    .build()?;

// vs. monolithic constructor with many parameters
```

### 4. Separate Application State

```rust
// ✓ Good - clear separation
pub struct MyApp {
    app: Application,        // Framework
    documents: Vec<Document>, // App state
    settings: Settings,      // App state
}

// ✗ Bad - mixing concerns
pub struct MyApp {
    terminal: Terminal,
    documents: Vec<Document>,
    menu_bar: Option<MenuBar>,
    // Hard to see what's framework vs app
}
```

### 5. Don't Block in Idle

```rust
// ✓ Good - quick checks
fn idle(&mut self) {
    self.clock.update();           // Fast
    if self.should_auto_save() {   // Fast check
        self.auto_save();          // Fast (async)
    }
}

// ✗ Bad - blocking operations
fn idle(&mut self) {
    self.download_updates();  // Network I/O - blocks!
    self.rebuild_index();     // Heavy compute - slow!
}
```

---

## Pascal vs. Rust Summary

| Concept | Pascal | Rust |
|---------|--------|------|
| **Structure** | `TApplication = object(TProgram)` | `struct Application` |
| **Constructor** | `MyApp.Init` | `Application::new()` |
| **Main Loop** | `MyApp.Run` | `app.run()` |
| **Destructor** | `MyApp.Done` | Automatic `Drop` |
| **Subviews** | `InitDesktop`, `InitMenuBar`, `InitStatusLine` | Constructor or builder |
| **Global** | `Application` variable | Pass `&mut app` |
| **Idle** | Override `Idle` virtual method | Override `idle()` method |
| **Help Context** | `GetHelpCtx` method | `get_help_context()` method |
| **Insert Window** | `Desktop^.Insert(Window)` | `desktop.add(Box::new(window))` |
| **Execute Dialog** | `Desktop^.Execute(Dialog)` | `app.execute_dialog(dialog)` |

---

## Summary

### Key Concepts

1. **Application is the Root** - Owns terminal, desktop, menu bar, status line
2. **Application is a View** - Manages entire screen
3. **Application is a Group** - Contains subviews
4. **Application is Modal** - Receives events first
5. **Three-Stage Lifecycle** - new() → run() → drop()
6. **Customization** - Override initialization, handle events, use idle time
7. **Context System** - Help contexts for different views

### The Application Pattern

```rust
// 1. Create
let mut app = Application::new()?;

// 2. Customize
app.menu_bar = Some(create_menu_bar());
app.status_line = Some(create_status_line());
app.desktop.add(Box::new(create_window()));

// 3. Run
app.run()?;

// 4. Cleanup automatic (Drop)
```

---

## See Also

- **Chapter 7** - Architecture Overview
- **Chapter 8** - Views and Groups
- **Chapter 9** - Event-Driven Programming
- **Chapter 11** - Windows and Dialogs (upcoming)
- **docs/TURBOVISION-DESIGN.md** - Implementation details
- **examples/full_app_demo.rs** - Complete application example
- **examples/menu.rs** - Menu bar examples
- **examples/status_line_demo.rs** - Status line examples

---

The Application object is the foundation of every Turbo Vision program. Master it, and you can build sophisticated terminal applications with ease.
