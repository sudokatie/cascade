//! Paradox detection and resolution system.
//!
//! Handles temporal paradoxes that occur when loop state conflicts arise,
//! allowing players to avoid, contain, or exploit paradox energy.

use std::fmt;

use glam::IVec3;
use serde::{Deserialize, Serialize};

/// Maximum number of active paradoxes before overload.
pub const MAX_PARADOXES: usize = 100;

/// Radius for instability calculation.
pub const INSTABILITY_RADIUS: i32 = 20;

/// Energy multiplier when exploiting a paradox.
pub const EXPLOIT_ENERGY_MULTIPLIER: f32 = 0.5;

/// Types of temporal paradoxes that can occur.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParadoxType {
    /// Conflicting terrain states from different loops.
    TerrainConflict,
    /// Creature spawn location conflicts.
    CreatureSpawnConflict,
    /// Overlapping game state from parallel timelines.
    StateOverlap,
    /// Duplicated resources from loop manipulation.
    ResourceDuplication,
}

impl fmt::Display for ParadoxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParadoxType::TerrainConflict => write!(f, "TerrainConflict"),
            ParadoxType::CreatureSpawnConflict => write!(f, "CreatureSpawnConflict"),
            ParadoxType::StateOverlap => write!(f, "StateOverlap"),
            ParadoxType::ResourceDuplication => write!(f, "ResourceDuplication"),
        }
    }
}

/// A single paradox instance in the world.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Paradox {
    /// World position where the paradox manifests.
    pub position: IVec3,
    /// Type of paradox.
    pub paradox_type: ParadoxType,
    /// Severity of the paradox (0-100).
    pub severity: f32,
}

impl Paradox {
    /// Create a new paradox.
    #[must_use]
    pub fn new(position: IVec3, paradox_type: ParadoxType, severity: f32) -> Self {
        Self {
            position,
            paradox_type,
            severity: severity.clamp(0.0, 100.0),
        }
    }
}

/// Resolution strategies for handling paradoxes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParadoxResolution {
    /// Prevent the paradox from occurring.
    Avoid,
    /// Stabilize and contain the paradox.
    Contain,
    /// Harvest energy from the paradox.
    Exploit,
}

impl fmt::Display for ParadoxResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParadoxResolution::Avoid => write!(f, "Avoid"),
            ParadoxResolution::Contain => write!(f, "Contain"),
            ParadoxResolution::Exploit => write!(f, "Exploit"),
        }
    }
}

/// Tracks and manages all active paradoxes in the world.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParadoxTracker {
    /// Active paradoxes.
    paradoxes: Vec<Paradox>,
    /// Maximum allowed paradoxes before overload.
    max_paradoxes: usize,
}

impl ParadoxTracker {
    /// Create a new paradox tracker.
    #[must_use]
    pub fn new() -> Self {
        Self {
            paradoxes: Vec::new(),
            max_paradoxes: MAX_PARADOXES,
        }
    }

    /// Detect and register a paradox at a position.
    ///
    /// Returns the index of the new paradox if added, None if severity is 0 or less.
    pub fn detect_paradox(&mut self, position: IVec3, ptype: ParadoxType) -> Option<usize> {
        // Calculate severity based on existing paradoxes nearby
        let base_severity = 50.0;
        let nearby_count = self
            .paradoxes
            .iter()
            .filter(|p| Self::distance_squared(p.position, position) < 100)
            .count();
        let severity = base_severity + (nearby_count as f32 * 10.0);

        if severity <= 0.0 {
            return None;
        }

        let paradox = Paradox::new(position, ptype, severity);
        self.paradoxes.push(paradox);
        Some(self.paradoxes.len() - 1)
    }

    /// Get all active paradoxes.
    #[must_use]
    pub fn active_paradoxes(&self) -> &[Paradox] {
        &self.paradoxes
    }

    /// Calculate total instability at a position.
    ///
    /// Sums severity of all paradoxes within INSTABILITY_RADIUS blocks.
    #[must_use]
    pub fn instability_at(&self, pos: IVec3) -> f32 {
        let radius_sq = (INSTABILITY_RADIUS * INSTABILITY_RADIUS) as i32;
        self.paradoxes
            .iter()
            .filter(|p| Self::distance_squared(p.position, pos) <= radius_sq)
            .map(|p| p.severity)
            .sum()
    }

    /// Resolve and remove a paradox by index.
    ///
    /// Returns true if the paradox was removed.
    pub fn resolve_paradox(&mut self, index: usize) -> bool {
        if index < self.paradoxes.len() {
            self.paradoxes.remove(index);
            true
        } else {
            false
        }
    }

    /// Exploit a paradox for energy.
    ///
    /// Returns energy gained (severity * 0.5) and removes the paradox.
    pub fn exploit_paradox(&mut self, index: usize) -> f32 {
        if index < self.paradoxes.len() {
            let energy = self.paradoxes[index].severity * EXPLOIT_ENERGY_MULTIPLIER;
            self.paradoxes.remove(index);
            energy
        } else {
            0.0
        }
    }

