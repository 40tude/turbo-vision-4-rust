# Chapter 16 — Collections and Streams (Rust Edition)

**Version:** 0.2.11
**Updated:** 2025-11-04

This chapter explores collections of heterogeneous objects and streaming (serialization) in Turbo Vision. You'll learn how to work with trait objects, serialize view hierarchies, save and load application state, and create resource files.

**Prerequisites:** Chapters 8-11 (Views, Application, Windows, Dialogs)

---

## Table of Contents

1. [Understanding Collections](#understanding-collections)
2. [Heterogeneous Collections](#heterogeneous-collections)
3. [Serialization with Serde](#serialization-with-serde)
4. [Streaming Views](#streaming-views)
5. [Application State](#application-state)
6. [Resource Files](#resource-files)
7. [Custom Serialization](#custom-serialization)
8. [Binary Streams](#binary-streams)
9. [Complete Examples](#complete-examples)

---

## Understanding Collections

### Collections in Pascal vs Rust

**Pascal Approach:**
```pascal
// TCollection holds pointers to TObject
type
  PCollection = ^TCollection;
  TCollection = object(TObject)
    Items: PItemList;      // Array of pointers
    Count: Integer;
    procedure Insert(Item: Pointer);
    procedure ForEach(Action: Pointer);  // Iterator
  end;
```

**Rust Approach:**
```rust
// Type-safe collections with generics
let items: Vec<Box<dyn View>> = vec![
    Box::new(Button::new(...)),
    Box::new(InputLine::new(...)),
    Box::new(Label::new(...)),
];

// Or homogeneous collections
let buttons: Vec<Button> = vec![
    Button::new(...),
    Button::new(...),
];
```

### Why Rust Collections Are Different

| Pascal | Rust |
|--------|------|
| Runtime type checking | Compile-time type safety |
| Manual memory management | Automatic with ownership |
| Virtual method dispatch | Trait dispatch |
| Registration required | Type system handles it |
| Stream IDs for types | Serde handles serialization |

---

## Heterogeneous Collections

### The Problem

You want a collection that holds different types of objects:

```rust
// How to store different view types together?
let window_contents = vec![
    Button::new(...),      // Different type
    InputLine::new(...),   // Different type
    Label::new(...),       // Different type
];
// Error: Vec can only hold one type!
```

### Solution: Trait Objects

Use `Box<dyn Trait>` for dynamic dispatch:

```rust
pub trait View {
    fn draw(&mut self, terminal: &mut Terminal);
    fn handle_event(&mut self, event: &mut Event);
    // ... other methods
}

// Collection of heterogeneous views
pub struct Group {
    bounds: Rect,
    children: Vec<Box<dyn View>>,
    state: StateFlags,
}

impl Group {
    pub fn add(&mut self, view: Box<dyn View>) {
        self.children.push(view);
    }

    pub fn draw(&mut self, terminal: &mut Terminal) {
        for child in &mut self.children {
            child.draw(terminal);  // Dynamic dispatch
        }
    }
}
```

### Example: Shape Collection

```rust
// Define trait for shapes
pub trait Shape {
    fn area(&self) -> f64;
    fn draw(&self);
}

// Different shape types
pub struct Circle {
    x: i16,
    y: i16,
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn draw(&self) {
        println!("Circle at ({}, {}) with radius {}", self.x, self.y, self.radius);
    }
}

pub struct Rectangle {
    x: i16,
    y: i16,
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn draw(&self) {
        println!("Rectangle at ({}, {}) with size {}x{}",
                 self.x, self.y, self.width, self.height);
    }
}

// Collection of shapes
pub struct ShapeCollection {
    shapes: Vec<Box<dyn Shape>>,
}

impl ShapeCollection {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
        }
    }

    pub fn add(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }

    pub fn draw_all(&self) {
        for shape in &self.shapes {
            shape.draw();
        }
    }

    pub fn total_area(&self) -> f64 {
        self.shapes.iter().map(|s| s.area()).sum()
    }
}

// Usage
let mut collection = ShapeCollection::new();
collection.add(Box::new(Circle { x: 10, y: 10, radius: 5.0 }));
collection.add(Box::new(Rectangle { x: 20, y: 20, width: 10.0, height: 5.0 }));
collection.draw_all();
```

---

## Serialization with Serde

### What is Serde?

**Serde** is Rust's standard serialization framework. It replaces Pascal's manual `Store`/`Load` methods with automatic serialization.

**Pascal Approach (Manual):**
```pascal
procedure TGraphObject.Store(var S: TStream);
begin
  S.Write(X, SizeOf(X));
  S.Write(Y, SizeOf(Y));
end;

procedure TGraphCircle.Store(var S: TStream);
begin
  inherited Store(S);
  S.Write(Radius, SizeOf(Radius));
end;
```

**Rust Approach (Automatic):**
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GraphObject {
    pub x: i16,
    pub y: i16,
}

#[derive(Serialize, Deserialize)]
pub struct GraphCircle {
    #[serde(flatten)]
    pub base: GraphObject,
    pub radius: f64,
}

// Serialization is automatic!
let circle = GraphCircle {
    base: GraphObject { x: 10, y: 20 },
    radius: 5.0,
};

// To JSON
let json = serde_json::to_string(&circle)?;

// To binary
let bytes = bincode::serialize(&circle)?;
```

### Setting Up Serde

Add to `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"      # For JSON format
bincode = "1.3"         # For binary format
```

### Basic Serialization

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rect {
    pub a: Point,
    pub b: Point,
}

// Serialize to JSON
let rect = Rect {
    a: Point { x: 0, y: 0 },
    b: Point { x: 80, y: 24 },
};

let json = serde_json::to_string_pretty(&rect)?;
println!("{}", json);
// Output:
// {
//   "a": { "x": 0, "y": 0 },
//   "b": { "x": 80, "y": 24 }
// }

// Deserialize from JSON
let rect2: Rect = serde_json::from_str(&json)?;
assert_eq!(rect2.a.x, rect.a.x);
```

### Binary Serialization

```rust
use bincode;

// Serialize to bytes
let bytes = bincode::serialize(&rect)?;

// Deserialize from bytes
let rect2: Rect = bincode::deserialize(&bytes)?;
```

### File I/O

```rust
use std::fs::File;
use std::io::{BufReader, BufWriter};

// Write to file
let file = File::create("data.json")?;
let writer = BufWriter::new(file);
serde_json::to_writer_pretty(writer, &rect)?;

// Read from file
let file = File::open("data.json")?;
let reader = BufReader::new(file);
let rect2: Rect = serde_json::from_reader(reader)?;

// Binary format
let file = File::create("data.bin")?;
bincode::serialize_into(file, &rect)?;

let file = File::open("data.bin")?;
let rect2: Rect = bincode::deserialize_from(file)?;
```

---

## Streaming Views

### The Challenge

Views contain trait objects and ownership relationships that can't be automatically serialized:

```rust
pub struct Window {
    bounds: Rect,
    title: String,
    children: Vec<Box<dyn View>>,  // Can't derive Serialize!
    state: StateFlags,
}
```

### Solution 1: State Objects

Separate view state from view logic:

```rust
// Serializable state
#[derive(Serialize, Deserialize)]
pub struct WindowState {
    pub bounds: Rect,
    pub title: String,
    pub child_states: Vec<ViewState>,
    pub state_flags: u16,
}

// Runtime view
pub struct Window {
    bounds: Rect,
    title: String,
    children: Vec<Box<dyn View>>,
    state: StateFlags,
}

impl Window {
    // Extract state
    pub fn save_state(&self) -> WindowState {
        WindowState {
            bounds: self.bounds,
            title: self.title.clone(),
            child_states: self.children.iter()
                .map(|c| c.save_state())
                .collect(),
            state_flags: self.state.bits(),
        }
    }

    // Restore state
    pub fn from_state(state: WindowState) -> Self {
        let mut window = Self {
            bounds: state.bounds,
            title: state.title,
            children: Vec::new(),
            state: StateFlags::from_bits_truncate(state.state_flags),
        };

        for child_state in state.child_states {
            window.children.push(view_from_state(child_state));
        }

        window
    }
}
```

### Solution 2: Tagged Enums

Use enums to represent different view types:

```rust
#[derive(Serialize, Deserialize)]
pub enum ViewState {
    Button {
        bounds: Rect,
        text: String,
        command: u16,
        is_default: bool,
    },
    InputLine {
        bounds: Rect,
        text: String,
        max_length: usize,
    },
    Label {
        bounds: Rect,
        text: String,
    },
    Window {
        bounds: Rect,
        title: String,
        children: Vec<ViewState>,
    },
}

impl ViewState {
    // Convert to runtime view
    pub fn into_view(self) -> Box<dyn View> {
        match self {
            ViewState::Button { bounds, text, command, is_default } => {
                Box::new(Button::new(bounds, &text, command, is_default))
            }
            ViewState::InputLine { bounds, text, max_length } => {
                let data = Rc::new(RefCell::new(text));
                Box::new(InputLine::new(bounds, max_length, data))
            }
            ViewState::Label { bounds, text } => {
                Box::new(Label::new(bounds, &text))
            }
            ViewState::Window { bounds, title, children } => {
                let mut window = Window::new(bounds, &title);
                for child in children {
                    window.add(child.into_view());
                }
                Box::new(window)
            }
        }
    }
}
```

### Solution 3: View Builders

Use builder pattern with serializable configs:

```rust
#[derive(Serialize, Deserialize)]
pub struct ButtonConfig {
    pub bounds: Rect,
    pub text: String,
    pub command: u16,
    pub is_default: bool,
}

impl ButtonConfig {
    pub fn build(self) -> Button {
        Button::new(self.bounds, &self.text, self.command, self.is_default)
    }
}

#[derive(Serialize, Deserialize)]
pub struct DialogConfig {
    pub bounds: Rect,
    pub title: String,
    pub controls: Vec<ControlConfig>,
}

#[derive(Serialize, Deserialize)]
pub enum ControlConfig {
    Button(ButtonConfig),
    InputLine {
        bounds: Rect,
        max_length: usize,
        initial_text: String,
    },
    Label {
        bounds: Rect,
        text: String,
    },
}

impl DialogConfig {
    pub fn build(self) -> Dialog {
        let mut dialog = Dialog::new(self.bounds, &self.title);

        for control in self.controls {
            match control {
                ControlConfig::Button(config) => {
                    dialog.add(Box::new(config.build()));
                }
                ControlConfig::InputLine { bounds, max_length, initial_text } => {
                    let data = Rc::new(RefCell::new(initial_text));
                    dialog.add(Box::new(InputLine::new(bounds, max_length, data)));
                }
                ControlConfig::Label { bounds, text } => {
                    dialog.add(Box::new(Label::new(bounds, &text)));
                }
            }
        }

        dialog
    }
}

// Usage
let config = DialogConfig {
    bounds: Rect::new(20, 8, 60, 16),
    title: "User Info".to_string(),
    controls: vec![
        ControlConfig::Label {
            bounds: Rect::new(2, 2, 15, 3),
            text: "Name:".to_string(),
        },
        ControlConfig::InputLine {
            bounds: Rect::new(15, 2, 36, 3),
            max_length: 50,
            initial_text: String::new(),
        },
        ControlConfig::Button(ButtonConfig {
            bounds: Rect::new(15, 5, 25, 7),
            text: "OK".to_string(),
            command: CM_OK,
            is_default: true,
        }),
    ],
};

// Save to file
let json = serde_json::to_string_pretty(&config)?;
std::fs::write("dialog.json", json)?;

// Load from file
let json = std::fs::read_to_string("dialog.json")?;
let config: DialogConfig = serde_json::from_str(&json)?;
let dialog = config.build();
```

---

## Application State

### Desktop State

Save the entire desktop (all windows):

```rust
#[derive(Serialize, Deserialize)]
pub struct DesktopState {
    pub windows: Vec<WindowState>,
    pub focused_window: Option<usize>,
}

impl Desktop {
    pub fn save_state(&self) -> DesktopState {
        DesktopState {
            windows: self.children.iter()
                .filter_map(|c| {
                    // Only save windows
                    if let Some(window) = c.as_any().downcast_ref::<Window>() {
                        Some(window.save_state())
                    } else {
                        None
                    }
                })
                .collect(),
            focused_window: self.focused_index(),
        }
    }

    pub fn restore_state(&mut self, state: DesktopState) {
        // Clear current windows
        self.children.clear();

        // Restore windows
        for window_state in state.windows {
            let window = Window::from_state(window_state);
            self.add(Box::new(window));
        }

        // Restore focus
        if let Some(index) = state.focused_window {
            self.focus_child(index);
        }
    }
}
```

### Application Preferences

```rust
#[derive(Serialize, Deserialize)]
pub struct AppPreferences {
    pub palette: Vec<u8>,
    pub recent_files: Vec<String>,
    pub window_positions: HashMap<String, Rect>,
    pub key_bindings: HashMap<String, u16>,
}

impl Application {
    pub fn save_preferences(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let prefs = AppPreferences {
            palette: self.get_palette().to_vec(),
            recent_files: self.recent_files.clone(),
            window_positions: self.window_positions.clone(),
            key_bindings: self.key_bindings.clone(),
        };

        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &prefs)?;

        Ok(())
    }

    pub fn load_preferences(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let prefs: AppPreferences = serde_json::from_reader(reader)?;

        self.set_custom_palette(prefs.palette);
        self.recent_files = prefs.recent_files;
        self.window_positions = prefs.window_positions;
        self.key_bindings = prefs.key_bindings;

        Ok(())
    }
}
```

### Saving on Exit

```rust
impl Application {
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Load preferences on startup
        let pref_path = self.preferences_path();
        if pref_path.exists() {
            if let Err(e) = self.load_preferences(&pref_path) {
                eprintln!("Failed to load preferences: {}", e);
            }
        }

        // Main event loop
        self.event_loop()?;

        // Save preferences on exit
        if let Err(e) = self.save_preferences(&pref_path) {
            eprintln!("Failed to save preferences: {}", e);
        }

        Ok(())
    }

    fn preferences_path(&self) -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("turbo-vision")
            .join("preferences.json")
    }
}
```

---

## Resource Files

### What are Resource Files?

**Resource files** are indexed collections of serialized objects. They allow random access to objects by ID without loading the entire file.

```rust
pub struct ResourceFile {
    file: File,
    index: HashMap<u32, ResourceEntry>,
}

pub struct ResourceEntry {
    id: u32,
    offset: u64,
    size: u64,
}
```

### Creating a Resource File

```rust
use std::io::{Seek, SeekFrom, Write, Read};

impl ResourceFile {
    pub fn create(path: &Path) -> Result<Self, std::io::Error> {
        let file = File::create(path)?;

        Ok(Self {
            file,
            index: HashMap::new(),
        })
    }

    pub fn add<T: Serialize>(&mut self, id: u32, data: &T) -> Result<(), Box<dyn std::error::Error>> {
        // Serialize data
        let bytes = bincode::serialize(data)?;

        // Get current position
        let offset = self.file.seek(SeekFrom::End(0))?;

        // Write data
        self.file.write_all(&bytes)?;

        // Update index
        self.index.insert(id, ResourceEntry {
            id,
            offset,
            size: bytes.len() as u64,
        });

        Ok(())
    }

    pub fn get<T: DeserializeOwned>(&mut self, id: u32) -> Result<T, Box<dyn std::error::Error>> {
        // Find in index
        let entry = self.index.get(&id)
            .ok_or("Resource not found")?;

        // Seek to position
        self.file.seek(SeekFrom::Start(entry.offset))?;

        // Read data
        let mut bytes = vec![0u8; entry.size as usize];
        self.file.read_exact(&mut bytes)?;

        // Deserialize
        let data = bincode::deserialize(&bytes)?;

        Ok(data)
    }

    pub fn save_index(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Write index at end of file
        let index_offset = self.file.seek(SeekFrom::End(0))?;

        let index_bytes = bincode::serialize(&self.index)?;
        self.file.write_all(&index_bytes)?;

        // Write index offset at very end
        self.file.write_all(&index_offset.to_le_bytes())?;

        Ok(())
    }

    pub fn open(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;

        // Read index offset from end
        file.seek(SeekFrom::End(-8))?;
        let mut offset_bytes = [0u8; 8];
        file.read_exact(&mut offset_bytes)?;
        let index_offset = u64::from_le_bytes(offset_bytes);

        // Read index
        file.seek(SeekFrom::Start(index_offset))?;
        let index_size = (file.metadata()?.len() - index_offset - 8) as usize;
        let mut index_bytes = vec![0u8; index_size];
        file.read_exact(&mut index_bytes)?;
        let index = bincode::deserialize(&index_bytes)?;

        Ok(Self { file, index })
    }
}
```

### Using Resource Files

```rust
// Create resource file
let mut resources = ResourceFile::create(Path::new("app.res"))?;

// Add resources
resources.add(100, &"Application Title")?;
resources.add(101, &dialog_config)?;
resources.add(102, &menu_config)?;
resources.save_index()?;

// Open resource file
let mut resources = ResourceFile::open(Path::new("app.res"))?;

// Get resources
let title: String = resources.get(100)?;
let dialog: DialogConfig = resources.get(101)?;
let menu: MenuConfig = resources.get(102)?;
```

---

## Custom Serialization

### Custom Serialize Implementation

```rust
use serde::ser::{Serialize, Serializer, SerializeStruct};

pub struct CustomView {
    pub id: u32,
    pub name: String,
    pub data: Vec<u8>,
}

impl Serialize for CustomView {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CustomView", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("data_len", &self.data.len())?;
        // Don't serialize data if empty
        if !self.data.is_empty() {
            state.serialize_field("data", &self.data)?;
        }
        state.end()
    }
}
```

### Custom Deserialize Implementation

```rust
use serde::de::{Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};

impl<'de> Deserialize<'de> for CustomView {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field { Id, Name, DataLen, Data }

        struct CustomViewVisitor;

        impl<'de> Visitor<'de> for CustomViewVisitor {
            type Value = CustomView;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct CustomView")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CustomView, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut id = None;
                let mut name = None;
                let mut data = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            id = Some(map.next_value()?);
                        }
                        Field::Name => {
                            name = Some(map.next_value()?);
                        }
                        Field::DataLen => {
                            let _ = map.next_value::<usize>()?;
                        }
                        Field::Data => {
                            data = Some(map.next_value()?);
                        }
                    }
                }

                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let data = data.unwrap_or_default();

                Ok(CustomView { id, name, data })
            }
        }

        const FIELDS: &[&str] = &["id", "name", "data_len", "data"];
        deserializer.deserialize_struct("CustomView", FIELDS, CustomViewVisitor)
    }
}
```

### Serde Attributes

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ViewConfig {
    // Rename field in serialized form
    #[serde(rename = "type")]
    pub view_type: String,

    // Skip serializing if None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    // Use default value if missing
    #[serde(default)]
    pub visible: bool,

    // Custom serialization function
    #[serde(serialize_with = "serialize_bounds")]
    pub bounds: Rect,

    // Flatten nested struct
    #[serde(flatten)]
    pub state: ViewState,

    // Skip field entirely
    #[serde(skip)]
    pub internal: InternalData,
}

fn serialize_bounds<S>(bounds: &Rect, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // Custom serialization logic
    serializer.serialize_str(&format!("{}:{}", bounds.a, bounds.b))
}
```

---

## Binary Streams

### Binary Format

For efficiency, use binary serialization:

```rust
use bincode;
use std::io::{BufReader, BufWriter};

pub struct BinaryStream {
    writer: Option<BufWriter<File>>,
    reader: Option<BufReader<File>>,
}

impl BinaryStream {
    pub fn create(path: &Path) -> Result<Self, std::io::Error> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        Ok(Self {
            writer: Some(writer),
            reader: None,
        })
    }

    pub fn open(path: &Path) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        Ok(Self {
            writer: None,
            reader: Some(reader),
        })
    }

    pub fn write<T: Serialize>(&mut self, data: &T) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut writer) = self.writer {
            bincode::serialize_into(writer, data)?;
            Ok(())
        } else {
            Err("Stream not opened for writing".into())
        }
    }

    pub fn read<T: DeserializeOwned>(&mut self) -> Result<T, Box<dyn std::error::Error>> {
        if let Some(ref mut reader) = self.reader {
            let data = bincode::deserialize_from(reader)?;
            Ok(data)
        } else {
            Err("Stream not opened for reading".into())
        }
    }
}

// Usage
let mut stream = BinaryStream::create(Path::new("data.bin"))?;
stream.write(&window_state)?;
stream.write(&dialog_state)?;

let mut stream = BinaryStream::open(Path::new("data.bin"))?;
let window_state: WindowState = stream.read()?;
let dialog_state: DialogState = stream.read()?;
```

### Versioning

Handle format changes with versioning:

```rust
#[derive(Serialize, Deserialize)]
pub struct VersionedData {
    pub version: u32,
    pub data: DataVersion,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum DataVersion {
    #[serde(rename = "1")]
    V1 {
        name: String,
        value: i32,
    },
    #[serde(rename = "2")]
    V2 {
        name: String,
        value: i64,
        description: String,
    },
}

impl DataVersion {
    pub fn upgrade(self) -> DataVersion {
        match self {
            DataVersion::V1 { name, value } => {
                DataVersion::V2 {
                    name,
                    value: value as i64,
                    description: String::new(),
                }
            }
            v2 @ DataVersion::V2 { .. } => v2,
        }
    }
}
```

---

## Complete Examples

### Example 1: Save/Load Desktop

```rust
use turbo_vision::prelude::*;
use std::path::Path;

pub struct PersistentApp {
    app: Application,
    state_file: PathBuf,
}

impl PersistentApp {
    pub fn new(state_file: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let mut app = Application::new()?;

        // Load state if exists
        if state_file.exists() {
            if let Err(e) = Self::load_desktop(&mut app, &state_file) {
                eprintln!("Failed to load desktop: {}", e);
            }
        }

        Ok(Self { app, state_file })
    }

    pub fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.app.run()?;

        // Save state on exit
        Self::save_desktop(&self.app, &self.state_file)?;

        Ok(())
    }

    fn save_desktop(app: &Application, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let state = app.desktop.save_state();

        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &state)?;

        Ok(())
    }

    fn load_desktop(app: &mut Application, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let state: DesktopState = serde_json::from_reader(reader)?;

        app.desktop.restore_state(state);

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state_file = PathBuf::from("desktop.json");
    let app = PersistentApp::new(state_file)?;
    app.run()?;

    Ok(())
}
```

### Example 2: Resource File Manager

```rust
pub struct AppResources {
    resources: ResourceFile,
}

impl AppResources {
    pub fn create(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut resources = ResourceFile::create(path)?;

        // Add standard resources
        resources.add(RES_APP_TITLE, &"My Application")?;
        resources.add(RES_ABOUT_TEXT, &"Version 1.0\nCopyright 2025")?;

        // Add dialog configs
        let about_dialog = DialogConfig {
            bounds: Rect::new(20, 8, 60, 16),
            title: "About".to_string(),
            controls: vec![
                ControlConfig::Label {
                    bounds: Rect::new(2, 2, 36, 4),
                    text: "My Application\nVersion 1.0".to_string(),
                },
                ControlConfig::Button(ButtonConfig {
                    bounds: Rect::new(15, 5, 25, 7),
                    text: "OK".to_string(),
                    command: CM_OK,
                    is_default: true,
                }),
            ],
        };
        resources.add(RES_ABOUT_DIALOG, &about_dialog)?;

        resources.save_index()?;

        Ok(Self { resources })
    }

    pub fn open(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let resources = ResourceFile::open(path)?;
        Ok(Self { resources })
    }

    pub fn get_dialog(&mut self, id: u32) -> Result<Dialog, Box<dyn std::error::Error>> {
        let config: DialogConfig = self.resources.get(id)?;
        Ok(config.build())
    }

    pub fn get_string(&mut self, id: u32) -> Result<String, Box<dyn std::error::Error>> {
        self.resources.get(id)
    }
}

// Resource IDs
const RES_APP_TITLE: u32 = 100;
const RES_ABOUT_TEXT: u32 = 101;
const RES_ABOUT_DIALOG: u32 = 200;

// Usage
let mut resources = AppResources::open(Path::new("app.res"))?;
let about_dialog = resources.get_dialog(RES_ABOUT_DIALOG)?;
about_dialog.execute(&mut app);
```

### Example 3: Configuration Manager

```rust
#[derive(Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub window_size: (u16, u16),
    pub palette: String,
    pub recent_files: Vec<String>,
    pub editor_settings: EditorSettings,
}

#[derive(Serialize, Deserialize)]
pub struct EditorSettings {
    pub tab_size: usize,
    pub auto_indent: bool,
    pub show_line_numbers: bool,
    pub wrap_lines: bool,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            tab_size: 4,
            auto_indent: true,
            show_line_numbers: true,
            wrap_lines: false,
        }
    }
}

pub struct ConfigManager {
    config: AppConfig,
    path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Self {
        let path = Self::config_path();

        let config = if path.exists() {
            Self::load_from_file(&path).unwrap_or_default()
        } else {
            AppConfig::default()
        };

        Self { config, path }
    }

    fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("myapp")
            .join("config.json")
    }

    fn load_from_file(path: &Path) -> Result<AppConfig, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&contents)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure directory exists
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(&self.config)?;
        std::fs::write(&self.path, json)?;

        Ok(())
    }

    pub fn get(&self) -> &AppConfig {
        &self.config
    }

    pub fn get_mut(&mut self) -> &mut AppConfig {
        &mut self.config
    }
}

