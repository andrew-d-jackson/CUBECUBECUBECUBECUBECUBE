use specs::{Component, VecStorage};

pub struct RotateRandomly {}

impl Component for RotateRandomly {
    type Storage = VecStorage<Self>;
}
