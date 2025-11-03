# Missing Features Inventory

*Generated from Borland Turbo Vision source analysis*
*Last updated: 2025-11-03 (post-v0.2.1)*

This document catalogs missing features compared to the original Borland Turbo Vision framework, providing a roadmap for future development.

## Summary Statistics

- **Total Missing Components**: 92
- **Estimated Total Effort**: 1,088 hours (~27 weeks at 40 hrs/week)
- **HIGH Priority**: 29 items (340 hours) - Core functionality
- **MEDIUM Priority**: 45 items (486 hours) - Extended features
- **LOW Priority**: 18 items (262 hours) - Nice to have

## Quick Reference by Category

| Category | Count | Priority | Effort |
|----------|-------|----------|--------|
| Core Views/Controls | 22 | HIGH-MEDIUM | 220h |
| Specialized Dialogs | 13 | LOW-MEDIUM | 126h |
| Editor Components | 3 | HIGH-MEDIUM | 52h |
| System Utilities | 24 | MEDIUM | 168h |
| Helper Classes | 20 | HIGH-MEDIUM | 160h |
| Advanced Features | 10 | HIGH-LOW | 162h |

## High Priority Components (Core Functionality)

### Collections & Data Structures (38 hours)
- **TCollection** - Dynamic array collection (12h)
- **TSortedCollection** - Sorted collection with binary search (10h)
- **TNSCollection** - Non-streamable collection (8h)
- **TNSSortedCollection** - Non-streamable sorted (8h)

### Menu & Status Infrastructure (20 hours)
- **TSItem** - Linked list node for items (2h)
- **TMenu** - Menu data structure (6h)
- **TMenuItem** - Single menu item (4h)
- **TSubMenu** - Submenu builder (4h)
- **TStatusDef** - Status line definition (4h)
- **TStatusItem** - Status line item (3h) (total 7h)

### List Components (38 hours)
- **TListViewer** - Base for list views (16h)
- **TMenuView** - Base for menu views (12h)
- **TMenuBox** - Popup menu container (10h)

### Input Controls (26 hours)
- **TCluster** - Base for radio/checkbox (8h)
- **THistory** - History dropdown (12h)
- **THistoryViewer** - History list viewer (6h)

### File System (26 hours)
- **TFileList** - File browser list (12h)
- **TDirListBox** - Directory tree (14h)

### Editor (32 hours)
- **TFileEditor** - File editor with load/save (24h)
- **TEditWindow** - Editor window wrapper (8h)

### Application Framework (58 hours)
- **TProgram** - Base application (20h)
- **TApplication** - Extended application (16h)
- **TScreen** - Screen manager (20h)
- **TDisplay** - Display abstraction (16h)
- **TMouse** - Mouse system (12h)
- **TEventQueue** - Event queue (10h)

**Total HIGH Priority: 340 hours**

## Medium Priority Components (Extended Features)

### File Dialog Components (28 hours)
- **TFileInputLine** - File path input (6h)
- **TFileInfoPane** - File info display (6h)
- **TChDirDialog** - Change directory dialog (10h)
- **TFileCollection** - File entry collection (8h)
- **TDirCollection** - Directory collection (8h)

### Resource System (28 hours)
- **TResourceFile** - Resource file manager (16h)
- **TResourceCollection** - Resource container (8h)
- **TResourceItem** - Resource entry (4h)

### Help System (56 hours)
- **THelpFile** - Help file manager (20h)
- **THelpBase** - Help infrastructure (12h)
- **THelpWindow** - Help display window (12h)
- **THelpViewer** - Help content viewer (12h)

### Streaming System (78 hours)
Complete persistence infrastructure including:
- Stream base classes (pstream, ipstream, opstream - 26h)
- File streams (fpstream, ifpstream, ofpstream, iopstream - 28h)
- Stream helpers (TWriteObjects, TReadObjects - 12h)
- Streamable base (TStreamable - 12h)

### String Utilities (20 hours)
- **TStringCollection** - String collection (8h)
- **TStringList** - Indexed string list (10h)
- **TStrListMaker** - String list builder (6h)
- **TStrIndexRec** - String index record (4h)

### List Enhancements (8 hours)
- **TSortedListBox** - Sorted list with search (8h)

### Application Enhancements (20 hours)
- **TDeskTop** - Enhanced desktop features (10h)
- **TEditorApp** - Editor application framework (20h)
- **TDrawBuffer** - Drawing utilities (8h)
- **CodePage** - Character encoding (12h)
- **OSClipboard** - System clipboard (10h)

**Total MEDIUM Priority: 486 hours**

