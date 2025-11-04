# Chapter 14 — Palettes and Color Selection (Rust Edition)

**Version:** 0.2.11
**Updated:** 2025-11-04

This chapter explores Turbo Vision's color palette system, which enables flexible, centralized color management. You'll learn how views obtain their colors through palette mapping, how to customize colors, and how to let users change colors at runtime.

**Prerequisites:** Chapter 8 (Views and Groups)

---

## Table of Contents

1. [Understanding Color Palettes](#understanding-color-palettes)
2. [How Color Mapping Works](#how-color-mapping-works)
3. [Using Default Colors](#using-default-colors)
4. [Changing Colors](#changing-colors)
5. [Extending Palettes](#extending-palettes)
6. [The Color Selection Dialog](#the-color-selection-dialog)
7. [Saving and Restoring Colors](#saving-and-restoring-colors)
8. [Complete Examples](#complete-examples)

---

## Understanding Color Palettes

### What is a Color Palette?

A **color palette** is a mapping system that indirectly specifies colors for views. Instead of hardcoding "yellow on blue" in every view, you use symbolic names like "normal text" that map through a chain of palettes to actual color values.

**Without Palettes:**
```rust
// Bad - hardcoded colors
fn draw(&mut self, terminal: &mut Terminal) {
    buf.move_str(0, "Hello", 0x1E);  // Yellow on blue - hardcoded!
}
```

**With Palettes:**
```rust
// Good - symbolic color reference
fn draw(&mut self, terminal: &mut Terminal) {
    let color = self.get_color(1);  // "Normal text" color
    buf.move_str(0, "Hello", color);
}
```

### Why Use Palettes?

**Benefits:**
1. **Centralized Colors** - Change window colors once, affects all windows
2. **Context Awareness** - Same view looks different in window vs dialog
3. **User Customization** - Users can change colors without recompiling
4. **Consistency** - All windows automatically match

**Example:**
```rust
// All these use "normal text" color from palette
let scroller1 = Scroller::new(bounds1);  // Yellow on blue in window
let scroller2 = Scroller::new(bounds2);  // Black on white in dialog
// Same code, different colors based on container!
```

### Color Attributes

Colors are represented as 8-bit attributes (DOS/VGA text mode):

```
Bit:  7  6 5 4  3 2 1 0
      B  bgbgbg  fgfgfgfg

B = Blink bit
bg = Background color (0-7)
fg = Foreground color (0-15)
```

**In Rust:**
```rust
pub type ColorAttr = u8;

// Helper to create color attribute
pub fn make_color(fg: u8, bg: u8) -> ColorAttr {
    ((bg & 0x07) << 4) | (fg & 0x0F)
}

// Standard colors
pub const BLACK: u8 = 0;
pub const BLUE: u8 = 1;
pub const GREEN: u8 = 2;
pub const CYAN: u8 = 3;
pub const RED: u8 = 4;
pub const MAGENTA: u8 = 5;
pub const BROWN: u8 = 6;
pub const LIGHT_GRAY: u8 = 7;
pub const DARK_GRAY: u8 = 8;
pub const LIGHT_BLUE: u8 = 9;
pub const LIGHT_GREEN: u8 = 10;
pub const LIGHT_CYAN: u8 = 11;
pub const LIGHT_RED: u8 = 12;
pub const LIGHT_MAGENTA: u8 = 13;
pub const YELLOW: u8 = 14;
pub const WHITE: u8 = 15;

// Examples
const YELLOW_ON_BLUE: ColorAttr = make_color(YELLOW, BLUE);  // 0x1E
const WHITE_ON_RED: ColorAttr = make_color(WHITE, RED);      // 0x4F
```

---

## How Color Mapping Works

### The Palette Chain

Colors are mapped through the ownership chain:

```
View (Scroller)
    ↓ palette entry #1 = 6
Owner (Window)
    ↓ palette entry #6 = 13
Owner (Desktop)
    ↓ palette entry #13 = 13 (transparent)
Owner (Application)
    ↓ palette entry #13 = 0x1E
Result: Yellow on Blue
```

### Simple Example: Scroller Palette

```rust
// Scroller has a 2-entry palette
pub const SCROLLER_PALETTE: &[u8] = &[
    6,  // Entry 1: Normal text    → Window entry #6
    7,  // Entry 2: Selected text  → Window entry #7
];

impl View for Scroller {
    fn draw(&mut self, terminal: &mut Terminal) {
        // Get normal text color
        let normal_color = self.get_color(1);  // → maps to entry #1 → 6

        // Get selected text color
        let selected_color = self.get_color(2);  // → maps to entry #2 → 7

        // Use colors
        buf.move_str(0, "Normal", normal_color);
        buf.move_str(10, "Selected", selected_color);
    }
}
```

### The get_color() Method

```rust
pub trait View {
    /// Get palette for this view
    fn get_palette(&self) -> &[u8] {
        &[]  // Default: empty palette
    }

    /// Map palette entry to color attribute
    fn get_color(&self, entry: usize) -> ColorAttr {
        if entry == 0 || entry > self.get_palette().len() {
            return ERROR_COLOR;  // Blinking white on red
        }

        // Get index from this view's palette
        let mut index = self.get_palette()[entry - 1] as usize;

        // Map through owner chain
        let mut current = self.owner();
        while let Some(owner) = current {
            let palette = owner.get_palette();
            if palette.is_empty() {
                // Transparent palette - pass through
                current = owner.owner();
            } else if index > 0 && index <= palette.len() {
                // Map to next level
                index = palette[index - 1] as usize;
                current = owner.owner();
            } else {
                // Invalid index
                return ERROR_COLOR;
            }
        }

        // Final index is the color attribute
        index as ColorAttr
    }
}

const ERROR_COLOR: ColorAttr = 0x4F;  // Blinking white on red
```

### Example: Tracing a Color Lookup

```rust
// Setup
let app = Application::new();           // Has color palette
let desktop = Desktop::new(...);        // Transparent palette
let window = Window::new(...);          // Blue window palette
let scroller = Scroller::new(...);      // Scroller palette

// Insert chain
app.add(desktop);
desktop.add(window);
window.add(scroller);

// Get color for scroller normal text
let color = scroller.get_color(1);

// Trace:
// 1. scroller.get_palette()[0] = 6
// 2. window.get_palette()[5] = 13  (entry #6, zero-indexed)
// 3. desktop.get_palette() is empty (transparent)
// 4. app.get_palette()[12] = 0x1E  (entry #13, zero-indexed)
// Result: 0x1E (yellow on blue)
```

---

## Using Default Colors

### Standard Palettes

Turbo Vision defines standard palettes for common view types.

#### Application Palette (Color)

```rust
pub const APP_COLOR_PALETTE: &[u8] = &[
    // Desktop (1-3)
    0x71,  // 1:  Background
    0x70,  // 2:  Normal text
    0x78,  // 3:  Selected text

    // Menu bar (4-9)
    0x74,  // 4:  Normal
    0x7F,  // 5:  Disabled
    0x70,  // 6:  Shortcut
    0x7E,  // 7:  Selected
    0x7A,  // 8:  Selected disabled
    0x7F,  // 9:  Shortcut selected

    // Blue window (10-17)
    0x1E,  // 10: Frame passive
    0x1F,  // 11: Frame active
    0x1A,  // 12: Frame icons
    0x31,  // 13: Scrollbar page
    0x3E,  // 14: Scrollbar controls
    0x1E,  // 15: Normal text
    0x1F,  // 16: Selected text
    0x1A,  // 17: Reserved

    // Dialogs (18-31)
    0x70,  // 18: Frame
    0x7F,  // 19: Frame icons
    0x7A,  // 20: Scrollbar page
    0x7E,  // 21: Scrollbar controls
    0x70,  // 22: Normal text
    0x7F,  // 23: Selected text
    0x7A,  // 24: Button normal
    0x7E,  // 25: Button default
    0x7F,  // 26: Button selected
    0x78,  // 27: Button disabled
    0x74,  // 28: Button shortcut
    0x7E,  // 29: Input normal
    0x7F,  // 30: Input selected
    0x70,  // 31: Label

    // More entries...
];

// Black & White palette
pub const APP_BW_PALETTE: &[u8] = &[
    // All entries are combinations of 0x07, 0x0F, 0x70, 0x7F
    // ...
];

// Monochrome palette
pub const APP_MONO_PALETTE: &[u8] = &[
    // All entries use monochrome attributes
    // ...
];
```

#### Window Palette

```rust
// Blue window palette
pub const BLUE_WINDOW_PALETTE: &[u8] = &[
    0x08,  // 1: Frame passive    → App entry #8
    0x09,  // 2: Frame active     → App entry #9
    0x0A,  // 3: Frame icons      → App entry #10
    0x0B,  // 4: Scrollbar page   → App entry #11
    0x0C,  // 5: Scrollbar controls → App entry #12
    0x0D,  // 6: Normal text      → App entry #13
    0x0E,  // 7: Selected text    → App entry #14
    0x0F,  // 8: Reserved         → App entry #15
];

// Gray window palette
pub const GRAY_WINDOW_PALETTE: &[u8] = &[
    0x10,  // 1: Frame passive    → App entry #16
    0x11,  // 2: Frame active     → App entry #17
    // ...
];

// Cyan window palette
pub const CYAN_WINDOW_PALETTE: &[u8] = &[
    0x20,  // 1: Frame passive    → App entry #32
    0x21,  // 2: Frame active     → App entry #33
    // ...
];
```

#### Dialog Palette

```rust
pub const DIALOG_PALETTE: &[u8] = &[
    0x20,  // 1: Frame            → App entry #32
    0x21,  // 2: Frame icons      → App entry #33
    0x22,  // 3: Scrollbar page   → App entry #34
    0x23,  // 4: Scrollbar controls → App entry #35
    0x24,  // 5: Normal text      → App entry #36
    0x25,  // 6: Selected text    → App entry #37
    0x26,  // 7: Button normal    → App entry #38
    0x27,  // 8: Button default   → App entry #39
    0x28,  // 9: Button selected  → App entry #40
    0x29,  // 10: Button disabled → App entry #41
    0x2A,  // 11: Button shortcut → App entry #42
    0x2B,  // 12: Input normal    → App entry #43
    0x2C,  // 13: Input selected  → App entry #44
    0x2D,  // 14: Label           → App entry #45
];
```

### Getting Palettes in Views

```rust
impl View for Window {
    fn get_palette(&self) -> &[u8] {
        match self.palette_type {
            WindowPalette::Blue => BLUE_WINDOW_PALETTE,
            WindowPalette::Gray => GRAY_WINDOW_PALETTE,
            WindowPalette::Cyan => CYAN_WINDOW_PALETTE,
        }
    }
}

impl View for Dialog {
    fn get_palette(&self) -> &[u8] {
        DIALOG_PALETTE
    }
}

impl View for Application {
    fn get_palette(&self) -> &[u8] {
        match self.palette_type {
            AppPalette::Color => APP_COLOR_PALETTE,
            AppPalette::BlackWhite => APP_BW_PALETTE,
            AppPalette::Monochrome => APP_MONO_PALETTE,
        }
    }
}
```

### Using Colors in Draw Methods

```rust
impl View for MyView {
    fn draw(&mut self, terminal: &mut Terminal) {
        // Get colors from palette
        let normal_color = self.get_color(1);
        let selected_color = self.get_color(2);
        let disabled_color = self.get_color(3);

        let width = self.bounds.width() as usize;
        let mut buf = DrawBuffer::new(width);

        // Use colors
        if self.is_focused() {
            buf.move_str(0, "Focused", selected_color);
        } else if self.is_disabled() {
            buf.move_str(0, "Disabled", disabled_color);
        } else {
            buf.move_str(0, "Normal", normal_color);
        }

        write_line_to_terminal(terminal, self.bounds.a.x, self.bounds.a.y, &buf);
    }
}
```

---

## Changing Colors

### Changing Application Palette

**Easiest approach:** Modify the application's palette to change all views:

```rust
impl Application {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut app = Self {
            // ... initialization
            palette_type: AppPalette::Color,
            custom_palette: None,
        };

        Ok(app)
    }

    pub fn set_custom_palette(&mut self, palette: Vec<u8>) {
        self.custom_palette = Some(palette);
    }
}

impl View for Application {
    fn get_palette(&self) -> &[u8] {
        if let Some(ref custom) = self.custom_palette {
            custom
        } else {
            match self.palette_type {
                AppPalette::Color => APP_COLOR_PALETTE,
                AppPalette::BlackWhite => APP_BW_PALETTE,
                AppPalette::Monochrome => APP_MONO_PALETTE,
            }
        }
    }
}

// Usage
let mut app = Application::new()?;

// Change window normal text to white on green
let mut palette = APP_COLOR_PALETTE.to_vec();
palette[12] = make_color(WHITE, GREEN);  // Entry #13 (index 12)
app.set_custom_palette(palette);
```

### Changing Individual View Palette

Override `get_palette()` for a specific view type:

```rust
pub struct MyScroller {
    scroller: Scroller,
}

impl View for MyScroller {
    fn get_palette(&self) -> &[u8] {
        // Custom palette for this scroller
        static CUSTOM_PALETTE: &[u8] = &[
            1,  // Normal text uses frame color
            7,  // Selected text stays the same
        ];
        CUSTOM_PALETTE
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        self.scroller.draw(terminal)
    }

    // ... other View methods
}
```

### Creating Palette Variants

```rust
pub enum ScrollerPalette {
    Normal,
    Inverted,
    HighContrast,
}

pub struct ConfigurableScroller {
    scroller: Scroller,
    palette_variant: ScrollerPalette,
}

impl View for ConfigurableScroller {
    fn get_palette(&self) -> &[u8] {
        match self.palette_variant {
            ScrollerPalette::Normal => &[6, 7],
            ScrollerPalette::Inverted => &[7, 6],  // Swap colors
            ScrollerPalette::HighContrast => &[1, 2],  // Use frame colors
        }
    }

    // ... other methods
}
```

---

## Extending Palettes

### When to Extend Palettes

Extend palettes when creating new view types that need additional colors:

**Scenario:** Custom scroller with three text styles (normal, selected, emphasized)

### Step 1: Extend View Palette

```rust
pub struct ExtendedScroller {
    scroller: Scroller,
}

impl View for ExtendedScroller {
    fn get_palette(&self) -> &[u8] {
        // Extend scroller palette from 2 to 3 entries
        static PALETTE: &[u8] = &[
            6,  // 1: Normal text
            7,  // 2: Selected text
            4,  // 3: Emphasized text → Window scrollbar controls
        ];
        PALETTE
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        let normal = self.get_color(1);
        let selected = self.get_color(2);
        let emphasized = self.get_color(3);  // New color!

        // Use all three colors
        // ...
    }
}
```

### Step 2: Extend Owner Palette (If Needed)

If pointing to a **new** color (not existing in owner):

```rust
pub struct ExtendedWindow {
    window: Window,
}

impl View for ExtendedWindow {
    fn get_palette(&self) -> &[u8] {
        // Extend window palette from 8 to 9 entries
        static PALETTE: &[u8] = &[
            0x08,  // 1: Frame passive
            0x09,  // 2: Frame active
            0x0A,  // 3: Frame icons
            0x0B,  // 4: Scrollbar page
            0x0C,  // 5: Scrollbar controls
            0x0D,  // 6: Normal text
            0x0E,  // 7: Selected text
            0x0F,  // 8: Reserved
            0x40,  // 9: NEW - Emphasized → App entry #64
        ];
        PALETTE
    }
}
```

### Step 3: Extend Application Palette

```rust
impl Application {
    fn get_extended_palette() -> Vec<u8> {
        let mut palette = APP_COLOR_PALETTE.to_vec();

        // Add new entry #64 (index 63)
        palette.push(make_color(LIGHT_RED, BLUE));  // Red on blue

        palette
    }
}
```

### Complete Extension Example

```rust
// New view with 3 palette entries
pub struct TriColorView {
    bounds: Rect,
    state: StateFlags,
}

impl View for TriColorView {
    fn get_palette(&self) -> &[u8] {
        &[1, 2, 9]  // Uses new 9th entry in window
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        let color1 = self.get_color(1);  // Frame passive
        let color2 = self.get_color(2);  // Frame active
        let color3 = self.get_color(3);  // NEW emphasized

        let mut buf = DrawBuffer::new(self.bounds.width() as usize);
        buf.move_str(0, "Normal ", color1);
        buf.move_str(7, "Active ", color2);
        buf.move_str(14, "Emphasized", color3);

        write_line_to_terminal(terminal, self.bounds.a.x, self.bounds.a.y, &buf);
    }

    // ... other methods
}

// Extended window with 9 palette entries
pub struct ExtendedWindow {
    interior: Group,
    // ...
}

impl View for ExtendedWindow {
    fn get_palette(&self) -> &[u8] {
        &[8, 9, 10, 11, 12, 13, 14, 15, 64]  // Added entry 64
    }
}

// Extended application with entry #64
impl Application {
    pub fn new_extended() -> Self {
        let mut app = Self::new().unwrap();

        let mut palette = APP_COLOR_PALETTE.to_vec();
        palette.resize(64, 0);  // Extend to 64 entries
        palette[63] = make_color(LIGHT_RED, BLUE);  // Entry #64

        app.set_custom_palette(palette);
        app
    }
}
```

---

## The Color Selection Dialog

### Overview

The `ColorDialog` allows users to customize application colors at runtime.

```rust
pub struct ColorDialog {
    dialog: Dialog,
    groups: Vec<ColorGroup>,
    palette: Vec<u8>,
}

pub struct ColorGroup {
    pub name: String,
    pub items: Vec<ColorItem>,
}

pub struct ColorItem {
    pub name: String,
    pub index: usize,  // Palette index
}
```

### Creating a Color Dialog

```rust
impl ColorDialog {
    pub fn new(groups: Vec<ColorGroup>, palette: Vec<u8>) -> Self {
        let bounds = Rect::new(5, 3, 75, 20);
        let mut dialog = Dialog::new(bounds, "Colors");

        // Add group list
        let group_list = ListBox::new(
            Rect::new(2, 2, 20, 15),
            groups.iter().map(|g| g.name.clone()).collect(),
        );
        dialog.add(Box::new(group_list));

        // Add item list
        let item_list = ListBox::new(
            Rect::new(22, 2, 45, 15),
            Vec::new(),
        );
        dialog.add(Box::new(item_list));

        // Add color selector
        let color_selector = ColorSelector::new(
            Rect::new(47, 2, 68, 10),
        );
        dialog.add(Box::new(color_selector));

        // Add buttons
        dialog.add(Box::new(Button::new(
            Rect::new(25, 16, 35, 18),
            "~O~K",
            CM_OK,
            true,
        )));

        dialog.add(Box::new(Button::new(
            Rect::new(37, 16, 47, 18),
            "~C~ancel",
            CM_CANCEL,
            false,
        )));

        Self { dialog, groups, palette }
    }

    pub fn execute(&mut self, app: &mut Application) -> Option<Vec<u8>> {
        let result = self.dialog.execute(app);

        if result == CM_OK {
            Some(self.palette.clone())
        } else {
            None
        }
    }
}
```

### Color Groups and Items

```rust
// Helper to create color groups
pub fn desktop_color_group() -> ColorGroup {
    ColorGroup {
        name: "Desktop".to_string(),
        items: vec![
            ColorItem { name: "Background".to_string(), index: 1 },
            ColorItem { name: "Normal Text".to_string(), index: 2 },
            ColorItem { name: "Selected Text".to_string(), index: 3 },
        ],
    }
}

pub fn menu_color_group() -> ColorGroup {
    ColorGroup {
        name: "Menus".to_string(),
        items: vec![
            ColorItem { name: "Normal".to_string(), index: 4 },
            ColorItem { name: "Disabled".to_string(), index: 5 },
            ColorItem { name: "Shortcut".to_string(), index: 6 },
            ColorItem { name: "Selected".to_string(), index: 7 },
            ColorItem { name: "Selected Disabled".to_string(), index: 8 },
            ColorItem { name: "Shortcut Selected".to_string(), index: 9 },
        ],
    }
}

pub fn window_color_group() -> ColorGroup {
    ColorGroup {
        name: "Blue Window".to_string(),
        items: vec![
            ColorItem { name: "Frame Passive".to_string(), index: 10 },
            ColorItem { name: "Frame Active".to_string(), index: 11 },
            ColorItem { name: "Frame Icons".to_string(), index: 12 },
            ColorItem { name: "Scrollbar Page".to_string(), index: 13 },
            ColorItem { name: "Scrollbar Controls".to_string(), index: 14 },
            ColorItem { name: "Normal Text".to_string(), index: 15 },
            ColorItem { name: "Selected Text".to_string(), index: 16 },
        ],
    }
}

pub fn dialog_color_group() -> ColorGroup {
    ColorGroup {
        name: "Dialogs".to_string(),
        items: vec![
            ColorItem { name: "Frame".to_string(), index: 32 },
            ColorItem { name: "Frame Icons".to_string(), index: 33 },
            ColorItem { name: "Normal Text".to_string(), index: 36 },
            ColorItem { name: "Selected Text".to_string(), index: 37 },
            ColorItem { name: "Button Normal".to_string(), index: 38 },
            ColorItem { name: "Button Default".to_string(), index: 39 },
            ColorItem { name: "Button Selected".to_string(), index: 40 },
            ColorItem { name: "Input Normal".to_string(), index: 43 },
            ColorItem { name: "Input Selected".to_string(), index: 44 },
        ],
    }
}
```

### Using the Color Dialog

```rust
impl MyApp {
    pub fn show_color_dialog(&mut self) {
        // Get current palette
        let current_palette = self.app.get_palette().to_vec();

        // Create color groups
        let groups = vec![
            desktop_color_group(),
            menu_color_group(),
            window_color_group(),
            dialog_color_group(),
        ];

        // Create and execute dialog
        let mut color_dialog = ColorDialog::new(groups, current_palette);

        if let Some(new_palette) = color_dialog.execute(&mut self.app) {
            // Apply new palette
            self.app.set_custom_palette(new_palette);

            // Redraw everything
            self.app.redraw();
        }
    }
}
```

### Color Selector Widget

```rust
pub struct ColorSelector {
    bounds: Rect,
    selected_fg: u8,
    selected_bg: u8,
    state: StateFlags,
}

impl ColorSelector {
    pub fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            selected_fg: YELLOW,
            selected_bg: BLUE,
            state: SF_VISIBLE | SF_SELECTABLE,
        }
    }

    pub fn get_color(&self) -> ColorAttr {
        make_color(self.selected_fg, self.selected_bg)
    }
}

impl View for ColorSelector {
    fn draw(&mut self, terminal: &mut Terminal) {
        let width = self.bounds.width() as usize;
        let height = self.bounds.height() as usize;

        // Draw foreground color palette (16 colors)
        for i in 0..16 {
            let row = i / 8;
            let col = i % 8;
            let color = make_color(i as u8, BLACK);

            let mut buf = DrawBuffer::new(2);
            buf.move_str(0, "██", color);

            write_line_to_terminal(
                terminal,
                self.bounds.a.x + (col * 2) as i16,
                self.bounds.a.y + row as i16,
                &buf,
            );
        }

        // Draw background color palette (8 colors)
        for i in 0..8 {
            let color = make_color(WHITE, i as u8);

            let mut buf = DrawBuffer::new(2);
            buf.move_str(0, "  ", color);

            write_line_to_terminal(
                terminal,
                self.bounds.a.x + (i * 2) as i16,
                self.bounds.a.y + 3,
                &buf,
            );
        }

        // Draw preview
        let preview_color = self.get_color();
        let mut buf = DrawBuffer::new(width);
        buf.move_str(0, "Sample Text", preview_color);

        write_line_to_terminal(
            terminal,
            self.bounds.a.x,
            self.bounds.a.y + 5,
            &buf,
        );
    }

    fn handle_event(&mut self, event: &mut Event) {
        if event.what == EventType::MouseDown {
            let local = Point {
                x: event.mouse.position.x - self.bounds.a.x,
                y: event.mouse.position.y - self.bounds.a.y,
            };

            // Click on foreground colors
            if local.y < 2 {
                let index = (local.y * 8 + local.x / 2) as u8;
                if index < 16 {
                    self.selected_fg = index;
                    event.clear();
                }
            }
            // Click on background colors
            else if local.y == 3 {
                let index = (local.x / 2) as u8;
                if index < 8 {
                    self.selected_bg = index;
                    event.clear();
                }
            }
        }
    }

    // ... other View methods
}
```

---

## Saving and Restoring Colors

### Serializing Palettes

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PaletteConfig {
    pub palette: Vec<u8>,
    pub name: String,
}

impl PaletteConfig {
    pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&json)?;
        Ok(config)
    }
}
```

### Saving User Preferences

```rust
impl Application {
    pub fn save_palette(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let palette = self.get_palette().to_vec();

        let config = PaletteConfig {
            palette,
            name: "User Palette".to_string(),
        };

        config.save(path)
    }

    pub fn load_palette(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let config = PaletteConfig::load(path)?;
        self.set_custom_palette(config.palette);
        Ok(())
    }
}

// Usage
impl MyApp {
    pub fn save_settings(&self) {
        let palette_file = dirs::config_dir()
            .unwrap()
            .join("myapp")
            .join("palette.json");

        if let Err(e) = self.app.save_palette(&palette_file) {
            eprintln!("Failed to save palette: {}", e);
        }
    }

    pub fn load_settings(&mut self) {
        let palette_file = dirs::config_dir()
            .unwrap()
            .join("myapp")
            .join("palette.json");

        if palette_file.exists() {
            if let Err(e) = self.app.load_palette(&palette_file) {
                eprintln!("Failed to load palette: {}", e);
            }
        }
    }
}
```

### Palette Presets

```rust
pub struct PalettePresets;

impl PalettePresets {
    pub fn solarized_dark() -> Vec<u8> {
        vec![
            // Desktop
            make_color(12, 0),  // Base0 on Base03
            make_color(14, 0),  // Base1 on Base03
            make_color(11, 0),  // Cyan on Base03

            // Menus
            make_color(14, 0),  // Base1 on Base03
            make_color(10, 0),  // Base01 on Base03
            make_color(9, 0),   // Blue on Base03
            make_color(0, 11),  // Base03 on Cyan

            // ... more entries
        ]
    }

    pub fn monokai() -> Vec<u8> {
        vec![
            make_color(7, 0),   // Light gray on black
            make_color(15, 0),  // White on black
            make_color(11, 0),  // Cyan on black

            // ... more entries
        ]
    }

    pub fn classic_dos() -> Vec<u8> {
        APP_COLOR_PALETTE.to_vec()  // Use default
    }

    pub fn high_contrast() -> Vec<u8> {
        vec![
            make_color(15, 0),  // White on black everywhere
            make_color(15, 0),
            make_color(0, 15),  // Black on white for selection

            // ... more entries
        ]
    }
}

// Usage
impl MyApp {
    pub fn apply_preset(&mut self, preset: &str) {
        let palette = match preset {
            "solarized" => PalettePresets::solarized_dark(),
            "monokai" => PalettePresets::monokai(),
            "dos" => PalettePresets::classic_dos(),
            "contrast" => PalettePresets::high_contrast(),
            _ => return,
        };

        self.app.set_custom_palette(palette);
        self.app.redraw();
    }
}
```

---

## Complete Examples

### Example 1: Custom Application Palette

```rust
use turbo_vision::prelude::*;

pub struct ThemedApp {
    app: Application,
}

impl ThemedApp {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut app = Application::new()?;

        // Create custom palette
        let mut palette = APP_COLOR_PALETTE.to_vec();

        // Change window colors to green theme
        palette[12] = make_color(LIGHT_GREEN, BLACK);   // Normal text
        palette[13] = make_color(WHITE, GREEN);         // Selected text
        palette[14] = make_color(DARK_GRAY, BLACK);     // Frame passive
        palette[15] = make_color(LIGHT_GREEN, BLACK);   // Frame active

        app.set_custom_palette(palette);

        Ok(Self { app })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Create window with green theme
        let window = Window::new(
            Rect::new(10, 5, 70, 20),
            "Green Theme"
        );

        self.app.desktop.add(Box::new(window));
        self.app.run()?;

        Ok(())
    }
}
```

### Example 2: Custom View with Extended Palette

```rust
pub struct StatusBar {
    bounds: Rect,
    message: String,
    status: StatusType,
    state: StateFlags,
}

pub enum StatusType {
    Normal,
    Warning,
    Error,
}

impl StatusBar {
    pub fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            message: String::new(),
            status: StatusType::Normal,
            state: SF_VISIBLE,
        }
    }

    pub fn set_message(&mut self, message: &str, status: StatusType) {
        self.message = message.to_string();
        self.status = status;
    }
}

impl View for StatusBar {
    fn get_palette(&self) -> &[u8] {
        // 3-entry palette: normal, warning, error
        static PALETTE: &[u8] = &[
            2,   // Normal   → Desktop normal text
            64,  // Warning  → New app entry
            65,  // Error    → New app entry
        ];
        PALETTE
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        let color = match self.status {
            StatusType::Normal => self.get_color(1),
            StatusType::Warning => self.get_color(2),
            StatusType::Error => self.get_color(3),
        };

        let width = self.bounds.width() as usize;
        let mut buf = DrawBuffer::new(width);

        buf.move_char(0, ' ', color, width);
        buf.move_str(1, &self.message, color);

        write_line_to_terminal(
            terminal,
            self.bounds.a.x,
            self.bounds.a.y,
            &buf,
        );
    }

    // ... other View methods
}

// Extended application with warning/error colors
impl Application {
    pub fn new_with_status_colors() -> Result<Self, Box<dyn std::error::Error>> {
        let mut app = Self::new()?;

        let mut palette = APP_COLOR_PALETTE.to_vec();
        palette.resize(66, 0);

        palette[63] = make_color(YELLOW, BLACK);        // Warning
        palette[64] = make_color(LIGHT_RED, BLACK);     // Error

        app.set_custom_palette(palette);

        Ok(app)
    }
}
```

### Example 3: Theme Manager

```rust
pub struct ThemeManager {
    themes: HashMap<String, Vec<u8>>,
    current_theme: String,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut themes = HashMap::new();

        // Add built-in themes
        themes.insert("Default".to_string(), APP_COLOR_PALETTE.to_vec());
        themes.insert("Solarized".to_string(), PalettePresets::solarized_dark());
        themes.insert("Monokai".to_string(), PalettePresets::monokai());
        themes.insert("High Contrast".to_string(), PalettePresets::high_contrast());

        Self {
            themes,
            current_theme: "Default".to_string(),
        }
    }

    pub fn get_theme(&self, name: &str) -> Option<&Vec<u8>> {
        self.themes.get(name)
    }

    pub fn apply_theme(&mut self, name: &str, app: &mut Application) -> bool {
        if let Some(palette) = self.themes.get(name) {
            app.set_custom_palette(palette.clone());
            self.current_theme = name.to_string();
            true
        } else {
            false
        }
    }

    pub fn add_theme(&mut self, name: String, palette: Vec<u8>) {
        self.themes.insert(name, palette);
    }

    pub fn list_themes(&self) -> Vec<&str> {
        self.themes.keys().map(|s| s.as_str()).collect()
    }

    pub fn current_theme(&self) -> &str {
        &self.current_theme
    }
}

// Usage in application
pub struct MyApp {
    app: Application,
    theme_manager: ThemeManager,
}

impl MyApp {
    pub fn show_theme_selector(&mut self) {
        let themes = self.theme_manager.list_themes();

        let mut dialog = Dialog::new(
            Rect::new(20, 8, 60, 18),
            "Select Theme"
        );

        let theme_list = ListBox::new(
            Rect::new(2, 2, 36, 8),
            themes.iter().map(|s| s.to_string()).collect(),
        );
        dialog.add(Box::new(theme_list));

        dialog.add(Box::new(Button::new(
            Rect::new(10, 9, 20, 11),
            "~O~K",
            CM_OK,
            true,
        )));

        let result = dialog.execute(&mut self.app);

        if result == CM_OK {
            // Get selected theme
            if let Some(selected) = theme_list.get_selected() {
                let theme_name = themes[selected];
                self.theme_manager.apply_theme(theme_name, &mut self.app);
                self.app.redraw();
            }
        }
    }
}
```

---

## Best Practices

### 1. Use Symbolic Colors, Not Hardcoded Values

```rust
// ✓ Good - symbolic reference
let color = self.get_color(1);  // "Normal text" color

// ✗ Bad - hardcoded
let color = 0x1E;  // What does this mean?
```

### 2. Define Palette Entry Constants

```rust
// ✓ Good - named constants
pub const SCROLLER_NORMAL: usize = 1;
pub const SCROLLER_SELECTED: usize = 2;

impl View for Scroller {
    fn draw(&mut self, terminal: &mut Terminal) {
        let normal = self.get_color(SCROLLER_NORMAL);
        let selected = self.get_color(SCROLLER_SELECTED);
    }
}

// ✗ Bad - magic numbers
let normal = self.get_color(1);  // What's 1?
let selected = self.get_color(2);  // What's 2?
```

### 3. Extend Palettes Carefully

```rust
// ✓ Good - check bounds
if entry > 0 && entry <= self.get_palette().len() {
    // Safe to use
}

// ✗ Bad - assume bounds
let index = self.get_palette()[entry - 1];  // Might panic!
```

### 4. Document Palette Layout

```rust
impl View for MyView {
    fn get_palette(&self) -> &[u8] {
        // Palette layout:
        // 1 = Normal text
        // 2 = Selected text
        // 3 = Disabled text
        // 4 = Emphasized text
        static PALETTE: &[u8] = &[6, 7, 8, 9];
        PALETTE
    }
}
```

### 5. Provide Palette Presets

```rust
// ✓ Good - give users choices
impl MyApp {
    pub fn get_preset_themes() -> Vec<(&'static str, Vec<u8>)> {
        vec![
            ("Default", APP_COLOR_PALETTE.to_vec()),
            ("Dark", PalettePresets::solarized_dark()),
            ("Light", PalettePresets::solarized_light()),
            ("High Contrast", PalettePresets::high_contrast()),
        ]
    }
}
```

### 6. Save User Preferences

```rust
// ✓ Good - persist user choices
impl MyApp {
    pub fn on_exit(&self) {
        self.save_settings();  // Saves palette
    }

    pub fn on_startup(&mut self) {
        self.load_settings();  // Restores palette
    }
}
```

---

## Pascal vs. Rust Summary

| Concept | Pascal | Rust |
|---------|--------|------|
| **Palette Type** | `String` (array of bytes) | `&[u8]` slice |
| **Get Palette** | `function GetPalette: PPalette` | `fn get_palette(&self) -> &[u8]` |
| **Get Color** | `function GetColor(Color: Byte): Byte` | `fn get_color(&self, entry: usize) -> ColorAttr` |
| **Palette Constant** | `const CPalette = #6#7#8` | `static PALETTE: &[u8] = &[6, 7, 8]` |
| **Color Attribute** | `Byte` | `ColorAttr` (type alias for `u8`) |
| **Application Palette** | `AppPalette: Integer` (index) | `palette_type: AppPalette` (enum) |
| **Window Palette** | `Palette: Integer` (wpBlue, etc.) | `palette_type: WindowPalette` (enum) |
| **Extend Palette** | `CPalette + #9` (concatenation) | `vec![6, 7, 8, 9]` or `&[6, 7, 8, 9]` |
| **Color Dialog** | `TColorDialog` object | `ColorDialog` struct |
| **Save Colors** | `SaveIndexes(Stream)` | `save_palette(path)` with serde |

---

## Summary

### Key Concepts

1. **Palette Mapping** - Colors map through ownership chain to application
2. **Symbolic Colors** - Use palette entries, not hardcoded attributes
3. **Centralized Management** - Change application palette affects all views
4. **Context Awareness** - Same view different colors based on container
5. **User Customization** - Color dialog for runtime changes
6. **Extensibility** - Add palette entries for new view types
7. **Persistence** - Save/restore user color preferences

### The Palette Pattern

```rust
// 1. Define palette for view
impl View for MyView {
    fn get_palette(&self) -> &[u8] {
        static PALETTE: &[u8] = &[6, 7, 8];  // Map to owner entries
        PALETTE
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        // 2. Get colors from palette
        let normal = self.get_color(1);
        let selected = self.get_color(2);
        let disabled = self.get_color(3);

        // 3. Use colors
        buf.move_str(0, "Text", normal);
    }
}

// 4. Color maps through chain
// MyView entry #1 → Window entry #6 → App entry #13 → 0x1E
```

---

## See Also

- **Chapter 8** - Views and Groups (Drawing and view hierarchy)
- **Chapter 11** - Windows and Dialogs (Window palettes)
- **Chapter 10** - Application Objects (Application palette)
- **docs/TURBOVISION-DESIGN.md** - Implementation details
- **examples/colors_demo.rs** - Color examples

---

Palettes provide a flexible, centralized color management system. Use them to create consistent, customizable, professional-looking terminal applications.
