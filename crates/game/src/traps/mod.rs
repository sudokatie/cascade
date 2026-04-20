//! Trap systems for time-loop survival.
//!
//! Provides static traps, loop-aware traps, and paradox hazards.

mod loop_traps;
mod paradox_hazards;
mod static_traps;

pub use loop_traps::{LoopTrap, LoopTrapRegistry};
pub use paradox_hazards::{ParadoxHazard, ParadoxHazardManager, ParadoxHazardType};
pub use static_traps::{StaticTrap, StaticTrapType};
