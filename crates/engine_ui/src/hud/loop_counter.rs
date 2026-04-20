//! Loop counter HUD display for time-loop survival.
//!
//! Shows the current loop number to the player.

use serde::{Deserialize, Serialize};

/// Display style for the loop counter.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoopCounterStyle {
    /// Simple numeric display.
    #[default]
    Numeric,
    /// Roman numerals.
    Roman,
    /// Circular dial.
    Dial,
    /// Tally marks.
    Tally,
}

/// Loop counter HUD element.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LoopCounterDisplay {
    /// Current loop number.
    current_loop: u32,
    /// Maximum recorded loop.
    max_loop: u32,
    /// Display style.
    style: LoopCounterStyle,
    /// Whether to show max loop.
    show_max: bool,
    /// Animation progress for loop change.
    animation_progress: f32,
    /// Whether currently animating.
    animating: bool,
    /// Visibility flag.
    visible: bool,
}

impl LoopCounterDisplay {
    /// Create a new loop counter display.
    #[must_use]
    pub fn new() -> Self {
        Self {
            current_loop: 1,
            max_loop: 1,
            style: LoopCounterStyle::Numeric,
            show_max: true,
            animation_progress: 0.0,
            animating: false,
            visible: true,
        }
    }

    /// Get the current loop number.
    #[must_use]
    pub fn current_loop(&self) -> u32 {
        self.current_loop
    }

    /// Get the maximum recorded loop.
    #[must_use]
    pub fn max_loop(&self) -> u32 {
        self.max_loop
    }

    /// Get the display style.
    #[must_use]
    pub fn style(&self) -> LoopCounterStyle {
        self.style
    }

    /// Check if showing max loop.
    #[must_use]
    pub fn show_max(&self) -> bool {
        self.show_max
    }

    /// Check if visible.
    #[must_use]
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Check if animating.
    #[must_use]
    pub fn is_animating(&self) -> bool {
        self.animating
    }

    /// Get animation progress (0.0 - 1.0).
    #[must_use]
    pub fn animation_progress(&self) -> f32 {
        self.animation_progress
    }

    /// Set the current loop number.
    pub fn set_loop(&mut self, loop_num: u32) {
        if loop_num != self.current_loop {
            self.current_loop = loop_num;
            self.max_loop = self.max_loop.max(loop_num);
            self.animating = true;
            self.animation_progress = 0.0;
        }
    }

    /// Set display style.
    pub fn set_style(&mut self, style: LoopCounterStyle) {
        self.style = style;
    }

    /// Set whether to show max loop.
    pub fn set_show_max(&mut self, show: bool) {
        self.show_max = show;
    }

    /// Set visibility.
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Update the display animation.
    pub fn update(&mut self, delta_time: f32) {
        if self.animating {
            self.animation_progress += delta_time * 2.0;
            if self.animation_progress >= 1.0 {
                self.animation_progress = 1.0;
                self.animating = false;
            }
        }
    }

    /// Get the display text.
    #[must_use]
    pub fn display_text(&self) -> String {
        let current = self.format_number(self.current_loop);
        if self.show_max && self.max_loop > 1 {
            let max = self.format_number(self.max_loop);
            format!("{} / {}", current, max)
        } else {
            current
        }
    }

    /// Format a number according to the current style.
    fn format_number(&self, num: u32) -> String {
        match self.style {
            LoopCounterStyle::Numeric => num.to_string(),
            LoopCounterStyle::Roman => self.to_roman(num),
            LoopCounterStyle::Dial => format!("[{}]", num),
            LoopCounterStyle::Tally => self.to_tally(num),
        }
    }

    /// Convert to roman numerals.
    fn to_roman(&self, mut num: u32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let numerals = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        let mut result = String::new();
        for (value, symbol) in numerals {
            while num >= value {
                result.push_str(symbol);
                num -= value;
            }
        }
        result
    }

