//! Paradox meter HUD display for time-loop survival.
//!
//! Shows the current paradox level to the player.

use serde::{Deserialize, Serialize};

/// Paradox severity level.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParadoxSeverity {
    /// No paradox.
    #[default]
    None,
    /// Low paradox level.
    Low,
    /// Medium paradox level.
    Medium,
    /// High paradox level.
    High,
    /// Critical paradox level.
    Critical,
}

impl ParadoxSeverity {
    /// Get display name.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            ParadoxSeverity::None => "Stable",
            ParadoxSeverity::Low => "Minor Instability",
            ParadoxSeverity::Medium => "Moderate Instability",
            ParadoxSeverity::High => "Severe Instability",
            ParadoxSeverity::Critical => "CRITICAL",
        }
    }

    /// Get color hint (r, g, b).
    #[must_use]
    pub fn color(&self) -> (f32, f32, f32) {
        match self {
            ParadoxSeverity::None => (0.2, 0.8, 0.2),
            ParadoxSeverity::Low => (0.8, 0.8, 0.2),
            ParadoxSeverity::Medium => (0.9, 0.6, 0.1),
            ParadoxSeverity::High => (0.9, 0.3, 0.1),
            ParadoxSeverity::Critical => (1.0, 0.0, 0.0),
        }
    }
}

/// Paradox meter HUD element.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParadoxMeterDisplay {
    /// Current paradox level (0.0 - 1.0).
    level: f32,
    /// Maximum safe level before hazards spawn.
    safe_threshold: f32,
    /// Current severity.
    severity: ParadoxSeverity,
    /// Visibility flag.
    visible: bool,
    /// Pulse animation phase.
    pulse_phase: f32,
    /// Whether to show numeric value.
    show_numeric: bool,
    /// Rate of change (for trend indicator).
    rate_of_change: f32,
}

impl ParadoxMeterDisplay {
    /// Create a new paradox meter display.
    #[must_use]
    pub fn new() -> Self {
        Self {
            level: 0.0,
            safe_threshold: 0.5,
            severity: ParadoxSeverity::None,
            visible: true,
            pulse_phase: 0.0,
            show_numeric: true,
            rate_of_change: 0.0,
        }
    }

    /// Get current paradox level.
    #[must_use]
    pub fn level(&self) -> f32 {
        self.level
    }

    /// Get the safe threshold.
    #[must_use]
    pub fn safe_threshold(&self) -> f32 {
        self.safe_threshold
    }

    /// Get current severity.
    #[must_use]
    pub fn severity(&self) -> ParadoxSeverity {
        self.severity
    }

    /// Check if visible.
    #[must_use]
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get pulse phase.
    #[must_use]
    pub fn pulse_phase(&self) -> f32 {
        self.pulse_phase
    }

    /// Get rate of change.
    #[must_use]
    pub fn rate_of_change(&self) -> f32 {
        self.rate_of_change
    }

    /// Check if showing numeric value.
    #[must_use]
    pub fn show_numeric(&self) -> bool {
        self.show_numeric
    }

    /// Set the paradox level.
    pub fn set_level(&mut self, level: f32) {
        let old_level = self.level;
        self.level = level.clamp(0.0, 1.0);
        self.rate_of_change = self.level - old_level;
        self.update_severity();
    }

    /// Set the safe threshold.
    pub fn set_safe_threshold(&mut self, threshold: f32) {
        self.safe_threshold = threshold.clamp(0.0, 1.0);
    }

    /// Set visibility.
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Set numeric display.
    pub fn set_show_numeric(&mut self, show: bool) {
        self.show_numeric = show;
    }

    /// Update the severity based on level.
    fn update_severity(&mut self) {
        self.severity = if self.level < 0.1 {
            ParadoxSeverity::None
        } else if self.level < 0.3 {
            ParadoxSeverity::Low
        } else if self.level < 0.5 {
            ParadoxSeverity::Medium
        } else if self.level < 0.8 {
            ParadoxSeverity::High
        } else {
            ParadoxSeverity::Critical
        };
    }

    /// Update the display animation.
    pub fn update(&mut self, delta_time: f32) {
        if self.severity != ParadoxSeverity::None {
            let speed = match self.severity {
                ParadoxSeverity::None => 0.0,
                ParadoxSeverity::Low => 1.0,
                ParadoxSeverity::Medium => 2.0,
                ParadoxSeverity::High => 3.0,
                ParadoxSeverity::Critical => 5.0,
            };
            self.pulse_phase += delta_time * speed;
            if self.pulse_phase > std::f32::consts::TAU {
                self.pulse_phase -= std::f32::consts::TAU;
            }
        }
    }

    /// Get the display text.
    #[must_use]
    pub fn display_text(&self) -> String {
        let percent = (self.level * 100.0) as u32;
        let name = self.severity.display_name();
        if self.show_numeric {
            format!("{}% - {}", percent, name)
        } else {
            name.to_string()
        }
    }

    /// Get pulse intensity for visual effects.
    #[must_use]
    pub fn pulse_intensity(&self) -> f32 {
        if self.severity == ParadoxSeverity::None {
            return 0.0;
        }
        let base = self.pulse_phase.sin() * 0.5 + 0.5;
        base * self.level
    }

