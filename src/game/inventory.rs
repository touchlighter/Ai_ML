use crate::world::BlockType;
use std::collections::HashMap;

/// Item stack with type and count
#[derive(Debug, Clone, PartialEq)]
pub struct ItemStack {
    pub item_type: BlockType,
    pub count: u32,
    pub max_stack_size: u32,
}

impl ItemStack {
    pub fn new(item_type: BlockType, count: u32) -> Self {
        Self {
            item_type,
            count,
            max_stack_size: Self::get_max_stack_size(item_type),
        }
    }

    pub fn empty() -> Self {
        Self {
            item_type: BlockType::Air,
            count: 0,
            max_stack_size: 64,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0 || self.item_type == BlockType::Air
    }

    pub fn is_full(&self) -> bool {
        self.count >= self.max_stack_size
    }

    pub fn can_stack_with(&self, other: &ItemStack) -> bool {
        self.item_type == other.item_type && !self.is_full()
    }

    pub fn add(&mut self, count: u32) -> u32 {
        let can_add = (self.max_stack_size - self.count).min(count);
        self.count += can_add;
        count - can_add // Return leftover
    }

    pub fn remove(&mut self, count: u32) -> u32 {
        let removed = self.count.min(count);
        self.count -= removed;
        if self.count == 0 {
            self.item_type = BlockType::Air;
        }
        removed
    }

    fn get_max_stack_size(item_type: BlockType) -> u32 {
        match item_type {
            // Tools and weapons typically stack to 1
            // For now, everything stacks to 64
            _ => 64,
        }
    }
}

/// Player inventory with hotbar and storage
pub struct Inventory {
    // 9 slots for hotbar
    hotbar: [ItemStack; 9],
    // 27 slots for main inventory
    main: [ItemStack; 27],
    // 4 slots for armor
    armor: [ItemStack; 4],
    // 1 slot for offhand
    offhand: ItemStack,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            hotbar: [
                ItemStack::empty(), ItemStack::empty(), ItemStack::empty(),
                ItemStack::empty(), ItemStack::empty(), ItemStack::empty(),
                ItemStack::empty(), ItemStack::empty(), ItemStack::empty(),
            ],
            main: [ItemStack::empty(); 27],
            armor: [ItemStack::empty(); 4],
            offhand: ItemStack::empty(),
        }
    }

    pub fn update(&mut self, _delta_time: f32) {
        // TODO: Handle item updates (durability, etc.)
    }

    /// Add an item to the inventory
    pub fn add_item(&mut self, mut item: ItemStack) -> ItemStack {
        if item.is_empty() {
            return item;
        }

        // Try to add to existing stacks first
        item = self.add_to_existing_stacks(item);
        if item.is_empty() {
            return item;
        }

        // Then try to add to empty slots
        item = self.add_to_empty_slots(item);
        item
    }

    fn add_to_existing_stacks(&mut self, mut item: ItemStack) -> ItemStack {
        // Check hotbar
        for slot in &mut self.hotbar {
            if slot.can_stack_with(&item) {
                let leftover = slot.add(item.count);
                item.count = leftover;
                if item.count == 0 {
                    return ItemStack::empty();
                }
            }
        }

        // Check main inventory
        for slot in &mut self.main {
            if slot.can_stack_with(&item) {
                let leftover = slot.add(item.count);
                item.count = leftover;
                if item.count == 0 {
                    return ItemStack::empty();
                }
            }
        }

        item
    }

    fn add_to_empty_slots(&mut self, mut item: ItemStack) -> ItemStack {
        // Check hotbar for empty slots
        for slot in &mut self.hotbar {
            if slot.is_empty() {
                *slot = item.clone();
                return ItemStack::empty();
            }
        }

        // Check main inventory for empty slots
        for slot in &mut self.main {
            if slot.is_empty() {
                *slot = item.clone();
                return ItemStack::empty();
            }
        }

        item // Inventory is full
    }

    /// Remove an item from the inventory
    pub fn remove_item(&mut self, item_type: BlockType, count: u32) -> u32 {
        let mut remaining = count;

        // Remove from hotbar first
        for slot in &mut self.hotbar {
            if slot.item_type == item_type {
                let removed = slot.remove(remaining);
                remaining -= removed;
                if remaining == 0 {
                    return count;
                }
            }
        }

        // Remove from main inventory
        for slot in &mut self.main {
            if slot.item_type == item_type {
                let removed = slot.remove(remaining);
                remaining -= removed;
                if remaining == 0 {
                    return count;
                }
            }
        }

        count - remaining // Return amount actually removed
    }

    /// Check if inventory has a specific item
    pub fn has_item(&self, item_type: BlockType) -> bool {
        self.get_item_count(item_type) > 0
    }

    /// Get total count of a specific item
    pub fn get_item_count(&self, item_type: BlockType) -> u32 {
        let mut total = 0;

        // Count in hotbar
        for slot in &self.hotbar {
            if slot.item_type == item_type {
                total += slot.count;
            }
        }

        // Count in main inventory
        for slot in &self.main {
            if slot.item_type == item_type {
                total += slot.count;
            }
        }

        total
    }

    /// Get hotbar slots
    pub fn hotbar(&self) -> &[ItemStack; 9] {
        &self.hotbar
    }

    /// Get main inventory slots
    pub fn main(&self) -> &[ItemStack; 27] {
        &self.main
    }

    /// Get armor slots
    pub fn armor(&self) -> &[ItemStack; 4] {
        &self.armor
    }

    /// Get offhand slot
    pub fn offhand(&self) -> &ItemStack {
        &self.offhand
    }

    /// Get item in specific hotbar slot
    pub fn get_hotbar_item(&self, slot: usize) -> Option<&ItemStack> {
        if slot < 9 {
            Some(&self.hotbar[slot])
        } else {
            None
        }
    }

    /// Set item in specific hotbar slot
    pub fn set_hotbar_item(&mut self, slot: usize, item: ItemStack) -> Option<ItemStack> {
        if slot < 9 {
            let old_item = std::mem::replace(&mut self.hotbar[slot], item);
            if old_item.is_empty() {
                None
            } else {
                Some(old_item)
            }
        } else {
            Some(item) // Return the item if slot is invalid
        }
    }

    /// Clear all items from inventory
    pub fn clear(&mut self) {
        for slot in &mut self.hotbar {
            *slot = ItemStack::empty();
        }
        for slot in &mut self.main {
            *slot = ItemStack::empty();
        }
        for slot in &mut self.armor {
            *slot = ItemStack::empty();
        }
        self.offhand = ItemStack::empty();
    }

    /// Check if inventory is empty
    pub fn is_empty(&self) -> bool {
        self.hotbar.iter().all(|slot| slot.is_empty()) &&
        self.main.iter().all(|slot| slot.is_empty()) &&
        self.armor.iter().all(|slot| slot.is_empty()) &&
        self.offhand.is_empty()
    }

    /// Check if inventory is full
    pub fn is_full(&self) -> bool {
        self.hotbar.iter().all(|slot| slot.is_full()) &&
        self.main.iter().all(|slot| slot.is_full())
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new()
    }
}