// Utility functions and helpers

use glam::Vec3;

/// Math utilities
pub mod math {
    use super::*;
    
    pub fn distance_squared(a: Vec3, b: Vec3) -> f32 {
        (a - b).length_squared()
    }

    pub fn distance(a: Vec3, b: Vec3) -> f32 {
        (a - b).length()
    }

    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + t * (b - a)
    }

    pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
        value.max(min).min(max)
    }
}

/// Performance measurement utilities
pub mod perf {
    use std::time::{Duration, Instant};

    pub struct Timer {
        start: Instant,
    }

    impl Timer {
        pub fn new() -> Self {
            Self {
                start: Instant::now(),
            }
        }

        pub fn elapsed(&self) -> Duration {
            self.start.elapsed()
        }

        pub fn elapsed_ms(&self) -> f64 {
            self.elapsed().as_secs_f64() * 1000.0
        }

        pub fn reset(&mut self) {
            self.start = Instant::now();
        }
    }

    impl Default for Timer {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// String utilities
pub mod string {
    pub fn format_duration(duration: std::time::Duration) -> String {
        let total_seconds = duration.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        if hours > 0 {
            format!("{}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{}:{:02}", minutes, seconds)
        }
    }

    pub fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Color utilities
pub mod color {
    pub fn hex_to_rgb(hex: u32) -> (f32, f32, f32) {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;
        (r, g, b)
    }

    pub fn rgb_to_hex(r: f32, g: f32, b: f32) -> u32 {
        let r = (r * 255.0) as u32 & 0xFF;
        let g = (g * 255.0) as u32 & 0xFF;
        let b = (b * 255.0) as u32 & 0xFF;
        (r << 16) | (g << 8) | b
    }
}