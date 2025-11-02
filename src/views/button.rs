use super::view::{write_line_to_terminal, View};
use crate::core::command::CommandId;
use crate::core::draw::DrawBuffer;
use crate::core::event::{Event, EventType, KB_ENTER, MB_LEFT_BUTTON};
use crate::core::geometry::Rect;
use crate::core::palette::colors;
use crate::core::state::{SHADOW_BOTTOM, SHADOW_SOLID, SHADOW_TOP};
use crate::terminal::Terminal;

pub struct Button {
    bounds: Rect,
    title: String,
    command: CommandId,
    is_default: bool,
    focused: bool,
}

impl Button {
    pub fn new(bounds: Rect, title: &str, command: CommandId, is_default: bool) -> Self {
        Self {
            bounds,
            title: title.to_string(),
            command,
            is_default,
            focused: false,
        }
    }
}

impl View for Button {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        let width = self.bounds.width() as usize;
        let height = self.bounds.height() as usize;

        let button_attr = if self.focused {
            colors::BUTTON_SELECTED
        } else if self.is_default {
            colors::BUTTON_DEFAULT
        } else {
            colors::BUTTON_NORMAL
        };

        // Shadow uses DarkGray on LightGray (not black background!)
        let shadow_attr = colors::BUTTON_SHADOW;

        // Shortcut attributes - use white for button shortcuts
        let shortcut_attr = if self.focused {
            colors::BUTTON_SELECTED  // Same as focused button
        } else {
            colors::BUTTON_SHORTCUT  // White on LightGray
        };

        // Draw all lines except the last (which is the bottom shadow)
        for y in 0..(height - 1) {
            let mut buf = DrawBuffer::new(width);

            // Fill entire line with button color
            buf.move_char(0, ' ', button_attr, width);

            // Right edge gets shadow character and attribute (last column)
            let shadow_char = if y == 0 { SHADOW_TOP } else { SHADOW_SOLID };
            buf.put_char(width - 1, shadow_char, shadow_attr);

            // Draw the label on the middle line
            if y == (height - 1) / 2 {
                // Calculate display length without tildes
                let display_len = self.title.chars().filter(|&c| c != '~').count();
                let content_width = width - 1; // Exclude right shadow column
                let start = (content_width.saturating_sub(display_len)) / 2;
                buf.move_str_with_shortcut(start, &self.title, button_attr, shortcut_attr);
            }

            write_line_to_terminal(
                terminal,
                self.bounds.a.x,
                self.bounds.a.y + y as i16,
                &buf,
            );
        }

        // Draw bottom shadow line (1 char shorter, offset 1 to the right)
        let mut bottom_buf = DrawBuffer::new(width - 1);
        // Bottom shadow character across width-1
        bottom_buf.move_char(0, SHADOW_BOTTOM, shadow_attr, width - 1);
        write_line_to_terminal(
            terminal,
            self.bounds.a.x + 1,
            self.bounds.a.y + (height - 1) as i16,
            &bottom_buf,
        );
    }

    fn handle_event(&mut self, event: &mut Event) {
        match event.what {
            EventType::Keyboard => {
                // Only handle keyboard events if focused
                if !self.focused {
                    return;
                }
                if event.key_code == KB_ENTER || event.key_code == ' ' as u16 {
                    *event = Event::command(self.command);
                }
            }
            EventType::MouseDown => {
                // Check if click is within button bounds
                let mouse_pos = event.mouse.pos;
                if event.mouse.buttons & MB_LEFT_BUTTON != 0
                    && mouse_pos.x >= self.bounds.a.x
                    && mouse_pos.x < self.bounds.b.x
                    && mouse_pos.y >= self.bounds.a.y
                    && mouse_pos.y < self.bounds.b.y - 1  // Exclude shadow line
                {
                    // Button clicked - generate command
                    *event = Event::command(self.command);
                }
            }
            _ => {}
        }
    }

    fn can_focus(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool) {
        self.focused = focused;
    }
}
