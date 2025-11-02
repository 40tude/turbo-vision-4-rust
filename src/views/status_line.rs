use crate::core::geometry::Rect;
use crate::core::event::{Event, EventType, KeyCode, MB_LEFT_BUTTON};
use crate::core::draw::DrawBuffer;
use crate::core::palette::colors;
use crate::core::command::CommandId;
use crate::terminal::Terminal;
use super::view::{View, write_line_to_terminal};

pub struct StatusItem {
    pub text: String,
    pub key_code: KeyCode,
    pub command: CommandId,
}

impl StatusItem {
    pub fn new(text: &str, key_code: KeyCode, command: CommandId) -> Self {
        Self {
            text: text.to_string(),
            key_code,
            command,
        }
    }
}

pub struct StatusLine {
    bounds: Rect,
    items: Vec<StatusItem>,
    item_positions: Vec<(i16, i16)>, // (start_x, end_x) for each item
}

impl StatusLine {
    pub fn new(bounds: Rect, items: Vec<StatusItem>) -> Self {
        Self {
            bounds,
            items,
            item_positions: Vec::new(),
        }
    }
}

impl View for StatusLine {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        let width = self.bounds.width() as usize;
        let mut buf = DrawBuffer::new(width);
        buf.move_char(0, ' ', colors::STATUS_NORMAL, width);

        // Clear previous item positions
        self.item_positions.clear();

        let mut x = 1;
        for item in &self.items {
            if x + item.text.len() + 2 < width {
                let start_x = x as i16;

                // Parse ~X~ for highlighting - everything between tildes is red
                let mut chars = item.text.chars();
                #[allow(clippy::while_let_on_iterator)]
                while let Some(ch) = chars.next() {
                    if ch == '~' {
                        // Read all characters until closing ~ in red
                        #[allow(clippy::while_let_on_iterator)]
                        while let Some(shortcut_ch) = chars.next() {
                            if shortcut_ch == '~' {
                                break;  // Found closing tilde
                            }
                            buf.put_char(x, shortcut_ch, colors::STATUS_SHORTCUT);  // Red color
                            x += 1;
                        }
                    } else {
                        buf.put_char(x, ch, colors::STATUS_NORMAL);
                        x += 1;
                    }
                }

                let end_x = x as i16;
                self.item_positions.push((start_x, end_x));

                buf.move_str(x, " │ ", colors::STATUS_NORMAL);
                x += 3;
            }
        }

        write_line_to_terminal(terminal, self.bounds.a.x, self.bounds.a.y, &buf);
    }

    fn handle_event(&mut self, event: &mut Event) {
        // Handle mouse clicks on status items
        if event.what == EventType::MouseDown {
            let mouse_pos = event.mouse.pos;

            if event.mouse.buttons & MB_LEFT_BUTTON != 0 {
                // Check if click is on the status line
                if mouse_pos.y == self.bounds.a.y {
                    // Check which item was clicked
                    for (i, &(start_x, end_x)) in self.item_positions.iter().enumerate() {
                        if i < self.items.len() {
                            let absolute_start = self.bounds.a.x + start_x;
                            let absolute_end = self.bounds.a.x + end_x;

                            if mouse_pos.x >= absolute_start && mouse_pos.x < absolute_end {
                                // Item clicked - execute its command
                                let item = &self.items[i];
                                if item.command != 0 {
                                    *event = Event::command(item.command);
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Handle keyboard shortcuts
        if event.what == EventType::Keyboard {
            for item in &self.items {
                if event.key_code == item.key_code {
                    *event = Event::command(item.command);
                    return;
                }
            }
        }
    }
}
