//! Loop-aware trap types for time-loop survival.
//!
//! Provides traps that persist knowledge across loops.

use glam::IVec3;
use serde::{Deserialize, Serialize};

/// A trap that the player can discover and remember across loops.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoopTrap {
    /// Position in the world.
    position: IVec3,
    /// Whether the trap has been discovered this loop.
    discovered: bool,
    /// Difficulty to disarm (1-10 scale).
    disarm_difficulty: u32,
    /// Trap identifier for cross-loop tracking.
    trap_id: u32,
    /// Whether permanently disarmed (persists across loops).
    permanently_disarmed: bool,
    /// Number of times triggered across all loops.
    trigger_count: u32,
    /// Damage dealt by this trap.
    damage: u32,
}

impl LoopTrap {
    /// Create a new loop trap.
    #[must_use]
    pub fn new(position: IVec3, disarm_difficulty: u32, trap_id: u32) -> Self {
        Self {
            position,
            discovered: false,
            disarm_difficulty: disarm_difficulty.clamp(1, 10),
            trap_id,
            permanently_disarmed: false,
            trigger_count: 0,
            damage: 20,
        }
    }

    /// Create a new loop trap with custom damage.
    #[must_use]
    pub fn with_damage(position: IVec3, disarm_difficulty: u32, trap_id: u32, damage: u32) -> Self {
        let mut trap = Self::new(position, disarm_difficulty, trap_id);
        trap.damage = damage;
        trap
    }

    /// Get the position.
    #[must_use]
    pub fn position(&self) -> IVec3 {
        self.position
    }

    /// Check if discovered this loop.
    #[must_use]
    pub fn is_discovered(&self) -> bool {
        self.discovered
    }

    /// Get the disarm difficulty.
    #[must_use]
    pub fn disarm_difficulty(&self) -> u32 {
        self.disarm_difficulty
    }

    /// Get the trap ID for cross-loop tracking.
    #[must_use]
    pub fn trap_id(&self) -> u32 {
        self.trap_id
    }

    /// Check if permanently disarmed.
    #[must_use]
    pub fn is_permanently_disarmed(&self) -> bool {
        self.permanently_disarmed
    }

    /// Get the number of times triggered.
    #[must_use]
    pub fn trigger_count(&self) -> u32 {
        self.trigger_count
    }

    /// Get the base damage.
    #[must_use]
    pub fn damage(&self) -> u32 {
        self.damage
    }

    /// Mark as discovered.
    pub fn discover(&mut self) {
        self.discovered = true;
    }

    /// Trigger the trap.
    ///
    /// Returns damage dealt, or 0 if disarmed.
    pub fn trigger(&mut self) -> u32 {
        if self.permanently_disarmed {
            return 0;
        }
        self.trigger_count += 1;
        self.damage
    }

    /// Attempt to disarm the trap.
    ///
    /// Returns true if successful based on skill level.
    pub fn attempt_disarm(&mut self, skill_level: u32) -> bool {
        if self.permanently_disarmed {
            return true;
        }
        if skill_level >= self.disarm_difficulty {
            self.permanently_disarmed = true;
            true
        } else {
            false
        }
    }

    /// Reset for a new loop (discovered resets, permanent state persists).
    pub fn reset_for_loop(&mut self) {
        self.discovered = false;
    }

    /// Check if player has prior knowledge of this trap.
    #[must_use]
    pub fn has_prior_knowledge(&self) -> bool {
        self.trigger_count > 0 || self.permanently_disarmed
    }

    /// Get hint text based on trigger history.
    #[must_use]
    pub fn get_hint(&self) -> Option<&'static str> {
        if self.permanently_disarmed {
            Some("This trap has been permanently disarmed")
        } else if self.trigger_count >= 3 {
            Some("You've been hurt by this trap many times...")
        } else if self.trigger_count >= 1 {
            Some("You remember this trap from a previous loop")
        } else {
            None
        }
    }

    /// Calculate effective damage based on loop knowledge.
    #[must_use]
    pub fn effective_damage(&self, has_knowledge: bool) -> u32 {
        if self.permanently_disarmed {
            return 0;
        }
        if has_knowledge {
            // Knowledge reduces damage by 50%
            self.damage / 2
        } else {
            self.damage
        }
    }
}

/// Manager for tracking loop traps across loops.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LoopTrapRegistry {
    /// All known traps.
    traps: Vec<LoopTrap>,
    /// IDs of discovered traps (persists across loops).
    known_trap_ids: Vec<u32>,
}

impl LoopTrapRegistry {
    /// Create a new registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a trap to the registry.
    pub fn add_trap(&mut self, trap: LoopTrap) {
        self.traps.push(trap);
    }

