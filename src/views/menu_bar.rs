use crate::core::geometry::Rect;
use crate::core::event::{Event, EventType, KeyCode, KB_ALT_F, KB_ALT_H, KB_ENTER, KB_ESC, KB_LEFT, KB_RIGHT, KB_DOWN, KB_UP, KB_ESC_F, KB_ESC_H, KB_ESC_E, KB_ESC_S, KB_ESC_V, KB_ESC_ESC, MB_LEFT_BUTTON};
use crate::core::draw::DrawBuffer;
use crate::core::palette::colors;
use crate::core::command::CommandId;
use crate::terminal::Terminal;
use super::view::{View, write_line_to_terminal};

pub enum MenuItem {
    Regular {
        text: String,
        command: CommandId,
        key_code: KeyCode,
        enabled: bool,
    },
    Separator,
}

impl MenuItem {
    pub fn new(text: &str, command: CommandId, key_code: KeyCode) -> Self {
        Self::Regular {
            text: text.to_string(),
            command,
            key_code,
            enabled: true,
        }
    }

    pub fn new_disabled(text: &str, command: CommandId, key_code: KeyCode) -> Self {
        Self::Regular {
            text: text.to_string(),
            command,
            key_code,
            enabled: false,
        }
    }

    pub fn separator() -> Self {
        Self::Separator
    }

    pub fn is_selectable(&self) -> bool {
        match self {
            Self::Regular { enabled, .. } => *enabled,
            Self::Separator => false,
        }
    }

    /// Extract the accelerator key from the text (character between ~ marks)
    pub fn get_accelerator(&self) -> Option<char> {
        match self {
            Self::Regular { text, .. } => {
                let mut chars = text.chars();
                while let Some(ch) = chars.next() {
                    if ch == '~' {
                        // Next char is the accelerator
                        if let Some(accel) = chars.next() {
                            return Some(accel.to_ascii_lowercase());
                        }
                    }
                }
                None
            }
            Self::Separator => None,
        }
    }
}

pub struct SubMenu {
    pub name: String,
    pub items: Vec<MenuItem>,
}

impl SubMenu {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: MenuItem) {
        self.items.push(item);
    }
}

pub struct MenuBar {
    bounds: Rect,
    menus: Vec<SubMenu>,
    menu_positions: Vec<i16>,  // X positions of each menu for dropdown placement
    active_menu: Option<usize>,
    selected_item: usize,
}

impl MenuBar {
    pub fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            menus: Vec::new(),
            menu_positions: Vec::new(),
            active_menu: None,
            selected_item: 0,
        }
    }

    pub fn add_menu(&mut self, menu: SubMenu) {
        self.menus.push(menu);
        self.menu_positions.push(0);  // Will be updated during draw
    }

    fn select_first_item(&mut self, menu_idx: usize) {
        if menu_idx < self.menus.len() {
            let menu = &self.menus[menu_idx];
            // Find first selectable item
            for (i, item) in menu.items.iter().enumerate() {
                if item.is_selectable() {
                    self.selected_item = i;
                    return;
                }
            }
            self.selected_item = 0; // Fallback
        }
    }
}

impl View for MenuBar {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn draw(&mut self, terminal: &mut Terminal) {
        let width = self.bounds.width() as usize;
        let mut buf = DrawBuffer::new(width);
        buf.move_char(0, ' ', colors::MENU_NORMAL, width);

        // Draw menu names and track their positions
        let mut x: usize = 1;
        for (i, menu) in self.menus.iter().enumerate() {
            // Store the starting position of this menu
            if i < self.menu_positions.len() {
                self.menu_positions[i] = x as i16;
            }

            let attr = if self.active_menu == Some(i) {
                colors::MENU_SELECTED
            } else {
                colors::MENU_NORMAL
            };

            // Parse ~X~ for highlighting - everything between tildes is red
            buf.put_char(x, ' ', attr);
            x += 1;

            let mut chars = menu.name.chars();
            #[allow(clippy::while_let_on_iterator)]
            while let Some(ch) = chars.next() {
                if ch == '~' {
                    // Read all characters until closing ~ in shortcut color
                    let shortcut_attr = if self.active_menu == Some(i) {
                        colors::MENU_SELECTED
                    } else {
                        colors::MENU_SHORTCUT  // Red on LightGray
                    };
                    #[allow(clippy::while_let_on_iterator)]
                    while let Some(shortcut_ch) = chars.next() {
                        if shortcut_ch == '~' {
                            break;  // Found closing tilde
                        }
                        buf.put_char(x, shortcut_ch, shortcut_attr);
                        x += 1;
                    }
                } else {
                    buf.put_char(x, ch, attr);
                    x += 1;
                }
            }

            buf.put_char(x, ' ', attr);
            x += 1;
        }

        write_line_to_terminal(terminal, self.bounds.a.x, self.bounds.a.y, &buf);

