use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use glam::Vec3;

mod chunk;
mod block;
mod generation;
mod lighting;

pub use chunk::{Chunk, ChunkCoordinate, CHUNK_SIZE, CHUNK_HEIGHT};
pub use block::BlockType;
pub use generation::WorldGenerator;

/// Main world manager that handles chunks, blocks, and world generation
pub struct World {
    chunks: HashMap<ChunkCoordinate, Chunk>,
    generator: WorldGenerator,
    seed: u64,
    spawn_point: Vec3,
    
    // Chunk loading/unloading
    loaded_chunks: Vec<ChunkCoordinate>,
    render_distance: i32,
}

impl World {
    pub fn new() -> Self {
        let seed = 12345; // TODO: Make configurable
        let generator = WorldGenerator::new(seed);
        
        Self {
            chunks: HashMap::new(),
            generator,
            seed,
            spawn_point: Vec3::new(0.0, 100.0, 0.0),
            loaded_chunks: Vec::new(),
            render_distance: 8, // 8 chunk radius
        }
    }

    pub fn with_seed(seed: u64) -> Self {
        let generator = WorldGenerator::new(seed);
        
        Self {
            chunks: HashMap::new(),
            generator,
            seed,
            spawn_point: Vec3::new(0.0, 100.0, 0.0),
            loaded_chunks: Vec::new(),
            render_distance: 8,
        }
    }

    pub fn update(&mut self, _delta_time: f32) {
        // TODO: Implement world tick updates (water flow, plant growth, etc.)
    }

    /// Load chunks around a player position
    pub fn load_chunks_around(&mut self, player_pos: Vec3) {
        let player_chunk_x = (player_pos.x / CHUNK_SIZE as f32).floor() as i32;
        let player_chunk_z = (player_pos.z / CHUNK_SIZE as f32).floor() as i32;

        let mut chunks_to_load = Vec::new();
        let mut chunks_to_unload = Vec::new();

        // Find chunks that should be loaded
        for x in (player_chunk_x - self.render_distance)..=(player_chunk_x + self.render_distance) {
            for z in (player_chunk_z - self.render_distance)..=(player_chunk_z + self.render_distance) {
                let chunk_coord = ChunkCoordinate { x, z };
                let distance = ((x - player_chunk_x).pow(2) + (z - player_chunk_z).pow(2)) as f32;
                
                if distance <= (self.render_distance as f32).powi(2) {
                    if !self.chunks.contains_key(&chunk_coord) {
                        chunks_to_load.push(chunk_coord);
                    }
                }
            }
        }

        // Find chunks that should be unloaded
        for &chunk_coord in self.chunks.keys() {
            let distance = ((chunk_coord.x - player_chunk_x).pow(2) + 
                           (chunk_coord.z - player_chunk_z).pow(2)) as f32;
            
            if distance > ((self.render_distance + 2) as f32).powi(2) {
                chunks_to_unload.push(chunk_coord);
            }
        }

        // Load new chunks
        for chunk_coord in chunks_to_load {
            self.load_chunk(chunk_coord);
        }

        // Unload distant chunks
        for chunk_coord in chunks_to_unload {
            self.unload_chunk(chunk_coord);
        }
    }

    fn load_chunk(&mut self, coord: ChunkCoordinate) {
        if !self.chunks.contains_key(&coord) {
            let chunk = self.generator.generate_chunk(coord);
            self.chunks.insert(coord, chunk);
            self.loaded_chunks.push(coord);
        }
    }

    fn unload_chunk(&mut self, coord: ChunkCoordinate) {
        // TODO: Save chunk data before unloading
        self.chunks.remove(&coord);
        self.loaded_chunks.retain(|&c| c != coord);
    }

    pub fn get_chunk(&self, coord: ChunkCoordinate) -> Option<&Chunk> {
        self.chunks.get(&coord)
    }

    pub fn get_chunk_mut(&mut self, coord: ChunkCoordinate) -> Option<&mut Chunk> {
        self.chunks.get_mut(&coord)
    }

    pub fn is_chunk_loaded(&self, coord: ChunkCoordinate) -> bool {
        self.chunks.contains_key(&coord)
    }

    pub fn get_block_at(&self, x: i32, y: i32, z: i32) -> Option<BlockType> {
        if y < 0 || y >= CHUNK_HEIGHT as i32 {
            return None;
        }

        let chunk_x = x.div_euclid(CHUNK_SIZE as i32);
        let chunk_z = z.div_euclid(CHUNK_SIZE as i32);
        let chunk_coord = ChunkCoordinate { x: chunk_x, z: chunk_z };

        if let Some(chunk) = self.get_chunk(chunk_coord) {
            let local_x = x.rem_euclid(CHUNK_SIZE as i32) as usize;
            let local_z = z.rem_euclid(CHUNK_SIZE as i32) as usize;
            Some(chunk.get_block(local_x, y as usize, local_z))
        } else {
            None
        }
    }

    pub fn set_block_at(&mut self, x: i32, y: i32, z: i32, block: BlockType) -> bool {
        if y < 0 || y >= CHUNK_HEIGHT as i32 {
            return false;
        }

        let chunk_x = x.div_euclid(CHUNK_SIZE as i32);
        let chunk_z = z.div_euclid(CHUNK_SIZE as i32);
        let chunk_coord = ChunkCoordinate { x: chunk_x, z: chunk_z };

        if let Some(chunk) = self.get_chunk_mut(chunk_coord) {
            let local_x = x.rem_euclid(CHUNK_SIZE as i32) as usize;
            let local_z = z.rem_euclid(CHUNK_SIZE as i32) as usize;
            chunk.set_block(local_x, y as usize, local_z, block);
            true
        } else {
            false
        }
    }

    /// Cast a ray for block interaction
    pub fn raycast(&self, ray: &crate::rendering::camera::Ray) -> Option<RaycastHit> {
        let mut t = 0.0;
        let step_size = 0.1;

        while t < ray.max_distance {
            let point = ray.point_at(t);
            let block_x = point.x.floor() as i32;
            let block_y = point.y.floor() as i32;
            let block_z = point.z.floor() as i32;

            if let Some(block) = self.get_block_at(block_x, block_y, block_z) {
                if block != BlockType::Air {
                    return Some(RaycastHit {
                        position: Vec3::new(block_x as f32, block_y as f32, block_z as f32),
                        distance: t,
                        block_type: block,
                    });
                }
            }

            t += step_size;
        }

        None
    }

    pub fn spawn_point(&self) -> Vec3 {
        self.spawn_point
    }

    pub fn set_spawn_point(&mut self, point: Vec3) {
        self.spawn_point = point;
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn loaded_chunks(&self) -> &[ChunkCoordinate] {
        &self.loaded_chunks
    }

    pub fn set_render_distance(&mut self, distance: i32) {
        self.render_distance = distance.max(1).min(32);
    }

    pub fn render_distance(&self) -> i32 {
        self.render_distance
    }
}

/// Result of a raycast operation
#[derive(Debug, Clone)]
pub struct RaycastHit {
    pub position: Vec3,
    pub distance: f32,
    pub block_type: BlockType,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}