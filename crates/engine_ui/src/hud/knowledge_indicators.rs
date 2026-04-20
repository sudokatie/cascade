//! Knowledge indicators HUD display for time-loop survival.
//!
//! Shows discovered knowledge and hints from previous loops.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Type of knowledge that can be discovered.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KnowledgeType {
    /// Trap location discovered.
    TrapLocation,
    /// Safe path found.
    SafePath,
    /// Item location remembered.
    ItemLocation,
    /// Enemy pattern learned.
    EnemyPattern,
    /// Secret area found.
    SecretArea,
    /// Crafting recipe discovered.
    Recipe,
    /// Shortcut route found.
    Shortcut,
}

impl KnowledgeType {
    /// Get display name.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            KnowledgeType::TrapLocation => "Trap",
            KnowledgeType::SafePath => "Safe Path",
            KnowledgeType::ItemLocation => "Item",
            KnowledgeType::EnemyPattern => "Enemy",
            KnowledgeType::SecretArea => "Secret",
            KnowledgeType::Recipe => "Recipe",
            KnowledgeType::Shortcut => "Shortcut",
        }
    }

    /// Get icon identifier.
    #[must_use]
    pub fn icon(&self) -> &'static str {
        match self {
            KnowledgeType::TrapLocation => "icon_trap",
            KnowledgeType::SafePath => "icon_path",
            KnowledgeType::ItemLocation => "icon_item",
            KnowledgeType::EnemyPattern => "icon_enemy",
            KnowledgeType::SecretArea => "icon_secret",
            KnowledgeType::Recipe => "icon_recipe",
            KnowledgeType::Shortcut => "icon_shortcut",
        }
    }
}

/// A piece of discovered knowledge.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KnowledgeEntry {
    /// Type of knowledge.
    pub knowledge_type: KnowledgeType,
    /// Unique identifier.
    pub id: String,
    /// Display description.
    pub description: String,
    /// Loop when discovered.
    pub discovered_loop: u32,
    /// Whether it's been viewed.
    pub viewed: bool,
}

impl KnowledgeEntry {
    /// Create a new knowledge entry.
    #[must_use]
    pub fn new(knowledge_type: KnowledgeType, id: &str, description: &str, loop_num: u32) -> Self {
        Self {
            knowledge_type,
            id: id.to_string(),
            description: description.to_string(),
            discovered_loop: loop_num,
            viewed: false,
        }
    }
}

/// Knowledge indicators HUD element.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct KnowledgeIndicatorsDisplay {
    /// All discovered knowledge entries.
    entries: Vec<KnowledgeEntry>,
    /// IDs of knowledge that has been viewed.
    viewed_ids: HashSet<String>,
    /// Maximum entries to show in HUD.
    max_display: usize,
    /// Whether to show new indicators.
    show_new: bool,
    /// Visibility flag.
    visible: bool,
    /// Current filter type (None = show all).
    filter: Option<KnowledgeType>,
    /// Animation for new knowledge.
    new_knowledge_pulse: f32,
}