    /// Get trend indicator.
    #[must_use]
    pub fn trend(&self) -> &'static str {
        if self.rate_of_change > 0.01 {
            "^"
        } else if self.rate_of_change < -0.01 {
            "v"
        } else {
            "-"
        }
    }

    /// Check if above safe threshold.
    #[must_use]
    pub fn is_dangerous(&self) -> bool {
        self.level >= self.safe_threshold
    }

    /// Query the paradox level (alias).
    #[must_use]
    pub fn query_level(&self) -> f32 {
        self.level
    }

    /// Reset the meter.
    pub fn reset(&mut self) {
        self.level = 0.0;
        self.severity = ParadoxSeverity::None;
        self.pulse_phase = 0.0;
        self.rate_of_change = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paradox_meter_new() {
        let meter = ParadoxMeterDisplay::new();
        assert!((meter.level() - 0.0).abs() < f32::EPSILON);
        assert_eq!(meter.severity(), ParadoxSeverity::None);
    }

    #[test]
    fn test_paradox_severity_none() {
        let mut meter = ParadoxMeterDisplay::new();
        meter.set_level(0.05);
        assert_eq!(meter.severity(), ParadoxSeverity::None);
    }

    #[test]
    fn test_paradox_severity_low() {
        let mut meter = ParadoxMeterDisplay::new();
        meter.set_level(0.2);
        assert_eq!(meter.severity(), ParadoxSeverity::Low);
    }

    #[test]
    fn test_paradox_severity_medium() {
        let mut meter = ParadoxMeterDisplay::new();
        meter.set_level(0.4);
        assert_eq!(meter.severity(), ParadoxSeverity::Medium);
    }

    #[test]
    fn test_paradox_severity_high() {
        let mut meter = ParadoxMeterDisplay::new();
        meter.set_level(0.6);
        assert_eq!(meter.severity(), ParadoxSeverity::High);
    }

    #[test]
    fn test_paradox_severity_critical() {
        let mut meter = ParadoxMeterDisplay::new();
        meter.set_level(0.9);
        assert_eq!(meter.severity(), ParadoxSeverity::Critical);
    }

    #[test]
    fn test_paradox_severity_colors() {
        let (r, g, b) = ParadoxSeverity::None.color();
        assert!(g > r); // Green for safe

        let (r, g, b) = ParadoxSeverity::Critical.color();
        assert!(r > g && r > b); // Red for critical
    }

    #[test]
    fn test_paradox_meter_display_text() {
        let mut meter = ParadoxMeterDisplay::new();
        meter.set_level(0.5);

        assert!(meter.display_text().contains("50%"));
        assert!(meter.display_text().contains("High"));
    }

    #[test]
    fn test_paradox_meter_display_text_no_numeric() {
        let mut meter = ParadoxMeterDisplay::new();
        meter.set_level(0.5);
        meter.set_show_numeric(false);

        assert!(!meter.display_text().contains("%"));
    }

    #[test]
    fn test_paradox_meter_rate_of_change() {
        let mut meter = ParadoxMeterDisplay::new();
        meter.set_level(0.2);
        assert!(meter.rate_of_change() > 0.0);

        meter.set_level(0.1);
        assert!(meter.rate_of_change() < 0.0);
    }

    #[test]
    fn test_paradox_meter_trend() {
        let mut meter = ParadoxMeterDisplay::new();

        meter.set_level(0.2);
        assert_eq!(meter.trend(), "^");

        meter.set_level(0.1);
        assert_eq!(meter.trend(), "v");
    }

    #[test]
    fn test_paradox_meter_is_dangerous() {
        let mut meter = ParadoxMeterDisplay::new();

        meter.set_level(0.3);
        assert!(!meter.is_dangerous());

        meter.set_level(0.6);
        assert!(meter.is_dangerous());
    }

    #[test]
    fn test_paradox_meter_update_pulse() {
        let mut meter = ParadoxMeterDisplay::new();
        meter.set_level(0.5);
        meter.update(0.5);

        assert!(meter.pulse_phase() > 0.0);
    }

    #[test]
    fn test_paradox_meter_pulse_intensity() {
        let mut meter = ParadoxMeterDisplay::new();
        meter.set_level(0.0);
        assert!((meter.pulse_intensity() - 0.0).abs() < f32::EPSILON);

        meter.set_level(0.8);
        meter.update(0.5);
        assert!(meter.pulse_intensity() > 0.0);
    }

    #[test]
    fn test_paradox_meter_visibility() {
        let mut meter = ParadoxMeterDisplay::new();
        meter.set_visible(false);
        assert!(!meter.is_visible());
    }

    #[test]
    fn test_paradox_meter_query() {
        let mut meter = ParadoxMeterDisplay::new();
        meter.set_level(0.42);
        assert!((meter.query_level() - 0.42).abs() < f32::EPSILON);
    }

    #[test]
    fn test_paradox_meter_reset() {
        let mut meter = ParadoxMeterDisplay::new();
        meter.set_level(0.8);
        meter.reset();

        assert!((meter.level() - 0.0).abs() < f32::EPSILON);
        assert_eq!(meter.severity(), ParadoxSeverity::None);
    }
}
