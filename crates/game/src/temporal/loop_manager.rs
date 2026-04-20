//! High-level loop manager combining mechanics and persistence.
//!
//! Provides a unified interface for managing time loops, handling
//! death/timeout resets, and coordinating state persistence.

use engine_physics::temporal::{LoopMechanics, LoopPhase};
use serde::{Deserialize, Serialize};

use super::state_persistence::StatePersistence;

/// Manages the complete time loop lifecycle.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoopManager {
    /// Core loop mechanics from the physics engine.
    mechanics: LoopMechanics,
    /// State persistence handler.
    persistence: StatePersistence,
}

impl LoopManager {
    /// Create a new loop manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            mechanics: LoopMechanics::new(),
            persistence: StatePersistence::new(),
        }
    }

    /// Update the loop by delta time.
    ///
    /// Returns `Some(new_phase)` if the phase changed.
    pub fn tick(&mut self, dt: f32) -> Option<LoopPhase> {
        self.mechanics.tick(dt)
    }

    /// Get the current loop number.
    #[must_use]
    pub fn current_loop(&self) -> u32 {
        self.mechanics.loop_count()
    }

    /// Get time remaining in the current phase.
    #[must_use]
    pub fn time_remaining(&self) -> f32 {
        self.mechanics.time_remaining()
    }

    /// Get the current phase.
    #[must_use]
    pub fn current_phase(&self) -> LoopPhase {
        self.mechanics.current_phase()
    }

    /// Get the difficulty modifier for the current loop.
    #[must_use]
    pub fn difficulty(&self) -> f32 {
        self.mechanics.difficulty_modifier()
    }

    /// Handle player death - reset loop and manage persistence.
    ///
    /// Returns the new loop count.
    pub fn on_death(&mut self) -> u32 {
        // Reset volatile state
        self.persistence.reset_volatile();
        // Mark semi-persistent for regeneration
        self.persistence.regenerate_semi();
        // Reset the loop
        self.mechanics.trigger_death_reset()
    }

    /// Handle midnight timeout - same as death reset.
    ///
    /// Returns the new loop count.
    pub fn on_midnight(&mut self) -> u32 {
        self.persistence.reset_volatile();
        self.persistence.regenerate_semi();
        self.mechanics.trigger_midnight_reset()
    }

    /// Break the loop (win condition).
    pub fn break_loop(&mut self) {
        self.mechanics.break_loop();
    }

    /// Get read access to the persistence manager.
    #[must_use]
    pub fn persistence(&self) -> &StatePersistence {
        &self.persistence
    }

    /// Get mutable access to the persistence manager.
    pub fn persistence_mut(&mut self) -> &mut StatePersistence {
        &mut self.persistence
    }

    /// Get read access to the mechanics.
    #[must_use]
    pub fn mechanics(&self) -> &LoopMechanics {
        &self.mechanics
    }
}

impl Default for LoopManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_manager_new() {
        let manager = LoopManager::new();
        assert_eq!(manager.current_loop(), 1);
        assert_eq!(manager.current_phase(), LoopPhase::Dawn);
    }

    #[test]
    fn test_loop_manager_tick() {
        let mut manager = LoopManager::new();
        let result = manager.tick(10.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_loop_manager_tick_phase_change() {
        let mut manager = LoopManager::new();
        // Dawn is 30 seconds
        let result = manager.tick(35.0);
        assert_eq!(result, Some(LoopPhase::Day));
    }

    #[test]
    fn test_loop_manager_on_death() {
        let mut manager = LoopManager::new();
        manager.tick(100.0); // Advance some time

        let new_loop = manager.on_death();
        assert_eq!(new_loop, 2);
        assert_eq!(manager.current_phase(), LoopPhase::Dawn);
    }

    #[test]
    fn test_loop_manager_on_midnight() {
        let mut manager = LoopManager::new();
        let new_loop = manager.on_midnight();
        assert_eq!(new_loop, 2);
    }

    #[test]
    fn test_loop_manager_break_loop() {
        let mut manager = LoopManager::new();
        manager.on_death();
        manager.on_death();
        assert_eq!(manager.current_loop(), 3);

        manager.break_loop();
        assert_eq!(manager.current_loop(), 0);
    }

    #[test]
    fn test_loop_manager_difficulty() {
        let mut manager = LoopManager::new();
        assert!((manager.difficulty() - 1.0).abs() < f32::EPSILON);

        for _ in 0..5 {
            manager.on_death();
        }
        assert!((manager.difficulty() - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_loop_manager_time_remaining() {
        let mut manager = LoopManager::new();
        manager.tick(10.0);
        let remaining = manager.time_remaining();
        assert!((remaining - 20.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_loop_manager_persistence_access() {
        let mut manager = LoopManager::new();
        manager.persistence_mut().save_persistent(&[1, 2, 3]);
        assert_eq!(manager.persistence().load_persistent(), &[1, 2, 3]);
    }
}
