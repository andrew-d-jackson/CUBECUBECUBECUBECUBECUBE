use crate::components::{RotateRandomly, Position};
use rand::Rng;
use specs::{System, WriteStorage, ReadStorage};
use specs::Join;

pub struct RotateRandomlySystem {}

impl<'a> System<'a> for RotateRandomlySystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, RotateRandomly>);

    fn run(&mut self, (mut position, rotate_randomly): Self::SystemData) {  
        //let mut rand = rand::thread_rng();
        for (position, _) in (&mut position, &rotate_randomly).join() {
            position.rotate(
                position.yaw + 0.01f32,
                position.pitch + 0.01f32,
                position.roll + 0.01f32,
            );
        }
    }
}