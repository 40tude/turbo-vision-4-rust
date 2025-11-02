# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.1.0]: https://github.com/aovestdipaperino/turbo-vision-4-rust/releases/tag/v0.1.0
