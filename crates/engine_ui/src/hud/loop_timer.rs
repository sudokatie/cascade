//! Loop timer HUD display for time-loop survival.
//!
//! Shows the remaining time in the current loop.

use serde::{Deserialize, Serialize};

/// Display format for the timer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimerFormat {
    /// Digital clock format (MM:SS).
    #[default]
    Digital,
    /// Progress bar.
    Bar,
    /// Circular countdown.
    Circular,
    /// Percentage remaining.
    Percentage,
}

/// Urgency level for visual feedback.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimerUrgency {
    /// Plenty of time remaining.
    #[default]
    Calm,
    /// Time is running low.
    Warning,
    /// Very little time left.
    Critical,
}

/// Loop timer HUD element.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LoopTimerDisplay {
    /// Total loop duration in seconds.
    total_duration: f32,
    /// Time remaining in seconds.
    time_remaining: f32,
    /// Display format.
    format: TimerFormat,
    /// Current urgency level.
    urgency: TimerUrgency,
    /// Warning threshold (percentage).
    warning_threshold: f32,
    /// Critical threshold (percentage).
    critical_threshold: f32,
    /// Visibility flag.
    visible: bool,
    /// Pulse animation phase.
    pulse_phase: f32,
}

impl LoopTimerDisplay {
    /// Create a new loop timer display.
    #[must_use]
    pub fn new(total_duration: f32) -> Self {
        Self {
            total_duration,
            time_remaining: total_duration,
            format: TimerFormat::Digital,
            urgency: TimerUrgency::Calm,
            warning_threshold: 0.3,
            critical_threshold: 0.1,
            visible: true,
            pulse_phase: 0.0,
        }
    }

    /// Get total loop duration.
    #[must_use]
    pub fn total_duration(&self) -> f32 {
        self.total_duration
    }

    /// Get time remaining.
    #[must_use]
    pub fn time_remaining(&self) -> f32 {
        self.time_remaining
    }

    /// Get time remaining as a percentage (0.0 - 1.0).
    #[must_use]
    pub fn remaining_percentage(&self) -> f32 {
        if self.total_duration <= 0.0 {
            return 0.0;
        }
        (self.time_remaining / self.total_duration).clamp(0.0, 1.0)
    }

    /// Get the display format.
    #[must_use]
    pub fn format(&self) -> TimerFormat {
        self.format
    }

    /// Get the current urgency level.
    #[must_use]
    pub fn urgency(&self) -> TimerUrgency {
        self.urgency
    }

    /// Check if visible.
    #[must_use]
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get pulse phase for animation.
    #[must_use]
    pub fn pulse_phase(&self) -> f32 {
        self.pulse_phase
    }

    /// Set the total loop duration.
    pub fn set_duration(&mut self, duration: f32) {
        self.total_duration = duration.max(0.0);
        self.time_remaining = self.time_remaining.min(self.total_duration);
    }

    /// Set time remaining.
    pub fn set_time_remaining(&mut self, time: f32) {
        self.time_remaining = time.clamp(0.0, self.total_duration);
        self.update_urgency();
    }

    /// Set display format.
    pub fn set_format(&mut self, format: TimerFormat) {
        self.format = format;
    }

    /// Set visibility.
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Update the timer display.
    pub fn update(&mut self, delta_time: f32) {
        // Update pulse animation for urgent states
        if self.urgency != TimerUrgency::Calm {
            let speed = match self.urgency {
                TimerUrgency::Warning => 2.0,
                TimerUrgency::Critical => 4.0,
                TimerUrgency::Calm => 0.0,
            };
            self.pulse_phase += delta_time * speed;
            if self.pulse_phase > std::f32::consts::TAU {
                self.pulse_phase -= std::f32::consts::TAU;
            }
        }
    }

    /// Update the urgency level based on time remaining.
    fn update_urgency(&mut self) {
        let percentage = self.remaining_percentage();
        self.urgency = if percentage <= self.critical_threshold {
            TimerUrgency::Critical
        } else if percentage <= self.warning_threshold {
            TimerUrgency::Warning
        } else {
            TimerUrgency::Calm
        };
    }

    /// Get the display text.
    #[must_use]
    pub fn display_text(&self) -> String {
        match self.format {
            TimerFormat::Digital => {
                let minutes = (self.time_remaining / 60.0) as u32;
                let seconds = (self.time_remaining % 60.0) as u32;
                format!("{:02}:{:02}", minutes, seconds)
            }
            TimerFormat::Bar => {
                let filled = (self.remaining_percentage() * 10.0) as usize;
                let empty = 10 - filled;
                format!("[{}{}]", "=".repeat(filled), " ".repeat(empty))
            }
            TimerFormat::Circular => {
                let degrees = (self.remaining_percentage() * 360.0) as u32;
                format!("{}°", degrees)
            }
            TimerFormat::Percentage => {
                format!("{}%", (self.remaining_percentage() * 100.0) as u32)
            }
        }
    }

