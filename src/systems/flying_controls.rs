use crate::resources::{Inputs, WindowInfo};
use crate::components::{Rotation, Position, FlyingControls};
use specs::{System, WriteStorage};
use specs::prelude::*;
use glium::glutin::event::VirtualKeyCode;

pub struct FlyingControlsSystem {}

impl<'a> System<'a> for FlyingControlsSystem {
    type SystemData = (Read<'a, Inputs>, Read<'a, WindowInfo>, WriteStorage<'a, Position>, WriteStorage<'a, Rotation>, ReadStorage<'a, FlyingControls>);

    fn run(&mut self, (inputs, window_info, mut position, mut rotation, flying_controls): Self::SystemData) {    

        let dt = window_info.delta_time;
        let rot_mul = 3.0f32;
        let mov_mul = 2.0f32;

        for (posistion, rotation, _) in (&mut position, &mut rotation, &flying_controls).join() {
            let forward_vector = glm::quat_rotate_vec3(
                 &rotation.to_quaternion(), &glm::vec3(0.0f32, 0.0, 1.0),
            );

            let right_vector = glm::quat_rotate_vec3(
                 &rotation.to_quaternion(), &glm::vec3(1.0f32, 0.0, 0.0),
            );

            if inputs.is_pressed(VirtualKeyCode::W) {
                posistion.x = posistion.x + (forward_vector.x * dt * mov_mul);
                posistion.y = posistion.y + (forward_vector.y * dt * mov_mul);
                posistion.z = posistion.z + (forward_vector.z * dt * mov_mul);
            }

            if inputs.is_pressed(VirtualKeyCode::S) {
                posistion.x = posistion.x - (forward_vector.x * dt * mov_mul);
                posistion.y = posistion.y - (forward_vector.y * dt * mov_mul);
                posistion.z = posistion.z - (forward_vector.z * dt * mov_mul);
            }
            
            if inputs.is_pressed(VirtualKeyCode::D) {
                posistion.x = posistion.x + (right_vector.x * dt * mov_mul);
                posistion.y = posistion.y + (right_vector.y * dt * mov_mul);
                posistion.z = posistion.z + (right_vector.z * dt * mov_mul);
            }

            if inputs.is_pressed(VirtualKeyCode::A) {
                posistion.x = posistion.x - (right_vector.x * dt * mov_mul);
                posistion.y = posistion.y - (right_vector.y * dt * mov_mul);
                posistion.z = posistion.z - (right_vector.z * dt * mov_mul);
            }

            rotation.yaw = rotation.yaw + (inputs.mouse_movement_x * dt * rot_mul);
            rotation.pitch = (rotation.pitch - (inputs.mouse_movement_y * dt * rot_mul)).max(-1.5).min(1.5);
        }
    }
}
