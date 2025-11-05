// (C) 2025 - Enzo Lombardi

//! Turbo Vision - A modern Rust implementation of the classic Turbo Vision TUI framework.
//! Provides a complete toolkit for building text-based user interfaces with windows, dialogs,
//! menus, and controls matching the original Borland Turbo Vision architecture.

// Core modules
pub mod core;
pub mod terminal;
pub mod views;
pub mod app;

// Re-export commonly used types
pub mod prelude {
    pub use crate::core::geometry::{Point, Rect};
    pub use crate::core::event::{Event, EventType, KeyCode};
    pub use crate::core::command::*;
    pub use crate::views::View;
    pub use crate::app::Application;
}
