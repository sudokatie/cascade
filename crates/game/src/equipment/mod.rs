//! Equipment systems for Titan survival.
//!
//! Provides balance gear for maintaining stability on the moving Titan.

mod balance_gear;
mod phase_suits;
pub mod stability_equip;

pub use balance_gear::{BalanceEquipment, BalanceGear};
pub use phase_suits::{PhaseSuit, PhaseSuitTier, MAX_DURABILITY};
pub use stability_equip::{AnchorBuilder, StabilityDetector, VoidTether};