// Usage
let mut config_manager = ConfigManager::new();

// Read config
let config = config_manager.get();
println!("Tab size: {}", config.editor_settings.tab_size);

// Modify config
config_manager.get_mut().editor_settings.tab_size = 8;
config_manager.get_mut().recent_files.push("document.txt".to_string());

// Save config
config_manager.save()?;
```

---

## Best Practices

### 1. Use Serde for Serialization

```rust
// ✓ Good - automatic serialization
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub value: i32,
}

// ✗ Bad - manual binary writing
// (Error-prone, not portable)
```

### 2. Separate State from Logic

```rust
// ✓ Good - separate state struct
#[derive(Serialize, Deserialize)]
pub struct WindowState {
    pub bounds: Rect,
    pub title: String,
}

pub struct Window {
    state: WindowState,
    children: Vec<Box<dyn View>>,  // Not serialized
}

// ✗ Bad - try to serialize everything
// (Won't work with trait objects)
```

### 3. Version Your Formats

```rust
// ✓ Good - include version
#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    pub version: u32,
    pub data: ConfigData,
}

// ✗ Bad - no versioning
// (Can't handle format changes)
```

### 4. Handle Errors Gracefully

```rust
// ✓ Good - fallback to defaults
let config = ConfigManager::load()
    .unwrap_or_else(|e| {
        eprintln!("Failed to load config: {}", e);
        Config::default()
    });

