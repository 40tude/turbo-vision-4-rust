// (C) 2025 - Enzo Lombardi
// Example to test modal vs non-modal window behavior
//
// This example demonstrates:
// 1. Non-modal windows: clicking brings them to front
// 2. Modal dialogs: clicking background windows has no effect
//
// Usage:
//   cargo run --example window_modal_overlap_test
//
// Instructions:
//   1. Initially you'll see two non-modal windows
//   2. Click on the background window - it should come to the front
//   3. Drag the windows around to test z-order and redrawing
//   4. Press ESC ESC to exit
//
// To test modal behavior, uncomment the modal dialog code below

use turbo_vision::app::Application;
use turbo_vision::core::geometry::Rect;
use turbo_vision::views::{static_text::StaticTextBuilder, window::WindowBuilder};

fn main() -> turbo_vision::core::error::Result<()> {
    let mut app = Application::new()?;

    // Create first non-modal window
    let mut window1 = WindowBuilder::new().bounds(Rect::new(5, 3, 55, 16)).title("Non-Modal Window 1").resizable(false).build();

    let text1 = StaticTextBuilder::new()
        .bounds(Rect::new(2, 1, 46, 12))
        .text("This is a NON-MODAL window.\n\nClick anywhere on Window 2 to bring it to\nthe front.\n\nThen click here to bring this window back\nto the front.\n\nPress ESC ESC to exit.")
        .build();
    window1.add(Box::new(text1));

    app.desktop.add(Box::new(window1));

    // Create second non-modal window
    let mut window2 = WindowBuilder::new().bounds(Rect::new(20, 8, 70, 23)).title("Non-Modal Window 2").resizable(false).build();

    let text2 = StaticTextBuilder::new()
        .bounds(Rect::new(2, 1, 46, 13))
        .text("This is also NON-MODAL.\n\nClick on Window 1 behind this one to bring\nit to the front.\n\nYou can drag both windows around.\n\nTry clicking back and forth to see z-order\nchanges.\n\nPress ESC ESC to exit.")
        .build();
    window2.add(Box::new(text2));

    app.desktop.add(Box::new(window2));

    // Create third overlapping window to make z-order more obvious
    let mut window3 = WindowBuilder::new().bounds(Rect::new(35, 5, 81, 17)).title("Non-Modal Window 3").resizable(false).build();

    let text3 = StaticTextBuilder::new()
        .bounds(Rect::new(2, 1, 42, 12))
        .text("Third NON-MODAL window.\n\nClick on any window behind to bring\nit forward.\n\nDrag windows to test overlap redrawing.\n\nPress ESC ESC to exit.")
        .build();
    window3.add(Box::new(text3));

    app.desktop.add(Box::new(window3));

    // Run the application
    // The desktop will automatically handle:
    // - Bringing clicked windows to front
    // - Z-order management
    // - Proper redrawing on overlaps
    app.run();

    Ok(())
}
