# Command Set System Implementation

## Overview

The Command Set system provides automatic button enable/disable based on application state. This matches Borland Turbo Vision's architecture where buttons automatically disable themselves when their associated commands are not available.

## Implementation Status

### ✅ Completed

1. **CommandSet Struct** (`src/core/command_set.rs`)
   - Bitfield-based storage for 65,536 commands
   - Individual command enable/disable
   - Range-based operations
   - Set operations (union, intersect)
   - Full test coverage
   - Matches Borland's `TCommandSet` implementation

2. **Command Constants** (`src/core/command.rs`)
   - Added `CM_COMMAND_SET_CHANGED = 52`
   - Added related broadcast commands (CM_RECEIVED_FOCUS, etc.)

3. **Event System Updates** (`src/core/event.rs`)
   - Added `Event::broadcast(cmd)` constructor
   - EventType::Broadcast already existed

### 🚧 In Progress / TODO

4. **Global Command Set Storage**
   - Need to add to Application struct
   - Borland stores in `TView::curCommandSet` (static)
   - We should store in Application and pass reference through

5. **Command Set API Methods**
   - Need methods on Application/View:
     - `enable_command(cmd)`
     - `disable_command(cmd)`
     - `command_enabled(cmd) -> bool`
     - `set_command_set_changed()`

6. **Idle Processing & Broadcast**
   - Need Application::idle() method
   - Check if command_set_changed flag is set
   - Broadcast CM_COMMAND_SET_CHANGED to all views

7. **Button Auto-Disable**
   - Update Button constructor to check `command_enabled()`
   - Add broadcast handler for CM_COMMAND_SET_CHANGED
   - Auto-update disabled state when command set changes

8. **Example/Demo**
   - Create example showing command enable/disable
   - Demonstrate automatic button state updates

## Architecture Design

### Borland's Approach

```cpp
// Global static in TView
static TCommandSet curCommandSet;
static Boolean commandSetChanged;

// Any view can modify
void TView::enableCommand(ushort cmd) {
    commandSetChanged = True;
    curCommandSet += cmd;
}

// Program broadcasts changes
void TProgram::idle() {
    if (commandSetChanged) {
        message(this, evBroadcast, cmCommandSetChanged, 0);
        commandSetChanged = False;
    }
}

// Buttons respond
case cmCommandSetChanged:
    if (command enabled changed) {
        setState(sfDisabled, !commandEnabled(command));
        drawView();
    }
```

### Rust Approach (Proposed)

```rust
// Store in Application
pub struct Application {
    command_set: CommandSet,
    command_set_changed: bool,
    // ...
}

// Views access via Application reference
impl Application {
    pub fn enable_command(&mut self, cmd: CommandId) {
        self.command_set_changed = true;
        self.command_set.enable_command(cmd);
    }

    pub fn command_enabled(&self, cmd: CommandId) -> bool {
        self.command_set.has(cmd)
    }

    fn idle(&mut self) {
        if self.command_set_changed {
            // Broadcast to all views
            let event = Event::broadcast(CM_COMMAND_SET_CHANGED);
            self.desktop.handle_event(&mut event);
            self.command_set_changed = false;
        }
    }
}

// Button responds
EventType::Broadcast => {
    if event.command == CM_COMMAND_SET_CHANGED {
        // Check if command enabled status changed
        let should_be_disabled = !app.command_enabled(self.command);
        if should_be_disabled != self.is_disabled() {
            self.set_disabled(should_be_disabled);
        }
    }
}
```

### Challenge: Accessing Application from Button

**Problem**: Buttons need to query `command_enabled()` but don't have access to Application reference.

**Possible Solutions**:

1. **Pass Application reference through handle_event**
   - Change signature: `fn handle_event(&mut self, event: &mut Event, app: &Application)`
   - Pros: Clean, explicit
   - Cons: Breaking change to all handle_event implementations

2. **Store command enabled state in Event**
   - Add `enabled_commands: Option<&CommandSet>` to Event
   - Pros: No signature change
   - Cons: Lifetime complexities

3. **Use thread-local static (like Borland)**
   - `static COMMAND_SET: RefCell<CommandSet>`
   - Pros: Matches Borland exactly
   - Cons: Global mutable state (not Rust-idiomatic)

4. **Lazy implementation: Store enabled state in Button**
   - Button remembers last known command enabled state
   - On CM_COMMAND_SET_CHANGED, button requests update from parent
   - Pros: Simple, works now
   - Cons: Not exactly like Borland

## Recommended Next Steps

### Phase 1: Basic Implementation (Minimal)

1. Add `command_set` and `command_set_changed` to Application
2. Add `enable_command()`, `disable_command()` methods to Application
3. Create example that manually broadcasts CM_COMMAND_SET_CHANGED
4. Update Button to handle CM_COMMAND_SET_CHANGED (with stored enabled state)

### Phase 2: Automatic Broadcasting

5. Add Application::idle() that auto-broadcasts changes
6. Integrate idle() into main event loop

### Phase 3: Full Integration

7. Add command set initialization (disable certain commands by default)
8. Add range and set operations to API
9. Add examples for common patterns (clipboard, undo/redo)

## Usage Example (Target API)

```rust
// In application setup
app.disable_command(CM_PASTE);  // No clipboard content yet
app.disable_command(CM_UNDO);   // Nothing to undo

// User copies text
clipboard.set_text("Hello");
app.enable_command(CM_PASTE);  // Button automatically enables!

// User performs action
perform_action();
app.enable_command(CM_UNDO);   // Undo button lights up!

// User undoes
undo();
if !can_undo_more() {
    app.disable_command(CM_UNDO);  // Button automatically disables!
}
```

## Files

### Implemented
- `src/core/command_set.rs` - CommandSet bitfield implementation
- `src/core/command.rs` - Command constants (CM_COMMAND_SET_CHANGED)
- `src/core/event.rs` - Event::broadcast() constructor

### Need Implementation
- `src/app/application.rs` - Add command set storage and API
- `src/views/button.rs` - Add CM_COMMAND_SET_CHANGED handler
- `examples/command_set_demo.rs` - Demonstration example

## References

- Borland: `/local-only/borland-tvision/include/tv/cmdset.h`
- Borland: `/local-only/borland-tvision/classes/tcommand.cc`
- Borland: `/local-only/borland-tvision/classes/tview.cc` (lines 142-389)
- Borland: `/local-only/borland-tvision/classes/tbutton.cc` (lines 255-262)
- Borland: `/local-only/borland-tvision/classes/tprogram.cc` (lines 248-257)

## Notes

The command set system is a powerful pattern for managing UI state consistency. When fully implemented, it eliminates the need for manual button enable/disable code throughout the application. Buttons "just work" based on application state.
