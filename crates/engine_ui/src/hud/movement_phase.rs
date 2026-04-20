//! Movement phase HUD display.
//!
//! Shows the current movement phase of the Titan and remaining time.

/// Display phase states.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DisplayPhase {
    #[default]
    Resting,
    Walking,
    Running,
    Scratching,
}

impl DisplayPhase {
    /// Create from a phase value (0-3).
    #[must_use]
    pub fn from_value(value: u8) -> Self {
        match value {
            0 => DisplayPhase::Resting,
            1 => DisplayPhase::Walking,
            2 => DisplayPhase::Running,
            _ => DisplayPhase::Scratching,
        }
    }

    /// Convert to a numeric value.
    #[must_use]
    pub fn to_value(self) -> u8 {
        match self {
            DisplayPhase::Resting => 0,
            DisplayPhase::Walking => 1,
            DisplayPhase::Running => 2,
            DisplayPhase::Scratching => 3,
        }
    }
}

/// HUD display for movement phase.
#[derive(Clone, Debug, Default)]
pub struct MovementPhaseDisplay {
    /// Current phase.
    phase: DisplayPhase,
    /// Timer remaining in current phase (seconds).
    timer: f32,
    /// Whether display is visible.
    visible: bool,
}

impl MovementPhaseDisplay {
    /// Create a new movement phase display.
    #[must_use]
    pub fn new() -> Self {
        Self {
            phase: DisplayPhase::Resting,
            timer: 0.0,
            visible: true,
        }
    }

    /// Update the phase display.
    pub fn update(&mut self, phase: DisplayPhase, timer: f32) {
        self.phase = phase;
        self.timer = timer.max(0.0);
    }

    /// Update from numeric values.
    pub fn update_from_values(&mut self, phase_value: u8, timer: f32) {
        self.phase = DisplayPhase::from_value(phase_value);
        self.timer = timer.max(0.0);
    }

