use serde::{Deserialize, Serialize};
use crate::world::block::BlockType;

/// Size of a chunk in blocks (16x16 horizontal)
pub const CHUNK_SIZE: usize = 16;
/// Height of a chunk in blocks (256 blocks tall)
pub const CHUNK_HEIGHT: usize = 256;

/// Coordinate for identifying chunks in the world
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkCoordinate {
    pub x: i32,
    pub z: i32,
}

impl ChunkCoordinate {
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    /// Get world position of the chunk's origin (bottom-left corner)
    pub fn world_position(&self) -> (i32, i32) {
        (self.x * CHUNK_SIZE as i32, self.z * CHUNK_SIZE as i32)
    }

    /// Get neighboring chunk coordinates
    pub fn neighbors(&self) -> [ChunkCoordinate; 4] {
        [
            ChunkCoordinate::new(self.x + 1, self.z),     // East
            ChunkCoordinate::new(self.x - 1, self.z),     // West
            ChunkCoordinate::new(self.x, self.z + 1),     // North
            ChunkCoordinate::new(self.x, self.z - 1),     // South
        ]
    }

    /// Get all 8 surrounding chunks (including diagonals)
    pub fn surrounding(&self) -> [ChunkCoordinate; 8] {
        [
            ChunkCoordinate::new(self.x + 1, self.z),     // East
            ChunkCoordinate::new(self.x - 1, self.z),     // West
            ChunkCoordinate::new(self.x, self.z + 1),     // North
            ChunkCoordinate::new(self.x, self.z - 1),     // South
            ChunkCoordinate::new(self.x + 1, self.z + 1), // Northeast
            ChunkCoordinate::new(self.x + 1, self.z - 1), // Southeast
            ChunkCoordinate::new(self.x - 1, self.z + 1), // Northwest
            ChunkCoordinate::new(self.x - 1, self.z - 1), // Southwest
        ]
    }
}

/// A chunk represents a 16x16x256 section of the world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    /// Coordinate of this chunk
    pub coordinate: ChunkCoordinate,
    
    /// Block data stored as a 3D array [x][z][y]
    /// Using Vec<Vec<Vec<BlockType>>> for flexibility, though this could be optimized
    blocks: Vec<Vec<Vec<BlockType>>>,
    
    /// Highest non-air block at each (x, z) position for optimization
    height_map: Vec<Vec<usize>>,
    
    /// Whether this chunk has been modified since last save
    pub dirty: bool,
    
    /// Light levels for each block position
    /// Using u8 where: 
    /// - bits 0-3: block light (torch light, etc.)
    /// - bits 4-7: sky light (sunlight)
    light_levels: Vec<Vec<Vec<u8>>>,
}

impl Chunk {
    /// Create a new empty chunk filled with air
    pub fn new(coordinate: ChunkCoordinate) -> Self {
        let mut blocks = Vec::with_capacity(CHUNK_SIZE);
        let mut height_map = Vec::with_capacity(CHUNK_SIZE);
        let mut light_levels = Vec::with_capacity(CHUNK_SIZE);

        for _x in 0..CHUNK_SIZE {
            let mut x_blocks = Vec::with_capacity(CHUNK_SIZE);
            let mut x_heights = Vec::with_capacity(CHUNK_SIZE);
            let mut x_lights = Vec::with_capacity(CHUNK_SIZE);

            for _z in 0..CHUNK_SIZE {
                let mut z_blocks = Vec::with_capacity(CHUNK_HEIGHT);
                let mut z_lights = Vec::with_capacity(CHUNK_HEIGHT);

                for _y in 0..CHUNK_HEIGHT {
                    z_blocks.push(BlockType::Air);
                    z_lights.push(0xFF); // Full sky light initially
                }

                x_blocks.push(z_blocks);
                x_heights.push(0); // All air initially, so height is 0
                x_lights.push(z_lights);
            }

            blocks.push(x_blocks);
            height_map.push(x_heights);
            light_levels.push(x_lights);
        }

        Self {
            coordinate,
            blocks,
            height_map,
            dirty: false,
            light_levels,
        }
    }

    /// Get block at local chunk coordinates
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> BlockType {
        if x >= CHUNK_SIZE || y >= CHUNK_HEIGHT || z >= CHUNK_SIZE {
            return BlockType::Air;
        }
        self.blocks[x][z][y]
    }

