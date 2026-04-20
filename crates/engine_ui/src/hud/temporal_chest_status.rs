//! Temporal chest status HUD display for time-loop survival.
//!
//! Shows the status of temporal chests and their contents.

use serde::{Deserialize, Serialize};

/// A tracked temporal chest.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackedChest {
    /// Chest identifier.
    pub id: u32,
    /// Display name.
    pub name: String,
    /// Number of items stored.
    pub item_count: u32,
    /// Maximum capacity.
    pub capacity: u32,
    /// Distance from player.
    pub distance: f32,
    /// Whether the chest is accessible this loop.
    pub accessible: bool,
}

impl TrackedChest {
    /// Create a new tracked chest.
    #[must_use]
    pub fn new(id: u32, name: &str, capacity: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            item_count: 0,
            capacity,
            distance: 0.0,
            accessible: true,
        }
    }

    /// Get fill percentage.
    #[must_use]
    pub fn fill_percentage(&self) -> f32 {
        if self.capacity == 0 {
            return 0.0;
        }
        (self.item_count as f32 / self.capacity as f32).clamp(0.0, 1.0)
    }

    /// Check if the chest is full.
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.item_count >= self.capacity
    }

    /// Check if the chest is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.item_count == 0
    }
}

/// Temporal chest status HUD element.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TemporalChestStatusDisplay {
    /// All tracked chests.
    chests: Vec<TrackedChest>,
    /// Currently selected chest index.
    selected_index: Option<usize>,
    /// Maximum chests to show in compact view.
    max_compact_display: usize,
    /// Whether to show detailed view.
    detailed_view: bool,
    /// Visibility flag.
    visible: bool,
    /// Pulse animation for chests with items.
    pulse_phase: f32,
}

impl TemporalChestStatusDisplay {
    /// Create a new temporal chest status display.
    #[must_use]
    pub fn new() -> Self {
        Self {
            chests: Vec::new(),
            selected_index: None,
            max_compact_display: 3,
            detailed_view: false,
            visible: true,
            pulse_phase: 0.0,
        }
    }

    /// Get all tracked chests.
    #[must_use]
    pub fn chests(&self) -> &[TrackedChest] {
        &self.chests
    }

    /// Get chest count.
    #[must_use]
    pub fn chest_count(&self) -> usize {
        self.chests.len()
    }

    /// Get total items across all chests.
    #[must_use]
    pub fn total_items(&self) -> u32 {
        self.chests.iter().map(|c| c.item_count).sum()
    }

    /// Get total capacity across all chests.
    #[must_use]
    pub fn total_capacity(&self) -> u32 {
        self.chests.iter().map(|c| c.capacity).sum()
    }

    /// Get currently selected chest.
    #[must_use]
    pub fn selected_chest(&self) -> Option<&TrackedChest> {
        self.selected_index.and_then(|i| self.chests.get(i))
    }

    /// Check if detailed view is enabled.
    #[must_use]
    pub fn detailed_view(&self) -> bool {
        self.detailed_view
    }

    /// Check if visible.
    #[must_use]
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get pulse phase.
    #[must_use]
    pub fn pulse_phase(&self) -> f32 {
        self.pulse_phase
    }

    /// Add a chest to track.
    pub fn add_chest(&mut self, chest: TrackedChest) {
        if !self.chests.iter().any(|c| c.id == chest.id) {
            self.chests.push(chest);
        }
    }

    /// Update a chest's item count.
    pub fn update_chest(&mut self, id: u32, item_count: u32) {
        if let Some(chest) = self.chests.iter_mut().find(|c| c.id == id) {
            chest.item_count = item_count.min(chest.capacity);
        }
    }

    /// Update a chest's distance.
    pub fn update_distance(&mut self, id: u32, distance: f32) {
        if let Some(chest) = self.chests.iter_mut().find(|c| c.id == id) {
            chest.distance = distance;
        }
    }

    /// Update a chest's accessibility.
    pub fn update_accessibility(&mut self, id: u32, accessible: bool) {
        if let Some(chest) = self.chests.iter_mut().find(|c| c.id == id) {
            chest.accessible = accessible;
        }
    }

    /// Remove a chest from tracking.
    pub fn remove_chest(&mut self, id: u32) {
        self.chests.retain(|c| c.id != id);
        self.selected_index = None;
    }

    /// Select a chest by index.
    pub fn select_chest(&mut self, index: usize) {
        if index < self.chests.len() {
            self.selected_index = Some(index);
        }
    }

