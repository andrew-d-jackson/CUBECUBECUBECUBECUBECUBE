use specs::{VecStorage, Component};

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,

    pub scaleX: f32,
    pub scaleY: f32,
    pub scaleZ: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Default for Position {
    fn default() -> Self {
        let forward = glm::vec3(0.0f32, 0.0, 1.0);
        let up = glm::vec3(0.0f32, 1.0, 0.0);
        let look = glm::quat_look_at(&forward, &up);
        let angles = glm::quat_euler_angles(&look);

        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            roll: angles.x,
            pitch: angles.y,
            yaw: angles.z,
            scaleX: 1.0,
            scaleY: 1.0,
            scaleZ: 1.0,
        }
    }
}

impl Position {
    pub fn new() -> Self {
        let forward = glm::vec3(0.0f32, 0.0, 1.0);
        let up = glm::vec3(0.0f32, 1.0, 0.0);
        let look = glm::quat_look_at(&forward, &up);
        let angles = glm::quat_euler_angles(&look);

        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            roll: angles.x,
            pitch: angles.y,
            yaw: angles.z,
            scaleX: 1.0,
            scaleY: 1.0,
            scaleZ: 1.0,
        }
    }
    
    pub fn new_pos(x: f32, y: f32, z: f32) -> Self {
        let forward = glm::vec3(0.0f32, 0.0, 1.0);
        let up = glm::vec3(0.0f32, 1.0, 0.0);
        let look = glm::quat_look_at(&forward, &up);
        let angles = glm::quat_euler_angles(&look);

        Self {
            x,
            y,
            z,
            roll: angles.x,
            pitch: angles.y,
            yaw: angles.z,
            scaleX: 1.0,
            scaleY: 1.0,
            scaleZ: 1.0,
        }
    }


    pub fn get_pos_vec(&self) -> glm::Vec3 {
        glm::vec3(self.x, self.y, self.z)
    }

    pub fn get_scale_vec(&self) -> glm::Vec3 {
        glm::vec3(self.scaleX, self.scaleY, self.scaleZ)
    }

    pub fn get_rot_as_quat(&self) -> glm::Quat {
        let yaw = glm::quat_angle_axis( self.yaw, &glm::vec3(0.0f32, 1.0, 0.0));
        let pitch = glm::quat_angle_axis(self.pitch, &glm::vec3(1.0f32, 0.0, 0.0));
        let roll = glm::quat_angle_axis(self.roll, &glm::vec3(0.0f32, 0.0, 1.0));
        yaw * pitch * roll
    }

    pub fn get_transform_matrix(&self) -> glm::TMat<f32, glm::U4, glm::U4> {
        let model_mat: glm::TMat<f32, glm::U4, glm::U4> = glm::identity();
        let translated = glm::translate(&model_mat, &glm::vec3(self.x, self.y, self.z));
        let scaled = glm::scale(&translated, &glm::vec3(self.scaleX, self.scaleY, self.scaleZ));
        scaled * glm::quat_to_mat4(&self.get_rot_as_quat())
    }

    pub fn scale(&mut self, scaleX: f32, scaleY: f32, scaleZ: f32) -> Self {
        self.scaleX = scaleX;
        self.scaleY = scaleY;
        self.scaleZ = scaleZ;
        *self
    }
    
    pub fn rotate(&mut self, yaw: f32, pitch: f32, roll: f32) -> Self {
        self.yaw = yaw;
        self.pitch = pitch;
        self.roll = roll;
        *self
    }
    
}