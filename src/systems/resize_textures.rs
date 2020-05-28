use crate::resources::{WindowInfo, WritableTextures};
use specs::{System};
use specs::prelude::*;
use glium::glutin::event::VirtualKeyCode;

pub struct ResizeTexturesSystem {}

impl<'a> System<'a> for ResizeTexturesSystem {
    type SystemData = (Read<'a, WindowInfo>, Write<'a, WritableTextures>);

    fn run(&mut self, (window_info, mut writeable_textures): Self::SystemData) {    
        let display_mutex = window_info.display.clone().unwrap();
        let display = display_mutex.lock().unwrap();

        if window_info.resized {
            writeable_textures.resize_all(&*display);
        }
    }
}
