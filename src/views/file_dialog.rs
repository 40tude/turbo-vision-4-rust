//! File Dialog - A file selection dialog for opening files
//!
//! ## Current Implementation
//!
//! The FileDialog provides a classic file selection interface with:
//! - Input line for typing filenames
//! - ListBox showing files and directories in the current path
//! - **Mouse support**: Click on files to select, double-click to open
//! - **Keyboard navigation**: Arrow keys, PgUp/PgDn, Home/End
//! - Directory navigation (double-click directories or select and press Enter)
//! - Wildcard filtering (e.g., "*.rs" shows only Rust files)
//! - Parent directory navigation via ".."
//!
//! ## Usage
//!
//! Users can select files in multiple ways:
//! 1. **Click a file** once to select it (updates the input field)
//! 2. **Double-click a file** to select and open it
//! 3. **Use arrow keys** to navigate the list, press Enter to select
//! 4. **Type a filename** directly in the input box
//!
//! Directory navigation:
//! - Press Enter on a folder (`[dirname]`) to navigate into it
//! - Press Enter on `..` to go to parent directory
//! - Click on folders to navigate (single click selects, double-click or Enter opens)
//! - The dialog stays open while navigating directories
//!
//! Wildcard patterns:
//! - `"*"` - Shows all files
//! - `"*.rs"` - Shows only files ending with `.rs`
//! - `"*.toml"` - Shows only files ending with `.toml`
//! - `"test"` - Shows files containing "test" in their name
//!
//! **Note**: Directories are always shown regardless of the wildcard pattern.
//!
//! ## Implementation Notes
//!
//! The FileDialog tracks ListBox selection state by intercepting keyboard and mouse
//! events before passing them to the dialog. This allows it to:
//! - Maintain a shadow selection index
//! - Update the InputLine when files are selected
//! - Handle directory navigation seamlessly
//!
//! ### Architecture
//!
//! The Dialog/Window/Group hierarchy now provides `child_at_mut()` methods for accessing
//! child views. This architectural improvement allows components to:
//! - Query child view state after adding them to containers
//! - Modify child views dynamically
//! - Create more sophisticated interactions between parent and child views
//!
//! The current FileDialog implementation uses event interception for simplicity and
//! performance, but could alternatively use direct child access if needed for more
//! complex scenarios.

use crate::core::geometry::Rect;
use crate::core::event::{Event, EventType};
use crate::core::command::{CM_OK, CM_CANCEL};
use crate::terminal::Terminal;
use super::dialog::Dialog;
use super::input_line::InputLine;
use super::listbox::ListBox;
use super::button::Button;
use super::label::Label;
use super::View;
use std::path::PathBuf;
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

const CMD_FILE_SELECTED: u16 = 1000;

pub struct FileDialog {
    dialog: Dialog,
    current_path: PathBuf,
    wildcard: String,
    file_name_data: Rc<RefCell<String>>,
    files: Vec<String>,
    selected_file_index: usize,  // Track ListBox selection
}

impl FileDialog {
    pub fn new(bounds: Rect, title: &str, wildcard: &str, initial_dir: Option<PathBuf>) -> Self {
        let dialog = Dialog::new(bounds, title);

        let current_path = initial_dir.unwrap_or_else(|| {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        });

        let file_name_data = Rc::new(RefCell::new(String::new()));

        Self {
            dialog,
            current_path,
            wildcard: wildcard.to_string(),
            file_name_data,
            files: Vec::new(),
            selected_file_index: 0,
        }
    }

