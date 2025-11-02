use crate::core::draw::Cell;
use crate::core::event::{Event, EventType, EscSequenceTracker, MB_LEFT_BUTTON, MB_MIDDLE_BUTTON, MB_RIGHT_BUTTON};
use crate::core::geometry::Point;
use crate::core::palette::Attr;
use crossterm::{
    cursor, execute, queue, style,
    terminal::{self},
    event::{self, Event as CTEvent, MouseEventKind, MouseButton},
};
use std::io::{self, Write, stdout};
use std::time::Duration;

/// Terminal abstraction for crossterm backend
pub struct Terminal {
    buffer: Vec<Vec<Cell>>,
    prev_buffer: Vec<Vec<Cell>>,
    width: u16,
    height: u16,
    esc_tracker: EscSequenceTracker,
    last_mouse_pos: Point,
    last_mouse_buttons: u8,
}

impl Terminal {
    /// Initialize the terminal
    pub fn init() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(
            stdout,
            terminal::EnterAlternateScreen,
            cursor::Hide,
            event::EnableMouseCapture  // Enable mouse support
        )?;

        let (width, height) = terminal::size()?;

        let empty_cell = Cell::new(' ', Attr::from_u8(0x07));
        let buffer = vec![vec![empty_cell; width as usize]; height as usize];
        let prev_buffer = vec![vec![empty_cell; width as usize]; height as usize];

        Ok(Self {
            buffer,
            prev_buffer,
            width,
            height,
            esc_tracker: EscSequenceTracker::new(),
            last_mouse_pos: Point::zero(),
            last_mouse_buttons: 0,
        })
    }

    /// Shutdown the terminal
    pub fn shutdown(&mut self) -> io::Result<()> {
        let mut stdout = stdout();
        execute!(
            stdout,
            event::DisableMouseCapture,  // Disable mouse support
            cursor::Show,
            terminal::LeaveAlternateScreen
        )?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    /// Get terminal size
    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    /// Write a cell at the given position
    pub fn write_cell(&mut self, x: u16, y: u16, cell: Cell) {
        if (x as usize) < self.width as usize && (y as usize) < self.height as usize {
            self.buffer[y as usize][x as usize] = cell;
        }
    }

    /// Write a line from a draw buffer
    pub fn write_line(&mut self, x: u16, y: u16, cells: &[Cell]) {
        if (y as usize) >= self.height as usize {
            return;
        }

        let max_width = (self.width as usize).saturating_sub(x as usize);
        let len = cells.len().min(max_width);

        for (i, cell) in cells.iter().enumerate().take(len) {
            self.buffer[y as usize][(x as usize) + i] = *cell;
        }
    }

    /// Clear the entire screen
    pub fn clear(&mut self) {
        let empty_cell = Cell::new(' ', Attr::from_u8(0x07));
        for row in &mut self.buffer {
            for cell in row {
                *cell = empty_cell;
            }
        }
    }

    /// Flush changes to the terminal
    pub fn flush(&mut self) -> io::Result<()> {
        let mut stdout = stdout();

        for y in 0..self.height as usize {
            let mut x = 0;
            while x < self.width as usize {
                // Find the start of a changed region
                if self.buffer[y][x] == self.prev_buffer[y][x] {
                    x += 1;
                    continue;
                }

                // Find the end of the changed region
                let start_x = x;
                let current_attr = self.buffer[y][x].attr;

                while x < self.width as usize
                    && self.buffer[y][x] != self.prev_buffer[y][x]
                    && self.buffer[y][x].attr == current_attr
                {
                    x += 1;
                }

                // Move cursor and set colors
                queue!(
                    stdout,
                    cursor::MoveTo(start_x as u16, y as u16),
                    style::SetForegroundColor(current_attr.fg.to_crossterm()),
                    style::SetBackgroundColor(current_attr.bg.to_crossterm())
                )?;

                // Write the changed characters
                for i in start_x..x {
                    write!(stdout, "{}", self.buffer[y][i].ch)?;
                }
            }
        }

        stdout.flush()?;

        // Copy current buffer to previous buffer
        self.prev_buffer.clone_from(&self.buffer);

        Ok(())
    }

    /// Show the cursor at the specified position
    pub fn show_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        let mut stdout = stdout();
        execute!(
            stdout,
            cursor::MoveTo(x, y),
            cursor::Show
        )?;
        Ok(())
    }

    /// Hide the cursor
    pub fn hide_cursor(&mut self) -> io::Result<()> {
        let mut stdout = stdout();
        execute!(stdout, cursor::Hide)?;
        Ok(())
    }

    /// Poll for an event with timeout
    pub fn poll_event(&mut self, timeout: Duration) -> io::Result<Option<Event>> {
        if event::poll(timeout)? {
            match event::read()? {
                CTEvent::Key(key) => {
                    let key_code = self.esc_tracker.process_key(key);
                    if key_code == 0 {
                        // ESC sequence in progress, don't generate event yet
                        return Ok(None);
                    }
                    Ok(Some(Event::keyboard(key_code)))
                }
                CTEvent::Mouse(mouse) => {
                    Ok(self.convert_mouse_event(mouse))
                }
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    /// Read an event (blocking)
    pub fn read_event(&mut self) -> io::Result<Event> {
        loop {
            match event::read()? {
                CTEvent::Key(key) => {
                    let key_code = self.esc_tracker.process_key(key);
                    if key_code == 0 {
                        // ESC sequence in progress, wait for next key
                        continue;
                    }
                    return Ok(Event::keyboard(key_code));
                }
                CTEvent::Mouse(mouse) => {
                    if let Some(event) = self.convert_mouse_event(mouse) {
                        return Ok(event);
                    }
                }
                _ => continue,
            }
        }
    }

    /// Convert crossterm mouse event to our Event type
    fn convert_mouse_event(&mut self, mouse: event::MouseEvent) -> Option<Event> {
        let pos = Point::new(mouse.column as i16, mouse.row as i16);

        // Convert button state to our format
        let buttons = match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) | MouseEventKind::Drag(MouseButton::Left) => MB_LEFT_BUTTON,
            MouseEventKind::Down(MouseButton::Right) | MouseEventKind::Drag(MouseButton::Right) => MB_RIGHT_BUTTON,
            MouseEventKind::Down(MouseButton::Middle) | MouseEventKind::Drag(MouseButton::Middle) => MB_MIDDLE_BUTTON,
            MouseEventKind::Up(_) => 0, // No buttons pressed on release
            MouseEventKind::Moved => self.last_mouse_buttons, // Maintain button state during move
            _ => return None, // Ignore scroll events for now
        };

        // Determine event type
        let event_type = match mouse.kind {
            MouseEventKind::Down(_) => {
                self.last_mouse_buttons = buttons;
                self.last_mouse_pos = pos;
                EventType::MouseDown
            }
            MouseEventKind::Up(_) => {
                self.last_mouse_buttons = 0;
                EventType::MouseUp
            }
            MouseEventKind::Drag(_) | MouseEventKind::Moved => {
                self.last_mouse_pos = pos;
                EventType::MouseMove
            }
            _ => return None,
        };

        // TODO: implement proper double-click detection
        Some(Event::mouse(event_type, pos, buttons, false))
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}
