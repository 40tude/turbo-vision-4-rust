# FileDialog Navigation and Focus Fixes

**Date:** 2025-11-02
**Status:** ✅ Complete
**Original Issues:** Navigation off-by-one, Enter key not working on folders, focus "limbo" after directory change

---

## Executive Summary

Three critical bugs in the FileDialog navigation system have been identified and fixed. All issues stemmed from improper event handling and focus management that didn't match the original Borland Turbo Vision architecture.

**Quick Summary:**
1. **Double Event Processing** → Events were processed twice (FileDialog + ListBox)
2. **Missing Initial Broadcast** → First item after directory change wasn't broadcast to InputLine
3. **Broken Focus Chain** → Manual `set_focus()` didn't update Group's `focused` index

All fixes are complete and the FileDialog now operates identically to the original Borland Turbo Vision implementation.

---

## Issue #1: Double Event Processing

### Symptoms
- Pressing Up/Down moved by 2 positions instead of 1
- InputLine showed wrong file (off by one)
- ENTER key didn't work correctly on selected items

### Root Cause

The FileDialog was intercepting events **before** passing them to children:

```rust
// ❌ WRONG: Double processing
self.track_listbox_events(&event);  // FileDialog manually updates selection (0→1)
self.dialog.handle_event(&mut event);  // ListBox ALSO processes event (1→2)
```

**Flow:**
1. User presses Down key
2. FileDialog's `track_listbox_events()` increments selection: 0 → 1
3. Event passed to Dialog → ListBox
4. ListBox ALSO increments selection: 1 → 2
5. **Result:** Selection moved by 2 positions!

### Solution

Let the ListBox handle its own events, then read the result:

```rust
// ✅ CORRECT: Single processing
self.dialog.handle_event(&mut event);  // ListBox handles event (0→1)
self.sync_inputline_with_listbox();  // Read result and update InputLine
```

**Changes:**
- **src/views/file_dialog.rs:192-197** - Reordered event flow
- **src/views/file_dialog.rs:259-304** - Replaced complex `track_listbox_events()` (100+ lines) with simple `sync_inputline_with_listbox()` (40 lines)

### Files Modified
- `src/views/file_dialog.rs` - Event processing order and sync logic
- `src/views/view.rs:93-97` - Added `get_list_selection()` trait method
- `src/views/listbox.rs:311-313` - Implemented `get_list_selection()`

---

## Issue #2: InputLine Not Updating After Directory Navigation

### Symptoms
- After navigating into a directory, the InputLine didn't show the first file
- InputLine remained blank or showed the previous directory's selection

### Root Cause

The `rebuild_and_redraw()` method recreated the dialog with new files but never broadcast the initial selection to the InputLine.

In Borland's implementation:
```cpp
// tfilelis.cc:588-595
newList(fileList);
if (list()->getCount() > 0)
   message(owner, evBroadcast, cmFileFocused, list()->at(0));  // ← Broadcasts first item!
```

Our implementation was missing this broadcast after `rebuild_and_redraw()`.

### Solution

Added broadcast of initial selection after directory navigation:

```rust
// src/views/file_dialog.rs:384-411
if !self.files.is_empty() {
    let first_item = self.files[0].clone();

    // Format display text (directories show "dirname/*.txt" format)
    let display_text = format_for_display(&first_item, &self.wildcard);

    // Update shared data
    *self.file_name_data.borrow_mut() = display_text;

    // Broadcast to InputLine
    let mut broadcast = Event::broadcast(CM_FILE_FOCUSED);
    self.dialog.handle_event(&mut broadcast);
}
```

### Files Modified
- `src/views/file_dialog.rs:384-414` - Added initial selection broadcast in `rebuild_and_redraw()`

---

## Issue #3: Focus "Limbo" State

### Symptoms
- After navigating to a directory, ListBox **appeared** focused (correct colors)
- But keyboard navigation didn't work until Tab was pressed
- ListBox was in "limbo" - visual focus but no logical focus

### Root Cause

The most subtle and critical bug. Manual `set_focus()` calls only updated the child's visual state, not the Group's internal `focused` index:

```rust
// ❌ WRONG: Only sets visual focus
for i in 0..self.dialog.child_count() {
    self.dialog.child_at_mut(i).set_focus(false);
}
self.dialog.child_at_mut(CHILD_LISTBOX).set_focus(true);  // ← Doesn't update Group state!
```

**What happened:**
1. ListBox's `focused` field set to `true` → **correct colors** ✅
2. Group's `focused` index still pointed to InputLine → **events went to InputLine** ❌
3. User pressed arrow keys → InputLine received events → no visible response
4. Pressing Tab triggered `select_next()` → Group updated its `focused` index → keyboard worked ✅

This is because `Group::handle_event()` routes keyboard events based on its `focused` index:

```rust
// src/views/group.rs - Event routing
if self.focused < self.children.len() {
    self.children[self.focused].handle_event(event);  // ← Uses focused INDEX
}
```

**Borland's approach:**
```cpp
// tfiledia.cc:275,287
fileList->select();  // Calls owner->setCurrent(this, normalSelect)

// tview.cc:658-664
void TView::select() {
    if (owner != 0)
        owner->setCurrent(this, normalSelect);  // ← Updates owner's current pointer!
}
```

Borland's `select()` updates the owner's `current` pointer (equivalent to our `focused` index).

### Solution

Added proper focus delegation that updates both visual state and Group index:

