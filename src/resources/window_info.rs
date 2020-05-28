use std::sync::{Arc, Mutex};
use std::option::Option;
use glium::Display;

#[derive(Clone, Default)]
pub struct WindowInfo {
    pub display: Option<Arc<Mutex<Display>>>,
    pub width: u32,
    pub height: u32,
    pub delta_time: f32,
    pub resized: bool,
}

unsafe impl Send for WindowInfo {}
unsafe impl Sync for WindowInfo {}
