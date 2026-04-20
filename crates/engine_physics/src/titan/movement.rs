//! Titan movement system for the living colossus.
//!
//! Tracks the current phase of Titan's movement and provides
//! world origin shifting for the survival game.

use std::fmt;

use glam::IVec3;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// The current movement phase of the Titan.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum TitanPhase {
    /// Titan is at rest, minimal movement.
    #[default]
    Resting,
    /// Titan is walking slowly.
    Walking,
    /// Titan is running, high instability.
    Running,
    /// Titan is scratching itself, localized movement.
    Scratching,
}

impl fmt::Display for TitanPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TitanPhase::Resting => write!(f, "Resting"),
            TitanPhase::Walking => write!(f, "Walking"),
            TitanPhase::Running => write!(f, "Running"),
            TitanPhase::Scratching => write!(f, "Scratching"),
        }
    }
}

impl TitanPhase {
    /// Get all phase variants.
    #[must_use]
    pub fn all() -> &'static [TitanPhase] {
        &[
            TitanPhase::Resting,
            TitanPhase::Walking,
            TitanPhase::Running,
            TitanPhase::Scratching,
        ]
    }
}

/// Properties associated with each Titan phase.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhaseProperties {
    /// The phase these properties describe.
    pub phase: TitanPhase,
    /// Blocks shifted per tick.
    pub shift_rate: i32,
    /// Duration range in seconds (min, max).
    pub duration_range: (f32, f32),
    /// Wind force applied to entities.
    pub wind_force: f32,
    /// Stability modifier for structures (1.0 = stable).
    pub stability_modifier: f32,
}

impl PhaseProperties {
    /// Get properties for a specific phase.
    #[must_use]
    pub fn for_phase(phase: TitanPhase) -> Self {
        match phase {
            TitanPhase::Resting => Self {
                phase,
                shift_rate: 0,
                duration_range: (30.0, 60.0),
                wind_force: 0.0,
                stability_modifier: 1.0,
            },
            TitanPhase::Walking => Self {
                phase,
                shift_rate: 2,
                duration_range: (20.0, 40.0),
                wind_force: 0.3,
                stability_modifier: 0.8,
            },
            TitanPhase::Running => Self {
                phase,
                shift_rate: 7,
                duration_range: (10.0, 20.0),
                wind_force: 0.8,
                stability_modifier: 0.4,
            },
            TitanPhase::Scratching => Self {
                phase,
                shift_rate: 3,
                duration_range: (5.0, 15.0),
                wind_force: 0.5,
                stability_modifier: 0.6,
            },
        }
    }
}

/// Manages the Titan's movement state and world origin shifting.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TitanMovement {
    /// Current movement phase.
    current_phase: TitanPhase,
    /// Time remaining in current phase.
    phase_timer: f32,
    /// Accumulated world offset from origin.
    world_offset: IVec3,
    /// Total shift distance traveled.
    total_shift: IVec3,
    /// Current day (affects phase durations).
    current_day: u32,
}

impl TitanMovement {
    /// Create a new Titan movement manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            current_phase: TitanPhase::Resting,
            phase_timer: 30.0,
            world_offset: IVec3::ZERO,
            total_shift: IVec3::ZERO,
            current_day: 1,
        }
    }

    /// Update the movement system.
    ///
    /// Returns `Some(new_phase)` if the phase changed this tick.
    pub fn tick(&mut self, dt: f32) -> Option<TitanPhase> {
        self.phase_timer -= dt;

        if self.phase_timer <= 0.0 {
            let new_phase = self.select_next_phase();
            self.transition_to(new_phase);
            return Some(new_phase);
        }

        // Apply shift based on current phase
        let props = self.current_properties();
        if props.shift_rate > 0 {
            let shift = IVec3::new(props.shift_rate, 0, 0);
            self.world_offset += shift;
            self.total_shift += shift;
        }

        None
    }

    /// Get the current movement phase.
    #[must_use]
    pub fn current_phase(&self) -> TitanPhase {
        self.current_phase
    }

    /// Get properties for the current phase.
    #[must_use]
    pub fn current_properties(&self) -> PhaseProperties {
        let mut props = PhaseProperties::for_phase(self.current_phase);
        // Adjust duration based on day (more walking at higher days)
        if self.current_day > 1 && self.current_phase == TitanPhase::Walking {
            let day_modifier = 1.0 + (self.current_day as f32 - 1.0) * 0.1;
            props.duration_range.0 *= day_modifier;
            props.duration_range.1 *= day_modifier;
        }
        props
    }

    /// Get the current world offset.
    #[must_use]
    pub fn world_offset(&self) -> IVec3 {
        self.world_offset
    }

    /// Set the current phase (for testing).
    pub fn set_phase(&mut self, phase: TitanPhase) {
        self.transition_to(phase);
    }

    /// Set the current day (affects phase durations).
    pub fn set_day(&mut self, day: u32) {
        self.current_day = day.max(1);
    }

    fn select_next_phase(&self) -> TitanPhase {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen_range(0.0..1.0);

        // Weight transitions based on current phase
        match self.current_phase {
            TitanPhase::Resting => {
                if roll < 0.6 {
                    TitanPhase::Walking
                } else if roll < 0.85 {
                    TitanPhase::Scratching
                } else {
                    TitanPhase::Running
                }
            }
            TitanPhase::Walking => {
                if roll < 0.4 {
                    TitanPhase::Resting
                } else if roll < 0.7 {
                    TitanPhase::Running
                } else {
                    TitanPhase::Scratching
                }
            }
            TitanPhase::Running => {
                if roll < 0.5 {
                    TitanPhase::Walking
                } else if roll < 0.8 {
                    TitanPhase::Resting
                } else {
                    TitanPhase::Scratching
                }
            }
            TitanPhase::Scratching => {
                if roll < 0.5 {
                    TitanPhase::Resting
                } else if roll < 0.8 {
                    TitanPhase::Walking
                } else {
                    TitanPhase::Running
                }
            }
        }
    }

    fn transition_to(&mut self, phase: TitanPhase) {
        self.current_phase = phase;
        let props = self.current_properties();
        let mut rng = rand::thread_rng();
        self.phase_timer = rng.r#gen_range(props.duration_range.0..=props.duration_range.1);
    }
}

