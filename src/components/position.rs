use specs::{VecStorage, Component};

#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}