    /// Get the phase name.
    #[must_use]
    pub fn phase_name(&self) -> &'static str {
        match self.phase {
            DisplayPhase::Resting => "Resting",
            DisplayPhase::Walking => "Walking",
            DisplayPhase::Running => "Running",
            DisplayPhase::Scratching => "Scratching",
        }
    }

    /// Get short phase name (for compact display).
    #[must_use]
    pub fn phase_short(&self) -> &'static str {
        match self.phase {
            DisplayPhase::Resting => "REST",
            DisplayPhase::Walking => "WALK",
            DisplayPhase::Running => "RUN",
            DisplayPhase::Scratching => "SCRATCH",
        }
    }

    /// Get current phase.
    #[must_use]
    pub fn phase(&self) -> DisplayPhase {
        self.phase
    }

    /// Get remaining timer.
    #[must_use]
    pub fn timer(&self) -> f32 {
        self.timer
    }

    /// Get timer as formatted string (MM:SS).
    #[must_use]
    pub fn timer_text(&self) -> String {
        let minutes = (self.timer / 60.0).floor() as u32;
        let seconds = (self.timer % 60.0).floor() as u32;
        format!("{:02}:{:02}", minutes, seconds)
    }

    /// Get full display text.
    #[must_use]
    pub fn display_text(&self) -> String {
        format!("Phase: {} ({})", self.phase_name(), self.timer_text())
    }

    /// Get display color for current phase.
    #[must_use]
    pub fn color(&self) -> [f32; 3] {
        match self.phase {
            DisplayPhase::Resting => [0.3, 0.6, 0.3],     // Green - safe
            DisplayPhase::Walking => [0.6, 0.6, 0.3],     // Yellow - caution
            DisplayPhase::Running => [0.8, 0.3, 0.3],     // Red - danger
            DisplayPhase::Scratching => [0.8, 0.5, 0.2],  // Orange - high danger
        }
    }

    /// Check if in an active (non-resting) phase.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.phase != DisplayPhase::Resting
    }

    /// Check if in a dangerous phase.
    #[must_use]
    pub fn is_dangerous(&self) -> bool {
        matches!(self.phase, DisplayPhase::Running | DisplayPhase::Scratching)
    }

    /// Get danger level (0.0 = safe, 1.0 = maximum danger).
    #[must_use]
    pub fn danger_level(&self) -> f32 {
        match self.phase {
            DisplayPhase::Resting => 0.0,
            DisplayPhase::Walking => 0.3,
            DisplayPhase::Running => 0.8,
            DisplayPhase::Scratching => 1.0,
        }
    }

    /// Get stability modifier for current phase.
    #[must_use]
    pub fn stability_modifier(&self) -> f32 {
        match self.phase {
            DisplayPhase::Resting => 1.0,
            DisplayPhase::Walking => 0.8,
            DisplayPhase::Running => 0.4,
            DisplayPhase::Scratching => 0.3,
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

    /// Get icon character for phase.
    #[must_use]
    pub fn icon(&self) -> char {
        match self.phase {
            DisplayPhase::Resting => '○',
            DisplayPhase::Walking => '→',
            DisplayPhase::Running => '»',
            DisplayPhase::Scratching => '~',
        }
    }

    /// Get warning if in dangerous phase.
    #[must_use]
    pub fn warning(&self) -> Option<&'static str> {
        match self.phase {
            DisplayPhase::Resting | DisplayPhase::Walking => None,
            DisplayPhase::Running => Some("Hold on tight!"),
            DisplayPhase::Scratching => Some("Localized tremors!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movement_phase_display_new() {
        let display = MovementPhaseDisplay::new();
        assert_eq!(display.phase(), DisplayPhase::Resting);
        assert!((display.timer() - 0.0).abs() < f32::EPSILON);
        assert!(display.is_visible());
    }

    #[test]
    fn test_movement_phase_display_update() {
        let mut display = MovementPhaseDisplay::new();
        display.update(DisplayPhase::Running, 30.0);
        assert_eq!(display.phase(), DisplayPhase::Running);
        assert!((display.timer() - 30.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_movement_phase_display_update_from_values() {
        let mut display = MovementPhaseDisplay::new();
        display.update_from_values(2, 45.0);
        assert_eq!(display.phase(), DisplayPhase::Running);
        assert!((display.timer() - 45.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_movement_phase_display_phase_name() {
        let mut display = MovementPhaseDisplay::new();

        display.update(DisplayPhase::Resting, 0.0);
        assert_eq!(display.phase_name(), "Resting");

        display.update(DisplayPhase::Walking, 0.0);
        assert_eq!(display.phase_name(), "Walking");

        display.update(DisplayPhase::Running, 0.0);
        assert_eq!(display.phase_name(), "Running");

        display.update(DisplayPhase::Scratching, 0.0);
        assert_eq!(display.phase_name(), "Scratching");
    }

    #[test]
    fn test_movement_phase_display_phase_short() {
        let mut display = MovementPhaseDisplay::new();

        display.update(DisplayPhase::Resting, 0.0);
        assert_eq!(display.phase_short(), "REST");

        display.update(DisplayPhase::Running, 0.0);
        assert_eq!(display.phase_short(), "RUN");
    }

    #[test]
    fn test_movement_phase_display_timer_text() {
        let mut display = MovementPhaseDisplay::new();

        display.update(DisplayPhase::Resting, 90.0);
        assert_eq!(display.timer_text(), "01:30");

        display.update(DisplayPhase::Resting, 5.0);
        assert_eq!(display.timer_text(), "00:05");
    }

    #[test]
    fn test_movement_phase_display_display_text() {
        let mut display = MovementPhaseDisplay::new();
        display.update(DisplayPhase::Walking, 60.0);
        assert!(display.display_text().contains("Walking"));
        assert!(display.display_text().contains("01:00"));
    }

    #[test]
    fn test_movement_phase_display_color() {
        let mut display = MovementPhaseDisplay::new();

        display.update(DisplayPhase::Resting, 0.0);
        let color = display.color();
        assert!(color[1] > color[0]); // Green dominant

        display.update(DisplayPhase::Running, 0.0);
        let color = display.color();
        assert!(color[0] > color[1]); // Red dominant
    }

    #[test]
    fn test_movement_phase_display_is_active() {
        let mut display = MovementPhaseDisplay::new();

        display.update(DisplayPhase::Resting, 0.0);
        assert!(!display.is_active());

        display.update(DisplayPhase::Walking, 0.0);
        assert!(display.is_active());
    }

    #[test]
    fn test_movement_phase_display_is_dangerous() {
        let mut display = MovementPhaseDisplay::new();

        display.update(DisplayPhase::Resting, 0.0);
        assert!(!display.is_dangerous());

        display.update(DisplayPhase::Walking, 0.0);
        assert!(!display.is_dangerous());

        display.update(DisplayPhase::Running, 0.0);
        assert!(display.is_dangerous());

        display.update(DisplayPhase::Scratching, 0.0);
        assert!(display.is_dangerous());
    }

    #[test]
    fn test_movement_phase_display_danger_level() {
        let mut display = MovementPhaseDisplay::new();

        display.update(DisplayPhase::Resting, 0.0);
        assert!((display.danger_level() - 0.0).abs() < f32::EPSILON);

        display.update(DisplayPhase::Scratching, 0.0);
        assert!((display.danger_level() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_movement_phase_display_stability_modifier() {
        let mut display = MovementPhaseDisplay::new();

        display.update(DisplayPhase::Resting, 0.0);
        assert!((display.stability_modifier() - 1.0).abs() < f32::EPSILON);

        display.update(DisplayPhase::Running, 0.0);
        assert!((display.stability_modifier() - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn test_movement_phase_display_visibility() {
        let mut display = MovementPhaseDisplay::new();
        assert!(display.is_visible());

        display.set_visible(false);
        assert!(!display.is_visible());
    }

    #[test]
    fn test_movement_phase_display_icon() {
        let mut display = MovementPhaseDisplay::new();

        display.update(DisplayPhase::Resting, 0.0);
        assert_eq!(display.icon(), '○');

        display.update(DisplayPhase::Running, 0.0);
        assert_eq!(display.icon(), '»');
    }

    #[test]
    fn test_movement_phase_display_warning() {
        let mut display = MovementPhaseDisplay::new();

        display.update(DisplayPhase::Resting, 0.0);
        assert!(display.warning().is_none());

        display.update(DisplayPhase::Running, 0.0);
        assert!(display.warning().is_some());
        assert!(display.warning().unwrap().contains("Hold"));
    }

    #[test]
    fn test_display_phase_from_value() {
        assert_eq!(DisplayPhase::from_value(0), DisplayPhase::Resting);
        assert_eq!(DisplayPhase::from_value(1), DisplayPhase::Walking);
        assert_eq!(DisplayPhase::from_value(2), DisplayPhase::Running);
        assert_eq!(DisplayPhase::from_value(3), DisplayPhase::Scratching);
        assert_eq!(DisplayPhase::from_value(99), DisplayPhase::Scratching);
    }

    #[test]
    fn test_display_phase_to_value() {
        assert_eq!(DisplayPhase::Resting.to_value(), 0);
        assert_eq!(DisplayPhase::Walking.to_value(), 1);
        assert_eq!(DisplayPhase::Running.to_value(), 2);
        assert_eq!(DisplayPhase::Scratching.to_value(), 3);
    }

    #[test]
    fn test_display_phase_default() {
        assert_eq!(DisplayPhase::default(), DisplayPhase::Resting);
    }

    #[test]
    fn test_movement_phase_display_default() {
        let display = MovementPhaseDisplay::default();
        assert_eq!(display.phase(), DisplayPhase::Resting);
    }

    #[test]
    fn test_movement_phase_display_timer_clamp() {
        let mut display = MovementPhaseDisplay::new();
        display.update(DisplayPhase::Resting, -10.0);
        assert!((display.timer() - 0.0).abs() < f32::EPSILON);
    }
}
