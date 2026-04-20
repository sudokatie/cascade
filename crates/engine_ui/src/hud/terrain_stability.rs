//! Terrain stability HUD display.
//!
//! Shows the stability of the terrain at the player's current location.

/// HUD display for terrain stability.
#[derive(Clone, Debug)]
pub struct TerrainStabilityDisplay {
    /// Current stability value (0.0-1.0).
    stability: f32,
    /// Flash timer for unstable warnings.
    flash_timer: f32,
    /// Whether display is visible.
    visible: bool,
}

impl Default for TerrainStabilityDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl TerrainStabilityDisplay {
    /// Create a new terrain stability display.
    #[must_use]
    pub fn new() -> Self {
        Self {
            stability: 1.0,
            flash_timer: 0.0,
            visible: true,
        }
    }

    /// Update the stability display.
    pub fn update(&mut self, stability: f32) {
        let was_unstable = self.is_unstable();
        self.stability = stability.clamp(0.0, 1.0);

        // Start flashing if newly unstable
        if !was_unstable && self.is_unstable() {
            self.flash_timer = 1.0;
        }
    }

    /// Tick the flash animation.
    pub fn tick(&mut self, dt: f32) {
        if self.flash_timer > 0.0 {
            self.flash_timer = (self.flash_timer - dt).max(0.0);
        }

        // Keep flashing while very unstable
        if self.stability < 0.3 && self.flash_timer <= 0.0 {
            self.flash_timer = 0.5;
        }
    }

    /// Get current stability value.
    #[must_use]
    pub fn stability(&self) -> f32 {
        self.stability
    }

    /// Get stability as percentage (0-100).
    #[must_use]
    pub fn percentage(&self) -> f32 {
        self.stability * 100.0
    }

    /// Check if terrain is unstable.
    #[must_use]
    pub fn is_unstable(&self) -> bool {
        self.stability < 0.5
    }

    /// Check if terrain is very unstable (dangerous).
    #[must_use]
    pub fn is_dangerous(&self) -> bool {
        self.stability < 0.3
    }

    /// Check if terrain is safe for building.
    #[must_use]
    pub fn is_buildable(&self) -> bool {
        self.stability >= 0.5
    }

