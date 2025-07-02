// Physics system placeholder
// TODO: Implement proper physics with collision detection

use glam::Vec3;

pub struct Physics {
    gravity: f32,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            gravity: 9.81,
        }
    }

    pub fn apply_gravity(&self, velocity: &mut Vec3, delta_time: f32) {
        velocity.y -= self.gravity * delta_time;
    }
}

impl Default for Physics {
    fn default() -> Self {
        Self::new()
    }
}