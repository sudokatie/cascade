//! Titan anatomy and behavior systems.
//!
//! Provides zone management, behavioral state tracking, and
//! Titan-specific mechanics for the survival game.

mod anatomy;
mod behavior;

pub use anatomy::{TitanZone, ZoneProperties};
pub use behavior::{
    TitanBehavior, TitanMood, AGITATED_THRESHOLD, ENRAGED_THRESHOLD, HARVEST_AGITATION,
    MAX_TITAN_HP, PARASITE_KILL_RELIEF,
};

// Re-export engine types for convenience
pub use engine_physics::titan::{PhaseProperties, TitanMovement, TitanPhase};
