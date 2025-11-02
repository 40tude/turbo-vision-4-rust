# Turbo Vision for Rust - TODO List

This document tracks missing controls, classes, and features from the original Borland Turbo Vision that are not yet implemented in the Rust version.

## Status Legend
- ✅ **Implemented** - Feature is complete and working
- 🚧 **Partial** - Basic implementation exists but incomplete
- ❌ **Missing** - Not yet implemented

---

## Core Views & Controls

### Basic Controls
- ✅ **TView** - Base view class (implemented as `View` trait)
- ✅ **TGroup** - Container for child views (implemented as `Group`)
- ✅ **TWindow** - Window with frame (implemented as `Window`)
- ✅ **TDialog** - Modal dialog (implemented as `Dialog`)
- ✅ **TFrame** - Window frame with title and close button
- ✅ **TButton** - Push button with mouse/keyboard support
- ✅ **TCheckBoxes** - Checkbox control (implemented as `Checkbox`)
- ✅ **TRadioButtons** - Radio button control (implemented as `RadioButton`)
- ✅ **TInputLine** - Single-line text input (implemented as `InputLine`)
- ✅ **TStaticText** - Static text label with centering support
- ✅ **TLabel** - Text label (implemented as `Label`)
- ✅ **TListBox** - List selection control (implemented as `ListBox`)
- ❌ **TCluster** - Base class for check/radio groups

### Editor Components
- 🚧 **TEditor** - Text editor (basic implementation exists)
- 🚧 **TMemo** - Multi-line text input (basic implementation exists)
- ❌ **TFileEditor** - File editor with load/save
- ❌ **TEditWindow** - Window for editor

### Advanced Controls
- ✅ **TScrollBar** - Scrollbar (vertical/horizontal)
- ✅ **TScroller** - Scrollable view base class
- ✅ **TTextDevice** - Base for text output (implemented as `TextViewer`)
- ❌ **TListViewer** - Advanced list viewer
- ❌ **TOutline** - Tree/outline view
- ❌ **TParamText** - Parameterized text display
- ❌ **THistory** - History dropdown
- ❌ **THistoryViewer** - History list viewer
- ❌ **THistoryWindow** - History popup window

### Menu System
- ✅ **TMenuBar** - Top menu bar with mouse support
- ✅ **TStatusLine** - Bottom status line with mouse support
- ✅ **TMenuItem** - Menu item
- ✅ **TSubMenu** - Submenu container
- ❌ **TMenuBox** - Popup menu box
- ❌ **TMenuView** - Base menu view class
- ❌ **TMenuPopup** - Popup menu

### Application Framework
- ✅ **TApplication** - Main application class (implemented as `Application`)
- ✅ **TDesktop** - Desktop/workspace (implemented as `Desktop`)
- ❌ **TDeskTop** (original spelling) - Compatibility
- ✅ **TBackground** - Desktop background pattern
- ❌ **TProgram** - Alternative application base

---

## Standard Dialogs

### File Dialogs
- ✅ **TFileDialog** - File open/save dialog (fully functional with mouse/keyboard support)
- ✅ **TFileList** - File list viewer (implemented as ListBox integration)
- ❌ **TFileInfoPane** - File info display (size, date, attributes)
- ❌ **TFileInputLine** - File path input with completion
- ❌ **TDirListBox** - Directory list (directory navigation works via ListBox)
- ❌ **TDirEntry** - Directory entry
- ❌ **TDirCollection** - Directory collection (use Vec<PathBuf> instead)
- ❌ **TChDirDialog** - Change directory dialog

### Color & Display Dialogs
- ❌ **TColorDialog** - Color picker dialog
- ❌ **TColorSelector** - Color selection widget
- ❌ **TColorDisplay** - Color preview
- ❌ **TColorGroup** - Color group editor
- ❌ **TColorGroupList** - Color group list
- ❌ **TColorItem** - Color item
- ❌ **TColorItemList** - Color item list
- ❌ **TMonoSelector** - Monochrome color selector

### Search & Replace
- ❌ **TFindDialog** - Find dialog
- ❌ **TReplaceDialog** - Find & replace dialog

### Utility Dialogs
- ✅ **TMessageBox** - Message box utility (implemented as `message_box()`)
- ✅ **TInputBox** - Simple input box (implemented as `input_box()`)

---

## Collections & Data Structures

**Note**: Original Turbo Vision collection classes are NOT being ported. Use Rust standard library instead:

### Rust Stdlib Replacements
- **TCollection** → Use `Vec<T>`
- **TSortedCollection** → Use `Vec<T>` with `.sort()` or `BTreeSet<T>`
- **TStringCollection** → Use `Vec<String>`
- **TStringList** → Use `Vec<String>` or `HashSet<String>`
- **TResourceCollection** → Use `HashMap<String, T>` or custom struct
- **File/Dir Collections** → Use `Vec<PathBuf>` or `Vec<DirEntry>`