    /// Convert to tally marks.
    fn to_tally(&self, num: u32) -> String {
        let groups = num / 5;
        let remainder = num % 5;
        let mut result = String::new();
        for _ in 0..groups {
            result.push_str("IIII ");
        }
        for _ in 0..remainder {
            result.push('I');
        }
        result.trim().to_string()
    }

    /// Query current loop (alias for current_loop).
    #[must_use]
    pub fn query_loop(&self) -> u32 {
        self.current_loop
    }

    /// Reset the counter.
    pub fn reset(&mut self) {
        self.current_loop = 1;
        self.max_loop = 1;
        self.animating = false;
        self.animation_progress = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_counter_new() {
        let counter = LoopCounterDisplay::new();
        assert_eq!(counter.current_loop(), 1);
        assert_eq!(counter.max_loop(), 1);
        assert!(counter.is_visible());
    }

    #[test]
    fn test_loop_counter_set_loop() {
        let mut counter = LoopCounterDisplay::new();
        counter.set_loop(5);

        assert_eq!(counter.current_loop(), 5);
        assert_eq!(counter.max_loop(), 5);
        assert!(counter.is_animating());
    }

    #[test]
    fn test_loop_counter_max_tracking() {
        let mut counter = LoopCounterDisplay::new();
        counter.set_loop(10);
        counter.set_loop(3);

        assert_eq!(counter.current_loop(), 3);
        assert_eq!(counter.max_loop(), 10);
    }

    #[test]
    fn test_loop_counter_update_animation() {
        let mut counter = LoopCounterDisplay::new();
        counter.set_loop(2);
        assert!(counter.is_animating());

        counter.update(0.6);
        assert!(!counter.is_animating());
        assert!((counter.animation_progress() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_loop_counter_display_text_numeric() {
        let mut counter = LoopCounterDisplay::new();
        counter.set_loop(5);

        assert_eq!(counter.display_text(), "5 / 5");
    }

    #[test]
    fn test_loop_counter_display_text_no_max() {
        let mut counter = LoopCounterDisplay::new();
        counter.set_loop(5);
        counter.set_show_max(false);

        assert_eq!(counter.display_text(), "5");
    }

    #[test]
    fn test_loop_counter_roman_numerals() {
        let mut counter = LoopCounterDisplay::new();
        counter.set_style(LoopCounterStyle::Roman);
        counter.set_show_max(false);

        counter.set_loop(1);
        assert_eq!(counter.display_text(), "I");

        counter.set_loop(4);
        assert_eq!(counter.display_text(), "IV");

        counter.set_loop(9);
        assert_eq!(counter.display_text(), "IX");

        counter.set_loop(14);
        assert_eq!(counter.display_text(), "XIV");
    }

    #[test]
    fn test_loop_counter_tally() {
        let mut counter = LoopCounterDisplay::new();
        counter.set_style(LoopCounterStyle::Tally);
        counter.set_show_max(false);

        counter.set_loop(3);
        assert_eq!(counter.display_text(), "III");

        counter.set_loop(7);
        assert_eq!(counter.display_text(), "IIII II");
    }

    #[test]
    fn test_loop_counter_dial() {
        let mut counter = LoopCounterDisplay::new();
        counter.set_style(LoopCounterStyle::Dial);
        counter.set_show_max(false);
        counter.set_loop(5);

        assert_eq!(counter.display_text(), "[5]");
    }

    #[test]
    fn test_loop_counter_visibility() {
        let mut counter = LoopCounterDisplay::new();
        counter.set_visible(false);
        assert!(!counter.is_visible());
    }

    #[test]
    fn test_loop_counter_query() {
        let mut counter = LoopCounterDisplay::new();
        counter.set_loop(7);
        assert_eq!(counter.query_loop(), 7);
    }

    #[test]
    fn test_loop_counter_reset() {
        let mut counter = LoopCounterDisplay::new();
        counter.set_loop(10);
        counter.reset();

        assert_eq!(counter.current_loop(), 1);
        assert_eq!(counter.max_loop(), 1);
    }
}
