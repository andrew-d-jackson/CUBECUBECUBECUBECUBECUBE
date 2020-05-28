use std::sync::{Mutex, Arc};
use specs::{VecStorage, Component};
use glium::{IndexBuffer};
use glium::vertex::{VertexBufferAny};

pub struct Model {
    pub vertex_buffer: Arc<Mutex<VertexBufferAny>>,
    pub index_buffer: Arc<Mutex<IndexBuffer<u32>>>,
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}

impl Component for Model {
    type Storage = VecStorage<Self>;
}