Where components need collection-like behavior, they will accept standard Rust types:
- `Vec<T>` for ordered lists
- `HashMap<K, V>` for key-value pairs
- `HashSet<T>` for unique items
- `BTreeMap<K, V>` / `BTreeSet<T>` for sorted data

### Example: ListBox with Vec
```rust
// Original TV: TListBox with TCollection
// Rust version: ListBox with Vec<String>
let items = vec!["Item 1".to_string(), "Item 2".to_string()];
let listbox = ListBox::new(bounds, items);

// Or with custom types
struct FileItem { name: String, size: u64 }
let files: Vec<FileItem> = load_files();
let listbox = ListBox::new_with_display(bounds, files, |item| &item.name);
```

### Example: FileDialog with Vec<PathBuf>
```rust
// Original TV: TFileDialog with TFileCollection
// Rust version: FileDialog with Vec<PathBuf>
use std::path::PathBuf;
let files: Vec<PathBuf> = std::fs::read_dir(path)?
    .filter_map(|e| e.ok())
    .map(|e| e.path())
    .collect();
let dialog = FileDialog::new(bounds, files);
```

---

## Streaming & Persistence

### Stream Classes
- ❌ **TStream** - Base stream
- ❌ **TBufStream** - Buffered stream
- ❌ **TDosStream** - DOS file stream
- ❌ **TFPStream** - File pointer stream
- ❌ **TipStream** - Input stream
- ❌ **TopStream** - Output stream
- ❌ **TiopStream** - Input/output stream
- ❌ **TifpStream** - Input file pointer stream
- ❌ **TofpStream** - Output file pointer stream

### Resource Management
- ❌ **TResourceFile** - Resource file
- ❌ **TResourceCollection** - Resource collection
- ❌ **TResourceItem** - Resource item
- ❌ **TStringList** - String list for resources

---

## Help System

- ❌ **THelpFile** - Help file manager
- ❌ **THelpTopic** - Help topic
- ❌ **THelpIndex** - Help index
- ❌ **THelpViewer** - Help viewer window
- ❌ **THelpWindow** - Help display window
- ❌ **TCrossRef** - Cross-reference links
- ❌ **TCrossRefHandler** - Cross-reference handler

---

## Validation

- ❌ **TValidator** - Base validator
- ❌ **TRangeValidator** - Range validation
- ❌ **TStringLookupValidator** - String lookup validation
- ❌ **TPXPictureValidator** - Picture/mask validation
- ❌ **TFilterValidator** - Character filter validation

---

## Advanced Features

### Calculator
- ❌ **TCalculator** - Calculator dialog
- ❌ **TCalcDisplay** - Calculator display

### Configuration
- ❌ **TConfigFile** - Configuration file management
- ❌ **TVideoMode** - Video mode settings

### Clipboard
- ❌ **TClipboard** - Clipboard support (platform-specific)

### Event Queue
- ❌ **TEventQueue** - Event queue management

---

## Internationalization

- ❌ **TIntl** - Internationalization support
- ❌ **TCodePage** - Code page management
- ❌ **TLanguage** - Language support

---

## Graphics & Display

- ❌ **TScreen** - Screen management
- ❌ **TDisplay** - Display handling
- ❌ **TFont** - Font management
- ❌ **TFontCollection** - Font collection
- ❌ **TPalette** - Palette management (different from color palette)
- ❌ **TDrawBuffer** - Already have basic implementation, may need enhancement

---

## Missing Features in Existing Components

### Current Limitations

#### TEditor / TMemo
- ❌ Block operations (copy/cut/paste blocks)
- ❌ Search and replace
- ❌ Undo/redo (basic undo exists, needs enhancement)
- ❌ Line/column indicators
- ❌ Word wrap
- ❌ Syntax highlighting hooks
- ❌ File operations (load/save)

#### TMenuBar
- ✅ Mouse support (implemented)
- ✅ Keyboard navigation (implemented)
- ❌ Submenus (only one level supported)
- ❌ Menu separators in dropdowns
- ❌ Disabled menu items (partially supported)
- ❌ Checkmarks on menu items

#### TListBox
- ✅ Basic list display
- ✅ Keyboard navigation
- ❌ Mouse support for scrolling
- ❌ Multiple selection
- ❌ Column display
- ❌ Icons/symbols per item

#### TDialog
- ✅ Modal execution
- ✅ Mouse support on close button
- ❌ Non-modal dialogs
- ❌ Tab order customization
- ❌ Default button highlighting animation

