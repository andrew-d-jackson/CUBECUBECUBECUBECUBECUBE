use specs::{VecStorage, Component};

pub struct RotateRandomly { }

impl Component for RotateRandomly {
    type Storage = VecStorage<Self>;
}
