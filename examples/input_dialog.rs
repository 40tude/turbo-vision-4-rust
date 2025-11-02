use turbo_vision::prelude::*;
use turbo_vision::app::Application;
use turbo_vision::views::{
    dialog::Dialog,
    button::Button,
    static_text::StaticText,
    input_line::InputLine,
};
use std::rc::Rc;
use std::cell::RefCell;

fn main() -> std::io::Result<()> {
    let mut app = Application::new()?;

    // Create shared data storage for input fields
    let name_data = Rc::new(RefCell::new(String::new()));
    let email_data = Rc::new(RefCell::new(String::new()));
    let age_data = Rc::new(RefCell::new(String::new()));

    // Create a dialog with input fields and buttons
    // Dialog interior is 48 wide x 14 tall (excluding frame)
    let mut dialog = Dialog::new(
        Rect::new(15, 5, 65, 19),
        "User Information"
    );

    // Add labels and input fields (relative coordinates)
    let name_label = StaticText::new(
        Rect::new(2, 1, 12, 2),
        "Name:"
    );
    dialog.add(Box::new(name_label));

    let name_input = InputLine::new(
        Rect::new(13, 1, 46, 2),
        32,
        name_data.clone()
    );
    dialog.add(Box::new(name_input));

    let email_label = StaticText::new(
        Rect::new(2, 3, 12, 4),
        "Email:"
    );
    dialog.add(Box::new(email_label));

    let email_input = InputLine::new(
        Rect::new(13, 3, 46, 4),
        32,
        email_data.clone()
    );
    dialog.add(Box::new(email_input));

    let age_label = StaticText::new(
        Rect::new(2, 5, 12, 6),
        "Age:"
    );
    dialog.add(Box::new(age_label));

    let age_input = InputLine::new(
        Rect::new(13, 5, 20, 6),
        3,
        age_data.clone()
    );
    dialog.add(Box::new(age_input));

    // Add some help text
    let help_text = StaticText::new(
        Rect::new(2, 8, 46, 10),
        "Tab: next field  Shift+Tab: previous\nDouble ESC to cancel"
    );
    dialog.add(Box::new(help_text));

    // Add OK button (default)
    let ok_button = Button::new(
        Rect::new(10, 11, 20, 13),
        "  ~O~K  ",
        CM_OK,
        true
    );
    dialog.add(Box::new(ok_button));

    // Add Cancel button
    let cancel_button = Button::new(
        Rect::new(28, 11, 38, 13),
        "~C~ancel",
        CM_CANCEL,
        false
    );
    dialog.add(Box::new(cancel_button));

    // Set focus to the first input field
    dialog.set_initial_focus();

    // Execute the dialog
    let result = dialog.execute(&mut app.terminal);

    // Clean up terminal before printing
    drop(app);

    // Print the result
    match result {
        CM_OK => {
            println!("User clicked OK");
            println!("Name: {}", name_data.borrow());
            println!("Email: {}", email_data.borrow());
            println!("Age: {}", age_data.borrow());
        },
        CM_CANCEL => println!("User cancelled the dialog"),
        _ => println!("Dialog closed with result: {}", result),
    }

    Ok(())
}