    /// Get the number of active paradoxes.
    #[must_use]
    pub fn paradox_count(&self) -> usize {
        self.paradoxes.len()
    }

    /// Check if paradox count has reached the overload threshold.
    #[must_use]
    pub fn is_overloaded(&self) -> bool {
        self.paradoxes.len() >= self.max_paradoxes
    }

    /// Calculate squared distance between two positions.
    fn distance_squared(a: IVec3, b: IVec3) -> i32 {
        let diff = a - b;
        diff.x * diff.x + diff.y * diff.y + diff.z * diff.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paradox_type_display() {
        assert_eq!(format!("{}", ParadoxType::TerrainConflict), "TerrainConflict");
        assert_eq!(
            format!("{}", ParadoxType::CreatureSpawnConflict),
            "CreatureSpawnConflict"
        );
        assert_eq!(format!("{}", ParadoxType::StateOverlap), "StateOverlap");
        assert_eq!(
            format!("{}", ParadoxType::ResourceDuplication),
            "ResourceDuplication"
        );
    }

    #[test]
    fn test_paradox_resolution_display() {
        assert_eq!(format!("{}", ParadoxResolution::Avoid), "Avoid");
        assert_eq!(format!("{}", ParadoxResolution::Contain), "Contain");
        assert_eq!(format!("{}", ParadoxResolution::Exploit), "Exploit");
    }

    #[test]
    fn test_paradox_new() {
        let paradox = Paradox::new(IVec3::new(10, 20, 30), ParadoxType::TerrainConflict, 75.0);
        assert_eq!(paradox.position, IVec3::new(10, 20, 30));
        assert_eq!(paradox.paradox_type, ParadoxType::TerrainConflict);
        assert!((paradox.severity - 75.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_paradox_severity_clamped() {
        let low = Paradox::new(IVec3::ZERO, ParadoxType::StateOverlap, -50.0);
        assert!((low.severity - 0.0).abs() < f32::EPSILON);

        let high = Paradox::new(IVec3::ZERO, ParadoxType::StateOverlap, 150.0);
        assert!((high.severity - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_tracker_new() {
        let tracker = ParadoxTracker::new();
        assert_eq!(tracker.paradox_count(), 0);
        assert!(!tracker.is_overloaded());
    }

    #[test]
    fn test_tracker_detect_paradox() {
        let mut tracker = ParadoxTracker::new();
        let index = tracker.detect_paradox(IVec3::new(5, 5, 5), ParadoxType::TerrainConflict);
        assert_eq!(index, Some(0));
        assert_eq!(tracker.paradox_count(), 1);
    }

    #[test]
    fn test_tracker_active_paradoxes() {
        let mut tracker = ParadoxTracker::new();
        tracker.detect_paradox(IVec3::new(0, 0, 0), ParadoxType::TerrainConflict);
        tracker.detect_paradox(IVec3::new(100, 100, 100), ParadoxType::StateOverlap);

        let active = tracker.active_paradoxes();
        assert_eq!(active.len(), 2);
    }

    #[test]
    fn test_tracker_instability_at() {
        let mut tracker = ParadoxTracker::new();
        tracker.detect_paradox(IVec3::new(0, 0, 0), ParadoxType::TerrainConflict);
        tracker.detect_paradox(IVec3::new(5, 5, 5), ParadoxType::StateOverlap);

        // Both should contribute to instability at origin
        let instability = tracker.instability_at(IVec3::ZERO);
        assert!(instability > 50.0);

        // Far away should have less instability
        let far_instability = tracker.instability_at(IVec3::new(1000, 1000, 1000));
        assert!((far_instability - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_tracker_resolve_paradox() {
        let mut tracker = ParadoxTracker::new();
        tracker.detect_paradox(IVec3::ZERO, ParadoxType::TerrainConflict);
        assert_eq!(tracker.paradox_count(), 1);

        let resolved = tracker.resolve_paradox(0);
        assert!(resolved);
        assert_eq!(tracker.paradox_count(), 0);

        let invalid = tracker.resolve_paradox(0);
        assert!(!invalid);
    }

    #[test]
    fn test_tracker_exploit_paradox() {
        let mut tracker = ParadoxTracker::new();
        tracker.detect_paradox(IVec3::ZERO, ParadoxType::ResourceDuplication);

        let energy = tracker.exploit_paradox(0);
        assert!(energy > 0.0);
        assert_eq!(tracker.paradox_count(), 0);

        // Exploiting non-existent paradox returns 0
        let no_energy = tracker.exploit_paradox(0);
        assert!((no_energy - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_tracker_is_overloaded() {
        let mut tracker = ParadoxTracker::new();
        tracker.max_paradoxes = 3;

        tracker.detect_paradox(IVec3::new(0, 0, 0), ParadoxType::TerrainConflict);
        tracker.detect_paradox(IVec3::new(100, 0, 0), ParadoxType::StateOverlap);
        assert!(!tracker.is_overloaded());

        tracker.detect_paradox(IVec3::new(200, 0, 0), ParadoxType::ResourceDuplication);
        assert!(tracker.is_overloaded());
    }
}