    /// Get pulse intensity for visual effects.
    #[must_use]
    pub fn pulse_intensity(&self) -> f32 {
        if self.urgency == TimerUrgency::Calm {
            return 0.0;
        }
        (self.pulse_phase.sin() * 0.5 + 0.5)
            * match self.urgency {
                TimerUrgency::Warning => 0.5,
                TimerUrgency::Critical => 1.0,
                TimerUrgency::Calm => 0.0,
            }
    }

    /// Query time remaining (alias).
    #[must_use]
    pub fn query_time(&self) -> f32 {
        self.time_remaining
    }

    /// Reset the timer.
    pub fn reset(&mut self) {
        self.time_remaining = self.total_duration;
        self.urgency = TimerUrgency::Calm;
        self.pulse_phase = 0.0;
    }

    /// Check if time has expired.
    #[must_use]
    pub fn is_expired(&self) -> bool {
        self.time_remaining <= 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_timer_new() {
        let timer = LoopTimerDisplay::new(300.0);
        assert!((timer.total_duration() - 300.0).abs() < f32::EPSILON);
        assert!((timer.time_remaining() - 300.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_loop_timer_remaining_percentage() {
        let mut timer = LoopTimerDisplay::new(100.0);
        timer.set_time_remaining(50.0);

        assert!((timer.remaining_percentage() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_loop_timer_urgency_calm() {
        let mut timer = LoopTimerDisplay::new(100.0);
        timer.set_time_remaining(80.0);

        assert_eq!(timer.urgency(), TimerUrgency::Calm);
    }

    #[test]
    fn test_loop_timer_urgency_warning() {
        let mut timer = LoopTimerDisplay::new(100.0);
        timer.set_time_remaining(20.0);

        assert_eq!(timer.urgency(), TimerUrgency::Warning);
    }

    #[test]
    fn test_loop_timer_urgency_critical() {
        let mut timer = LoopTimerDisplay::new(100.0);
        timer.set_time_remaining(5.0);

        assert_eq!(timer.urgency(), TimerUrgency::Critical);
    }

    #[test]
    fn test_loop_timer_display_digital() {
        let mut timer = LoopTimerDisplay::new(300.0);
        timer.set_time_remaining(125.0);

        assert_eq!(timer.display_text(), "02:05");
    }

    #[test]
    fn test_loop_timer_display_percentage() {
        let mut timer = LoopTimerDisplay::new(100.0);
        timer.set_format(TimerFormat::Percentage);
        timer.set_time_remaining(50.0);

        assert_eq!(timer.display_text(), "50%");
    }

    #[test]
    fn test_loop_timer_display_bar() {
        let mut timer = LoopTimerDisplay::new(100.0);
        timer.set_format(TimerFormat::Bar);
        timer.set_time_remaining(50.0);

        assert!(timer.display_text().contains("="));
    }

    #[test]
    fn test_loop_timer_display_circular() {
        let mut timer = LoopTimerDisplay::new(100.0);
        timer.set_format(TimerFormat::Circular);
        timer.set_time_remaining(50.0);

        assert!(timer.display_text().contains("180"));
    }

    #[test]
    fn test_loop_timer_update_pulse() {
        let mut timer = LoopTimerDisplay::new(100.0);
        timer.set_time_remaining(5.0); // Critical

        timer.update(0.5);
        assert!(timer.pulse_phase() > 0.0);
    }

    #[test]
    fn test_loop_timer_pulse_intensity() {
        let mut timer = LoopTimerDisplay::new(100.0);
        timer.set_time_remaining(50.0);
        assert!((timer.pulse_intensity() - 0.0).abs() < f32::EPSILON);

        timer.set_time_remaining(5.0);
        timer.update(0.5);
        assert!(timer.pulse_intensity() > 0.0);
    }

    #[test]
    fn test_loop_timer_visibility() {
        let mut timer = LoopTimerDisplay::new(100.0);
        timer.set_visible(false);
        assert!(!timer.is_visible());
    }

    #[test]
    fn test_loop_timer_query() {
        let mut timer = LoopTimerDisplay::new(100.0);
        timer.set_time_remaining(42.0);
        assert!((timer.query_time() - 42.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_loop_timer_reset() {
        let mut timer = LoopTimerDisplay::new(100.0);
        timer.set_time_remaining(10.0);
        timer.reset();

        assert!((timer.time_remaining() - 100.0).abs() < f32::EPSILON);
        assert_eq!(timer.urgency(), TimerUrgency::Calm);
    }

    #[test]
    fn test_loop_timer_is_expired() {
        let mut timer = LoopTimerDisplay::new(100.0);
        assert!(!timer.is_expired());

        timer.set_time_remaining(0.0);
        assert!(timer.is_expired());
    }
}
