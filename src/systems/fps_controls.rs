use crate::resources::{Inputs, WindowInfo};
use crate::components::{Position, FlyingControls};
use specs::{System, WriteStorage};
use specs::prelude::*;
use glium::glutin::event::VirtualKeyCode;
use crate::map::Color;

pub struct FPSSystem {
    pub map: Vec<Vec<Vec<Option<Color>>>>,
    pub velocity: glm::Vec3,
}

impl FPSSystem {
    pub fn is_point_blocked(&self, point: glm::Vec3) -> bool {
        let x: usize = point.x as usize;
        let y: usize = 512 - point.y as usize;
        let z: usize = point.z as usize;
        println!("{} {} {}", x, y, z);
        match self.map[x][z][y] {
            Some(_) => true,
            None => false,
        }
    }

    pub fn is_points_blocked(&self, point: glm::Vec3) -> bool {
        let mut points: Vec<glm::Vec3> = vec![];
        points.push(point);
        points.push(point + glm::vec3(0.0f32, -1.0, 0.0));
        points.push(point + glm::vec3(0.0f32, -2.0, 0.0));
        points.push(point + glm::vec3(0.0f32, -3.0, 0.0));
        points.iter().any(|p| self.is_point_blocked(*p))        
    }
}

impl<'a> System<'a> for FPSSystem {
    type SystemData = (Read<'a, Inputs>, Read<'a, WindowInfo>, WriteStorage<'a, Position>, ReadStorage<'a, FlyingControls>);

    fn run(&mut self, (inputs, window_info, mut position, flying_controls): Self::SystemData) {    

        let dt = window_info.delta_time;
        let rot_mul = 3.0f32;
        let mov_mul = 20.0f32;

        for (posistion, _) in (&mut position, &flying_controls).join() {
            let mut forward_vector = glm::quat_rotate_vec3(
                 &posistion.get_rot_as_quat(), &glm::vec3(0.0f32, 0.0, 1.0),
            );
            forward_vector.y = 0.0f32;
            forward_vector = glm::normalize(&forward_vector);

            let right_vector = glm::quat_rotate_vec3(
                 &posistion.get_rot_as_quat(), &glm::vec3(1.0f32, 0.0, 0.0),
            );

            self.velocity.x = 0.0;
            self.velocity.z = 0.0;

            if inputs.is_pressed(VirtualKeyCode::W) {
                self.velocity.x = self.velocity.x + (forward_vector.x * dt * mov_mul);
                self.velocity.z = self.velocity.z + (forward_vector.z * dt * mov_mul);
            }

            if inputs.is_pressed(VirtualKeyCode::S) {
                self.velocity.x = self.velocity.x - (forward_vector.x * dt * mov_mul);
                self.velocity.z = self.velocity.z - (forward_vector.z * dt * mov_mul);
            }
            
            if inputs.is_pressed(VirtualKeyCode::D) {
                self.velocity.x = self.velocity.x + (right_vector.x * dt * mov_mul);
                self.velocity.z = self.velocity.z + (right_vector.z * dt * mov_mul);
            }

            if inputs.is_pressed(VirtualKeyCode::A) {
                self.velocity.x = self.velocity.x - (right_vector.x * dt * mov_mul);
                self.velocity.z = self.velocity.z - (right_vector.z * dt * mov_mul);
            }


            if inputs.is_pressed(VirtualKeyCode::A) {
                self.velocity.x = self.velocity.x - (right_vector.x * dt * mov_mul);
                self.velocity.z = self.velocity.z - (right_vector.z * dt * mov_mul);
            }

            if inputs.was_pressed(VirtualKeyCode::Space) {
                self.velocity.y = self.velocity.y + (1.0);
            }


            self.velocity.y = self.velocity.y - (dt * 2.0);
            if self.velocity.y >= 0.99f32 {
                self.velocity.y = 0.99
            }
            if self.velocity.y <= -0.99f32 {
                self.velocity.y = -0.99
            }
            println!("{}" , self.velocity.y);
            let mut new_position = posistion.get_pos_vec();
            let mut test = new_position + glm::vec3(self.velocity.x, 0.0, 0.0);
            if !self.is_points_blocked(test) {
                new_position = test.clone();
            }
            test = new_position + glm::vec3(0.0, self.velocity.y, 0.0);
            if !self.is_points_blocked(test) {
                new_position = new_position + glm::vec3(0.0, self.velocity.y, 0.0);
            } else {
                println!("Stopped on Y");
                self.velocity.y = 0.0;
            }
            test = new_position + glm::vec3(0.0, 0.0, self.velocity.z);
            if !self.is_points_blocked(test) {
                new_position = test.clone();
            }

            posistion.x = new_position.x;
            posistion.y = new_position.y;
            posistion.z = new_position.z;

            posistion.yaw = posistion.yaw - (inputs.mouse_movement_x * dt * rot_mul);
            posistion.pitch = (posistion.pitch + (inputs.mouse_movement_y * dt * rot_mul)).max(-1.5).min(1.5);
        }
    }
}
