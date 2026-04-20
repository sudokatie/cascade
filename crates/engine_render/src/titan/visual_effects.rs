//! Titan visual effects.
//!
//! Provides visual effects and rendering data for the Titan's surface,
//! including zone-specific textures and movement parallax.

use engine_physics::titan::TitanPhase;

/// Re-export TitanZone from game crate for convenience.
/// In a real implementation, this would reference the game crate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TitanZone {
    #[default]
    ShellRidge,
    ScaleValley,
    ParasiteForest,
    BreathingVent,
    WoundSite,
    NeuralNode,
}

/// Visual effects handler for Titan rendering.
#[derive(Clone, Debug)]
pub struct TitanVisuals {
    /// Current visual intensity (affects all effects).
    intensity: f32,
    /// Whether effects are enabled.
    enabled: bool,
}

impl Default for TitanVisuals {
    fn default() -> Self {
        Self::new()
    }
}

impl TitanVisuals {
    /// Create a new TitanVisuals handler.
    #[must_use]
    pub fn new() -> Self {
        Self {
            intensity: 1.0,
            enabled: true,
        }
    }

    /// Get the shell texture color for a zone.
    ///
    /// Returns RGBA color as [f32; 4].
    #[must_use]
    pub fn get_shell_texture(&self, zone: TitanZone) -> [f32; 4] {
        if !self.enabled {
            return [0.5, 0.5, 0.5, 1.0]; // Gray fallback
        }

        let base_color = match zone {
            TitanZone::ShellRidge => [0.6, 0.55, 0.45, 1.0],      // Tan/bone
            TitanZone::ScaleValley => [0.4, 0.45, 0.35, 1.0],     // Greenish gray
            TitanZone::ParasiteForest => [0.3, 0.5, 0.3, 1.0],    // Dark green
            TitanZone::BreathingVent => [0.8, 0.4, 0.2, 1.0],     // Orange/heat
            TitanZone::WoundSite => [0.7, 0.2, 0.2, 1.0],         // Blood red
            TitanZone::NeuralNode => [0.5, 0.4, 0.8, 1.0],        // Purple/neural
        };

        // Apply intensity
        [
            base_color[0] * self.intensity,
            base_color[1] * self.intensity,
            base_color[2] * self.intensity,
            base_color[3],
        ]
    }

    /// Get the movement parallax factor for the current phase.
    ///
    /// Higher values indicate more visible movement effects.
    #[must_use]
    pub fn get_movement_parallax(&self, phase: TitanPhase) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        let base_parallax = match phase {
            TitanPhase::Resting => 0.0,
            TitanPhase::Walking => 0.3,
            TitanPhase::Running => 0.8,
            TitanPhase::Scratching => 0.5,
        };

