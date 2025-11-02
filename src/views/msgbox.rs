use crate::core::command::{CommandId, CM_OK, CM_CANCEL};
use crate::core::geometry::Rect;
use crate::terminal::Terminal;
use super::dialog::Dialog;
use super::button::Button;
use super::static_text::StaticText;
use super::label::Label;
use super::input_line::InputLine;
use std::rc::Rc;
use std::cell::RefCell;

// Message box types
pub const MF_WARNING: u16 = 0x0000;
pub const MF_ERROR: u16 = 0x0001;
pub const MF_INFORMATION: u16 = 0x0002;
pub const MF_CONFIRMATION: u16 = 0x0003;

// Button flags
pub const MF_YES_BUTTON: u16 = 0x0100;
pub const MF_NO_BUTTON: u16 = 0x0200;
pub const MF_OK_BUTTON: u16 = 0x0400;
pub const MF_CANCEL_BUTTON: u16 = 0x0800;

// Combined flags
pub const MF_YES_NO_CANCEL: u16 = MF_YES_BUTTON | MF_NO_BUTTON | MF_CANCEL_BUTTON;
pub const MF_OK_CANCEL: u16 = MF_OK_BUTTON | MF_CANCEL_BUTTON;

// Command IDs for message box buttons
pub const CM_YES: CommandId = 100;
pub const CM_NO: CommandId = 101;

/// Display a message box with the given message and options
pub fn message_box(terminal: &mut Terminal, message: &str, options: u16) -> CommandId {
    // Calculate dialog size based on message
    let msg_width = message.lines().map(|l| l.len()).max().unwrap_or(20);
    let msg_height = message.lines().count().max(1);

    let width = (msg_width + 6).min(60).max(30);
    let height = msg_height + 6;

    // Center on screen
    let (screen_w, screen_h) = terminal.size();
    let x = (screen_w as i16 - width as i16) / 2;
    let y = (screen_h as i16 - height as i16) / 2;

    let bounds = Rect::new(x, y, x + width as i16, y + height as i16);

    message_box_rect(terminal, bounds, message, options)
}

/// Display a message box at a specific location
pub fn message_box_rect(terminal: &mut Terminal, bounds: Rect, message: &str, options: u16) -> CommandId {
    // Determine title based on message type
    let title = match options & 0x03 {
        MF_WARNING => "Warning",
        MF_ERROR => "Error",
        MF_INFORMATION => "Information",
        MF_CONFIRMATION => "Confirm",
        _ => "Message",
    };

    let mut dialog = Dialog::new(bounds, title);

    // Add static text with message
    let text_bounds = Rect::new(3, 2, bounds.width() - 2, bounds.height() - 3);
    dialog.add(Box::new(StaticText::new_centered(text_bounds, message)));

    // Determine which buttons to show
    let button_configs = [
        (MF_YES_BUTTON, "~Y~es", CM_YES),
        (MF_NO_BUTTON, "~N~o", CM_NO),
        (MF_OK_BUTTON, "  ~O~K  ", CM_OK),
        (MF_CANCEL_BUTTON, " ~C~ancel ", CM_CANCEL),
    ];

    let mut buttons = Vec::new();
    for (flag, label, cmd) in &button_configs {
        if options & flag != 0 {
            buttons.push((*label, *cmd));
        }
    }

    // Calculate button positions
    let button_y = bounds.height() - 3;
    let total_width: usize = buttons.iter().map(|(label, _)| label.len() + 2).sum();
    let mut x = (bounds.width() as usize - total_width) / 2;

    // Add buttons
    let is_default = buttons.len() == 1 || (options & MF_OK_BUTTON != 0);
    for (i, (label, cmd)) in buttons.iter().enumerate() {
        let button_width = label.len() as i16;
        let button_bounds = Rect::new(x as i16, button_y, x as i16 + button_width, button_y + 2);
        let is_this_default = is_default && (i == 0 || *cmd == CM_OK);
        dialog.add(Box::new(Button::new(button_bounds, label, *cmd, is_this_default)));
        x += button_width as usize + 2;
    }

    dialog.set_initial_focus();
    dialog.execute(terminal)
}

/// Display an input box that prompts the user for a string
pub fn input_box(terminal: &mut Terminal, title: &str, label: &str, initial: &str, max_length: usize) -> Option<String> {
    // Calculate dialog size
    let label_len = label.len();
    let width = (label_len + max_length + 12).min(60).max(30);
    let height = 7;

    // Center on screen
    let (screen_w, screen_h) = terminal.size();
    let x = (screen_w as i16 - width as i16) / 2;
    let y = (screen_h as i16 - height as i16) / 2;

    let bounds = Rect::new(x, y, x + width as i16, y + height as i16);

    input_box_rect(terminal, bounds, title, label, initial, max_length)
}

/// Display an input box at a specific location
pub fn input_box_rect(terminal: &mut Terminal, bounds: Rect, title: &str, label: &str, initial: &str, max_length: usize) -> Option<String> {
    let mut dialog = Dialog::new(bounds, title);

    // Create shared data for input line
    let data = Rc::new(RefCell::new(initial.to_string()));

    // Add label
    let label_x = 2;
    let label_width = label.len() as i16;
    let label_bounds = Rect::new(label_x, 2, label_x + label_width, 3);
    dialog.add(Box::new(Label::new(label_bounds, label)));

    // Add input line
    let input_x = label_x + label_width + 1;
    let input_width = (bounds.width() - input_x - 3).min(max_length as i16 + 2);
    let input_bounds = Rect::new(input_x, 2, input_x + input_width, 3);
    dialog.add(Box::new(InputLine::new(input_bounds, max_length, data.clone())));

    // Add OK button
    let button_y = bounds.height() - 3;
    let ok_x = bounds.width() / 2 - 11;
    let ok_bounds = Rect::new(ok_x, button_y, ok_x + 10, button_y + 2);
    dialog.add(Box::new(Button::new(ok_bounds, "  ~O~K  ", CM_OK, true)));

    // Add Cancel button
    let cancel_x = ok_x + 12;
    let cancel_bounds = Rect::new(cancel_x, button_y, cancel_x + 10, button_y + 2);
    dialog.add(Box::new(Button::new(cancel_bounds, " Cancel ", CM_CANCEL, false)));

    dialog.set_initial_focus();

    let result = dialog.execute(terminal);

    if result == CM_OK {
        Some(data.borrow().clone())
    } else {
        None
    }
}
