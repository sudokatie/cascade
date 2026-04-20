//! Knowledge system for persistent discoveries across loops.
//!
//! Tracks what the player has learned about the world,
//! preserving knowledge across time loop resets.

pub mod discoveries;

pub use discoveries::{DiscoveryID, KnowledgeCategory, KnowledgeSystem};
