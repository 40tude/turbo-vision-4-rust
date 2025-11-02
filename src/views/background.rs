use crate::core::geometry::Rect;
use crate::core::event::Event;
use crate::core::palette::colors;
use crate::core::draw::DrawBuffer;
use crate::terminal::Terminal;
use super::view::{View, write_line_to_terminal};

/// Background - Desktop background with pattern
/// Displays a repeating pattern character across the entire bounds
pub struct Background {
    bounds: Rect,
    pattern: char,
}

impl Background {
    /// Create a new background with the classic checkered pattern (░)
    pub fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            pattern: '░', // Light shade character (U+2591)
        }
    }

    /// Create a background with a custom pattern character
    pub fn with_pattern(bounds: Rect, pattern: char) -> Self {
        Self {
            bounds,
            pattern,
        }
    }

    /// Set the pattern character
    pub fn set_pattern(&mut self, pattern: char) {
        self.pattern = pattern;
    }

    /// Get the pattern character
    pub fn get_pattern(&self) -> char {
        self.pattern
    }
}

impl View for Background {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        let width = self.bounds.width() as usize;
        let height = self.bounds.height() as usize;

        // Draw the pattern across all lines
        for y in 0..height {
            let mut buf = DrawBuffer::new(width);
            buf.move_char(0, self.pattern, colors::DESKTOP, width);
            write_line_to_terminal(terminal, self.bounds.a.x, self.bounds.a.y + y as i16, &buf);
        }
    }

    fn handle_event(&mut self, _event: &mut Event) {
        // Background doesn't handle events
    }
}

/// Predefined background patterns
impl Background {
    /// Classic light checkered pattern
    pub fn checkered_light(bounds: Rect) -> Self {
        Self::with_pattern(bounds, '░')
    }

    /// Medium checkered pattern
    pub fn checkered_medium(bounds: Rect) -> Self {
        Self::with_pattern(bounds, '▒')
    }

    /// Dark checkered pattern
    pub fn checkered_dark(bounds: Rect) -> Self {
        Self::with_pattern(bounds, '▓')
    }

    /// Solid block pattern
    pub fn solid(bounds: Rect) -> Self {
        Self::with_pattern(bounds, '█')
    }

    /// Dots pattern
    pub fn dots(bounds: Rect) -> Self {
        Self::with_pattern(bounds, '·')
    }

    /// Cross pattern
    pub fn cross(bounds: Rect) -> Self {
        Self::with_pattern(bounds, '┼')
    }

    /// Blank (space)
    pub fn blank(bounds: Rect) -> Self {
        Self::with_pattern(bounds, ' ')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_background_creation() {
        let bg = Background::new(Rect::new(0, 0, 80, 25));
        assert_eq!(bg.get_pattern(), '░');
        assert_eq!(bg.bounds(), Rect::new(0, 0, 80, 25));
    }

    #[test]
    fn test_background_custom_pattern() {
        let bg = Background::with_pattern(Rect::new(0, 0, 80, 25), '*');
        assert_eq!(bg.get_pattern(), '*');
    }

    #[test]
    fn test_background_set_pattern() {
        let mut bg = Background::new(Rect::new(0, 0, 80, 25));
        assert_eq!(bg.get_pattern(), '░');

        bg.set_pattern('▒');
        assert_eq!(bg.get_pattern(), '▒');
    }

    #[test]
    fn test_background_predefined_patterns() {
        let bg_light = Background::checkered_light(Rect::new(0, 0, 80, 25));
        assert_eq!(bg_light.get_pattern(), '░');

        let bg_medium = Background::checkered_medium(Rect::new(0, 0, 80, 25));
        assert_eq!(bg_medium.get_pattern(), '▒');

        let bg_dark = Background::checkered_dark(Rect::new(0, 0, 80, 25));
        assert_eq!(bg_dark.get_pattern(), '▓');

        let bg_solid = Background::solid(Rect::new(0, 0, 80, 25));
        assert_eq!(bg_solid.get_pattern(), '█');

        let bg_dots = Background::dots(Rect::new(0, 0, 80, 25));
        assert_eq!(bg_dots.get_pattern(), '·');

        let bg_cross = Background::cross(Rect::new(0, 0, 80, 25));
        assert_eq!(bg_cross.get_pattern(), '┼');

        let bg_blank = Background::blank(Rect::new(0, 0, 80, 25));
        assert_eq!(bg_blank.get_pattern(), ' ');
    }

    #[test]
    fn test_background_set_bounds() {
        let mut bg = Background::new(Rect::new(0, 0, 80, 25));
        assert_eq!(bg.bounds(), Rect::new(0, 0, 80, 25));

        bg.set_bounds(Rect::new(10, 10, 90, 35));
        assert_eq!(bg.bounds(), Rect::new(10, 10, 90, 35));
    }
}
