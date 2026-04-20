//! Temporal chest system for persistent storage across loops.
//!
//! Temporal chests preserve their contents across time loops,
//! allowing players to store items for their future selves.

pub mod chest;

pub use chest::{ItemStack, TemporalChest};
