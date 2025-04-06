use std::collections::HashMap;
use std::path::Path;
use macroquad::input::{is_key_down, is_key_released, is_mouse_button_down, is_mouse_button_released, KeyCode, MouseButton};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Action {
    // movement
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Sprint,

    // interaction
    Interact,
    Inventory,
    Pause,

    // combat
    BasicAttack,

    // UI
    UIClick, // pressing a button
    UIRightClick, // right-click on a button

    // MISC
    Debug,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum ExpectedPressType {
    /*
    Press - When the button is pressed down
    If the button is pressed down, will continue to be fired until it is released
    */
    Press,

    /*
    Release - When the button is released
    This will fire one time when the button is released, and will not fire again until after it is pressed down again
    */
    Release,

    // /*
    // PressCapture - When the button is pressed down, fired once
    // This will fire one time when the button is pressed down, and will not fire again until after it is released
    // Can be used for things like the pause menu where the same key triggers open / close
    // */
    // PressCapture,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum BindingType {
    Key(u16),
    Mouse(u16),
}

impl Into<u16> for BindingType {
    fn into(self) -> u16 {
        match self {
            BindingType::Key(k) => k,
            BindingType::Mouse(m) => m,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Binding {
    pub binding: Vec<(BindingType, ExpectedPressType)>,
}

impl Binding {
    pub fn new(binding: Vec<(BindingType, ExpectedPressType)>) -> Self {
        Self {
            binding,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ControlHandler {
    bindings: HashMap<Action, Binding>,
}

impl ControlHandler {
    /// Create the default control mapping file if it doesn't exist
    fn create_default_control_mapping() -> Result<(), String> {
        let raw_path = "./data/controls.dat".to_string();
        let path = Path::new(&raw_path);
        
        if !path.exists() {
            // make the directories
            if let Err(e) = std::fs::create_dir_all(path.parent().unwrap()) {
                return Err(format!("Failed to create data directory: {}", e));
            }
            // create the file
            if let Err(e) = std::fs::File::create(path) {
                return Err(format!("Failed to create controls.dat: {}", e));
            }
        }
        
        // serialize default values
        let defaults = ControlHandler::default();
        let Ok(serialized) = serde_json::to_string(&defaults) else {
            return Err(format!("Failed to create default control mapping. Delete {} to regenerate.", path.display()));
        };
        
        // write the defaults to the file
        if let Err(e) = std::fs::write(path, serialized) {
            return Err(format!("Failed to write default control mapping to file: {}", e));
        }
        
        Ok(())
    }

    /// Load the control mapping from the file, creating it if it doesn't exist
    pub fn load() -> Result<Self, String> {
        let raw_path = "./data/controls.dat".to_string();
        let path = Path::new(&raw_path);
        
        if !path.exists() {
            Self::create_default_control_mapping()?;
        }
        
        let contents = std::fs::read_to_string(path).unwrap();

        let res = serde_json::from_str(&contents);

        if let Err(e) = res {
            return Err(format!("Failed to load control mapping: `{}`. If this error persists, delete {}", e, path.display()));
        }

        Ok(res.unwrap())
    }

    /// Save the control mapping to the file
    pub fn save(&self) {
        let raw_path = "./data/controls.dat".to_string();
        let path = Path::new(&raw_path);
        
        if !path.exists() {
            Self::create_default_control_mapping().unwrap();
        }
        
        let serialized = serde_json::to_string(self).unwrap();
        
        std::fs::write(path, serialized).unwrap();
    }

    /// Get the actions that have occurred
    pub fn get_actions(&self) -> Vec<Action> {
        let mut active = Vec::new();

        for action in self.bindings.keys() {
            if self.is_action_active(action) {
                active.push(action.clone());
            }
        }

        active
    }

    pub fn is_action_active(&self, action: &Action) -> bool {
        let mut is_active = true;
        for (bind, ept) in &self.bindings.get(&action).unwrap().binding {
            match ept {
                ExpectedPressType::Press => {
                    if !self.is_bind_pressed(bind) {
                        is_active = false;
                    }
                }
                ExpectedPressType::Release => {
                    if !self.is_bind_released(bind) {
                        is_active = false;
                    }
                }
            }
        }

        is_active
    }

    fn is_bind_pressed(&self, bind: &BindingType) -> bool {
        match bind {
            BindingType::Key(key) => is_key_down(u16_to_keycode(*key)),
            BindingType::Mouse(mb) => is_mouse_button_down(u16_to_mousecode(*mb)),
        }
    }

    fn is_bind_released(&self, bind: &BindingType) -> bool {
        match bind {
            BindingType::Key(key) => is_key_released(u16_to_keycode(*key)),
            BindingType::Mouse(mb) => is_mouse_button_released(u16_to_mousecode(*mb)),
        }
    }
    
    pub fn edit_keybind(&mut self, action: Action, new_key: Binding) {
        self.bindings.insert(action, new_key);
        
        self.save();
    }
}

impl Default for ControlHandler {
    fn default() -> Self {
        let mut bindings = HashMap::new();

        // == Movement ==
        
        bindings.insert(Action::MoveUp, Binding::new(
            vec!((BindingType::Key(KeyCode::W as u16), ExpectedPressType::Press))));

        bindings.insert(Action::MoveDown, Binding::new(
            vec!((BindingType::Key(KeyCode::S as u16), ExpectedPressType::Press))));

        bindings.insert(Action::MoveLeft, Binding::new(
            vec!((BindingType::Key(KeyCode::A as u16), ExpectedPressType::Press))));

        bindings.insert(Action::MoveRight, Binding::new(
            vec!((BindingType::Key(KeyCode::D as u16), ExpectedPressType::Press))));

        bindings.insert(Action::Sprint, Binding::new(vec!((BindingType::Key(KeyCode::LeftShift as u16),
                                                           ExpectedPressType::Press)))); // todo: change to release for toggle sprint

        // == Interaction ==

        bindings.insert(Action::Interact, Binding::new(vec!((BindingType::Mouse(MouseButton::Right as u16),
                                                             ExpectedPressType::Release))));

        bindings.insert(Action::Inventory, Binding::new(vec!((BindingType::Key(KeyCode::Tab as u16),
                                                             ExpectedPressType::Release))));

        bindings.insert(Action::Pause, Binding::new(vec!((BindingType::Key(KeyCode::Escape as u16),
                                                             ExpectedPressType::Release))));

        // == UI ==

        bindings.insert(Action::UIClick, Binding::new(vec!((BindingType::Mouse(MouseButton::Left as u16),
                                                            ExpectedPressType::Release))));
        bindings.insert(Action::UIRightClick, Binding::new(vec!((BindingType::Mouse(MouseButton::Right as u16),
                                                            ExpectedPressType::Release))));

        // == Combat ==

        bindings.insert(Action::BasicAttack, Binding::new(vec!((BindingType::Mouse(MouseButton::Left as u16),
                  ExpectedPressType::Press))));

        // == Misc ==

        bindings.insert(Action::Debug, Binding::new(vec!((BindingType::Key(KeyCode::GraveAccent as u16),
                                                           ExpectedPressType::Release))));
        
        Self {
            bindings,
        }
    }
}

pub fn u16_to_keycode(key: u16) -> KeyCode {
    match key {
        0x0020 => KeyCode::Space,
        0x0027 => KeyCode::Apostrophe,
        0x002c => KeyCode::Comma,
        0x002d => KeyCode::Minus,
        0x002e => KeyCode::Period,
        0x002f => KeyCode::Slash,
        0x0030 => KeyCode::Key0,
        0x0031 => KeyCode::Key1,
        0x0032 => KeyCode::Key2,
        0x0033 => KeyCode::Key3,
        0x0034 => KeyCode::Key4,
        0x0035 => KeyCode::Key5,
        0x0036 => KeyCode::Key6,
        0x0037 => KeyCode::Key7,
        0x0038 => KeyCode::Key8,
        0x0039 => KeyCode::Key9,
        0x003b => KeyCode::Semicolon,
        0x003d => KeyCode::Equal,
        0x0041 => KeyCode::A,
        0x0042 => KeyCode::B,
        0x0043 => KeyCode::C,
        0x0044 => KeyCode::D,
        0x0045 => KeyCode::E,
        0x0046 => KeyCode::F,
        0x0047 => KeyCode::G,
        0x0048 => KeyCode::H,
        0x0049 => KeyCode::I,
        0x004a => KeyCode::J,
        0x004b => KeyCode::K,
        0x004c => KeyCode::L,
        0x004d => KeyCode::M,
        0x004e => KeyCode::N,
        0x004f => KeyCode::O,
        0x0050 => KeyCode::P,
        0x0051 => KeyCode::Q,
        0x0052 => KeyCode::R,
        0x0053 => KeyCode::S,
        0x0054 => KeyCode::T,
        0x0055 => KeyCode::U,
        0x0056 => KeyCode::V,
        0x0057 => KeyCode::W,
        0x0058 => KeyCode::X,
        0x0059 => KeyCode::Y,
        0x005a => KeyCode::Z,
        0x005b => KeyCode::LeftBracket,
        0x005c => KeyCode::Backslash,
        0x005d => KeyCode::RightBracket,
        0x0060 => KeyCode::GraveAccent,
        0x0100 => KeyCode::World1,
        0x0101 => KeyCode::World2,
        0xff1b => KeyCode::Escape,
        0xff0d => KeyCode::Enter,
        0xff09 => KeyCode::Tab,
        0xff08 => KeyCode::Backspace,
        0xff63 => KeyCode::Insert,
        0xffff => KeyCode::Delete,
        0xff53 => KeyCode::Right,
        0xff51 => KeyCode::Left,
        0xff54 => KeyCode::Down,
        0xff52 => KeyCode::Up,
        0xff55 => KeyCode::PageUp,
        0xff56 => KeyCode::PageDown,
        0xff50 => KeyCode::Home,
        0xff57 => KeyCode::End,
        0xffe5 => KeyCode::CapsLock,
        0xff14 => KeyCode::ScrollLock,
        0xff7f => KeyCode::NumLock,
        0xfd1d => KeyCode::PrintScreen,
        0xff13 => KeyCode::Pause,
        0xffbe => KeyCode::F1,
        0xffbf => KeyCode::F2,
        0xffc0 => KeyCode::F3,
        0xffc1 => KeyCode::F4,
        0xffc2 => KeyCode::F5,
        0xffc3 => KeyCode::F6,
        0xffc4 => KeyCode::F7,
        0xffc5 => KeyCode::F8,
        0xffc6 => KeyCode::F9,
        0xffc7 => KeyCode::F10,
        0xffc8 => KeyCode::F11,
        0xffc9 => KeyCode::F12,
        0xffca => KeyCode::F13,
        0xffcb => KeyCode::F14,
        0xffcc => KeyCode::F15,
        0xffcd => KeyCode::F16,
        0xffce => KeyCode::F17,
        0xffcf => KeyCode::F18,
        0xffd0 => KeyCode::F19,
        0xffd1 => KeyCode::F20,
        0xffd2 => KeyCode::F21,
        0xffd3 => KeyCode::F22,
        0xffd4 => KeyCode::F23,
        0xffd5 => KeyCode::F24,
        0xffd6 => KeyCode::F25,
        0xffb0 => KeyCode::Kp0,
        0xffb1 => KeyCode::Kp1,
        0xffb2 => KeyCode::Kp2,
        0xffb3 => KeyCode::Kp3,
        0xffb4 => KeyCode::Kp4,
        0xffb5 => KeyCode::Kp5,
        0xffb6 => KeyCode::Kp6,
        0xffb7 => KeyCode::Kp7,
        0xffb8 => KeyCode::Kp8,
        0xffb9 => KeyCode::Kp9,
        0xffae => KeyCode::KpDecimal,
        0xffaf => KeyCode::KpDivide,
        0xffaa => KeyCode::KpMultiply,
        0xffad => KeyCode::KpSubtract,
        0xffab => KeyCode::KpAdd,
        0xff8d => KeyCode::KpEnter,
        0xffbd => KeyCode::KpEqual,
        0xffe1 => KeyCode::LeftShift,
        0xffe3 => KeyCode::LeftControl,
        0xffe9 => KeyCode::LeftAlt,
        0xffeb => KeyCode::LeftSuper,
        0xffe2 => KeyCode::RightShift,
        0xffe4 => KeyCode::RightControl,
        0xffea => KeyCode::RightAlt,
        0xffec => KeyCode::RightSuper,
        0xff67 => KeyCode::Menu,
        _ => KeyCode::Unknown,
    }
}

pub fn u16_to_mousecode(key: u16) -> MouseButton {
    match key {
        0 => MouseButton::Left,
        1 => MouseButton::Middle,
        2 => MouseButton::Right,
        _ => MouseButton::Unknown,
    }
}