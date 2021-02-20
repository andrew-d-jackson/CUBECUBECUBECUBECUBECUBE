use glium::glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

#[derive(Default, Clone, Debug)]
pub struct Inputs {
    pub keys_down: Vec<VirtualKeyCode>,
    pub keys_pressed: Vec<VirtualKeyCode>,
    pub keys_released: Vec<VirtualKeyCode>,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_movement_x: f32,
    pub mouse_movement_y: f32,
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            keys_down: vec![],
            keys_pressed: vec![],
            keys_released: vec![],
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_movement_x: 0.0,
            mouse_movement_y: 0.0,
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

    pub fn mouse_movement(&self) -> (f32, f32) {
        (self.mouse_movement_x, self.mouse_movement_y)
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
            }
            ElementState::Released => {
                self.keys_down = self
                    .keys_down
                    .iter()
                    .filter(|k| **k != key)
                    .cloned()
                    .collect();

                self.keys_released.push(key);
            }
        }
    }

    pub fn process_events(&mut self, events: &Vec<Event<()>>) {
        self.reset_presses();

        self.mouse_movement_x = 0.0;
        self.mouse_movement_y = 0.0;

        for event in events.iter() {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CursorMoved { position, .. } => {
                        self.mouse_movement_x = position.x as f32 - self.mouse_x;
                        self.mouse_movement_y = position.y as f32 - self.mouse_y;
                        self.mouse_x = position.x as f32;
                        self.mouse_y = position.y as f32;
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        self.process_keyboard_input(*input);
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }
}