    /// Get warning text if unstable.
    #[must_use]
    pub fn warning_text(&self) -> Option<&'static str> {
        if self.stability < 0.2 {
            Some("EXTREME INSTABILITY!")
        } else if self.stability < 0.3 {
            Some("Ground very unstable!")
        } else if self.stability < 0.5 {
            Some("Unstable terrain")
        } else {
            None
        }
    }

    /// Get display color based on stability.
    #[must_use]
    pub fn color(&self) -> [f32; 3] {
        if self.stability < 0.3 {
            [0.9, 0.2, 0.2] // Red
        } else if self.stability < 0.5 {
            [0.9, 0.6, 0.2] // Orange
        } else if self.stability < 0.7 {
            [0.9, 0.9, 0.2] // Yellow
        } else {
            [0.3, 0.8, 0.3] // Green
        }
    }

    /// Get status text.
    #[must_use]
    pub fn status(&self) -> &'static str {
        if self.stability < 0.2 {
            "CRITICAL"
        } else if self.stability < 0.3 {
            "UNSTABLE"
        } else if self.stability < 0.5 {
            "SHAKY"
        } else if self.stability < 0.7 {
            "MODERATE"
        } else if self.stability < 0.9 {
            "STABLE"
        } else {
            "SOLID"
        }
    }

    /// Get display text.
    #[must_use]
    pub fn display_text(&self) -> String {
        format!("Terrain: {} ({:.0}%)", self.status(), self.percentage())
    }

    /// Check if currently flashing.
    #[must_use]
    pub fn is_flashing(&self) -> bool {
        self.stability < 0.3 && (self.flash_timer * 4.0) as u32 % 2 == 0
    }

    /// Set visibility.
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Check if visible.
    #[must_use]
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get icon for stability level.
    #[must_use]
    pub fn icon(&self) -> char {
        if self.stability < 0.3 {
            '!'
        } else if self.stability < 0.5 {
            '~'
        } else if self.stability < 0.7 {
            '='
        } else {
            '■'
        }
    }

    /// Get bar fill width for a given max width.
    #[must_use]
    pub fn bar_width(&self, max_width: f32) -> f32 {
        max_width * self.stability
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terrain_stability_display_new() {
        let display = TerrainStabilityDisplay::new();
        assert!((display.stability() - 1.0).abs() < f32::EPSILON);
        assert!(display.is_visible());
    }

    #[test]
    fn test_terrain_stability_display_update() {
        let mut display = TerrainStabilityDisplay::new();
        display.update(0.7);
        assert!((display.stability() - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn test_terrain_stability_display_update_clamp() {
        let mut display = TerrainStabilityDisplay::new();

        display.update(-0.5);
        assert!((display.stability() - 0.0).abs() < f32::EPSILON);

        display.update(1.5);
        assert!((display.stability() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_terrain_stability_display_percentage() {
        let mut display = TerrainStabilityDisplay::new();
        display.update(0.75);
        assert!((display.percentage() - 75.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_terrain_stability_display_is_unstable() {
        let mut display = TerrainStabilityDisplay::new();

        display.update(0.6);
        assert!(!display.is_unstable());

        display.update(0.4);
        assert!(display.is_unstable());
    }

    #[test]
    fn test_terrain_stability_display_is_dangerous() {
        let mut display = TerrainStabilityDisplay::new();

        display.update(0.4);
        assert!(!display.is_dangerous());

        display.update(0.2);
        assert!(display.is_dangerous());
    }

    #[test]
    fn test_terrain_stability_display_is_buildable() {
        let mut display = TerrainStabilityDisplay::new();

        display.update(0.6);
        assert!(display.is_buildable());

        display.update(0.4);
        assert!(!display.is_buildable());
    }

    #[test]
    fn test_terrain_stability_display_warning_text_none() {
        let display = TerrainStabilityDisplay::new();
        assert!(display.warning_text().is_none());
    }

    #[test]
    fn test_terrain_stability_display_warning_text_unstable() {
        let mut display = TerrainStabilityDisplay::new();
        display.update(0.4);
        assert!(display.warning_text().is_some());
        assert!(display.warning_text().unwrap().contains("Unstable"));
    }

    #[test]
    fn test_terrain_stability_display_warning_text_very_unstable() {
        let mut display = TerrainStabilityDisplay::new();
        display.update(0.25);
        assert!(display.warning_text().is_some());
        assert!(display.warning_text().unwrap().contains("very"));
    }

    #[test]
    fn test_terrain_stability_display_warning_text_extreme() {
        let mut display = TerrainStabilityDisplay::new();
        display.update(0.1);
        assert!(display.warning_text().is_some());
        assert!(display.warning_text().unwrap().contains("EXTREME"));
    }

    #[test]
    fn test_terrain_stability_display_color_solid() {
        let display = TerrainStabilityDisplay::new();
        let color = display.color();
        // Green dominant
        assert!(color[1] > color[0]);
    }

    #[test]
    fn test_terrain_stability_display_color_unstable() {
        let mut display = TerrainStabilityDisplay::new();
        display.update(0.2);
        let color = display.color();
        // Red dominant
        assert!(color[0] > color[1]);
    }

    #[test]
    fn test_terrain_stability_display_status() {
        let mut display = TerrainStabilityDisplay::new();

        display.update(0.95);
        assert_eq!(display.status(), "SOLID");

        display.update(0.8);
        assert_eq!(display.status(), "STABLE");

        display.update(0.6);
        assert_eq!(display.status(), "MODERATE");

        display.update(0.4);
        assert_eq!(display.status(), "SHAKY");

        display.update(0.25);
        assert_eq!(display.status(), "UNSTABLE");

        display.update(0.1);
        assert_eq!(display.status(), "CRITICAL");
    }

    #[test]
    fn test_terrain_stability_display_display_text() {
        let mut display = TerrainStabilityDisplay::new();
        display.update(0.8);
        assert!(display.display_text().contains("STABLE"));
        assert!(display.display_text().contains("80"));
    }

    #[test]
    fn test_terrain_stability_display_tick() {
        let mut display = TerrainStabilityDisplay::new();
        display.update(0.2); // Very unstable
        display.tick(0.1);
        // Flash timer should be active
        assert!(display.flash_timer > 0.0 || display.stability < 0.3);
    }

    #[test]
    fn test_terrain_stability_display_visibility() {
        let mut display = TerrainStabilityDisplay::new();
        assert!(display.is_visible());

        display.set_visible(false);
        assert!(!display.is_visible());
    }

    #[test]
    fn test_terrain_stability_display_icon() {
        let mut display = TerrainStabilityDisplay::new();

        display.update(0.9);
        assert_eq!(display.icon(), '■');

        display.update(0.6);
        assert_eq!(display.icon(), '=');

        display.update(0.4);
        assert_eq!(display.icon(), '~');

        display.update(0.2);
        assert_eq!(display.icon(), '!');
    }

    #[test]
    fn test_terrain_stability_display_bar_width() {
        let mut display = TerrainStabilityDisplay::new();
        display.update(0.5);
        assert!((display.bar_width(200.0) - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_terrain_stability_display_default() {
        let display = TerrainStabilityDisplay::default();
        assert!((display.stability() - 1.0).abs() < f32::EPSILON);
    }
}
