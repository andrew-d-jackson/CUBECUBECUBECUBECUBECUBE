use crate::components::{FlyingControls, Light, Position};
use crate::resources::Inputs;
use glium::glutin::event::VirtualKeyCode;
use rand::Rng;
use specs::{Entities, Join, Read, ReadStorage, System, WriteStorage};

pub struct AddLightSystem {}

impl<'a> System<'a> for AddLightSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Inputs>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Light>,
        ReadStorage<'a, FlyingControls>,
    );

    fn run(
        &mut self,
        (entities, inputs, mut position, mut light, flying_controls): Self::SystemData,
    ) {
        if inputs.was_pressed(VirtualKeyCode::L) {
            let mut camera_position = Position::default();
            for (position, _) in (&position, &flying_controls).join() {
                camera_position = position.clone();
            }

            let mut rand = rand::thread_rng();
            let color = glm::vec3(
                rand.gen_range(0.0f32, 1.0),
                rand.gen_range(0.0, 1.0),
                rand.gen_range(0.0, 1.0),
            );
            let strength = rand.gen_range(0.0f32, 1.0);

            entities
                .build_entity()
                .with(camera_position.clone(), &mut position)
                .with(Light { strength, color }, &mut light)
                .build();
        }
    }
}
