use crate::core::geometry::Rect;
use crate::core::event::{Event, EventType, KB_TAB, KB_SHIFT_TAB};
use crate::core::draw::DrawBuffer;
use crate::core::palette::Attr;
use crate::terminal::Terminal;
use super::view::{View, write_line_to_terminal};

/// Group - a container for child views
pub struct Group {
    bounds: Rect,
    children: Vec<Box<dyn View>>,
    focused: usize,
    background: Option<Attr>,
}

impl Group {
    pub fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            children: Vec::new(),
            focused: 0,
            background: None,
        }
    }

    pub fn with_background(bounds: Rect, background: Attr) -> Self {
        Self {
            bounds,
            children: Vec::new(),
            focused: 0,
            background: Some(background),
        }
    }

    pub fn add(&mut self, mut view: Box<dyn View>) {
        // Convert child's bounds from relative to absolute coordinates
        // Child bounds are specified relative to this Group's interior
        let child_bounds = view.bounds();
        let absolute_bounds = Rect::new(
            self.bounds.a.x + child_bounds.a.x,
            self.bounds.a.y + child_bounds.a.y,
            self.bounds.a.x + child_bounds.b.x,
            self.bounds.a.y + child_bounds.b.y,
        );
        view.set_bounds(absolute_bounds);
        self.children.push(view);
    }

    pub fn set_initial_focus(&mut self) {
        if self.children.is_empty() {
            return;
        }

        // Find first focusable child and set focus
        for i in 0..self.children.len() {
            if self.children[i].can_focus() {
                self.focused = i;
                self.children[i].set_focus(true);
                break;
            }
        }
    }

    pub fn clear_all_focus(&mut self) {
        for child in &mut self.children {
            child.set_focus(false);
        }
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub fn child_at(&self, index: usize) -> &dyn View {
        &*self.children[index]
    }

    pub fn child_at_mut(&mut self, index: usize) -> &mut dyn View {
        &mut *self.children[index]
    }

    pub fn set_focus_to(&mut self, index: usize) {
        if index < self.children.len() {
            self.focused = index;
            self.children[index].set_focus(true);
        }
    }

    pub fn select_next(&mut self) {
        if self.children.is_empty() {
            return;
        }

        // Clear focus from current child
        if self.focused < self.children.len() {
            self.children[self.focused].set_focus(false);
        }

        let start_index = self.focused;
        loop {
            self.focused = (self.focused + 1) % self.children.len();
            if self.children[self.focused].can_focus() {
                self.children[self.focused].set_focus(true);
                break;
            }
            // Prevent infinite loop if no focusable children
            if self.focused == start_index {
                break;
            }
        }
    }

    pub fn select_previous(&mut self) {
        if self.children.is_empty() {
            return;
        }

        // Clear focus from current child
        if self.focused < self.children.len() {
            self.children[self.focused].set_focus(false);
        }

        let start_index = self.focused;
        loop {
            // Move to previous, wrapping around
            if self.focused == 0 {
                self.focused = self.children.len() - 1;
            } else {
                self.focused -= 1;
            }

            if self.children[self.focused].can_focus() {
                self.children[self.focused].set_focus(true);
                break;
            }
            // Prevent infinite loop if no focusable children
            if self.focused == start_index {
                break;
            }
        }
    }
}

impl View for Group {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        // Calculate the offset (how much the group moved)
        let dx = bounds.a.x - self.bounds.a.x;
        let dy = bounds.a.y - self.bounds.a.y;

        // Update our bounds
        self.bounds = bounds;

        // Update all children's bounds by the same offset
        for child in &mut self.children {
            let child_bounds = child.bounds();
            let new_bounds = Rect::new(
                child_bounds.a.x + dx,
                child_bounds.a.y + dy,
                child_bounds.b.x + dx,
                child_bounds.b.y + dy,
            );
            child.set_bounds(new_bounds);
        }
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        // Draw background if specified
        if let Some(bg_attr) = self.background {
            let width = self.bounds.width() as usize;
            let height = self.bounds.height() as usize;

            for y in 0..height {
                let mut buf = DrawBuffer::new(width);
                buf.move_char(0, ' ', bg_attr, width);
                write_line_to_terminal(
                    terminal,
                    self.bounds.a.x,
                    self.bounds.a.y + y as i16,
                    &buf,
                );
            }
        }

        // Push clipping region for this group's bounds
        terminal.push_clip(self.bounds);

        // Only draw children that intersect with this group's bounds
        // The clipping region ensures children can't render outside parent boundaries
        for child in &mut self.children {
            let child_bounds = child.bounds();
            if self.bounds.intersects(&child_bounds) {
                child.draw(terminal);
            }
        }

