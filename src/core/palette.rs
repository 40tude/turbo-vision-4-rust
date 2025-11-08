// (C) 2025 - Enzo Lombardi

//! Color palette - 16-color palette definitions and attribute management.
// Color Palette
// Color definitions, attributes, and palette management matching Borland Turbo Vision
use crossterm::style::Color;

/// 16-color palette matching Turbo Vision
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TvColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}

impl TvColor {
    /// Converts TvColor to crossterm Color with RGB values
    pub fn to_crossterm(self) -> Color {
        match self {
            TvColor::Black => Color::Rgb { r: 0, g: 0, b: 0 },
            TvColor::Blue => Color::Rgb { r: 0, g: 0, b: 170 },
            TvColor::Green => Color::Rgb { r: 0, g: 170, b: 0 },
            TvColor::Cyan => Color::Rgb { r: 0, g: 170, b: 170 },
            TvColor::Red => Color::Rgb { r: 170, g: 0, b: 0 },
            TvColor::Magenta => Color::Rgb { r: 170, g: 0, b: 170 },
            TvColor::Brown => Color::Rgb { r: 170, g: 85, b: 0 },
            TvColor::LightGray => Color::Rgb { r: 170, g: 170, b: 170 },
            TvColor::DarkGray => Color::Rgb { r: 85, g: 85, b: 85 },
            TvColor::LightBlue => Color::Rgb { r: 85, g: 85, b: 255 },
            TvColor::LightGreen => Color::Rgb { r: 85, g: 255, b: 85 },
            TvColor::LightCyan => Color::Rgb { r: 85, g: 255, b: 255 },
            TvColor::LightRed => Color::Rgb { r: 255, g: 85, b: 85 },
            TvColor::LightMagenta => Color::Rgb { r: 255, g: 85, b: 255 },
            TvColor::Yellow => Color::Rgb { r: 255, g: 255, b: 85 },
            TvColor::White => Color::Rgb { r: 255, g: 255, b: 255 },
        }
    }

    /// Gets the RGB components of this color
    pub fn to_rgb(self) -> (u8, u8, u8) {
        match self {
            TvColor::Black => (0, 0, 0),
            TvColor::Blue => (0, 0, 170),
            TvColor::Green => (0, 170, 0),
            TvColor::Cyan => (0, 170, 170),
            TvColor::Red => (170, 0, 0),
            TvColor::Magenta => (170, 0, 170),
            TvColor::Brown => (170, 85, 0),
            TvColor::LightGray => (170, 170, 170),
            TvColor::DarkGray => (85, 85, 85),
            TvColor::LightBlue => (85, 85, 255),
            TvColor::LightGreen => (85, 255, 85),
            TvColor::LightCyan => (85, 255, 255),
            TvColor::LightRed => (255, 85, 85),
            TvColor::LightMagenta => (255, 85, 255),
            TvColor::Yellow => (255, 255, 85),
            TvColor::White => (255, 255, 255),
        }
    }

    /// Creates a TvColor from RGB values by finding the closest match
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        // Find closest color in the palette
        let all_colors = [
            TvColor::Black, TvColor::Blue, TvColor::Green, TvColor::Cyan,
            TvColor::Red, TvColor::Magenta, TvColor::Brown, TvColor::LightGray,
            TvColor::DarkGray, TvColor::LightBlue, TvColor::LightGreen, TvColor::LightCyan,
            TvColor::LightRed, TvColor::LightMagenta, TvColor::Yellow, TvColor::White,
        ];

        let mut best_color = TvColor::Black;
        let mut best_distance = u32::MAX;

        for &color in &all_colors {
            let (cr, cg, cb) = color.to_rgb();
            let distance = (r as i32 - cr as i32).pow(2) as u32 +
                          (g as i32 - cg as i32).pow(2) as u32 +
                          (b as i32 - cb as i32).pow(2) as u32;
            if distance < best_distance {
                best_distance = distance;
                best_color = color;
            }
        }

