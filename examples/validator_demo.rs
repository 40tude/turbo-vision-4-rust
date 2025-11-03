// Comprehensive Validator Demo
//
// Demonstrates all Validator types in one example:
// - FilterValidator (character filtering)
// - RangeValidator (numeric ranges)
// - PictureValidator (format masks)

use turbo_vision::app::Application;
use turbo_vision::core::geometry::Rect;
use turbo_vision::core::command::{CM_OK, CM_CANCEL};
use turbo_vision::views::{
    dialog::Dialog,
    button::Button,
    static_text::StaticText,
    label::Label,
    input_line::InputLine,
    validator::{FilterValidator, RangeValidator, Validator},
    picture_validator::PictureValidator,
};
use std::rc::Rc;
use std::cell::RefCell;

fn main() -> std::io::Result<()> {
    let mut app = Application::new()?;

    // Show menu to choose demo type
    loop {
        let choice = show_menu(&mut app);

        match choice {
            1 => demo_filter_range(&mut app),
            2 => demo_picture_masks(&mut app),
            _ => break,
        }
    }

    Ok(())
}

fn show_menu(app: &mut Application) -> u16 {
    let mut dialog = Dialog::new(
        Rect::new(20, 8, 60, 16),
        "Validator Demonstrations"
    );

    let text = StaticText::new(
        Rect::new(2, 2, 38, 3),
        "Choose a validator type to demonstrate:"
    );
    dialog.add(Box::new(text));

    let btn1 = Button::new(
        Rect::new(3, 4, 37, 6),
        "1. ~F~ilter and Range Validators",
        1,
        true
    );
    dialog.add(Box::new(btn1));

    let btn2 = Button::new(
        Rect::new(3, 6, 37, 8),
        "2. ~P~icture Mask Validator",
        2,
        false
    );
    dialog.add(Box::new(btn2));

    dialog.execute(app)
}

fn demo_filter_range(app: &mut Application) {
    let (width, height) = app.terminal.size();

    // Create dialog
    let dialog_width = 60;
    let dialog_height = 22;
    let dialog_x = (width as i16 - dialog_width) / 2;
    let dialog_y = (height as i16 - dialog_height) / 2;

    let mut dialog = Dialog::new(
        Rect::new(dialog_x, dialog_y, dialog_x + dialog_width, dialog_y + dialog_height),
        "Filter & Range Validators"
    );

    // Instructions
    let instructions = StaticText::new(
        Rect::new(2, 1, dialog_width - 4, 3),
        "Try typing in each field. Invalid characters are rejected.\nClick OK to validate final values.",
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
    let result = dialog.execute(app);

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
}

fn demo_picture_masks(app: &mut Application) {
    // Create dialog
    let mut dialog = Dialog::new(
        Rect::new(10, 4, 70, 20),
        "Picture Mask Validator"
    );

    // Title
    let title = StaticText::new(
        Rect::new(12, 6, 68, 7),
        "Enter formatted data using picture masks:"
    );
    dialog.add(Box::new(title));

    // Phone number field with validator
    let phone_label = Label::new(
        Rect::new(12, 8, 28, 9),
        "~P~hone Number:"
    );
    dialog.add(Box::new(phone_label));

    let phone_data = Rc::new(RefCell::new(String::new()));
    let mut phone_input = InputLine::new(Rect::new(29, 8, 52, 9), 20, phone_data.clone());
    phone_input.set_validator(
        Rc::new(RefCell::new(
            PictureValidator::new("(###) ###-####")
        ))
    );
    dialog.add(Box::new(phone_input));

    let phone_hint = StaticText::new(
        Rect::new(53, 8, 68, 9),
        "(###) ###-####"
    );
    dialog.add(Box::new(phone_hint));

    // Date field with validator
    let date_label = Label::new(
        Rect::new(12, 10, 28, 11),
        "~D~ate:"
    );
    dialog.add(Box::new(date_label));

    let date_data = Rc::new(RefCell::new(String::new()));
    let mut date_input = InputLine::new(Rect::new(29, 10, 41, 11), 10, date_data.clone());
    date_input.set_validator(
        Rc::new(RefCell::new(
            PictureValidator::new("##/##/####")
        ))
    );
    dialog.add(Box::new(date_input));

    let date_hint = StaticText::new(
        Rect::new(42, 10, 68, 11),
        "##/##/####"
    );
    dialog.add(Box::new(date_hint));

    // Product code field
    let code_label = Label::new(
        Rect::new(12, 12, 28, 13),
        "Product ~C~ode:"
    );
    dialog.add(Box::new(code_label));

    let code_data = Rc::new(RefCell::new(String::new()));
    let mut code_input = InputLine::new(Rect::new(29, 12, 42, 13), 9, code_data.clone());
    code_input.set_validator(
        Rc::new(RefCell::new(
            PictureValidator::new("@@@@-####")
        ))
    );
    dialog.add(Box::new(code_input));

    let code_hint = StaticText::new(
        Rect::new(43, 12, 68, 13),
        "@@@@-####"
    );
    dialog.add(Box::new(code_hint));

    // Explanation text
    let legend = StaticText::new(
        Rect::new(12, 14, 68, 16),
        "Legend: # = digit, @ = letter, ! = any\nLiterals (like /, -, ()) are inserted automatically"
    );
    dialog.add(Box::new(legend));

    // OK button
    let ok_button = Button::new(
        Rect::new(25, 17, 35, 19),
        "  ~O~K  ",
        CM_OK,
        true
    );
    dialog.add(Box::new(ok_button));

    // Cancel button
    let cancel_button = Button::new(
        Rect::new(37, 17, 47, 19),
        "Cancel",
        CM_CANCEL,
        false
    );
    dialog.add(Box::new(cancel_button));

    // Execute dialog
    let result = dialog.execute(app);

    if result == CM_OK {
        println!("\n\nFormatted Data Entered:");
        println!("======================");
        println!("Phone: {}", phone_data.borrow());
        println!("Date: {}", date_data.borrow());
        println!("Code: {}", code_data.borrow());
    }
}
