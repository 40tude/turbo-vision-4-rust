# Debug Utilities

This folder contains debug tools and utilities for development and troubleshooting.

## Tools

### key_debug.rs

A utility to debug keyboard input and see the key codes that are actually being received.

**Usage:**
```bash
cargo run --bin key_debug
```

Press any keys to see their codes. The output shows:
- Event type
- Key code in hexadecimal format
- Expected values for F11 and F12

Press Ctrl+C to exit.

**Use cases:**
- Verify that keyboard shortcuts are being detected correctly
- Debug terminal emulator key code differences
- Test ESC sequences and special key combinations
