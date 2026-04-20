//! Titan rendering systems.
//!
//! Provides visual effects, terrain animation, and rendering data
//! for the living Titan colossus.

mod terrain_animation;
mod visual_effects;

pub use terrain_animation::TerrainAnimation;
pub use visual_effects::{TitanVisuals, TitanZone};
