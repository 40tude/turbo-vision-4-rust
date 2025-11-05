# Dialog Close Button Fix

## Issue

Dialog close buttons (×) were not working reliably. Clicking the close button on a modal dialog would not close the dialog.

## Root Cause

In `src/views/frame.rs`, the event handling order was incorrect:

```rust
// BROKEN CODE (before fix)
} else if event.what == EventType::MouseUp {
    // Step 1: Clear drag/resize state
    if (self.state & SF_DRAGGING) != 0 {
        self.state &= !SF_DRAGGING;
        event.clear();  // ← Event cleared here!
    }
}

// Step 2: Check close button (but event is already cleared!)
if event.what == EventType::MouseUp {
    if mouse on close button {
        *event = Event::command(CM_CLOSE);  // ← Never executed!
    }
}
```

**Problem:** The close button check happened AFTER the event was potentially cleared by the drag/resize cleanup code. If there was any drag state (even from accidental mouse movement), the event would be cleared before the close button check could run.

## Fix

Reordered the event handling to check the close button FIRST:

```rust
// FIXED CODE (after fix)
} else if event.what == EventType::MouseUp {
    // Step 1: Check close button FIRST (before anything else)
    let mouse_pos = event.mouse.pos;
    if mouse_pos.y == self.bounds.a.y
        && mouse_pos.x >= self.bounds.a.x + 2
        && mouse_pos.x <= self.bounds.a.x + 4
    {
        // Generate close command
        *event = Event::command(CM_CLOSE);
        // Also clear drag/resize state if set
        self.state &= !(SF_DRAGGING | SF_RESIZING);
        return;  // ← Exit early with CM_CLOSE
    }

    // Step 2: Handle drag/resize cleanup (only if not close button)
    if (self.state & SF_DRAGGING) != 0 {
        self.state &= !SF_DRAGGING;
        event.clear();
    }
}
```

## Event Flow After Fix

When clicking a dialog close button:

1. **Frame** receives MouseUp on close button
2. **Frame** checks close button position FIRST (line 187-190)
3. **Frame** generates CM_CLOSE command (line 192)
4. **Frame** clears any drag/resize state (line 194)
5. **Frame** returns early (line 195) - skips drag cleanup
6. **Window** receives CM_CLOSE command
7. **Window** sees SF_MODAL flag is set (dialogs are modal)
8. **Window** converts CM_CLOSE → CM_CANCEL (line 359)
9. **Dialog** receives CM_CANCEL command
10. **Dialog** calls end_modal(CM_CANCEL) (line 188)
11. **Dialog** execute loop checks end_state (line 118)
12. **Dialog** breaks from event loop and returns result

## Files Changed

- **`src/views/frame.rs`** (lines 183-206)
  - Moved close button check before drag/resize cleanup
  - Added early return after generating CM_CLOSE
  - Ensures close button always works, even with accidental drag

## Testing

All dialog close buttons now work correctly:

✅ Biorhythm calculator dialog - close button works
✅ About dialog - close button works
✅ All modal dialogs - close buttons work
✅ Window dragging still works
✅ Resize still works
✅ No regressions in other functionality

## Impact

This fix affects ALL windows and dialogs in the Turbo Vision library:

- Modal dialogs can now be reliably closed with the × button
- Non-modal windows can be closed with the × button
- No impact on keyboard shortcuts (Esc still works)
- No impact on Cancel buttons in dialogs
- Improved UX - close button is more reliable

## Technical Details

The fix prioritizes the close button check over drag/resize state cleanup. This is the correct behavior because:

1. **Close is a deliberate action** - user explicitly clicked the × button
2. **Drag state is ephemeral** - temporary state that should be cleared
3. **Close takes precedence** - if mouse ends on close button, user wants to close
4. **Matches Borland behavior** - original Turbo Vision worked this way

## Related Issues

This fix resolves the issue where dialog close buttons appeared non-functional in the biorhythm example. The Cancel button worked, but the × button did not.

The fix is in the core library (`frame.rs`), so it benefits all applications using Turbo Vision, not just the biorhythm example.