impl KnowledgeIndicatorsDisplay {
    /// Create a new knowledge indicators display.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            viewed_ids: HashSet::new(),
            max_display: 5,
            show_new: true,
            visible: true,
            filter: None,
            new_knowledge_pulse: 0.0,
        }
    }

    /// Get all entries.
    #[must_use]
    pub fn entries(&self) -> &[KnowledgeEntry] {
        &self.entries
    }

    /// Get count of all knowledge.
    #[must_use]
    pub fn total_count(&self) -> usize {
        self.entries.len()
    }

    /// Get count of unviewed knowledge.
    #[must_use]
    pub fn unviewed_count(&self) -> usize {
        self.entries.iter().filter(|e| !e.viewed).count()
    }

    /// Check if visible.
    #[must_use]
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get the current filter.
    #[must_use]
    pub fn filter(&self) -> Option<KnowledgeType> {
        self.filter
    }

    /// Get new knowledge pulse animation.
    #[must_use]
    pub fn new_knowledge_pulse(&self) -> f32 {
        self.new_knowledge_pulse
    }

    /// Add a knowledge entry.
    pub fn add_knowledge(&mut self, entry: KnowledgeEntry) {
        if !self.entries.iter().any(|e| e.id == entry.id) {
            self.entries.push(entry);
            self.new_knowledge_pulse = 1.0;
        }
    }

    /// Mark knowledge as viewed.
    pub fn mark_viewed(&mut self, id: &str) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.viewed = true;
            self.viewed_ids.insert(id.to_string());
        }
    }

    /// Mark all knowledge as viewed.
    pub fn mark_all_viewed(&mut self) {
        for entry in &mut self.entries {
            entry.viewed = true;
            self.viewed_ids.insert(entry.id.clone());
        }
    }

    /// Check if specific knowledge is known.
    #[must_use]
    pub fn has_knowledge(&self, id: &str) -> bool {
        self.entries.iter().any(|e| e.id == id)
    }

    /// Set visibility.
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Set filter.
    pub fn set_filter(&mut self, filter: Option<KnowledgeType>) {
        self.filter = filter;
    }

    /// Set max display count.
    pub fn set_max_display(&mut self, max: usize) {
        self.max_display = max;
    }

    /// Update the display.
    pub fn update(&mut self, delta_time: f32) {
        if self.new_knowledge_pulse > 0.0 {
            self.new_knowledge_pulse = (self.new_knowledge_pulse - delta_time * 2.0).max(0.0);
        }
    }

    /// Get entries to display (filtered and limited).
    #[must_use]
    pub fn display_entries(&self) -> Vec<&KnowledgeEntry> {
        let mut entries: Vec<_> = self
            .entries
            .iter()
            .filter(|e| {
                if let Some(filter) = self.filter {
                    e.knowledge_type == filter
                } else {
                    true
                }
            })
            .collect();

        // Sort by most recent first
        entries.sort_by(|a, b| b.discovered_loop.cmp(&a.discovered_loop));

        // Limit to max display
        entries.truncate(self.max_display);
        entries
    }

    /// Get summary text.
    #[must_use]
    pub fn summary_text(&self) -> String {
        let total = self.total_count();
        let unviewed = self.unviewed_count();
        if unviewed > 0 {
            format!("Knowledge: {} ({} new)", total, unviewed)
        } else {
            format!("Knowledge: {}", total)
        }
    }

    /// Query knowledge count by type.
    #[must_use]
    pub fn query_count_by_type(&self, knowledge_type: KnowledgeType) -> usize {
        self.entries
            .iter()
            .filter(|e| e.knowledge_type == knowledge_type)
            .count()
    }

    /// Clear all knowledge (for testing).
    pub fn clear(&mut self) {
        self.entries.clear();
        self.viewed_ids.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knowledge_type_display_names() {
        assert_eq!(KnowledgeType::TrapLocation.display_name(), "Trap");
        assert_eq!(KnowledgeType::SafePath.display_name(), "Safe Path");
        assert_eq!(KnowledgeType::SecretArea.display_name(), "Secret");
    }

    #[test]
    fn test_knowledge_type_icons() {
        assert!(KnowledgeType::TrapLocation.icon().contains("trap"));
        assert!(KnowledgeType::Recipe.icon().contains("recipe"));
    }

    #[test]
    fn test_knowledge_entry_new() {
        let entry = KnowledgeEntry::new(
            KnowledgeType::TrapLocation,
            "trap_1",
            "A spike trap in the corridor",
            3,
        );

        assert_eq!(entry.knowledge_type, KnowledgeType::TrapLocation);
        assert_eq!(entry.id, "trap_1");
        assert_eq!(entry.discovered_loop, 3);
        assert!(!entry.viewed);
    }

    #[test]
    fn test_knowledge_indicators_new() {
        let display = KnowledgeIndicatorsDisplay::new();
        assert_eq!(display.total_count(), 0);
        assert!(display.is_visible());
    }

    #[test]
    fn test_knowledge_indicators_add() {
        let mut display = KnowledgeIndicatorsDisplay::new();
        let entry = KnowledgeEntry::new(KnowledgeType::SafePath, "path_1", "Safe path", 1);

        display.add_knowledge(entry);
        assert_eq!(display.total_count(), 1);
    }

    #[test]
    fn test_knowledge_indicators_no_duplicates() {
        let mut display = KnowledgeIndicatorsDisplay::new();

        display.add_knowledge(KnowledgeEntry::new(
            KnowledgeType::TrapLocation,
            "trap_1",
            "Trap",
            1,
        ));
        display.add_knowledge(KnowledgeEntry::new(
            KnowledgeType::TrapLocation,
            "trap_1",
            "Trap",
            2,
        ));

        assert_eq!(display.total_count(), 1);
    }

    #[test]
    fn test_knowledge_indicators_unviewed_count() {
        let mut display = KnowledgeIndicatorsDisplay::new();

        display.add_knowledge(KnowledgeEntry::new(
            KnowledgeType::ItemLocation,
            "item_1",
            "Item",
            1,
        ));
        display.add_knowledge(KnowledgeEntry::new(
            KnowledgeType::ItemLocation,
            "item_2",
            "Item",
            2,
        ));

        assert_eq!(display.unviewed_count(), 2);

        display.mark_viewed("item_1");
        assert_eq!(display.unviewed_count(), 1);
    }

    #[test]
    fn test_knowledge_indicators_mark_all_viewed() {
        let mut display = KnowledgeIndicatorsDisplay::new();

        display.add_knowledge(KnowledgeEntry::new(
            KnowledgeType::Recipe,
            "recipe_1",
            "Recipe",
            1,
        ));
        display.add_knowledge(KnowledgeEntry::new(
            KnowledgeType::Recipe,
            "recipe_2",
            "Recipe",
            2,
        ));

        display.mark_all_viewed();
        assert_eq!(display.unviewed_count(), 0);
    }

    #[test]
    fn test_knowledge_indicators_has_knowledge() {
        let mut display = KnowledgeIndicatorsDisplay::new();

        display.add_knowledge(KnowledgeEntry::new(
            KnowledgeType::SecretArea,
            "secret_1",
            "Secret",
            1,
        ));

        assert!(display.has_knowledge("secret_1"));
        assert!(!display.has_knowledge("secret_2"));
    }

    #[test]
    fn test_knowledge_indicators_filter() {
        let mut display = KnowledgeIndicatorsDisplay::new();

        display.add_knowledge(KnowledgeEntry::new(
            KnowledgeType::TrapLocation,
            "trap_1",
            "Trap",
            1,
        ));
        display.add_knowledge(KnowledgeEntry::new(
            KnowledgeType::Recipe,
            "recipe_1",
            "Recipe",
            2,
        ));

        display.set_filter(Some(KnowledgeType::TrapLocation));
        let entries = display.display_entries();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].knowledge_type, KnowledgeType::TrapLocation);
    }

    #[test]
    fn test_knowledge_indicators_display_limit() {
        let mut display = KnowledgeIndicatorsDisplay::new();
        display.set_max_display(2);

        for i in 0..5 {
            display.add_knowledge(KnowledgeEntry::new(
                KnowledgeType::Shortcut,
                &format!("shortcut_{}", i),
                "Shortcut",
                i as u32,
            ));
        }

        let entries = display.display_entries();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_knowledge_indicators_summary_text() {
        let mut display = KnowledgeIndicatorsDisplay::new();

        display.add_knowledge(KnowledgeEntry::new(
            KnowledgeType::EnemyPattern,
            "enemy_1",
            "Enemy",
            1,
        ));

        assert!(display.summary_text().contains("1"));
        assert!(display.summary_text().contains("new"));
    }

    #[test]
    fn test_knowledge_indicators_query_by_type() {
        let mut display = KnowledgeIndicatorsDisplay::new();

        display.add_knowledge(KnowledgeEntry::new(
            KnowledgeType::TrapLocation,
            "trap_1",
            "Trap",
            1,
        ));
        display.add_knowledge(KnowledgeEntry::new(
            KnowledgeType::TrapLocation,
            "trap_2",
            "Trap",
            2,
        ));
        display.add_knowledge(KnowledgeEntry::new(
            KnowledgeType::Recipe,
            "recipe_1",
            "Recipe",
            3,
        ));

        assert_eq!(display.query_count_by_type(KnowledgeType::TrapLocation), 2);
        assert_eq!(display.query_count_by_type(KnowledgeType::Recipe), 1);
    }

    #[test]
    fn test_knowledge_indicators_update_pulse() {
        let mut display = KnowledgeIndicatorsDisplay::new();

        display.add_knowledge(KnowledgeEntry::new(
            KnowledgeType::SafePath,
            "path_1",
            "Path",
            1,
        ));
        assert!((display.new_knowledge_pulse() - 1.0).abs() < f32::EPSILON);

        display.update(0.5);
        assert!(display.new_knowledge_pulse() < 1.0);
    }

    #[test]
    fn test_knowledge_indicators_clear() {
        let mut display = KnowledgeIndicatorsDisplay::new();

        display.add_knowledge(KnowledgeEntry::new(
            KnowledgeType::ItemLocation,
            "item_1",
            "Item",
            1,
        ));
        display.clear();

        assert_eq!(display.total_count(), 0);
    }

    #[test]
    fn test_knowledge_indicators_visibility() {
        let mut display = KnowledgeIndicatorsDisplay::new();
        display.set_visible(false);
        assert!(!display.is_visible());
    }
}
