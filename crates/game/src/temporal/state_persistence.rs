//! State persistence system for managing what survives between loops.
//!
//! Different types of game state have different persistence rules:
//! - Persistent: Survives all loops (player knowledge, messages)
//! - SemiPersistent: 50% chance to regenerate each loop (resources)
//! - Volatile: Resets every loop (creature positions, weather)

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Categories of state persistence.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateCategory {
    /// State that survives all loops.
    Persistent,
    /// State that has 50% chance to regenerate each loop.
    SemiPersistent,
    /// State that resets every loop.
    Volatile,
}

impl fmt::Display for StateCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateCategory::Persistent => write!(f, "Persistent"),
            StateCategory::SemiPersistent => write!(f, "SemiPersistent"),
            StateCategory::Volatile => write!(f, "Volatile"),
        }
    }
}

/// Types of game state that can be tracked.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateType {
    /// Terrain changes made by player.
    TerrainModification,
    /// Player-built structures.
    BuiltStructure,
    /// Contents of temporal chests.
    TemporalChestContents,
    /// Warning signs and messages.
    Messages,
    /// Discovered knowledge.
    Knowledge,
    /// Depleted resource nodes.
    ResourceDepletion,
    /// Killed creatures.
    CreatureDeaths,
    /// Current creature positions.
    CreaturePositions,
    /// Weather state.
    Weather,
    /// Trap states (armed/disarmed).
    TrapStates,
}

impl fmt::Display for StateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateType::TerrainModification => write!(f, "TerrainModification"),
            StateType::BuiltStructure => write!(f, "BuiltStructure"),
            StateType::TemporalChestContents => write!(f, "TemporalChestContents"),
            StateType::Messages => write!(f, "Messages"),
            StateType::Knowledge => write!(f, "Knowledge"),
            StateType::ResourceDepletion => write!(f, "ResourceDepletion"),
            StateType::CreatureDeaths => write!(f, "CreatureDeaths"),
            StateType::CreaturePositions => write!(f, "CreaturePositions"),
            StateType::Weather => write!(f, "Weather"),
            StateType::TrapStates => write!(f, "TrapStates"),
        }
    }
}

/// Manages state persistence across time loops.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatePersistence {
    /// Maps each state type to its persistence category.
    category_map: HashMap<StateType, StateCategory>,
    /// Binary blob of persistent data.
    persistent_data: Vec<u8>,
    /// Markers for volatile state that needs reset.
    volatile_markers: Vec<StateType>,
    /// Markers for semi-persistent state needing regeneration check.
    semi_markers: Vec<StateType>,
}

impl StatePersistence {
    /// Create a new state persistence manager with default category mappings.
    #[must_use]
    pub fn new() -> Self {
        let mut category_map = HashMap::new();

        // Persistent states
        category_map.insert(StateType::TemporalChestContents, StateCategory::Persistent);
        category_map.insert(StateType::Messages, StateCategory::Persistent);
        category_map.insert(StateType::Knowledge, StateCategory::Persistent);

        // Semi-persistent states
        category_map.insert(StateType::TerrainModification, StateCategory::SemiPersistent);
        category_map.insert(StateType::BuiltStructure, StateCategory::SemiPersistent);
        category_map.insert(StateType::ResourceDepletion, StateCategory::SemiPersistent);
        category_map.insert(StateType::TrapStates, StateCategory::SemiPersistent);

        // Volatile states
        category_map.insert(StateType::CreatureDeaths, StateCategory::Volatile);
        category_map.insert(StateType::CreaturePositions, StateCategory::Volatile);
        category_map.insert(StateType::Weather, StateCategory::Volatile);

        Self {
            category_map,
            persistent_data: Vec::new(),
            volatile_markers: Vec::new(),
            semi_markers: Vec::new(),
        }
    }

    /// Get the persistence category for a state type.
    #[must_use]
    pub fn category_of(&self, state_type: StateType) -> StateCategory {
        self.category_map
            .get(&state_type)
            .copied()
            .unwrap_or(StateCategory::Volatile)
    }

    /// Save data to persistent storage.
    pub fn save_persistent(&mut self, data: &[u8]) {
        self.persistent_data = data.to_vec();
    }

    /// Load data from persistent storage.
    #[must_use]
    pub fn load_persistent(&self) -> &[u8] {
        &self.persistent_data
    }

    /// Clear volatile state markers.
    pub fn reset_volatile(&mut self) {
        self.volatile_markers.clear();
    }

    /// Mark semi-persistent states for 50% regeneration.
    pub fn regenerate_semi(&mut self) {
        self.semi_markers.clear();
        // The actual regeneration logic would use random chance
        // This just marks them as needing the check
        for (state_type, category) in &self.category_map {
            if *category == StateCategory::SemiPersistent {
                self.semi_markers.push(*state_type);
            }
        }
    }

