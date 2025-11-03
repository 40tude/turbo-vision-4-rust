// File Editor Example
//
// Demonstrates:
// - Editor with file load/save capabilities
// - Search and replace functionality
// - Modified flag tracking
// - Undo/redo operations
//
// Controls:
// - Arrow keys: Move cursor
// - Ctrl+A: Select all
// - Ctrl+C/X/V: Copy/Cut/Paste
// - Ctrl+Z/Y: Undo/Redo
// - ESC: Exit

use turbo_vision::app::Application;
use turbo_vision::core::geometry::Rect;
use turbo_vision::views::editor::{Editor, SearchOptions};
use turbo_vision::views::view::View;
use turbo_vision::views::static_text::StaticText;

fn main() -> std::io::Result<()> {
    let mut app = Application::new()?;
    let terminal_size = app.terminal.size();

    // Create status line at bottom
    let status_bounds = Rect::new(0, terminal_size.1 as i16 - 1, terminal_size.0 as i16, terminal_size.1 as i16);
    let mut status = StaticText::new(
        status_bounds,
        " File Editor Demo | Ctrl+Z/Y: Undo/Redo | Ctrl+C/X/V: Copy/Cut/Paste | ESC: Exit",
    );

    // Create editor (leave room for status line)
    let editor_bounds = Rect::new(0, 0, terminal_size.0 as i16, terminal_size.1 as i16 - 1);
    let mut editor = Editor::new(editor_bounds).with_scrollbars_and_indicator();
    editor.set_focus(true);

    // Load this source file as an example
    let source_file = file!();
    match editor.load_file(source_file) {
        Ok(_) => {
            // Demonstrate search functionality - find and highlight first occurrence
            let _ = editor.find("Editor", SearchOptions::new());
        }
        Err(e) => {
            eprintln!("Error loading file: {}", e);
            editor.set_text("// Failed to load file\n// Starting with empty editor");
        }
    }

    // Event loop
    use std::time::Duration;
    use turbo_vision::core::event::EventType;

    loop {
        // Draw the editor and status
        editor.draw(&mut app.terminal);
        editor.update_cursor(&mut app.terminal);
        status.draw(&mut app.terminal);
        let _ = app.terminal.flush();

        // Handle events
        if let Ok(Some(mut event)) = app.terminal.poll_event(Duration::from_millis(50)) {
            // Let editor handle the event
            editor.handle_event(&mut event);

            // Check for ESC to exit
            if event.what == EventType::Keyboard && event.key_code == 0x001B {
                break;
            }
        }
    }

    Ok(())
}
