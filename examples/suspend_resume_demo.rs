// (C) 2025 - Enzo Lombardi
// Suspend/Resume Demo - demonstrates terminal suspend/resume functionality

use std::time::Duration;
use turbo_vision::app::Application;
use turbo_vision::core::command::CM_QUIT;
use turbo_vision::core::event::EventType;
use turbo_vision::core::geometry::Rect;
use turbo_vision::helpers::msgbox::{MF_INFORMATION, MF_OK_BUTTON, message_box};
use turbo_vision::views::View;
use turbo_vision::views::button::ButtonBuilder;
use turbo_vision::views::dialog::Dialog;
use turbo_vision::views::static_text::StaticTextBuilder;

const CM_SUSPEND: u16 = 100;

fn main() -> turbo_vision::core::error::Result<()> {
    let mut app = Application::new()?;

    let mut dialog = Dialog::new(Rect::new(10, 3, 10 + 60, 3 + 15), "Suspend-Resume Demo");

    dialog.add(Box::new(
        StaticTextBuilder::new()
            .bounds(Rect::new(2, 1, 2 + 54, 1 + 5))
            .text(
                "This demo shows suspend/resume functionality.\n\n\
                Click 'Suspend' to temporarily return to shell.\n\
                The application will exit raw mode and show your\n\
                shell prompt. Press Enter to resume.",
            )
            .build(),
    ));

    dialog.add(Box::new(
        ButtonBuilder::new()
            .bounds(Rect::new(20, 9, 20 + 20, 9 + 2))
            .title("Suspend")
            .command(CM_SUSPEND)
            .default(false)
            .build(),
    ));

    dialog.add(Box::new(
        ButtonBuilder::new().bounds(Rect::new(20, 11, 20 + 20, 11 + 2)).title("Quit").command(CM_QUIT).default(true).build(),
    ));

    app.desktop.add(Box::new(dialog));

    loop {
        app.desktop.draw(&mut app.terminal);
        let _ = app.terminal.flush();

        if let Some(mut event) = app.terminal.poll_event(Duration::from_millis(50)).ok().flatten() {
            app.desktop.handle_event(&mut event);

            if event.what == EventType::Command {
                match event.command {
                    CM_SUSPEND => {
                        // Suspend the application
                        app.suspend()?;

                        // At this point, the terminal is in normal mode
                        // The user can use the shell, and when they type Enter,
                        // we'll continue here

                        // For this demo, we'll immediately resume
                        // In a real implementation with signal handlers,
                        // the process would be stopped here (SIGSTOP)
                        // and resumed later (SIGCONT)

                        println!("Application suspended. Press Enter to resume...");
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input)?;

                        // Resume the application
                        app.resume()?;

                        // Show a message that we're back
                        message_box(&mut app, "Welcome back! Application resumed.", MF_INFORMATION | MF_OK_BUTTON);
                    }
                    CM_QUIT => break,
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