    /// Get all state types that need volatile reset.
    #[must_use]
    pub fn volatile_state_types(&self) -> Vec<StateType> {
        self.category_map
            .iter()
            .filter(|(_, cat)| **cat == StateCategory::Volatile)
            .map(|(st, _)| *st)
            .collect()
    }

    /// Get all state types that are persistent.
    #[must_use]
    pub fn persistent_state_types(&self) -> Vec<StateType> {
        self.category_map
            .iter()
            .filter(|(_, cat)| **cat == StateCategory::Persistent)
            .map(|(st, _)| *st)
            .collect()
    }

    /// Check if persistent data is empty.
    #[must_use]
    pub fn is_persistent_empty(&self) -> bool {
        self.persistent_data.is_empty()
    }
}

impl Default for StatePersistence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_category_display() {
        assert_eq!(format!("{}", StateCategory::Persistent), "Persistent");
        assert_eq!(format!("{}", StateCategory::SemiPersistent), "SemiPersistent");
        assert_eq!(format!("{}", StateCategory::Volatile), "Volatile");
    }

    #[test]
    fn test_state_type_display() {
        assert_eq!(
            format!("{}", StateType::TerrainModification),
            "TerrainModification"
        );
        assert_eq!(format!("{}", StateType::Messages), "Messages");
        assert_eq!(format!("{}", StateType::Weather), "Weather");
    }

    #[test]
    fn test_persistence_new() {
        let persistence = StatePersistence::new();
        assert!(persistence.is_persistent_empty());
    }

    #[test]
    fn test_category_of_persistent() {
        let persistence = StatePersistence::new();
        assert_eq!(
            persistence.category_of(StateType::TemporalChestContents),
            StateCategory::Persistent
        );
        assert_eq!(
            persistence.category_of(StateType::Messages),
            StateCategory::Persistent
        );
        assert_eq!(
            persistence.category_of(StateType::Knowledge),
            StateCategory::Persistent
        );
    }

    #[test]
    fn test_category_of_semi_persistent() {
        let persistence = StatePersistence::new();
        assert_eq!(
            persistence.category_of(StateType::TerrainModification),
            StateCategory::SemiPersistent
        );
        assert_eq!(
            persistence.category_of(StateType::BuiltStructure),
            StateCategory::SemiPersistent
        );
        assert_eq!(
            persistence.category_of(StateType::ResourceDepletion),
            StateCategory::SemiPersistent
        );
    }

    #[test]
    fn test_category_of_volatile() {
        let persistence = StatePersistence::new();
        assert_eq!(
            persistence.category_of(StateType::CreatureDeaths),
            StateCategory::Volatile
        );
        assert_eq!(
            persistence.category_of(StateType::CreaturePositions),
            StateCategory::Volatile
        );
        assert_eq!(
            persistence.category_of(StateType::Weather),
            StateCategory::Volatile
        );
    }

    #[test]
    fn test_save_load_persistent() {
        let mut persistence = StatePersistence::new();
        let data = vec![1, 2, 3, 4, 5];

        persistence.save_persistent(&data);
        let loaded = persistence.load_persistent();

        assert_eq!(loaded, &[1, 2, 3, 4, 5]);
        assert!(!persistence.is_persistent_empty());
    }

    #[test]
    fn test_reset_volatile() {
        let mut persistence = StatePersistence::new();
        persistence.volatile_markers.push(StateType::Weather);
        persistence.volatile_markers.push(StateType::CreaturePositions);

        persistence.reset_volatile();
        assert!(persistence.volatile_markers.is_empty());
    }

    #[test]
    fn test_regenerate_semi() {
        let mut persistence = StatePersistence::new();
        persistence.regenerate_semi();

        assert!(!persistence.semi_markers.is_empty());
        assert!(persistence.semi_markers.contains(&StateType::TerrainModification));
    }

    #[test]
    fn test_volatile_state_types() {
        let persistence = StatePersistence::new();
        let volatile = persistence.volatile_state_types();

        assert!(volatile.contains(&StateType::CreatureDeaths));
        assert!(volatile.contains(&StateType::CreaturePositions));
        assert!(volatile.contains(&StateType::Weather));
        assert!(!volatile.contains(&StateType::Messages));
    }

    #[test]
    fn test_persistent_state_types() {
        let persistence = StatePersistence::new();
        let persistent = persistence.persistent_state_types();

        assert!(persistent.contains(&StateType::TemporalChestContents));
        assert!(persistent.contains(&StateType::Messages));
        assert!(persistent.contains(&StateType::Knowledge));
        assert!(!persistent.contains(&StateType::Weather));
    }
}