    /// Get a trap by ID.
    #[must_use]
    pub fn get_trap(&self, trap_id: u32) -> Option<&LoopTrap> {
        self.traps.iter().find(|t| t.trap_id == trap_id)
    }

    /// Get a mutable trap by ID.
    pub fn get_trap_mut(&mut self, trap_id: u32) -> Option<&mut LoopTrap> {
        self.traps.iter_mut().find(|t| t.trap_id == trap_id)
    }

    /// Check if a trap is known from previous loops.
    #[must_use]
    pub fn is_trap_known(&self, trap_id: u32) -> bool {
        self.known_trap_ids.contains(&trap_id)
    }

    /// Mark a trap as known.
    pub fn mark_trap_known(&mut self, trap_id: u32) {
        if !self.known_trap_ids.contains(&trap_id) {
            self.known_trap_ids.push(trap_id);
        }
    }

    /// Get all traps.
    #[must_use]
    pub fn all_traps(&self) -> &[LoopTrap] {
        &self.traps
    }

    /// Get the count of known traps.
    #[must_use]
    pub fn known_count(&self) -> usize {
        self.known_trap_ids.len()
    }

    /// Reset all traps for a new loop.
    pub fn reset_for_loop(&mut self) {
        for trap in &mut self.traps {
            trap.reset_for_loop();
        }
    }

