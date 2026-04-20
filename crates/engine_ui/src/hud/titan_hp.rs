//! Titan HP HUD display.
//!
//! Shows the Titan's health, which affects game progression.

/// HUD display for Titan HP.
#[derive(Clone, Debug)]
pub struct TitanHPDisplay {
    /// Current HP.
    hp: f32,
    /// Maximum HP.
    max_hp: f32,
    /// Whether display is visible.
    visible: bool,
}

impl Default for TitanHPDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl TitanHPDisplay {
    /// Create a new Titan HP display.
    #[must_use]
    pub fn new() -> Self {
        Self {
            hp: 10000.0,
            max_hp: 10000.0,
            visible: true,
        }
    }

    /// Create with specific max HP.
    #[must_use]
    pub fn with_max(max_hp: f32) -> Self {
        Self {
            hp: max_hp,
            max_hp,
            visible: true,
        }
    }

    /// Update the HP display.
    pub fn update(&mut self, hp: f32) {
        self.hp = hp.clamp(0.0, self.max_hp);
    }

    /// Update both HP and max HP.
    pub fn update_full(&mut self, hp: f32, max_hp: f32) {
        self.max_hp = max_hp.max(1.0);
        self.hp = hp.clamp(0.0, self.max_hp);
    }

    /// Get current HP.
    #[must_use]
    pub fn hp(&self) -> f32 {
        self.hp
    }

    /// Get maximum HP.
    #[must_use]
    pub fn max_hp(&self) -> f32 {
        self.max_hp
    }

    /// Get HP as percentage (0.0-1.0).
    #[must_use]
    pub fn percentage(&self) -> f32 {
        if self.max_hp > 0.0 {
            self.hp / self.max_hp
        } else {
            0.0
        }
    }

