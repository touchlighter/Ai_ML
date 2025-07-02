use serde::{Deserialize, Serialize};

/// All block types in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BlockType {
    // Basic blocks
    Air,
    Stone,
    Grass,
    Dirt,
    Cobblestone,
    Wood,
    Sand,
    Gravel,
    
    // Ores
    CoalOre,
    IronOre,
    GoldOre,
    DiamondOre,
    RedstoneOre,
    LapisOre,
    EmeraldOre,
    
    // Nature blocks
    Leaves,
    Log,
    Cactus,
    DeadBush,
    TallGrass,
    Flower,
    Mushroom,
    
    // Liquids
    Water,
    Lava,
    
    // Crafted blocks
    Planks,
    Glass,
    Brick,
    MossyCobblestone,
    Obsidian,
    
    // Redstone
    Redstone,
    RedstoneTorch,
    RedstoneWire,
    Lever,
    Button,
    PressurePlate,
    
    // Utility blocks
    Chest,
    Furnace,
    CraftingTable,
    Bed,
    Door,
    Ladder,
    Torch,
    
    // Building blocks
    Wool,
    Clay,
    Sandstone,
    Netherrack,
    SoulSand,
    Glowstone,
}

impl BlockType {
    /// Check if the block is solid (player can't walk through it)
    pub fn is_solid(&self) -> bool {
        match self {
            BlockType::Air 
            | BlockType::Water 
            | BlockType::Lava 
            | BlockType::TallGrass 
            | BlockType::Flower 
            | BlockType::Mushroom 
            | BlockType::DeadBush 
            | BlockType::Torch 
            | BlockType::RedstoneWire 
            | BlockType::RedstoneTorch => false,
            _ => true,
        }
    }

    /// Check if the block is transparent (light passes through)
    pub fn is_transparent(&self) -> bool {
        match self {
            BlockType::Air
            | BlockType::Glass
            | BlockType::Water
            | BlockType::Leaves
            | BlockType::TallGrass
            | BlockType::Flower
            | BlockType::Mushroom
            | BlockType::DeadBush
            | BlockType::Torch
            | BlockType::RedstoneWire
            | BlockType::RedstoneTorch => true,
            _ => false,
        }
    }

    /// Check if the block emits light
    pub fn light_level(&self) -> u8 {
        match self {
            BlockType::Torch => 14,
            BlockType::RedstoneTorch => 7,
            BlockType::Glowstone => 15,
            BlockType::Lava => 15,
            _ => 0,
        }
    }

    /// Check if the block can be mined by hand
    pub fn can_mine_by_hand(&self) -> bool {
        match self {
            BlockType::Stone
            | BlockType::Cobblestone
            | BlockType::CoalOre
            | BlockType::IronOre
            | BlockType::GoldOre
            | BlockType::DiamondOre
            | BlockType::RedstoneOre
            | BlockType::LapisOre
            | BlockType::EmeraldOre
            | BlockType::Obsidian => false,
            _ => true,
        }
    }

    /// Get mining time in seconds (simplified)
    pub fn mining_time(&self) -> f32 {
        match self {
            BlockType::Air => 0.0,
            BlockType::TallGrass
            | BlockType::Flower
            | BlockType::Mushroom
            | BlockType::DeadBush
            | BlockType::Torch
            | BlockType::RedstoneWire
            | BlockType::RedstoneTorch => 0.1,
            BlockType::Dirt
            | BlockType::Sand
            | BlockType::Gravel => 0.5,
            BlockType::Wood
            | BlockType::Planks
            | BlockType::Leaves => 0.75,
            BlockType::Stone
            | BlockType::Cobblestone => 1.5,
            BlockType::CoalOre
            | BlockType::IronOre => 3.0,
            BlockType::GoldOre
            | BlockType::DiamondOre => 4.0,
            BlockType::Obsidian => 15.0,
            _ => 1.0,
        }
    }

    /// Get the block that drops when this block is mined
    pub fn drops(&self) -> Vec<(BlockType, u32)> {
        match self {
            BlockType::Stone => vec![(BlockType::Cobblestone, 1)],
            BlockType::Grass => vec![(BlockType::Dirt, 1)],
            BlockType::CoalOre => vec![(BlockType::Redstone, 1)], // Simplified - should drop coal item
            BlockType::DiamondOre => vec![(BlockType::Redstone, 1)], // Simplified - should drop diamond item
            BlockType::RedstoneOre => vec![(BlockType::Redstone, 4)],
            BlockType::Leaves => {
                // TODO: Random chance for saplings and apples
                vec![]
            },
            BlockType::TallGrass => {
                // TODO: Random chance for seeds
                vec![]
            },
            _ => vec![(*self, 1)],
        }
    }

    /// Check if the block is affected by gravity
    pub fn is_affected_by_gravity(&self) -> bool {
        match self {
            BlockType::Sand | BlockType::Gravel => true,
            _ => false,
        }
    }

    /// Check if the block can be replaced (like tall grass, flowers)
    pub fn is_replaceable(&self) -> bool {
        match self {
            BlockType::Air
            | BlockType::TallGrass
            | BlockType::Flower
            | BlockType::Mushroom
            | BlockType::DeadBush
            | BlockType::Water => true,
            _ => false,
        }
    }

    /// Get hardness value (affects mining speed)
    pub fn hardness(&self) -> f32 {
        match self {
            BlockType::Air => 0.0,
            BlockType::TallGrass
            | BlockType::Flower
            | BlockType::Mushroom
            | BlockType::DeadBush => 0.0,
            BlockType::Dirt
            | BlockType::Sand
            | BlockType::Gravel => 0.5,
            BlockType::Wood
            | BlockType::Planks => 2.0,
            BlockType::Stone => 1.5,
            BlockType::Cobblestone => 2.0,
            BlockType::CoalOre
            | BlockType::IronOre => 3.0,
            BlockType::GoldOre => 3.0,
            BlockType::DiamondOre => 3.0,
            BlockType::Obsidian => 50.0,
            _ => 1.0,
        }
    }