    /// Get traps near a position.
    #[must_use]
    pub fn traps_near(&self, pos: IVec3, radius: i32) -> Vec<&LoopTrap> {
        self.traps
            .iter()
            .filter(|trap| {
                let diff = trap.position - pos;
                diff.x.abs() <= radius && diff.y.abs() <= radius && diff.z.abs() <= radius
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_trap_new() {
        let trap = LoopTrap::new(IVec3::new(5, 0, 10), 3, 1);

        assert_eq!(trap.position(), IVec3::new(5, 0, 10));
        assert!(!trap.is_discovered());
        assert_eq!(trap.disarm_difficulty(), 3);
        assert_eq!(trap.trap_id(), 1);
        assert!(!trap.is_permanently_disarmed());
        assert_eq!(trap.trigger_count(), 0);
    }

    #[test]
    fn test_loop_trap_with_damage() {
        let trap = LoopTrap::with_damage(IVec3::ZERO, 5, 2, 50);
        assert_eq!(trap.damage(), 50);
    }

    #[test]
    fn test_loop_trap_disarm_difficulty_clamped() {
        let trap1 = LoopTrap::new(IVec3::ZERO, 0, 1);
        assert_eq!(trap1.disarm_difficulty(), 1);

        let trap2 = LoopTrap::new(IVec3::ZERO, 15, 2);
        assert_eq!(trap2.disarm_difficulty(), 10);
    }

    #[test]
    fn test_loop_trap_discover() {
        let mut trap = LoopTrap::new(IVec3::ZERO, 5, 1);
        assert!(!trap.is_discovered());

        trap.discover();
        assert!(trap.is_discovered());
    }

    #[test]
    fn test_loop_trap_trigger() {
        let mut trap = LoopTrap::new(IVec3::ZERO, 5, 1);

        let damage = trap.trigger();
        assert_eq!(damage, 20);
        assert_eq!(trap.trigger_count(), 1);

        trap.trigger();
        assert_eq!(trap.trigger_count(), 2);
    }

    #[test]
    fn test_loop_trap_trigger_when_disarmed() {
        let mut trap = LoopTrap::new(IVec3::ZERO, 1, 1);
        trap.attempt_disarm(5);

        let damage = trap.trigger();
        assert_eq!(damage, 0);
    }

    #[test]
    fn test_loop_trap_attempt_disarm_success() {
        let mut trap = LoopTrap::new(IVec3::ZERO, 5, 1);

        assert!(trap.attempt_disarm(5));
        assert!(trap.is_permanently_disarmed());
    }

    #[test]
    fn test_loop_trap_attempt_disarm_failure() {
        let mut trap = LoopTrap::new(IVec3::ZERO, 5, 1);

        assert!(!trap.attempt_disarm(4));
        assert!(!trap.is_permanently_disarmed());
    }

    #[test]
    fn test_loop_trap_reset_for_loop() {
        let mut trap = LoopTrap::new(IVec3::ZERO, 5, 1);
        trap.discover();
        trap.attempt_disarm(10);
        trap.trigger();

        trap.reset_for_loop();

        assert!(!trap.is_discovered());
        assert!(trap.is_permanently_disarmed()); // Persists
        assert_eq!(trap.trigger_count(), 1); // Persists
    }

    #[test]
    fn test_loop_trap_has_prior_knowledge() {
        let mut trap = LoopTrap::new(IVec3::ZERO, 5, 1);
        assert!(!trap.has_prior_knowledge());

        trap.trigger();
        assert!(trap.has_prior_knowledge());
    }

    #[test]
    fn test_loop_trap_has_prior_knowledge_disarmed() {
        let mut trap = LoopTrap::new(IVec3::ZERO, 1, 1);
        trap.attempt_disarm(5);
        assert!(trap.has_prior_knowledge());
    }

    #[test]
    fn test_loop_trap_get_hint() {
        let mut trap = LoopTrap::new(IVec3::ZERO, 5, 1);
        assert!(trap.get_hint().is_none());

        trap.trigger();
        assert!(trap.get_hint().unwrap().contains("previous loop"));

        trap.trigger();
        trap.trigger();
        assert!(trap.get_hint().unwrap().contains("many times"));
    }

    #[test]
    fn test_loop_trap_get_hint_disarmed() {
        let mut trap = LoopTrap::new(IVec3::ZERO, 1, 1);
        trap.attempt_disarm(5);
        assert!(trap.get_hint().unwrap().contains("disarmed"));
    }

    #[test]
    fn test_loop_trap_effective_damage() {
        let trap = LoopTrap::new(IVec3::ZERO, 5, 1);

        assert_eq!(trap.effective_damage(false), 20);
        assert_eq!(trap.effective_damage(true), 10);
    }

    #[test]
    fn test_loop_trap_effective_damage_disarmed() {
        let mut trap = LoopTrap::new(IVec3::ZERO, 1, 1);
        trap.attempt_disarm(5);

        assert_eq!(trap.effective_damage(false), 0);
        assert_eq!(trap.effective_damage(true), 0);
    }

    #[test]
    fn test_loop_trap_registry_new() {
        let registry = LoopTrapRegistry::new();
        assert!(registry.all_traps().is_empty());
        assert_eq!(registry.known_count(), 0);
    }

    #[test]
    fn test_loop_trap_registry_add_trap() {
        let mut registry = LoopTrapRegistry::new();
        registry.add_trap(LoopTrap::new(IVec3::ZERO, 5, 1));
        registry.add_trap(LoopTrap::new(IVec3::new(10, 0, 0), 3, 2));

        assert_eq!(registry.all_traps().len(), 2);
    }

    #[test]
    fn test_loop_trap_registry_get_trap() {
        let mut registry = LoopTrapRegistry::new();
        registry.add_trap(LoopTrap::new(IVec3::ZERO, 5, 42));

        let trap = registry.get_trap(42);
        assert!(trap.is_some());
        assert_eq!(trap.unwrap().trap_id(), 42);

        assert!(registry.get_trap(999).is_none());
    }

    #[test]
    fn test_loop_trap_registry_get_trap_mut() {
        let mut registry = LoopTrapRegistry::new();
        registry.add_trap(LoopTrap::new(IVec3::ZERO, 5, 1));

        if let Some(trap) = registry.get_trap_mut(1) {
            trap.discover();
        }

        assert!(registry.get_trap(1).unwrap().is_discovered());
    }

    #[test]
    fn test_loop_trap_registry_mark_known() {
        let mut registry = LoopTrapRegistry::new();
        assert!(!registry.is_trap_known(1));

        registry.mark_trap_known(1);
        assert!(registry.is_trap_known(1));
        assert_eq!(registry.known_count(), 1);

        // No duplicates
        registry.mark_trap_known(1);
        assert_eq!(registry.known_count(), 1);
    }

    #[test]
    fn test_loop_trap_registry_reset_for_loop() {
        let mut registry = LoopTrapRegistry::new();
        registry.add_trap(LoopTrap::new(IVec3::ZERO, 5, 1));
        registry.mark_trap_known(1);

        if let Some(trap) = registry.get_trap_mut(1) {
            trap.discover();
        }

        registry.reset_for_loop();

        assert!(!registry.get_trap(1).unwrap().is_discovered());
        assert!(registry.is_trap_known(1)); // Known persists
    }

    #[test]
    fn test_loop_trap_registry_traps_near() {
        let mut registry = LoopTrapRegistry::new();
        registry.add_trap(LoopTrap::new(IVec3::new(0, 0, 0), 5, 1));
        registry.add_trap(LoopTrap::new(IVec3::new(5, 0, 0), 5, 2));
        registry.add_trap(LoopTrap::new(IVec3::new(100, 0, 0), 5, 3));

        let nearby = registry.traps_near(IVec3::ZERO, 10);
        assert_eq!(nearby.len(), 2);
    }
}
