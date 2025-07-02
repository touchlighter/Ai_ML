use glam::Vec3;
use crate::game::inventory::Inventory;

/// Player state and data
pub struct Player {
    position: Vec3,
    velocity: Vec3,
    health: f32,
    max_health: f32,
    hunger: f32,
    max_hunger: f32,
    experience: u32,
    level: u32,
    inventory: Inventory,
    selected_hotbar_slot: usize,
    
    // Player properties
    reach_distance: f32,
    walking_speed: f32,
    sprinting_speed: f32,
    flying: bool,
}

impl Player {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            velocity: Vec3::ZERO,
            health: 20.0,
            max_health: 20.0,
            hunger: 20.0,
            max_hunger: 20.0,
            experience: 0,
            level: 0,
            inventory: Inventory::new(),
            selected_hotbar_slot: 0,
            reach_distance: 5.0,
            walking_speed: 4.317, // Minecraft walking speed
            sprinting_speed: 5.612, // Minecraft sprinting speed
            flying: false,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update player physics (simplified)
        self.position += self.velocity * delta_time;
        
        // Apply gravity if not flying
        if !self.flying {
            self.velocity.y -= 9.81 * delta_time; // Gravity
        }
        
        // Update inventory
        self.inventory.update(delta_time);
    }

    // Position and movement
    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn velocity(&self) -> Vec3 {
        self.velocity
    }

    pub fn set_velocity(&mut self, velocity: Vec3) {
        self.velocity = velocity;
    }

    // Health and hunger
    pub fn health(&self) -> f32 {
        self.health
    }

    pub fn max_health(&self) -> f32 {
        self.max_health
    }

    pub fn health_percentage(&self) -> f32 {
        self.health / self.max_health
    }

    pub fn damage(&mut self, amount: f32) {
        self.health = (self.health - amount).max(0.0);
    }

    pub fn heal(&mut self, amount: f32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    pub fn hunger(&self) -> f32 {
        self.hunger
    }

    pub fn max_hunger(&self) -> f32 {
        self.max_hunger
    }

    pub fn hunger_percentage(&self) -> f32 {
        self.hunger / self.max_hunger
    }

    // Experience and leveling
    pub fn experience(&self) -> u32 {
        self.experience
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn add_experience(&mut self, amount: u32) {
        self.experience += amount;
        // TODO: Calculate level progression
    }

    // Inventory
    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub fn inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }

    pub fn selected_hotbar_slot(&self) -> usize {
        self.selected_hotbar_slot
    }

    pub fn set_selected_hotbar_slot(&mut self, slot: usize) {
        if slot < 9 {
            self.selected_hotbar_slot = slot;
        }
    }

    // Abilities
    pub fn reach_distance(&self) -> f32 {
        self.reach_distance
    }

    pub fn walking_speed(&self) -> f32 {
        self.walking_speed
    }

    pub fn sprinting_speed(&self) -> f32 {
        self.sprinting_speed
    }

    pub fn is_flying(&self) -> bool {
        self.flying
    }

    pub fn set_flying(&mut self, flying: bool) {
        self.flying = flying;
        if flying {
            self.velocity.y = 0.0; // Stop falling when starting to fly
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }
}