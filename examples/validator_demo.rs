// Example demonstrating input validation with FilterValidator and RangeValidator
//
// This example shows:
// - FilterValidator: Only allows specific characters (e.g., digits only)
// - RangeValidator: Only allows numbers within a range (e.g., 0-100)
// - Real-time validation: Invalid characters are rejected as you type
// - Final validation: Check if complete input is valid before accepting

use turbo_vision::app::Application;
use turbo_vision::core::command::{CM_OK, CM_CANCEL};
use turbo_vision::core::geometry::Rect;
use turbo_vision::views::dialog::Dialog;
use turbo_vision::views::input_line::InputLine;
use turbo_vision::views::label::Label;
use turbo_vision::views::button::Button;
use turbo_vision::views::static_text::StaticText;
use turbo_vision::views::validator::{FilterValidator, RangeValidator, Validator};
use std::rc::Rc;
use std::cell::RefCell;

fn main() -> std::io::Result<()> {
    let mut app = Application::new()?;
    let (width, height) = app.terminal.size();

    // Create dialog
    let dialog_width = 60;
    let dialog_height = 22;
    let dialog_x = (width as i16 - dialog_width) / 2;
    let dialog_y = (height as i16 - dialog_height) / 2;

    let mut dialog = Dialog::new(
        Rect::new(dialog_x, dialog_y, dialog_x + dialog_width, dialog_y + dialog_height),
        "Input Validation Demo"
    );

    // Instructions
    let instructions = StaticText::new(
        Rect::new(2, 1, dialog_width - 4, 3),
        "Try typing in each field. Invalid characters are rejected.\n\
         Click OK to validate final values.",
    );
    dialog.add(Box::new(instructions));

    // Field 1: Digits only (FilterValidator)
    let label1 = Label::new(Rect::new(2, 4, dialog_width - 4, 5), "Digits only:");
    dialog.add(Box::new(label1));

    let field1_data = Rc::new(RefCell::new(String::from("12345")));
    let field1_validator = Rc::new(RefCell::new(FilterValidator::new("0123456789")));
    let input1 = InputLine::with_validator(
        Rect::new(2, 5, dialog_width - 4, 6),
        20,
        field1_data.clone(),
        field1_validator.clone()
    );
    dialog.add(Box::new(input1));

    // Field 2: Range 0-100 (RangeValidator)
    let label2 = Label::new(Rect::new(2, 7, dialog_width - 4, 8), "Number (0-100):");
    dialog.add(Box::new(label2));

    let field2_data = Rc::new(RefCell::new(String::from("50")));
    let field2_validator = Rc::new(RefCell::new(RangeValidator::new(0, 100)));
    let input2 = InputLine::with_validator(
        Rect::new(2, 8, dialog_width - 4, 9),
        20,
        field2_data.clone(),
        field2_validator.clone()
    );
    dialog.add(Box::new(input2));

    // Field 3: Range -50 to 50 (negative numbers allowed)
    let label3 = Label::new(Rect::new(2, 10, dialog_width - 4, 11), "Number (-50 to 50):");
    dialog.add(Box::new(label3));

    let field3_data = Rc::new(RefCell::new(String::from("-25")));
    let field3_validator = Rc::new(RefCell::new(RangeValidator::new(-50, 50)));
    let input3 = InputLine::with_validator(
        Rect::new(2, 11, dialog_width - 4, 12),
        20,
        field3_data.clone(),
        field3_validator.clone()
    );
    dialog.add(Box::new(input3));

    // Field 4: Hex numbers 0x00-0xFF (RangeValidator with hex support)
    let label4 = Label::new(Rect::new(2, 13, dialog_width - 4, 14), "Hex (0x00-0xFF):");
    dialog.add(Box::new(label4));

    let field4_data = Rc::new(RefCell::new(String::from("0xAB")));
    let field4_validator = Rc::new(RefCell::new(RangeValidator::new(0, 255)));
    let input4 = InputLine::with_validator(
        Rect::new(2, 14, dialog_width - 4, 15),
        20,
        field4_data.clone(),
        field4_validator.clone()
    );
    dialog.add(Box::new(input4));

    // Buttons
    let ok_button = Button::new(
        Rect::new(15, 17, 25, 19),
        "  OK  ",
        CM_OK,
        true
    );
    dialog.add(Box::new(ok_button));

    let cancel_button = Button::new(
        Rect::new(30, 17, 40, 19),
        "Cancel",
        CM_CANCEL,
        false
    );
    dialog.add(Box::new(cancel_button));

    dialog.set_initial_focus();

    // Execute dialog
    let result = dialog.execute(&mut app);

    if result == CM_OK {
        // Validate all fields
        let mut all_valid = true;

        println!("\n\nValidation Results:");
        println!("==================");

        // Field 1
        let field1_text = field1_data.borrow().clone();
        let field1_valid = field1_validator.borrow().is_valid(&field1_text);
        println!("Field 1 (Digits only): \"{}\" - {}", field1_text, if field1_valid { "VALID" } else { "INVALID" });
        all_valid &= field1_valid;

        // Field 2
        let field2_text = field2_data.borrow().clone();
        let field2_valid = field2_validator.borrow().is_valid(&field2_text);
        println!("Field 2 (0-100): \"{}\" - {}", field2_text, if field2_valid { "VALID" } else { "INVALID" });
        all_valid &= field2_valid;

        // Field 3
        let field3_text = field3_data.borrow().clone();
        let field3_valid = field3_validator.borrow().is_valid(&field3_text);
        println!("Field 3 (-50 to 50): \"{}\" - {}", field3_text, if field3_valid { "VALID" } else { "INVALID" });
        all_valid &= field3_valid;

        // Field 4
        let field4_text = field4_data.borrow().clone();
        let field4_valid = field4_validator.borrow().is_valid(&field4_text);
        println!("Field 4 (0x00-0xFF): \"{}\" - {}", field4_text, if field4_valid { "VALID" } else { "INVALID" });
        all_valid &= field4_valid;

        println!("\nOverall: {}", if all_valid { "ALL FIELDS VALID" } else { "SOME FIELDS INVALID" });
    } else {
        println!("\nDialog cancelled");
    }

    Ok(())
}
