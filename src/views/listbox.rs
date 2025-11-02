use crate::core::geometry::Rect;
use crate::core::event::{Event, EventType, KB_UP, KB_DOWN, KB_PGUP, KB_PGDN, KB_HOME, KB_END, KB_ENTER, MB_LEFT_BUTTON};
use crate::core::palette::colors;
use crate::core::draw::DrawBuffer;
use crate::terminal::Terminal;
use crate::core::command::CommandId;
use super::view::{View, write_line_to_terminal};

/// ListBox - A scrollable list of selectable items
pub struct ListBox {
    bounds: Rect,
    items: Vec<String>,
    selected: Option<usize>,
    top_item: usize,
    focused: bool,
    on_select_command: CommandId,
}

impl ListBox {
    /// Create a new list box
    pub fn new(bounds: Rect, on_select_command: CommandId) -> Self {
        Self {
            bounds,
            items: Vec::new(),
            selected: None,
            top_item: 0,
            focused: false,
            on_select_command,
        }
    }

    /// Set the items in the list
    pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
        if !self.items.is_empty() && self.selected.is_none() {
            self.selected = Some(0);
        }
        self.ensure_visible();
    }

    /// Add an item to the list
    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
        if self.items.len() == 1 {
            self.selected = Some(0);
        }
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
        self.selected = None;
        self.top_item = 0;
    }

    /// Get the currently selected item index
    pub fn get_selection(&self) -> Option<usize> {
        self.selected
    }

    /// Get the currently selected item text
    pub fn get_selected_item(&self) -> Option<&str> {
        self.selected.and_then(|idx| self.items.get(idx).map(|s| s.as_str()))
    }

    /// Set the selected item by index
    pub fn set_selection(&mut self, index: usize) {
        if index < self.items.len() {
            self.selected = Some(index);
            self.ensure_visible();
        }
    }

    /// Get the number of items
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Ensure the selected item is visible
    fn ensure_visible(&mut self) {
        if let Some(selected) = self.selected {
            let visible_count = self.bounds.height() as usize;

            // If selected is above visible area, scroll up
            if selected < self.top_item {
                self.top_item = selected;
            }
            // If selected is below visible area, scroll down
            else if selected >= self.top_item + visible_count {
                self.top_item = selected - visible_count + 1;
            }
        }
    }

    /// Move selection up
    fn select_prev(&mut self) {
        if self.items.is_empty() {
            return;
        }

        if let Some(selected) = self.selected {
            if selected > 0 {
                self.selected = Some(selected - 1);
                self.ensure_visible();
            }
        } else {
            self.selected = Some(0);
        }
    }

    /// Move selection down
    fn select_next(&mut self) {
        if self.items.is_empty() {
            return;
        }

        if let Some(selected) = self.selected {
            if selected + 1 < self.items.len() {
                self.selected = Some(selected + 1);
                self.ensure_visible();
            }
        } else {
            self.selected = Some(0);
        }
    }

    /// Select first item
    fn select_first(&mut self) {
        if !self.items.is_empty() {
            self.selected = Some(0);
            self.top_item = 0;
        }
    }

    /// Select last item
    fn select_last(&mut self) {
        if !self.items.is_empty() {
            self.selected = Some(self.items.len() - 1);
            self.ensure_visible();
        }
    }

    /// Page up
    fn page_up(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let page_size = self.bounds.height() as usize;
        if let Some(selected) = self.selected {
            let new_selected = selected.saturating_sub(page_size);
            self.selected = Some(new_selected);
            self.ensure_visible();
        }
    }

    /// Page down
    fn page_down(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let page_size = self.bounds.height() as usize;
        if let Some(selected) = self.selected {
            let new_selected = (selected + page_size).min(self.items.len() - 1);
            self.selected = Some(new_selected);
            self.ensure_visible();
        }
    }
}

impl View for ListBox {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
        self.ensure_visible();
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        let width = self.bounds.width() as usize;
        let height = self.bounds.height() as usize;

        let color_normal = if self.focused {
            colors::LISTBOX_FOCUSED
        } else {
            colors::LISTBOX_NORMAL
        };
        let color_selected = if self.focused {
            colors::LISTBOX_SELECTED_FOCUSED
        } else {
            colors::LISTBOX_SELECTED
        };

