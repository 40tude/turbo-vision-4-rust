use crate::core::geometry::Rect;
use crate::core::event::Event;
use crate::core::draw::DrawBuffer;
use crate::core::palette::colors;
use crate::terminal::Terminal;
use super::view::{View, write_line_to_terminal};
use super::group::Group;

pub struct Desktop {
    bounds: Rect,
    children: Group,
}

impl Desktop {
    pub fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            children: Group::new(bounds),
        }
    }

    pub fn add(&mut self, view: Box<dyn View>) {
        self.children.add(view);
        // Focus on the newly added window (last child)
        let num_children = self.children.len();
        if num_children > 0 {
            let last_idx = num_children - 1;
            if self.children.child_at(last_idx).can_focus() {
                // Clear focus from all children first
                self.children.clear_all_focus();
                // Then give focus to the new window
                self.children.set_focus_to(last_idx);
            }
        }
    }

    /// Get the number of child views (windows) on the desktop
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Get a reference to a child view by index
    pub fn child_at(&self, index: usize) -> &dyn View {
        self.children.child_at(index)
    }
}

impl View for Desktop {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
        self.children.set_bounds(bounds);
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        // Fill background with pattern
        let width = self.bounds.width() as usize;
        let mut buf = DrawBuffer::new(width);
        buf.move_char(0, '░', colors::DESKTOP, width);

        for y in self.bounds.a.y..self.bounds.b.y {
            write_line_to_terminal(terminal, self.bounds.a.x, y, &buf);
        }

        // Draw children
        self.children.draw(terminal);
    }

    fn handle_event(&mut self, event: &mut Event) {
        self.children.handle_event(event);
    }
}
