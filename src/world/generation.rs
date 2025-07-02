use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use crate::world::{Chunk, ChunkCoordinate, BlockType, CHUNK_SIZE, CHUNK_HEIGHT};

/// World generator that creates Minecraft-like terrain using multiple noise layers
pub struct WorldGenerator {
    seed: u64,
    
    // Terrain noise generators
    terrain_noise: OpenSimplex,
    cave_noise: OpenSimplex,
    ore_noise: OpenSimplex,
    biome_temperature: OpenSimplex,
    biome_humidity: OpenSimplex,
    
    // Generation parameters
    sea_level: usize,
    max_height: usize,
    min_height: usize,
}

impl WorldGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            terrain_noise: OpenSimplex::new(seed as u32),
            cave_noise: OpenSimplex::new(seed.wrapping_add(1) as u32),
            ore_noise: OpenSimplex::new(seed.wrapping_add(2) as u32),
            biome_temperature: OpenSimplex::new(seed.wrapping_add(3) as u32),
            biome_humidity: OpenSimplex::new(seed.wrapping_add(4) as u32),
            sea_level: 64,
            max_height: 120,
            min_height: 30,
        }
    }

    /// Generate a complete chunk with terrain, caves, ores, and structures
    pub fn generate_chunk(&self, coord: ChunkCoordinate) -> Chunk {
        let mut chunk = Chunk::new(coord);
        
        // Generate base terrain
        self.generate_terrain(&mut chunk);
        
        // Generate caves
        self.generate_caves(&mut chunk);
        
        // Generate ores
        self.generate_ores(&mut chunk);
        
        // Generate surface features (trees, grass, etc.)
        self.generate_surface_features(&mut chunk);
        
        // Calculate lighting
        chunk.calculate_lighting();
        
        chunk
    }

    /// Generate base terrain using multiple octaves of noise
    fn generate_terrain(&self, chunk: &mut Chunk) {
        let (world_x, world_z) = chunk.coordinate.world_position();

        for local_x in 0..CHUNK_SIZE {
            for local_z in 0..CHUNK_SIZE {
                let world_pos_x = world_x + local_x as i32;
                let world_pos_z = world_z + local_z as i32;

                // Get biome for this position
                let biome = self.get_biome(world_pos_x as f64, world_pos_z as f64);
                
                // Generate height using multiple noise octaves
                let height = self.get_terrain_height(world_pos_x as f64, world_pos_z as f64, &biome);
                
                // Fill terrain column
                self.fill_terrain_column(chunk, local_x, local_z, height, &biome);
            }
        }
    }

    /// Calculate terrain height using multiple noise octaves
    fn get_terrain_height(&self, x: f64, z: f64, biome: &Biome) -> usize {
        let scale = 0.01; // Noise scale
        
        // Base terrain with multiple octaves
        let noise1 = self.terrain_noise.get([x * scale, z * scale]) * 0.5;
        let noise2 = self.terrain_noise.get([x * scale * 2.0, z * scale * 2.0]) * 0.25;
        let noise3 = self.terrain_noise.get([x * scale * 4.0, z * scale * 4.0]) * 0.125;
        
        let combined_noise = noise1 + noise2 + noise3;
        
        // Apply biome-specific height modifiers
        let height_modifier = match biome {
            Biome::Mountains => 1.5,
            Biome::Hills => 1.2,
            Biome::Plains => 0.8,
            Biome::Desert => 0.9,
            Biome::Forest => 1.0,
            Biome::Swamp => 0.6,
            Biome::Ocean => 0.3,
        };
        
        let height_range = (self.max_height - self.min_height) as f64;
        let normalized_height = (combined_noise + 1.0) * 0.5; // Normalize to 0-1
        let final_height = self.min_height as f64 + normalized_height * height_range * height_modifier;
        
        final_height.max(self.min_height as f64).min(self.max_height as f64) as usize
    }

    /// Fill a terrain column with appropriate blocks
    fn fill_terrain_column(&self, chunk: &mut Chunk, x: usize, z: usize, height: usize, biome: &Biome) {
        for y in 0..CHUNK_HEIGHT {
            let block = if y == 0 {
                BlockType::Stone // Bedrock equivalent
            } else if y <= height {
                if y == height {
                    // Surface block
                    match biome {
                        Biome::Desert => BlockType::Sand,
                        Biome::Ocean | Biome::Swamp => BlockType::Dirt,
                        _ => BlockType::Grass,
                    }
                } else if y >= height.saturating_sub(3) {
                    // Subsurface (dirt layer)
                    match biome {
                        Biome::Desert => BlockType::Sand,
                        _ => BlockType::Dirt,
                    }
                } else {
                    // Deep underground
                    BlockType::Stone
                }
            } else if y <= self.sea_level {
                // Water below sea level
                BlockType::Water
            } else {
                // Air above terrain
                BlockType::Air
            };

            chunk.set_block(x, y, z, block);
        }
    }

    /// Generate cave systems using 3D noise
    fn generate_caves(&self, chunk: &mut Chunk) {
        let (world_x, world_z) = chunk.coordinate.world_position();
        let cave_scale = 0.02;
        let cave_threshold = 0.4;

        for local_x in 0..CHUNK_SIZE {
            for local_z in 0..CHUNK_SIZE {
                for y in 5..80 { // Caves only in certain Y range
                    let world_pos_x = (world_x + local_x as i32) as f64;
                    let world_pos_z = (world_z + local_z as i32) as f64;
                    
                    let cave_noise = self.cave_noise.get([
                        world_pos_x * cave_scale,
                        y as f64 * cave_scale * 2.0, // Stretch vertically
                        world_pos_z * cave_scale
                    ]);

                    if cave_noise > cave_threshold {
                        let current_block = chunk.get_block(local_x, y, local_z);
                        if current_block == BlockType::Stone || current_block == BlockType::Dirt {
                            chunk.set_block(local_x, y, local_z, BlockType::Air);
                        }
                    }
                }
            }
        }
    }

    /// Generate ore deposits
    fn generate_ores(&self, chunk: &mut Chunk) {
        let (world_x, world_z) = chunk.coordinate.world_position();
        let mut rng = StdRng::seed_from_u64(
            self.seed.wrapping_add((world_x as u64) << 32).wrapping_add(world_z as u64)
        );

        // Coal ore (common, high levels)
        self.generate_ore_type(chunk, &mut rng, BlockType::CoalOre, 10..70, 0.02, 8);
        
        // Iron ore (common, mid levels)
        self.generate_ore_type(chunk, &mut rng, BlockType::IronOre, 5..50, 0.015, 6);
        
        // Gold ore (uncommon, mid levels)
        self.generate_ore_type(chunk, &mut rng, BlockType::GoldOre, 5..35, 0.008, 4);
        
        // Diamond ore (rare, low levels)
        self.generate_ore_type(chunk, &mut rng, BlockType::DiamondOre, 1..16, 0.003, 3);
        
        // Redstone ore (uncommon, low levels)
        self.generate_ore_type(chunk, &mut rng, BlockType::RedstoneOre, 1..20, 0.01, 5);
    }

    fn generate_ore_type(
        &self,
        chunk: &mut Chunk,
        rng: &mut StdRng,
        ore_type: BlockType,
        y_range: std::ops::Range<usize>,
        frequency: f64,
        vein_size: usize,
    ) {
        let (world_x, world_z) = chunk.coordinate.world_position();

        for _ in 0..(CHUNK_SIZE * CHUNK_SIZE / 64) { // Attempt frequency
            if rng.gen::<f64>() < frequency {
                let local_x = rng.gen_range(0..CHUNK_SIZE);
                let local_z = rng.gen_range(0..CHUNK_SIZE);
                let y = rng.gen_range(y_range.clone());

                // Generate ore vein
                self.place_ore_vein(chunk, local_x, y, local_z, ore_type, vein_size, rng);
            }
        }
    }

    fn place_ore_vein(
        &self,
        chunk: &mut Chunk,
        start_x: usize,
        start_y: usize,
        start_z: usize,
        ore_type: BlockType,
        max_size: usize,
        rng: &mut StdRng,
    ) {
        let mut placed = 0;
        let mut positions = vec![(start_x, start_y, start_z)];

        while !positions.is_empty() && placed < max_size {
            let (x, y, z) = positions.pop().unwrap();

            if x < CHUNK_SIZE && y < CHUNK_HEIGHT && z < CHUNK_SIZE {
                let current_block = chunk.get_block(x, y, z);
                if current_block == BlockType::Stone {
                    chunk.set_block(x, y, z, ore_type);
                    placed += 1;

                    // Add adjacent positions
                    if rng.gen::<f64>() < 0.6 {
                        for &(dx, dy, dz) in &[(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)] {
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;
                            let nz = z as i32 + dz;

                            if nx >= 0 && ny >= 0 && nz >= 0 {
                                positions.push((nx as usize, ny as usize, nz as usize));
                            }
                        }
                    }
                }
            }
        }
    }

    /// Generate surface features like trees and grass
    fn generate_surface_features(&self, chunk: &mut Chunk) {
        let (world_x, world_z) = chunk.coordinate.world_position();
        let mut rng = StdRng::seed_from_u64(
            self.seed.wrapping_add((world_x as u64) << 32).wrapping_add(world_z as u64)
        );

        for local_x in 0..CHUNK_SIZE {
            for local_z in 0..CHUNK_SIZE {
                let world_pos_x = world_x + local_x as i32;
                let world_pos_z = world_z + local_z as i32;
                let biome = self.get_biome(world_pos_x as f64, world_pos_z as f64);

                let surface_y = self.find_surface_level(chunk, local_x, local_z);
                
                if let Some(y) = surface_y {
                    if y < CHUNK_HEIGHT - 1 {
                        match biome {
                            Biome::Forest => {
                                if rng.gen::<f64>() < 0.1 {
                                    self.place_tree(chunk, local_x, y + 1, local_z, &mut rng);
                                } else if rng.gen::<f64>() < 0.3 {
                                    chunk.set_block(local_x, y + 1, local_z, BlockType::TallGrass);
                                }
                            },
                            Biome::Plains => {
                                if rng.gen::<f64>() < 0.2 {
                                    chunk.set_block(local_x, y + 1, local_z, BlockType::TallGrass);
                                }
                            },
                            Biome::Desert => {
                                if rng.gen::<f64>() < 0.02 {
                                    chunk.set_block(local_x, y + 1, local_z, BlockType::DeadBush);
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn find_surface_level(&self, chunk: &Chunk, x: usize, z: usize) -> Option<usize> {
        for y in (0..CHUNK_HEIGHT).rev() {
            let block = chunk.get_block(x, y, z);
            if block != BlockType::Air && block != BlockType::Water {
                return Some(y);
            }
        }
        None
    }

    fn place_tree(&self, chunk: &mut Chunk, x: usize, y: usize, z: usize, rng: &mut StdRng) {
        let tree_height = rng.gen_range(4..8);
        
        // Place trunk
        for h in 0..tree_height {
            if y + h < CHUNK_HEIGHT {
                chunk.set_block(x, y + h, z, BlockType::Log);
            }
        }
        
        // Place leaves
        let leaf_start = y + tree_height - 3;
        for leaf_y in leaf_start..(y + tree_height + 2) {
            if leaf_y >= CHUNK_HEIGHT { break; }
            
            let radius = if leaf_y >= y + tree_height { 1 } else { 2 };
            
            for dx in -(radius as i32)..=(radius as i32) {
                for dz in -(radius as i32)..=(radius as i32) {
                    let leaf_x = x as i32 + dx;
                    let leaf_z = z as i32 + dz;
                    
                    if leaf_x >= 0 && leaf_x < CHUNK_SIZE as i32 && 
                       leaf_z >= 0 && leaf_z < CHUNK_SIZE as i32 {
                        
                        let distance = (dx * dx + dz * dz) as f32;
                        if distance <= (radius * radius) as f32 && rng.gen::<f64>() < 0.8 {
                            let current = chunk.get_block(leaf_x as usize, leaf_y, leaf_z as usize);
                            if current == BlockType::Air {
                                chunk.set_block(leaf_x as usize, leaf_y, leaf_z as usize, BlockType::Leaves);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Determine biome based on temperature and humidity noise
    fn get_biome(&self, x: f64, z: f64) -> Biome {
        let biome_scale = 0.005;
        let temperature = self.biome_temperature.get([x * biome_scale, z * biome_scale]);
        let humidity = self.biome_humidity.get([x * biome_scale * 1.3, z * biome_scale * 1.7]);

        match (temperature, humidity) {
            (t, _) if t < -0.5 => Biome::Mountains,
            (t, h) if t > 0.5 && h < -0.3 => Biome::Desert,
            (t, h) if t < 0.2 && h > 0.3 => Biome::Swamp,
            (_, h) if h < -0.6 => Biome::Ocean,
            (t, h) if t > 0.0 && h > 0.0 => Biome::Forest,
            (t, _) if t > 0.2 => Biome::Hills,
            _ => Biome::Plains,
        }
    }
}

/// Different biome types that affect terrain generation
#[derive(Debug, Clone, Copy)]
pub enum Biome {
    Plains,
    Forest,
    Desert,
    Mountains,
    Hills,
    Swamp,
    Ocean,
}

impl Biome {
    pub fn name(&self) -> &'static str {
        match self {
            Biome::Plains => "Plains",
            Biome::Forest => "Forest",
            Biome::Desert => "Desert",
            Biome::Mountains => "Mountains",
            Biome::Hills => "Hills",
            Biome::Swamp => "Swamp",
            Biome::Ocean => "Ocean",
        }
    }
}