    /// Get status text based on HP percentage.
    #[must_use]
    pub fn status(&self) -> &'static str {
        let pct = self.percentage();
        if pct <= 0.0 {
            "DEAD"
        } else if pct <= 0.25 {
            "Critical"
        } else if pct <= 0.5 {
            "Wounded"
        } else if pct <= 0.75 {
            "Injured"
        } else {
            "Healthy"
        }
    }

    /// Check if Titan is alive.
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.hp > 0.0
    }

    /// Check if Titan is critically wounded.
    #[must_use]
    pub fn is_critical(&self) -> bool {
        self.percentage() <= 0.25 && self.is_alive()
    }

    /// Get display color based on HP.
    #[must_use]
    pub fn color(&self) -> [f32; 3] {
        let pct = self.percentage();
        if pct <= 0.25 {
            [0.9, 0.2, 0.2] // Red
        } else if pct <= 0.5 {
            [0.9, 0.6, 0.2] // Orange
        } else if pct <= 0.75 {
            [0.9, 0.9, 0.2] // Yellow
        } else {
            [0.3, 0.8, 0.3] // Green
        }
    }

    /// Get display text.
    #[must_use]
    pub fn display_text(&self) -> String {
        format!("Titan HP: {:.0}/{:.0}", self.hp, self.max_hp)
    }

    /// Get compact display text.
    #[must_use]
    pub fn display_compact(&self) -> String {
        format!("{:.0}%", self.percentage() * 100.0)
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

    /// Get bar fill width for a given max width.
    #[must_use]
    pub fn bar_width(&self, max_width: f32) -> f32 {
        max_width * self.percentage()
    }

    /// Get warning text if HP is low.
    #[must_use]
    pub fn warning_text(&self) -> Option<&'static str> {
        if !self.is_alive() {
            Some("The Titan has perished!")
        } else if self.is_critical() {
            Some("Titan HP critical!")
        } else if self.percentage() <= 0.5 {
            Some("Titan is wounded")
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_titan_hp_display_new() {
        let display = TitanHPDisplay::new();
        assert!((display.hp() - 10000.0).abs() < f32::EPSILON);
        assert!((display.max_hp() - 10000.0).abs() < f32::EPSILON);
        assert!(display.is_visible());
    }

    #[test]
    fn test_titan_hp_display_with_max() {
        let display = TitanHPDisplay::with_max(5000.0);
        assert!((display.hp() - 5000.0).abs() < f32::EPSILON);
        assert!((display.max_hp() - 5000.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_hp_display_update() {
        let mut display = TitanHPDisplay::new();
        display.update(8000.0);
        assert!((display.hp() - 8000.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_hp_display_update_clamp() {
        let mut display = TitanHPDisplay::new();

        display.update(-100.0);
        assert!((display.hp() - 0.0).abs() < f32::EPSILON);

        display.update(20000.0);
        assert!((display.hp() - 10000.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_hp_display_update_full() {
        let mut display = TitanHPDisplay::new();
        display.update_full(500.0, 1000.0);
        assert!((display.hp() - 500.0).abs() < f32::EPSILON);
        assert!((display.max_hp() - 1000.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_hp_display_percentage() {
        let mut display = TitanHPDisplay::new();
        display.update(5000.0);
        assert!((display.percentage() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_hp_display_status_healthy() {
        let display = TitanHPDisplay::new();
        assert_eq!(display.status(), "Healthy");
    }

    #[test]
    fn test_titan_hp_display_status_injured() {
        let mut display = TitanHPDisplay::new();
        display.update(7000.0);
        assert_eq!(display.status(), "Injured");
    }

    #[test]
    fn test_titan_hp_display_status_wounded() {
        let mut display = TitanHPDisplay::new();
        display.update(4000.0);
        assert_eq!(display.status(), "Wounded");
    }

    #[test]
    fn test_titan_hp_display_status_critical() {
        let mut display = TitanHPDisplay::new();
        display.update(2000.0);
        assert_eq!(display.status(), "Critical");
    }

    #[test]
    fn test_titan_hp_display_status_dead() {
        let mut display = TitanHPDisplay::new();
        display.update(0.0);
        assert_eq!(display.status(), "DEAD");
    }

    #[test]
    fn test_titan_hp_display_is_alive() {
        let mut display = TitanHPDisplay::new();
        assert!(display.is_alive());

        display.update(0.0);
        assert!(!display.is_alive());
    }

    #[test]
    fn test_titan_hp_display_is_critical() {
        let mut display = TitanHPDisplay::new();
        assert!(!display.is_critical());

        display.update(2000.0);
        assert!(display.is_critical());

        display.update(0.0);
        assert!(!display.is_critical()); // Dead, not critical
    }

    #[test]
    fn test_titan_hp_display_color_healthy() {
        let display = TitanHPDisplay::new();
        let color = display.color();
        // Green dominant
        assert!(color[1] > color[0]);
    }

    #[test]
    fn test_titan_hp_display_color_critical() {
        let mut display = TitanHPDisplay::new();
        display.update(1000.0);
        let color = display.color();
        // Red dominant
        assert!(color[0] > color[1]);
    }

    #[test]
    fn test_titan_hp_display_display_text() {
        let mut display = TitanHPDisplay::new();
        display.update(8000.0);
        assert!(display.display_text().contains("8000"));
        assert!(display.display_text().contains("10000"));
    }

    #[test]
    fn test_titan_hp_display_display_compact() {
        let mut display = TitanHPDisplay::new();
        display.update(5000.0);
        assert!(display.display_compact().contains("50"));
    }

    #[test]
    fn test_titan_hp_display_visibility() {
        let mut display = TitanHPDisplay::new();
        assert!(display.is_visible());

        display.set_visible(false);
        assert!(!display.is_visible());
    }

    #[test]
    fn test_titan_hp_display_bar_width() {
        let mut display = TitanHPDisplay::new();
        display.update(5000.0);
        assert!((display.bar_width(200.0) - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_hp_display_warning_text_none() {
        let display = TitanHPDisplay::new();
        assert!(display.warning_text().is_none());
    }

    #[test]
    fn test_titan_hp_display_warning_text_wounded() {
        let mut display = TitanHPDisplay::new();
        display.update(4000.0);
        assert!(display.warning_text().is_some());
        assert!(display.warning_text().unwrap().contains("wounded"));
    }

    #[test]
    fn test_titan_hp_display_warning_text_critical() {
        let mut display = TitanHPDisplay::new();
        display.update(2000.0);
        assert!(display.warning_text().is_some());
        assert!(display.warning_text().unwrap().contains("critical"));
    }

    #[test]
    fn test_titan_hp_display_warning_text_dead() {
        let mut display = TitanHPDisplay::new();
        display.update(0.0);
        assert!(display.warning_text().is_some());
        assert!(display.warning_text().unwrap().contains("perished"));
    }

    #[test]
    fn test_titan_hp_display_default() {
        let display = TitanHPDisplay::default();
        assert!((display.hp() - 10000.0).abs() < f32::EPSILON);
    }
}
