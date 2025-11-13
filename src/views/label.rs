// (C) 2025 - Enzo Lombardi

//! Label view - static text display with optional linked control focus.

use super::view::{write_line_to_terminal, View};
use crate::core::draw::DrawBuffer;
use crate::core::event::Event;
use crate::core::geometry::Rect;
use crate::core::palette::{LABEL_NORMAL, LABEL_SHORTCUT};
use crate::terminal::Terminal;

pub struct Label {
    bounds: Rect,
    text: String,
    link: Option<*const dyn View>, // Pointer to linked control
    owner: Option<*const dyn View>,
    owner_type: super::view::OwnerType,
}

impl Label {
    pub fn new(bounds: Rect, text: &str) -> Self {
        Self {
            bounds,
            text: text.to_string(),
            link: None,
            owner: None,
            owner_type: super::view::OwnerType::Dialog, // Labels default to Dialog context
        }
    }

    /// Set the linked control
    /// Matches Borland: TLabel constructor takes TView* aLink parameter
    /// When label is clicked, focus transfers to the linked control
    ///
    /// # Safety
    /// The target view must remain valid for the lifetime of this label.
    /// This is guaranteed when both views are children of the same Group.
    pub fn set_link(&mut self, target: &dyn View) {
        // SAFETY: We're storing a raw pointer to a view that must outlive this label.
        // This is guaranteed by the Group ownership model where both views are children
        // of the same parent.
        self.link = Some(unsafe { std::mem::transmute(target as *const dyn View) });
    }
}

impl View for Label {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        let width = self.bounds.width() as usize;
        let mut buf = DrawBuffer::new(width);

        // Label palette indices:
        // 1: Normal, 2: Selected, 3: Shortcut
        let normal_attr = self.map_color(LABEL_NORMAL);
        let shortcut_attr = self.map_color(LABEL_SHORTCUT);

        buf.move_char(0, ' ', normal_attr, width);
        buf.move_str_with_shortcut(0, &self.text, normal_attr, shortcut_attr);

        write_line_to_terminal(terminal, self.bounds.a.x, self.bounds.a.y, &buf);
    }

    fn handle_event(&mut self, _event: &mut Event) {
        // Labels don't handle events directly
        // Focus linking is handled by Group
    }

    /// Return the linked control pointer for this label
    /// Matches Borland: TLabel::link field
    fn label_link(&self) -> Option<*const dyn View> {
        self.link
    }

    fn set_owner(&mut self, owner: *const dyn View) {
        self.owner = Some(owner);
    }

    fn get_owner(&self) -> Option<*const dyn View> {
        self.owner
    }

    fn get_owner_type(&self) -> super::view::OwnerType {
        self.owner_type
    }

    fn set_owner_type(&mut self, owner_type: super::view::OwnerType) {
        self.owner_type = owner_type;
    }

    fn get_palette(&self) -> Option<crate::core::palette::Palette> {
        use crate::core::palette::{palettes, Palette};
        Some(Palette::from_slice(palettes::CP_LABEL))
    }
}

/// Builder for creating labels with a fluent API.
pub struct LabelBuilder {
    bounds: Option<Rect>,
    text: Option<String>,
    link: Option<*const dyn View>,
}

impl LabelBuilder {
    pub fn new() -> Self {
        Self { bounds: None, text: None, link: None }
    }

    #[must_use]
    pub fn bounds(mut self, bounds: Rect) -> Self {
        self.bounds = Some(bounds);
        self
    }

    #[must_use]
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    #[must_use]
    pub fn link(mut self, target: &dyn View) -> Self {
        // SAFETY: We're storing a raw pointer to a view that must outlive this label.
        // This is guaranteed by the Group ownership model where both views are children
        // of the same parent.
        self.link = Some(unsafe { std::mem::transmute(target as *const dyn View) });
        self
    }

    pub fn build(self) -> Label {
        let bounds = self.bounds.expect("Label bounds must be set");
        let text = self.text.expect("Label text must be set");
        let mut label = Label::new(bounds, &text);
        if let Some(link) = self.link {
            label.link = Some(link);
        }
        label
    }

    pub fn build_boxed(self) -> Box<Label> {
        Box::new(self.build())
    }
}

impl Default for LabelBuilder {
    fn default() -> Self {
        Self::new()
    }
}
