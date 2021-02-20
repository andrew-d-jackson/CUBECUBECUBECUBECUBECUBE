use crate::resources::{Inputs, Shaders, WindowInfo};
use glium::glutin::event::VirtualKeyCode;
use specs::prelude::*;
use specs::System;

pub struct ReloadShadersSystem {}

impl<'a> System<'a> for ReloadShadersSystem {
    type SystemData = (Read<'a, Inputs>, Read<'a, WindowInfo>, Write<'a, Shaders>);

    fn run(&mut self, (inputs, window_info, mut shaders): Self::SystemData) {
        if inputs.was_pressed(VirtualKeyCode::P) {
            let display_mutex = window_info.display.clone().unwrap();
            shaders.reload_all(&*display_mutex.lock().unwrap());
        }
    }
}