        // Pop clipping region
        terminal.pop_clip();
    }

    fn handle_event(&mut self, event: &mut Event) {
        // Handle Tab key for focus navigation
        if event.what == EventType::Keyboard {
            if event.key_code == KB_TAB {
                self.select_next();
                event.clear();
                return;
            } else if event.key_code == KB_SHIFT_TAB {
                self.select_previous();
                event.clear();
                return;
            }
        }

        // Mouse events: send to the child under the mouse
        if event.what == EventType::MouseDown || event.what == EventType::MouseMove || event.what == EventType::MouseUp {
            let mouse_pos = event.mouse.pos;

            // First pass: find which child contains the mouse and needs focus
            let mut clicked_child_index: Option<usize> = None;
            for (i, child) in self.children.iter().enumerate() {
                let child_bounds = child.bounds();
                if child_bounds.contains(mouse_pos) {
                    clicked_child_index = Some(i);
                    break;
                }
            }

            // If a focusable child was clicked, give it focus
            if let Some(i) = clicked_child_index {
                if event.what == EventType::MouseDown && self.children[i].can_focus() {
                    self.clear_all_focus();
                    self.focused = i;
                    self.children[i].set_focus(true);
                }

                // Second pass: handle the event
                self.children[i].handle_event(event);
                return;
            }
        }

        // Keyboard and other events: only send to focused child
        if self.focused < self.children.len() {
            self.children[self.focused].handle_event(event);
        }
    }

    fn update_cursor(&self, terminal: &mut Terminal) {
        // Hide cursor by default
        let _ = terminal.hide_cursor();

        // Update cursor for the focused child (it can show it if needed)
        if self.focused < self.children.len() {
            self.children[self.focused].update_cursor(terminal);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to count how many times draw is called on views
    struct DrawCountView {
        bounds: Rect,
        draw_count: std::cell::RefCell<usize>,
    }

    impl DrawCountView {
        fn new(bounds: Rect) -> Self {
            Self {
                bounds,
                draw_count: std::cell::RefCell::new(0),
            }
        }
    }

    impl View for DrawCountView {
        fn bounds(&self) -> Rect {
            self.bounds
        }

        fn set_bounds(&mut self, bounds: Rect) {
            self.bounds = bounds;
        }

        fn draw(&mut self, _terminal: &mut Terminal) {
            *self.draw_count.borrow_mut() += 1;
        }

        fn handle_event(&mut self, _event: &mut Event) {}
    }

    #[test]
    fn test_child_completely_outside_parent_not_drawn() {
        // Create a group at (10, 10) with size 20x20
        let group = Group::new(Rect::new(10, 10, 30, 30));

        // Add a child completely outside the parent bounds (to the right)
        let child_bounds = Rect::new(100, 15, 110, 20);

        // Verify the child is outside parent bounds
        assert!(!group.bounds.intersects(&child_bounds));
    }

    #[test]
    fn test_child_inside_parent_is_drawn() {
        // Create a group at (10, 10) with size 20x20
        let mut group = Group::new(Rect::new(10, 10, 30, 30));

        // Add a child at relative position (5, 5) which becomes absolute (15, 15)
        // This is inside the parent bounds (10, 10, 30, 30)
        let child = Box::new(DrawCountView::new(Rect::new(5, 5, 15, 15)));
        group.add(child);

        // Verify the child was converted to absolute coordinates
        assert_eq!(group.children.len(), 1);
        assert_eq!(group.children[0].bounds(), Rect::new(15, 15, 25, 25));

        // Verify child intersects with parent (so it would be drawn)
        assert!(group.bounds.intersects(&group.children[0].bounds()));
    }

    #[test]
    fn test_child_partially_outside_parent() {
        // Create a group at (10, 10) with size 20x20 (bounds: 10-30, 10-30)
        let mut group = Group::new(Rect::new(10, 10, 30, 30));

        // Add a child at relative position (15, 15) with size 10x10
        // Absolute bounds: (25, 25, 35, 35)
        // This extends beyond parent (30, 30), so partially outside
        let child = Box::new(DrawCountView::new(Rect::new(15, 15, 25, 25)));
        group.add(child);

        // Verify conversion to absolute
        assert_eq!(group.children[0].bounds(), Rect::new(25, 25, 35, 35));

        // Verify child still intersects with parent (partially visible)
        assert!(group.bounds.intersects(&group.children[0].bounds()));

        // Note: The child will be drawn, but the Terminal's write methods
        // will clip at the terminal boundaries. For proper parent clipping,
        // we would need to implement a clipping region in Terminal.
        // For now, we just verify that intersecting children would be drawn.
    }

    #[test]
    fn test_coordinate_conversion_on_add() {
        // Create a group at (20, 30) with size 40x50
        let mut group = Group::new(Rect::new(20, 30, 60, 80));

        // Add a child with relative coordinates (5, 10)
        let child = Box::new(DrawCountView::new(Rect::new(5, 10, 15, 20)));
        group.add(child);

        // Verify the child's bounds were converted to absolute
        // Relative (5, 10, 15, 20) + Group origin (20, 30) = Absolute (25, 40, 35, 50)
        assert_eq!(group.children[0].bounds(), Rect::new(25, 40, 35, 50));
    }

    #[test]
    fn test_multiple_children_clipping() {
        // Create a group at (0, 0) with size 50x50
        let mut group = Group::new(Rect::new(0, 0, 50, 50));

        // Child 1: Inside (10, 10, 20, 20) -> absolute (10, 10, 20, 20)
        group.add(Box::new(DrawCountView::new(Rect::new(10, 10, 20, 20))));

        // Child 2: Completely outside (100, 100, 110, 110) -> absolute (100, 100, 110, 110)
        group.add(Box::new(DrawCountView::new(Rect::new(100, 100, 110, 110))));

        // Child 3: Partially outside (40, 40, 60, 60) -> absolute (40, 40, 60, 60)
        group.add(Box::new(DrawCountView::new(Rect::new(40, 40, 60, 60))));

        assert_eq!(group.children.len(), 3);

        // Verify intersections
        // Child 1: completely inside, should intersect
        assert!(group.bounds.intersects(&group.children[0].bounds()));

        // Child 2: completely outside, should NOT intersect
        assert!(!group.bounds.intersects(&group.children[1].bounds()));

        // Child 3: partially outside, should intersect
        assert!(group.bounds.intersects(&group.children[2].bounds()));
    }
}
