use crate::resources::{ActiveTexture, Inputs, WritableTextures};
use glium::glutin::event::VirtualKeyCode;
use specs::prelude::*;
use specs::System;

pub struct SwitchActiveTextureSystem {}

impl<'a> System<'a> for SwitchActiveTextureSystem {
    type SystemData = (
        Read<'a, Inputs>,
        Write<'a, ActiveTexture>,
        Read<'a, WritableTextures>,
    );

    fn run(&mut self, (inputs, mut active_texture, writable_textures): Self::SystemData) {
        let mut texture_list: Vec<String> = writable_textures
            .color_textures
            .keys()
            .map(|k| k.clone())
            .collect();
        texture_list.sort();
        let index = texture_list
            .iter()
            .position(|r| r.clone() == active_texture.active_texture.clone())
            .unwrap();
        let len = texture_list.len();

        if inputs.was_pressed(VirtualKeyCode::N) {
            if active_texture.depth {
                let mut new_index = index + 1;
                if new_index == len {
                    new_index = 0;
                }
                active_texture.active_texture = texture_list[new_index].clone();
                active_texture.depth = false;
            } else {
                active_texture.depth = true;
            }
        }

        if inputs.was_pressed(VirtualKeyCode::B) {
            if active_texture.depth {
                active_texture.depth = false;
            } else {
                let new_index;
                if index == 0 {
                    new_index = len;
                } else {
                    new_index = index - 1;
                }
                active_texture.active_texture = texture_list[new_index].clone();
                active_texture.depth = true;
            }
        }
    }
}
