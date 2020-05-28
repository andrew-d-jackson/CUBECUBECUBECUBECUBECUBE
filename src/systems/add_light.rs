use crate::resources::{Shaders, WindowInfo, Inputs};
use crate::components::{Model, Position, FlyingControls, Rotation, Light};
use specs::{Read, System, ReadStorage, Entities};
use specs::prelude::*;
use glium::DrawParameters;
use glium::Depth;
use glium::DepthTest;
use glium::uniform;
use glium::Surface;
use glium::index::PrimitiveType;
use crate::quad::*;
use glium::{VertexBuffer, IndexBuffer};
use glium::glutin::event::VirtualKeyCode;
use specs::prelude::*;
use rand::Rng;

pub struct AddLightSystem {}

impl<'a> System<'a> for AddLightSystem {
    type SystemData = (Entities<'a>, Read<'a, Inputs>, WriteStorage<'a, Position>, WriteStorage<'a, Rotation>, WriteStorage<'a, Light>, ReadStorage<'a, FlyingControls>);

    fn run(&mut self, (mut entities, inputs, mut position, mut rotation,mut  light, flying_controls): Self::SystemData) {    
        if inputs.was_pressed(VirtualKeyCode::L) {
            let mut camera_direction = Rotation::default();
            let mut camera_position = Position::default();
            for (position, rotation, flying_controls) in (&position, &rotation, &flying_controls).join() {
                camera_position = position.clone();
                camera_direction = rotation.clone();
            }

            let mut rand = rand::thread_rng();
            let color = glm::vec3(rand.gen_range(0.0f32, 1.0), rand.gen_range(0.0, 1.0), rand.gen_range(0.0, 1.0));
            let strength = rand.gen_range(0.0f32, 1.0);

            entities.build_entity()
                .with(camera_direction.clone(), &mut rotation)
                .with(camera_position.clone(), &mut position)
                .with(Light { strength, color }, &mut light)
                .build();
        }
    }
}