// (C) 2025 - Enzo Lombardi
//
// Bar minimum app press ALt+X exit.

use turbo_vision::prelude::*;
fn main() -> turbo_vision::core::error::Result<()> {
    let mut app = Application::new()?;
    app.run();
    Ok(())
}