        base_parallax * self.intensity
    }

    /// Get shake intensity for camera effects.
    #[must_use]
    pub fn get_camera_shake(&self, phase: TitanPhase) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        let base_shake = match phase {
            TitanPhase::Resting => 0.0,
            TitanPhase::Walking => 0.1,
            TitanPhase::Running => 0.4,
            TitanPhase::Scratching => 0.6,
        };

        base_shake * self.intensity
    }

    /// Get fog density modifier for a zone.
    #[must_use]
    pub fn get_fog_density(&self, zone: TitanZone) -> f32 {
        if !self.enabled {
            return 0.5;
        }

        match zone {
            TitanZone::ShellRidge => 0.2,
            TitanZone::ScaleValley => 0.4,
            TitanZone::ParasiteForest => 0.7,
            TitanZone::BreathingVent => 0.9,
            TitanZone::WoundSite => 0.6,
            TitanZone::NeuralNode => 0.3,
        }
    }

    /// Get ambient light color for a zone.
    #[must_use]
    pub fn get_ambient_light(&self, zone: TitanZone) -> [f32; 3] {
        if !self.enabled {
            return [0.5, 0.5, 0.5];
        }

        match zone {
            TitanZone::ShellRidge => [0.8, 0.8, 0.75],
            TitanZone::ScaleValley => [0.6, 0.65, 0.6],
            TitanZone::ParasiteForest => [0.4, 0.5, 0.35],
            TitanZone::BreathingVent => [0.9, 0.6, 0.4],
            TitanZone::WoundSite => [0.8, 0.5, 0.5],
            TitanZone::NeuralNode => [0.6, 0.5, 0.8],
        }
    }

    /// Set the visual intensity (0.0-2.0).
    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity.clamp(0.0, 2.0);
    }

    /// Get current intensity.
    #[must_use]
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    /// Enable or disable effects.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if effects are enabled.
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_titan_visuals_new() {
        let visuals = TitanVisuals::new();
        assert!((visuals.intensity() - 1.0).abs() < f32::EPSILON);
        assert!(visuals.is_enabled());
    }

    #[test]
    fn test_titan_visuals_shell_texture_shell_ridge() {
        let visuals = TitanVisuals::new();
        let color = visuals.get_shell_texture(TitanZone::ShellRidge);
        assert!((color[0] - 0.6).abs() < f32::EPSILON);
        assert!((color[3] - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_visuals_shell_texture_wound_site() {
        let visuals = TitanVisuals::new();
        let color = visuals.get_shell_texture(TitanZone::WoundSite);
        // Red component should be highest
        assert!(color[0] > color[1]);
        assert!(color[0] > color[2]);
    }

    #[test]
    fn test_titan_visuals_shell_texture_neural_node() {
        let visuals = TitanVisuals::new();
        let color = visuals.get_shell_texture(TitanZone::NeuralNode);
        // Purple - blue component highest
        assert!(color[2] > color[0]);
    }

    #[test]
    fn test_titan_visuals_shell_texture_with_intensity() {
        let mut visuals = TitanVisuals::new();
        visuals.set_intensity(0.5);
        let color = visuals.get_shell_texture(TitanZone::ShellRidge);
        assert!((color[0] - 0.3).abs() < f32::EPSILON); // 0.6 * 0.5
    }

    #[test]
    fn test_titan_visuals_shell_texture_disabled() {
        let mut visuals = TitanVisuals::new();
        visuals.set_enabled(false);
        let color = visuals.get_shell_texture(TitanZone::WoundSite);
        // Should return gray fallback
        assert!((color[0] - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_visuals_movement_parallax_resting() {
        let visuals = TitanVisuals::new();
        let parallax = visuals.get_movement_parallax(TitanPhase::Resting);
        assert!((parallax - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_visuals_movement_parallax_walking() {
        let visuals = TitanVisuals::new();
        let parallax = visuals.get_movement_parallax(TitanPhase::Walking);
        assert!((parallax - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_visuals_movement_parallax_running() {
        let visuals = TitanVisuals::new();
        let parallax = visuals.get_movement_parallax(TitanPhase::Running);
        assert!((parallax - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_visuals_movement_parallax_scratching() {
        let visuals = TitanVisuals::new();
        let parallax = visuals.get_movement_parallax(TitanPhase::Scratching);
        assert!((parallax - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_visuals_movement_parallax_disabled() {
        let mut visuals = TitanVisuals::new();
        visuals.set_enabled(false);
        let parallax = visuals.get_movement_parallax(TitanPhase::Running);
        assert!((parallax - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_visuals_camera_shake() {
        let visuals = TitanVisuals::new();

        assert!((visuals.get_camera_shake(TitanPhase::Resting) - 0.0).abs() < f32::EPSILON);
        assert!((visuals.get_camera_shake(TitanPhase::Walking) - 0.1).abs() < f32::EPSILON);
        assert!((visuals.get_camera_shake(TitanPhase::Running) - 0.4).abs() < f32::EPSILON);
        assert!((visuals.get_camera_shake(TitanPhase::Scratching) - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_visuals_fog_density() {
        let visuals = TitanVisuals::new();

        assert!((visuals.get_fog_density(TitanZone::ShellRidge) - 0.2).abs() < f32::EPSILON);
        assert!((visuals.get_fog_density(TitanZone::BreathingVent) - 0.9).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_visuals_ambient_light() {
        let visuals = TitanVisuals::new();

        let shell_light = visuals.get_ambient_light(TitanZone::ShellRidge);
        assert!(shell_light[0] > 0.7);

        let vent_light = visuals.get_ambient_light(TitanZone::BreathingVent);
        assert!(vent_light[0] > vent_light[2]); // More red than blue
    }

    #[test]
    fn test_titan_visuals_set_intensity_clamp() {
        let mut visuals = TitanVisuals::new();

        visuals.set_intensity(-1.0);
        assert!((visuals.intensity() - 0.0).abs() < f32::EPSILON);

        visuals.set_intensity(5.0);
        assert!((visuals.intensity() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_visuals_default() {
        let visuals = TitanVisuals::default();
        // Default calls new() which initializes with proper values
        assert!((visuals.intensity() - 1.0).abs() < f32::EPSILON);
        assert!(visuals.is_enabled());
    }

    #[test]
    fn test_titan_zone_default() {
        assert_eq!(TitanZone::default(), TitanZone::ShellRidge);
    }
}