        best_color
    }

    pub fn from_u8(n: u8) -> Self {
        match n & 0x0F {
            0 => TvColor::Black,
            1 => TvColor::Blue,
            2 => TvColor::Green,
            3 => TvColor::Cyan,
            4 => TvColor::Red,
            5 => TvColor::Magenta,
            6 => TvColor::Brown,
            7 => TvColor::LightGray,
            8 => TvColor::DarkGray,
            9 => TvColor::LightBlue,
            10 => TvColor::LightGreen,
            11 => TvColor::LightCyan,
            12 => TvColor::LightRed,
            13 => TvColor::LightMagenta,
            14 => TvColor::Yellow,
            15 => TvColor::White,
            _ => TvColor::LightGray,
        }
    }
}

/// Text attributes (foreground and background colors)
///
/// # Examples
///
/// ```
/// use turbo_vision::core::palette::{Attr, TvColor, colors};
///
/// // Create custom attribute
/// let attr = Attr::new(TvColor::White, TvColor::Blue);
/// assert_eq!(attr.fg, TvColor::White);
/// assert_eq!(attr.bg, TvColor::Blue);
///
/// // Use predefined colors from colors module
/// let button_attr = colors::BUTTON_NORMAL;
/// assert_eq!(button_attr.fg, TvColor::Black);
/// assert_eq!(button_attr.bg, TvColor::Green);
///
/// // Convert to/from byte representation
/// let byte = attr.to_u8();
/// let restored = Attr::from_u8(byte);
/// assert_eq!(attr, restored);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Attr {
    pub fg: TvColor,
    pub bg: TvColor,
}

impl Attr {
    pub const fn new(fg: TvColor, bg: TvColor) -> Self {
        Self { fg, bg }
    }

    pub fn from_u8(byte: u8) -> Self {
        Self {
            fg: TvColor::from_u8(byte & 0x0F),
            bg: TvColor::from_u8((byte >> 4) & 0x0F),
        }
    }

    pub fn to_u8(self) -> u8 {
        (self.fg as u8) | ((self.bg as u8) << 4)
    }

    /// Creates a darkened version of this attribute (for semi-transparent shadows)
    /// Reduces RGB values by the given factor (0.0 = black, 1.0 = unchanged)
    /// Default shadow factor is 0.5 (50% darker)
    pub fn darken(&self, factor: f32) -> Self {
        let darken_color = |color: TvColor| -> TvColor {
            let (r, g, b) = color.to_rgb();
            let new_r = ((r as f32) * factor).min(255.0) as u8;
            let new_g = ((g as f32) * factor).min(255.0) as u8;
            let new_b = ((b as f32) * factor).min(255.0) as u8;
            TvColor::from_rgb(new_r, new_g, new_b)
        };

        Self {
            fg: darken_color(self.fg),
            bg: darken_color(self.bg),
        }
    }
}

/// Standard color pairs for UI elements
pub mod colors {
    use super::*;

    pub const NORMAL: Attr = Attr::new(TvColor::LightGray, TvColor::Blue);
    pub const HIGHLIGHTED: Attr = Attr::new(TvColor::Yellow, TvColor::Blue);
    pub const SELECTED: Attr = Attr::new(TvColor::White, TvColor::Cyan);
    pub const DISABLED: Attr = Attr::new(TvColor::DarkGray, TvColor::Blue);

    pub const MENU_NORMAL: Attr = Attr::new(TvColor::Black, TvColor::LightGray);
    pub const MENU_SELECTED: Attr = Attr::new(TvColor::White, TvColor::Green);
    pub const MENU_DISABLED: Attr = Attr::new(TvColor::DarkGray, TvColor::LightGray);
    pub const MENU_SHORTCUT: Attr = Attr::new(TvColor::Red, TvColor::LightGray);

    pub const DIALOG_NORMAL: Attr = Attr::new(TvColor::Black, TvColor::LightGray);     // cpDialog[0] = 0x70 interior
    pub const DIALOG_FRAME: Attr = Attr::new(TvColor::White, TvColor::LightGray);      // cpDialog[1] = 0x7F
    pub const DIALOG_FRAME_ACTIVE: Attr = Attr::new(TvColor::White, TvColor::LightGray); // cpDialog[1] = 0x7F
    pub const DIALOG_TITLE: Attr = Attr::new(TvColor::White, TvColor::LightGray);      // cpDialog[1] = 0x7F
    pub const DIALOG_SHORTCUT: Attr = Attr::new(TvColor::Red, TvColor::LightGray);     // Shortcut letters in dialogs

