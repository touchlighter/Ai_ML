use glam::Vec3;
use crate::world::{BlockType, World, RaycastHit};
use crate::rendering::camera::{Camera, CameraMovement, Ray};
use crate::input::InputManager;

mod player;
mod inventory;
mod physics;

pub use player::Player;
pub use inventory::{Inventory, ItemStack};

/// Main game manager that handles game logic and player state
pub struct GameManager {
    player: Player,
    game_mode: GameMode,
    selected_block_type: BlockType,
    breaking_progress: f32,
    breaking_target: Option<Vec3>,
    breaking_time: f32,
    
    // Game state
    paused: bool,
    debug_mode: bool,
    show_inventory: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            player: Player::new(Vec3::new(0.0, 100.0, 0.0)),
            game_mode: GameMode::Creative, // Start in creative for testing
            selected_block_type: BlockType::Stone,
            breaking_progress: 0.0,
            breaking_target: None,
            breaking_time: 0.0,
            paused: false,
            debug_mode: false,
            show_inventory: false,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.paused {
            return;
        }

        // Update player
        self.player.update(delta_time);
        
        // Update breaking progress
        if let Some(_target) = self.breaking_target {
            self.breaking_time += delta_time;
            let block_type = self.selected_block_type; // In real game, this would be the target block
            let mining_time = block_type.mining_time();
            
            self.breaking_progress = (self.breaking_time / mining_time).min(1.0);
        }
    }

    /// Process input and update game state
    pub fn handle_input(&mut self, input: &InputManager, camera: &mut Camera, world: &mut World, delta_time: f32) {
        // Handle UI toggles
        if input.escape() {
            self.paused = !self.paused;
        }

        if input.toggle_debug() {
            self.debug_mode = !self.debug_mode;
        }

        if input.open_inventory() {
            self.show_inventory = !self.show_inventory;
        }

        if self.paused || self.show_inventory {
            return;
        }

        // Handle camera movement
        self.handle_camera_movement(input, camera, delta_time);
        
        // Handle block interaction
        self.handle_block_interaction(input, camera, world, delta_time);
        
        // Handle hotbar selection
        if let Some(slot) = input.get_hotbar_selection() {
            self.player.set_selected_hotbar_slot(slot);
            
            // Set selected block type based on hotbar (simplified)
            self.selected_block_type = match slot {
                0 => BlockType::Stone,
                1 => BlockType::Dirt,
                2 => BlockType::Grass,
                3 => BlockType::Wood,
                4 => BlockType::Sand,
                5 => BlockType::Glass,
                6 => BlockType::Cobblestone,
                7 => BlockType::Leaves,
                8 => BlockType::Torch,
                _ => BlockType::Stone,
            };
        }

        // Update player position and world chunk loading
        let player_pos = camera.position();
        self.player.set_position(player_pos);
        world.load_chunks_around(player_pos);
    }

    fn handle_camera_movement(&mut self, input: &InputManager, camera: &mut Camera, delta_time: f32) {
        // Movement
        if input.move_forward() {
            camera.process_keyboard(CameraMovement::Forward, delta_time);
        }
        if input.move_backward() {
            camera.process_keyboard(CameraMovement::Backward, delta_time);
        }
        if input.move_left() {
            camera.process_keyboard(CameraMovement::Left, delta_time);
        }
        if input.move_right() {
            camera.process_keyboard(CameraMovement::Right, delta_time);
        }
        if input.jump() {
            camera.process_keyboard(CameraMovement::Up, delta_time);
        }
        if input.sneak() {
            camera.process_keyboard(CameraMovement::Down, delta_time);
        }

        // Mouse look
        if input.is_mouse_captured() {
            let (mouse_dx, mouse_dy) = input.mouse_delta();
            camera.process_mouse_movement(mouse_dx as f32, -mouse_dy as f32, true);
        }

        // Capture mouse on first click
        if input.is_mouse_button_just_pressed(winit::event::MouseButton::Left) && !input.is_mouse_captured() {
            // TODO: Actually capture the mouse cursor
        }
    }

    fn handle_block_interaction(&mut self, input: &InputManager, camera: &Camera, world: &mut World, delta_time: f32) {
        let ray = camera.cast_ray(5.0); // 5 block reach distance
        
        if input.break_block() {
            self.handle_block_breaking(&ray, world, delta_time);
        } else if input.place_block() {
            self.handle_block_placement(&ray, world);
        } else {
            // Reset breaking if not holding break
            self.breaking_target = None;
            self.breaking_progress = 0.0;
            self.breaking_time = 0.0;
        }
    }

    fn handle_block_breaking(&mut self, ray: &Ray, world: &mut World, delta_time: f32) {
        if let Some(hit) = world.raycast(ray) {
            let target_pos = hit.position;
            
            // Check if we're breaking the same block
            if let Some(current_target) = self.breaking_target {
                if current_target != target_pos {
                    // Started breaking a different block
                    self.breaking_target = Some(target_pos);
                    self.breaking_progress = 0.0;
                    self.breaking_time = 0.0;
                }
            } else {
                // Started breaking a new block
                self.breaking_target = Some(target_pos);
                self.breaking_progress = 0.0;
                self.breaking_time = 0.0;
            }

            // Update breaking progress
            self.breaking_time += delta_time;
            let mining_time = hit.block_type.mining_time();
            self.breaking_progress = (self.breaking_time / mining_time).min(1.0);

            // Break the block if progress is complete
            if self.breaking_progress >= 1.0 {
                let x = target_pos.x as i32;
                let y = target_pos.y as i32;
                let z = target_pos.z as i32;
                
                // Add drops to player inventory (simplified)
                let drops = hit.block_type.drops();
                for (block_type, count) in drops {
                    self.player.inventory_mut().add_item(ItemStack::new(block_type, count));
                }
                
                // Remove the block
                world.set_block_at(x, y, z, BlockType::Air);
                
                // Reset breaking state
                self.breaking_target = None;
                self.breaking_progress = 0.0;
                self.breaking_time = 0.0;
            }
        }
    }

    fn handle_block_placement(&mut self, ray: &Ray, world: &mut World) {
        if let Some(hit) = world.raycast(ray) {
            // Calculate placement position (adjacent to hit block)
            let place_pos = self.calculate_placement_position(&hit, ray);
            
            if let Some(pos) = place_pos {
                let x = pos.x as i32;
                let y = pos.y as i32;
                let z = pos.z as i32;
                
                // Check if position is valid for placement
                if let Some(existing_block) = world.get_block_at(x, y, z) {
                    if existing_block.is_replaceable() {
                        // Remove item from inventory if in survival mode
                        if self.game_mode == GameMode::Survival {
                            if self.player.inventory().has_item(self.selected_block_type) {
                                self.player.inventory_mut().remove_item(self.selected_block_type, 1);
                                world.set_block_at(x, y, z, self.selected_block_type);
                            }
                        } else {
                            // Creative mode - place without cost
                            world.set_block_at(x, y, z, self.selected_block_type);
                        }
                    }
                }
            }
        }
    }

    fn calculate_placement_position(&self, hit: &RaycastHit, ray: &Ray) -> Option<Vec3> {
        // Simple approach: place adjacent to the hit block
        // This should be improved to check which face was hit
        let hit_pos = hit.position;
        let ray_dir = ray.direction.normalize();
        
        // Try different adjacent positions
        let offsets = [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, -1.0),
        ];
        
        // Choose the offset that's most opposite to the ray direction
        let mut best_offset = offsets[0];
        let mut best_dot = ray_dir.dot(offsets[0]);
        
        for offset in offsets.iter() {
            let dot = ray_dir.dot(*offset);
            if dot < best_dot {
                best_dot = dot;
                best_offset = *offset;
            }
        }
        
        Some(hit_pos + best_offset)
    }

    // Getters
    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn game_mode(&self) -> GameMode {
        self.game_mode
    }

    pub fn set_game_mode(&mut self, mode: GameMode) {
        self.game_mode = mode;
    }

    pub fn selected_block_type(&self) -> BlockType {
        self.selected_block_type
    }

    pub fn breaking_progress(&self) -> f32 {
        self.breaking_progress
    }

    pub fn breaking_target(&self) -> Option<Vec3> {
        self.breaking_target
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn is_debug_mode(&self) -> bool {
        self.debug_mode
    }

    pub fn is_inventory_open(&self) -> bool {
        self.show_inventory
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }
}

impl Default for GameManager {
    fn default() -> Self {
        Self::new()
    }
}