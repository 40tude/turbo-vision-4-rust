use crate::core::geometry::Rect;
use crate::core::event::{Event, EventType, KB_F10, KB_ALT_X, KB_ESC_X};
use crate::core::command::CM_QUIT;
use crate::terminal::Terminal;
use crate::views::{View, menu_bar::MenuBar, status_line::StatusLine, desktop::Desktop};
use std::time::Duration;

pub struct Application {
    pub terminal: Terminal,
    pub menu_bar: Option<MenuBar>,
    pub status_line: Option<StatusLine>,
    pub desktop: Desktop,
    pub running: bool,
}

impl Application {
    pub fn new() -> std::io::Result<Self> {
        let terminal = Terminal::init()?;
        let (width, height) = terminal.size();

        let desktop = Desktop::new(Rect::new(0, 1, width as i16, height as i16 - 1));

        Ok(Self {
            terminal,
            menu_bar: None,
            status_line: None,
            desktop,
            running: false,
        })
    }

    pub fn set_menu_bar(&mut self, menu_bar: MenuBar) {
        self.menu_bar = Some(menu_bar);
    }

    pub fn set_status_line(&mut self, status_line: StatusLine) {
        self.status_line = Some(status_line);
    }

    pub fn run(&mut self) {
        self.running = true;

        while self.running {
            // Update active view bounds for F11 dumps
            self.update_active_view_bounds();

            // Draw everything
            self.draw();
            let _ = self.terminal.flush();

            // Handle events
            if let Ok(Some(mut event)) = self.terminal.poll_event(Duration::from_millis(50)) {
                self.handle_event(&mut event);
            }
        }
    }

    fn update_active_view_bounds(&mut self) {
        // The active view is the topmost window on the desktop (last child with shadow)
        // Get the focused child from the desktop
        let child_count = self.desktop.child_count();
        if child_count > 0 {
            let last_child = self.desktop.child_at(child_count - 1);
            self.terminal.set_active_view_bounds(last_child.shadow_bounds());
        } else {
            self.terminal.clear_active_view_bounds();
        }
    }

    fn draw(&mut self) {
        // Draw desktop first, then menu bar on top (so dropdown appears over desktop)
        self.desktop.draw(&mut self.terminal);

        if let Some(ref mut menu_bar) = self.menu_bar {
            menu_bar.draw(&mut self.terminal);
        }

        if let Some(ref mut status_line) = self.status_line {
            status_line.draw(&mut self.terminal);
        }

        // Update cursor after drawing all views
        // Desktop contains windows/dialogs with focused controls
        self.desktop.update_cursor(&mut self.terminal);
    }

    fn handle_event(&mut self, event: &mut Event) {
        // Menu bar gets first shot
        if let Some(ref mut menu_bar) = self.menu_bar {
            menu_bar.handle_event(event);
            if event.what == EventType::Nothing {
                return;
            }
        }

        // Desktop/windows
        self.desktop.handle_event(event);
        if event.what == EventType::Nothing {
            return;
        }

        // Status line
        if let Some(ref mut status_line) = self.status_line {
            status_line.handle_event(event);
            if event.what == EventType::Nothing {
                return;
            }
        }

        // Application-level command handling
        if event.what == EventType::Command && event.command == CM_QUIT {
            self.running = false;
            event.clear();
        }

        // Handle Ctrl+C, F10, Alt+X, and ESC+X at application level
        if event.what == EventType::Keyboard
            && (event.key_code == 0x0003
                || event.key_code == KB_F10
                || event.key_code == KB_ALT_X
                || event.key_code == KB_ESC_X)
        {
            // Treat these as quit command
            *event = Event::command(CM_QUIT);
            self.running = false;
        }
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        let _ = self.terminal.shutdown();
    }
}
