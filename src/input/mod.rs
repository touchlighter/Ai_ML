use std::collections::HashSet;
use winit::event::{WindowEvent, KeyEvent, MouseButton, ElementState};
use winit::keyboard::{KeyCode, PhysicalKey};

/// Input manager for handling keyboard and mouse input
pub struct InputManager {
    // Keyboard state
    pressed_keys: HashSet<KeyCode>,
    just_pressed_keys: HashSet<KeyCode>,
    just_released_keys: HashSet<KeyCode>,
    
    // Mouse state
    mouse_position: (f64, f64),
    mouse_delta: (f64, f64),
    pressed_mouse_buttons: HashSet<MouseButton>,
    just_pressed_mouse_buttons: HashSet<MouseButton>,
    just_released_mouse_buttons: HashSet<MouseButton>,
    
    // Mouse capture
    mouse_captured: bool,
    last_mouse_position: Option<(f64, f64)>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            just_pressed_keys: HashSet::new(),
            just_released_keys: HashSet::new(),
            mouse_position: (0.0, 0.0),
            mouse_delta: (0.0, 0.0),
            pressed_mouse_buttons: HashSet::new(),
            just_pressed_mouse_buttons: HashSet::new(),
            just_released_mouse_buttons: HashSet::new(),
            mouse_captured: false,
            last_mouse_position: None,
        }
    }

    /// Handle window events
    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                self.handle_keyboard_input(event);
            },
            WindowEvent::MouseInput { state, button, .. } => {
                self.handle_mouse_input(*state, *button);
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.handle_mouse_movement(position.x, position.y);
            },
            _ => {}
        }
    }

    fn handle_keyboard_input(&mut self, event: &KeyEvent) {
        if let PhysicalKey::Code(keycode) = event.physical_key {
            match event.state {
                ElementState::Pressed => {
                    if !self.pressed_keys.contains(&keycode) {
                        self.just_pressed_keys.insert(keycode);
                    }
                    self.pressed_keys.insert(keycode);
                },
                ElementState::Released => {
                    self.pressed_keys.remove(&keycode);
                    self.just_released_keys.insert(keycode);
                }
            }
        }
    }

    fn handle_mouse_input(&mut self, state: ElementState, button: MouseButton) {
        match state {
            ElementState::Pressed => {
                if !self.pressed_mouse_buttons.contains(&button) {
                    self.just_pressed_mouse_buttons.insert(button);
                }
                self.pressed_mouse_buttons.insert(button);
            },
            ElementState::Released => {
                self.pressed_mouse_buttons.remove(&button);
                self.just_released_mouse_buttons.insert(button);
            }
        }
    }

    fn handle_mouse_movement(&mut self, x: f64, y: f64) {
        if let Some((last_x, last_y)) = self.last_mouse_position {
            self.mouse_delta = (x - last_x, y - last_y);
        } else {
            self.mouse_delta = (0.0, 0.0);
        }
        
        self.mouse_position = (x, y);
        self.last_mouse_position = Some((x, y));
    }

    /// Update input state (call once per frame)
    pub fn update(&mut self) {
        // Clear just pressed/released states
        self.just_pressed_keys.clear();
        self.just_released_keys.clear();
        self.just_pressed_mouse_buttons.clear();
        self.just_released_mouse_buttons.clear();
        
        // Reset mouse delta if not captured
        if !self.mouse_captured {
            self.mouse_delta = (0.0, 0.0);
        }
    }

    // Keyboard queries
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed_keys.contains(&key)
    }

    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        self.just_released_keys.contains(&key)
    }

    // Mouse queries
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed_mouse_buttons.contains(&button)
    }

    pub fn is_mouse_button_just_pressed(&self, button: MouseButton) -> bool {
        self.just_pressed_mouse_buttons.contains(&button)
    }

    pub fn is_mouse_button_just_released(&self, button: MouseButton) -> bool {
        self.just_released_mouse_buttons.contains(&button)
    }

    pub fn mouse_position(&self) -> (f64, f64) {
        self.mouse_position
    }

    pub fn mouse_delta(&self) -> (f64, f64) {
        self.mouse_delta
    }

    // Mouse capture
    pub fn set_mouse_captured(&mut self, captured: bool) {
        self.mouse_captured = captured;
        if captured {
            self.mouse_delta = (0.0, 0.0);
        }
    }

    pub fn is_mouse_captured(&self) -> bool {
        self.mouse_captured
    }

    // Common game input queries
    pub fn move_forward(&self) -> bool {
        self.is_key_pressed(KeyCode::KeyW)
    }

    pub fn move_backward(&self) -> bool {
        self.is_key_pressed(KeyCode::KeyS)
    }

    pub fn move_left(&self) -> bool {
        self.is_key_pressed(KeyCode::KeyA)
    }

    pub fn move_right(&self) -> bool {
        self.is_key_pressed(KeyCode::KeyD)
    }

    pub fn jump(&self) -> bool {
        self.is_key_pressed(KeyCode::Space)
    }

    pub fn sneak(&self) -> bool {
        self.is_key_pressed(KeyCode::ShiftLeft)
    }

    pub fn sprint(&self) -> bool {
        self.is_key_pressed(KeyCode::ControlLeft)
    }

    pub fn break_block(&self) -> bool {
        self.is_mouse_button_pressed(MouseButton::Left)
    }

    pub fn place_block(&self) -> bool {
        self.is_mouse_button_pressed(MouseButton::Right)
    }

    pub fn open_inventory(&self) -> bool {
        self.is_key_just_pressed(KeyCode::KeyE)
    }

    pub fn toggle_debug(&self) -> bool {
        self.is_key_just_pressed(KeyCode::F3)
    }

    pub fn escape(&self) -> bool {
        self.is_key_just_pressed(KeyCode::Escape)
    }

    pub fn enter(&self) -> bool {
        self.is_key_just_pressed(KeyCode::Enter)
    }

    // Hotbar selection (1-9 keys)
    pub fn get_hotbar_selection(&self) -> Option<usize> {
        for i in 1..=9 {
            let keycode = match i {
                1 => KeyCode::Digit1,
                2 => KeyCode::Digit2,
                3 => KeyCode::Digit3,
                4 => KeyCode::Digit4,
                5 => KeyCode::Digit5,
                6 => KeyCode::Digit6,
                7 => KeyCode::Digit7,
                8 => KeyCode::Digit8,
                9 => KeyCode::Digit9,
                _ => continue,
            };

            if self.is_key_just_pressed(keycode) {
                return Some(i - 1); // Return 0-based index
            }
        }
        None
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}