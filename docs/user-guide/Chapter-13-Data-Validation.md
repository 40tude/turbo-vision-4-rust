# Chapter 13 — Data Validation (Rust Edition)

**Version:** 0.2.11
**Updated:** 2025-11-04

This chapter explores data validation in Turbo Vision applications. You'll learn how to ensure users enter valid data through input filtering, field validation, and full-form validation using validator objects.

**Prerequisites:** Chapter 12 (Control Objects, especially InputLine)

---

## Table of Contents

1. [Understanding Data Validation](#understanding-data-validation)
2. [Validation Strategies](#validation-strategies)
3. [The Validator Trait](#the-validator-trait)
4. [Standard Validators](#standard-validators)
5. [Using Validators with Input Lines](#using-validators-with-input-lines)
6. [Full-Screen Validation](#full-screen-validation)
7. [Creating Custom Validators](#creating-custom-validators)
8. [Complete Examples](#complete-examples)

---

## Understanding Data Validation

### What is Data Validation?

**Data validation** ensures that user input meets your application's requirements before being processed. Without validation, users might enter:

- Text where numbers are expected
- Values outside acceptable ranges
- Improperly formatted data (dates, phone numbers, etc.)
- Inconsistent or contradictory information

### Three Levels of Validation

Turbo Vision provides three complementary validation approaches:

#### 1. Input Filtering

**When:** As the user types, character by character

**Purpose:** Prevent invalid characters from being entered

```rust
// Only allow digits
let validator = FilterValidator::new(['0'..='9']);

// Result: User can only type numbers
```

#### 2. Field Validation

**When:** User tries to leave the field (Tab, Enter, mouse click)

**Purpose:** Ensure complete field contents are valid

```rust
// Ensure value is in range
let validator = RangeValidator::new(1, 100);

// Result: User can't leave field until value is 1-100
```

#### 3. Form Validation

**When:** User tries to close/submit the dialog

**Purpose:** Ensure all fields together form valid data

```rust
// Check all fields before closing dialog
if dialog.execute(&mut app) == CM_OK {
    // All validators passed
    process_data();
}
```

### Why Separate Validation?

**Problem:** Without validators, each control would need its own validation logic.

**Solution:** Validator objects are reusable, composable, and testable:

```rust
// Reuse the same validator
let zip_validator = Rc::new(RefCell::new(
    PictureValidator::new("#####", false)
));

let zip1 = InputLine::new(bounds1, 5, data1.clone())
    .with_validator(zip_validator.clone());

let zip2 = InputLine::new(bounds2, 5, data2.clone())
    .with_validator(zip_validator.clone());
```

---

## Validation Strategies

### Strategy 1: Filtering Input

**Concept:** Only allow valid characters to be typed

**Use Cases:**
- Numeric fields (only digits)
- Alphabetic fields (only letters)
- Phone numbers (digits, spaces, hyphens)

**Example:**
```
User tries to type 'ABC123' in numeric field
    ↓
Validator filters: 'A' rejected, 'B' rejected, 'C' rejected
    ↓
Result: '123' in field
```

### Strategy 2: Validate on Tab

**Concept:** Check field when user tries to leave it

**Use Cases:**
- Database lookup keys
- Required fields
- Range-checked values

**Example:**
```
User types '150' in 1-100 range field, presses Tab
    ↓
Validator checks: 150 > 100, invalid!
    ↓
Result: Error message, focus stays in field
```

### Strategy 3: Validate on Close

**Concept:** Check all fields when user tries to submit/close

**Use Cases:**
- Modal dialogs
- Data entry forms
- Ensuring completeness

**Example:**
```
User clicks OK on dialog
    ↓
Dialog validates all fields
    ↓
If any field invalid: show error, keep dialog open
If all valid: close dialog, return CM_OK
```

### Combining Strategies

The most robust approach uses all three:

```rust
// 1. Filter input (only digits)
// 2. Validate on Tab (check range)
// 3. Validate on Close (ensure not empty)

let validator = RangeValidator::new(1, 100);
let input = InputLine::new(bounds, 3, data.clone())
    .with_validator(Box::new(validator))
    .with_option(OF_VALIDATE);  // Validate on Tab

dialog.add(Box::new(input));

// Automatically validates on close
let result = dialog.execute(&mut app);
```

---

## The Validator Trait

### Core Trait Definition

```rust
pub trait Validator {
    /// Validate complete input string
    fn is_valid(&self, input: &str) -> bool;

    /// Validate single character input
    fn is_valid_input(&self, current: &str, ch: char) -> bool {
        true  // Default: accept all characters
    }

    /// Report validation error to user
    fn error(&self, input: &str, app: &mut Application);

    /// Validate and show error if invalid
    fn valid(&self, input: &str, app: &mut Application) -> bool {
        if self.is_valid(input) {
            true
        } else {
            self.error(input, app);
            false
        }
    }
}
```

### The Four Methods

#### is_valid()

**Purpose:** Check if complete string is valid

**When Called:** When user tries to leave field or close dialog

**Example:**
```rust
impl Validator for RangeValidator {
    fn is_valid(&self, input: &str) -> bool {
        if let Ok(value) = input.parse::<i32>() {
            value >= self.min && value <= self.max
        } else {
            false
        }
    }
}
```

#### is_valid_input()

**Purpose:** Check if character should be accepted

**When Called:** As user types each character

**Example:**
```rust
impl Validator for FilterValidator {
    fn is_valid_input(&self, _current: &str, ch: char) -> bool {
        self.valid_chars.contains(&ch)
    }
}
```

#### error()

**Purpose:** Show error message to user

**When Called:** By `valid()` if validation fails

**Example:**
```rust
impl Validator for RangeValidator {
    fn error(&self, _input: &str, app: &mut Application) {
        MessageBox::error(&format!(
            "Value must be between {} and {}",
            self.min, self.max
        )).show(app);
    }
}
```

#### valid()

**Purpose:** Validate and report error if needed

**When Called:** External validation (you call this)

**Default Implementation:** Calls `is_valid()`, then `error()` if needed

---

## Standard Validators

Turbo Vision provides several ready-to-use validators.

### FilterValidator

**Purpose:** Restrict input to specific characters

```rust
pub struct FilterValidator {
    valid_chars: HashSet<char>,
}

impl FilterValidator {
    pub fn new(chars: impl IntoIterator<Item = char>) -> Self {
        Self {
            valid_chars: chars.into_iter().collect(),
        }
    }

    pub fn digits() -> Self {
        Self::new('0'..='9')
    }

    pub fn letters() -> Self {
        Self::new(('a'..='z').chain('A'..='Z'))
    }

    pub fn alphanumeric() -> Self {
        Self::new(('a'..='z').chain('A'..='Z').chain('0'..='9'))
    }
}

impl Validator for FilterValidator {
    fn is_valid(&self, _input: &str) -> bool {
        true  // If it got through is_valid_input, it's valid
    }

    fn is_valid_input(&self, _current: &str, ch: char) -> bool {
        self.valid_chars.contains(&ch)
    }

    fn error(&self, _input: &str, _app: &mut Application) {
        // No error - filtering prevents invalid input
    }
}
```

**Usage:**
```rust
// Only digits
let validator = FilterValidator::digits();

// Only letters
let validator = FilterValidator::letters();

// Custom set
let validator = FilterValidator::new(['0'..='9', '+', '-', '.']);

let input = InputLine::new(bounds, 10, data.clone())
    .with_validator(Box::new(validator));
```

### RangeValidator

**Purpose:** Ensure numeric value is within range

```rust
pub struct RangeValidator {
    min: i32,
    max: i32,
    filter: FilterValidator,
}

impl RangeValidator {
    pub fn new(min: i32, max: i32) -> Self {
        // Filter to allow digits, plus, minus
        let filter = FilterValidator::new(
            ('0'..='9').chain(['+', '-'])
        );

        Self { min, max, filter }
    }
}

impl Validator for RangeValidator {
    fn is_valid(&self, input: &str) -> bool {
        if input.is_empty() {
            return false;
        }

        if let Ok(value) = input.parse::<i32>() {
            value >= self.min && value <= self.max
        } else {
            false
        }
    }

    fn is_valid_input(&self, current: &str, ch: char) -> bool {
        // Use filter validator for characters
        self.filter.is_valid_input(current, ch)
    }

    fn error(&self, input: &str, app: &mut Application) {
        let msg = if input.is_empty() {
            format!("Please enter a value between {} and {}", self.min, self.max)
        } else {
            format!(
                "Value '{}' is not between {} and {}",
                input, self.min, self.max
            )
        };

        MessageBox::error(&msg).show(app);
    }
}
```

**Usage:**
```rust
// Age: 0-120
let validator = RangeValidator::new(0, 120);

// Year: 1900-2100
let validator = RangeValidator::new(1900, 2100);

let input = InputLine::new(bounds, 4, data.clone())
    .with_validator(Box::new(validator));
```

### LookupValidator

**Purpose:** Abstract base for validators that check against a list

```rust
pub trait LookupValidator: Validator {
    /// Look up value in list
    fn lookup(&self, input: &str) -> bool;
}

// Default implementations
impl<T: LookupValidator> Validator for T {
    fn is_valid(&self, input: &str) -> bool {
        self.lookup(input)
    }

    fn error(&self, input: &str, app: &mut Application) {
        MessageBox::error(&format!(
            "'{}' is not a valid selection",
            input
        )).show(app);
    }
}
```

### StringLookupValidator

**Purpose:** Validate against a list of strings

```rust
pub struct StringLookupValidator {
    strings: Vec<String>,
    case_sensitive: bool,
}

impl StringLookupValidator {
    pub fn new(strings: Vec<String>, case_sensitive: bool) -> Self {
        Self { strings, case_sensitive }
    }

    pub fn from_slice(strings: &[&str], case_sensitive: bool) -> Self {
        Self::new(
            strings.iter().map(|s| s.to_string()).collect(),
            case_sensitive
        )
    }
}

impl Validator for StringLookupValidator {
    fn is_valid(&self, input: &str) -> bool {
        self.strings.iter().any(|s| {
            if self.case_sensitive {
                s == input
            } else {
                s.eq_ignore_ascii_case(input)
            }
        })
    }

    fn is_valid_input(&self, _current: &str, _ch: char) -> bool {
        true  // Allow any character
    }

    fn error(&self, input: &str, app: &mut Application) {
        MessageBox::error(&format!(
            "'{}' is not in the list of valid values",
            input
        )).show(app);
    }
}
```

**Usage:**
```rust
// State abbreviations
let validator = StringLookupValidator::from_slice(
    &["CA", "NY", "TX", "FL"],
    false  // Case insensitive
);

// Color names
let validator = StringLookupValidator::new(
    vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
    true  // Case sensitive
);

let input = InputLine::new(bounds, 10, data.clone())
    .with_validator(Box::new(validator));
```

### PictureValidator

**Purpose:** Validate against a format template (like Paradox)

**Picture Characters:**
- `#` - Required digit
- `@` - Required letter
- `!` - Required letter (uppercase)
- `*` - Any character
- `;` - Next character is literal

```rust
pub struct PictureValidator {
    picture: String,
    auto_fill: bool,
}

impl PictureValidator {
    pub fn new(picture: &str, auto_fill: bool) -> Self {
        Self {
            picture: picture.to_string(),
            auto_fill,
        }
    }

    fn picture_char_valid(&self, pic_ch: char, input_ch: char) -> bool {
        match pic_ch {
            '#' => input_ch.is_ascii_digit(),
            '@' => input_ch.is_ascii_alphabetic(),
            '!' => input_ch.is_ascii_alphabetic(),
            '*' => true,
            _ => pic_ch == input_ch,  // Literal
        }
    }

    fn format_char(&self, pic_ch: char, input_ch: char) -> char {
        match pic_ch {
            '!' => input_ch.to_ascii_uppercase(),
            _ => input_ch,
        }
    }
}

impl Validator for PictureValidator {
    fn is_valid(&self, input: &str) -> bool {
        if input.len() != self.picture.len() {
            return false;
        }

        for (i, pic_ch) in self.picture.chars().enumerate() {
            if let Some(input_ch) = input.chars().nth(i) {
                if !self.picture_char_valid(pic_ch, input_ch) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    fn is_valid_input(&self, current: &str, ch: char) -> bool {
        if current.len() >= self.picture.len() {
            return false;
        }

        if let Some(pic_ch) = self.picture.chars().nth(current.len()) {
            self.picture_char_valid(pic_ch, ch)
        } else {
            false
        }
    }

    fn error(&self, _input: &str, app: &mut Application) {
        MessageBox::error(&format!(
            "Input must match format: {}",
            self.picture
        )).show(app);
    }
}
```

**Usage:**
```rust
// US Phone: (###) ###-####
let validator = PictureValidator::new("(###) ###-####", true);

// US ZIP: #####
let validator = PictureValidator::new("#####", false);

// Date: ##/##/####
let validator = PictureValidator::new("##/##/####", false);

// License plate: @@@-####
let validator = PictureValidator::new("@@@-####", false);

let input = InputLine::new(bounds, 14, data.clone())
    .with_validator(Box::new(validator));
```

---

## Using Validators with Input Lines

### Basic Usage

**From Chapter 12**, InputLine has an optional validator:

```rust
pub struct InputLine {
    bounds: Rect,
    data: Rc<RefCell<String>>,
    max_length: usize,
    cursor_pos: usize,
    first_pos: usize,
    state: StateFlags,
    validator: Option<Box<dyn Validator>>,  // ← Optional validator
}
```

### Adding a Validator

```rust
impl InputLine {
    pub fn with_validator(mut self, validator: Box<dyn Validator>) -> Self {
        self.validator = Some(validator);
        self
    }

    pub fn set_validator(&mut self, validator: Box<dyn Validator>) {
        self.validator = Some(validator);
    }
}
```

**Usage:**
```rust
// Method 1: Builder pattern
let input = InputLine::new(bounds, 10, data.clone())
    .with_validator(Box::new(RangeValidator::new(1, 100)));

// Method 2: Set after construction
let mut input = InputLine::new(bounds, 10, data.clone());
input.set_validator(Box::new(FilterValidator::digits()));
```

### How InputLine Uses Validators

#### During Character Input

```rust
impl View for InputLine {
    fn handle_event(&mut self, event: &mut Event) {
        if event.what == EventType::Keyboard {
            if let Some(ch) = key_to_char(event.key_code) {
                // Check with validator first
                if let Some(ref validator) = self.validator {
                    let current = self.data.borrow();
                    if !validator.is_valid_input(&current, ch) {
                        // Invalid character - reject
                        event.clear();
                        return;
                    }
                }

                // Character is valid - insert it
                self.insert_char(ch);
                event.clear();
            }
        }
    }
}
```

#### During Validation

```rust
impl View for InputLine {
    fn valid(&self, app: &mut Application, command: u16) -> bool {
        if command == CM_CANCEL {
            return true;  // Always allow cancel
        }

        if let Some(ref validator) = self.validator {
            let input = self.data.borrow();
            validator.valid(&input, app)
        } else {
            true  // No validator - always valid
        }
    }
}
```

### Validate on Tab

Enable field validation when user leaves the field:

```rust
let mut input = InputLine::new(bounds, 10, data.clone())
    .with_validator(Box::new(RangeValidator::new(1, 100)));

// Set OF_VALIDATE option
input.set_option(OF_VALIDATE, true);

dialog.add(Box::new(input));
```

**What happens:**
```
User types '150', presses Tab
    ↓
InputLine loses focus
    ↓
OF_VALIDATE is set → call valid()
    ↓
Validator checks: 150 > 100 → invalid!
    ↓
Error message shown, focus returns to field
```

---

## Full-Screen Validation

### Modal Dialog Validation

Modal dialogs automatically validate on close:

```rust
impl Dialog {
    pub fn execute(&mut self, app: &mut Application) -> u16 {
        self.window.set_state_flag(SF_MODAL, true);

        let mut running = true;
        let mut result = 0;

        while running {
            // ... event loop ...

            if event.what == EventType::Command {
                match event.command {
                    CM_OK => {
                        // Validate all controls
                        if self.valid(app, CM_OK) {
                            result = CM_OK;
                            running = false;
                        }
                        // If invalid, stay open
                    }
                    CM_CANCEL => {
                        // Don't validate on cancel
                        result = CM_CANCEL;
                        running = false;
                    }
                    _ => {}
                }
            }
        }

        self.window.set_state_flag(SF_MODAL, false);
        result
    }
}
```

### Dialog valid() Method

```rust
impl Dialog {
    pub fn valid(&mut self, app: &mut Application, command: u16) -> bool {
        if command == CM_CANCEL {
            return true;  // Always allow cancel
        }

        // Validate all child views
        self.interior.valid(app, command)
    }
}

impl Group {
    pub fn valid(&mut self, app: &mut Application, command: u16) -> bool {
        // Validate children in Z-order
        for child in &self.children {
            if !child.valid(app, command) {
                return false;
            }
        }
        true
    }
}
```

### Modeless Window Validation

For modeless windows, explicitly call `valid()`:

```rust
impl MyApp {
    fn save_data(&mut self) {
        if let Some(ref mut data_window) = self.data_window {
            // Validate before saving
            if data_window.valid(&mut self.app, CM_OK) {
                // All fields valid - save
                self.save_to_database();
            } else {
                // Validation failed - validator showed error
            }
        }
    }
}
```

### Validate on Focus Change

Set `OF_VALIDATE` on the window itself:

```rust
let mut window = Window::new(bounds, "Data Entry");
window.set_option(OF_VALIDATE, true);

// Now when window loses focus, it validates all fields
```

---

## Creating Custom Validators

### Simple Custom Validator

**Example:** Email validator

```rust
pub struct EmailValidator;

impl EmailValidator {
    pub fn new() -> Self {
        Self
    }
}

impl Validator for EmailValidator {
    fn is_valid(&self, input: &str) -> bool {
        // Simple email check
        input.contains('@') &&
        input.split('@').count() == 2 &&
        input.split('@').nth(1).map_or(false, |domain| domain.contains('.'))
    }

    fn is_valid_input(&self, _current: &str, _ch: char) -> bool {
        true  // Allow any character
    }

    fn error(&self, input: &str, app: &mut Application) {
        MessageBox::error(&format!(
            "'{}' is not a valid email address",
            input
        )).show(app);
    }
}
```

### Composite Validator

**Example:** Validator that combines multiple validators

```rust
pub struct CompositeValidator {
    validators: Vec<Box<dyn Validator>>,
}

impl CompositeValidator {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
        }
    }

    pub fn add(mut self, validator: Box<dyn Validator>) -> Self {
        self.validators.push(validator);
        self
    }
}

impl Validator for CompositeValidator {
    fn is_valid(&self, input: &str) -> bool {
        // All validators must pass
        self.validators.iter().all(|v| v.is_valid(input))
    }

    fn is_valid_input(&self, current: &str, ch: char) -> bool {
        // All validators must accept character
        self.validators.iter().all(|v| v.is_valid_input(current, ch))
    }

    fn error(&self, input: &str, app: &mut Application) {
        // Show error from first failing validator
        for validator in &self.validators {
            if !validator.is_valid(input) {
                validator.error(input, app);
                return;
            }
        }
    }
}
```

**Usage:**
```rust
// Combine filter + range validation
let validator = CompositeValidator::new()
    .add(Box::new(FilterValidator::digits()))
    .add(Box::new(RangeValidator::new(18, 65)));

let input = InputLine::new(bounds, 3, data.clone())
    .with_validator(Box::new(validator));
```

### Stateful Validator

**Example:** Validator that tracks attempts

```rust
pub struct AttemptsValidator {
    inner: Box<dyn Validator>,
    max_attempts: usize,
    attempts: RefCell<usize>,
}

impl AttemptsValidator {
    pub fn new(inner: Box<dyn Validator>, max_attempts: usize) -> Self {
        Self {
            inner,
            max_attempts,
            attempts: RefCell::new(0),
        }
    }
}

impl Validator for AttemptsValidator {
    fn is_valid(&self, input: &str) -> bool {
        self.inner.is_valid(input)
    }

    fn is_valid_input(&self, current: &str, ch: char) -> bool {
        self.inner.is_valid_input(current, ch)
    }

    fn error(&self, input: &str, app: &mut Application) {
        *self.attempts.borrow_mut() += 1;

        if *self.attempts.borrow() >= self.max_attempts {
            MessageBox::error(&format!(
                "Too many invalid attempts ({}). Please contact support.",
                self.max_attempts
            )).show(app);
        } else {
            self.inner.error(input, app);
        }
    }
}
```

### Database Lookup Validator

**Example:** Validator that checks against database

```rust
pub struct DatabaseValidator {
    table: String,
    column: String,
    // In real implementation, would have database connection
}

impl DatabaseValidator {
    pub fn new(table: &str, column: &str) -> Self {
        Self {
            table: table.to_string(),
            column: column.to_string(),
        }
    }

    fn lookup_in_database(&self, value: &str) -> bool {
        // Simulate database lookup
        // In real implementation:
        // SELECT COUNT(*) FROM table WHERE column = value

        // For demo, just check if it's "admin"
        value == "admin"
    }
}

impl Validator for DatabaseValidator {
    fn is_valid(&self, input: &str) -> bool {
        if input.is_empty() {
            return false;
        }

        self.lookup_in_database(input)
    }

    fn is_valid_input(&self, _current: &str, _ch: char) -> bool {
        true  // Allow typing
    }

    fn error(&self, input: &str, app: &mut Application) {
        MessageBox::error(&format!(
            "'{}' not found in {}",
            input, self.table
        )).show(app);
    }
}
```

---

## Complete Examples

### Example 1: User Registration Form

```rust
use turbo_vision::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct RegistrationData {
    pub username: Rc<RefCell<String>>,
    pub password: Rc<RefCell<String>>,
    pub email: Rc<RefCell<String>>,
    pub age: Rc<RefCell<String>>,
    pub zip: Rc<RefCell<String>>,
}

impl RegistrationData {
    pub fn new() -> Self {
        Self {
            username: Rc::new(RefCell::new(String::new())),
            password: Rc::new(RefCell::new(String::new())),
            email: Rc::new(RefCell::new(String::new())),
            age: Rc::new(RefCell::new(String::new())),
            zip: Rc::new(RefCell::new(String::new())),
        }
    }
}

pub fn create_registration_dialog(data: &RegistrationData) -> Dialog {
    let mut dialog = Dialog::new(
        Rect::new(10, 5, 70, 20),
        "User Registration"
    );

    let mut y = 2;

    // Username: 3-20 alphanumeric characters
    dialog.add(Box::new(Label::new(
        Rect::new(2, y, 14, y + 1),
        "~U~sername:"
    )));

    let username_input = InputLine::new(
        Rect::new(14, y, 46, y + 1),
        20,
        data.username.clone()
    )
    .with_validator(Box::new(CompositeValidator::new()
        .add(Box::new(FilterValidator::alphanumeric()))
        .add(Box::new(LengthValidator::new(3, 20)))
    ))
    .with_option(OF_VALIDATE, true);  // Validate on Tab

    dialog.add(Box::new(username_input));
    y += 2;

    // Password: minimum 8 characters
    dialog.add(Box::new(Label::new(
        Rect::new(2, y, 14, y + 1),
        "~P~assword:"
    )));

    let password_input = InputLine::new(
        Rect::new(14, y, 46, y + 1),
        50,
        data.password.clone()
    )
    .with_password_mode()
    .with_validator(Box::new(LengthValidator::new(8, 50)))
    .with_option(OF_VALIDATE, true);

    dialog.add(Box::new(password_input));
    y += 2;

    // Email: must be valid email format
    dialog.add(Box::new(Label::new(
        Rect::new(2, y, 14, y + 1),
        "~E~mail:"
    )));

    let email_input = InputLine::new(
        Rect::new(14, y, 46, y + 1),
        50,
        data.email.clone()
    )
    .with_validator(Box::new(EmailValidator::new()))
    .with_option(OF_VALIDATE, true);

    dialog.add(Box::new(email_input));
    y += 2;

    // Age: 13-120
    dialog.add(Box::new(Label::new(
        Rect::new(2, y, 14, y + 1),
        "~A~ge:"
    )));

    let age_input = InputLine::new(
        Rect::new(14, y, 20, y + 1),
        3,
        data.age.clone()
    )
    .with_validator(Box::new(RangeValidator::new(13, 120)))
    .with_option(OF_VALIDATE, true);

    dialog.add(Box::new(age_input));
    y += 2;

    // ZIP code: #####
    dialog.add(Box::new(Label::new(
        Rect::new(2, y, 14, y + 1),
        "~Z~IP Code:"
    )));

    let zip_input = InputLine::new(
        Rect::new(14, y, 20, y + 1),
        5,
        data.zip.clone()
    )
    .with_validator(Box::new(PictureValidator::new("#####", false)))
    .with_option(OF_VALIDATE, true);

    dialog.add(Box::new(zip_input));
    y += 2;

    // Buttons
    dialog.add(Box::new(Button::new(
        Rect::new(20, y, 30, y + 2),
        "~R~egister",
        CM_OK,
        true
    )));

    dialog.add(Box::new(Button::new(
        Rect::new(32, y, 42, y + 2),
        "~C~ancel",
        CM_CANCEL,
        false
    )));

    dialog
}

// Usage
fn show_registration(&mut self) {
    let data = RegistrationData::new();
    let mut dialog = create_registration_dialog(&data);

    let result = dialog.execute(&mut self.app);

    if result == CM_OK {
        // All validation passed!
        println!("Username: {}", data.username.borrow());
        println!("Email: {}", data.email.borrow());
        println!("Age: {}", data.age.borrow());
        println!("ZIP: {}", data.zip.borrow());

        // Save to database
        self.save_user(&data);
    }
}
```

### Example 2: Custom LengthValidator

```rust
pub struct LengthValidator {
    min: usize,
    max: usize,
}

impl LengthValidator {
    pub fn new(min: usize, max: usize) -> Self {
        Self { min, max }
    }
}

impl Validator for LengthValidator {
    fn is_valid(&self, input: &str) -> bool {
        let len = input.len();
        len >= self.min && len <= self.max
    }

    fn is_valid_input(&self, _current: &str, _ch: char) -> bool {
        true  // Allow any character
    }

    fn error(&self, input: &str, app: &mut Application) {
        let len = input.len();

        let msg = if len < self.min {
            format!("Too short: {} characters (minimum {})", len, self.min)
        } else {
            format!("Too long: {} characters (maximum {})", len, self.max)
        };

        MessageBox::error(&msg).show(app);
    }
}
```

### Example 3: Date Validator

```rust
pub struct DateValidator;

impl DateValidator {
    pub fn new() -> Self {
        Self
    }

    fn parse_date(input: &str) -> Option<(u32, u32, u32)> {
        let parts: Vec<&str> = input.split('/').collect();

        if parts.len() != 3 {
            return None;
        }

        let month = parts[0].parse::<u32>().ok()?;
        let day = parts[1].parse::<u32>().ok()?;
        let year = parts[2].parse::<u32>().ok()?;

        Some((month, day, year))
    }

    fn is_valid_date(month: u32, day: u32, year: u32) -> bool {
        if month < 1 || month > 12 {
            return false;
        }

        if year < 1900 || year > 2100 {
            return false;
        }

        let days_in_month = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                // Leap year check
                if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => return false,
        };

        day >= 1 && day <= days_in_month
    }
}

impl Validator for DateValidator {
    fn is_valid(&self, input: &str) -> bool {
        if let Some((month, day, year)) = Self::parse_date(input) {
            Self::is_valid_date(month, day, year)
        } else {
            false
        }
    }

    fn is_valid_input(&self, current: &str, ch: char) -> bool {
        // Allow digits and slashes
        ch.is_ascii_digit() || ch == '/'
    }

    fn error(&self, input: &str, app: &mut Application) {
        MessageBox::error(&format!(
            "'{}' is not a valid date (MM/DD/YYYY)",
            input
        )).show(app);
    }
}

// Usage with PictureValidator for format
let date_input = InputLine::new(bounds, 10, data.clone())
    .with_validator(Box::new(CompositeValidator::new()
        .add(Box::new(PictureValidator::new("##/##/####", false)))
        .add(Box::new(DateValidator::new()))
    ));
```

### Example 4: Conditional Validator

**Example:** Validate based on other field

```rust
pub struct ConditionalValidator {
    condition: Rc<RefCell<bool>>,
    validator: Box<dyn Validator>,
}

impl ConditionalValidator {
    pub fn new(condition: Rc<RefCell<bool>>, validator: Box<dyn Validator>) -> Self {
        Self { condition, validator }
    }
}

impl Validator for ConditionalValidator {
    fn is_valid(&self, input: &str) -> bool {
        if *self.condition.borrow() {
            self.validator.is_valid(input)
        } else {
            true  // Not required
        }
    }

    fn is_valid_input(&self, current: &str, ch: char) -> bool {
        if *self.condition.borrow() {
            self.validator.is_valid_input(current, ch)
        } else {
            true
        }
    }

    fn error(&self, input: &str, app: &mut Application) {
        self.validator.error(input, app);
    }
}

// Usage: SSN required if US citizen
let is_us_citizen = Rc::new(RefCell::new(false));

// Checkbox
let checkbox = Checkbox::new(
    Rect::new(2, 2, 20, 3),
    "~U~S Citizen",
    is_us_citizen.clone()
);
dialog.add(Box::new(checkbox));

// SSN field - only validated if US citizen
let ssn_input = InputLine::new(
    Rect::new(12, 4, 24, 5),
    11,
    ssn_data.clone()
)
.with_validator(Box::new(ConditionalValidator::new(
    is_us_citizen.clone(),
    Box::new(PictureValidator::new("###-##-####", false))
)));
dialog.add(Box::new(ssn_input));
```

---

## Best Practices

### 1. Use Appropriate Validator Type

```rust
// ✓ Good - filter for single characters
let validator = FilterValidator::digits();

// ✗ Bad - range validator for character filtering
// (Range validator filters, but also does range check which may confuse users)
```

### 2. Combine Validators When Needed

```rust
// ✓ Good - filter + range
let validator = CompositeValidator::new()
    .add(Box::new(FilterValidator::digits()))
    .add(Box::new(RangeValidator::new(1, 100)));

// ✗ Bad - only range (user can type letters, then see error)
let validator = RangeValidator::new(1, 100);
```

### 3. Set OF_VALIDATE for Important Fields

```rust
// ✓ Good - validate on Tab for database keys
let input = InputLine::new(bounds, 10, data.clone())
    .with_validator(Box::new(DatabaseValidator::new("users", "id")))
    .with_option(OF_VALIDATE, true);

// ✗ Bad - only validate on dialog close
// (User might fill entire form before finding out first field is invalid)
```

### 4. Always Allow Cancel

```rust
// ✓ Good - validator checks command
impl Dialog {
    fn valid(&mut self, app: &mut Application, command: u16) -> bool {
        if command == CM_CANCEL {
            return true;  // Always allow cancel
        }
        // ... validate fields
    }
}

// ✗ Bad - can't escape invalid dialog
```

### 5. Clear Error Messages

```rust
// ✓ Good - specific error
fn error(&self, input: &str, app: &mut Application) {
    MessageBox::error(&format!(
        "Age must be between 18 and 65 (you entered '{}')",
        input
    )).show(app);
}

// ✗ Bad - vague error
fn error(&self, _input: &str, app: &mut Application) {
    MessageBox::error("Invalid input").show(app);
}
```

### 6. Don't Duplicate Logic

```rust
// ✓ Good - reuse validator
let phone_validator = Rc::new(RefCell::new(
    PictureValidator::new("(###) ###-####", true)
));

let home_phone = InputLine::new(bounds1, 14, home.clone())
    .with_validator(Box::new(phone_validator.clone()));

let work_phone = InputLine::new(bounds2, 14, work.clone())
    .with_validator(Box::new(phone_validator.clone()));

// ✗ Bad - duplicate validators
let home_phone = InputLine::new(bounds1, 14, home.clone())
    .with_validator(Box::new(PictureValidator::new("(###) ###-####", true)));

let work_phone = InputLine::new(bounds2, 14, work.clone())
    .with_validator(Box::new(PictureValidator::new("(###) ###-####", true)));
```

---

## Pascal vs. Rust Summary

| Concept | Pascal | Rust |
|---------|--------|------|
| **Base Type** | `TValidator = object` | `trait Validator` |
| **Validator Field** | `Validator: PValidator` | `validator: Option<Box<dyn Validator>>` |
| **Set Validator** | `SetValidator(New(...))` | `with_validator(Box::new(...))` |
| **Valid Check** | `function Valid: Boolean` | `fn is_valid(&self, &str) -> bool` |
| **Input Filter** | `function IsValidInput: Boolean` | `fn is_valid_input(&self, &str, char) -> bool` |
| **Error Report** | `procedure Error` | `fn error(&self, &str, &mut Application)` |
| **Filter Set** | `ValidChars: TCharSet` | `valid_chars: HashSet<char>` |
| **Inheritance** | `TRangeValidator = object(TFilterValidator)` | Trait composition |
| **Lookup** | Virtual `function Lookup: Boolean` | `trait LookupValidator: Validator` |
| **String List** | `Strings: PStringCollection` | `strings: Vec<String>` |
| **Picture** | `Pic: String` | `picture: String` |

---

## Summary

### Key Concepts

1. **Three Validation Levels** - Filter input, validate field, validate form
2. **Validator Trait** - Four methods: `is_valid`, `is_valid_input`, `error`, `valid`
3. **Standard Validators** - Filter, Range, Lookup, StringLookup, Picture
4. **InputLine Integration** - Optional validator, automatic validation
5. **OF_VALIDATE Option** - Validate when field loses focus
6. **Modal Validation** - Automatic on close (except CM_CANCEL)
7. **Custom Validators** - Implement Validator trait
8. **Composite Validators** - Combine multiple validators

### The Validation Pattern

```rust
// 1. Create validator
let validator = RangeValidator::new(1, 100);

// 2. Attach to input line
let input = InputLine::new(bounds, 3, data.clone())
    .with_validator(Box::new(validator))
    .with_option(OF_VALIDATE, true);  // Validate on Tab

// 3. Add to dialog
dialog.add(Box::new(input));

// 4. Execute - validates on close
let result = dialog.execute(&mut app);

if result == CM_OK {
    // All validation passed
    process_data(&data.borrow());
}
```

---

## See Also

- **Chapter 12** - Control Objects (InputLine)
- **Chapter 11** - Windows and Dialogs (Modal execution)
- **Chapter 9** - Event-Driven Programming (Event handling)
- **docs/TURBOVISION-DESIGN.md** - Implementation details
- **examples/form_validation_demo.rs** - Validation examples

---

Data validation is essential for robust applications. Use validators to ensure data quality, provide clear feedback to users, and prevent invalid data from entering your system.
