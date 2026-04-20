//! Game-level paradox handling and effects.
//!
//! Builds on the physics engine's paradox system to provide
//! gameplay effects and resolution strategies.

use engine_physics::temporal::{Paradox, ParadoxResolution, ParadoxTracker, ParadoxType};
use glam::IVec3;
use serde::{Deserialize, Serialize};

/// Threshold for visual distortion effects.
pub const DISTORTION_THRESHOLD: f32 = 50.0;

/// Threshold for damaging instability.
pub const DAMAGE_THRESHOLD: f32 = 100.0;

/// Game-level paradox with additional gameplay data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameParadox {
    /// The underlying physics paradox.
    pub core: Paradox,
    /// Whether this paradox has been discovered by the player.
    pub discovered: bool,
    /// Number of loops this paradox has existed.
    pub loop_age: u32,
}

impl GameParadox {
    /// Create a new game paradox.
    #[must_use]
    pub fn new(position: IVec3, paradox_type: ParadoxType, severity: f32) -> Self {
        Self {
            core: Paradox::new(position, paradox_type, severity),
            discovered: false,
            loop_age: 0,
        }
    }

    /// Mark this paradox as discovered.
    pub fn discover(&mut self) {
        self.discovered = true;
    }

    /// Age this paradox by one loop.
    pub fn age_loop(&mut self) {
        self.loop_age += 1;
    }

    /// Check if this paradox causes visual distortion.
    #[must_use]
    pub fn causes_distortion(&self) -> bool {
        self.core.severity >= DISTORTION_THRESHOLD
    }

    /// Check if this paradox is damaging.
    #[must_use]
    pub fn is_damaging(&self) -> bool {
        self.core.severity >= DAMAGE_THRESHOLD
    }
}

/// Handles paradox detection, resolution, and gameplay effects.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParadoxHandler {
    /// Physics-level paradox tracker.
    tracker: ParadoxTracker,
    /// Game-level paradox data.
    game_paradoxes: Vec<GameParadox>,
    /// Total energy harvested from paradoxes.
    harvested_energy: f32,
}

impl ParadoxHandler {
    /// Create a new paradox handler.
    #[must_use]
    pub fn new() -> Self {
        Self {
            tracker: ParadoxTracker::new(),
            game_paradoxes: Vec::new(),
            harvested_energy: 0.0,
        }
    }

    /// Detect and register a new paradox.
    ///
    /// Returns the index if added.
    pub fn detect(&mut self, position: IVec3, ptype: ParadoxType, severity: f32) -> Option<usize> {
        let game_paradox = GameParadox::new(position, ptype, severity);
        self.tracker.detect_paradox(position, ptype)?;
        self.game_paradoxes.push(game_paradox);
        Some(self.game_paradoxes.len() - 1)
    }

    /// Resolve a paradox using the specified strategy.
    ///
    /// Returns energy gained (for Exploit) or 0.
    pub fn resolve(&mut self, index: usize, strategy: ParadoxResolution) -> f32 {
        if index >= self.game_paradoxes.len() {
            return 0.0;
        }

        match strategy {
            ParadoxResolution::Avoid | ParadoxResolution::Contain => {
                self.tracker.resolve_paradox(index);
                self.game_paradoxes.remove(index);
                0.0
            }
            ParadoxResolution::Exploit => {
                let energy = self.tracker.exploit_paradox(index);
                self.game_paradoxes.remove(index);
                self.harvested_energy += energy;
                energy
            }
        }
    }

    /// Get instability level at a position.
    #[must_use]
    pub fn instability_at(&self, pos: IVec3) -> f32 {
        self.tracker.instability_at(pos)
    }

    /// Check if the world is overloaded with paradoxes.
    #[must_use]
    pub fn is_overloaded(&self) -> bool {
        self.tracker.is_overloaded()
    }

    /// Get total harvested energy.
    #[must_use]
    pub fn harvested_energy(&self) -> f32 {
        self.harvested_energy
    }

    /// Get number of active paradoxes.
    #[must_use]
    pub fn paradox_count(&self) -> usize {
        self.game_paradoxes.len()
    }

    /// Get all game paradoxes.
    #[must_use]
    pub fn paradoxes(&self) -> &[GameParadox] {
        &self.game_paradoxes
    }

