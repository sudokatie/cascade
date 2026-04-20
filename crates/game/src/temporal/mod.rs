//! Temporal systems for time-loop survival gameplay.
//!
//! Provides state persistence across loops, loop management,
//! and paradox handling at the game logic level.

pub mod loop_manager;
pub mod paradox;
pub mod state_persistence;

pub use loop_manager::LoopManager;
pub use paradox::{GameParadox, ParadoxHandler};
pub use state_persistence::{StateCategory, StatePersistence, StateType};