        // Draw dropdown if active
        if let Some(idx) = self.active_menu {
            if idx < self.menus.len() {
                let menu = &self.menus[idx];
                let menu_x = if idx < self.menu_positions.len() {
                    self.menu_positions[idx]
                } else {
                    1
                };
                let menu_y = self.bounds.a.y + 1;

                for (i, item) in menu.items.iter().enumerate() {
                    let mut item_buf = DrawBuffer::new(30);

                    match item {
                        MenuItem::Separator => {
                            // Draw separator line
                            item_buf.move_char(0, '─', colors::MENU_NORMAL, 30);
                        }
                        MenuItem::Regular { text, enabled, .. } => {
                            let attr = if i == self.selected_item && *enabled {
                                colors::MENU_SELECTED
                            } else if !enabled {
                                colors::MENU_DISABLED
                            } else {
                                colors::MENU_NORMAL
                            };

                            item_buf.move_char(0, ' ', attr, 30);

                            // Parse ~X~ for highlighting in menu items - everything between tildes is red
                            let mut x = 1;
                            let mut chars = text.chars();
                            #[allow(clippy::while_let_on_iterator)]
                            while let Some(ch) = chars.next() {
                                if ch == '~' {
                                    // Read all characters until closing ~ in shortcut color
                                    let shortcut_attr = if i == self.selected_item && *enabled {
                                        colors::MENU_SELECTED
                                    } else if !enabled {
                                        colors::MENU_DISABLED
                                    } else {
                                        colors::MENU_SHORTCUT  // Red on LightGray
                                    };
                                    #[allow(clippy::while_let_on_iterator)]
                                    while let Some(shortcut_ch) = chars.next() {
                                        if shortcut_ch == '~' {
                                            break;  // Found closing tilde
                                        }
                                        item_buf.put_char(x, shortcut_ch, shortcut_attr);
                                        x += 1;
                                    }
                                } else {
                                    item_buf.put_char(x, ch, attr);
                                    x += 1;
                                }
                            }
                        }
                    }

                    write_line_to_terminal(terminal, menu_x, menu_y + i as i16, &item_buf);
                }
            }
        }
    }

