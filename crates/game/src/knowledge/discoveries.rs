//! Discovery and knowledge tracking system.
//!
//! Players permanently learn map locations, recipes, trap patterns,
//! creature behaviors, and escape routes across loops.

use std::collections::HashSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Categories of knowledge that can be discovered.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KnowledgeCategory {
    /// Map locations and landmarks.
    Map,
    /// Crafting recipes.
    Recipe,
    /// Trap patterns and locations.
    Trap,
    /// Creature behaviors and weaknesses.
    Creature,
    /// Safe routes and escape paths.
    Route,
}

impl fmt::Display for KnowledgeCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KnowledgeCategory::Map => write!(f, "Map"),
            KnowledgeCategory::Recipe => write!(f, "Recipe"),
            KnowledgeCategory::Trap => write!(f, "Trap"),
            KnowledgeCategory::Creature => write!(f, "Creature"),
            KnowledgeCategory::Route => write!(f, "Route"),
        }
    }
}

impl KnowledgeCategory {
    /// Get all knowledge categories.
    #[must_use]
    pub fn all() -> &'static [KnowledgeCategory] {
        &[
            KnowledgeCategory::Map,
            KnowledgeCategory::Recipe,
            KnowledgeCategory::Trap,
            KnowledgeCategory::Creature,
            KnowledgeCategory::Route,
        ]
    }
}

/// Unique identifier for a discovery.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DiscoveryID {
    /// Category of this discovery.
    pub category: KnowledgeCategory,
    /// Unique ID within the category.
    pub id: u32,
}

impl DiscoveryID {
    /// Create a new discovery ID.
    #[must_use]
    pub fn new(category: KnowledgeCategory, id: u32) -> Self {
        Self { category, id }
    }
}

/// Tracks all player discoveries across loops.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct KnowledgeSystem {
    /// Set of all discovered items.
    discoveries: HashSet<DiscoveryID>,
}

impl KnowledgeSystem {
    /// Create a new knowledge system.
    #[must_use]
    pub fn new() -> Self {
        Self {
            discoveries: HashSet::new(),
        }
    }

    /// Discover something new.
    ///
    /// Returns true if this was a new discovery, false if already known.
    pub fn discover(&mut self, id: DiscoveryID) -> bool {
        self.discoveries.insert(id)
    }

    /// Check if something has been discovered.
    #[must_use]
    pub fn is_discovered(&self, id: DiscoveryID) -> bool {
        self.discoveries.contains(&id)
    }

    /// Get all discoveries in a category.
    #[must_use]
    pub fn discoveries_for_category(&self, cat: KnowledgeCategory) -> Vec<DiscoveryID> {
        self.discoveries
            .iter()
            .filter(|d| d.category == cat)
            .copied()
            .collect()
    }

    /// Get total number of discoveries.
    #[must_use]
    pub fn total_discoveries(&self) -> usize {
        self.discoveries.len()
    }

    /// Get count of discoveries in a specific category.
    #[must_use]
    pub fn category_count(&self, cat: KnowledgeCategory) -> usize {
        self.discoveries.iter().filter(|d| d.category == cat).count()
    }

    /// Get all discoveries.
    #[must_use]
    pub fn all_discoveries(&self) -> impl Iterator<Item = &DiscoveryID> {
        self.discoveries.iter()
    }

    /// Check if any knowledge has been gained.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.discoveries.is_empty()
    }

    /// Clear all discoveries (for testing or new game).
    pub fn clear(&mut self) {
        self.discoveries.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knowledge_category_display() {
        assert_eq!(format!("{}", KnowledgeCategory::Map), "Map");
        assert_eq!(format!("{}", KnowledgeCategory::Recipe), "Recipe");
        assert_eq!(format!("{}", KnowledgeCategory::Trap), "Trap");
        assert_eq!(format!("{}", KnowledgeCategory::Creature), "Creature");
        assert_eq!(format!("{}", KnowledgeCategory::Route), "Route");
    }

    #[test]
    fn test_knowledge_category_all() {
        let all = KnowledgeCategory::all();
        assert_eq!(all.len(), 5);
    }

    #[test]
    fn test_discovery_id_new() {
        let id = DiscoveryID::new(KnowledgeCategory::Map, 42);
        assert_eq!(id.category, KnowledgeCategory::Map);
        assert_eq!(id.id, 42);
    }

    #[test]
    fn test_knowledge_system_new() {
        let system = KnowledgeSystem::new();
        assert_eq!(system.total_discoveries(), 0);
        assert!(system.is_empty());
    }

    #[test]
    fn test_knowledge_system_discover() {
        let mut system = KnowledgeSystem::new();
        let id = DiscoveryID::new(KnowledgeCategory::Recipe, 1);

        let was_new = system.discover(id);
        assert!(was_new);
        assert!(!system.is_empty());
        assert_eq!(system.total_discoveries(), 1);
    }

    #[test]
    fn test_knowledge_system_discover_duplicate() {
        let mut system = KnowledgeSystem::new();
        let id = DiscoveryID::new(KnowledgeCategory::Trap, 5);

        system.discover(id);
        let was_new = system.discover(id);

        assert!(!was_new);
        assert_eq!(system.total_discoveries(), 1);
    }

    #[test]
    fn test_knowledge_system_is_discovered() {
        let mut system = KnowledgeSystem::new();
        let id = DiscoveryID::new(KnowledgeCategory::Creature, 10);

        assert!(!system.is_discovered(id));
        system.discover(id);
        assert!(system.is_discovered(id));
    }

    #[test]
    fn test_knowledge_system_discoveries_for_category() {
        let mut system = KnowledgeSystem::new();
        system.discover(DiscoveryID::new(KnowledgeCategory::Map, 1));
        system.discover(DiscoveryID::new(KnowledgeCategory::Map, 2));
        system.discover(DiscoveryID::new(KnowledgeCategory::Recipe, 1));

        let map_discoveries = system.discoveries_for_category(KnowledgeCategory::Map);
        assert_eq!(map_discoveries.len(), 2);

        let recipe_discoveries = system.discoveries_for_category(KnowledgeCategory::Recipe);
        assert_eq!(recipe_discoveries.len(), 1);
    }

    #[test]
    fn test_knowledge_system_category_count() {
        let mut system = KnowledgeSystem::new();
        system.discover(DiscoveryID::new(KnowledgeCategory::Route, 1));
        system.discover(DiscoveryID::new(KnowledgeCategory::Route, 2));
        system.discover(DiscoveryID::new(KnowledgeCategory::Route, 3));

        assert_eq!(system.category_count(KnowledgeCategory::Route), 3);
        assert_eq!(system.category_count(KnowledgeCategory::Trap), 0);
    }

    #[test]
    fn test_knowledge_system_clear() {
        let mut system = KnowledgeSystem::new();
        system.discover(DiscoveryID::new(KnowledgeCategory::Map, 1));
        system.discover(DiscoveryID::new(KnowledgeCategory::Recipe, 1));

        system.clear();
        assert!(system.is_empty());
        assert_eq!(system.total_discoveries(), 0);
    }

    #[test]
    fn test_knowledge_system_all_discoveries() {
        let mut system = KnowledgeSystem::new();
        system.discover(DiscoveryID::new(KnowledgeCategory::Map, 1));
        system.discover(DiscoveryID::new(KnowledgeCategory::Recipe, 1));

        let all: Vec<_> = system.all_discoveries().collect();
        assert_eq!(all.len(), 2);
    }
}
