use std::collections::VecDeque;
use crate::world::{Chunk, ChunkCoordinate, BlockType, CHUNK_SIZE, CHUNK_HEIGHT};

/// Lighting engine for calculating light propagation
pub struct LightingEngine {
    light_queue: VecDeque<LightNode>,
}

#[derive(Debug, Clone)]
struct LightNode {
    x: usize,
    y: usize,
    z: usize,
    light_level: u8,
}

impl LightingEngine {
    pub fn new() -> Self {
        Self {
            light_queue: VecDeque::new(),
        }
    }

    /// Calculate lighting for a single chunk
    pub fn calculate_chunk_lighting(&mut self, chunk: &mut Chunk) {
        // First pass: Sky lighting (from top down)
        self.calculate_sky_lighting(chunk);
        
        // Second pass: Block lighting (from light sources)
        self.calculate_block_lighting(chunk);
        
        // Third pass: Propagate lighting
        self.propagate_lighting(chunk);
    }

    /// Calculate sky lighting for the chunk
    fn calculate_sky_lighting(&mut self, chunk: &mut Chunk) {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let mut sky_light = 15; // Start with full sunlight
                
                // Propagate from top to bottom
                for y in (0..CHUNK_HEIGHT).rev() {
                    let block = chunk.get_block(x, y, z);
                    
                    // Reduce light if block is not transparent
                    if !block.is_transparent() {
                        sky_light = 0;
                    }
                    
                    chunk.set_sky_light(x, y, z, sky_light);
                    
                    // Add to light queue for propagation
                    if sky_light > 0 {
                        self.light_queue.push_back(LightNode {
                            x, y, z,
                            light_level: sky_light,
                        });
                    }
                }
            }
        }
    }

    /// Calculate block lighting from light-emitting blocks
    fn calculate_block_lighting(&mut self, chunk: &mut Chunk) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_SIZE {
                    let block = chunk.get_block(x, y, z);
                    let light_level = block.light_level();
                    
                    if light_level > 0 {
                        chunk.set_block_light(x, y, z, light_level);
                        
                        // Add to propagation queue
                        self.light_queue.push_back(LightNode {
                            x, y, z,
                            light_level,
                        });
                    }
                }
            }
        }
    }

    /// Propagate lighting throughout the chunk
    fn propagate_lighting(&mut self, chunk: &mut Chunk) {
        while let Some(node) = self.light_queue.pop_front() {
            if node.light_level <= 1 {
                continue;
            }

            let new_light_level = node.light_level.saturating_sub(1);
            
            // Check all 6 adjacent positions
            let neighbors = [
                (node.x.wrapping_add(1), node.y, node.z),
                (node.x.wrapping_sub(1), node.y, node.z),
                (node.x, node.y.wrapping_add(1), node.z),
                (node.x, node.y.wrapping_sub(1), node.z),
                (node.x, node.y, node.z.wrapping_add(1)),
                (node.x, node.y, node.z.wrapping_sub(1)),
            ];

            for (nx, ny, nz) in neighbors {
                if nx < CHUNK_SIZE && ny < CHUNK_HEIGHT && nz < CHUNK_SIZE {
                    let neighbor_block = chunk.get_block(nx, ny, nz);
                    
                    // Only propagate through transparent blocks
                    if neighbor_block.is_transparent() {
                        let current_light = chunk.get_block_light(nx, ny, nz);
                        
                        if new_light_level > current_light {
                            chunk.set_block_light(nx, ny, nz, new_light_level);
                            
                            self.light_queue.push_back(LightNode {
                                x: nx,
                                y: ny,
                                z: nz,
                                light_level: new_light_level,
                            });
                        }
                    }
                }
            }
        }
    }

    /// Update lighting when a block is placed
    pub fn update_lighting_add_block(&mut self, chunk: &mut Chunk, x: usize, y: usize, z: usize) {
        // Remove light from this position
        chunk.set_sky_light(x, y, z, 0);
        chunk.set_block_light(x, y, z, 0);
        
        // Recalculate lighting in the affected area
        self.recalculate_area_lighting(chunk, x, y, z, 2);
    }

    /// Update lighting when a block is removed
    pub fn update_lighting_remove_block(&mut self, chunk: &mut Chunk, x: usize, y: usize, z: usize) {
        let block = chunk.get_block(x, y, z);
        
        // If it was a light source, remove its contribution
        if block.light_level() > 0 {
            self.remove_light_source(chunk, x, y, z, block.light_level());
        }
        
        // Recalculate sky lighting for this column
        self.recalculate_sky_column(chunk, x, z);
        
        // Propagate light into the newly empty space
        self.propagate_light_to_position(chunk, x, y, z);
    }

    /// Recalculate lighting in a specific area
    fn recalculate_area_lighting(&mut self, chunk: &mut Chunk, center_x: usize, center_y: usize, center_z: usize, radius: usize) {
        let start_x = center_x.saturating_sub(radius);
        let start_y = center_y.saturating_sub(radius);
        let start_z = center_z.saturating_sub(radius);
        
        let end_x = (center_x + radius).min(CHUNK_SIZE - 1);
        let end_y = (center_y + radius).min(CHUNK_HEIGHT - 1);
        let end_z = (center_z + radius).min(CHUNK_SIZE - 1);

        for x in start_x..=end_x {
            for y in start_y..=end_y {
                for z in start_z..=end_z {
                    let block = chunk.get_block(x, y, z);
                    
                    // Reset block light
                    let light_level = block.light_level();
                    chunk.set_block_light(x, y, z, light_level);
                    
                    if light_level > 0 {
                        self.light_queue.push_back(LightNode {
                            x, y, z,
                            light_level,
                        });
                    }
                }
            }
        }
        
        // Propagate the changes
        self.propagate_lighting(chunk);
    }

    /// Recalculate sky lighting for a column
    fn recalculate_sky_column(&mut self, chunk: &mut Chunk, x: usize, z: usize) {
        let mut sky_light = 15;
        
        for y in (0..CHUNK_HEIGHT).rev() {
            let block = chunk.get_block(x, y, z);
            
            if !block.is_transparent() {
                sky_light = 0;
            }
            
            chunk.set_sky_light(x, y, z, sky_light);
        }
    }

    /// Remove light from a light source
    fn remove_light_source(&mut self, chunk: &mut Chunk, x: usize, y: usize, z: usize, light_level: u8) {
        // Simple approach: just recalculate the area
        // TODO: Implement proper light removal algorithm
        self.recalculate_area_lighting(chunk, x, y, z, light_level as usize);
    }

    /// Propagate light to a specific position
    fn propagate_light_to_position(&mut self, chunk: &mut Chunk, x: usize, y: usize, z: usize) {
        let mut max_light = 0u8;
        
        // Check all neighbors and take the maximum light level
        let neighbors = [
            (x.wrapping_add(1), y, z),
            (x.wrapping_sub(1), y, z),
            (x, y.wrapping_add(1), z),
            (x, y.wrapping_sub(1), z),
            (x, y, z.wrapping_add(1)),
            (x, y, z.wrapping_sub(1)),
        ];

        for (nx, ny, nz) in neighbors {
            if nx < CHUNK_SIZE && ny < CHUNK_HEIGHT && nz < CHUNK_SIZE {
                let neighbor_light = chunk.get_block_light(nx, ny, nz);
                if neighbor_light > max_light {
                    max_light = neighbor_light;
                }
            }
        }
        
        if max_light > 1 {
            let new_light = max_light - 1;
            chunk.set_block_light(x, y, z, new_light);
            
            self.light_queue.push_back(LightNode {
                x, y, z,
                light_level: new_light,
            });
            
            self.propagate_lighting(chunk);
        }
    }

    /// Calculate ambient occlusion for a vertex
    pub fn calculate_ambient_occlusion(&self, chunk: &Chunk, x: f32, y: f32, z: f32) -> f32 {
        // Simple ambient occlusion based on nearby blocks
        let block_x = x.floor() as i32;
        let block_y = y.floor() as i32;
        let block_z = z.floor() as i32;
        
        let mut occlusion = 0.0;
        let mut sample_count = 0;
        
        // Sample blocks in a small radius
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let check_x = block_x + dx;
                    let check_y = block_y + dy;
                    let check_z = block_z + dz;
                    
                    if check_x >= 0 && check_x < CHUNK_SIZE as i32 &&
                       check_y >= 0 && check_y < CHUNK_HEIGHT as i32 &&
                       check_z >= 0 && check_z < CHUNK_SIZE as i32 {
                        
                        let block = chunk.get_block(check_x as usize, check_y as usize, check_z as usize);
                        if !block.is_transparent() {
                            occlusion += 1.0;
                        }
                        sample_count += 1;
                    }
                }
            }
        }
        
        if sample_count > 0 {
            1.0 - (occlusion / sample_count as f32) * 0.3 // Max 30% darkening
        } else {
            1.0
        }
    }
}

impl Default for LightingEngine {
    fn default() -> Self {
        Self::new()
    }
}