    pub fn build(mut self) -> Self {
        let bounds = self.dialog.bounds();
        let dialog_width = bounds.width();

        // Label for file name input
        let name_label = Label::new(Rect::new(2, 1, 12, 2), "~N~ame:");
        self.dialog.add(Box::new(name_label));

        // File name input line
        let file_input = InputLine::new(
            Rect::new(12, 1, dialog_width - 4, 2),
            255,
            self.file_name_data.clone()
        );
        self.dialog.add(Box::new(file_input));

        // Current path label
        let path_str = format!(" {}", self.current_path.display());
        let path_label = Label::new(Rect::new(2, 3, dialog_width - 4, 4), &path_str);
        self.dialog.add(Box::new(path_label));

        // Label for files list
        let files_label = Label::new(Rect::new(2, 5, 12, 6), "~F~iles:");
        self.dialog.add(Box::new(files_label));

        // File list box - will be populated after reading directory
        let mut file_list = ListBox::new(
            Rect::new(2, 6, dialog_width - 4, bounds.height() - 6),
            CMD_FILE_SELECTED,
        );

        // Load directory contents first
        self.read_directory();

        // Populate the list box with files
        file_list.set_items(self.files.clone());
        self.dialog.add(Box::new(file_list));

        // Buttons
        let button_y = bounds.height() - 4;
        let button_spacing = 14;
        let mut button_x = 2;

        let open_button = Button::new(
            Rect::new(button_x, button_y, button_x + 12, button_y + 2),
            "  ~O~pen  ",
            CM_OK,
            true,
        );
        self.dialog.add(Box::new(open_button));
        button_x += button_spacing;

        let cancel_button = Button::new(
            Rect::new(button_x, button_y, button_x + 12, button_y + 2),
            " ~C~ancel ",
            CM_CANCEL,
            false,
        );
        self.dialog.add(Box::new(cancel_button));

        self.dialog.set_initial_focus();

        self
    }

