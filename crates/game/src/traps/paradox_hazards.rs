//! Paradox hazard types for time-loop survival.
//!
//! Provides temporal hazards caused by paradox buildup.

use glam::IVec3;
use serde::{Deserialize, Serialize};

/// Types of paradox hazards.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParadoxHazardType {
    /// An unstable zone where time flows unpredictably.
    UnstableZone,
    /// A bubble of frozen or accelerated time.
    TimeBubble,
    /// A glitch that creates duplicates of entities.
    DuplicationGlitch,
}

impl ParadoxHazardType {
    /// Get display name for this hazard type.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            ParadoxHazardType::UnstableZone => "Unstable Zone",
            ParadoxHazardType::TimeBubble => "Time Bubble",
            ParadoxHazardType::DuplicationGlitch => "Duplication Glitch",
        }
    }

    /// Get description for this hazard type.
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            ParadoxHazardType::UnstableZone => "Time flows erratically, causing disorientation",
            ParadoxHazardType::TimeBubble => "A bubble where time is frozen or accelerated",
            ParadoxHazardType::DuplicationGlitch => "Creates unstable copies of nearby entities",
        }
    }

    /// Get base radius for this hazard type.
    #[must_use]
    pub fn base_radius(&self) -> f32 {
        match self {
            ParadoxHazardType::UnstableZone => 5.0,
            ParadoxHazardType::TimeBubble => 3.0,
            ParadoxHazardType::DuplicationGlitch => 2.0,
        }
    }

    /// Get base severity (1-10 scale).
    #[must_use]
    pub fn base_severity(&self) -> u32 {
        match self {
            ParadoxHazardType::UnstableZone => 3,
            ParadoxHazardType::TimeBubble => 5,
            ParadoxHazardType::DuplicationGlitch => 7,
        }
    }

    /// Get all paradox hazard types.
    #[must_use]
    pub fn all() -> &'static [ParadoxHazardType] {
        &[
            ParadoxHazardType::UnstableZone,
            ParadoxHazardType::TimeBubble,
            ParadoxHazardType::DuplicationGlitch,
        ]
    }
}

/// A paradox hazard instance in the world.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParadoxHazard {
    /// Type of hazard.
    hazard_type: ParadoxHazardType,
    /// Center position.
    position: IVec3,
    /// Effect radius.
    radius: f32,
    /// Severity level (1-10).
    severity: u32,
    /// Whether the hazard is active.
    active: bool,
    /// Remaining duration in ticks (0 = permanent).
    duration: u32,
    /// Paradox level that spawned this hazard.
    source_paradox: f32,
}

impl ParadoxHazard {
    /// Create a new paradox hazard.
    #[must_use]
    pub fn new(hazard_type: ParadoxHazardType, position: IVec3) -> Self {
        Self {
            hazard_type,
            position,
            radius: hazard_type.base_radius(),
            severity: hazard_type.base_severity(),
            active: true,
            duration: 0,
            source_paradox: 0.0,
        }
    }

    /// Create a hazard with custom radius and severity.
    #[must_use]
    pub fn with_params(
        hazard_type: ParadoxHazardType,
        position: IVec3,
        radius: f32,
        severity: u32,
    ) -> Self {
        Self {
            hazard_type,
            position,
            radius,
            severity: severity.clamp(1, 10),
            active: true,
            duration: 0,
            source_paradox: 0.0,
        }
    }

    /// Create a temporary hazard with duration.
    #[must_use]
    pub fn temporary(hazard_type: ParadoxHazardType, position: IVec3, duration: u32) -> Self {
        let mut hazard = Self::new(hazard_type, position);
        hazard.duration = duration;
        hazard
    }

    /// Get the hazard type.
    #[must_use]
    pub fn hazard_type(&self) -> ParadoxHazardType {
        self.hazard_type
    }

    /// Get the position.
    #[must_use]
    pub fn position(&self) -> IVec3 {
        self.position
    }

    /// Get the radius.
    #[must_use]
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// Get the severity.
    #[must_use]
    pub fn severity(&self) -> u32 {
        self.severity
    }

