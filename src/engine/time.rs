use std::time::{Duration, Instant};

/// Manages game timing with support for fixed timestep and delta time
pub struct TimeManager {
    last_update: Instant,
    delta_time: f32,
    total_time: f32,
    fixed_timestep: f32,
    accumulator: f32,
    frame_count: u64,
    fps_timer: Instant,
    current_fps: u32,
}

impl TimeManager {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            last_update: now,
            delta_time: 0.0,
            total_time: 0.0,
            fixed_timestep: 1.0 / 60.0, // 60 FPS target
            accumulator: 0.0,
            frame_count: 0,
            fps_timer: now,
            current_fps: 0,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        // Cap frame time to prevent spiral of death
        self.delta_time = frame_time.min(0.25);
        self.total_time += self.delta_time;
        self.accumulator += self.delta_time;

        // Update FPS counter
        self.frame_count += 1;
        if now.duration_since(self.fps_timer) >= Duration::from_secs(1) {
            self.current_fps = self.frame_count as u32;
            self.frame_count = 0;
            self.fps_timer = now;
        }
    }

    /// Get the delta time for the current frame
    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    /// Get the total elapsed time
    pub fn total_time(&self) -> f32 {
        self.total_time
    }

    /// Get the fixed timestep value
    pub fn fixed_timestep(&self) -> f32 {
        self.fixed_timestep
    }

    /// Check if we should run a fixed timestep update
    pub fn should_fixed_update(&mut self) -> bool {
        if self.accumulator >= self.fixed_timestep {
            self.accumulator -= self.fixed_timestep;
            true
        } else {
            false
        }
    }

    /// Get the current FPS
    pub fn fps(&self) -> u32 {
        self.current_fps
    }

    /// Get the interpolation factor for rendering between fixed updates
    pub fn interpolation_factor(&self) -> f32 {
        self.accumulator / self.fixed_timestep
    }
}