    pub fn execute(&mut self, terminal: &mut Terminal) -> Option<PathBuf> {
        loop {
            // Draw
            self.dialog.draw(terminal);
            self.dialog.update_cursor(terminal);
            let _ = terminal.flush();

            // Get event
            if let Ok(Some(mut event)) = terminal.poll_event(std::time::Duration::from_millis(50)) {
                // Handle double ESC to close
                if event.what == EventType::Keyboard && event.key_code == crate::core::event::KB_ESC_ESC {
                    return None;
                }

                // Track ListBox navigation events to maintain selection state
                self.track_listbox_events(&event);

                self.dialog.handle_event(&mut event);

                // Check if dialog should close
                if event.what == EventType::Command {
                    match event.command {
                        CM_OK => {
                            // Get file name from input field
                            let file_name = self.file_name_data.borrow().clone();
                            if !file_name.is_empty() {
                                // Check if it's a directory navigation request or file selection
                                if let Some(path) = self.handle_selection(&file_name, terminal) {
                                    // File selected - return it
                                    return Some(path);
                                }
                                // Directory navigation - continue loop
                            } else {
                                // Empty input - just close
                                return None;
                            }
                        }
                        CM_CANCEL | crate::core::command::CM_CLOSE => {
                            return None;
                        }
                        CMD_FILE_SELECTED => {
                            // User double-clicked or pressed Enter on a file in the list
                            if self.selected_file_index < self.files.len() {
                                let selected = self.files[self.selected_file_index].clone();

                                // Update the input line with the selected file
                                *self.file_name_data.borrow_mut() = selected.clone();

                                // Handle the selection (navigate dirs or return file)
                                if let Some(path) = self.handle_selection(&selected, terminal) {
                                    // File selected - return it
                                    return Some(path);
                                }
                                // Directory navigation - continue loop
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// Track keyboard and mouse events to maintain ListBox selection state
    fn track_listbox_events(&mut self, event: &Event) {
        use crate::core::event::{KB_UP, KB_DOWN, KB_HOME, KB_END, KB_PGUP, KB_PGDN};

        match event.what {
            EventType::Keyboard => {
                match event.key_code {
                    KB_UP => {
                        if self.selected_file_index > 0 {
                            self.selected_file_index -= 1;
                        }
                    }
                    KB_DOWN => {
                        if self.selected_file_index + 1 < self.files.len() {
                            self.selected_file_index += 1;
                        }
                    }
                    KB_HOME => {
                        self.selected_file_index = 0;
                    }
                    KB_END => {
                        if !self.files.is_empty() {
                            self.selected_file_index = self.files.len() - 1;
                        }
                    }
                    KB_PGUP => {
                        // Page size is roughly the height of the listbox
                        // This is an approximation
                        let page_size = 10;
                        self.selected_file_index = self.selected_file_index.saturating_sub(page_size);
                    }
                    KB_PGDN => {
                        // Page down
                        let page_size = 10;
                        if !self.files.is_empty() {
                            self.selected_file_index = (self.selected_file_index + page_size).min(self.files.len() - 1);
                        }
                    }
                    _ => {}
                }
            }
            EventType::MouseDown => {
                // The ListBox will handle the click and update its selection
                // We need to sync our state with it
                // Calculate if the click is within the ListBox bounds
                let mouse_pos = event.mouse.pos;

                // ListBox is at position (2, 6) relative to dialog interior
                // and has height (bounds.height() - 6)
                let dialog_bounds = self.dialog.bounds();
                let listbox_y_start = dialog_bounds.a.y + 6;
                let listbox_y_end = dialog_bounds.b.y - 6;
                let listbox_x_start = dialog_bounds.a.x + 2;
                let listbox_x_end = dialog_bounds.b.x - 4;

                if mouse_pos.x >= listbox_x_start && mouse_pos.x < listbox_x_end &&
                   mouse_pos.y >= listbox_y_start && mouse_pos.y < listbox_y_end {

                    // Calculate which item was clicked
                    let relative_y = (mouse_pos.y - listbox_y_start) as usize;
                    if relative_y < self.files.len() {
                        self.selected_file_index = relative_y;
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_selection(&mut self, file_name: &str, terminal: &mut Terminal) -> Option<PathBuf> {
        if file_name == ".." {
            // Go to parent directory
            if let Some(parent) = self.current_path.parent() {
                self.current_path = parent.to_path_buf();
                self.read_directory();
                self.rebuild_and_redraw(terminal);
            }
            None
        } else if file_name.starts_with('[') && file_name.ends_with(']') {
            // Directory selected - navigate into it
            let dir_name = &file_name[1..file_name.len() - 1];
            self.current_path.push(dir_name);
            self.read_directory();
            self.rebuild_and_redraw(terminal);
            None
        } else {
            // File selected - update input and return
            *self.file_name_data.borrow_mut() = file_name.to_string();
            Some(self.current_path.join(file_name))
        }
    }

    fn rebuild_and_redraw(&mut self, _terminal: &mut Terminal) {
        // Create a new dialog with updated file list
        let old_bounds = self.dialog.bounds();
        let old_title = "Open File"; // TODO: Store title

        *self = Self::new(old_bounds, old_title, &self.wildcard.clone(), Some(self.current_path.clone())).build();
    }

    fn read_directory(&mut self) {
        self.files.clear();

        // Add parent directory entry
        if self.current_path.parent().is_some() {
            self.files.push("..".to_string());
        }

        // Read directory contents
        if let Ok(entries) = fs::read_dir(&self.current_path) {
            let mut dirs = Vec::new();
            let mut regular_files = Vec::new();

            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    let name = entry.file_name().to_string_lossy().to_string();

                    if metadata.is_dir() {
                        dirs.push(format!("[{}]", name));
                    } else if self.matches_wildcard(&name) {
                        regular_files.push(name);
                    }
                }
            }

            // Sort and combine: directories first, then files
            dirs.sort();
            regular_files.sort();
            self.files.extend(dirs);
            self.files.extend(regular_files);
        }
    }

    fn matches_wildcard(&self, name: &str) -> bool {
        if self.wildcard == "*" || self.wildcard.is_empty() {
            return true;
        }

        // Simple wildcard matching (*.ext)
        if let Some(ext) = self.wildcard.strip_prefix("*.") {
            name.ends_with(&format!(".{}", ext))
        } else {
            name.contains(&self.wildcard)
        }
    }

    pub fn get_selected_file(&self) -> Option<PathBuf> {
        let file_name = self.file_name_data.borrow().clone();
        if !file_name.is_empty() {
            Some(self.current_path.join(file_name))
        } else {
            None
        }
    }
}

impl View for FileDialog {
    fn bounds(&self) -> Rect {
        self.dialog.bounds()
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.dialog.set_bounds(bounds);
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        self.dialog.draw(terminal);
    }

    fn handle_event(&mut self, event: &mut Event) {
        self.dialog.handle_event(event);
    }
}