    /// Discover a paradox at an index.
    pub fn discover(&mut self, index: usize) {
        if let Some(paradox) = self.game_paradoxes.get_mut(index) {
            paradox.discover();
        }
    }

    /// Age all paradoxes by one loop.
    pub fn age_all(&mut self) {
        for paradox in &mut self.game_paradoxes {
            paradox.age_loop();
        }
    }

    /// Get count of discovered paradoxes.
    #[must_use]
    pub fn discovered_count(&self) -> usize {
        self.game_paradoxes.iter().filter(|p| p.discovered).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_paradox_new() {
        let paradox = GameParadox::new(IVec3::new(1, 2, 3), ParadoxType::TerrainConflict, 75.0);
        assert_eq!(paradox.core.position, IVec3::new(1, 2, 3));
        assert!(!paradox.discovered);
        assert_eq!(paradox.loop_age, 0);
    }

    #[test]
    fn test_game_paradox_discover() {
        let mut paradox = GameParadox::new(IVec3::ZERO, ParadoxType::StateOverlap, 50.0);
        assert!(!paradox.discovered);
        paradox.discover();
        assert!(paradox.discovered);
    }

    #[test]
    fn test_game_paradox_age() {
        let mut paradox = GameParadox::new(IVec3::ZERO, ParadoxType::StateOverlap, 50.0);
        paradox.age_loop();
        paradox.age_loop();
        assert_eq!(paradox.loop_age, 2);
    }

    #[test]
    fn test_game_paradox_causes_distortion() {
        let low = GameParadox::new(IVec3::ZERO, ParadoxType::StateOverlap, 30.0);
        assert!(!low.causes_distortion());

        let high = GameParadox::new(IVec3::ZERO, ParadoxType::StateOverlap, 60.0);
        assert!(high.causes_distortion());
    }

    #[test]
    fn test_game_paradox_is_damaging() {
        let safe = GameParadox::new(IVec3::ZERO, ParadoxType::StateOverlap, 80.0);
        assert!(!safe.is_damaging());

        let dangerous = GameParadox::new(IVec3::ZERO, ParadoxType::StateOverlap, 100.0);
        assert!(dangerous.is_damaging());
    }

    #[test]
    fn test_handler_new() {
        let handler = ParadoxHandler::new();
        assert_eq!(handler.paradox_count(), 0);
        assert!((handler.harvested_energy() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_handler_detect() {
        let mut handler = ParadoxHandler::new();
        let index = handler.detect(IVec3::ZERO, ParadoxType::TerrainConflict, 50.0);
        assert!(index.is_some());
        assert_eq!(handler.paradox_count(), 1);
    }

    #[test]
    fn test_handler_resolve_avoid() {
        let mut handler = ParadoxHandler::new();
        handler.detect(IVec3::ZERO, ParadoxType::TerrainConflict, 50.0);

        let energy = handler.resolve(0, ParadoxResolution::Avoid);
        assert!((energy - 0.0).abs() < f32::EPSILON);
        assert_eq!(handler.paradox_count(), 0);
    }

    #[test]
    fn test_handler_resolve_exploit() {
        let mut handler = ParadoxHandler::new();
        handler.detect(IVec3::ZERO, ParadoxType::ResourceDuplication, 50.0);

        let energy = handler.resolve(0, ParadoxResolution::Exploit);
        assert!(energy > 0.0);
        assert!(handler.harvested_energy() > 0.0);
    }

    #[test]
    fn test_handler_discover() {
        let mut handler = ParadoxHandler::new();
        handler.detect(IVec3::ZERO, ParadoxType::TerrainConflict, 50.0);
        assert_eq!(handler.discovered_count(), 0);

        handler.discover(0);
        assert_eq!(handler.discovered_count(), 1);
    }

    #[test]
    fn test_handler_age_all() {
        let mut handler = ParadoxHandler::new();
        handler.detect(IVec3::new(0, 0, 0), ParadoxType::TerrainConflict, 50.0);
        handler.detect(IVec3::new(100, 0, 0), ParadoxType::StateOverlap, 50.0);

        handler.age_all();
        assert_eq!(handler.paradoxes()[0].loop_age, 1);
        assert_eq!(handler.paradoxes()[1].loop_age, 1);
    }
}
