//! Titan creature systems.
//!
//! Provides hostile and passive creatures that live on the Titan's body.

mod hostile;
mod passive;

pub use hostile::{
    AbilityResult, HostileCreature, HostileSpawnCondition, HostileType, MoodRequirement,
    SpecialAbilityInfo, TimeOfDay,
};
pub use passive::{PassiveCreature, PassiveSpawnCondition, PassiveType};
