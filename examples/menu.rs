// Simple example demonstrating a basic menu bar structure

use turbo_vision::app::Application;
use turbo_vision::core::command::{CM_QUIT, CM_NEW, CM_OPEN};
use turbo_vision::core::event::{EventType, KB_F10};
use turbo_vision::core::geometry::Rect;
use turbo_vision::views::button::Button;
use turbo_vision::views::dialog::Dialog;
use turbo_vision::views::menu_bar::{MenuBar, MenuItem, SubMenu};
use turbo_vision::views::static_text::StaticText;
use turbo_vision::views::status_line::{StatusItem, StatusLine};
use turbo_vision::views::View;

// Custom command IDs for this example
const CMD_ABOUT: u16 = 100;

fn main() -> std::io::Result<()> {
    let mut app = Application::new()?;
    let (width, height) = app.terminal.size();

    // Create menu bar
    let mut menu_bar = MenuBar::new(Rect::new(0, 0, width as i16, 1));

    // File menu (with keyboard shortcuts displayed)
    let mut file_menu = SubMenu::new("~F~ile");
    file_menu.add_item(MenuItem::new_with_shortcut("~N~ew", CM_NEW, 0, "Ctrl+N"));
    file_menu.add_item(MenuItem::new_with_shortcut("~O~pen...", CM_OPEN, 0, "Ctrl+O"));
    file_menu.add_item(MenuItem::separator());
    file_menu.add_item(MenuItem::new_with_shortcut("E~x~it", CM_QUIT, 0, "Alt+X"));

    // Help menu
    let mut help_menu = SubMenu::new("~H~elp");
    help_menu.add_item(MenuItem::new_with_shortcut("~A~bout", CMD_ABOUT, 0, "F1"));

    menu_bar.add_menu(file_menu);
    menu_bar.add_menu(help_menu);
    app.set_menu_bar(menu_bar);

    // Create status line
    let status_line = StatusLine::new(
        Rect::new(0, height as i16 - 1, width as i16, height as i16),
        vec![
            StatusItem::new("~F10~ Exit", KB_F10, CM_QUIT),
            StatusItem::new("~F1~ Help", 0, 0),
        ],
    );
    app.set_status_line(status_line);

    // Draw initial screen
    app.desktop.draw(&mut app.terminal);
    if let Some(ref mut menu_bar) = app.menu_bar {
        menu_bar.draw(&mut app.terminal);
    }
    if let Some(ref mut status_line) = app.status_line {
        status_line.draw(&mut app.terminal);
    }
    let _ = app.terminal.flush();

    // Show about dialog on startup
    show_about(&mut app);

    // Redraw after dialog closes
    app.desktop.draw(&mut app.terminal);
    if let Some(ref mut menu_bar) = app.menu_bar {
        menu_bar.draw(&mut app.terminal);
    }
    if let Some(ref mut status_line) = app.status_line {
        status_line.draw(&mut app.terminal);
    }
    let _ = app.terminal.flush();

    // Event loop
    app.running = true;
    while app.running {
        // Draw everything
        app.desktop.draw(&mut app.terminal);
        if let Some(ref mut menu_bar) = app.menu_bar {
            menu_bar.draw(&mut app.terminal);
        }
        if let Some(ref mut status_line) = app.status_line {
            status_line.draw(&mut app.terminal);
        }
        let _ = app.terminal.flush();

        // Poll for events
        if let Ok(Some(mut event)) = app
            .terminal
            .poll_event(std::time::Duration::from_millis(50))
        {
            // Menu bar handles events first
            if let Some(ref mut menu_bar) = app.menu_bar {
                menu_bar.handle_event(&mut event);
            }

            // Redraw before showing dialog
            if event.what == EventType::Command {
                app.desktop.draw(&mut app.terminal);
                if let Some(ref mut menu_bar) = app.menu_bar {
                    menu_bar.draw(&mut app.terminal);
                }
                if let Some(ref mut status_line) = app.status_line {
                    status_line.draw(&mut app.terminal);
                }
                let _ = app.terminal.flush();
            }

            // Handle commands
            if event.what == EventType::Command {
                match event.command {
                    CM_QUIT => {
                        app.running = false;
                    }
                    CM_NEW => {
                        show_message(&mut app, "New", "Create a new file");
                    }
                    CM_OPEN => {
                        show_message(&mut app, "Open", "Open an existing file");
                    }
                    CMD_ABOUT => {
                        show_about(&mut app);
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn show_message(app: &mut Application, title: &str, message: &str) {
    let (term_width, term_height) = app.terminal.size();

    // Dialog dimensions
    let dialog_width = 40;
    let dialog_height = 7;

    // Center dialog on screen
    let dialog_x = (term_width as i16 - dialog_width) / 2;
    let dialog_y = (term_height as i16 - dialog_height) / 2;

    let mut dialog = Dialog::new(
        Rect::new(dialog_x, dialog_y, dialog_x + dialog_width, dialog_y + dialog_height),
        title
    );

    // Text positioned relative to dialog interior (coordinates are relative)
    let text_width = dialog_width - 4;  // Leave margin
    let text = StaticText::new_centered(
        Rect::new(2, 1, text_width, 2),
        message
    );
    dialog.add(Box::new(text));

    // Center button horizontally
    let button_width = 10;
    let button_x = (dialog_width - 2 - button_width) / 2;  // -2 for frame
    let button = Button::new(Rect::new(button_x, 3, button_x + button_width, 5), "  ~O~K  ", 0, true);
    dialog.add(Box::new(button));
    dialog.set_initial_focus();

    dialog.execute(app);
}

fn show_about(app: &mut Application) {
    let (term_width, term_height) = app.terminal.size();

    // Dialog dimensions
    let dialog_width = 50;
    let dialog_height = 10;

    // Center dialog on screen
    let dialog_x = (term_width as i16 - dialog_width) / 2;
    let dialog_y = (term_height as i16 - dialog_height) / 2;

    let mut dialog = Dialog::new(
        Rect::new(dialog_x, dialog_y, dialog_x + dialog_width, dialog_y + dialog_height),
        "Turbo Vision for Rust"
    );

    // Text positioned relative to dialog interior (coordinates are relative)
    let text_width = dialog_width - 4;  // Leave margin
    let text = StaticText::new_centered(
        Rect::new(2, 1, text_width, 5),
        "Welcome To Turbo Vision\n\nfor Rust!\n\nSimple Menu Example",
    );
    dialog.add(Box::new(text));

    // Center button horizontally
    let button_width = 10;
    let button_x = (dialog_width - 2 - button_width) / 2;  // -2 for frame
    let button = Button::new(Rect::new(button_x, 6, button_x + button_width, 8), "  ~O~K  ", 0, true);
    dialog.add(Box::new(button));
    dialog.set_initial_focus();

    dialog.execute(app);
}
