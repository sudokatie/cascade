//! Static trap types for time-loop survival.
//!
//! Provides environmental hazards that persist across loops.

use glam::IVec3;
use serde::{Deserialize, Serialize};

/// Types of static traps in the environment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StaticTrapType {
    /// Pit that deals fall damage.
    Pit,
    /// Spike trap that deals piercing damage.
    Spike,
    /// Gas trap that deals poison damage over time.
    Gas,
    /// Arrow trap that fires projectiles.
    Arrow,
}

impl StaticTrapType {
    /// Get base damage for this trap type.
    #[must_use]
    pub fn base_damage(&self) -> u32 {
        match self {
            StaticTrapType::Pit => 15,
            StaticTrapType::Spike => 25,
            StaticTrapType::Gas => 5,
            StaticTrapType::Arrow => 20,
        }
    }

    /// Get display name for this trap type.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            StaticTrapType::Pit => "Pit Trap",
            StaticTrapType::Spike => "Spike Trap",
            StaticTrapType::Gas => "Gas Trap",
            StaticTrapType::Arrow => "Arrow Trap",
        }
    }

    /// Get description for this trap type.
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            StaticTrapType::Pit => "A concealed pit that causes fall damage",
            StaticTrapType::Spike => "Sharp spikes that deal piercing damage",
            StaticTrapType::Gas => "Releases toxic gas dealing damage over time",
            StaticTrapType::Arrow => "Fires arrows when triggered",
        }
    }

    /// Check if this trap deals damage over time.
    #[must_use]
    pub fn is_dot(&self) -> bool {
        matches!(self, StaticTrapType::Gas)
    }

    /// Get the trigger radius for this trap.
    #[must_use]
    pub fn trigger_radius(&self) -> f32 {
        match self {
            StaticTrapType::Pit => 1.0,
            StaticTrapType::Spike => 0.5,
            StaticTrapType::Gas => 2.0,
            StaticTrapType::Arrow => 3.0,
        }
    }

    /// Get all static trap types.
    #[must_use]
    pub fn all() -> &'static [StaticTrapType] {
        &[
            StaticTrapType::Pit,
            StaticTrapType::Spike,
            StaticTrapType::Gas,
            StaticTrapType::Arrow,
        ]
    }
}

/// A static trap instance in the world.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StaticTrap {
    /// Type of trap.
    trap_type: StaticTrapType,
    /// Position in the world.
    position: IVec3,
    /// Whether the trap has been triggered.
    triggered: bool,
    /// Whether the trap has been discovered by the player.
    discovered: bool,
    /// Whether the trap is armed (can trigger).
    armed: bool,
    /// Cooldown ticks remaining before trap can trigger again.
    cooldown: u32,
}

impl StaticTrap {
    /// Create a new static trap.
    #[must_use]
    pub fn new(trap_type: StaticTrapType, position: IVec3) -> Self {
        Self {
            trap_type,
            position,
            triggered: false,
            discovered: false,
            armed: true,
            cooldown: 0,
        }
    }

    /// Get the trap type.
    #[must_use]
    pub fn trap_type(&self) -> StaticTrapType {
        self.trap_type
    }

    /// Get the position.
    #[must_use]
    pub fn position(&self) -> IVec3 {
        self.position
    }

    /// Check if the trap has been triggered.
    #[must_use]
    pub fn is_triggered(&self) -> bool {
        self.triggered
    }

    /// Check if the trap has been discovered.
    #[must_use]
    pub fn is_discovered(&self) -> bool {
        self.discovered
    }

    /// Check if the trap is armed.
    #[must_use]
    pub fn is_armed(&self) -> bool {
        self.armed && self.cooldown == 0
    }

    /// Mark the trap as discovered.
    pub fn discover(&mut self) {
        self.discovered = true;
    }

    /// Trigger the trap.
    ///
    /// Returns the damage dealt, or 0 if not armed.
    pub fn trigger(&mut self) -> u32 {
        if !self.is_armed() {
            return 0;
        }
        self.triggered = true;
        self.cooldown = 60; // 60 tick cooldown
        self.trap_type.base_damage()
    }

    /// Update the trap state (called each tick).
    pub fn update(&mut self) {
        if self.cooldown > 0 {
            self.cooldown -= 1;
        }
        if self.cooldown == 0 && self.triggered {
            self.triggered = false;
        }
    }

    /// Disarm the trap permanently.
    pub fn disarm(&mut self) {
        self.armed = false;
        self.triggered = false;
    }

    /// Re-arm the trap.
    pub fn rearm(&mut self) {
        self.armed = true;
    }

    /// Check if a position is within trigger range.
    #[must_use]
    pub fn in_trigger_range(&self, pos: IVec3) -> bool {
        let diff = pos - self.position;
        let distance = ((diff.x * diff.x + diff.y * diff.y + diff.z * diff.z) as f32).sqrt();
        distance <= self.trap_type.trigger_radius()
    }

    /// Get remaining cooldown ticks.
    #[must_use]
    pub fn cooldown(&self) -> u32 {
        self.cooldown
    }