## Low Priority Components (Nice to Have)

### Color Customization Suite (66 hours)
Complete color editor system:
- TColorDialog, TColorSelector, TMonoSelector (40h)
- TColorDisplay, TColorGroup, TColorItem (14h)
- TColorGroupList, TColorItemList (12h)

### Calculator (24 hours)
- TCalculator dialog (16h)
- TCalcDisplay component (8h)

### Advanced Validators (20 hours)
- **TPXPictureValidator** - Mask validation (12h)
- **TLookupValidator** - List validation (8h)

### Text Output (40 hours)
- **TTextDevice** - Text output base (12h)
- **TTerminal** - Terminal emulator (20h)
- **otstream** - Output text stream (8h)

### Configuration (10 hours)
- **ConfigFile** - Configuration manager (10h)

**Total LOW Priority: 262 hours**

## Recommended Implementation Roadmap

### Phase 1: Core Collections (80 hours)
Foundation for all other components:
- TCollection, TSortedCollection, TNSCollection, TNSSortedCollection
- TSItem, TMenu, TMenuItem, TSubMenu
- TStatusDef, TStatusItem

### Phase 2: List Infrastructure (40 hours)
Proper hierarchy for list controls:
- TListViewer (refactor TListBox)
- TSortedListBox

### Phase 3: Menu System (32 hours)
Match Borland architecture:
- TMenuView, TMenuBox separation
- Complete menu infrastructure

### Phase 4: History System (32 hours)
Essential for professional UIs:
- THistory, THistoryViewer, THistoryWindow

### Phase 5: Cluster Controls (16 hours)
Architectural improvement:
- TCluster base class
- Refactor RadioButton/CheckBox

### Phase 6: File Dialogs (52 hours)
Complete file system UI:
- TFileList, TDirListBox
- TFileInputLine, TFileInfoPane, TChDirDialog

### Phase 7: Editor (52 hours)
Full-featured text editing:
- TFileEditor with search/replace
- TEditWindow, load/save

### Phase 8: Application Framework (56 hours)
Enhanced core infrastructure:
- TProgram, TApplication
- TScreen, TDisplay, TMouse, TEventQueue

### Phase 9: Resources & Persistence (90 hours)
Professional app development:
- Complete streaming system
- Resource file support

### Phase 10: Help System (56 hours)
Context-sensitive help:
- THelpFile, THelpBase
- THelpWindow, THelpViewer

### Phase 11: Polish (262+ hours)
Optional enhancements:
- Color customization
- Calculator, validators
- Configuration system

## Milestone Markers

- **After Phase 5** (252 hours): Most commonly used components complete
- **After Phase 7** (356 hours): Professional editing applications possible
- **After Phase 10** (506 hours): Feature parity with Borland's core framework
- **After Phase 11** (768+ hours): Complete framework with all utilities

## Quick Win Opportunities

These items provide high architectural value for relatively low effort:

1. **TSItem** (2 hours) - Simple linked list node
2. **TCluster** (8 hours) - Refactor existing RadioButton/CheckBox
3. **TStatusDef/TStatusItem** (7 hours) - Enhance existing status line
4. **TMenu/TMenuItem/TSubMenu** (14 hours) - Formalize menu data

**Total: 31 hours for significant architectural improvements**

## Current Implementation Status (v0.2.1)

### What We Have
- Basic controls: Button, InputLine, StaticText, Label, CheckBox, RadioButton
- Lists: ListBox (needs TListViewer refactoring)
- Menus: MenuBar with dropdowns (needs TMenuView/TMenuBox refactoring)
- Dialogs: Dialog, FileDialog (basic), MsgBox
- Text: Memo, TextView, Editor (basic)
- System: Desktop, StatusLine, Frame, Window, Group
- Utilities: ScrollBar, Scroller, Indicator, ParamText, Background
- Validation: Validator trait, FilterValidator, RangeValidator
- Event system: Three-phase processing, event re-queuing, broadcasts

### Architectural Gaps
- Missing collection infrastructure (using Vec instead of TCollection)
- Menu system doesn't match TMenuView/TMenuBox hierarchy
- ListBox needs refactoring to extend TListViewer
- No history system for input fields
- No resource/streaming system
- No help system infrastructure

## Next Steps

For v0.2.x series, focus on:
1. TEditor enhancement with search/replace
2. Window Min/Max buttons

For v0.3.0+, consider starting Phase 1 (Collections) to establish proper architectural foundation before building more complex features.

---

*This inventory was generated by analyzing 105 .cc files and 130+ headers from the original Borland Turbo Vision source code.*
