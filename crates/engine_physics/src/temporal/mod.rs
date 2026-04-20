//! Temporal mechanics for time-loop survival gameplay.
//!
//! Provides the core loop mechanics, phase tracking, and paradox resolution
//! for the Cascade time-loop game.

mod loop_mechanics;
mod paradox_resolution;

pub use loop_mechanics::{LoopMechanics, LoopPhase, LoopPhaseProperties};
pub use paradox_resolution::{Paradox, ParadoxResolution, ParadoxTracker, ParadoxType};