    /// Calculate damage with loop scaling.
    #[must_use]
    pub fn scaled_damage(&self, loop_count: u32) -> u32 {
        let base = self.trap_type.base_damage();
        let scale = 1.0 + (loop_count.saturating_sub(1) as f32 * 0.1);
        (base as f32 * scale) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_trap_type_damage() {
        assert_eq!(StaticTrapType::Pit.base_damage(), 15);
        assert_eq!(StaticTrapType::Spike.base_damage(), 25);
        assert_eq!(StaticTrapType::Gas.base_damage(), 5);
        assert_eq!(StaticTrapType::Arrow.base_damage(), 20);
    }

    #[test]
    fn test_static_trap_type_display_names() {
        assert_eq!(StaticTrapType::Pit.display_name(), "Pit Trap");
        assert_eq!(StaticTrapType::Spike.display_name(), "Spike Trap");
        assert_eq!(StaticTrapType::Gas.display_name(), "Gas Trap");
        assert_eq!(StaticTrapType::Arrow.display_name(), "Arrow Trap");
    }

    #[test]
    fn test_static_trap_type_descriptions() {
        assert!(StaticTrapType::Pit.description().contains("pit"));
        assert!(StaticTrapType::Spike.description().contains("spikes"));
        assert!(StaticTrapType::Gas.description().contains("gas"));
        assert!(StaticTrapType::Arrow.description().contains("arrows"));
    }

    #[test]
    fn test_static_trap_type_is_dot() {
        assert!(!StaticTrapType::Pit.is_dot());
        assert!(!StaticTrapType::Spike.is_dot());
        assert!(StaticTrapType::Gas.is_dot());
        assert!(!StaticTrapType::Arrow.is_dot());
    }

    #[test]
    fn test_static_trap_type_trigger_radius() {
        assert!((StaticTrapType::Pit.trigger_radius() - 1.0).abs() < f32::EPSILON);
        assert!((StaticTrapType::Spike.trigger_radius() - 0.5).abs() < f32::EPSILON);
        assert!((StaticTrapType::Gas.trigger_radius() - 2.0).abs() < f32::EPSILON);
        assert!((StaticTrapType::Arrow.trigger_radius() - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_static_trap_type_all() {
        let all = StaticTrapType::all();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&StaticTrapType::Pit));
        assert!(all.contains(&StaticTrapType::Spike));
        assert!(all.contains(&StaticTrapType::Gas));
        assert!(all.contains(&StaticTrapType::Arrow));
    }

    #[test]
    fn test_static_trap_new() {
        let trap = StaticTrap::new(StaticTrapType::Spike, IVec3::new(10, 0, 5));

        assert_eq!(trap.trap_type(), StaticTrapType::Spike);
        assert_eq!(trap.position(), IVec3::new(10, 0, 5));
        assert!(!trap.is_triggered());
        assert!(!trap.is_discovered());
        assert!(trap.is_armed());
    }

    #[test]
    fn test_static_trap_discover() {
        let mut trap = StaticTrap::new(StaticTrapType::Pit, IVec3::ZERO);
        assert!(!trap.is_discovered());

        trap.discover();
        assert!(trap.is_discovered());
    }

    #[test]
    fn test_static_trap_trigger() {
        let mut trap = StaticTrap::new(StaticTrapType::Arrow, IVec3::ZERO);

        let damage = trap.trigger();
        assert_eq!(damage, 20);
        assert!(trap.is_triggered());
        assert_eq!(trap.cooldown(), 60);
    }

    #[test]
    fn test_static_trap_trigger_unarmed() {
        let mut trap = StaticTrap::new(StaticTrapType::Spike, IVec3::ZERO);
        trap.disarm();

        let damage = trap.trigger();
        assert_eq!(damage, 0);
    }

    #[test]
    fn test_static_trap_trigger_on_cooldown() {
        let mut trap = StaticTrap::new(StaticTrapType::Gas, IVec3::ZERO);
        trap.trigger();
        assert!(!trap.is_armed());

        let damage = trap.trigger();
        assert_eq!(damage, 0);
    }

    #[test]
    fn test_static_trap_update_cooldown() {
        let mut trap = StaticTrap::new(StaticTrapType::Pit, IVec3::ZERO);
        trap.trigger();
        assert_eq!(trap.cooldown(), 60);

        for _ in 0..30 {
            trap.update();
        }
        assert_eq!(trap.cooldown(), 30);

        for _ in 0..30 {
            trap.update();
        }
        assert_eq!(trap.cooldown(), 0);
        assert!(!trap.is_triggered());
        assert!(trap.is_armed());
    }

    #[test]
    fn test_static_trap_disarm() {
        let mut trap = StaticTrap::new(StaticTrapType::Spike, IVec3::ZERO);
        trap.disarm();

        assert!(!trap.is_armed());
        assert_eq!(trap.trigger(), 0);
    }

    #[test]
    fn test_static_trap_rearm() {
        let mut trap = StaticTrap::new(StaticTrapType::Arrow, IVec3::ZERO);
        trap.disarm();
        assert!(!trap.is_armed());

        trap.rearm();
        assert!(trap.is_armed());
    }

    #[test]
    fn test_static_trap_in_trigger_range() {
        let trap = StaticTrap::new(StaticTrapType::Gas, IVec3::ZERO);

        assert!(trap.in_trigger_range(IVec3::ZERO));
        assert!(trap.in_trigger_range(IVec3::new(1, 0, 0)));
        assert!(trap.in_trigger_range(IVec3::new(1, 1, 0)));
        assert!(!trap.in_trigger_range(IVec3::new(10, 0, 0)));
    }

    #[test]
    fn test_static_trap_scaled_damage() {
        let trap = StaticTrap::new(StaticTrapType::Spike, IVec3::ZERO);

        assert_eq!(trap.scaled_damage(1), 25);
        assert_eq!(trap.scaled_damage(2), 27); // 25 * 1.1
        assert_eq!(trap.scaled_damage(5), 35); // 25 * 1.4
    }
}