    pub const BUTTON_NORMAL: Attr = Attr::new(TvColor::Black, TvColor::Green);      // Inactive but focusable
    pub const BUTTON_DEFAULT: Attr = Attr::new(TvColor::LightGreen, TvColor::Green); // Default but not focused
    pub const BUTTON_SELECTED: Attr = Attr::new(TvColor::White, TvColor::Green);    // Focused
    pub const BUTTON_DISABLED: Attr = Attr::new(TvColor::DarkGray, TvColor::Green); // Disabled (not implemented yet)
    pub const BUTTON_SHORTCUT: Attr = Attr::new(TvColor::Yellow, TvColor::Green);   // Shortcut letters
    pub const BUTTON_SHADOW: Attr = Attr::new(TvColor::LightGray, TvColor::DarkGray);

    pub const STATUS_NORMAL: Attr = Attr::new(TvColor::Black, TvColor::LightGray);
    pub const STATUS_SHORTCUT: Attr = Attr::new(TvColor::Red, TvColor::LightGray);
    pub const STATUS_SELECTED: Attr = Attr::new(TvColor::White, TvColor::Green);
    pub const STATUS_SELECTED_SHORTCUT: Attr = Attr::new(TvColor::Yellow, TvColor::Green);

    // InputLine colors - matching actual C++ rendering (see colors.png)
    // Focused state uses Yellow on Blue (clearly visible in screenshot)
    // Both states use same color per C++ cpInputLine behavior
    pub const INPUT_NORMAL: Attr = Attr::new(TvColor::Yellow, TvColor::Blue);     // Same as focused
    pub const INPUT_FOCUSED: Attr = Attr::new(TvColor::Yellow, TvColor::Blue);    // SAME as unfocused!
    pub const INPUT_SELECTED: Attr = Attr::new(TvColor::Cyan, TvColor::Cyan);     // cpDialog[20] = 0x33
    pub const INPUT_ARROWS: Attr = Attr::new(TvColor::Red, TvColor::Cyan);        // cpDialog[21] = 0x34

    // Editor colors (matching original Turbo Vision)
    pub const EDITOR_NORMAL: Attr = Attr::new(TvColor::White, TvColor::Blue);
    pub const EDITOR_SELECTED: Attr = Attr::new(TvColor::Black, TvColor::Cyan);

    pub const LISTBOX_NORMAL: Attr = Attr::new(TvColor::Black, TvColor::LightGray);
    pub const LISTBOX_FOCUSED: Attr = Attr::new(TvColor::Black, TvColor::White);
    pub const LISTBOX_SELECTED: Attr = Attr::new(TvColor::White, TvColor::Blue);
    pub const LISTBOX_SELECTED_FOCUSED: Attr = Attr::new(TvColor::White, TvColor::Cyan);

    pub const SCROLLBAR_PAGE: Attr = Attr::new(TvColor::DarkGray, TvColor::LightGray);
    pub const SCROLLBAR_INDICATOR: Attr = Attr::new(TvColor::Blue, TvColor::LightGray);
    pub const SCROLLBAR_ARROW: Attr = Attr::new(TvColor::Black, TvColor::LightGray);

    pub const SCROLLER_NORMAL: Attr = Attr::new(TvColor::Black, TvColor::LightGray);
    pub const SCROLLER_SELECTED: Attr = Attr::new(TvColor::White, TvColor::Blue);

    pub const DESKTOP: Attr = Attr::new(TvColor::LightGray, TvColor::DarkGray);

    // Help system colors
    pub const HELP_NORMAL: Attr = Attr::new(TvColor::Black, TvColor::LightGray);
    pub const HELP_FOCUSED: Attr = Attr::new(TvColor::Black, TvColor::White);
}