```rust
// ✅ CORRECT: Updates both visual and logical focus
self.dialog.set_focus_to_child(CHILD_LISTBOX);
```

**Implementation:**

1. **src/views/window.rs:42-49** - Added `set_focus_to_child()`
```rust
pub fn set_focus_to_child(&mut self, index: usize) {
    self.interior.clear_all_focus();  // Clear visual focus
    self.interior.set_focus_to(index);  // Update focused index + visual focus
}
```

2. **src/views/dialog.rs:30-34** - Exposed method
```rust
pub fn set_focus_to_child(&mut self, index: usize) {
    self.window.set_focus_to_child(index);
}
```

3. **src/views/file_dialog.rs:375-379** - Used proper focus method
```rust
// Matches Borland: fileList->select() calls owner->setCurrent(this, normalSelect)
if CHILD_LISTBOX < self.dialog.child_count() {
    self.dialog.set_focus_to_child(CHILD_LISTBOX);  // ✅ Proper focus chain!
    self.dialog.child_at_mut(CHILD_LISTBOX).set_list_selection(0);
}
```

### Files Modified
- `src/views/window.rs:42-49` - Added `set_focus_to_child()`
- `src/views/dialog.rs:30-34` - Exposed `set_focus_to_child()`
- `src/views/file_dialog.rs:371-379` - Used proper focus method in `rebuild_and_redraw()`

---

## Architecture Insights

### Borland's Focus Management Pattern

```
View Layer                 Container Layer
┌─────────────┐           ┌──────────────┐
│  FileList   │           │   TGroup     │
│             │           │              │
│ select()────┼──────────>│ current ───> │ (updates focused pointer)
│             │           │              │
│             │<──────────┼ setState()   │ (sets visual focus)
└─────────────┘           └──────────────┘
```

Borland uses `owner->setCurrent()` to establish the focus chain:
1. Child calls `owner->setCurrent(this, normalSelect)`
2. Owner updates its `current` pointer
3. Owner calls `setState(sfFocused, true)` on child
4. Result: Both logical routing and visual state are synchronized

### Our Rust Implementation

```
View Layer                 Container Layer
┌─────────────┐           ┌──────────────┐
│  ListBox    │           │   Group      │
│             │           │              │
│             │           │ focused: 4 ─>│ (index of focused child)
│             │           │              │
│ focused:    │<──────────┼ set_focus()  │ (sets visual state)
│   true      │           │              │
└─────────────┘           └──────────────┘
```

We replicate this with `set_focus_to_child()`:
1. Dialog calls `set_focus_to_child(index)`
2. Group updates its `focused` index
3. Group calls `set_focus(true)` on child
4. Result: Synchronized logical routing and visual state

---

## Key Learnings

### 1. Event Flow Must Be Unidirectional

**❌ Bad:** Parent intercepts events → modifies state → passes to child → child also modifies state
**✅ Good:** Parent passes events to child → child handles event → parent reads result

### 2. Visual State ≠ Logical State

Having `focused: true` on a child is insufficient. The container must also know which child is focused for event routing.

### 3. Focus Chain Requires Bidirectional Update

- **Top-down:** Container tells child "you have focus" via `set_focus(true)`
- **Bottom-up:** Container updates its own routing state (focused index)
- **Borland:** Achieves this with `owner->setCurrent()`
- **Rust:** Achieves this with `set_focus_to_child()`

### 4. State Synchronization Is Critical

When recreating UI components (like `rebuild_and_redraw()`), ALL state must be restored:
- Visual appearance (colors, focus highlights)
- Logical routing (focused index)
- Data broadcast (initial selection)

---

## Testing Checklist

After these fixes, the FileDialog should:

- ✅ Navigate up/down by exactly 1 position per keypress
- ✅ Show correct file in InputLine at all times
- ✅ Respond to ENTER key on folders by navigating into them
- ✅ Keep focus on ListBox after directory navigation
- ✅ Respond to keyboard immediately (no Tab needed)
- ✅ Handle mouse clicks and double-clicks correctly
- ✅ Support PgUp/PgDn, Home/End navigation
- ✅ Update InputLine when navigating with arrows
- ✅ Show first file after entering directory

---

## Related Documentation

- **FOCUS_ARCHITECTURE.md** - Focus management system and best practices
- **DISCREPANCIES.md** - Differences from Borland Turbo Vision
- **TO-DO-LIST.md** - FileDialog implementation notes and recent updates

---

## Borland Reference Code

Key files from original implementation:
- `tfiledia.cc:251-302` - TFileDialog::valid() navigation logic
- `tfiledia.cc:275,287` - fileList->select() calls
- `tfilelis.cc:73-76` - TFileList::focusItem() broadcasts
- `tfilelis.cc:588-595` - readDirectory() initial broadcast
- `tview.cc:658-664` - TView::select() calls owner->setCurrent()
- `tgroup.cc` - TGroup::setCurrent() and focusView()
- `tlistvie.cc` - TListViewer::handleEvent() keyboard handling

---

## Commit Reference

All fixes were implemented in a single session on 2025-11-02. Key changes:

1. **Event Processing** - Removed pre-interception, let ListBox handle events
2. **Sync Logic** - Replaced complex tracking with simple post-event sync
3. **Focus Chain** - Added `set_focus_to_child()` hierarchy
4. **Initial Broadcast** - Added broadcast after directory navigation

Total lines changed: ~150 lines modified/added across 5 files

**Result:** FileDialog now matches original Borland Turbo Vision behavior exactly.
