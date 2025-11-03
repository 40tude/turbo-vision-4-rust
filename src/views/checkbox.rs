// CheckBox - Boolean selection control
//
// Original Turbo Vision equivalent: TCheckBoxes (but simplified to single checkbox)
//
// A checkbox control displays a box with a label. The box can be checked or unchecked
// by clicking on it or pressing Space when focused.
//
// Visual appearance:
//   [ ] Unchecked option
//   [X] Checked option
//
// Usage:
//   let checkbox = CheckBox::new(
//       Rect::new(3, 5, 20, 6),
//       "Enable feature",
//   );

use crate::core::draw::DrawBuffer;
use crate::core::event::{Event, EventType};
use crate::core::geometry::Rect;
use crate::core::palette::{Attr, TvColor};
use crate::core::state::StateFlags;
use crate::terminal::Terminal;
use crate::views::view::{View, write_line_to_terminal};

/// CheckBox - A boolean selection control with a label
#[derive(Debug)]
pub struct CheckBox {
    bounds: Rect,
    label: String,
    checked: bool,
    state: StateFlags,
}

impl CheckBox {
    /// Create a new checkbox with the given bounds and label
    pub fn new(bounds: Rect, label: &str) -> Self {
        CheckBox {
            bounds,
            label: label.to_string(),
            checked: false,
            state: 0,
        }
    }

    /// Set the checked state
    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }

    /// Get the checked state
    pub fn is_checked(&self) -> bool {
        self.checked
    }

    /// Toggle the checked state
    pub fn toggle(&mut self) {
        self.checked = !self.checked;
    }
}

impl View for CheckBox {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn handle_event(&mut self, event: &mut Event) {
        if event.what == EventType::Keyboard && self.is_focused() {
            // Space key toggles
            if event.key_code == ' ' as u16 {
                self.toggle();
                event.clear();
            }
            // TODO: Add hotkey support (need to map key_code to character)
        }
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        let width = self.bounds.width() as usize;
        let mut buffer = DrawBuffer::new(width);

        // Determine colors based on focus state
        let color = if self.is_focused() {
            Attr::new(TvColor::Yellow, TvColor::Blue)
        } else {
            Attr::new(TvColor::Black, TvColor::LightGray)
        };

        let hotkey_color = if self.is_focused() {
            Attr::new(TvColor::LightRed, TvColor::Blue)
        } else {
            Attr::new(TvColor::Red, TvColor::LightGray)
        };

        // Draw checkbox
        let checkbox_str = if self.checked { "[X] " } else { "[ ] " };
        let mut x = 0;

        for ch in checkbox_str.chars() {
            if x < width {
                buffer.put_char(x, ch, color);
                x += 1;
            }
        }

        // Draw label with shortcut highlighting (returns number of chars written)
        let written = buffer.move_str_with_shortcut(x, &self.label, color, hotkey_color);
        x += written;

        // Fill remaining space
        if x < width {
            buffer.move_char(x, ' ', color, width - x);
        }

        // Write to terminal
        write_line_to_terminal(terminal, self.bounds.a.x, self.bounds.a.y, &buffer);
    }

    fn can_focus(&self) -> bool {
        true
    }

    // set_focus() now uses default implementation from View trait
    // which sets/clears SF_FOCUSED flag

    fn state(&self) -> StateFlags {
        self.state
    }

    fn set_state(&mut self, state: StateFlags) {
        self.state = state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkbox_creation() {
        let checkbox = CheckBox::new(Rect::new(0, 0, 20, 1), "Test option");
        assert!(!checkbox.is_checked());
        assert_eq!(checkbox.label, "Test option");
    }

    #[test]
    fn test_checkbox_toggle() {
        let mut checkbox = CheckBox::new(Rect::new(0, 0, 20, 1), "Test");
        assert!(!checkbox.is_checked());

        checkbox.toggle();
        assert!(checkbox.is_checked());

        checkbox.toggle();
        assert!(!checkbox.is_checked());
    }

    #[test]
    fn test_checkbox_set_checked() {
        let mut checkbox = CheckBox::new(Rect::new(0, 0, 20, 1), "Test");

        checkbox.set_checked(true);
        assert!(checkbox.is_checked());

        checkbox.set_checked(false);
        assert!(!checkbox.is_checked());
    }
}
