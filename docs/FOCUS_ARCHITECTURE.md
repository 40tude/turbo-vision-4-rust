# Focus Architecture

## Overview

The Turbo Vision framework implements a proper focus management system where controls only respond to input when they have focus. This document explains the architecture and guidelines for implementing focus-aware controls.

## Principles

### 1. **Only Focused Controls Handle Keyboard Input**

Controls should only process keyboard events when they have focus. This prevents:
- Input fields from capturing keys when not focused
- List boxes from scrolling when another control is active
- Buttons from activating when user is typing elsewhere

### 2. **Mouse Events Go to the Control Under the Mouse**

Unlike keyboard events, mouse events are sent to the control at the mouse position, regardless of focus state. However, clicking a focusable control automatically gives it focus.

### 3. **Tab Key Cycles Through Focusable Controls**

The Tab key is handled at the Group level to cycle focus between focusable children.

## Implementation

### Group-Level Event Routing

The `Group` class implements the core focus management logic in its `handle_event` method:

```rust
fn handle_event(&mut self, event: &mut Event) {
    // Tab key cycles focus
    if event.what == EventType::Keyboard && event.key_code == KB_TAB {
        self.select_next();
        event.clear();
        return;
    }

    // Mouse events: send to child under mouse
    if event.what == EventType::MouseDown || ... {
        // Find child at mouse position
        // If clicked on focusable child, give it focus
        // Send event to that child
    }

    // Keyboard events: only send to focused child
    if self.focused < self.children.len() {
        self.children[self.focused].handle_event(event);
    }
}
```

**Key Points:**
- Tab is handled at Group level
- Mouse events find the child at mouse position
- Keyboard events only go to the focused child
- Clicking a focusable control gives it focus

### Control-Level Focus Checks

Each focusable control must check its focus state before handling keyboard input:

```rust
fn handle_event(&mut self, event: &mut Event) {
    if event.what == EventType::Keyboard {
        // Check focus before processing keyboard input
        if !self.focused {
            return;
        }

        // Process keyboard events...
    }

    // Mouse events don't need focus check
    if event.what == EventType::MouseDown {
        // Process click...
    }
}
```

### Controls That Check Focus

The following controls properly check focus before handling keyboard events:

- ✅ **InputLine** - Text input field
- ✅ **ListBox** - Scrollable list
- ✅ **Button** - Push button
- ✅ **CheckBox** - Checkbox control
- ✅ **RadioButton** - Radio button control
- ✅ **Editor** - Text editor
- ✅ **Memo** - Multi-line text input

### Focus State Management

Controls that can receive focus must:

1. **Implement `can_focus()` to return `true`**:
```rust
fn can_focus(&self) -> bool {
    true
}
```

2. **Implement `set_focus()` to track focus state**:
```rust
fn set_focus(&mut self, focused: bool) {
    self.focused = focused;
}
```

3. **Have a `focused: bool` field** in their struct

## Example: FileDialog Focus Behavior

In a FileDialog:
- User tabs to the InputLine → InputLine gets focus
- User types → only InputLine responds, ListBox below doesn't scroll
- User tabs to ListBox → ListBox gets focus
- User presses arrow keys → ListBox scrolls, InputLine doesn't receive keys
- User clicks on ListBox → ListBox gets focus automatically
- User presses Enter on folder → FileDialog navigates, stays open

## Testing Focus Behavior

To test proper focus handling:

1. Create a dialog with multiple focusable controls
2. Give focus to first control (input field)
3. Press keys that both controls respond to (e.g., arrow keys)
4. Verify only the focused control responds
5. Tab to next control
6. Verify focus changed and keys now go to new control

## Common Mistakes

❌ **DON'T** pass events to all children and let them decide
```rust
// BAD: All children get events
for child in &mut self.children {
    child.handle_event(event);
}
```

✅ **DO** only send keyboard events to focused child
```rust
// GOOD: Only focused child gets keyboard events
if self.focused < self.children.len() {
    self.children[self.focused].handle_event(event);
}
```

❌ **DON'T** forget to check focus in control's handle_event
```rust
// BAD: Control processes all keyboard input
fn handle_event(&mut self, event: &mut Event) {
    if event.what == EventType::Keyboard {
        // Process keys...
    }
}
```

✅ **DO** check focus before processing keyboard input
```rust
// GOOD: Only process keys when focused
fn handle_event(&mut self, event: &mut Event) {
    if event.what == EventType::Keyboard {
        if !self.focused {
            return;
        }
        // Process keys...
    }
}
```

## Related Classes

- **Group** (`src/views/group.rs`) - Container with focus management
- **Window** (`src/views/window.rs`) - Wraps Group, delegates focus
- **Dialog** (`src/views/dialog.rs`) - Modal dialog with focus management
- **View trait** (`src/views/view.rs`) - Defines `can_focus()` and `set_focus()`

## Programmatic Focus Control

### Setting Focus to Specific Child

When a dialog needs to set focus to a specific child (e.g., after refreshing contents), use the `set_focus_to_child()` method:

```rust
// FileDialog after directory navigation
self.dialog.set_focus_to_child(CHILD_LISTBOX);
```

This method properly:
1. Clears focus from all children
2. Updates the Group's internal `focused` index
3. Calls `set_focus(true)` on the target child

**⚠️ IMPORTANT:** Do NOT manually call `set_focus()` on individual children without updating the Group's `focused` index:

```rust
// ❌ BAD: Only sets visual focus, Group still thinks another child is focused
self.dialog.child_at_mut(index).set_focus(true);

// ✅ GOOD: Updates both Group state and child focus
self.dialog.set_focus_to_child(index);
```

**Symptoms of improper focus management:**
- Control appears focused (correct colors) but doesn't respond to keyboard
- Need to press Tab before keyboard events work
- Events go to wrong control

This matches Borland's `fileList->select()` pattern which calls `owner->setCurrent(this, normalSelect)` to properly establish focus chain.

## Future Enhancements

Possible improvements to the focus system:

1. **Shift+Tab** for reverse focus navigation
2. **Focus indicators** - visual highlighting of focused control
3. **Focus groups** - nested focus management within complex controls
4. **Focus events** - callbacks when focus changes
