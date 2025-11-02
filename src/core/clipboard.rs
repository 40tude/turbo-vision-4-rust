use std::sync::Mutex;

/// Global clipboard for copy/cut/paste operations
/// This is a simple in-memory clipboard shared across all editor components
static CLIPBOARD: Mutex<String> = Mutex::new(String::new());

/// Set the clipboard content
pub fn set_clipboard(text: &str) {
    if let Ok(mut clipboard) = CLIPBOARD.lock() {
        *clipboard = text.to_string();
    }
}

/// Get the clipboard content
pub fn get_clipboard() -> String {
    CLIPBOARD.lock()
        .map(|clipboard| clipboard.clone())
        .unwrap_or_default()
}

/// Check if the clipboard has content
pub fn has_clipboard_content() -> bool {
    CLIPBOARD.lock()
        .map(|clipboard| !clipboard.is_empty())
        .unwrap_or(false)
}

/// Clear the clipboard
pub fn clear_clipboard() {
    if let Ok(mut clipboard) = CLIPBOARD.lock() {
        clipboard.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_operations() {
        clear_clipboard();
        assert!(!has_clipboard_content());

        set_clipboard("Hello, World!");
        assert!(has_clipboard_content());
        assert_eq!(get_clipboard(), "Hello, World!");

        set_clipboard("New content");
        assert_eq!(get_clipboard(), "New content");

        clear_clipboard();
        assert!(!has_clipboard_content());
        assert_eq!(get_clipboard(), "");
    }
}