// ✗ Bad - crash on error
let config = ConfigManager::load().unwrap();  // Panics!
```

### 5. Use Binary for Performance

```rust
// ✓ Good - binary for large data
let bytes = bincode::serialize(&large_data)?;

// ✗ Bad - JSON for large data
// (Much slower and larger)
let json = serde_json::to_string(&large_data)?;
```

---

## Pascal vs. Rust Summary

| Concept | Pascal | Rust |
|---------|--------|------|
| **Collections** | `TCollection` with pointers | `Vec<Box<dyn Trait>>` |
| **Serialization** | Manual `Store`/`Load` | Serde `#[derive(Serialize, Deserialize)]` |
| **Registration** | `RegisterType(RObject)` | Not needed (type system) |
| **Stream IDs** | Manual ID numbers | Not needed (Serde handles types) |
| **Binary I/O** | `S.Write(data, SizeOf(data))` | `bincode::serialize(&data)` |
| **JSON** | Not available | `serde_json` |
| **Versioning** | Manual | Serde `#[serde(tag = "version")]` |
| **Resources** | Manual indexing | HashMap + binary format |

---

## Summary

### Key Concepts

1. **Heterogeneous Collections** - `Vec<Box<dyn Trait>>` for mixed types
2. **Serde** - Automatic serialization/deserialization
3. **State Objects** - Separate serializable state from runtime logic
4. **Resource Files** - Indexed collections for random access
5. **Binary Format** - Use bincode for efficient storage
6. **JSON Format** - Use serde_json for human-readable config
7. **Versioning** - Handle format evolution gracefully
8. **Configuration** - Persistent app settings and preferences

### The Serialization Pattern

```rust
// 1. Define serializable state
#[derive(Serialize, Deserialize)]
pub struct AppState {
    pub config: Config,
    pub windows: Vec<WindowState>,
}

// 2. Save to file
let state = app.save_state();
let json = serde_json::to_string_pretty(&state)?;
std::fs::write("state.json", json)?;

// 3. Load from file
let json = std::fs::read_to_string("state.json")?;
let state: AppState = serde_json::from_str(&json)?;
app.restore_state(state);
```

---

## See Also

- **Chapter 8** - Views and Groups (View hierarchy)
- **Chapter 10** - Application Objects (Desktop management)
- **Chapter 11** - Windows and Dialogs (State saving)
- **Serde Documentation** - https://serde.rs/
- **Bincode Crate** - https://docs.rs/bincode/
- **examples/persistent_desktop.rs** - Desktop save/load example

---

Collections and streams provide powerful mechanisms for managing heterogeneous data and persistent state in Turbo Vision applications. Master Serde and Rust's type system to build robust, maintainable serialization code.
