# Chapter 6 — Creating Data Entry Forms (Rust Edition)

**Version:** 0.2.11
**Updated:** 2025-11-04

Now that you understand windows and data collections, it's time to create interactive data entry forms. This chapter shows how to build dialog boxes with input fields, labels, buttons, and validators—the building blocks of user interfaces.

In this chapter, you'll learn:

- Creating custom dialog windows
- Adding input controls (InputLine, Label, Button)
- Working with checkboxes and radio buttons
- Setting and reading control values with shared state
- Validating user input
- Managing dialog lifecycle

---

## Understanding Data Entry in Turbo Vision

### Dialog Boxes vs. Windows

In Turbo Vision, a **Dialog** is a specialized type of Window:

- **Windows** - Can be modal or modeless, resizable, movable
- **Dialogs** - Typically modal, centered, fixed size
- Both can contain controls (buttons, input fields, etc.)

### Modal vs. Modeless

**Modal dialogs:**
- Block interaction with other windows
- Used with `dialog.execute(app)`
- Return a command when closed (CM_OK, CM_CANCEL)

**Modeless dialogs:**
- Allow interaction with other windows
- Used with `app.desktop.add(Box::new(dialog))`
- Behave like regular windows

---

## Step 1: Creating a Custom Dialog

Let's create an order entry form for a simple inventory system.

### Defining the Dialog Structure