        // Draw each visible line
        for y in 0..height {
            let item_idx = self.top_item + y;
            let mut buf = DrawBuffer::new(width);

            if item_idx < self.items.len() {
                let is_selected = self.selected == Some(item_idx);
                let color = if is_selected { color_selected } else { color_normal };

                // Fill line with background
                buf.move_char(0, ' ', color, width);

                // Draw item text, truncating if needed
                let text = &self.items[item_idx];
                let display_text = if text.len() > width {
                    &text[..width]
                } else {
                    text
                };

                buf.move_str(0, display_text, color);
            } else {
                // Empty line
                buf.move_char(0, ' ', color_normal, width);
            }

            write_line_to_terminal(terminal, self.bounds.a.x, self.bounds.a.y + y as i16, &buf);
        }
    }

    fn handle_event(&mut self, event: &mut Event) {
        match event.what {
            EventType::Keyboard => {
                // Only handle keyboard events if focused
                if !self.focused {
                    return;
                }
                match event.key_code {
                    KB_UP => {
                        self.select_prev();
                        event.clear();
                    }
                    KB_DOWN => {
                        self.select_next();
                        event.clear();
                    }
                    KB_PGUP => {
                        self.page_up();
                        event.clear();
                    }
                    KB_PGDN => {
                        self.page_down();
                        event.clear();
                    }
                    KB_HOME => {
                        self.select_first();
                        event.clear();
                    }
                    KB_END => {
                        self.select_last();
                        event.clear();
                    }
                    KB_ENTER => {
                        if self.selected.is_some() {
                            *event = Event::command(self.on_select_command);
                        }
                    }
                    _ => {}
                }
            }
            EventType::MouseDown => {
                let mouse_pos = event.mouse.pos;

                // Check if click is within the listbox bounds
                if mouse_pos.x >= self.bounds.a.x && mouse_pos.x < self.bounds.b.x &&
                   mouse_pos.y >= self.bounds.a.y && mouse_pos.y < self.bounds.b.y {

                    if event.mouse.buttons & MB_LEFT_BUTTON != 0 {
                        // Calculate which item was clicked
                        let relative_y = (mouse_pos.y - self.bounds.a.y) as usize;
                        let clicked_item = self.top_item + relative_y;

                        if clicked_item < self.items.len() {
                            // Check if clicking the same item (double-click behavior)
                            let was_selected = self.selected == Some(clicked_item);

                            // Select the clicked item
                            self.selected = Some(clicked_item);
                            event.clear();

                            // If clicking already selected item, trigger selection command
                            if was_selected {
                                *event = Event::command(self.on_select_command);
                            }
                        }
                    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_listbox_creation() {
        let listbox = ListBox::new(Rect::new(0, 0, 20, 10), 1000);
        assert_eq!(listbox.item_count(), 0);
        assert_eq!(listbox.get_selection(), None);
    }

    #[test]
    fn test_listbox_add_items() {
        let mut listbox = ListBox::new(Rect::new(0, 0, 20, 10), 1000);
        listbox.add_item("Item 1".to_string());
        listbox.add_item("Item 2".to_string());
        listbox.add_item("Item 3".to_string());

        assert_eq!(listbox.item_count(), 3);
        assert_eq!(listbox.get_selection(), Some(0));
        assert_eq!(listbox.get_selected_item(), Some("Item 1"));
    }

    #[test]
    fn test_listbox_set_items() {
        let mut listbox = ListBox::new(Rect::new(0, 0, 20, 10), 1000);
        let items = vec![
            "Alpha".to_string(),
            "Beta".to_string(),
            "Gamma".to_string(),
        ];
        listbox.set_items(items);

        assert_eq!(listbox.item_count(), 3);
        assert_eq!(listbox.get_selection(), Some(0));
    }

    #[test]
    fn test_listbox_navigation() {
        let mut listbox = ListBox::new(Rect::new(0, 0, 20, 10), 1000);
        listbox.set_items(vec![
            "Item 1".to_string(),
            "Item 2".to_string(),
            "Item 3".to_string(),
        ]);

        assert_eq!(listbox.get_selection(), Some(0));

        listbox.select_next();
        assert_eq!(listbox.get_selection(), Some(1));

        listbox.select_next();
        assert_eq!(listbox.get_selection(), Some(2));

        listbox.select_next(); // Should stay at 2 (last item)
        assert_eq!(listbox.get_selection(), Some(2));

        listbox.select_prev();
        assert_eq!(listbox.get_selection(), Some(1));

        listbox.select_first();
        assert_eq!(listbox.get_selection(), Some(0));

        listbox.select_last();
        assert_eq!(listbox.get_selection(), Some(2));
    }

    #[test]
    fn test_listbox_set_selection() {
        let mut listbox = ListBox::new(Rect::new(0, 0, 20, 10), 1000);
        listbox.set_items(vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
        ]);

        listbox.set_selection(2);
        assert_eq!(listbox.get_selection(), Some(2));
        assert_eq!(listbox.get_selected_item(), Some("C"));

        listbox.set_selection(10); // Out of bounds, should be ignored
        assert_eq!(listbox.get_selection(), Some(2)); // Should not change
    }

    #[test]
    fn test_listbox_clear() {
        let mut listbox = ListBox::new(Rect::new(0, 0, 20, 10), 1000);
        listbox.set_items(vec!["Item 1".to_string(), "Item 2".to_string()]);
        assert_eq!(listbox.item_count(), 2);

        listbox.clear();
        assert_eq!(listbox.item_count(), 0);
        assert_eq!(listbox.get_selection(), None);
    }
}
