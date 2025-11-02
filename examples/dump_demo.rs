use turbo_vision::prelude::*;
use turbo_vision::app::Application;
use turbo_vision::views::{
    dialog::Dialog,
    button::Button,
    static_text::StaticText,
    input_line::InputLine,
    checkbox::CheckBox,
    View,
};
use std::rc::Rc;
use std::cell::RefCell;

fn main() -> std::io::Result<()> {
    let mut app = Application::new()?;

    // Create a dialog with various controls
    let mut dialog = Dialog::new(
        Rect::new(10, 5, 70, 20),
        "ANSI Dump Demo"
    );

    // Add some text (coordinates are relative to dialog interior)
    let text = StaticText::new(
        Rect::new(1, 1, 57, 3),
        "This dialog will be dumped to ANSI files for debugging.\nYou can view them with 'cat' or any text editor."
    );
    dialog.add(Box::new(text));

    // Add an input field
    let input_data = Rc::new(RefCell::new("Sample Input Text".to_string()));
    let input = InputLine::new(
        Rect::new(1, 4, 39, 5),
        50,
        input_data
    );
    dialog.add(Box::new(input));

    // Add checkboxes
    let checkbox1 = CheckBox::new(
        Rect::new(1, 6, 29, 7),
        "Enable feature A"
    );
    dialog.add(Box::new(checkbox1));

    let checkbox2 = CheckBox::new(
        Rect::new(1, 7, 29, 8),
        "Enable feature B"
    );
    dialog.add(Box::new(checkbox2));

    // Add buttons
    let ok_button = Button::new(
        Rect::new(14, 10, 24, 12),
        "  OK  ",
        CM_OK,
        true
    );
    dialog.add(Box::new(ok_button));

    let cancel_button = Button::new(
        Rect::new(29, 10, 41, 12),
        " Cancel ",
        CM_CANCEL,
        false
    );
    dialog.add(Box::new(cancel_button));

    // Draw the dialog
    dialog.draw(&mut app.terminal);
    app.terminal.flush()?;

    // Dump the entire screen to a file
    println!("Dumping entire screen to 'screen_dump.ans'...");
    app.terminal.dump_screen("screen_dump.ans")?;

    // Dump just the dialog region to a file
    println!("Dumping dialog region to 'dialog_dump.ans'...");
    dialog.dump_to_file(&app.terminal, "dialog_dump.ans")?;

    // Dump a specific region (just the buttons)
    println!("Dumping button region to 'buttons_dump.ans'...");
    app.terminal.dump_region(25, 16, 27, 2, "buttons_dump.ans")?;

    println!("\nDumps created successfully!");
    println!("View them with: cat screen_dump.ans");
    println!("                cat dialog_dump.ans");
    println!("                cat buttons_dump.ans");

    // Keep the dialog visible for a moment so users can see it
    println!("\nPress any key to continue...");
    app.terminal.read_event()?;

    Ok(())
}
