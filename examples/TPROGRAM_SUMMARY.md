# Summary: TProgram Examples and Biorhythm Demo

This directory contains comprehensive examples and documentation about TProgram customization in Borland Turbo Vision and its Rust port.

## What Was Created

### 📚 Documentation

1. **`custom_program_cpp_example.md`** - Four C++ examples showing how to derive from TProgram:
   - Minimal Application (no history lists)
   - Display-Only Application (read-only interface)
   - Embedded System Application (adaptive subsystems)
   - Testing Framework Application (headless mode)

2. **`custom_application_rust_example.md`** - Rust equivalents demonstrating:
   - Composition over inheritance
   - Builder pattern for configuration
   - Feature flags for compile-time optimization
   - Zero-cost abstractions

3. **`BIORHYTHM_EXAMPLE.md`** - Complete design document for biorhythm calculator
4. **`TPROGRAM_EXAMPLES.md`** - Index and quick start guide

### 💻 Code Examples

1. **`minimal_app.rs`** ✅ **WORKING**
   - Demonstrates minimal Turbo Vision application
   - No menu bar, only status line
   - Similar to deriving from TProgram instead of TApplication
   - Run with: `cargo run --example minimal_app`

2. **`biorhythm.rs`** 🚧 **WORK-IN-PROGRESS**
   - Semi-graphical biorhythm chart calculator
   - Demonstrates custom View implementation
   - Needs Terminal API updates (write_cell instead of write_str)
   - Fully documented in BIORHYTHM_EXAMPLE.md

## Key Findings About TProgram

### What is TProgram?

- **Abstract base class** between TGroup and TApplication in Borland Turbo Vision
- Provides basic application framework without full subsystems
- Allows selective initialization of 5 subsystems:
  1. Memory manager
  2. Video manager
  3. Event manager
  4. System error handler
  5. History list manager

### What Derives From TProgram?

**Answer: Only TApplication** in the standard Borland Turbo Vision library.

No other classes derive from TProgram in the official codebase. TProgram exists as an extension point for theoretical customizations, but in practice:
- Almost all applications need the full subsystems
- Memory savings are negligible (~4KB for history lists)
- Complexity outweighs benefits
- TProgram was rarely used directly

### Why Rust Doesn't Need TProgram

The Rust port correctly merges TProgram and TApplication into a single `Application` struct because:

1. **No inheritance** - Rust uses composition, not class hierarchies
2. **Feature flags** - Better mechanism for optional functionality
3. **Builder pattern** - Runtime configuration without subclassing
4. **Zero-cost abstractions** - Unused code eliminated at compile time
5. **Type safety** - Compile-time verification vs. runtime overhead

## Design Philosophy

### C++ (Borland): Inheritance-Based Flexibility

```cpp
TObject → TView → TGroup → TProgram → TApplication
                              ↑
                         (rarely used)
```

### Rust: Composition-Based Flexibility

```rust
Application {
    terminal: Terminal,          // Required
    menu_bar: Option<MenuBar>,   // Optional
    status_line: Option<StatusLine>, // Optional
    desktop: Desktop,            // Required
}
```

## Practical Comparison

| Aspect | TProgram (C++) | Application (Rust) |
|--------|----------------|-------------------|
| Subsystem selection | Override constructor | Feature flags |
| Memory overhead | Runtime | Compile-time |
| Flexibility | Inheritance | Composition |
| Safety | Manual | Type-checked |
| Practical use | Rare | Default |

## Running the Examples

### Minimal Application (Working)

```bash
cargo run --example minimal_app
```

Shows a simple Turbo Vision app with:
- No menu bar
- Status line only
- Single informational window
- Press Esc or Alt-X to exit

### Biorhythm Calculator (In Progress)

The biorhythm example is fully designed and documented but needs API updates:

**Current state:**
- Complete algorithm implementation ✅
- Dialog and menu structure ✅
- Custom View skeleton ✅
- Drawing code needs Terminal API update ⏳

**What needs updating:**
- Replace `terminal.write_str()` with `write_cell()` calls
- Create `Cell` objects for each character
- Use proper color attributes from the palette

**Design highlights:**
- Semi-graphical ASCII charts (P, E, I characters)
- Three sine wave cycles overlaid
- Vertical "today" marker
- Color-coded visualization
- Modal dialogs for input

## Educational Value

These examples teach:

1. **Historical Context** - Why TProgram existed and why it's not needed in Rust
2. **Design Patterns** - Inheritance vs. composition approaches
3. **API Design** - Trade-offs between flexibility and complexity
4. **Rust Idioms** - Feature flags, builders, type safety
5. **Custom Views** - How to implement the View trait
6. **TUI Programming** - Event handling, dialogs, menus

## Further Reading

- Original Borland docs: `local-only/books/original-guide-markdown/Chapter_9_Event-Driven_Programming.md`
- Rust design decisions: `docs/TURBOVISION-DESIGN.md`
- Application architecture: `docs/user-guide/Chapter-10-Application-Objects.md`

## Conclusion

The answer to "what derives from TProgram?" is simple: **only TApplication**. But the question reveals an interesting design choice in Borland Turbo Vision that made sense in C++ but is better solved through Rust's type system.

The Rust port demonstrates that modern language features can provide the same flexibility as inheritance hierarchies, with better safety guarantees and zero runtime overhead.
