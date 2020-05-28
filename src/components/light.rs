use specs::{VecStorage, Component};

pub struct Light {
    pub color: glm::Vec3,
    pub strength: f32,
}

impl Component for Light {
    type Storage = VecStorage<Self>;
}