    /// Set block at local chunk coordinates
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: BlockType) {
        if x >= CHUNK_SIZE || y >= CHUNK_HEIGHT || z >= CHUNK_SIZE {
            return;
        }

        let old_block = self.blocks[x][z][y];
        if old_block != block {
            self.blocks[x][z][y] = block;
            self.dirty = true;

            // Update height map
            self.update_height_at(x, z);
            
            // TODO: Update lighting
            self.update_lighting_at(x, y, z);
        }
    }

    /// Get the height of the highest non-air block at (x, z)
    pub fn get_height_at(&self, x: usize, z: usize) -> usize {
        if x >= CHUNK_SIZE || z >= CHUNK_SIZE {
            return 0;
        }
        self.height_map[x][z]
    }

    /// Update the height map for a specific column
    fn update_height_at(&mut self, x: usize, z: usize) {
        if x >= CHUNK_SIZE || z >= CHUNK_SIZE {
            return;
        }

        let mut height = 0;
        for y in (0..CHUNK_HEIGHT).rev() {
            if self.blocks[x][z][y] != BlockType::Air {
                height = y + 1;
                break;
            }
        }
        self.height_map[x][z] = height;
    }

    /// Update height map for the entire chunk
    pub fn update_height_map(&mut self) {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                self.update_height_at(x, z);
            }
        }
    }

    /// Get light level at a position
    pub fn get_light_level(&self, x: usize, y: usize, z: usize) -> u8 {
        if x >= CHUNK_SIZE || y >= CHUNK_HEIGHT || z >= CHUNK_SIZE {
            return 0;
        }
        self.light_levels[x][z][y]
    }

    /// Set light level at a position
    pub fn set_light_level(&mut self, x: usize, y: usize, z: usize, light: u8) {
        if x >= CHUNK_SIZE || y >= CHUNK_HEIGHT || z >= CHUNK_SIZE {
            return;
        }
        self.light_levels[x][z][y] = light;
    }

    /// Get sky light level (bits 4-7)
    pub fn get_sky_light(&self, x: usize, y: usize, z: usize) -> u8 {
        (self.get_light_level(x, y, z) >> 4) & 0x0F
    }

    /// Get block light level (bits 0-3)
    pub fn get_block_light(&self, x: usize, y: usize, z: usize) -> u8 {
        self.get_light_level(x, y, z) & 0x0F
    }

    /// Set sky light level
    pub fn set_sky_light(&mut self, x: usize, y: usize, z: usize, light: u8) {
        let current = self.get_light_level(x, y, z);
        let new_light = (current & 0x0F) | ((light & 0x0F) << 4);
        self.set_light_level(x, y, z, new_light);
    }

    /// Set block light level
    pub fn set_block_light(&mut self, x: usize, y: usize, z: usize, light: u8) {
        let current = self.get_light_level(x, y, z);
        let new_light = (current & 0xF0) | (light & 0x0F);
        self.set_light_level(x, y, z, new_light);
    }

    /// Simple lighting update for a single block
    fn update_lighting_at(&mut self, x: usize, y: usize, z: usize) {
        // TODO: Implement proper lighting propagation
        // For now, just set sky light based on whether there are blocks above
        
        let mut sky_light = 15; // Full sunlight
        for check_y in (y + 1)..CHUNK_HEIGHT {
            if self.blocks[x][z][check_y] != BlockType::Air {
                sky_light = 0;
                break;
            }
        }
        
        self.set_sky_light(x, y, z, sky_light);
        
        // Block light is 0 unless the block itself emits light
        let block_light = match self.blocks[x][z][y] {
            // TODO: Add light-emitting blocks
            _ => 0,
        };
        self.set_block_light(x, y, z, block_light);
    }

    /// Calculate lighting for the entire chunk
    pub fn calculate_lighting(&mut self) {
        // Sky lighting - propagate from top down
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let mut sky_light = 15;
                
                for y in (0..CHUNK_HEIGHT).rev() {
                    if self.blocks[x][z][y] != BlockType::Air {
                        sky_light = 0;
                    }
                    self.set_sky_light(x, y, z, sky_light);
                }
            }
        }

        // TODO: Implement block light propagation and more sophisticated lighting
    }

    /// Check if chunk is empty (all air blocks)
    pub fn is_empty(&self) -> bool {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if self.height_map[x][z] > 0 {
                    return false;
                }
            }
        }
        true
    }

    /// Get the total number of non-air blocks in this chunk
    pub fn block_count(&self) -> usize {
        let mut count = 0;
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for y in 0..CHUNK_HEIGHT {
                    if self.blocks[x][z][y] != BlockType::Air {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    /// Fill a region with a specific block type
    pub fn fill_region(
        &mut self,
        start_x: usize, start_y: usize, start_z: usize,
        end_x: usize, end_y: usize, end_z: usize,
        block: BlockType,
    ) {
        let end_x = end_x.min(CHUNK_SIZE);
        let end_y = end_y.min(CHUNK_HEIGHT);
        let end_z = end_z.min(CHUNK_SIZE);

        for x in start_x..end_x {
            for y in start_y..end_y {
                for z in start_z..end_z {
                    self.set_block(x, y, z, block);
                }
            }
        }
    }

    /// Mark chunk as dirty (needs to be saved)
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Mark chunk as clean (saved)
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }
}