    /// Check if the hazard is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get remaining duration (0 = permanent).
    #[must_use]
    pub fn duration(&self) -> u32 {
        self.duration
    }

    /// Check if the hazard is permanent.
    #[must_use]
    pub fn is_permanent(&self) -> bool {
        self.duration == 0 && self.active
    }

    /// Get the source paradox level.
    #[must_use]
    pub fn source_paradox(&self) -> f32 {
        self.source_paradox
    }

    /// Set the source paradox level.
    pub fn set_source_paradox(&mut self, level: f32) {
        self.source_paradox = level;
    }

    /// Check if a position is within the hazard's effect radius.
    #[must_use]
    pub fn contains(&self, pos: IVec3) -> bool {
        if !self.active {
            return false;
        }
        let diff = pos - self.position;
        let distance = ((diff.x * diff.x + diff.y * diff.y + diff.z * diff.z) as f32).sqrt();
        distance <= self.radius
    }

    /// Update the hazard (called each tick).
    ///
    /// Returns false if the hazard should be removed.
    pub fn update(&mut self) -> bool {
        if !self.active {
            return false;
        }
        if self.duration > 0 {
            self.duration -= 1;
            if self.duration == 0 {
                self.active = false;
                return false;
            }
        }
        true
    }

    /// Deactivate the hazard.
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Get the effect strength at a given position.
    #[must_use]
    pub fn effect_strength_at(&self, pos: IVec3) -> f32 {
        if !self.contains(pos) {
            return 0.0;
        }
        let diff = pos - self.position;
        let distance = ((diff.x * diff.x + diff.y * diff.y + diff.z * diff.z) as f32).sqrt();
        let falloff = 1.0 - (distance / self.radius);
        falloff * (self.severity as f32 / 10.0)
    }

    /// Expand the hazard radius.
    pub fn expand(&mut self, amount: f32) {
        self.radius += amount;
    }

    /// Intensify the hazard severity.
    pub fn intensify(&mut self, amount: u32) {
        self.severity = (self.severity + amount).min(10);
    }

    /// Get the time dilation factor for this hazard.
    #[must_use]
    pub fn time_dilation(&self) -> f32 {
        match self.hazard_type {
            ParadoxHazardType::UnstableZone => 1.0 + (self.severity as f32 * 0.1),
            ParadoxHazardType::TimeBubble => {
                if self.severity <= 5 {
                    0.5 // Slowed time
                } else {
                    2.0 // Accelerated time
                }
            }
            ParadoxHazardType::DuplicationGlitch => 1.0,
        }
    }
}

/// Manager for paradox hazards.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParadoxHazardManager {
    /// All active hazards.
    hazards: Vec<ParadoxHazard>,
    /// Current global paradox level.
    global_paradox: f32,
    /// Threshold for spawning new hazards.
    spawn_threshold: f32,
}

