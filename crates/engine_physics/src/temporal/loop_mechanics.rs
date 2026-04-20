//! Time loop mechanics for the day/night cycle system.
//!
//! Manages the four phases of each loop (Dawn, Day, Dusk, Midnight)
//! and tracks loop progression for difficulty scaling.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Default day length in seconds.
pub const DEFAULT_DAY_LENGTH: f32 = 600.0;

/// Duration constants for each phase.
pub const DAWN_DURATION: f32 = 30.0;
pub const DAY_DURATION: f32 = 480.0;
pub const DUSK_DURATION: f32 = 30.0;
pub const MIDNIGHT_DURATION: f32 = 60.0;

/// Light levels for each phase.
pub const DAWN_LIGHT: f32 = 0.3;
pub const DAY_LIGHT: f32 = 1.0;
pub const DUSK_LIGHT: f32 = 0.5;
pub const MIDNIGHT_LIGHT: f32 = 0.1;

/// Danger modifiers for each phase.
pub const DAWN_DANGER: f32 = 0.5;
pub const DAY_DANGER: f32 = 1.0;
pub const DUSK_DANGER: f32 = 1.5;
pub const MIDNIGHT_DANGER: f32 = 2.0;

/// The four phases of a time loop day.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum LoopPhase {
    /// Early morning, relatively safe.
    #[default]
    Dawn,
    /// Main active period, normal danger.
    Day,
    /// Evening, increased danger.
    Dusk,
    /// Night, maximum danger. Loop resets after midnight.
    Midnight,
}

impl fmt::Display for LoopPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoopPhase::Dawn => write!(f, "Dawn"),
            LoopPhase::Day => write!(f, "Day"),
            LoopPhase::Dusk => write!(f, "Dusk"),
            LoopPhase::Midnight => write!(f, "Midnight"),
        }
    }
}

impl LoopPhase {
    /// Get all phase variants in order.
    #[must_use]
    pub fn all() -> &'static [LoopPhase] {
        &[
            LoopPhase::Dawn,
            LoopPhase::Day,
            LoopPhase::Dusk,
            LoopPhase::Midnight,
        ]
    }

    /// Get the next phase in the cycle.
    #[must_use]
    pub fn next(self) -> LoopPhase {
        match self {
            LoopPhase::Dawn => LoopPhase::Day,
            LoopPhase::Day => LoopPhase::Dusk,
            LoopPhase::Dusk => LoopPhase::Midnight,
            LoopPhase::Midnight => LoopPhase::Dawn,
        }
    }
}

/// Properties defining a loop phase's characteristics.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoopPhaseProperties {
    /// The phase these properties belong to.
    pub phase: LoopPhase,
    /// Duration of this phase in seconds.
    pub duration: f32,
    /// Ambient light level (0.0 to 1.0).
    pub light_level: f32,
    /// Danger multiplier for enemy spawns/aggression.
    pub danger_modifier: f32,
}

impl LoopPhaseProperties {
    /// Get properties for a specific phase.
    #[must_use]
    pub fn for_phase(phase: LoopPhase) -> Self {
        match phase {
            LoopPhase::Dawn => Self {
                phase,
                duration: DAWN_DURATION,
                light_level: DAWN_LIGHT,
                danger_modifier: DAWN_DANGER,
            },
            LoopPhase::Day => Self {
                phase,
                duration: DAY_DURATION,
                light_level: DAY_LIGHT,
                danger_modifier: DAY_DANGER,
            },
            LoopPhase::Dusk => Self {
                phase,
                duration: DUSK_DURATION,
                light_level: DUSK_LIGHT,
                danger_modifier: DUSK_DANGER,
            },
            LoopPhase::Midnight => Self {
                phase,
                duration: MIDNIGHT_DURATION,
                light_level: MIDNIGHT_LIGHT,
                danger_modifier: MIDNIGHT_DANGER,
            },
        }
    }
}

/// Core time loop mechanics manager.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoopMechanics {
    /// Current phase of the day.
    current_phase: LoopPhase,
    /// Time elapsed in the current phase.
    phase_timer: f32,
    /// Number of completed loops.
    loop_count: u32,
    /// Total length of a full day cycle.
    day_length: f32,
}

