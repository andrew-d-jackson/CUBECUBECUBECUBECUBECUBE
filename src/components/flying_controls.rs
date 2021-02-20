use specs::{Component, VecStorage};

pub struct FlyingControls {}

impl Component for FlyingControls {
    type Storage = VecStorage<Self>;
}
