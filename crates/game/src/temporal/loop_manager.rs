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

    /// Get the difficulty scaling based on current loop number.
    ///
    /// - Loops 1-5: returns 1.0 (normal difficulty)
    /// - Loops 6-15: returns 1.5 (increased difficulty)
    /// - Loops 16+: returns 2.0 (maximum difficulty)
    #[must_use]
    pub fn difficulty_scaling(&self) -> f32 {
        let loop_num = self.current_loop();
        if loop_num <= 5 {
            1.0
        } else if loop_num <= 15 {
            1.5
        } else {
            2.0
        }
    }

    /// Attempt to break the time loop using temporal keys.
    ///
    /// Requires exactly 3 temporal keys to succeed.
    /// Returns true if the loop was successfully broken.
    #[must_use]
    pub fn loop_break_sequence(&mut self, temporal_keys: u32) -> bool {
        if temporal_keys >= 3 {
            self.break_loop();
            true
        } else {
            false
        }
    }

    /// Handle player death - reset loop and manage persistence.
    ///
    /// Returns the new loop count with smooth difficulty transition.
    pub fn on_death(&mut self) -> u32 {
        // Reset volatile state
        self.persistence.reset_volatile();
        // Mark semi-persistent for regeneration
        self.persistence.regenerate_semi();
        // Reset the loop with smooth difficulty transition
        let new_loop = self.mechanics.trigger_death_reset();
        // Apply smooth difficulty scaling (mechanics handles the base,
        // difficulty_scaling() provides the multiplier)
        new_loop
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

    #[test]
    fn test_difficulty_scaling_early_loops() {
        let manager = LoopManager::new();
        assert!((manager.difficulty_scaling() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_difficulty_scaling_loop_5() {
        let mut manager = LoopManager::new();
        for _ in 0..4 {
            manager.on_death();
        }
        assert_eq!(manager.current_loop(), 5);
        assert!((manager.difficulty_scaling() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_difficulty_scaling_loop_6() {
        let mut manager = LoopManager::new();
        for _ in 0..5 {
            manager.on_death();
        }
        assert_eq!(manager.current_loop(), 6);
        assert!((manager.difficulty_scaling() - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_difficulty_scaling_loop_15() {
        let mut manager = LoopManager::new();
        for _ in 0..14 {
            manager.on_death();
        }
        assert_eq!(manager.current_loop(), 15);
        assert!((manager.difficulty_scaling() - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_difficulty_scaling_loop_16() {
        let mut manager = LoopManager::new();
        for _ in 0..15 {
            manager.on_death();
        }
        assert_eq!(manager.current_loop(), 16);
        assert!((manager.difficulty_scaling() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_difficulty_scaling_high_loop() {
        let mut manager = LoopManager::new();
        for _ in 0..99 {
            manager.on_death();
        }
        assert_eq!(manager.current_loop(), 100);
        assert!((manager.difficulty_scaling() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_loop_break_sequence_with_3_keys() {
        let mut manager = LoopManager::new();
        manager.on_death();
        manager.on_death();
        assert_eq!(manager.current_loop(), 3);

        let result = manager.loop_break_sequence(3);
        assert!(result);
        assert_eq!(manager.current_loop(), 0);
    }

    #[test]
    fn test_loop_break_sequence_with_more_keys() {
        let mut manager = LoopManager::new();
        let result = manager.loop_break_sequence(5);
        assert!(result);
        assert_eq!(manager.current_loop(), 0);
    }

    #[test]
    fn test_loop_break_sequence_insufficient_keys() {
        let mut manager = LoopManager::new();
        manager.on_death();
        assert_eq!(manager.current_loop(), 2);

        let result = manager.loop_break_sequence(2);
        assert!(!result);
        assert_eq!(manager.current_loop(), 2);
    }

    #[test]
    fn test_loop_break_sequence_zero_keys() {
        let mut manager = LoopManager::new();
        let result = manager.loop_break_sequence(0);
        assert!(!result);
        assert_eq!(manager.current_loop(), 1);
    }

    #[test]
    fn test_on_death_returns_incremented_loop() {
        let mut manager = LoopManager::new();
        assert_eq!(manager.current_loop(), 1);

        let new_loop = manager.on_death();
        assert_eq!(new_loop, 2);
        assert_eq!(manager.current_loop(), 2);
    }

    #[test]
    fn test_on_death_smooth_transition() {
        let mut manager = LoopManager::new();
        // Progress through multiple deaths
        for i in 1..=10 {
            let new_loop = manager.on_death();
            assert_eq!(new_loop, i + 1);
        }
        assert_eq!(manager.current_loop(), 11);
    }
}
