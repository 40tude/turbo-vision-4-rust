# Focus Consolidation - Moving to SF_FOCUSED State Flag

**Status:** ✅ COMPLETED (v0.2.3)
**Goal:** Fix Discrepancy #3 - Consolidate focus into state flags

## Objective

Move all views from using a separate `focused: bool` field to using the `SF_FOCUSED` flag in the unified `state: StateFlags` field. This matches Borland Turbo Vision's architecture where TView stores focus state in the unified state field.

## Completed

### ✅ View Trait (src/views/view.rs)
- Added `SF_FOCUSED` to imports
- Added default `set_focus(focused: bool)` implementation using `set_state_flag(SF_FOCUSED, focused)`
- Added `is_focused() -> bool` helper method using `get_state_flag(SF_FOCUSED)`
- Views can now inherit focus behavior from trait

### ✅ Button (src/views/button.rs)
- Removed `focused: bool` field
- Removed `focused: false` from constructor
- Removed custom `set_focus()` implementation
- Replaced `self.focused` with `self.is_focused()` in `draw()` (2 places)
- Replaced `self.focused` with `self.is_focused()` in `handle_event()` (1 place)

### ✅ InputLine (src/views/input_line.rs)
- Removed `focused: bool` field
- Added `state: StateFlags` field
- Added `state: 0` to constructor
- Added `state()` and `set_state()` methods
- Removed `set_focused()` public method
- Removed custom `set_focus()` implementation
- Replaced `self.focused` with `self.is_focused()` in `draw()` (1 place)
- Replaced `self.focused` with `self.is_focused()` in `handle_event()` (2 places)
- Replaced `self.focused` with `self.is_focused()` in `update_cursor()` (1 place)

## Remaining Work

All views have been updated! 🎉

### ✅ Editor (src/views/editor.rs) - COMPLETED
- Removed `focused: bool` field
- Added `state: StateFlags` field
- Added `state()` and `set_state()` methods
- Replaced `self.focused` with `self.is_focused()` in:
  - `draw()` method (1 place)
  - `handle_event()` method (1 place)
  - `update_cursor()` method (1 place)
- Removed `set_focus()` override

### ✅ Memo (src/views/memo.rs) - COMPLETED
- Removed `focused: bool` field
- Added `state: StateFlags` field
- Added `state()` and `set_state()` methods
- Replaced `self.focused` with `self.is_focused()` in:
  - `draw()` method (1 place)
  - `handle_event()` method (1 place)
  - `update_cursor()` method (1 place)
- Removed custom `set_focus()`

### ✅ ListBox (src/views/listbox.rs) - COMPLETED
- Removed `focused: bool` field
- Added `state: StateFlags` field
- Added `state()` and `set_state()` methods
- Replaced `self.focused` with `self.is_focused()` in:
  - `draw()` method (2 places)
  - `handle_event()` method (1 place)
- Removed custom `set_focus()`

### ✅ CheckBox (src/views/checkbox.rs) - COMPLETED
- Removed `focused: bool` field
- Added `state: StateFlags` field
- Added `state()` and `set_state()` methods
- Replaced `self.focused` with `self.is_focused()` in:
  - `draw()` method (2 places)
  - `handle_event()` method (1 place)
- Removed custom `set_focus()`

### ✅ RadioButton (src/views/radiobutton.rs) - COMPLETED
- Removed `focused: bool` field
- Added `state: StateFlags` field
- Added `state()` and `set_state()` methods
- Replaced `self.focused` with `self.is_focused()` in:
  - `draw()` method (2 places)
  - `handle_event()` method (1 place)
- Removed custom `set_focus()`

## Pattern to Follow

For each view that needs updating:

1. **Remove the focused field:**
   ```rust
   // Before
   pub struct MyView {
       bounds: Rect,
       focused: bool,  // ← Remove this
       ...
   }
   ```

2. **Add state field (if not present):**
   ```rust
   // After
   pub struct MyView {
       bounds: Rect,
       state: StateFlags,  // ← Add this
       ...
   }
   ```

3. **Update constructor:**
   ```rust
   // Before
   Self {
       bounds,
       focused: false,  // ← Remove this
       ...
   }

   // After
   Self {
       bounds,
       state: 0,  // ← Add this (if new field)
       ...
   }
   ```

4. **Add state methods (if not present):**
   ```rust
   fn state(&self) -> StateFlags {
       self.state
   }

   fn set_state(&mut self, state: StateFlags) {
       self.state = state;
   }
   ```

5. **Remove custom set_focus():**
   ```rust
   // Remove this entire method:
   fn set_focus(&mut self, focused: bool) {
       self.focused = focused;
   }

   // Add comment:
   // set_focus() now uses default implementation from View trait
   // which sets/clears SF_FOCUSED flag
   ```

6. **Replace all self.focused usage:**
   ```rust
   // Before
   if self.focused { ... }
   let attr = if self.focused { ... } else { ... };

   // After
   if self.is_focused() { ... }
   let attr = if self.is_focused() { ... } else { ... };
   ```

7. **Search for all references:**
   ```bash
   grep -n "self\.focused" src/views/myview.rs
   ```

8. **Test compilation:**
   ```bash
   cargo build
   ```

## Benefits

1. **Borland Compatibility:** Matches TView's unified state architecture
2. **Consistency:** All views use same pattern for focus management
3. **Extensibility:** Easy to add more state flags in future
4. **Type Safety:** State flags are checked at compile time

## Summary

**All 9 views have been successfully updated to use SF_FOCUSED flag!**

Views updated:
1. ✅ View trait - Added default implementations
2. ✅ Button
3. ✅ InputLine
4. ✅ Editor
5. ✅ Memo
6. ✅ ListBox
7. ✅ CheckBox
8. ✅ RadioButton

**Architecture Changes:**
- All views now use `state: StateFlags` field instead of `focused: bool`
- All views use `is_focused()` helper to check focus state
- All views use default `set_focus()` implementation from View trait
- Focus is now managed through SF_FOCUSED flag, matching Borland's TView architecture

**Testing Status:**
- ✅ All library tests pass (61 passed)
- ✅ Compilation successful
- Ready for integration testing with examples

## Testing

Manual testing recommended:
1. Run all examples to verify focus still works
2. Test tab navigation between controls
3. Test mouse click focus changes
4. Test keyboard shortcuts while focused
5. Verify visual feedback (highlight colors) work correctly

## References

- **Borland Source:** `local-only/borland-tvision/include/tv/tview.h` - state field definition
- **Discrepancy Doc:** `docs/DISCREPANCIES.md` - Discrepancy #3