impl Default for TitanMovement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_titan_phase_display() {
        assert_eq!(format!("{}", TitanPhase::Resting), "Resting");
        assert_eq!(format!("{}", TitanPhase::Walking), "Walking");
        assert_eq!(format!("{}", TitanPhase::Running), "Running");
        assert_eq!(format!("{}", TitanPhase::Scratching), "Scratching");
    }

    #[test]
    fn test_titan_phase_default() {
        assert_eq!(TitanPhase::default(), TitanPhase::Resting);
    }

    #[test]
    fn test_titan_phase_all() {
        let all = TitanPhase::all();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&TitanPhase::Resting));
        assert!(all.contains(&TitanPhase::Walking));
        assert!(all.contains(&TitanPhase::Running));
        assert!(all.contains(&TitanPhase::Scratching));
    }

    #[test]
    fn test_phase_properties_resting() {
        let props = PhaseProperties::for_phase(TitanPhase::Resting);
        assert_eq!(props.phase, TitanPhase::Resting);
        assert_eq!(props.shift_rate, 0);
        assert!((props.duration_range.0 - 30.0).abs() < f32::EPSILON);
        assert!((props.duration_range.1 - 60.0).abs() < f32::EPSILON);
        assert!((props.wind_force - 0.0).abs() < f32::EPSILON);
        assert!((props.stability_modifier - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_phase_properties_walking() {
        let props = PhaseProperties::for_phase(TitanPhase::Walking);
        assert_eq!(props.shift_rate, 2);
        assert!((props.wind_force - 0.3).abs() < f32::EPSILON);
        assert!((props.stability_modifier - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_phase_properties_running() {
        let props = PhaseProperties::for_phase(TitanPhase::Running);
        assert_eq!(props.shift_rate, 7);
        assert!((props.wind_force - 0.8).abs() < f32::EPSILON);
        assert!((props.stability_modifier - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn test_phase_properties_scratching() {
        let props = PhaseProperties::for_phase(TitanPhase::Scratching);
        assert_eq!(props.shift_rate, 3);
        assert!((props.wind_force - 0.5).abs() < f32::EPSILON);
        assert!((props.stability_modifier - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_movement_new() {
        let movement = TitanMovement::new();
        assert_eq!(movement.current_phase(), TitanPhase::Resting);
        assert_eq!(movement.world_offset(), IVec3::ZERO);
    }

    #[test]
    fn test_titan_movement_set_phase() {
        let mut movement = TitanMovement::new();
        movement.set_phase(TitanPhase::Running);
        assert_eq!(movement.current_phase(), TitanPhase::Running);
    }

    #[test]
    fn test_titan_movement_set_day() {
        let mut movement = TitanMovement::new();
        movement.set_day(5);
        movement.set_phase(TitanPhase::Walking);
        let props = movement.current_properties();
        // Day 5 should increase walking duration by 40%
        assert!(props.duration_range.0 > 20.0);
    }

    #[test]
    fn test_titan_movement_tick_no_phase_change() {
        let mut movement = TitanMovement::new();
        // Small tick shouldn't change phase
        let result = movement.tick(1.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_titan_movement_tick_phase_change() {
        let mut movement = TitanMovement::new();
        // Large tick should trigger phase change
        let result = movement.tick(100.0);
        assert!(result.is_some());
        assert_ne!(movement.current_phase(), TitanPhase::Resting);
    }
}