#### TWindow
- ✅ Basic window with frame
- ✅ Close button
- ❌ Minimize/maximize
- ❌ Window resizing
- ❌ Window z-order management
- ❌ Window list menu

---

## Command Sets & Event System

- ❌ **TCommandSet** - Command set management
- ❌ Advanced event filtering
- ❌ Event broadcasting improvements
- ❌ Modal result values beyond CM_OK/CM_CANCEL

---

## Compatibility & Legacy

### DOS-Specific (Not Applicable to Rust)
- ❌ DOS interrupt handlers
- ❌ DOS-specific screen drivers
- ❌ EGA/VGA direct access
- ❌ DOS mouse drivers

### Platform Abstraction (Partially Complete)
- ✅ Terminal abstraction (crossterm)
- ✅ Basic mouse support
- ✅ Keyboard event handling
- ❌ Clipboard integration
- ❌ System clipboard access
- ❌ File system watchers

---

## Priority Assessment

### High Priority (Core Functionality)
1. ✅ **File Dialogs** - Fully functional with mouse/keyboard support
2. **Message Box** - Standard UI pattern
3. **Enhanced Editor** - Search/replace, block operations
4. **Validators** - Input validation framework
5. **History** - Command line history (use Vec<String> internally)

### Medium Priority (Enhanced UX)
1. **Color Dialogs** - Theme customization
2. **Help System** - Built-in help
3. **TListViewer** - Advanced list control
4. **Window Management** - Resize, minimize, z-order
5. **Streaming** - Save/load dialogs and data

### Low Priority (Nice to Have)
1. **Calculator** - Utility dialog
2. **TOutline** - Tree views
3. **Font Management** - Terminal-limited usefulness
4. **Legacy DOS compatibility** - Not applicable

---

## Architecture Notes

### Rust-Specific Considerations

Many original Turbo Vision features don't map directly to Rust:

1. **Streaming/Serialization** - Use serde instead
2. **Collections** - Use Rust Vec, HashMap, etc.
3. **Object hierarchy** - Use trait objects and composition
4. **Resource files** - Use Rust's resource embedding
5. **Platform abstraction** - Use crossterm and other crates

### Recommended Approach

Focus on:
- UI components and controls (high value)
- Standard dialogs (common patterns)
- Enhanced editor capabilities
- File operations
- Input validation

Defer or skip:
- Custom collection classes (use Rust std)
- Streaming system (use serde)
- DOS-specific features
- Complex compatibility layers

---

## Estimated Effort

Based on original Turbo Vision complexity (excluding collections, using Rust stdlib):

- **Implemented**: ~25% of relevant functionality
- **Remaining Core**: ~35% (7-10 weeks)
  - File Dialogs: 2-3 weeks
  - Message Box & Utilities: 1 week
  - Enhanced Editor: 2-3 weeks
  - Validators: 1-2 weeks
  - History: 1 week
- **Advanced Features**: ~25% (6-8 weeks)
  - Color Dialogs: 2 weeks
  - Help System: 2-3 weeks
  - Window Management: 2-3 weeks
- **Polish & Testing**: ~15% (3-4 weeks)

**Total remaining**: ~16-22 weeks for feature-complete implementation

**Note**: Estimate reduced by ~20% by using Rust standard library for collections and serde for serialization instead of porting original infrastructure.

---

## Notes

This assessment is based on the original Borland Turbo Vision 2.0 headers and source code found in `/Users/enzo/Code/turbo-vision/borland-turbo-vision/tvision/`.

Many features from the original may not be necessary in a modern Rust implementation, as Rust's standard library and ecosystem provide better alternatives for collections, serialization, and resource management.

The focus should be on UI components and user-facing features that provide value in a terminal UI context.

## Recent Updates

### FileDialog Implementation (2025-11-01 - Updated 2025-11-02)
A fully functional FileDialog has been implemented with the following features:
- Directory listing with wildcard filtering (*.ext patterns)
- **Mouse support**: Click to select files, double-click to open
- **Keyboard navigation**: Arrow keys, PgUp/PgDn, Home/End, Enter
- Directory navigation (click/Enter on `..` for parent, `[dirname]` for subdirectories)
- Visual file browser with ListBox
- Input field auto-populates when files are selected
- Open/Cancel buttons
- **Focus restoration after directory navigation** (Fixed 2025-11-02)

**Implementation Approach (Updated 2025-11-02):**
- FileDialog allows ListBox to handle its own navigation events
- After ListBox processes events, FileDialog reads the selection and updates InputLine
- Eliminates double-processing of events for accurate navigation
- Uses proper `set_focus_to_child()` to maintain focus after directory refresh

