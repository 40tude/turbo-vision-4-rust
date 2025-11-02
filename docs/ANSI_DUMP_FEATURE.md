# ANSI Dump Feature - Implementation Summary

## Overview

Added a comprehensive ANSI dump feature to the turbo-vision crate for debugging terminal UI. This allows developers to export screen buffers, views, and regions to text files with ANSI color codes.

## Files Added

### Core Implementation
- `src/core/ansi_dump.rs` - Main ANSI dump module with color conversion and buffer export functionality
- `examples/dump_demo.rs` - Complete working example demonstrating all dump features
- `docs/ansi-dump.md` - User documentation for the feature

## Files Modified

### Core Module
- `src/core/mod.rs` - Added `pub mod ansi_dump;`

### Terminal Module
- `src/terminal/mod.rs` - Added three new public methods:
  - `dump_screen(&self, path: &str)` - Dump entire screen
  - `dump_region(&self, x, y, width, height, path)` - Dump specific region
  - `buffer(&self)` - Get reference to internal buffer

### View Trait
- `src/views/view.rs` - Added `dump_to_file()` method to View trait

### Documentation
- `README.md` - Added ANSI Dump to features list and status section
- `CHANGELOG.md` - Not yet updated (should be done before release)

## API Design

### High-Level API (Recommended for most users)

```rust
// Dump entire screen
terminal.dump_screen("screen.ans")?;

// Dump a specific view (works with any View implementor)
dialog.dump_to_file(&terminal, "dialog.ans")?;

// Dump a specific region
terminal.dump_region(10, 5, 40, 20, "region.ans")?;
```

### Low-Level API (For custom needs)

```rust
use turbo_vision::core::ansi_dump;

// Get buffer and dump it manually
let buffer = terminal.buffer();
ansi_dump::dump_buffer_to_file(buffer, width, height, "custom.ans")?;

// Dump to any writer
let mut writer = std::io::stdout();
ansi_dump::dump_buffer(&mut writer, buffer, width, height)?;
```

## Technical Details

### ANSI Color Mapping
- Uses standard ANSI escape sequences (30-37, 40-47 for normal colors)
- Uses bright ANSI codes (90-97, 100-107 for bright colors)
- Maps TvColor enum to appropriate ANSI codes
- Optimizes output by only emitting color changes

### Output Format
- One line per screen row
- ANSI reset code (`\x1b[0m`) at end of each line
- Colors only change when necessary (not repeated for each cell)
- Standard text format viewable with `cat`, `less -R`, or text editors

### Performance Considerations
- Efficient color change detection minimizes escape sequences
- No buffering required - writes directly to file
- Suitable for large terminal buffers

## Testing

- Unit test in `src/core/ansi_dump.rs` verifies basic functionality
- Example program `dump_demo.rs` provides integration testing
- All existing tests still pass (54 tests)

## Usage Examples

### Basic Debugging
```rust
// After drawing your UI
dialog.draw(&mut terminal);
terminal.flush()?;
dialog.dump_to_file(&terminal, "debug.ans")?;
```

### Regression Testing
```rust
// Capture expected output
terminal.dump_screen("expected.ans")?;

// Later, compare with actual output
terminal.dump_screen("actual.ans")?;
// Then diff the files
```

### Development Workflow
```bash
# Run your app with dump enabled
cargo run --example dump_demo

# View the output
cat screen_dump.ans
less -R dialog_dump.ans
```

## Benefits

1. **Debugging** - Visualize exactly what's in the terminal buffer
2. **Documentation** - Capture screenshots of terminal UI
3. **Testing** - Create visual regression tests
4. **Development** - Quickly inspect layout issues without running the app
5. **Color Verification** - Verify color schemes render correctly

## Future Enhancements (Optional)

- HTML output format (with inline styles)
- PNG/SVG rendering of terminal output
- Diff mode to highlight changes between dumps
- Selective dumping (only visible cells, ignore background)
- Compression for large dumps

## Compatibility

- Works on all platforms (Unix, Windows, macOS)
- Files can be viewed on any system with ANSI support
- Standard format compatible with most terminal tools