impl ParadoxHazardManager {
    /// Create a new manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            hazards: Vec::new(),
            global_paradox: 0.0,
            spawn_threshold: 50.0,
        }
    }

    /// Add a hazard.
    pub fn add_hazard(&mut self, hazard: ParadoxHazard) {
        self.hazards.push(hazard);
    }

    /// Get all active hazards.
    #[must_use]
    pub fn hazards(&self) -> &[ParadoxHazard] {
        &self.hazards
    }

    /// Get the global paradox level.
    #[must_use]
    pub fn global_paradox(&self) -> f32 {
        self.global_paradox
    }

    /// Set the global paradox level.
    pub fn set_global_paradox(&mut self, level: f32) {
        self.global_paradox = level.max(0.0);
    }

    /// Add to the global paradox level.
    pub fn add_paradox(&mut self, amount: f32) {
        self.global_paradox = (self.global_paradox + amount).max(0.0);
    }

    /// Update all hazards.
    pub fn update(&mut self) {
        self.hazards.retain_mut(|h| h.update());
    }

    /// Get hazards affecting a position.
    #[must_use]
    pub fn hazards_at(&self, pos: IVec3) -> Vec<&ParadoxHazard> {
        self.hazards.iter().filter(|h| h.contains(pos)).collect()
    }

    /// Get total effect strength at a position.
    #[must_use]
    pub fn total_effect_at(&self, pos: IVec3) -> f32 {
        self.hazards
            .iter()
            .map(|h| h.effect_strength_at(pos))
            .sum()
    }

    /// Check if paradox level exceeds spawn threshold.
    #[must_use]
    pub fn should_spawn_hazard(&self) -> bool {
        self.global_paradox >= self.spawn_threshold
    }

    /// Spawn a random hazard at a position.
    pub fn spawn_hazard_at(&mut self, pos: IVec3) {
        let types = ParadoxHazardType::all();
        let index = (self.global_paradox as usize) % types.len();
        let mut hazard = ParadoxHazard::new(types[index], pos);
        hazard.set_source_paradox(self.global_paradox);
        self.hazards.push(hazard);
    }

    /// Clear all hazards.
    pub fn clear(&mut self) {
        self.hazards.clear();
    }

    /// Get hazard count.
    #[must_use]
    pub fn hazard_count(&self) -> usize {
        self.hazards.len()
    }

    /// Reset for a new loop.
    pub fn reset_for_loop(&mut self) {
        // Remove temporary hazards, keep permanent ones
        self.hazards.retain(|h| h.is_permanent());
        self.global_paradox *= 0.5; // Paradox partially carries over
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paradox_hazard_type_display_names() {
        assert_eq!(ParadoxHazardType::UnstableZone.display_name(), "Unstable Zone");
        assert_eq!(ParadoxHazardType::TimeBubble.display_name(), "Time Bubble");
        assert_eq!(
            ParadoxHazardType::DuplicationGlitch.display_name(),
            "Duplication Glitch"
        );
    }

    #[test]
    fn test_paradox_hazard_type_base_radius() {
        assert!((ParadoxHazardType::UnstableZone.base_radius() - 5.0).abs() < f32::EPSILON);
        assert!((ParadoxHazardType::TimeBubble.base_radius() - 3.0).abs() < f32::EPSILON);
        assert!((ParadoxHazardType::DuplicationGlitch.base_radius() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_paradox_hazard_type_base_severity() {
        assert_eq!(ParadoxHazardType::UnstableZone.base_severity(), 3);
        assert_eq!(ParadoxHazardType::TimeBubble.base_severity(), 5);
        assert_eq!(ParadoxHazardType::DuplicationGlitch.base_severity(), 7);
    }

    #[test]
    fn test_paradox_hazard_type_all() {
        let all = ParadoxHazardType::all();
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn test_paradox_hazard_new() {
        let hazard = ParadoxHazard::new(ParadoxHazardType::TimeBubble, IVec3::new(10, 0, 5));

        assert_eq!(hazard.hazard_type(), ParadoxHazardType::TimeBubble);
        assert_eq!(hazard.position(), IVec3::new(10, 0, 5));
        assert!((hazard.radius() - 3.0).abs() < f32::EPSILON);
        assert_eq!(hazard.severity(), 5);
        assert!(hazard.is_active());
        assert!(hazard.is_permanent());
    }

    #[test]
    fn test_paradox_hazard_with_params() {
        let hazard = ParadoxHazard::with_params(
            ParadoxHazardType::UnstableZone,
            IVec3::ZERO,
            10.0,
            8,
        );

        assert!((hazard.radius() - 10.0).abs() < f32::EPSILON);
        assert_eq!(hazard.severity(), 8);
    }

    #[test]
    fn test_paradox_hazard_severity_clamped() {
        let hazard = ParadoxHazard::with_params(
            ParadoxHazardType::UnstableZone,
            IVec3::ZERO,
            5.0,
            15,
        );
        assert_eq!(hazard.severity(), 10);
    }

    #[test]
    fn test_paradox_hazard_temporary() {
        let hazard = ParadoxHazard::temporary(ParadoxHazardType::TimeBubble, IVec3::ZERO, 100);

        assert!(!hazard.is_permanent());
        assert_eq!(hazard.duration(), 100);
    }

    #[test]
    fn test_paradox_hazard_contains() {
        let hazard = ParadoxHazard::new(ParadoxHazardType::UnstableZone, IVec3::ZERO);

        assert!(hazard.contains(IVec3::ZERO));
        assert!(hazard.contains(IVec3::new(2, 0, 0)));
        assert!(!hazard.contains(IVec3::new(100, 0, 0)));
    }

    #[test]
    fn test_paradox_hazard_contains_inactive() {
        let mut hazard = ParadoxHazard::new(ParadoxHazardType::TimeBubble, IVec3::ZERO);
        hazard.deactivate();

        assert!(!hazard.contains(IVec3::ZERO));
    }

    #[test]
    fn test_paradox_hazard_update_temporary() {
        let mut hazard = ParadoxHazard::temporary(ParadoxHazardType::TimeBubble, IVec3::ZERO, 3);

        assert!(hazard.update());
        assert_eq!(hazard.duration(), 2);

        assert!(hazard.update());
        assert_eq!(hazard.duration(), 1);

        assert!(!hazard.update());
        assert!(!hazard.is_active());
    }

    #[test]
    fn test_paradox_hazard_update_permanent() {
        let mut hazard = ParadoxHazard::new(ParadoxHazardType::UnstableZone, IVec3::ZERO);

        assert!(hazard.update());
        assert!(hazard.update());
        assert!(hazard.is_active());
    }

    #[test]
    fn test_paradox_hazard_effect_strength_at() {
        let hazard = ParadoxHazard::with_params(
            ParadoxHazardType::TimeBubble,
            IVec3::ZERO,
            10.0,
            10,
        );

        // At center: full strength
        let center_strength = hazard.effect_strength_at(IVec3::ZERO);
        assert!((center_strength - 1.0).abs() < f32::EPSILON);

        // At edge: zero
        let edge_strength = hazard.effect_strength_at(IVec3::new(10, 0, 0));
        assert!(edge_strength < 0.1);

        // Outside: zero
        let outside_strength = hazard.effect_strength_at(IVec3::new(100, 0, 0));
        assert!((outside_strength - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_paradox_hazard_expand() {
        let mut hazard = ParadoxHazard::new(ParadoxHazardType::TimeBubble, IVec3::ZERO);
        let initial = hazard.radius();

        hazard.expand(2.0);
        assert!((hazard.radius() - (initial + 2.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_paradox_hazard_intensify() {
        let mut hazard = ParadoxHazard::new(ParadoxHazardType::UnstableZone, IVec3::ZERO);
        assert_eq!(hazard.severity(), 3);

        hazard.intensify(5);
        assert_eq!(hazard.severity(), 8);

        hazard.intensify(10);
        assert_eq!(hazard.severity(), 10); // Capped at 10
    }

    #[test]
    fn test_paradox_hazard_time_dilation() {
        let unstable = ParadoxHazard::new(ParadoxHazardType::UnstableZone, IVec3::ZERO);
        assert!(unstable.time_dilation() > 1.0);

        let slow_bubble = ParadoxHazard::with_params(
            ParadoxHazardType::TimeBubble,
            IVec3::ZERO,
            3.0,
            3,
        );
        assert!((slow_bubble.time_dilation() - 0.5).abs() < f32::EPSILON);

        let fast_bubble = ParadoxHazard::with_params(
            ParadoxHazardType::TimeBubble,
            IVec3::ZERO,
            3.0,
            8,
        );
        assert!((fast_bubble.time_dilation() - 2.0).abs() < f32::EPSILON);

        let glitch = ParadoxHazard::new(ParadoxHazardType::DuplicationGlitch, IVec3::ZERO);
        assert!((glitch.time_dilation() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_paradox_hazard_manager_new() {
        let manager = ParadoxHazardManager::new();
        assert!(manager.hazards().is_empty());
        assert!((manager.global_paradox() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_paradox_hazard_manager_add_hazard() {
        let mut manager = ParadoxHazardManager::new();
        manager.add_hazard(ParadoxHazard::new(ParadoxHazardType::TimeBubble, IVec3::ZERO));

        assert_eq!(manager.hazard_count(), 1);
    }

    #[test]
    fn test_paradox_hazard_manager_paradox_level() {
        let mut manager = ParadoxHazardManager::new();

        manager.add_paradox(25.0);
        assert!((manager.global_paradox() - 25.0).abs() < f32::EPSILON);

        manager.add_paradox(30.0);
        assert!((manager.global_paradox() - 55.0).abs() < f32::EPSILON);

        manager.set_global_paradox(10.0);
        assert!((manager.global_paradox() - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_paradox_hazard_manager_should_spawn() {
        let mut manager = ParadoxHazardManager::new();

        assert!(!manager.should_spawn_hazard());

        manager.add_paradox(50.0);
        assert!(manager.should_spawn_hazard());
    }

    #[test]
    fn test_paradox_hazard_manager_update() {
        let mut manager = ParadoxHazardManager::new();
        manager.add_hazard(ParadoxHazard::temporary(
            ParadoxHazardType::TimeBubble,
            IVec3::ZERO,
            2,
        ));
        manager.add_hazard(ParadoxHazard::new(
            ParadoxHazardType::UnstableZone,
            IVec3::new(10, 0, 0),
        ));

        assert_eq!(manager.hazard_count(), 2);

        manager.update();
        manager.update();

        assert_eq!(manager.hazard_count(), 1);
    }

    #[test]
    fn test_paradox_hazard_manager_hazards_at() {
        let mut manager = ParadoxHazardManager::new();
        manager.add_hazard(ParadoxHazard::new(ParadoxHazardType::TimeBubble, IVec3::ZERO));
        manager.add_hazard(ParadoxHazard::new(
            ParadoxHazardType::UnstableZone,
            IVec3::new(100, 0, 0),
        ));

        let at_origin = manager.hazards_at(IVec3::ZERO);
        assert_eq!(at_origin.len(), 1);

        let far_away = manager.hazards_at(IVec3::new(1000, 0, 0));
        assert!(far_away.is_empty());
    }

    #[test]
    fn test_paradox_hazard_manager_total_effect_at() {
        let mut manager = ParadoxHazardManager::new();
        manager.add_hazard(ParadoxHazard::with_params(
            ParadoxHazardType::TimeBubble,
            IVec3::ZERO,
            10.0,
            5,
        ));
        manager.add_hazard(ParadoxHazard::with_params(
            ParadoxHazardType::UnstableZone,
            IVec3::new(2, 0, 0),
            10.0,
            5,
        ));

        let total = manager.total_effect_at(IVec3::new(1, 0, 0));
        assert!(total > 0.0);
    }

    #[test]
    fn test_paradox_hazard_manager_spawn_hazard_at() {
        let mut manager = ParadoxHazardManager::new();
        manager.set_global_paradox(100.0);
        manager.spawn_hazard_at(IVec3::new(50, 0, 50));

        assert_eq!(manager.hazard_count(), 1);
        let hazard = &manager.hazards()[0];
        assert_eq!(hazard.position(), IVec3::new(50, 0, 50));
        assert!((hazard.source_paradox() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_paradox_hazard_manager_clear() {
        let mut manager = ParadoxHazardManager::new();
        manager.add_hazard(ParadoxHazard::new(ParadoxHazardType::TimeBubble, IVec3::ZERO));
        manager.add_hazard(ParadoxHazard::new(ParadoxHazardType::UnstableZone, IVec3::ZERO));

        manager.clear();
        assert!(manager.hazards().is_empty());
    }

    #[test]
    fn test_paradox_hazard_manager_reset_for_loop() {
        let mut manager = ParadoxHazardManager::new();
        manager.set_global_paradox(100.0);
        manager.add_hazard(ParadoxHazard::temporary(
            ParadoxHazardType::TimeBubble,
            IVec3::ZERO,
            50,
        ));
        manager.add_hazard(ParadoxHazard::new(
            ParadoxHazardType::UnstableZone,
            IVec3::new(10, 0, 0),
        ));

        manager.reset_for_loop();

        assert_eq!(manager.hazard_count(), 1); // Only permanent remains
        assert!((manager.global_paradox() - 50.0).abs() < f32::EPSILON); // Half paradox
    }
}