```rust
// order_dialog.rs
use turbo_vision::core::geometry::Rect;
use turbo_vision::core::event::Event;
use turbo_vision::core::command::CommandId;
use turbo_vision::views::dialog::Dialog;
use turbo_vision::views::input_line::InputLine;
use turbo_vision::views::label::Label;
use turbo_vision::views::button::Button;
use turbo_vision::views::checkbox::Checkbox;
use turbo_vision::views::radiobutton::RadioButton;
use turbo_vision::views::view::View;
use turbo_vision::terminal::Terminal;
use std::rc::Rc;
use std::cell::RefCell;

// Custom commands for this dialog
pub const CMD_ORDER_SAVE: u16 = 3001;
pub const CMD_ORDER_CANCEL: u16 = 3002;

pub struct OrderDialog {
    dialog: Dialog,
    // Shared data for controls
    order_num: Rc<RefCell<String>>,
    customer: Rc<RefCell<String>>,
    product: Rc<RefCell<String>>,
    quantity: Rc<RefCell<String>>,
    payment_method: Rc<RefCell<u16>>,  // 0=Cash, 1=Check, 2=Card
    received: Rc<RefCell<bool>>,
}

impl OrderDialog {
    pub fn new() -> Self {
        // Create dialog centered on screen
        let bounds = Rect::new(10, 5, 70, 20);
        let mut dialog = Dialog::new(bounds, "Order Entry");

        // Shared data cells
        let order_num = Rc::new(RefCell::new(String::new()));
        let customer = Rc::new(RefCell::new(String::new()));
        let product = Rc::new(RefCell::new(String::new()));
        let quantity = Rc::new(RefCell::new(String::new()));
        let payment_method = Rc::new(RefCell::new(0u16));
        let received = Rc::new(RefCell::new(false));

        let mut y = 2;

        // Order Number field
        dialog.add(Box::new(Label::new(
            Rect::new(2, y, 15, y + 1),
            "~O~rder #:"
        )));
        dialog.add(Box::new(InputLine::new(
            Rect::new(16, y, 28, y + 1),
            10,
            order_num.clone()
        )));
        y += 2;

        // Customer field
        dialog.add(Box::new(Label::new(
            Rect::new(2, y, 15, y + 1),
            "~C~ustomer:"
        )));
        dialog.add(Box::new(InputLine::new(
            Rect::new(16, y, 56, y + 1),
            50,
            customer.clone()
        )));
        y += 2;

        // Product field
        dialog.add(Box::new(Label::new(
            Rect::new(2, y, 15, y + 1),
            "~P~roduct:"
        )));
        dialog.add(Box::new(InputLine::new(
            Rect::new(16, y, 56, y + 1),
            50,
            product.clone()
        )));
        y += 2;

        // Quantity field
        dialog.add(Box::new(Label::new(
            Rect::new(2, y, 15, y + 1),
            "~Q~uantity:"
        )));
        dialog.add(Box::new(InputLine::new(
            Rect::new(16, y, 26, y + 1),
            10,
            quantity.clone()
        )));
        y += 2;

        // Payment method (radio buttons)
        dialog.add(Box::new(Label::new(
            Rect::new(2, y, 20, y + 1),
            "Payment Method:"
        )));
        y += 1;

        let radio1 = RadioButton::new(
            Rect::new(4, y, 16, y + 1),
            "~C~ash",
            0,
            payment_method.clone()
        );
        dialog.add(Box::new(radio1));

        let radio2 = RadioButton::new(
            Rect::new(17, y, 29, y + 1),
            "C~h~eck",
            1,
            payment_method.clone()
        );
        dialog.add(Box::new(radio2));

        let radio3 = RadioButton::new(
            Rect::new(30, y, 42, y + 1),
            "Credi~t~ Card",
            2,
            payment_method.clone()
        );
        dialog.add(Box::new(radio3));
        y += 2;

        // Received checkbox
        let checkbox = Checkbox::new(
            Rect::new(4, y, 20, y + 1),
            "~R~eceived",
            received.clone()
        );
        dialog.add(Box::new(checkbox));
        y += 2;

        // Buttons
        let save_btn = Button::new(
            Rect::new(20, y, 30, y + 2),
            "~S~ave",
            turbo_vision::core::command::CM_OK,
            true  // default button
        );
        dialog.add(Box::new(save_btn));

        let cancel_btn = Button::new(
            Rect::new(32, y, 42, y + 2),
            "~A~bort",
            turbo_vision::core::command::CM_CANCEL,
            false
        );
        dialog.add(Box::new(cancel_btn));

        Self {
            dialog,
            order_num,
            customer,
            product,
            quantity,
            payment_method,
            received,
        }
    }

    /// Set dialog values from an Order struct
    pub fn set_data(&mut self, order: &Order) {
        *self.order_num.borrow_mut() = order.order_num.clone();
        *self.customer.borrow_mut() = order.customer.clone();
        *self.product.borrow_mut() = order.product.clone();
        *self.quantity.borrow_mut() = order.quantity.to_string();
        *self.payment_method.borrow_mut() = order.payment_method;
        *self.received.borrow_mut() = order.received;
    }

    /// Get dialog values into an Order struct
    pub fn get_data(&self) -> Order {
        Order {
            order_num: self.order_num.borrow().clone(),
            customer: self.customer.borrow().clone(),
            product: self.product.borrow().clone(),
            quantity: self.quantity.borrow().parse().unwrap_or(0),
            payment_method: *self.payment_method.borrow(),
            received: *self.received.borrow(),
        }
    }

    /// Execute the dialog (modal)
    pub fn execute(&mut self, app: &mut turbo_vision::app::Application) -> u16 {
        self.dialog.execute(app)
    }
}

// Data structure
#[derive(Debug, Clone)]
pub struct Order {
    pub order_num: String,
    pub customer: String,
    pub product: String,
    pub quantity: u32,
    pub payment_method: u16,
    pub received: bool,
}

impl Order {
    pub fn new() -> Self {
        Self {
            order_num: String::new(),
            customer: String::new(),
            product: String::new(),
            quantity: 0,
            payment_method: 0,
            received: false,
        }
    }
}
```

### Using the Dialog

```rust
// main.rs
use turbo_vision::app::Application;
use turbo_vision::core::command::{CM_OK, CM_CANCEL};

mod order_dialog;
use order_dialog::{OrderDialog, Order};

fn main() -> std::io::Result<()> {
    let mut app = Application::new()?;

    // Create dialog with initial data
    let mut dialog = OrderDialog::new();
    let mut order = Order::new();
    order.customer = "Acme Corp".to_string();
    order.product = "Widget".to_string();
    order.quantity = 100;

    dialog.set_data(&order);

    // Execute dialog (modal - blocks until closed)
    let result = dialog.execute(&mut app);

    if result == CM_OK {
        // User clicked Save - get the data
        let order = dialog.get_data();
        println!("Order saved: {:?}", order);
    } else {
        println!("Order cancelled");
    }

    Ok(())
}
```