impl LoopMechanics {
    /// Create a new loop mechanics instance.
    ///
    /// Starts at Dawn, loop 1, with default day length.
    #[must_use]
    pub fn new() -> Self {
        Self {
            current_phase: LoopPhase::Dawn,
            phase_timer: 0.0,
            loop_count: 1,
            day_length: DEFAULT_DAY_LENGTH,
        }
    }

    /// Update the time loop by delta time.
    ///
    /// Returns `Some(new_phase)` if the phase changed, `None` otherwise.
    pub fn tick(&mut self, dt: f32) -> Option<LoopPhase> {
        self.phase_timer += dt;

        let properties = LoopPhaseProperties::for_phase(self.current_phase);

        if self.phase_timer >= properties.duration {
            self.phase_timer -= properties.duration;
            let old_phase = self.current_phase;
            self.current_phase = old_phase.next();

            // If we wrapped back to Dawn, the loop would reset via midnight timeout
            // but phase transition still happens
            return Some(self.current_phase);
        }

        None
    }

    /// Get the current phase.
    #[must_use]
    pub fn current_phase(&self) -> LoopPhase {
        self.current_phase
    }

    /// Get properties for the current phase.
    #[must_use]
    pub fn current_properties(&self) -> LoopPhaseProperties {
        LoopPhaseProperties::for_phase(self.current_phase)
    }

    /// Get the current loop count.
    #[must_use]
    pub fn loop_count(&self) -> u32 {
        self.loop_count
    }

    /// Get time remaining in the current phase.
    #[must_use]
    pub fn time_remaining(&self) -> f32 {
        let properties = LoopPhaseProperties::for_phase(self.current_phase);
        (properties.duration - self.phase_timer).max(0.0)
    }

    /// Reset the loop due to player death.
    ///
    /// Increments loop counter and resets to Dawn.
    /// Returns the new loop count.
    pub fn trigger_death_reset(&mut self) -> u32 {
        self.loop_count += 1;
        self.current_phase = LoopPhase::Dawn;
        self.phase_timer = 0.0;
        self.loop_count
    }

    /// Reset the loop due to midnight timeout.
    ///
    /// Same as death reset but triggered by time running out.
    /// Returns the new loop count.
    pub fn trigger_midnight_reset(&mut self) -> u32 {
        self.trigger_death_reset()
    }

    /// Break the loop (win condition).
    ///
    /// Resets the loop counter to 0, indicating escape from the loop.
    pub fn break_loop(&mut self) {
        self.loop_count = 0;
        self.current_phase = LoopPhase::Dawn;
        self.phase_timer = 0.0;
    }

    /// Get the difficulty modifier based on loop count.
    ///
    /// - Loops 1-5: 1.0 (normal)
    /// - Loops 6-15: 1.5 (hard)
    /// - Loops 16+: 2.0 (extreme)
    #[must_use]
    pub fn difficulty_modifier(&self) -> f32 {
        match self.loop_count {
            1..=5 => 1.0,
            6..=15 => 1.5,
            _ => 2.0,
        }
    }

    /// Get the total day length.
    #[must_use]
    pub fn day_length(&self) -> f32 {
        self.day_length
    }

    /// Get the phase timer value.
    #[must_use]
    pub fn phase_timer(&self) -> f32 {
        self.phase_timer
    }
}

