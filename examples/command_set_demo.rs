//! Command Set System Demo
//!
//! Demonstrates automatic button enable/disable based on application state.
//! Shows how the command set system works like Borland Turbo Vision.
//!
//! ## What This Demo Shows:
//!
//! 1. Buttons are automatically created in disabled state if their command is disabled
//! 2. The Application idle() loop broadcasts CM_COMMAND_SET_CHANGED when commands change
//! 3. Buttons automatically update their disabled state when they receive the broadcast
//!
//! ## Try It:
//!
//! - Notice that Copy, Cut, Paste, Undo, Redo buttons start DISABLED (gray)
//! - Press 'E' to enable edit commands - buttons turn GREEN
//! - Press 'D' to disable them again - buttons turn GRAY
//! - This happens automatically through the command set broadcast system!

use turbo_vision::app::Application;
use turbo_vision::core::command::{CommandId, CM_CANCEL, CM_COPY, CM_CUT, CM_PASTE, CM_UNDO, CM_REDO};
use turbo_vision::core::command_set;
use turbo_vision::core::event::{Event, EventType};
use turbo_vision::core::geometry::Rect;
use turbo_vision::views::{View, dialog::Dialog, button::Button, static_text::StaticText};
use turbo_vision::terminal::Terminal;

// Custom commands for this demo
const CMD_ENABLE_EDITS: CommandId = 200;
const CMD_DISABLE_EDITS: CommandId = 201;

fn main() -> std::io::Result<()> {
    let mut app = Application::new()?;

    // Initially disable clipboard and undo commands
    // (Simulating empty clipboard and no history)
    command_set::disable_command(CM_COPY);
    command_set::disable_command(CM_CUT);
    command_set::disable_command(CM_PASTE);
    command_set::disable_command(CM_UNDO);
    command_set::disable_command(CM_REDO);

    // Create dialog
    let mut dialog = Dialog::new(
        Rect::new(10, 4, 70, 21),
        "Command Set Demo - Automatic Button Enable/Disable"
    );

    // Instructions
    let instructions = StaticText::new(
        Rect::new(2, 1, 56, 6),
        "This demo shows AUTOMATIC button enable/disable!\n\
         \n\
         Edit commands start DISABLED (gray).\n\
         Press ~E~ to Enable edits - buttons turn GREEN!\n\
         Press ~D~ to Disable edits - buttons turn GRAY!"
    );
    dialog.add(Box::new(instructions));

    // Edit command buttons (will be initially disabled due to command set)
    let cut_button = Button::new(
        Rect::new(2, 7, 14, 9),
        " C~u~t ",
        CM_CUT,
        false,
    );
    dialog.add(Box::new(cut_button));

    let copy_button = Button::new(
        Rect::new(15, 7, 27, 9),
        " ~C~opy ",
        CM_COPY,
        false,
    );
    dialog.add(Box::new(copy_button));

    let paste_button = Button::new(
        Rect::new(28, 7, 40, 9),
        " ~P~aste ",
        CM_PASTE,
        false,
    );
    dialog.add(Box::new(paste_button));

    let undo_button = Button::new(
        Rect::new(2, 10, 14, 12),
        " ~U~ndo ",
        CM_UNDO,
        false,
    );
    dialog.add(Box::new(undo_button));

    let redo_button = Button::new(
        Rect::new(15, 10, 27, 12),
        " ~R~edo ",
        CM_REDO,
        false,
    );
    dialog.add(Box::new(redo_button));

    // Control buttons
    let enable_button = Button::new(
        Rect::new(42, 7, 56, 9),
        "~E~nable Edits",
        CMD_ENABLE_EDITS,
        false,
    );
    dialog.add(Box::new(enable_button));

    let disable_button = Button::new(
        Rect::new(42, 10, 56, 12),
        "~D~isable Edits",
        CMD_DISABLE_EDITS,
        false,
    );
    dialog.add(Box::new(disable_button));

    // Close button
    let close_button = Button::new(
        Rect::new(22, 13, 38, 15),
        "  Close  ",
        CM_CANCEL,
        true,
    );
    dialog.add(Box::new(close_button));

    // Execute the dialog with custom event handling
    let result = execute_demo_dialog(&mut dialog, &mut app.terminal);

    // Clean up before exit
    drop(dialog);
    println!("\nDemo Result: Command {:?}", result);
    println!("Notice how buttons automatically updated when commands were enabled/disabled!");
    Ok(())
}

fn execute_demo_dialog(dialog: &mut Dialog, terminal: &mut Terminal) -> CommandId {
    loop {
        // Draw
        dialog.draw(terminal);
        dialog.update_cursor(terminal);
        let _ = terminal.flush();

        // Get event
        if let Ok(Some(mut event)) = terminal.poll_event(std::time::Duration::from_millis(50)) {
            // Handle double ESC to close
            if event.what == EventType::Keyboard && event.key_code == 0x011C {
                return CM_CANCEL;
            }

            dialog.handle_event(&mut event);

            // Check if dialog should close
            if event.what == EventType::Command {
                match event.command {
                    CMD_ENABLE_EDITS => {
                        // Enable all edit commands - buttons will auto-update!
                        command_set::enable_command(CM_COPY);
                        command_set::enable_command(CM_CUT);
                        command_set::enable_command(CM_PASTE);
                        command_set::enable_command(CM_UNDO);
                        command_set::enable_command(CM_REDO);
                        event.clear();
                    }
                    CMD_DISABLE_EDITS => {
                        // Disable all edit commands - buttons will auto-update!
                        command_set::disable_command(CM_COPY);
                        command_set::disable_command(CM_CUT);
                        command_set::disable_command(CM_PASTE);
                        command_set::disable_command(CM_UNDO);
                        command_set::disable_command(CM_REDO);
                        event.clear();
                    }
                    CM_CANCEL => {
                        return CM_CANCEL;
                    }
                    _ => {}
                }
            }
        }

        // CRITICAL: Call idle to broadcast command set changes
        // This is what triggers the automatic button updates!
        if command_set::command_set_changed() {
            let mut broadcast_event = Event::broadcast(
                turbo_vision::core::command::CM_COMMAND_SET_CHANGED
            );
            dialog.handle_event(&mut broadcast_event);
            command_set::clear_command_set_changed();
        }
    }
}
