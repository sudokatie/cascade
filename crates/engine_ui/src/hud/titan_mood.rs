//! Titan mood HUD display.
//!
//! Shows the current mood state of the Titan with color-coded indicators.

use egui::Color32;

/// Titan mood states for display.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DisplayMood {
    #[default]
    Calm,
    Agitated,
    Enraged,
}

impl DisplayMood {
    /// Create from a mood value (0 = Calm, 1 = Agitated, 2 = Enraged).
    #[must_use]
    pub fn from_value(value: u8) -> Self {
        match value {
            0 => DisplayMood::Calm,
            1 => DisplayMood::Agitated,
            _ => DisplayMood::Enraged,
        }
    }

    /// Convert to a numeric value.
    #[must_use]
    pub fn to_value(self) -> u8 {
        match self {
            DisplayMood::Calm => 0,
            DisplayMood::Agitated => 1,
            DisplayMood::Enraged => 2,
        }
    }
}

/// HUD display for Titan mood.
#[derive(Clone, Debug, Default)]
pub struct TitanMoodDisplay {
    /// Current mood.
    mood: DisplayMood,
    /// Animation pulse (0.0-1.0).
    pulse: f32,
    /// Whether display is visible.
    visible: bool,
}

impl TitanMoodDisplay {
    /// Create a new Titan mood display.
    #[must_use]
    pub fn new() -> Self {
        Self {
            mood: DisplayMood::Calm,
            pulse: 0.0,
            visible: true,
        }
    }

    /// Update the mood display.
    pub fn update(&mut self, mood: DisplayMood) {
        self.mood = mood;
    }

    /// Update from a numeric mood value.
    pub fn update_from_value(&mut self, value: u8) {
        self.mood = DisplayMood::from_value(value);
    }

    /// Tick the animation pulse.
    pub fn tick(&mut self, dt: f32) {
        let pulse_speed = match self.mood {
            DisplayMood::Calm => 0.5,
            DisplayMood::Agitated => 1.5,
            DisplayMood::Enraged => 3.0,
        };
        self.pulse = (self.pulse + dt * pulse_speed) % 1.0;
    }

    /// Get the display color for current mood.
    #[must_use]
    pub fn color(&self) -> [f32; 3] {
        match self.mood {
            DisplayMood::Calm => [0.2, 0.7, 0.3],     // Green
            DisplayMood::Agitated => [0.9, 0.7, 0.1], // Yellow/Orange
            DisplayMood::Enraged => [0.9, 0.2, 0.2],  // Red
        }
    }

    /// Get the egui Color32 for current mood.
    #[must_use]
    pub fn egui_color(&self) -> Color32 {
        let c = self.color();
        Color32::from_rgb(
            (c[0] * 255.0) as u8,
            (c[1] * 255.0) as u8,
            (c[2] * 255.0) as u8,
        )
    }

