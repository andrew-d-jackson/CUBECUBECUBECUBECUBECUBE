use glium::glutin::event::{WindowEvent, VirtualKeyCode, ElementState, KeyboardInput};

#[derive(Default, Clone, Debug)]
pub struct KeyboardState {
    pub keys_down: Vec<VirtualKeyCode>,
    pub keys_pressed: Vec<VirtualKeyCode>,
    pub keys_released: Vec<VirtualKeyCode>,
}

impl KeyboardState {
    pub fn new() -> KeyboardState {
        KeyboardState {
            keys_down: vec![],
            keys_pressed: vec![],
            keys_released: vec![],
        }
    }

    pub fn is_pressed(&self, key: VirtualKeyCode) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn was_pressed(&self, key: VirtualKeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn was_released(&self, key: VirtualKeyCode) -> bool {
        self.keys_released.contains(&key)
    }

    pub fn reset_presses(&mut self) {
        self.keys_pressed = vec![];
        self.keys_released = vec![];
    }

    pub fn process_keyboard_input(&mut self, input: KeyboardInput) {
        if input.virtual_keycode.is_none() {
            return;
        }

        let key = input.virtual_keycode.unwrap();

        match input.state {
            ElementState::Pressed => {
                if !self.keys_down.contains(&key) {
                    self.keys_down.push(key);
                    self.keys_pressed.push(key);
                }
            },
            ElementState::Released => {
                self.keys_down = self.keys_down
                    .iter()
                    .filter(|k| **k != key)
                    .cloned()
                    .collect();

                self.keys_released.push(key);
            }
        }
    }
}

