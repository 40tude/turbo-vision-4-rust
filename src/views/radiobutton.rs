// RadioButton - Mutually exclusive selection control
//
// Original Turbo Vision equivalent: TRadioButtons
//
// A radio button control displays a circle with a label. Only one radio button
// in a group can be selected at a time. Radio buttons with the same group_id
// form a mutually exclusive group.
//
// Visual appearance:
//   ( ) Unselected option
//   (•) Selected option
//
// Usage:
//   let radio1 = RadioButton::new(
//       Rect::new(3, 5, 20, 6),
//       "Option 1",
//       1,  // group_id
//   );

use crate::core::draw::DrawBuffer;
use crate::core::event::{Event, EventType};
use crate::core::geometry::Rect;
use crate::core::palette::{Attr, TvColor};
use crate::terminal::Terminal;
use crate::views::view::{View, write_line_to_terminal};

/// RadioButton - A mutually exclusive selection control with a label
#[derive(Debug)]
pub struct RadioButton {
    bounds: Rect,
    label: String,
    group_id: u16,
    selected: bool,
    focused: bool,
}

impl RadioButton {
    /// Create a new radio button with the given bounds, label, and group ID
    ///
    /// Radio buttons with the same group_id are mutually exclusive.
    pub fn new(bounds: Rect, label: &str, group_id: u16) -> Self {
        RadioButton {
            bounds,
            label: label.to_string(),
            group_id,
            selected: false,
            focused: false,
        }
    }

    /// Set the selected state
    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    /// Get the selected state
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    /// Get the group ID
    pub fn group_id(&self) -> u16 {
        self.group_id
    }

    /// Select this radio button (should deselect others in the group)
    pub fn select(&mut self) {
        self.selected = true;
    }

    /// Deselect this radio button
    pub fn deselect(&mut self) {
        self.selected = false;
    }
}

impl View for RadioButton {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn handle_event(&mut self, event: &mut Event) {
        if event.what == EventType::Keyboard && self.focused {
            // Space key selects
            if event.key_code == ' ' as u16 {
                self.select();
                // TODO: Parent should deselect other radio buttons in the same group
                event.clear();
            }
            // TODO: Add hotkey support (need to map key_code to character)
        }
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        let width = self.bounds.width() as usize;
        let mut buffer = DrawBuffer::new(width);

        // Determine colors based on focus state
        let color = if self.focused {
            Attr::new(TvColor::Yellow, TvColor::Blue)
        } else {
            Attr::new(TvColor::Black, TvColor::LightGray)
        };

        let hotkey_color = if self.focused {
            Attr::new(TvColor::LightRed, TvColor::Blue)
        } else {
            Attr::new(TvColor::Red, TvColor::LightGray)
        };

        // Draw radio button
        let radio_str = if self.selected { "(•) " } else { "( ) " };
        let mut x = 0;

        for ch in radio_str.chars() {
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

    fn set_focus(&mut self, focused: bool) {
        self.focused = focused;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radiobutton_creation() {
        let radio = RadioButton::new(Rect::new(0, 0, 20, 1), "Option 1", 1);
        assert!(!radio.is_selected());
        assert_eq!(radio.label, "Option 1");
        assert_eq!(radio.group_id(), 1);
    }

    #[test]
    fn test_radiobutton_select() {
        let mut radio = RadioButton::new(Rect::new(0, 0, 20, 1), "Option 1", 1);
        assert!(!radio.is_selected());

        radio.select();
        assert!(radio.is_selected());

        radio.deselect();
        assert!(!radio.is_selected());
    }

    #[test]
    fn test_radiobutton_set_selected() {
        let mut radio = RadioButton::new(Rect::new(0, 0, 20, 1), "Option 1", 1);

        radio.set_selected(true);
        assert!(radio.is_selected());

        radio.set_selected(false);
        assert!(!radio.is_selected());
    }

    #[test]
    fn test_radiobutton_group_id() {
        let radio1 = RadioButton::new(Rect::new(0, 0, 20, 1), "Option 1", 1);
        let radio2 = RadioButton::new(Rect::new(0, 1, 20, 2), "Option 2", 1);
        let radio3 = RadioButton::new(Rect::new(0, 2, 20, 3), "Option 3", 2);

        assert_eq!(radio1.group_id(), 1);
        assert_eq!(radio2.group_id(), 1);
        assert_eq!(radio3.group_id(), 2);
    }
}
