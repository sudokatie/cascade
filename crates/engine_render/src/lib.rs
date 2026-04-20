//! Rendering system for the Titan game engine.
//!
//! Provides GPU abstraction, voxel rendering, and visual effects
//! including Titan-specific rendering for the colossus survival game.

pub mod backend;
pub mod camera;
pub mod dimension;
pub mod fog;
pub mod ghost_block;
pub mod lighting;
mod renderer;
pub mod sky;
pub mod titan;
pub mod voxel;

pub use renderer::{TriangleRenderer, Vertex};
