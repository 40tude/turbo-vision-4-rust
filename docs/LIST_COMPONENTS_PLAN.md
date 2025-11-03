# List Components Implementation Plan

## Overview

This document outlines the implementation of List Components from MISSING_FEATURES.md:
- TListViewer (16h) - Base for list views
- TMenuView (12h) - Base for menu views
- TMenuBox (10h) - Popup menu container

## Borland Architecture Analysis

### TListViewer (lstviewr.h)
**Purpose**: Abstract base class for scrollable list views

**Key Features**:
- Manages scroll bars (horizontal and vertical)
- Tracks focused item and top visible item
- Handles keyboard/mouse navigation
- Abstract `getText()` method for subclasses to implement
- Support for multi-column lists
- Range management (total number of items)

**Key Fields**:
```cpp
TScrollBar *hScrollBar, *vScrollBar;
short numCols;           // Number of columns
ccIndex topItem;         // First visible item
ccIndex focused;         // Currently focused item
ccIndex range;           // Total number of items
Boolean handleSpace;     // Whether space bar selects items
```

**Key Methods**:
```cpp
virtual void getText(char *dest, ccIndex item, short maxLen);  // Get item text
virtual Boolean isSelected(ccIndex item);                       // Check if selected
virtual void selectItem(ccIndex item);                          // Select item
void focusItem(ccIndex item);                                   // Focus item
void focusItemCentered(ccIndex item);                           // Center focused item
void setRange(ccIndex aRange);                                  // Set total items
```

**Inheritance**:
- TListViewer extends TView
- TListBox extends TListViewer

### TMenuView (menuview.h)
**Purpose**: Abstract base class for menu views (menu bar, popup menus)

**Key Features**:
- Manages menu items and navigation
- Handles hot keys and accelerators
- Supports nested menus (parent/child relationship)
- Modal execution with `execute()` method
- Tracks current menu item

**Key Fields**:
```cpp
TMenuView *parentMenu;   // Parent menu (for submenus)
TMenu *menu;             // Menu data structure
TMenuItem *current;      // Currently selected item
char compactMenu;        // Compact menu flag
```

**Key Methods**:
```cpp
ushort execute();                                // Execute menu (modal loop)
TMenuItem *findItem(char ch);                   // Find item by accelerator
TMenuItem *hotKey(ushort keyCode);              // Check for hot key
TRect getItemRect(TMenuItem *item);             // Get item bounds
TMenuView *newSubView(...);                     // Create submenu view
```

**Inheritance**:
- TMenuView extends TView
- TMenuBar extends TMenuView
- TMenuBox extends TMenuView

### TMenuBox (menubox.h)
**Purpose**: Popup/dropdown menu implementation

**Key Features**:
- Draws menu items with borders
- Handles mouse tracking
- Displays shortcuts right-aligned
- Shows submenu arrow indicators
- Manages menu box frame

**Key Methods**:
```cpp
void draw();                                    // Draw menu box
TRect getItemRect(TMenuItem *item);             // Get item rectangle
```

## Rust Implementation Strategy

### Challenge: Trait vs. Inheritance

Borland uses class inheritance:
```
TView
 ├─ TListViewer
 │   └─ TListBox
 └─ TMenuView
     ├─ TMenuBar
     └─ TMenuBox
```

Rust doesn't have inheritance. We have several options:

#### Option 1: Trait-Based (Recommended)
Create traits that components can implement:

```rust
pub trait ListViewer: View {
    fn get_text(&self, item: usize, max_len: usize) -> String;
    fn is_selected(&self, item: usize) -> bool;
    fn select_item(&mut self, item: usize);
    fn focused_item(&self) -> Option<usize>;
    fn set_focused_item(&mut self, item: Option<usize>);
    fn item_count(&self) -> usize;
    fn set_item_count(&mut self, count: usize);
    // ... other methods with default implementations
}

pub trait MenuViewer: View {
    fn get_menu(&self) -> &Menu;
    fn get_menu_mut(&mut self) -> &mut Menu;
    fn current_item(&self) -> Option<usize>;
    fn set_current_item(&mut self, item: Option<usize>);
    fn find_hot_key(&self, key_code: KeyCode) -> Option<usize>;
    // ... other methods with default implementations
}
```

**Pros**:
- Idiomatic Rust
- Composition over inheritance
- Flexible (can implement multiple traits)

**Cons**:
- Can't share field storage (each impl must define own fields)
- More boilerplate for similar components

#### Option 2: Composition with Helper Structs
Create helper structs that components can embed:

```rust
pub struct ListViewerState {
    pub top_item: usize,
    pub focused: Option<usize>,
    pub range: usize,
    pub num_cols: u16,
    // ... other fields
}

impl ListViewerState {
    pub fn focus_item(&mut self, item: usize) { ... }
    pub fn scroll_to(&mut self, item: usize) { ... }
    // ... helper methods
}

pub struct ListBox {
    // Embed the state
    list_state: ListViewerState,
    items: Vec<String>,
    // ... other fields
}
```

**Pros**:
- Shares field storage
- Reusable logic in helper methods
- Clear separation of concerns

**Cons**:
- Not as polymorphic as traits
- More verbose access (list_state.focused vs self.focused)

#### Option 3: Hybrid Approach (Best of Both)
Combine traits for polymorphism with helper structs for shared logic:

```rust
// Helper struct for shared state/logic
pub struct ListViewerState {
    pub top_item: usize,
    pub focused: Option<usize>,
    pub range: usize,
    // ... helper methods
}

// Trait for polymorphism
pub trait ListViewer: View {
    fn list_state(&self) -> &ListViewerState;
    fn list_state_mut(&mut self) -> &mut ListViewerState;

    // Default implementations using list_state()
    fn focused_item(&self) -> Option<usize> {
        self.list_state().focused
    }

    fn set_focused_item(&mut self, item: Option<usize>) {
        self.list_state_mut().focused = item;
    }

    // Abstract methods (must implement)
    fn get_text(&self, item: usize, max_len: usize) -> String;
    fn is_selected(&self, item: usize) -> bool;
}

// Implementation
pub struct ListBox {
    list_state: ListViewerState,
    items: Vec<String>,
}

impl ListViewer for ListBox {
    fn list_state(&self) -> &ListViewerState { &self.list_state }
    fn list_state_mut(&mut self) -> &mut ListViewerState { &mut self.list_state }

    fn get_text(&self, item: usize, _max_len: usize) -> String {
        self.items.get(item).cloned().unwrap_or_default()
    }

    fn is_selected(&self, item: usize) -> bool {
        Some(item) == self.list_state.focused
    }
}
```

**Pros**:
- Trait polymorphism + shared state
- Clean API
- Reusable logic
- Minimal boilerplate

**Cons**:
- Slightly more complex design

### Recommended Implementation

Use **Option 3 (Hybrid Approach)** because it provides the best balance:
- Borland-compatible API through traits
- Shared state/logic through helper structs
- Clean, idiomatic Rust code

## Implementation Steps

### Phase 1: TListViewer (8 hours)
1. Create `src/views/list_viewer.rs`
2. Define `ListViewerState` struct with shared fields
3. Define `ListViewer` trait with default implementations
4. Add navigation helpers (focus_item, scroll_to, etc.)
5. Add tests

### Phase 2: Refactor ListBox (4 hours)
1. Update `src/views/listbox.rs` to use `ListViewer` trait
2. Embed `ListViewerState`
3. Implement trait methods
4. Ensure all existing tests still pass
5. Update examples if needed

### Phase 3: TMenuView (6 hours)
1. Create `src/views/menu_viewer.rs`
2. Define `MenuViewerState` struct with shared fields
3. Define `MenuViewer` trait with default implementations
4. Add hot key finding, item navigation
5. Add tests

### Phase 4: TMenuBox (6 hours)
1. Create `src/views/menu_box.rs`
2. Implement MenuBox with MenuViewer trait
3. Add drawing logic (borders, arrows, selection)
4. Handle mouse/keyboard events
5. Add tests

### Phase 5: Refactor MenuBar (4 hours)
1. Update `src/views/menu_bar.rs` to use `MenuViewer` trait
2. Embed `MenuViewerState`
3. Implement trait methods
4. Ensure all existing tests still pass

### Phase 6: Examples & Documentation (4 hours)
1. Create comprehensive examples
2. Update MISSING_FEATURES.md
3. Write migration guide
4. Document architecture decisions

**Total Estimated Time: 32 hours** (vs. 38 hours in MISSING_FEATURES.md - slightly optimized)

## Architecture Decisions

### Decision 1: Traits vs Inheritance
**Decision**: Use traits for polymorphism, helper structs for shared state
**Rationale**: Best Rust idiom while maintaining Borland compatibility

### Decision 2: Field Names
**Decision**: Keep Borland field names where possible (focused, topItem → top_item, range)
**Rationale**: Easier porting, clear mapping to original code

### Decision 3: ScrollBar Integration
**Decision**: Keep optional ScrollBar references (Option<Rc<RefCell<ScrollBar>>>)
**Rationale**: Matches Borland (can be null), safe shared ownership in Rust

### Decision 4: Backwards Compatibility
**Decision**: Keep existing ListBox/MenuBar APIs working
**Rationale**: Don't break existing code, provide migration path

## File Structure

```
src/views/
  ├── list_viewer.rs      (NEW - ListViewerState + ListViewer trait)
  ├── listbox.rs          (REFACTOR - use ListViewer)
  ├── menu_viewer.rs      (NEW - MenuViewerState + MenuViewer trait)
  ├── menu_box.rs         (NEW - MenuBox implementation)
  ├── menu_bar.rs         (REFACTOR - use MenuViewer)
  └── mod.rs              (UPDATE - add new modules)

examples/
  ├── list_viewer_demo.rs (NEW - demonstrate ListViewer)
  ├── menu_box_demo.rs    (NEW - demonstrate MenuBox)
  └── menu_status_data.rs (EXISTING)
```

## Success Criteria

- [ ] ListViewer trait defined with all key methods
- [ ] ListBox successfully refactored to use ListViewer
- [ ] All existing ListBox tests pass
- [ ] MenuViewer trait defined with all key methods
- [ ] MenuBox fully implemented
- [ ] MenuBar successfully refactored to use MenuViewer
- [ ] All existing MenuBar tests pass
- [ ] New comprehensive examples created
- [ ] MISSING_FEATURES.md updated
- [ ] Documentation complete

## Next Steps

1. Start with Phase 1 (TListViewer) as it's the foundation
2. Test thoroughly before moving to refactoring
3. Keep git commits small and focused
4. Run full test suite after each phase

---

*Document created: 2025-11-03*
*Estimated completion: 32 hours*