---

## Step 2: Understanding Shared State with Rc<RefCell<T>>

Rust's ownership system requires careful handling of shared mutable state in dialogs.

### The Pattern

```rust
// Create shared data
let name_data = Rc::new(RefCell::new(String::new()));

// Share with input line
let input = InputLine::new(bounds, 50, name_data.clone());
dialog.add(Box::new(input));

// Later, read the value
let value = name_data.borrow().clone();

// Or modify it
*name_data.borrow_mut() = "New value".to_string();
```

### Why Rc<RefCell<T>>?

- **Rc** - Reference counting allows multiple owners
- **RefCell** - Enables interior mutability (mutation through shared reference)
- **Together** - Multiple controls can share the same data

### Comparison with Pascal

| Pascal | Rust |
|--------|------|
| `TInputLine` stores string directly | `InputLine` uses `Rc<RefCell<String>>` |
| `GetData()`/`SetData()` with records | Direct access to shared data |
| Pointer-based sharing | Reference-counted sharing |
| Manual memory management | Automatic cleanup |

---

## Step 3: Adding Validators

Validators ensure users enter correct data before the dialog closes.

### Built-in Validators

The Rust implementation provides three main validators:

1. **FilterValidator** - Allow only specific characters
2. **RangeValidator** - Numeric range validation
3. **PictureValidator** - Format mask validation

### Example: Filter Validator

```rust
use turbo_vision::views::validator::FilterValidator;
use std::rc::Rc;
use std::cell::RefCell;

// Only allow digits
let quantity_data = Rc::new(RefCell::new(String::new()));
let quantity_validator = Rc::new(RefCell::new(
    FilterValidator::new("0123456789")
));

let mut quantity_input = InputLine::new(
    Rect::new(16, y, 26, y + 1),
    10,
    quantity_data.clone()
);
quantity_input.set_validator(quantity_validator);

dialog.add(Box::new(quantity_input));
```

### Example: Range Validator

```rust
use turbo_vision::views::validator::RangeValidator;

// Number between 1 and 999
let quantity_data = Rc::new(RefCell::new(String::from("1")));
let range_validator = Rc::new(RefCell::new(
    RangeValidator::new(1, 999)
));

let mut input = InputLine::with_validator(
    bounds,
    10,
    quantity_data.clone(),
    range_validator
);

dialog.add(Box::new(input));
```

### Example: Picture Validator

```rust
use turbo_vision::views::picture_validator::PictureValidator;

// Phone number format: (###) ###-####
let phone_data = Rc::new(RefCell::new(String::new()));
let phone_validator = Rc::new(RefCell::new(
    PictureValidator::new("(###) ###-####")
));

let mut phone_input = InputLine::with_validator(
    bounds,
    15,
    phone_data.clone(),
    phone_validator
);

dialog.add(Box::new(phone_input));
```

### Picture Validator Patterns

| Pattern | Meaning | Example |
|---------|---------|---------|
| `#` | Digit (0-9) | `###` → `"123"` |
| `@` | Letter (A-Z, a-z) | `@@@@` → `"ABCD"` |
| `!` | Letter (uppercase) | `!!!` → `"ABC"` |
| `&` | Any character | `&&&` → `"A1$"` |
| Literal | Must match exactly | `(###) ###-####` |

Common patterns:
- Phone: `"(###) ###-####"` → `"(555) 123-4567"`
- Date: `"##/##/####"` → `"12/25/2025"`
- SSN: `"###-##-####"` → `"123-45-6789"`
- Product code: `"@@@@-####"` → `"ABCD-1234"`

---

## Complete Example: Order Entry System

Here's a complete, working order entry system with validation:

```rust
// complete_order_system.rs
use turbo_vision::app::Application;
use turbo_vision::core::geometry::Rect;
use turbo_vision::core::command::{CM_OK, CM_CANCEL, CM_QUIT};
use turbo_vision::core::event::{EventType, KB_ALT_X, KB_F2};
use turbo_vision::core::menu_data::{Menu, MenuItem};
use turbo_vision::views::menu_bar::{MenuBar, SubMenu};
use turbo_vision::views::status_line::{StatusLine, StatusItem};
use turbo_vision::views::dialog::Dialog;
use turbo_vision::views::input_line::InputLine;
use turbo_vision::views::label::Label;
use turbo_vision::views::button::Button;
use turbo_vision::views::checkbox::Checkbox;
use turbo_vision::views::radiobutton::RadioButton;
use turbo_vision::views::static_text::StaticText;
use turbo_vision::views::validator::{FilterValidator, RangeValidator};
use turbo_vision::views::picture_validator::PictureValidator;
use turbo_vision::views::msgbox::{message_box_ok, confirmation_box};
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;

// Commands
const CMD_NEW_ORDER: u16 = 3001;
const CMD_EDIT_ORDER: u16 = 3002;

#[derive(Debug, Clone)]
struct Order {
    order_num: String,
    date: String,
    customer: String,
    product: String,
    quantity: String,
    price: String,
    payment_method: u16,
    received: bool,
}

impl Order {
    fn new() -> Self {
        Self {
            order_num: String::new(),
            date: String::new(),
            customer: String::new(),
            product: String::new(),
            quantity: String::from("1"),
            price: String::from("0.00"),
            payment_method: 0,
            received: false,
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut app = Application::new()?;
    let (width, height) = app.terminal.size();

    let menu_bar = create_menu_bar(width);
    app.set_menu_bar(menu_bar);

    let status_line = create_status_line(height, width);
    app.set_status_line(status_line);

    let mut current_order = Order::new();

    app.running = true;
    while app.running {
        // Draw
        app.desktop.draw(&mut app.terminal);
        if let Some(ref mut menu_bar) = app.menu_bar {
            menu_bar.draw(&mut app.terminal);
        }
        if let Some(ref mut status_line) = app.status_line {
            status_line.draw(&mut app.terminal);
        }
        let _ = app.terminal.flush();

        // Handle events
        if let Ok(Some(mut event)) = app.terminal.poll_event(Duration::from_millis(50)) {
            // PreProcess
            if let Some(ref mut status_line) = app.status_line {
                status_line.handle_event(&mut event);
            }

            // Menu bar
            if let Some(ref mut menu_bar) = app.menu_bar {
                menu_bar.handle_event(&mut event);
                if event.what == EventType::Keyboard || event.what == EventType::MouseUp {
                    if let Some(command) = menu_bar.check_cascading_submenu(&mut app.terminal) {
                        if command != 0 {
                            event = turbo_vision::core::event::Event::command(command);
                        }
                    }
                }
            }

            // Desktop
            app.desktop.handle_event(&mut event);

            // Commands
            if event.what == EventType::Command {
                match event.command {
                    CM_QUIT => {
                        app.running = false;
                    }
                    CMD_NEW_ORDER => {
                        current_order = Order::new();
                        if let Some(order) = show_order_dialog(&mut app, &current_order) {
                            current_order = order;
                            message_box_ok(&mut app, "Success", "Order created!");
                        }
                    }
                    CMD_EDIT_ORDER => {
                        if let Some(order) = show_order_dialog(&mut app, &current_order) {
                            current_order = order;
                            message_box_ok(&mut app, "Success", "Order updated!");
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn show_order_dialog(app: &mut Application, initial_data: &Order) -> Option<Order> {
    let mut dialog = Dialog::new(Rect::new(5, 3, 75, 22), "Order Entry");

    // Title
    let title = StaticText::new_centered(
        Rect::new(2, 1, 68, 2),
        "Enter Order Information"
    );
    dialog.add(Box::new(title));

    let mut y = 3;

    // Order Number (digits only, range 1-99999)
    dialog.add(Box::new(Label::new(
        Rect::new(2, y, 15, y + 1),
        "~O~rder #:"
    )));
    let order_num_data = Rc::new(RefCell::new(initial_data.order_num.clone()));
    let order_num_validator = Rc::new(RefCell::new(RangeValidator::new(1, 99999)));
    let order_num_input = InputLine::with_validator(
        Rect::new(16, y, 26, y + 1),
        10,
        order_num_data.clone(),
        order_num_validator
    );
    dialog.add(Box::new(order_num_input));

    // Date (picture validator MM/DD/YYYY)
    dialog.add(Box::new(Label::new(
        Rect::new(35, y, 45, y + 1),
        "~D~ate:"
    )));
    let date_data = Rc::new(RefCell::new(initial_data.date.clone()));
    let date_validator = Rc::new(RefCell::new(PictureValidator::new("##/##/####")));
    let date_input = InputLine::with_validator(
        Rect::new(46, y, 58, y + 1),
        10,
        date_data.clone(),
        date_validator
    );
    dialog.add(Box::new(date_input));
    y += 2;

    // Customer (no validation)
    dialog.add(Box::new(Label::new(
        Rect::new(2, y, 15, y + 1),
        "~C~ustomer:"
    )));
    let customer_data = Rc::new(RefCell::new(initial_data.customer.clone()));
    let customer_input = InputLine::new(
        Rect::new(16, y, 66, y + 1),
        50,
        customer_data.clone()
    );
    dialog.add(Box::new(customer_input));
    y += 2;

    // Product (no validation)
    dialog.add(Box::new(Label::new(
        Rect::new(2, y, 15, y + 1),
        "~P~roduct:"
    )));
    let product_data = Rc::new(RefCell::new(initial_data.product.clone()));
    let product_input = InputLine::new(
        Rect::new(16, y, 66, y + 1),
        50,
        product_data.clone()
    );
    dialog.add(Box::new(product_input));
    y += 2;

    // Quantity (range 1-9999)
    dialog.add(Box::new(Label::new(
        Rect::new(2, y, 15, y + 1),
        "~Q~uantity:"
    )));
    let quantity_data = Rc::new(RefCell::new(initial_data.quantity.clone()));
    let quantity_validator = Rc::new(RefCell::new(RangeValidator::new(1, 9999)));
    let quantity_input = InputLine::with_validator(
        Rect::new(16, y, 26, y + 1),
        10,
        quantity_data.clone(),
        quantity_validator
    );
    dialog.add(Box::new(quantity_input));

    // Price (digits and decimal point)
    dialog.add(Box::new(Label::new(
        Rect::new(35, y, 45, y + 1),
        "P~r~ice:"
    )));
    let price_data = Rc::new(RefCell::new(initial_data.price.clone()));
    let price_validator = Rc::new(RefCell::new(FilterValidator::new("0123456789.")));
    let price_input = InputLine::with_validator(
        Rect::new(46, y, 58, y + 1),
        10,
        price_data.clone(),
        price_validator
    );
    dialog.add(Box::new(price_input));
    y += 2;

    // Payment method (radio buttons)
    dialog.add(Box::new(Label::new(
        Rect::new(2, y, 20, y + 1),
        "Payment ~M~ethod:"
    )));
    y += 1;

    let payment_method = Rc::new(RefCell::new(initial_data.payment_method));

    let cash_radio = RadioButton::new(
        Rect::new(4, y, 16, y + 1),
        "C~a~sh",
        0,
        payment_method.clone()
    );
    dialog.add(Box::new(cash_radio));

    let check_radio = RadioButton::new(
        Rect::new(17, y, 29, y + 1),
        "C~h~eck",
        1,
        payment_method.clone()
    );
    dialog.add(Box::new(check_radio));

    let card_radio = RadioButton::new(
        Rect::new(30, y, 50, y + 1),
        "Credit Car~d~",
        2,
        payment_method.clone()
    );
    dialog.add(Box::new(card_radio));
    y += 2;

    // Received checkbox
    let received = Rc::new(RefCell::new(initial_data.received));
    let received_checkbox = Checkbox::new(
        Rect::new(4, y, 20, y + 1),
        "~R~eceived",
        received.clone()
    );
    dialog.add(Box::new(received_checkbox));
    y += 2;

    // Buttons
    let save_btn = Button::new(
        Rect::new(24, y, 34, y + 2),
        "~S~ave",
        CM_OK,
        true
    );
    dialog.add(Box::new(save_btn));

    let cancel_btn = Button::new(
        Rect::new(36, y, 46, y + 2),
        "~A~bort",
        CM_CANCEL,
        false
    );
    dialog.add(Box::new(cancel_btn));

    // Execute dialog
    let result = dialog.execute(app);

    if result == CM_OK {
        // Collect data
        Some(Order {
            order_num: order_num_data.borrow().clone(),
            date: date_data.borrow().clone(),
            customer: customer_data.borrow().clone(),
            product: product_data.borrow().clone(),
            quantity: quantity_data.borrow().clone(),
            price: price_data.borrow().clone(),
            payment_method: *payment_method.borrow(),
            received: *received.borrow(),
        })
    } else {
        None
    }
}

fn create_menu_bar(width: u16) -> MenuBar {
    let mut menu_bar = MenuBar::new(Rect::new(0, 0, width as i16, 1));

    let file_items = vec![
        MenuItem::with_shortcut("~N~ew Order", CMD_NEW_ORDER, 0, "", 0),
        MenuItem::with_shortcut("~E~dit Order", CMD_EDIT_ORDER, 0, "", 0),
        MenuItem::separator(),
        MenuItem::with_shortcut("E~x~it", CM_QUIT, KB_ALT_X, "Alt+X", 0),
    ];
    menu_bar.add_submenu(SubMenu::new("~F~ile", Menu::from_items(file_items)));

    menu_bar
}

fn create_status_line(height: u16, width: u16) -> StatusLine {
    StatusLine::new(
        Rect::new(0, height as i16 - 1, width as i16, height as i16),
        vec![
            StatusItem::new("~Alt+X~ Exit", KB_ALT_X, CM_QUIT),
            StatusItem::new("~F2~ New", KB_F2, CMD_NEW_ORDER),
        ],
    )
}
```

