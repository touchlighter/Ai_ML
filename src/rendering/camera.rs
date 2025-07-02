use glam::{Mat4, Vec3};

/// 3D camera with perspective projection and FPS-style controls
pub struct Camera {
    position: Vec3,
    yaw: f32,
    pitch: f32,
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,
    
    // Movement speed
    move_speed: f32,
    mouse_sensitivity: f32,
    
    // Cached vectors
    front: Vec3,
    up: Vec3,
    right: Vec3,
    world_up: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, yaw: f32, pitch: f32, aspect: f32) -> Self {
        let mut camera = Self {
            position,
            yaw,
            pitch,
            fov: 70.0,
            aspect,
            near: 0.1,
            far: 1000.0,
            move_speed: 4.317, // Minecraft walking speed (blocks/second)
            mouse_sensitivity: 0.1,
            front: Vec3::ZERO,
            up: Vec3::ZERO,
            right: Vec3::ZERO,
            world_up: Vec3::Y,
        };
        camera.update_camera_vectors();
        camera
    }

    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = self.view_matrix();
        let proj = self.projection_matrix();
        proj * view
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.position + self.front, self.up)
    }

    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov.to_radians(), self.aspect, self.near, self.far)
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = self.move_speed * delta_time;
        
        match direction {
            CameraMovement::Forward => self.position += self.front * velocity,
            CameraMovement::Backward => self.position -= self.front * velocity,
            CameraMovement::Left => self.position -= self.right * velocity,
            CameraMovement::Right => self.position += self.right * velocity,
            CameraMovement::Up => self.position += self.world_up * velocity,
            CameraMovement::Down => self.position -= self.world_up * velocity,
        }
    }

    pub fn process_mouse_movement(&mut self, mut xoffset: f32, mut yoffset: f32, constrain_pitch: bool) {
        xoffset *= self.mouse_sensitivity;
        yoffset *= self.mouse_sensitivity;

        self.yaw += xoffset;
        self.pitch += yoffset;

        if constrain_pitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0;
            }
            if self.pitch < -89.0 {
                self.pitch = -89.0;
            }
        }

        self.update_camera_vectors();
    }

    pub fn process_mouse_scroll(&mut self, yoffset: f32) {
        self.fov -= yoffset;
        if self.fov < 1.0 {
            self.fov = 1.0;
        }
        if self.fov > 90.0 {
            self.fov = 90.0;
        }
    }

    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn front(&self) -> Vec3 {
        self.front
    }

    pub fn right(&self) -> Vec3 {
        self.right
    }

    pub fn up(&self) -> Vec3 {
        self.up
    }

    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn set_move_speed(&mut self, speed: f32) {
        self.move_speed = speed;
    }

    pub fn set_mouse_sensitivity(&mut self, sensitivity: f32) {
        self.mouse_sensitivity = sensitivity;
    }

    // Cast a ray from the camera for block interaction
    pub fn cast_ray(&self, max_distance: f32) -> Ray {
        Ray {
            origin: self.position,
            direction: self.front,
            max_distance,
        }
    }

    fn update_camera_vectors(&mut self) {
        // Calculate the new front vector
        let front = Vec3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.front = front.normalize();
        
        // Re-calculate the right and up vectors
        self.right = self.front.cross(self.world_up).normalize();
        self.up = self.right.cross(self.front).normalize();
    }
}

pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

/// Ray for raycasting (block interaction)
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub max_distance: f32,
}

impl Ray {
    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}