impl Default for LoopMechanics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_phase_display() {
        assert_eq!(format!("{}", LoopPhase::Dawn), "Dawn");
        assert_eq!(format!("{}", LoopPhase::Day), "Day");
        assert_eq!(format!("{}", LoopPhase::Dusk), "Dusk");
        assert_eq!(format!("{}", LoopPhase::Midnight), "Midnight");
    }

    #[test]
    fn test_loop_phase_default() {
        assert_eq!(LoopPhase::default(), LoopPhase::Dawn);
    }

    #[test]
    fn test_loop_phase_all() {
        let all = LoopPhase::all();
        assert_eq!(all.len(), 4);
        assert_eq!(all[0], LoopPhase::Dawn);
        assert_eq!(all[1], LoopPhase::Day);
        assert_eq!(all[2], LoopPhase::Dusk);
        assert_eq!(all[3], LoopPhase::Midnight);
    }

    #[test]
    fn test_loop_phase_next() {
        assert_eq!(LoopPhase::Dawn.next(), LoopPhase::Day);
        assert_eq!(LoopPhase::Day.next(), LoopPhase::Dusk);
        assert_eq!(LoopPhase::Dusk.next(), LoopPhase::Midnight);
        assert_eq!(LoopPhase::Midnight.next(), LoopPhase::Dawn);
    }

    #[test]
    fn test_loop_phase_properties() {
        let dawn = LoopPhaseProperties::for_phase(LoopPhase::Dawn);
        assert!((dawn.duration - DAWN_DURATION).abs() < f32::EPSILON);
        assert!((dawn.light_level - DAWN_LIGHT).abs() < f32::EPSILON);
        assert!((dawn.danger_modifier - DAWN_DANGER).abs() < f32::EPSILON);

        let day = LoopPhaseProperties::for_phase(LoopPhase::Day);
        assert!((day.duration - DAY_DURATION).abs() < f32::EPSILON);
        assert!((day.light_level - DAY_LIGHT).abs() < f32::EPSILON);

        let midnight = LoopPhaseProperties::for_phase(LoopPhase::Midnight);
        assert!((midnight.danger_modifier - MIDNIGHT_DANGER).abs() < f32::EPSILON);
    }

    #[test]
    fn test_loop_mechanics_new() {
        let mechanics = LoopMechanics::new();
        assert_eq!(mechanics.current_phase(), LoopPhase::Dawn);
        assert_eq!(mechanics.loop_count(), 1);
        assert!((mechanics.day_length() - DEFAULT_DAY_LENGTH).abs() < f32::EPSILON);
        assert!((mechanics.phase_timer() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_loop_mechanics_tick_no_transition() {
        let mut mechanics = LoopMechanics::new();
        let result = mechanics.tick(10.0);
        assert!(result.is_none());
        assert_eq!(mechanics.current_phase(), LoopPhase::Dawn);
        assert!((mechanics.phase_timer() - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_loop_mechanics_tick_phase_transition() {
        let mut mechanics = LoopMechanics::new();
        // Dawn is 30 seconds
        let result = mechanics.tick(31.0);
        assert_eq!(result, Some(LoopPhase::Day));
        assert_eq!(mechanics.current_phase(), LoopPhase::Day);
    }

    #[test]
    fn test_loop_mechanics_time_remaining() {
        let mut mechanics = LoopMechanics::new();
        mechanics.tick(10.0);
        let remaining = mechanics.time_remaining();
        assert!((remaining - 20.0).abs() < f32::EPSILON); // 30 - 10 = 20
    }

    #[test]
    fn test_loop_mechanics_death_reset() {
        let mut mechanics = LoopMechanics::new();
        mechanics.tick(100.0); // Advance into Day phase
        let new_count = mechanics.trigger_death_reset();
        assert_eq!(new_count, 2);
        assert_eq!(mechanics.loop_count(), 2);
        assert_eq!(mechanics.current_phase(), LoopPhase::Dawn);
        assert!((mechanics.phase_timer() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_loop_mechanics_break_loop() {
        let mut mechanics = LoopMechanics::new();
        mechanics.trigger_death_reset();
        mechanics.trigger_death_reset();
        assert_eq!(mechanics.loop_count(), 3);

        mechanics.break_loop();
        assert_eq!(mechanics.loop_count(), 0);
        assert_eq!(mechanics.current_phase(), LoopPhase::Dawn);
    }

    #[test]
    fn test_loop_mechanics_difficulty_modifier() {
        let mut mechanics = LoopMechanics::new();
        assert!((mechanics.difficulty_modifier() - 1.0).abs() < f32::EPSILON);

        // Advance to loop 6
        for _ in 0..5 {
            mechanics.trigger_death_reset();
        }
        assert_eq!(mechanics.loop_count(), 6);
        assert!((mechanics.difficulty_modifier() - 1.5).abs() < f32::EPSILON);

        // Advance to loop 16
        for _ in 0..10 {
            mechanics.trigger_death_reset();
        }
        assert_eq!(mechanics.loop_count(), 16);
        assert!((mechanics.difficulty_modifier() - 2.0).abs() < f32::EPSILON);
    }
}
