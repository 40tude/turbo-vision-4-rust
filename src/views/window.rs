use crate::core::geometry::Rect;
use crate::core::event::Event;
use crate::core::state::{StateFlags, SF_SHADOW, SHADOW_ATTR};
use crate::core::palette::colors;
use crate::terminal::Terminal;
use super::view::{View, draw_shadow};
use super::frame::Frame;
use super::group::Group;

pub struct Window {
    bounds: Rect,
    frame: Frame,
    interior: Group,
    state: StateFlags,
}

impl Window {
    pub fn new(bounds: Rect, title: &str) -> Self {
        let frame = Frame::new(bounds, title);

        // Interior bounds are ABSOLUTE (inset by 1 from window bounds for frame)
        let mut interior_bounds = bounds;
        interior_bounds.grow(-1, -1);
        let interior = Group::with_background(interior_bounds, colors::DIALOG_NORMAL);

        Self {
            bounds,
            frame,
            interior,
            state: SF_SHADOW, // Windows have shadows by default
        }
    }

    pub fn add(&mut self, view: Box<dyn View>) {
        self.interior.add(view);
    }

    pub fn set_initial_focus(&mut self) {
        self.interior.set_initial_focus();
    }

    /// Get the number of child views in the interior
    pub fn child_count(&self) -> usize {
        self.interior.len()
    }

    /// Get a reference to a child view by index
    pub fn child_at(&self, index: usize) -> &dyn View {
        self.interior.child_at(index)
    }

    /// Get a mutable reference to a child view by index
    pub fn child_at_mut(&mut self, index: usize) -> &mut dyn View {
        self.interior.child_at_mut(index)
    }
}

impl View for Window {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
        self.frame.set_bounds(bounds);

        // Update interior bounds (absolute, inset by 1 for frame)
        let mut interior_bounds = bounds;
        interior_bounds.grow(-1, -1);
        self.interior.set_bounds(interior_bounds);
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        self.frame.draw(terminal);
        self.interior.draw(terminal);

        // Draw shadow if enabled
        if self.has_shadow() {
            draw_shadow(terminal, self.bounds, SHADOW_ATTR);
        }
    }

    fn update_cursor(&self, terminal: &mut Terminal) {
        // Propagate cursor update to interior group
        self.interior.update_cursor(terminal);
    }

    fn handle_event(&mut self, event: &mut Event) {
        // First, let the frame handle the event (for close button clicks, etc.)
        self.frame.handle_event(event);

        // Then let the interior handle it (if not already handled)
        self.interior.handle_event(event);
    }

    fn can_focus(&self) -> bool {
        true
    }

    fn set_focus(&mut self, focused: bool) {
        // Propagate focus to the interior group
        // When the window gets focus, set focus on its first focusable child
        if focused {
            self.interior.set_initial_focus();
        } else {
            self.interior.clear_all_focus();
        }
    }

    fn state(&self) -> StateFlags {
        self.state
    }

    fn set_state(&mut self, state: StateFlags) {
        self.state = state;
    }
}