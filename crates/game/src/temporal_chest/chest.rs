//! Temporal chest implementation for cross-loop item storage.
//!
//! Items stored in temporal chests persist across time loop resets,
//! with ghost previews showing what was stored in the previous loop.

use glam::IVec3;
use serde::{Deserialize, Serialize};

/// Number of slots in a temporal chest.
pub const CHEST_SLOTS: usize = 27;

/// A stack of items for the temporal chest system.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemStack {
    /// Type identifier for the item.
    pub item_type: String,
    /// Number of items in the stack.
    pub quantity: u32,
}

impl ItemStack {
    /// Create a new item stack.
    #[must_use]
    pub fn new(item_type: impl Into<String>, quantity: u32) -> Self {
        Self {
            item_type: item_type.into(),
            quantity,
        }
    }

    /// Check if this stack is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.quantity == 0
    }
}

/// A chest that preserves contents across time loops.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TemporalChest {
    /// Current contents of the chest.
    contents: Vec<Option<ItemStack>>,
    /// Ghost preview showing previous loop contents.
    ghost_preview: Vec<Option<ItemStack>>,
    /// World position of the chest.
    position: IVec3,
}

impl TemporalChest {
    /// Create a new temporal chest at the given position.
    #[must_use]
    pub fn new(position: IVec3) -> Self {
        Self {
            contents: vec![None; CHEST_SLOTS],
            ghost_preview: vec![None; CHEST_SLOTS],
            position,
        }
    }

    /// Get the chest contents.
    #[must_use]
    pub fn contents(&self) -> &[Option<ItemStack>] {
        &self.contents
    }

    /// Get the ghost preview (previous loop contents).
    #[must_use]
    pub fn ghost_preview(&self) -> &[Option<ItemStack>] {
        &self.ghost_preview
    }

    /// Insert an item into a specific slot.
    ///
    /// Returns true if successful, false if slot is out of bounds.
    pub fn insert(&mut self, slot: usize, item: ItemStack) -> bool {
        if slot >= CHEST_SLOTS {
            return false;
        }
        self.contents[slot] = Some(item);
        true
    }

    /// Remove an item from a specific slot.
    ///
    /// Returns the removed item, or None if slot was empty or invalid.
    pub fn remove(&mut self, slot: usize) -> Option<ItemStack> {
        if slot >= CHEST_SLOTS {
            return None;
        }
        self.contents[slot].take()
    }

    /// Persist contents across a loop reset.
    ///
    /// Saves current contents as ghost preview while keeping actual contents.
    pub fn persist_across_loop(&mut self) {
        self.ghost_preview = self.contents.clone();
    }

    /// Get the number of slots in the chest.
    #[must_use]
    pub fn slot_count(&self) -> usize {
        CHEST_SLOTS
    }

    /// Check if the chest is completely empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.contents.iter().all(|s| s.is_none())
    }

    /// Get the world position of this chest.
    #[must_use]
    pub fn position(&self) -> IVec3 {
        self.position
    }

    /// Get a specific slot's contents.
    #[must_use]
    pub fn get_slot(&self, slot: usize) -> Option<&ItemStack> {
        if slot >= CHEST_SLOTS {
            return None;
        }
        self.contents[slot].as_ref()
    }

    /// Count non-empty slots.
    #[must_use]
    pub fn used_slots(&self) -> usize {
        self.contents.iter().filter(|s| s.is_some()).count()
    }

    /// Find first empty slot.
    #[must_use]
    pub fn first_empty_slot(&self) -> Option<usize> {
        self.contents.iter().position(|s| s.is_none())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_stack_new() {
        let stack = ItemStack::new("iron_sword", 1);
        assert_eq!(stack.item_type, "iron_sword");
        assert_eq!(stack.quantity, 1);
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_item_stack_empty() {
        let stack = ItemStack::new("nothing", 0);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_chest_new() {
        let chest = TemporalChest::new(IVec3::new(10, 20, 30));
        assert_eq!(chest.position(), IVec3::new(10, 20, 30));
        assert_eq!(chest.slot_count(), CHEST_SLOTS);
        assert!(chest.is_empty());
    }

    #[test]
    fn test_chest_insert() {
        let mut chest = TemporalChest::new(IVec3::ZERO);
        let item = ItemStack::new("diamond", 5);

        let success = chest.insert(0, item);
        assert!(success);
        assert!(!chest.is_empty());
        assert_eq!(chest.used_slots(), 1);
    }

    #[test]
    fn test_chest_insert_out_of_bounds() {
        let mut chest = TemporalChest::new(IVec3::ZERO);
        let item = ItemStack::new("diamond", 5);

        let success = chest.insert(100, item);
        assert!(!success);
    }

    #[test]
    fn test_chest_remove() {
        let mut chest = TemporalChest::new(IVec3::ZERO);
        chest.insert(5, ItemStack::new("gold", 10));

        let removed = chest.remove(5);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().item_type, "gold");
        assert!(chest.is_empty());
    }

    #[test]
    fn test_chest_remove_empty_slot() {
        let mut chest = TemporalChest::new(IVec3::ZERO);
        let removed = chest.remove(0);
        assert!(removed.is_none());
    }

    #[test]
    fn test_chest_persist_across_loop() {
        let mut chest = TemporalChest::new(IVec3::ZERO);
        chest.insert(0, ItemStack::new("ancient_key", 1));
        chest.insert(1, ItemStack::new("time_crystal", 3));

        chest.persist_across_loop();

        // Ghost preview should match contents
        let ghost = chest.ghost_preview();
        assert!(ghost[0].is_some());
        assert_eq!(ghost[0].as_ref().unwrap().item_type, "ancient_key");
        assert!(ghost[1].is_some());
        assert_eq!(ghost[1].as_ref().unwrap().item_type, "time_crystal");
    }

    #[test]
    fn test_chest_contents() {
        let mut chest = TemporalChest::new(IVec3::ZERO);
        chest.insert(0, ItemStack::new("item1", 1));

        let contents = chest.contents();
        assert_eq!(contents.len(), CHEST_SLOTS);
        assert!(contents[0].is_some());
    }

    #[test]
    fn test_chest_get_slot() {
        let mut chest = TemporalChest::new(IVec3::ZERO);
        chest.insert(3, ItemStack::new("potion", 5));

        assert!(chest.get_slot(3).is_some());
        assert_eq!(chest.get_slot(3).unwrap().quantity, 5);
        assert!(chest.get_slot(0).is_none());
        assert!(chest.get_slot(100).is_none());
    }

    #[test]
    fn test_chest_first_empty_slot() {
        let mut chest = TemporalChest::new(IVec3::ZERO);
        assert_eq!(chest.first_empty_slot(), Some(0));

        chest.insert(0, ItemStack::new("item", 1));
        assert_eq!(chest.first_empty_slot(), Some(1));

        chest.insert(1, ItemStack::new("item", 1));
        assert_eq!(chest.first_empty_slot(), Some(2));
    }
}
