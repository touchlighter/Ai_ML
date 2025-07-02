use std::collections::HashMap;
use crate::world::{World, ChunkCoordinate};
use crate::rendering::vertex::ChunkMesh;

/// Handles rendering of world chunks with frustum culling and mesh batching
pub struct ChunkRenderer {
    // Cache of chunk meshes
    chunk_meshes: HashMap<ChunkCoordinate, ChunkMesh>,
    // Meshes that need to be updated
    dirty_chunks: Vec<ChunkCoordinate>,
}

impl ChunkRenderer {
    pub fn new(device: &wgpu::Device, _pipeline_layout: &wgpu::PipelineLayout) -> Self {
        Self {
            chunk_meshes: HashMap::new(),
            dirty_chunks: Vec::new(),
        }
    }

    pub fn update_chunk(&mut self, chunk_coord: ChunkCoordinate, device: &wgpu::Device, world: &World) {
        // Generate mesh for the chunk
        if let Some(chunk) = world.get_chunk(chunk_coord) {
            let mut mesh = ChunkMesh::new();
            self.generate_chunk_mesh(chunk_coord, chunk, world, &mut mesh);
            mesh.finalize(device);
            self.chunk_meshes.insert(chunk_coord, mesh);
        }
    }

    pub fn mark_chunk_dirty(&mut self, chunk_coord: ChunkCoordinate) {
        if !self.dirty_chunks.contains(&chunk_coord) {
            self.dirty_chunks.push(chunk_coord);
        }
    }

    pub fn update_dirty_chunks(&mut self, device: &wgpu::Device, world: &World) {
        let dirty_chunks = std::mem::take(&mut self.dirty_chunks);
        for chunk_coord in dirty_chunks {
            self.update_chunk(chunk_coord, device, world);
        }
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, world: &World) {
        // TODO: Implement frustum culling here
        // For now, render all loaded chunks
        for (chunk_coord, mesh) in &self.chunk_meshes {
            if world.is_chunk_loaded(*chunk_coord) {
                mesh.render(render_pass);
            }
        }
    }

    fn generate_chunk_mesh(
        &self,
        chunk_coord: ChunkCoordinate,
        chunk: &crate::world::Chunk,
        world: &World,
        mesh: &mut ChunkMesh,
    ) {
        use crate::rendering::vertex::Face;
        use crate::world::{BlockType, CHUNK_SIZE, CHUNK_HEIGHT};

        mesh.clear();

        let chunk_world_x = chunk_coord.x * CHUNK_SIZE as i32;
        let chunk_world_z = chunk_coord.z * CHUNK_SIZE as i32;

        // Iterate through all blocks in the chunk
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let block = chunk.get_block(x, y, z);
                    
                    // Skip air blocks
                    if block == BlockType::Air {
                        continue;
                    }

                    let world_x = chunk_world_x + x as i32;
                    let world_y = y as i32;
                    let world_z = chunk_world_z + z as i32;

                    // Check each face to see if it should be rendered
                    for face in Face::all() {
                        if self.should_render_face(
                            world_x, world_y, world_z, face, chunk, world, chunk_coord
                        ) {
                            let texture_id = self.get_texture_id_for_block(block, face);
                            let light_level = self.calculate_light_level(world_x, world_y, world_z, world);
                            
                            mesh.add_face(
                                face,
                                world_x as f32,
                                world_y as f32,
                                world_z as f32,
                                texture_id,
                                light_level,
                            );
                        }
                    }
                }
            }
        }
    }

    fn should_render_face(
        &self,
        world_x: i32,
        world_y: i32,
        world_z: i32,
        face: Face,
        chunk: &crate::world::Chunk,
        world: &World,
        chunk_coord: ChunkCoordinate,
    ) -> bool {
        use crate::world::{BlockType, CHUNK_SIZE, CHUNK_HEIGHT};

        // Calculate adjacent block position
        let (adj_x, adj_y, adj_z) = match face {
            Face::Top => (world_x, world_y + 1, world_z),
            Face::Bottom => (world_x, world_y - 1, world_z),
            Face::Front => (world_x, world_y, world_z + 1),
            Face::Back => (world_x, world_y, world_z - 1),
            Face::Left => (world_x - 1, world_y, world_z),
            Face::Right => (world_x + 1, world_y, world_z),
        };

        // Check if adjacent block is in the same chunk
        let chunk_world_x = chunk_coord.x * CHUNK_SIZE as i32;
        let chunk_world_z = chunk_coord.z * CHUNK_SIZE as i32;
        
        let adj_chunk_x = adj_x - chunk_world_x;
        let adj_chunk_z = adj_z - chunk_world_z;

        let adjacent_block = if adj_y < 0 || adj_y >= CHUNK_HEIGHT as i32 {
            // Outside vertical bounds
            if adj_y < 0 {
                BlockType::Stone // Assume bedrock below
            } else {
                BlockType::Air // Air above
            }
        } else if adj_chunk_x >= 0 && adj_chunk_x < CHUNK_SIZE as i32 && 
                  adj_chunk_z >= 0 && adj_chunk_z < CHUNK_SIZE as i32 {
            // Same chunk
            chunk.get_block(adj_chunk_x as usize, adj_y as usize, adj_chunk_z as usize)
        } else {
            // Different chunk - query world
            world.get_block_at(adj_x, adj_y, adj_z).unwrap_or(BlockType::Air)
        };

        // Render face if adjacent block is transparent (air)
        adjacent_block == BlockType::Air
    }

    fn get_texture_id_for_block(&self, block: BlockType, face: Face) -> u32 {
        use crate::world::BlockType;
        
        match block {
            BlockType::Air => 0, // Should not be rendered
            BlockType::Stone => match face {
                _ => 1, // Stone texture
            },
            BlockType::Dirt => match face {
                _ => 2, // Dirt texture
            },
            BlockType::Grass => match face {
                Face::Top => 3,    // Grass top
                Face::Bottom => 2, // Dirt bottom
                _ => 4,            // Grass side
            },
            BlockType::Sand => match face {
                _ => 5, // Sand texture
            },
            BlockType::Wood => match face {
                Face::Top | Face::Bottom => 6, // Wood rings
                _ => 7,                        // Wood bark
            },
            BlockType::Leaves => match face {
                _ => 8, // Leaves texture
            },
            BlockType::Water => match face {
                _ => 9, // Water texture
            },
            BlockType::Cobblestone => match face {
                _ => 10, // Cobblestone texture
            },
        }
    }

    fn calculate_light_level(&self, _x: i32, _y: i32, _z: i32, _world: &World) -> f32 {
        // TODO: Implement proper lighting calculation
        // For now, return full brightness
        1.0
    }

    pub fn remove_chunk(&mut self, chunk_coord: ChunkCoordinate) {
        self.chunk_meshes.remove(&chunk_coord);
    }

    pub fn clear(&mut self) {
        self.chunk_meshes.clear();
        self.dirty_chunks.clear();
    }
}