    /// Select next chest.
    pub fn select_next(&mut self) {
        if self.chests.is_empty() {
            return;
        }
        self.selected_index = Some(match self.selected_index {
            Some(i) => (i + 1) % self.chests.len(),
            None => 0,
        });
    }

    /// Select previous chest.
    pub fn select_previous(&mut self) {
        if self.chests.is_empty() {
            return;
        }
        self.selected_index = Some(match self.selected_index {
            Some(0) => self.chests.len() - 1,
            Some(i) => i - 1,
            None => self.chests.len() - 1,
        });
    }

    /// Clear selection.
    pub fn clear_selection(&mut self) {
        self.selected_index = None;
    }

    /// Set visibility.
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Set detailed view.
    pub fn set_detailed_view(&mut self, detailed: bool) {
        self.detailed_view = detailed;
    }

    /// Toggle detailed view.
    pub fn toggle_detailed_view(&mut self) {
        self.detailed_view = !self.detailed_view;
    }

    /// Update the display.
    pub fn update(&mut self, delta_time: f32) {
        // Update pulse animation
        self.pulse_phase += delta_time * 1.5;
        if self.pulse_phase > std::f32::consts::TAU {
            self.pulse_phase -= std::f32::consts::TAU;
        }

        // Sort chests by distance
        self.chests.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Get display text for compact view.
    #[must_use]
    pub fn compact_text(&self) -> String {
        let total = self.total_items();
        let capacity = self.total_capacity();
        format!("Chests: {}/{}", total, capacity)
    }

    /// Get display entries for the HUD.
    #[must_use]
    pub fn display_entries(&self) -> Vec<&TrackedChest> {
        self.chests.iter().take(self.max_compact_display).collect()
    }

    /// Get the nearest chest with space.
    #[must_use]
    pub fn nearest_available(&self) -> Option<&TrackedChest> {
        self.chests
            .iter()
            .filter(|c| c.accessible && !c.is_full())
            .min_by(|a, b| {
                a.distance
                    .partial_cmp(&b.distance)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    /// Query total items (alias).
    #[must_use]
    pub fn query_items(&self) -> u32 {
        self.total_items()
    }

    /// Clear all tracked chests.
    pub fn clear(&mut self) {
        self.chests.clear();
        self.selected_index = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracked_chest_new() {
        let chest = TrackedChest::new(1, "Main Chest", 20);

        assert_eq!(chest.id, 1);
        assert_eq!(chest.name, "Main Chest");
        assert_eq!(chest.capacity, 20);
        assert_eq!(chest.item_count, 0);
        assert!(chest.accessible);
    }

    #[test]
    fn test_tracked_chest_fill_percentage() {
        let mut chest = TrackedChest::new(1, "Chest", 10);
        chest.item_count = 5;

        assert!((chest.fill_percentage() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_tracked_chest_is_full() {
        let mut chest = TrackedChest::new(1, "Chest", 10);
        assert!(!chest.is_full());

        chest.item_count = 10;
        assert!(chest.is_full());
    }

    #[test]
    fn test_tracked_chest_is_empty() {
        let chest = TrackedChest::new(1, "Chest", 10);
        assert!(chest.is_empty());
    }

    #[test]
    fn test_chest_status_new() {
        let status = TemporalChestStatusDisplay::new();
        assert_eq!(status.chest_count(), 0);
        assert!(status.is_visible());
    }

    #[test]
    fn test_chest_status_add_chest() {
        let mut status = TemporalChestStatusDisplay::new();
        status.add_chest(TrackedChest::new(1, "Chest 1", 10));
        status.add_chest(TrackedChest::new(2, "Chest 2", 20));

        assert_eq!(status.chest_count(), 2);
    }

    #[test]
    fn test_chest_status_no_duplicates() {
        let mut status = TemporalChestStatusDisplay::new();
        status.add_chest(TrackedChest::new(1, "Chest", 10));
        status.add_chest(TrackedChest::new(1, "Chest", 10));

        assert_eq!(status.chest_count(), 1);
    }

    #[test]
    fn test_chest_status_update_chest() {
        let mut status = TemporalChestStatusDisplay::new();
        status.add_chest(TrackedChest::new(1, "Chest", 10));

        status.update_chest(1, 5);
        assert_eq!(status.chests()[0].item_count, 5);
    }

    #[test]
    fn test_chest_status_update_distance() {
        let mut status = TemporalChestStatusDisplay::new();
        status.add_chest(TrackedChest::new(1, "Chest", 10));

        status.update_distance(1, 15.5);
        assert!((status.chests()[0].distance - 15.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_chest_status_update_accessibility() {
        let mut status = TemporalChestStatusDisplay::new();
        status.add_chest(TrackedChest::new(1, "Chest", 10));

        status.update_accessibility(1, false);
        assert!(!status.chests()[0].accessible);
    }

    #[test]
    fn test_chest_status_remove_chest() {
        let mut status = TemporalChestStatusDisplay::new();
        status.add_chest(TrackedChest::new(1, "Chest", 10));
        status.remove_chest(1);

        assert_eq!(status.chest_count(), 0);
    }

    #[test]
    fn test_chest_status_total_items() {
        let mut status = TemporalChestStatusDisplay::new();
        let mut chest1 = TrackedChest::new(1, "Chest 1", 10);
        chest1.item_count = 5;
        let mut chest2 = TrackedChest::new(2, "Chest 2", 10);
        chest2.item_count = 3;

        status.add_chest(chest1);
        status.add_chest(chest2);

        assert_eq!(status.total_items(), 8);
        assert_eq!(status.total_capacity(), 20);
    }

    #[test]
    fn test_chest_status_select_chest() {
        let mut status = TemporalChestStatusDisplay::new();
        status.add_chest(TrackedChest::new(1, "Chest 1", 10));
        status.add_chest(TrackedChest::new(2, "Chest 2", 10));

        status.select_chest(1);
        assert_eq!(status.selected_chest().unwrap().id, 2);
    }

    #[test]
    fn test_chest_status_select_next() {
        let mut status = TemporalChestStatusDisplay::new();
        status.add_chest(TrackedChest::new(1, "Chest 1", 10));
        status.add_chest(TrackedChest::new(2, "Chest 2", 10));

        status.select_next();
        assert_eq!(status.selected_chest().unwrap().id, 1);

        status.select_next();
        assert_eq!(status.selected_chest().unwrap().id, 2);

        status.select_next(); // Wrap around
        assert_eq!(status.selected_chest().unwrap().id, 1);
    }

    #[test]
    fn test_chest_status_select_previous() {
        let mut status = TemporalChestStatusDisplay::new();
        status.add_chest(TrackedChest::new(1, "Chest 1", 10));
        status.add_chest(TrackedChest::new(2, "Chest 2", 10));

        status.select_previous();
        assert_eq!(status.selected_chest().unwrap().id, 2);
    }

    #[test]
    fn test_chest_status_compact_text() {
        let mut status = TemporalChestStatusDisplay::new();
        let mut chest = TrackedChest::new(1, "Chest", 20);
        chest.item_count = 8;
        status.add_chest(chest);

        assert!(status.compact_text().contains("8/20"));
    }

    #[test]
    fn test_chest_status_nearest_available() {
        let mut status = TemporalChestStatusDisplay::new();

        let mut chest1 = TrackedChest::new(1, "Chest 1", 10);
        chest1.distance = 20.0;
        chest1.item_count = 10; // Full

        let mut chest2 = TrackedChest::new(2, "Chest 2", 10);
        chest2.distance = 10.0;

        status.add_chest(chest1);
        status.add_chest(chest2);

        let nearest = status.nearest_available();
        assert!(nearest.is_some());
        assert_eq!(nearest.unwrap().id, 2);
    }

    #[test]
    fn test_chest_status_toggle_detailed() {
        let mut status = TemporalChestStatusDisplay::new();
        assert!(!status.detailed_view());

        status.toggle_detailed_view();
        assert!(status.detailed_view());
    }

    #[test]
    fn test_chest_status_query_items() {
        let mut status = TemporalChestStatusDisplay::new();
        let mut chest = TrackedChest::new(1, "Chest", 10);
        chest.item_count = 7;
        status.add_chest(chest);

        assert_eq!(status.query_items(), 7);
    }

    #[test]
    fn test_chest_status_clear() {
        let mut status = TemporalChestStatusDisplay::new();
        status.add_chest(TrackedChest::new(1, "Chest", 10));
        status.select_chest(0);
        status.clear();

        assert_eq!(status.chest_count(), 0);
        assert!(status.selected_chest().is_none());
    }

    #[test]
    fn test_chest_status_visibility() {
        let mut status = TemporalChestStatusDisplay::new();
        status.set_visible(false);
        assert!(!status.is_visible());
    }
}
