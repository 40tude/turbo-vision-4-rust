# Biorhythm Example - Usage Guide

## Running the Application

```bash
cargo run --example biorhythm
```

## Features

The biorhythm calculator displays three pseudoscientific cycles:
- **Physical (■ red)** - 23-day cycle
- **Emotional (■ green)** - 28-day cycle
- **Intellectual (■ blue)** - 33-day cycle

## Controls

### Menu Navigation
- **F10** - Open menu bar
- **Arrow keys** - Navigate menu items
- **Enter** - Select menu item
- **Esc** - Close menu
- **Alt+C** - Calculate biorhythm (opens dialog)
- **F1** - About dialog
- **Alt+X** - Exit application

### Window Controls
- **Click close box** (×) in top-right corner to close window
- **Drag title bar** to move window
- **Esc** - Also exits the application

### Dialog Navigation
- **Tab** / **Shift+Tab** - Move between buttons
- **Arrow keys** - Navigate buttons
- **Enter** or **Space** - Activate focused button
- **Mouse Click** - Click any button
- **Esc** - Cancel dialog

## How to Use

1. Launch the application - it starts with a default chart (10,000 days old, ~27 years)
2. Press **F10** to open the menu, select **Biorhythm → Calculate**
3. Choose an age from the dialog:
   - 5,000 days (~14 years)
   - 10,000 days (~27 years)
   - 15,000 days (~41 years)
4. The chart updates to show biorhythms for that age

## Reading the Chart

```
+1.0  ← Peak (good day)
                    ■    ■
 0.0  -------■---■-----■---------|----  ← Neutral
                          ■
-1.0  ← Low (bad day)

■:Physical(23d) ■:Emotional(28d) ■:Intellectual(33d) |:Today
  red            green            blue
```

- **■ (red)** = Physical cycle position
- **■ (green)** = Emotional cycle position
- **■ (blue)** = Intellectual cycle position
- **|** = Today (center of chart)
- **Horizontal line** = Zero/neutral point
- **Above line** = Positive phase (high energy, good mood, sharp mind)
- **Below line** = Negative phase (low energy, irritable, unfocused)

## Chart Spans

The chart shows 30 days centered on "today":
- 15 days in the past (left of |)
- Today (marked with |)
- 15 days in the future (right of |)

## Technical Notes

### Custom View Implementation
The chart is implemented as a custom `View` that:
- Calculates sine waves for each cycle
- Maps mathematical values to screen coordinates
- Draws using `DrawBuffer` and `write_line_to_terminal`
- Uses colored ■ blocks (red, green, blue) for visual distinction

### Biorhythm Formula
```rust
value = sin(2π × days_alive / cycle_length)
```

Where:
- `days_alive` = days since birth + day_offset
- `cycle_length` = 23 (physical), 28 (emotional), or 33 (intellectual)
- Result ranges from -1.0 (low) to +1.0 (high)

### Modal Dialog Pattern
The application demonstrates proper modal dialog usage:
1. Create dialog with buttons
2. Call `set_initial_focus()` to enable keyboard navigation
3. Call `execute(&mut app)` to run modal loop
4. Check return value to see which button was pressed

## Tips

- The chart updates live when you select a different age
- Try different ages to see how the cycles shift
- Look for days when all three cycles are high (good days!)
- Watch for "critical days" when cycles cross zero

## Known Limitations

This is a simplified demo:
- Uses preset age options instead of date input
- "Today" is relative to the chosen age, not actual dates
- Chart is fixed at 30-day span
- No save/load functionality

For a full implementation with date parsing, see the documentation in `BIORHYTHM_EXAMPLE.md`.