    /// Get the display label for current mood.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self.mood {
            DisplayMood::Calm => "CALM",
            DisplayMood::Agitated => "AGITATED",
            DisplayMood::Enraged => "ENRAGED",
        }
    }

    /// Get the full display text.
    #[must_use]
    pub fn display_text(&self) -> String {
        format!("Titan Mood: {}", self.label())
    }

    /// Get current mood.
    #[must_use]
    pub fn mood(&self) -> DisplayMood {
        self.mood
    }

    /// Get current pulse value (for animations).
    #[must_use]
    pub fn pulse(&self) -> f32 {
        self.pulse
    }

    /// Check if the Titan is dangerous (Agitated or Enraged).
    #[must_use]
    pub fn is_dangerous(&self) -> bool {
        matches!(self.mood, DisplayMood::Agitated | DisplayMood::Enraged)
    }

    /// Get warning text if dangerous.
    #[must_use]
    pub fn warning_text(&self) -> Option<&'static str> {
        match self.mood {
            DisplayMood::Calm => None,
            DisplayMood::Agitated => Some("Titan is becoming agitated!"),
            DisplayMood::Enraged => Some("DANGER: Titan is ENRAGED!"),
        }
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

    /// Get icon character for mood.
    #[must_use]
    pub fn icon(&self) -> char {
        match self.mood {
            DisplayMood::Calm => '♥',
            DisplayMood::Agitated => '!',
            DisplayMood::Enraged => '⚠',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_titan_mood_display_new() {
        let display = TitanMoodDisplay::new();
        assert_eq!(display.mood(), DisplayMood::Calm);
        assert!(display.is_visible());
    }

    #[test]
    fn test_titan_mood_display_update() {
        let mut display = TitanMoodDisplay::new();
        display.update(DisplayMood::Agitated);
        assert_eq!(display.mood(), DisplayMood::Agitated);
    }

    #[test]
    fn test_titan_mood_display_update_from_value() {
        let mut display = TitanMoodDisplay::new();
        display.update_from_value(2);
        assert_eq!(display.mood(), DisplayMood::Enraged);
    }

    #[test]
    fn test_titan_mood_display_color_calm() {
        let display = TitanMoodDisplay::new();
        let color = display.color();
        // Green - G component should be highest
        assert!(color[1] > color[0]);
        assert!(color[1] > color[2]);
    }

    #[test]
    fn test_titan_mood_display_color_enraged() {
        let mut display = TitanMoodDisplay::new();
        display.update(DisplayMood::Enraged);
        let color = display.color();
        // Red - R component should be highest
        assert!(color[0] > color[1]);
        assert!(color[0] > color[2]);
    }

    #[test]
    fn test_titan_mood_display_egui_color() {
        let display = TitanMoodDisplay::new();
        let _color = display.egui_color();
        // Just ensure it doesn't panic
    }

    #[test]
    fn test_titan_mood_display_label_calm() {
        let display = TitanMoodDisplay::new();
        assert_eq!(display.label(), "CALM");
    }

    #[test]
    fn test_titan_mood_display_label_agitated() {
        let mut display = TitanMoodDisplay::new();
        display.update(DisplayMood::Agitated);
        assert_eq!(display.label(), "AGITATED");
    }

    #[test]
    fn test_titan_mood_display_label_enraged() {
        let mut display = TitanMoodDisplay::new();
        display.update(DisplayMood::Enraged);
        assert_eq!(display.label(), "ENRAGED");
    }

    #[test]
    fn test_titan_mood_display_text() {
        let display = TitanMoodDisplay::new();
        assert!(display.display_text().contains("CALM"));
    }

    #[test]
    fn test_titan_mood_display_is_dangerous() {
        let mut display = TitanMoodDisplay::new();
        assert!(!display.is_dangerous());

        display.update(DisplayMood::Agitated);
        assert!(display.is_dangerous());

        display.update(DisplayMood::Enraged);
        assert!(display.is_dangerous());
    }

    #[test]
    fn test_titan_mood_display_warning_text_calm() {
        let display = TitanMoodDisplay::new();
        assert!(display.warning_text().is_none());
    }

    #[test]
    fn test_titan_mood_display_warning_text_agitated() {
        let mut display = TitanMoodDisplay::new();
        display.update(DisplayMood::Agitated);
        assert!(display.warning_text().is_some());
        assert!(display.warning_text().unwrap().contains("agitated"));
    }

    #[test]
    fn test_titan_mood_display_warning_text_enraged() {
        let mut display = TitanMoodDisplay::new();
        display.update(DisplayMood::Enraged);
        assert!(display.warning_text().is_some());
        assert!(display.warning_text().unwrap().contains("ENRAGED"));
    }

    #[test]
    fn test_titan_mood_display_tick() {
        let mut display = TitanMoodDisplay::new();
        display.tick(0.5);
        assert!(display.pulse() > 0.0);
    }

    #[test]
    fn test_titan_mood_display_tick_wraps() {
        let mut display = TitanMoodDisplay::new();
        for _ in 0..20 {
            display.tick(0.1);
        }
        assert!(display.pulse() >= 0.0 && display.pulse() < 1.0);
    }

    #[test]
    fn test_titan_mood_display_visibility() {
        let mut display = TitanMoodDisplay::new();
        assert!(display.is_visible());

        display.set_visible(false);
        assert!(!display.is_visible());
    }

    #[test]
    fn test_titan_mood_display_icon() {
        let mut display = TitanMoodDisplay::new();
        assert_eq!(display.icon(), '♥');

        display.update(DisplayMood::Enraged);
        assert_eq!(display.icon(), '⚠');
    }

    #[test]
    fn test_display_mood_from_value() {
        assert_eq!(DisplayMood::from_value(0), DisplayMood::Calm);
        assert_eq!(DisplayMood::from_value(1), DisplayMood::Agitated);
        assert_eq!(DisplayMood::from_value(2), DisplayMood::Enraged);
        assert_eq!(DisplayMood::from_value(99), DisplayMood::Enraged);
    }

    #[test]
    fn test_display_mood_to_value() {
        assert_eq!(DisplayMood::Calm.to_value(), 0);
        assert_eq!(DisplayMood::Agitated.to_value(), 1);
        assert_eq!(DisplayMood::Enraged.to_value(), 2);
    }

    #[test]
    fn test_display_mood_default() {
        assert_eq!(DisplayMood::default(), DisplayMood::Calm);
    }

    #[test]
    fn test_titan_mood_display_default() {
        let display = TitanMoodDisplay::default();
        assert_eq!(display.mood(), DisplayMood::Calm);
    }
}
