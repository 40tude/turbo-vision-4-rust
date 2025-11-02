use crate::core::geometry::Rect;
use crate::core::event::{Event, EventType, KB_ESC_ESC, KB_ENTER};
use crate::core::command::{CommandId, CM_CANCEL, CM_CLOSE};
use crate::terminal::Terminal;
use super::view::View;
use super::window::Window;
use std::time::Duration;

pub struct Dialog {
    window: Window,
    result: CommandId,
}

impl Dialog {
    pub fn new(bounds: Rect, title: &str) -> Self {
        Self {
            window: Window::new(bounds, title),
            result: CM_CANCEL,
        }
    }

    pub fn add(&mut self, view: Box<dyn View>) {
        self.window.add(view);
    }

    pub fn set_initial_focus(&mut self) {
        self.window.set_initial_focus();
    }

    /// Set focus to a specific child by index
    /// Matches Borland: owner->setCurrent(this, normalSelect)
    pub fn set_focus_to_child(&mut self, index: usize) {
        self.window.set_focus_to_child(index);
    }

    /// Get the number of child views
    pub fn child_count(&self) -> usize {
        self.window.child_count()
    }

    /// Get a reference to a child view by index
    pub fn child_at(&self, index: usize) -> &dyn View {
        self.window.child_at(index)
    }

    /// Get a mutable reference to a child view by index
    pub fn child_at_mut(&mut self, index: usize) -> &mut dyn View {
        self.window.child_at_mut(index)
    }

    pub fn execute(&mut self, app: &mut crate::app::Application) -> CommandId {
        use crate::core::state::SF_MODAL;

        self.result = CM_CANCEL;

        // Set modal flag - dialogs with their own event loop are modal
        // Matches Borland: TDialog::execute() is modal (tdialog.cc)
        let old_state = self.state();
        self.set_state(old_state | SF_MODAL);

        loop {
            // Set dialog as the active view for F11 dumps
            app.terminal.set_active_view_bounds(self.shadow_bounds());

            // Draw desktop first (background), then dialog on top
            // This matches Borland's pattern where TProgram::getEvent() triggers full screen redraw
            //
            // **Architecture Note**: In Borland TV, there is ONE event loop in TProgram, and
            // TGroup::execView() just calls p->execute() which returns immediately. The modal
            // flag blocks events from reaching views behind the modal view, but drawing happens
            // at the TProgram level.
            //
            // In our Rust implementation, Dialog::execute() has its own event loop for simplicity
            // (Rust ownership makes it difficult to have TProgram handle modal execution).
            // Therefore, we must draw the desktop here to match Borland's behavior and prevent
            // trails when the dialog moves.
            app.desktop.draw(&mut app.terminal);
            self.draw(&mut app.terminal);
            self.update_cursor(&mut app.terminal);
            let _ = app.terminal.flush();

            // Get event
            if let Ok(Some(mut event)) = app.terminal.poll_event(Duration::from_millis(50)) {
                // Double ESC closes the dialog
                if event.what == EventType::Keyboard && event.key_code == KB_ESC_ESC {
                    self.result = CM_CANCEL;
                    break;
                }

                self.handle_event(&mut event);

                // Check if dialog should close
                if event.what == EventType::Command {
                    // CM_CLOSE from close button should be treated as CM_CANCEL
                    if event.command == CM_CLOSE {
                        self.result = CM_CANCEL;
                    } else {
                        self.result = event.command;
                    }
                    break;
                }
            }
        }

        // Restore previous state (clear modal flag)
        self.set_state(old_state);

        self.result
    }
}

impl View for Dialog {
    fn bounds(&self) -> Rect {
        self.window.bounds()
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.window.set_bounds(bounds);
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        self.window.draw(terminal);
    }

    fn update_cursor(&self, terminal: &mut Terminal) {
        self.window.update_cursor(terminal);
    }

    fn handle_event(&mut self, event: &mut Event) {
        // First let the window (and its children) handle the event
        // This is critical: if a focused Memo/Editor handles Enter, it will clear the event
        // Borland's TDialog calls TWindow::handleEvent() FIRST (tdialog.cc line 47)
        self.window.handle_event(event);

        // Now check if the event is still active after children processed it
        // If a child (like Memo/Editor) handled Enter, event.what will be EventType::None
        // This matches Borland's TDialog architecture (tdialog.cc lines 48-86)
        match event.what {
            EventType::Keyboard => {
                match event.key_code {
                    KB_ESC_ESC => {
                        // Double ESC generates cancel command (lines 53-58)
                        *event = Event::command(CM_CANCEL);
                    }
                    KB_ENTER => {
                        // Enter key activates default button (lines 60-66)
                        // Borland converts to evBroadcast + cmDefault and re-queues
                        // We simplify by directly activating the default button
                        if let Some(cmd) = self.find_default_button_command() {
                            *event = Event::command(cmd);
                        } else {
                            event.clear();
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn state(&self) -> crate::core::state::StateFlags {
        self.window.state()
    }

    fn set_state(&mut self, state: crate::core::state::StateFlags) {
        self.window.set_state(state);
    }
}

impl Dialog {
    /// Find the default button and return its command if it's enabled
    /// Returns None if no default button found or if it's disabled
    /// Matches Borland's TButton::handleEvent() cmDefault broadcast handling (tbutton.cc lines 238-244)
    fn find_default_button_command(&self) -> Option<CommandId> {
        for i in 0..self.child_count() {
            let child = self.child_at(i);
            if child.is_default_button() {
                // Check if the button can receive focus (i.e., not disabled)
                // Borland checks: amDefault && !(state & sfDisabled)
                if child.can_focus() {
                    return child.button_command();
                } else {
                    // Default button is disabled
                    return None;
                }
            }
        }
        None
    }
}
