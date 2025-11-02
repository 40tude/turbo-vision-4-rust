//! ANSI dump utilities for debugging terminal output
//!
//! This module provides functionality to dump terminal buffers to ANSI text files,
//! which can be viewed with `cat` or any text editor that supports ANSI escape codes.
//!
//! # Examples
//!
//! ## Dumping the entire screen
//! ```no_run
//! use turbo_vision::terminal::Terminal;
//!
//! let terminal = Terminal::init().unwrap();
//! // ... draw some UI ...
//! terminal.dump_screen("debug_screen.ans").unwrap();
//! ```
//!
//! ## Dumping a specific view
//! ```no_run
//! use turbo_vision::prelude::*;
//! use turbo_vision::views::{dialog::Dialog, View};
//! use turbo_vision::terminal::Terminal;
//!
//! let mut terminal = Terminal::init().unwrap();
//! let dialog = Dialog::new(Rect::new(10, 5, 50, 15), "Test");
//! // ... draw the dialog ...
//! dialog.dump_to_file(&terminal, "debug_dialog.ans").unwrap();
//! ```
//!
//! ## Viewing the dumped files
//! On Unix-like systems, you can view the ANSI files with:
//! ```bash
//! cat debug_screen.ans
//! less -R debug_screen.ans  # For scrollable viewing
//! ```

use super::draw::Cell;
use super::palette::TvColor;
use std::io::{self, Write};
use std::fs::File;

/// Convert TvColor to ANSI escape code
fn color_to_ansi_fg(color: TvColor) -> u8 {
    match color {
        TvColor::Black => 30,
        TvColor::Red => 31,
        TvColor::Green => 32,
        TvColor::Brown => 33,
        TvColor::Blue => 34,
        TvColor::Magenta => 35,
        TvColor::Cyan => 36,
        TvColor::LightGray => 37,
        TvColor::DarkGray => 90,
        TvColor::LightRed => 91,
        TvColor::LightGreen => 92,
        TvColor::Yellow => 93,
        TvColor::LightBlue => 94,
        TvColor::LightMagenta => 95,
        TvColor::LightCyan => 96,
        TvColor::White => 97,
    }
}

fn color_to_ansi_bg(color: TvColor) -> u8 {
    match color {
        TvColor::Black => 40,
        TvColor::Red => 41,
        TvColor::Green => 42,
        TvColor::Brown => 43,
        TvColor::Blue => 44,
        TvColor::Magenta => 45,
        TvColor::Cyan => 46,
        TvColor::LightGray => 47,
        TvColor::DarkGray => 100,
        TvColor::LightRed => 101,
        TvColor::LightGreen => 102,
        TvColor::Yellow => 103,
        TvColor::LightBlue => 104,
        TvColor::LightMagenta => 105,
        TvColor::LightCyan => 106,
        TvColor::White => 107,
    }
}

/// Dump a buffer to an ANSI text file
///
/// Creates a new file at the specified path and writes the buffer contents
/// with ANSI color codes. The resulting file can be viewed with `cat` or
/// any text editor that supports ANSI escape sequences.
///
/// # Arguments
/// * `buffer` - The 2D cell buffer to dump
/// * `width` - Width of the region to dump
/// * `height` - Height of the region to dump
/// * `path` - File path where the dump will be saved
pub fn dump_buffer_to_file(
    buffer: &[Vec<Cell>],
    width: usize,
    height: usize,
    path: &str,
) -> io::Result<()> {
    let mut file = File::create(path)?;
    dump_buffer(&mut file, buffer, width, height)?;
    Ok(())
}

/// Dump a buffer to any writer (file, stdout, etc.)
///
/// Writes the buffer contents with ANSI color codes to the provided writer.
/// Color codes are only emitted when colors change between cells to minimize
/// output size.
///
/// # Arguments
/// * `writer` - Output writer (file, stdout, or any `Write` implementor)
/// * `buffer` - The 2D cell buffer to dump
/// * `width` - Width of the region to dump
/// * `height` - Height of the region to dump
pub fn dump_buffer<W: Write>(
    writer: &mut W,
    buffer: &[Vec<Cell>],
    width: usize,
    height: usize,
) -> io::Result<()> {
    for row in buffer.iter().take(height.min(buffer.len())) {
        let mut last_fg = None;
        let mut last_bg = None;

        for x in 0..width.min(row.len()) {
            let cell = row[x];

            // Only emit color codes when colors change
            let need_fg_change = Some(cell.attr.fg) != last_fg;
            let need_bg_change = Some(cell.attr.bg) != last_bg;

            if need_fg_change || need_bg_change {
                if need_fg_change && need_bg_change {
                    write!(
                        writer,
                        "\x1b[{};{}m",
                        color_to_ansi_fg(cell.attr.fg),
                        color_to_ansi_bg(cell.attr.bg)
                    )?;
                } else if need_fg_change {
                    write!(writer, "\x1b[{}m", color_to_ansi_fg(cell.attr.fg))?;
                } else {
                    write!(writer, "\x1b[{}m", color_to_ansi_bg(cell.attr.bg))?;
                }
                last_fg = Some(cell.attr.fg);
                last_bg = Some(cell.attr.bg);
            }

            write!(writer, "{}", cell.ch)?;
        }

        // Reset colors at end of line
        writeln!(writer, "\x1b[0m")?;
    }

    Ok(())
}

/// Dump a rectangular region of a buffer
///
/// Similar to `dump_buffer`, but only dumps a specific rectangular region
/// of the buffer. Useful for dumping individual views or UI components.
///
/// # Arguments
/// * `writer` - Output writer (file, stdout, or any `Write` implementor)
/// * `buffer` - The 2D cell buffer to dump from
/// * `x` - Starting X coordinate
/// * `y` - Starting Y coordinate
/// * `width` - Width of the region
/// * `height` - Height of the region
pub fn dump_buffer_region<W: Write>(
    writer: &mut W,
    buffer: &[Vec<Cell>],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> io::Result<()> {
    for row in buffer.iter().take((y + height).min(buffer.len())).skip(y) {
        let mut last_fg = None;
        let mut last_bg = None;

        for col in x..(x + width).min(row.len()) {
            let cell = row[col];

            let need_fg_change = Some(cell.attr.fg) != last_fg;
            let need_bg_change = Some(cell.attr.bg) != last_bg;

            if need_fg_change || need_bg_change {
                if need_fg_change && need_bg_change {
                    write!(
                        writer,
                        "\x1b[{};{}m",
                        color_to_ansi_fg(cell.attr.fg),
                        color_to_ansi_bg(cell.attr.bg)
                    )?;
                } else if need_fg_change {
                    write!(writer, "\x1b[{}m", color_to_ansi_fg(cell.attr.fg))?;
                } else {
                    write!(writer, "\x1b[{}m", color_to_ansi_bg(cell.attr.bg))?;
                }
                last_fg = Some(cell.attr.fg);
                last_bg = Some(cell.attr.bg);
            }

            write!(writer, "{}", cell.ch)?;
        }

        writeln!(writer, "\x1b[0m")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::palette::Attr;

    #[test]
    fn test_dump_simple_buffer() {
        let cells = vec![
            Cell::new('H', Attr::new(TvColor::White, TvColor::Blue)),
            Cell::new('i', Attr::new(TvColor::White, TvColor::Blue)),
        ];

        let buffer = vec![cells];
        let mut output = Vec::new();

        dump_buffer(&mut output, &buffer, 2, 1).unwrap();

        let result = String::from_utf8(output).unwrap();
        assert!(result.contains("Hi"));
        assert!(result.contains("\x1b[")); // Contains ANSI codes
    }
}