---

## Best Practices

### 1. Data Management

**Use Rc<RefCell<T>> for shared state:**
```rust
// Good - shared state
let name = Rc::new(RefCell::new(String::new()));
let input = InputLine::new(bounds, 50, name.clone());

// Later access
let value = name.borrow().clone();
```

### 2. Validation

**Validate early and clearly:**
```rust
// Attach validators to input lines
input.set_validator(Rc::new(RefCell::new(
    RangeValidator::new(1, 100)
)));

// Check validation before processing
if !input.validate() {
    message_box_error(app, "Invalid input!");
    return;
}
```

### 3. Tab Order

**Add controls in the order users should tab through them:**
```rust
dialog.add(Box::new(name_input));      // Tab 1
dialog.add(Box::new(address_input));   // Tab 2
dialog.add(Box::new(city_input));      // Tab 3
dialog.add(Box::new(ok_button));       // Tab 4
dialog.add(Box::new(cancel_button));   // Tab 5
```

### 4. Default Buttons

**Mark the primary action as default:**
```rust
let save_btn = Button::new(
    bounds,
    "~S~ave",
    CM_OK,
    true  // This is the default button (activated by Enter)
);
```

### 5. Labels

**Always link labels to their controls:**
```rust
// Bad - no association
dialog.add(Box::new(Label::new(bounds, "Name:")));

// Good - implicit association through positioning
// (Current implementation uses position-based association)
```

---

## Summary

In this chapter, you learned:

### Dialog Creation:
- Creating custom dialog boxes
- Adding controls (InputLine, Label, Button, Checkbox, RadioButton)
- Modal vs. modeless dialogs
- Dialog lifecycle and execution

### Data Management:
- Using `Rc<RefCell<T>>` for shared state
- Setting initial values
- Reading values after dialog closes
- Passing data structs in and out

### Validation:
- FilterValidator for character filtering
- RangeValidator for numeric ranges
- PictureValidator for format masks
- Attaching validators to input lines

### Best Practices:
- Proper tab order
- Default buttons
- Clear labeling
- Early validation
- Error handling

---

## See Also

- **Chapter 5** - Managing Data Collections (CRUD operations)
- **examples/validator_demo.rs** - All validator types
- **examples/dialogs_demo.rs** - Standard dialog examples
- **src/views/input_line.rs** - InputLine implementation
- **src/views/validator.rs** - Validator trait and implementations
- **src/views/picture_validator.rs** - Picture mask patterns

---

## Next Steps

Build on these concepts to create:
- Multi-page dialogs with tab controls
- Complex validation rules
- Lookup validators (database-backed)
- Custom control types
- Wizard-style interfaces
