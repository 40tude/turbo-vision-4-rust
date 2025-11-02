# Terminal-Level Shortcut Implementation

## Architecture Decision: Lowest-Level Event Handling

The screen dump shortcuts (F12 and F11) are implemented at the **Terminal level** rather than the Application level. This is a key architectural decision that provides significant benefits.

## Implementation Location

### Terminal::poll_event()
```rust
pub fn poll_event(&mut self, timeout: Duration) -> io::Result<Option<Event>> {
    if event::poll(timeout)? {
        match event::read()? {
            CTEvent::Key(key) => {
                let key_code = self.esc_tracker.process_key(key);
                if key_code == 0 {
                    return Ok(None);
                }

                // Handle global screen dump shortcuts at the lowest level
                if key_code == KB_F12 {
                    let _ = self.flash();
                    let _ = self.dump_screen("screen-dump.txt");
                    return Ok(None);  // Event consumed, not propagated
                }

                if key_code == KB_F11 {
                    let _ = self.flash();
                    if let Some(bounds) = self.active_view_bounds {
                        let _ = self.dump_region(..., "active-view-dump.txt");
                    }
                    return Ok(None);  // Event consumed, not propagated
                }

                Ok(Some(Event::keyboard(key_code)))
            }
            // ... mouse handling ...
        }
    } else {
        Ok(None)
    }
}
```

### Terminal::read_event()
```rust
pub fn read_event(&mut self) -> io::Result<Event> {
    loop {
        match event::read()? {
            CTEvent::Key(key) => {
                let key_code = self.esc_tracker.process_key(key);
                if key_code == 0 {
                    continue;
                }

                // Handle global screen dump shortcuts at the lowest level
                if key_code == KB_F12 {
                    let _ = self.flash();
                    let _ = self.dump_screen("screen-dump.txt");
                    continue;  // Don't return, wait for next event
                }

                if key_code == KB_F11 {
                    let _ = self.flash();
                    if let Some(bounds) = self.active_view_bounds {
                        let _ = self.dump_region(..., "active-view-dump.txt");
                    }
                    continue;  // Don't return, wait for next event
                }

                return Ok(Event::keyboard(key_code));
            }
            // ... mouse handling ...
        }
    }
}
```

## Benefits of This Approach

### 1. Universal Availability
Works everywhere without any integration code:
- ✅ `Application::run()` event loop
- ✅ `Dialog::execute()` event loop
- ✅ Custom event loops in examples
- ✅ Any code that calls `terminal.poll_event()` or `terminal.read_event()`

### 2. Zero Configuration
No need to:
- Add hooks in Application
- Add hooks in Dialog
- Add hooks in every custom event loop
- Remember to call special handler functions
- Pass terminal references around for screen dumps

### 3. Cannot Be Blocked
Since shortcuts are handled before events are returned to application code:
- Event handlers can't accidentally consume the shortcut
- Custom event loops don't need special handling
- Always works, regardless of application state

### 4. Clean Separation of Concerns
- **Terminal layer**: Handles low-level I/O and global system shortcuts
- **Application layer**: Handles business logic and UI events
- **View layer**: Handles widget-specific behavior

### 5. Consistent Behavior
The shortcuts behave identically whether you're:
- In the main application loop
- In a modal dialog
- In a message box
- In any custom UI component

## Comparison with Alternative Approaches

### ❌ Application-Level Handler
```rust
// In Application::handle_event()
if event.key_code == KB_F12 { ... }
```
**Problems:**
- Doesn't work in Dialog::execute()
- Doesn't work in custom event loops
- Requires integration in multiple places

### ❌ Per-Loop Handler
```rust
// In Dialog::execute()
if event.key_code == KB_F12 { ... }
// In Application::run()
if event.key_code == KB_F12 { ... }
// In custom loop
if event.key_code == KB_F12 { ... }
```
**Problems:**
- Code duplication
- Easy to forget in custom loops
- Maintenance burden

### ✅ Terminal-Level Handler (Our Solution)
```rust
// In Terminal::poll_event()
if key_code == KB_F12 { ... }
```
**Benefits:**
- Single implementation point
- Works everywhere automatically
- Zero integration burden

## Event Flow

```
┌─────────────────────┐
│  Crossterm Events   │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Terminal::poll_event│
│    or read_event    │
└──────────┬──────────┘
           │
           ├─► F12 or ESC+Shift+S? ──► Flash + Dump ──► Return None/Continue
           │                                              (Event consumed)
           │
           ▼
    ┌─────────────┐
    │ Other Events│ ──► Return to Application
    └─────────────┘         │
                            ▼
                    ┌───────────────┐
                    │ Application   │
                    │ Dialog        │
                    │ Custom Loops  │
                    └───────────────┘
```

## Implementation Notes

### Event Suppression
When a screen dump shortcut is detected:
- **poll_event**: Returns `Ok(None)` - "no event available"
- **read_event**: Loops with `continue` - waits for next event

This ensures the shortcut keys never reach application code.

### Error Handling
```rust
let _ = self.flash();
let _ = self.dump_screen("screen-dump.txt");
```

Errors are intentionally ignored because:
1. Screen dumps are debugging aids, not critical functionality
2. Shouldn't crash app if disk is full or path is invalid
3. User sees flash effect, so they know capture was attempted

### Thread Safety
All operations are performed on `&mut self`, ensuring:
- No concurrent access issues
- Clean buffer operations
- Predictable behavior

## Testing

The terminal-level implementation means:
- All existing tests continue to work (54 tests pass ✅)
- No test modifications needed
- Shortcuts work in all examples without changes
- New examples automatically inherit functionality

## Future Extensions

This pattern can be extended for other global shortcuts:

```rust
// In Terminal::poll_event()
match key_code {
    KB_F12 => {
        self.flash();
        self.dump_screen("screen-dump.txt")?;
        return Ok(None);
    }
    KB_F11 => {
        self.flash();
        if let Some(bounds) = self.active_view_bounds {
            self.dump_region(..., "active-view-dump.txt")?;
        }
        return Ok(None);
    }
    _ => Ok(Some(Event::keyboard(key_code)))
}
```

## Conclusion

Implementing screen dump shortcuts at the Terminal level is the correct architectural decision because:

1. **It works everywhere** without integration
2. **It's maintainable** with single implementation
3. **It's reliable** and can't be blocked
4. **It's clean** with proper separation of concerns
5. **It's extensible** for future global shortcuts

This approach transforms a feature that would normally require careful integration throughout the codebase into one that "just works" automatically.
