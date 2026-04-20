//! Temporal systems for time-loop survival gameplay.
//!
//! Provides state persistence across loops, loop management,
//! and paradox handling at the game logic level.

mod loop_manager;
mod paradox;
mod state_persistence;

pub use loop_manager::LoopManager;
pub use paradox::{GameParadox, ParadoxHandler};
pub use state_persistence::{StateCategory, StatePersistence, StateType};
