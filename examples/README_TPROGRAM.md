# TProgram Examples - Quick Reference

## ✅ Working Examples

### Minimal Application
```bash
cargo run --example minimal_app
```

A stripped-down Turbo Vision application demonstrating what you'd get by deriving from `TProgram` instead of `TApplication` - no menu bar, just a status line.

### Biorhythm Calculator
```bash
cargo run --example biorhythm
```

A complete biorhythm calculator with:
- Semi-graphical ASCII charts with colored ■ blocks (red, green, blue)
- Modal dialogs for selecting age
- Custom View implementation for chart rendering
- Menu bar and status line
- Interactive buttons to switch between different ages

## 📚 Documentation

### Quick Answer
**Q: What derives from TProgram besides TApplication?**
**A: Nothing.** Only TApplication derives from TProgram in the standard Borland library.

### Detailed Documentation

1. **`TPROGRAM_SUMMARY.md`** - Executive summary with key findings
2. **`TPROGRAM_EXAMPLES.md`** - Full index and guide
3. **`custom_program_cpp_example.md`** - 4 C++ customization scenarios
4. **`custom_application_rust_example.md`** - Rust equivalents
5. **`BIORHYTHM_EXAMPLE.md`** - Biorhythm calculator design

## 🚧 Work in Progress

None - all examples are working!

## 🎯 Key Takeaways

1. **TProgram** exists as an extension point for selective subsystem loading
2. **Rarely used** in practice - TApplication provides what everyone needs
3. **Rust approach** merges both into single `Application` struct
4. **Better design** using composition, feature flags, and zero-cost abstractions

## 📖 Read More

Start with `TPROGRAM_SUMMARY.md` for a complete overview, then explore the specific documentation files for detailed examples.