**Architectural Improvements:**
- Added `child_at_mut()` methods to Group, Window, and Dialog classes
- Added `set_focus_to_child()` method to Dialog/Window/Group hierarchy
- Added `get_list_selection()` to View trait for reading ListBox state
- Proper focus chain management matching Borland's `owner->setCurrent()` pattern

**Major Bug Fixes (2025-11-02):**
1. **Double Event Processing** - Events were processed twice causing navigation to skip items
2. **InputLine Not Updating** - Initial selection after directory change wasn't broadcast
3. **Focus "Limbo" State** - ListBox appeared focused but didn't respond to keyboard
   - Root cause: Manual `set_focus()` didn't update Group's internal `focused` index
   - Solution: Use `set_focus_to_child()` to update both visual and logical focus

**Known Limitations:**
- No file info pane (size, date, attributes)
- No directory tree view
- Basic wildcard matching only (*.ext patterns)

See `src/views/file_dialog.rs` for detailed documentation and Borland Turbo Vision reference comments.

### Focus Management & Keyboard Navigation (2025-11-01)
Enhanced focus management system to match original Turbo Vision behavior:

**Focus Architecture:**
- Only focused controls respond to keyboard input
- Mouse events route to control under cursor (and grant focus on click)
- Tab key cycles focus forward, Shift+Tab cycles backward
- Tab on last control wraps to first control (and vice versa)
- Group manages focus state and event routing

**Controls with Proper Focus Checks:**
- InputLine, ListBox, Button, CheckBox, RadioButton, Editor, Memo
- All check `self.focused` before handling keyboard events

**Documentation:**
- Created comprehensive `docs/FOCUS_ARCHITECTURE.md`
- Documents focus principles, implementation patterns, and common mistakes

### Label and Button Shortcut Rendering (2025-11-01)
Fixed rendering of keyboard shortcuts in labels and buttons:

**Implementation:**
- Added `move_str_with_shortcut()` to DrawBuffer
- Parses `~X~` format: text between tildes rendered in highlight color
- Example: `"~F~ile"` displays as "File" with "F" in red

**Updated Controls:**
- Label: Uses DIALOG_SHORTCUT color (Red on LightGray)
- Button: Shortcuts adapt to button state (focused/default/normal)
- StaticText: Supports shortcuts in multi-line text
- CheckBox & RadioButton: Simplified using new helper method

**Color Scheme:**
- Menu shortcuts: Red on LightGray
- Dialog shortcuts: Red on LightGray
- Status line shortcuts: Red on LightGray

### Cursor Display in Text Controls (2025-11-01)
Implemented visible cursor for text editing controls:

**Architecture:**
- Added `show_cursor(x, y)` and `hide_cursor()` to Terminal
- Added `update_cursor()` to View trait (default: hide cursor)
- Group hides cursor by default, then calls focused child's `update_cursor()`
- Application, Dialog, and Window call `update_cursor()` after drawing

**Controls with Cursor Display:**
- InputLine: Shows cursor at text insertion point
- Editor: Shows cursor at current editing position
- Memo: Shows cursor at current line/column

**Key Features:**
- Cursor only visible when control has focus
- Cursor hidden automatically when focus changes
- Proper cursor positioning with horizontal/vertical scrolling
- Works in modal dialogs with own event loops

### MessageBox and InputBox (2025-11-01)
Implemented standard utility dialogs matching Turbo Vision API:

**MessageBox Features:**
- Message types: Warning, Error, Information, Confirmation
- Button combinations: Yes/No/Cancel, OK/Cancel, single OK
- Functions: `message_box()` (auto-sized) and `message_box_rect()` (positioned)
- Returns CommandId of pressed button (CM_YES, CM_NO, CM_OK, CM_CANCEL)
- Auto-sizing based on message content (30-60 chars wide)
- Centered text display with multi-line support

**InputBox Features:**
- Prompts user for text input with label
- Functions: `input_box()` and `input_box_rect()`
- Returns `Option<String>` (Some on OK, None on Cancel)
- Label supports keyboard shortcuts (`~X~` format)
- Configurable maximum length
- Pre-filled initial value support

**Example Usage:**
```rust
// Message box
let result = message_box(terminal, "Save changes?", MF_CONFIRMATION | MF_YES_NO_CANCEL);

// Input box
if let Some(name) = input_box(terminal, "User Info", "~N~ame:", "John", 50) {
    println!("Hello, {}!", name);
}
```

**Implementation:**
- Uses existing Dialog, Button, StaticText, Label, and InputLine components
- Automatic button layout and centering
- First button or OK button is default
- Full keyboard shortcut support

See `src/views/msgbox.rs` and `examples/msgbox_test.rs` for details.
