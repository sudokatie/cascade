//! Temporal rendering systems for time-loop survival.
//!
//! Provides visual effects for loop transitions, paradox, ghost items, and time-based lighting.

mod time_effects;
mod visual_effects;

pub use time_effects::{LoopTimePhase, LoopVignette, TimeDilationEffect, TimeEffects};
pub use visual_effects::{
    ChestAuraEffect, EffectColor, GhostItemEffect, LoopTransitionEffect, ParadoxGlowEffect,
    TemporalVisuals,
};
