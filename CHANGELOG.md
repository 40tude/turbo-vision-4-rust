# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2025-11-02

### Added
- **Z-Order Management**: Non-modal windows can now be brought to the front by clicking on them, matching Borland Turbo Vision's `TGroup::selectView()` behavior.
- **Modal Window Support**: Modal dialogs (like `Dialog::execute()`) now properly block interaction with background windows. When a modal dialog is present, clicking background windows has no effect.
- **Menu Borders and Shadows**: Dropdown menus now display with single-line borders and shadows, matching Borland's TMenuBox styling:
  - Single-line box drawing characters (`┌─┐`, `│`, `└─┘`, `├─┤`)
  - 2x1 shadow (2 cells wide on right, 1 cell tall on bottom)
  - Verified against original Borland Turbo Vision source code
- **Window Overlap Test**: New `window_modal_overlap_test` example demonstrating z-order management with three overlapping non-modal windows.

### Fixed
- **Mouse Event Z-Order**: Fixed mouse event handling to search in reverse z-order (top-most view first), preventing background views from capturing events intended for foreground windows.
- **Upward Dragging**: Fixed issue where windows could not be dragged upward. Windows can now be dragged in all directions by sending mouse events to dragging windows even when the mouse moves outside their bounds.

### Changed
- **Group::bring_to_front()**: Added method to reorder children in z-order, automatically updating focused index.
- **Desktop Event Handling**: Desktop now manages z-order changes on mouse clicks and enforces modal blocking when modal windows are present.
- **Dialog Modal Flag**: `Dialog::execute()` now automatically sets and clears the `SF_MODAL` flag, making all executed dialogs modal by default.

### Technical Details
This release implements Borland Turbo Vision's window management architecture:
- **Z-Order**: Children vector index represents z-order (higher index = on top)
- **Modal Scope**: Top-most window with `SF_MODAL` flag captures all events
- **Border Drawing**: Uses Borland's `frameChars` pattern for consistent styling
- **Shadow Rendering**: Matches Borland's `shadowSize = {2, 1}` and rendering algorithm

## [0.1.1] - 2025-11-02

### Fixed
- **Window dragging trails**: Fixed visual corruption when dragging windows. Modal dialogs now properly redraw the desktop background on each frame, matching Borland Turbo Vision's `TProgram::getEvent()` pattern where the entire screen is redrawn before polling for events.

### Changed
- **Desktop architecture**: Desktop now uses a `Background` view as its first child (matching Borland's `TDeskTop` with `TBackground`), ensuring proper z-order rendering.
- **FileDialog execution**: `FileDialog::execute()` now takes an `Application` reference and redraws the desktop before drawing the dialog, following Borland's modal dialog pattern.

### Technical Details
The fix addresses a fundamental architectural issue where modal dialogs had their own event loops that only redrawed themselves, not the desktop background. This caused visible trails when windows moved. The solution follows Borland Turbo Vision's pattern where `getEvent()` triggers a full screen redraw before returning events to modal views.

## [0.1.0] - 2025-11-02

### Added

#### Core System
- Event-driven architecture with keyboard and command-based event routing
- Complete drawing system with color support (16-color palette with attribute system)
- Geometry primitives with absolute and relative positioning
- Focus management with Tab navigation and keyboard shortcuts
- Modal dialog execution system
- Cross-platform terminal I/O abstraction layer built on crossterm

#### UI Components
- **Dialog**: Dialog boxes with frames and close buttons
- **Button**: Interactive buttons with keyboard shortcuts and mouse support
- **StaticText**: Text labels with centered text support
- **InputLine**: Single-line text input fields
- **Menu**: Menu bar with dropdown menus and mouse support
- **StatusLine**: Status bar with clickable items
- **Desktop**: Desktop manager for window management
- **ScrollBar**: Vertical and horizontal scrollbars with mouse support
- **Scroller**: Base class for scrollable views
- **Indicator**: Position/status display widget
- **TextView**: Scrollable text viewer with line numbers
- **CheckBox**: Checkbox controls with mouse support
- **RadioButton**: Radio button groups with mouse support
- **ListBox**: List selection widget with mouse and keyboard navigation
- **Memo**: Multi-line text editor with basic editing capabilities
- **FileDialog**: Full-featured file selection dialog with directory navigation

#### Input & Navigation
- Full keyboard support with arrow keys, Tab, Enter, Escape
- Mouse support including:
  - Button clicks and hover effects
  - Menu interaction
  - Status bar clicks
  - Dialog close buttons
  - ListBox item selection
  - Scrollbar interaction
- Keyboard shortcuts for quick access

#### Application Framework
- Application class with event loop
- Terminal initialization and cleanup
- Resource management

### Documentation
- Comprehensive README with quick start guide
- Module overview documentation
- Example programs demonstrating framework usage

### Known Limitations
- Full text editor with search/replace not yet implemented (basic editing available in Memo)

[0.1.2]: https://github.com/aovestdipaperino/turbo-vision-4-rust/releases/tag/v0.1.2
[0.1.1]: https://github.com/aovestdipaperino/turbo-vision-4-rust/releases/tag/v0.1.1
[0.1.0]: https://github.com/aovestdipaperino/turbo-vision-4-rust/releases/tag/v0.1.0
