use crate::components::{FlyingControls, Position};
use crate::resources::{Inputs, WindowInfo};
use glium::glutin::event::VirtualKeyCode;
use specs::prelude::*;
use specs::{System, WriteStorage};

pub struct FlyingControlsSystem {}

impl<'a> System<'a> for FlyingControlsSystem {
    type SystemData = (
        Read<'a, Inputs>,
        Read<'a, WindowInfo>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, FlyingControls>,
    );

    fn run(&mut self, (inputs, window_info, mut position, flying_controls): Self::SystemData) {
        let dt = window_info.delta_time;
        let rot_mul = 3.0f32;
        let mov_mul = 200.0f32;

        for (posistion, _) in (&mut position, &flying_controls).join() {
            let forward_vector =
                glm::quat_rotate_vec3(&posistion.get_rot_as_quat(), &glm::vec3(0.0f32, 0.0, 1.0));

            let right_vector =
                glm::quat_rotate_vec3(&posistion.get_rot_as_quat(), &glm::vec3(1.0f32, 0.0, 0.0));

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

            posistion.yaw = posistion.yaw - (inputs.mouse_movement_x * dt * rot_mul);
            posistion.pitch = (posistion.pitch + (inputs.mouse_movement_y * dt * rot_mul))
                .max(-1.5)
                .min(1.5);
        }
    }
}
