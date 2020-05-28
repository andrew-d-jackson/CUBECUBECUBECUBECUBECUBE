use specs::{VecStorage, Component};
use glm::{Quat};

#[derive(Debug, Clone, Copy, Default)]
pub struct Rotation {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

impl Component for Rotation {
    type Storage = VecStorage<Self>;
}

impl Rotation {
    pub fn new() -> Rotation {
        let forward = glm::vec3(0.0f32, 0.0, 1.0);
        let up = glm::vec3(0.0f32, 1.0, 0.0);
        let look = glm::quat_look_at(&forward, &up);
        let angles = glm::quat_euler_angles(&look);
        Rotation {
            roll: angles.x,
            pitch: angles.y,
            yaw: angles.z,
        }
    }
    
    pub fn from_quaternion(quat: Quat) -> Rotation {
        let angles = glm::quat_euler_angles(&quat);
        Rotation {
            roll: angles.x,
            pitch: angles.y,
            yaw: angles.z,
        }
    }

    pub fn to_quaternion(&self) -> Quat {
        
        let yaw = glm::quat_angle_axis( self.yaw, &glm::vec3(0.0f32, 1.0, 0.0));
        let pitch = glm::quat_angle_axis(self.pitch, &glm::vec3(1.0f32, 0.0, 0.0));
        let roll = glm::quat_angle_axis(self.roll, &glm::vec3(0.0f32, 0.0, 1.0));
        yaw * pitch


       /* let euler = glm::vec3(self.roll, self.yaw, self.pitch);
        let c = glm::cos(&(euler * 0.5f32));
        let s = glm::sin(&(euler * 0.5f32));
        let w = c.x * c.y * c.z + s.x * s.y * s.z;
        let x = s.x * c.y * c.z - c.x * s.y * s.z;
        let y = c.x * s.y * c.z + s.x * c.y * s.z;
        let z = c.x * c.y * s.z - s.x * s.y * c.z;    
        Quat::new(w, x, y, z)*/
    }
}