    fn handle_event(&mut self, event: &mut Event) {
        // Handle mouse events
        if event.what == EventType::MouseDown {
            let mouse_pos = event.mouse.pos;

            if event.mouse.buttons & MB_LEFT_BUTTON != 0 {
                // Check if click is on the menu bar
                if mouse_pos.y == self.bounds.a.y {
                    // Check which menu was clicked
                    for (i, &menu_x) in self.menu_positions.iter().enumerate() {
                        if i < self.menus.len() {
                            let menu = &self.menus[i];
                            let menu_width = menu.name.replace('~', "").len() as i16 + 2;

                            if mouse_pos.x >= menu_x && mouse_pos.x < menu_x + menu_width {
                                // Toggle menu if clicking same menu, or switch to new menu
                                if self.active_menu == Some(i) {
                                    self.active_menu = None;
                                } else {
                                    self.active_menu = Some(i);
                                    self.select_first_item(i);
                                }
                                event.clear();
                                return;
                            }
                        }
                    }

                    // Clicked on menu bar but not on a menu - close any open menu
                    if self.active_menu.is_some() {
                        self.active_menu = None;
                        event.clear();
                        return;
                    }
                }

                // Check if click is on a dropdown menu item
                if let Some(menu_idx) = self.active_menu {
                    if menu_idx < self.menus.len() && menu_idx < self.menu_positions.len() {
                        let menu_x = self.menu_positions[menu_idx];
                        let menu_y = self.bounds.a.y + 1;
                        let menu = &self.menus[menu_idx];

                        // Check if click is within dropdown bounds
                        if mouse_pos.x >= menu_x && mouse_pos.x < menu_x + 30
                            && mouse_pos.y >= menu_y && mouse_pos.y < menu_y + menu.items.len() as i16 {
                            let item_idx = (mouse_pos.y - menu_y) as usize;

                            if item_idx < menu.items.len() {
                                let item = &menu.items[item_idx];

                                if item.is_selectable() {
                                    if let MenuItem::Regular { command, .. } = item {
                                        // Close menu and execute command
                                        self.active_menu = None;
                                        *event = Event::command(*command);
                                        return;
                                    }
                                }
                            }
                        } else {
                            // Clicked outside dropdown - close menu
                            self.active_menu = None;
                            event.clear();
                            return;
                        }
                    }
                }
            }
        }

        // Handle mouse move (hover) events
        if event.what == EventType::MouseMove {
            if let Some(menu_idx) = self.active_menu {
                if menu_idx < self.menus.len() && menu_idx < self.menu_positions.len() {
                    let mouse_pos = event.mouse.pos;
                    let menu_x = self.menu_positions[menu_idx];
                    let menu_y = self.bounds.a.y + 1;
                    let menu = &self.menus[menu_idx];

                    // Check if mouse is hovering over a menu item
                    if mouse_pos.x >= menu_x && mouse_pos.x < menu_x + 30
                        && mouse_pos.y >= menu_y && mouse_pos.y < menu_y + menu.items.len() as i16 {
                        let item_idx = (mouse_pos.y - menu_y) as usize;

                        if item_idx < menu.items.len() && item_idx != self.selected_item {
                            // Update selection based on hover
                            self.selected_item = item_idx;
                        }
                    }

                    // Check if mouse is hovering over a different menu on the menu bar
                    if mouse_pos.y == self.bounds.a.y {
                        for (i, &menu_x_pos) in self.menu_positions.iter().enumerate() {
                            if i < self.menus.len() && i != menu_idx {
                                let hover_menu = &self.menus[i];
                                let hover_menu_width = hover_menu.name.replace('~', "").len() as i16 + 2;

                                if mouse_pos.x >= menu_x_pos && mouse_pos.x < menu_x_pos + hover_menu_width {
                                    // Switch to the hovered menu
                                    self.active_menu = Some(i);
                                    self.select_first_item(i);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        if event.what == EventType::Keyboard {
            // Alt+F, F1, or ESC+F opens File menu
            if (event.key_code == KB_ALT_F
                || event.key_code == crate::core::event::KB_F1
                || event.key_code == KB_ESC_F)
                && !self.menus.is_empty() {
                self.active_menu = Some(0);
                self.select_first_item(0);
                event.clear();
                return;
            }

            // ESC+E opens Edit menu (index 1)
            if event.key_code == KB_ESC_E && self.menus.len() > 1 {
                self.active_menu = Some(1);
                self.select_first_item(1);
                event.clear();
                return;
            }

            // ESC+S opens Search menu (index 2)
            if event.key_code == KB_ESC_S && self.menus.len() > 2 {
                self.active_menu = Some(2);
                self.select_first_item(2);
                event.clear();
                return;
            }

            // ESC+V opens View menu (index 3)
            if event.key_code == KB_ESC_V && self.menus.len() > 3 {
                self.active_menu = Some(3);
                self.select_first_item(3);
                event.clear();
                return;
            }

            // Alt+H or ESC+H opens Help menu (last menu)
            if (event.key_code == KB_ALT_H || event.key_code == KB_ESC_H)
                && self.menus.len() > 1 {
                self.active_menu = Some(self.menus.len() - 1);
                self.select_first_item(self.menus.len() - 1);
                event.clear();
                return;
            }

            // Handle menu navigation
            if let Some(menu_idx) = self.active_menu {
                match event.key_code {
                    KB_ESC | KB_ESC_ESC => {
                        self.active_menu = None;
                        event.clear();
                    }
                    KB_LEFT => {
                        // Navigate to previous menu
                        if menu_idx > 0 {
                            self.active_menu = Some(menu_idx - 1);
                        } else {
                            self.active_menu = Some(self.menus.len() - 1);
                        }
                        self.select_first_item(self.active_menu.unwrap());
                        event.clear();
                    }
                    KB_RIGHT => {
                        // Navigate to next menu
                        self.active_menu = Some((menu_idx + 1) % self.menus.len());
                        self.select_first_item(self.active_menu.unwrap());
                        event.clear();
                    }
                    KB_DOWN => {
                        if menu_idx < self.menus.len() {
                            let menu = &self.menus[menu_idx];
                            let start_pos = self.selected_item;
                            loop {
                                self.selected_item = (self.selected_item + 1) % menu.items.len();
                                // Stop if we found a selectable item or we've wrapped around
                                if menu.items[self.selected_item].is_selectable() || self.selected_item == start_pos {
                                    break;
                                }
                            }
                            event.clear();
                        }
                    }
                    KB_UP => {
                        if menu_idx < self.menus.len() {
                            let menu = &self.menus[menu_idx];
                            let start_pos = self.selected_item;
                            loop {
                                if self.selected_item == 0 {
                                    self.selected_item = menu.items.len() - 1;
                                } else {
                                    self.selected_item -= 1;
                                }
                                // Stop if we found a selectable item or we've wrapped around
                                if menu.items[self.selected_item].is_selectable() || self.selected_item == start_pos {
                                    break;
                                }
                            }
                            event.clear();
                        }
                    }
                    KB_ENTER => {
                        if menu_idx < self.menus.len() && self.selected_item < self.menus[menu_idx].items.len() {
                            let item = &self.menus[menu_idx].items[self.selected_item];
                            if let MenuItem::Regular { command, enabled, .. } = item {
                                if *enabled {
                                    // Close menu first, then create command event
                                    self.active_menu = None;
                                    *event = Event::command(*command);
                                    return; // Return early so command event isn't cleared
                                }
                            }
                        }
                        event.clear();
                    }
                    key_code => {
                        // Check for accelerator keys (a-z, A-Z)
                        if (32..127).contains(&key_code) {
                            let pressed_char = (key_code as u8 as char).to_ascii_lowercase();

                            // Search for menu item with matching accelerator
                            if menu_idx < self.menus.len() {
                                let menu = &self.menus[menu_idx];
                                for item in &menu.items {
                                    if let Some(accel) = item.get_accelerator() {
                                        if accel == pressed_char && item.is_selectable() {
                                            // Found matching accelerator!
                                            if let MenuItem::Regular { command, .. } = item {
                                                // Close menu first, then create command event
                                                self.active_menu = None;
                                                *event = Event::command(*command);
                                                return;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}