    /// Get explosion resistance
    pub fn explosion_resistance(&self) -> f32 {
        match self {
            BlockType::Air => 0.0,
            BlockType::TallGrass
            | BlockType::Flower
            | BlockType::Mushroom
            | BlockType::DeadBush => 0.0,
            BlockType::Dirt
            | BlockType::Sand
            | BlockType::Gravel => 2.5,
            BlockType::Stone
            | BlockType::Cobblestone => 30.0,
            BlockType::Obsidian => 6000.0,
            _ => 15.0,
        }
    }

    /// Check if the block requires a support block below it
    pub fn needs_support(&self) -> bool {
        match self {
            BlockType::TallGrass
            | BlockType::Flower
            | BlockType::Mushroom
            | BlockType::DeadBush
            | BlockType::Torch
            | BlockType::RedstoneTorch => true,
            _ => false,
        }
    }

    /// Get the block ID for serialization and networking
    pub fn id(&self) -> u16 {
        match self {
            BlockType::Air => 0,
            BlockType::Stone => 1,
            BlockType::Grass => 2,
            BlockType::Dirt => 3,
            BlockType::Cobblestone => 4,
            BlockType::Wood => 5,
            BlockType::Sand => 12,
            BlockType::Gravel => 13,
            BlockType::CoalOre => 16,
            BlockType::IronOre => 15,
            BlockType::GoldOre => 14,
            BlockType::DiamondOre => 56,
            BlockType::RedstoneOre => 73,
            BlockType::LapisOre => 21,
            BlockType::EmeraldOre => 129,
            BlockType::Leaves => 18,
            BlockType::Log => 17,
            BlockType::Water => 8,
            BlockType::Lava => 10,
            BlockType::Planks => 5,
            BlockType::Glass => 20,
            BlockType::Torch => 50,
            _ => 255, // Unknown
        }
    }

    /// Create block from ID
    pub fn from_id(id: u16) -> Option<Self> {
        match id {
            0 => Some(BlockType::Air),
            1 => Some(BlockType::Stone),
            2 => Some(BlockType::Grass),
            3 => Some(BlockType::Dirt),
            4 => Some(BlockType::Cobblestone),
            5 => Some(BlockType::Wood),
            12 => Some(BlockType::Sand),
            13 => Some(BlockType::Gravel),
            16 => Some(BlockType::CoalOre),
            15 => Some(BlockType::IronOre),
            14 => Some(BlockType::GoldOre),
            56 => Some(BlockType::DiamondOre),
            73 => Some(BlockType::RedstoneOre),
            21 => Some(BlockType::LapisOre),
            129 => Some(BlockType::EmeraldOre),
            18 => Some(BlockType::Leaves),
            17 => Some(BlockType::Log),
            8 => Some(BlockType::Water),
            10 => Some(BlockType::Lava),
            20 => Some(BlockType::Glass),
            50 => Some(BlockType::Torch),
            _ => None,
        }
    }

    /// Get display name for the block
    pub fn name(&self) -> &'static str {
        match self {
            BlockType::Air => "Air",
            BlockType::Stone => "Stone",
            BlockType::Grass => "Grass Block",
            BlockType::Dirt => "Dirt",
            BlockType::Cobblestone => "Cobblestone",
            BlockType::Wood => "Wood",
            BlockType::Sand => "Sand",
            BlockType::Gravel => "Gravel",
            BlockType::CoalOre => "Coal Ore",
            BlockType::IronOre => "Iron Ore",
            BlockType::GoldOre => "Gold Ore",
            BlockType::DiamondOre => "Diamond Ore",
            BlockType::RedstoneOre => "Redstone Ore",
            BlockType::LapisOre => "Lapis Lazuli Ore",
            BlockType::EmeraldOre => "Emerald Ore",
            BlockType::Leaves => "Leaves",
            BlockType::Log => "Log",
            BlockType::Cactus => "Cactus",
            BlockType::DeadBush => "Dead Bush",
            BlockType::TallGrass => "Tall Grass",
            BlockType::Flower => "Flower",
            BlockType::Mushroom => "Mushroom",
            BlockType::Water => "Water",
            BlockType::Lava => "Lava",
            BlockType::Planks => "Wooden Planks",
            BlockType::Glass => "Glass",
            BlockType::Brick => "Brick",
            BlockType::MossyCobblestone => "Mossy Cobblestone",
            BlockType::Obsidian => "Obsidian",
            BlockType::Redstone => "Redstone",
            BlockType::RedstoneTorch => "Redstone Torch",
            BlockType::RedstoneWire => "Redstone Wire",
            BlockType::Lever => "Lever",
            BlockType::Button => "Button",
            BlockType::PressurePlate => "Pressure Plate",
            BlockType::Chest => "Chest",
            BlockType::Furnace => "Furnace",
            BlockType::CraftingTable => "Crafting Table",
            BlockType::Bed => "Bed",
            BlockType::Door => "Door",
            BlockType::Ladder => "Ladder",
            BlockType::Torch => "Torch",
            BlockType::Wool => "Wool",
            BlockType::Clay => "Clay",
            BlockType::Sandstone => "Sandstone",
            BlockType::Netherrack => "Netherrack",
            BlockType::SoulSand => "Soul Sand",
            BlockType::Glowstone => "Glowstone",
        }
    }
}

impl Default for BlockType {
    fn default() -> Self {
        BlockType::Air
    }
}