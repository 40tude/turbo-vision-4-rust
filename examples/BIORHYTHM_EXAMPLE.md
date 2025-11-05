# Biorhythm Calculator Example

✅ **Working example** - builds and runs successfully!

This example demonstrates a complete Turbo Vision application that calculates and displays biorhythm charts with semi-graphical visualization.

## What are Biorhythms?

Biorhythms are pseudoscientific cycles that supposedly affect human behavior:
- **Physical cycle:** 23 days - affects strength, coordination, physical well-being
- **Emotional cycle:** 28 days - affects mood, creativity, emotional sensitivity
- **Intellectual cycle:** 33 days - affects memory, alertness, analytical ability

## Features Demonstrated

This example showcases several advanced Turbo Vision concepts:

### 1. Custom View Implementation
The `BiorhythmChart` struct implements the `View` trait to create a custom visualization component:
- Custom drawing logic for semi-graphical charts
- ASCII character-based graphing (P, E, I for the three cycles)
- Color-coded visualization using different palette colors
- Vertical axis with scale markers (+1.0, 0.0, -1.0)
- Today marker using a vertical line

### 2. Semi-Graphical Chart Display
The chart uses ASCII art to display sine waves:
```
+1.0                                        P
                        E              P   E
 0.0  -------P------E------I------P--E--------I-------|----
                  I                               I
-1.0
     P:Physical(23d)  E:Emotional(28d)  I:Intellectual(33d)  |:Today
```

### 3. Modal Dialog Boxes
- Input dialog for entering birth date and target date
- About dialog with application information
- Button-based selection dialogs

### 4. Custom Command Handling
- Defines custom commands (CM_BIORHYTHM, CM_ABOUT)
- Intercepts commands before normal event handling
- Updates application state based on dialog results

### 5. Menu and Status Line Configuration
- Menu bar with Biorhythm and Help menus
- Status line showing keyboard shortcuts
- Keyboard shortcuts: Alt-B (calculate), Alt-X (exit), F1 (help)

## Implementation Highlights

### Date Calculations
The example includes:
- Julian day number calculation for accurate date arithmetic
- Days-since-birth calculation
- Leap year handling
- Date parsing and formatting

### Biorhythm Formula
Each cycle is calculated using:
```rust
value = sin(2π × days_alive / cycle_length)
```

Where:
- `days_alive` = days since birth + day_offset
- `cycle_length` = 23 (physical), 28 (emotional), or 33 (intellectual)
- Result ranges from -1.0 to +1.0

### Chart Rendering
The chart view:
1. Clears the drawing area
2. Draws axis labels and center line
3. For each cycle (Physical=red, Emotional=green, Intellectual=blue):
   - Calculates value for each day in the display range
   - Maps the sine value (-1.0 to +1.0) to screen Y coordinate
   - Plots a colored ■ block at the calculated position
4. Draws a vertical marker for "today"
5. Displays legend at the bottom with colored blocks

## Usage

```bash
cargo run --example biorhythm
```

### Controls
- **Alt-B** or menu: Open biorhythm calculation dialog
- **F1** or Help menu: Show about dialog
- **Alt-X** or **Esc**: Exit application
- **Mouse**: Can drag and move the window

### Dialog Operation
1. Press Alt-B to open the calculation dialog
2. Choose an age preset (simplified demo)
3. Click OK to calculate and display biorhythm
4. The chart shows 30 days centered on today

## Code Structure

```
main()
  ├─ Application setup
  ├─ Menu bar creation (Biorhythm, Help)
  ├─ Status line creation
  ├─ Main window with BiorhythmChart
  └─ Event loop with custom command handling
      ├─ CM_BIORHYTHM → Show dialog, calculate, update chart
      ├─ CM_ABOUT → Show about dialog
      └─ Normal event handling

BiorhythmChart (Custom View)
  ├─ bounds, state, biorhythm data
  ├─ draw() → Semi-graphical chart rendering
  ├─ handle_event() → (display-only, no interaction)
  └─ update_cursor() → (no cursor needed)

Biorhythm (Calculator)
  ├─ birth_date, target_date
  ├─ physical(day_offset) → sine wave calculation
  ├─ emotional(day_offset) → sine wave calculation
  └─ intellectual(day_offset) → sine wave calculation
```

## Educational Value

This example teaches:
1. **Custom Views** - How to implement the View trait for custom components
2. **Terminal Drawing** - Using `terminal.goto()`, `write_char()`, `write_str()`
3. **Color Management** - Using the palette system for colored output
4. **Modal Dialogs** - Creating and executing modal dialogs with `exec_view()`
5. **Event Interception** - Handling custom commands before normal processing
6. **State Management** - Using global or window-level state for application data
7. **Mathematical Visualization** - Mapping mathematical functions to screen coordinates

## Historical Context

Biorhythm calculators were popular applications in the early PC era, often included as demonstration programs in:
- Borland Turbo Pascal examples
- BASIC programming books
- Early shareware collections

This modern Rust version demonstrates that Turbo Vision applications can be just as engaging today as they were in the DOS era, with better safety guarantees and modern language features.

## Future Enhancements

Possible improvements:
- Real date input parsing from InputLine fields
- Save/load biorhythm profiles
- Export chart as text or image
- Multiple person comparison
- Critical days highlight (when cycles cross zero)
- Zoom in/out functionality
- Configurable date ranges

## Building the Example

Due to API complexities with input field data binding, the current example uses a simplified approach with button-based age selection. A complete implementation with full date input parsing would require:

1. Shared data state for InputLine fields (Rc<RefCell<String>>)
2. Dialog result extraction mechanism
3. Date validation logic
4. Error handling for invalid dates

The simplified version still demonstrates all the core TUI concepts and provides a fully functional biorhythm calculator.
