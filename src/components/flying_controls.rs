use specs::{VecStorage, Component};

pub struct FlyingControls { }

impl Component for FlyingControls {
    type Storage = VecStorage<Self>;
}
