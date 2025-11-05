# TProgram Customization Examples

This directory contains examples demonstrating theoretical customizations that would be achieved by deriving from `TProgram` instead of `TApplication` in Borland Turbo Vision, and how the same goals are achieved in the Rust port.

## Files

### Documentation

- **`custom_program_cpp_example.md`** - C++ examples showing how to derive from TProgram with selective subsystem initialization
- **`custom_application_rust_example.md`** - Rust examples showing how to achieve the same customizations using composition and feature flags

### Working Examples

- **`minimal_app.rs`** ✅ - Minimal application (no menu bar, only status line)
- **`biorhythm.rs`** ✅ - Biorhythm calculator with semi-graphical ASCII charts

## Quick Start

Run the minimal application example:

```bash
cargo run --example minimal_app
```

Run the biorhythm calculator:

```bash
cargo run --example biorhythm
```

The minimal app demonstrates an application similar to what you'd get by deriving from `TProgram` instead of `TApplication` in Borland Turbo Vision - a bare-bones application without the full subsystem overhead.

## Why TProgram Exists

In Borland Turbo Vision, the class hierarchy is:

```
TObject → TView → TGroup → TProgram → TApplication
```

**TProgram** provides the basic application framework (event loop, desktop, menu bar, status line) while **TApplication** adds five subsystems:

1. **Memory manager** - Safety pool and buffer management
2. **Video manager** - Screen mode tracking
3. **Event manager** - Event queue handling
4. **System error handler** - Error management
5. **History list manager** - Input field history

By deriving from `TProgram` directly, developers could selectively enable only the subsystems they needed.

## Customization Scenarios

The documentation files cover four main scenarios:

1. **Minimal Application** - Skip history lists for memory-constrained apps
2. **Display-Only Application** - Read-only UI with minimal event handling
3. **Embedded System Application** - Adaptive subsystem loading based on available memory
4. **Testing Framework Application** - Headless testing with conditional UI

## Rust Approach: Composition Over Inheritance

The Rust port doesn't separate `TProgram` and `TApplication` because:

1. **No inheritance** - Rust uses composition, not class hierarchies
2. **Feature flags** - Compile-time subsystem selection via Cargo features
3. **Builder pattern** - Runtime configuration through builders
4. **Zero-cost abstraction** - Unused code eliminated at compile time

### Example: Feature-based Customization

```toml
# Cargo.toml
[features]
default = ["menu-bar", "status-line", "history"]
minimal = ["status-line"]
embedded = ["status-line"]
```

```bash
# Build minimal version (no menu bar, no history)
cargo build --no-default-features --features minimal

# Build full version
cargo build
```

## Practical Reality

In practice, **TProgram was rarely used directly**. Almost all applications needed the full subsystems that `TApplication` provides. The separation existed for theoretical flexibility but provided little practical value.

This is why the Rust port correctly merges both into a single `Application` struct - it's a better design that achieves the same goals more elegantly.

## Further Reading

- Original Borland documentation: `local-only/books/original-guide-markdown/Chapter_9_Event-Driven_Programming.md`
- Rust design decisions: `docs/TURBOVISION-DESIGN.md`
- Application architecture: `docs/user-guide/Chapter-10-Application-Objects.md`
