//! Balance meter HUD display.
//!
//! Shows the player's balance/stability on the Titan's surface.

/// HUD display for player balance.
#[derive(Clone, Debug)]
pub struct BalanceMeterDisplay {
    /// Current balance value (0-100).
    balance: f32,
    /// Flash timer for critical warnings.
    flash_timer: f32,
    /// Whether display is visible.
    visible: bool,
}

impl Default for BalanceMeterDisplay {
    fn default() -> Self {
        Self::new()
    }
}

impl BalanceMeterDisplay {
    /// Create a new balance meter display.
    #[must_use]
    pub fn new() -> Self {
        Self {
            balance: 100.0,
            flash_timer: 0.0,
            visible: true,
        }
    }

    /// Update the balance display.
    pub fn update(&mut self, balance: f32) {
        let old_critical = self.is_critical();
        self.balance = balance.clamp(0.0, 100.0);

        // Start flashing if newly critical
        if !old_critical && self.is_critical() {
            self.flash_timer = 1.0;
        }
    }

    /// Tick the flash animation.
    pub fn tick(&mut self, dt: f32) {
        if self.flash_timer > 0.0 {
            self.flash_timer = (self.flash_timer - dt).max(0.0);
        }

        // Keep flashing while critical
        if self.is_critical() && self.flash_timer <= 0.0 {
            self.flash_timer = 0.5;
        }
    }

    /// Get current balance value.
    #[must_use]
    pub fn balance(&self) -> f32 {
        self.balance
    }

    /// Get balance as percentage (0.0-1.0).
    #[must_use]
    pub fn percentage(&self) -> f32 {
        self.balance / 100.0
    }

    /// Check if balance is critical (falling imminent).
    #[must_use]
    pub fn is_critical(&self) -> bool {
        self.balance <= 20.0
    }

    /// Check if balance is low.
    #[must_use]
    pub fn is_low(&self) -> bool {
        self.balance <= 40.0
    }

    /// Check if player is falling (balance depleted).
    #[must_use]
    pub fn is_falling(&self) -> bool {
        self.balance <= 0.0
    }

    /// Get warning text if applicable.
    #[must_use]
    pub fn warning_text(&self) -> Option<&'static str> {
        if self.is_falling() {
            Some("FALLING!")
        } else if self.is_critical() {
            Some("Balance critical!")
        } else if self.is_low() {
            Some("Low balance")
        } else {
            None
        }
    }

    /// Get display color based on balance level.
    #[must_use]
    pub fn color(&self) -> [f32; 3] {
        if self.balance <= 20.0 {
            [0.9, 0.2, 0.2] // Red - critical
        } else if self.balance <= 40.0 {
            [0.9, 0.6, 0.2] // Orange - low
        } else if self.balance <= 60.0 {
            [0.9, 0.9, 0.2] // Yellow - caution
        } else {
            [0.3, 0.8, 0.3] // Green - good
        }
    }

    /// Get display text.
    #[must_use]
    pub fn display_text(&self) -> String {
        format!("Balance: {:.0}%", self.balance)
    }

    /// Get status label.
    #[must_use]
    pub fn status(&self) -> &'static str {
        if self.is_falling() {
            "FALLING"
        } else if self.is_critical() {
            "CRITICAL"
        } else if self.is_low() {
            "UNSTABLE"
        } else if self.balance < 80.0 {
            "SHAKY"
        } else {
            "STABLE"
        }
    }

    /// Check if currently flashing.
    #[must_use]
    pub fn is_flashing(&self) -> bool {
        self.is_critical() && (self.flash_timer * 4.0) as u32 % 2 == 0
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balance_meter_display_new() {
        let display = BalanceMeterDisplay::new();
        assert!((display.balance() - 100.0).abs() < f32::EPSILON);
        assert!(display.is_visible());
    }

    #[test]
    fn test_balance_meter_display_update() {
        let mut display = BalanceMeterDisplay::new();
        display.update(75.0);
        assert!((display.balance() - 75.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_meter_display_update_clamp() {
        let mut display = BalanceMeterDisplay::new();

        display.update(-10.0);
        assert!((display.balance() - 0.0).abs() < f32::EPSILON);

        display.update(150.0);
        assert!((display.balance() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_meter_display_percentage() {
        let mut display = BalanceMeterDisplay::new();
        display.update(50.0);
        assert!((display.percentage() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_meter_display_is_critical() {
        let mut display = BalanceMeterDisplay::new();

        display.update(50.0);
        assert!(!display.is_critical());

        display.update(20.0);
        assert!(display.is_critical());

        display.update(10.0);
        assert!(display.is_critical());
    }

    #[test]
    fn test_balance_meter_display_is_low() {
        let mut display = BalanceMeterDisplay::new();

        display.update(60.0);
        assert!(!display.is_low());

        display.update(40.0);
        assert!(display.is_low());
    }

    #[test]
    fn test_balance_meter_display_is_falling() {
        let mut display = BalanceMeterDisplay::new();

        display.update(1.0);
        assert!(!display.is_falling());

        display.update(0.0);
        assert!(display.is_falling());
    }

    #[test]
    fn test_balance_meter_display_warning_text_none() {
        let mut display = BalanceMeterDisplay::new();
        display.update(80.0);
        assert!(display.warning_text().is_none());
    }

    #[test]
    fn test_balance_meter_display_warning_text_low() {
        let mut display = BalanceMeterDisplay::new();
        display.update(35.0);
        assert!(display.warning_text().is_some());
        assert!(display.warning_text().unwrap().contains("Low"));
    }

    #[test]
    fn test_balance_meter_display_warning_text_critical() {
        let mut display = BalanceMeterDisplay::new();
        display.update(15.0);
        assert!(display.warning_text().is_some());
        assert!(display.warning_text().unwrap().contains("critical"));
    }

    #[test]
    fn test_balance_meter_display_warning_text_falling() {
        let mut display = BalanceMeterDisplay::new();
        display.update(0.0);
        assert!(display.warning_text().is_some());
        assert!(display.warning_text().unwrap().contains("FALLING"));
    }

    #[test]
    fn test_balance_meter_display_color_good() {
        let mut display = BalanceMeterDisplay::new();
        display.update(80.0);
        let color = display.color();
        // Green dominant
        assert!(color[1] > color[0]);
    }

    #[test]
    fn test_balance_meter_display_color_critical() {
        let mut display = BalanceMeterDisplay::new();
        display.update(10.0);
        let color = display.color();
        // Red dominant
        assert!(color[0] > color[1]);
    }

    #[test]
    fn test_balance_meter_display_display_text() {
        let mut display = BalanceMeterDisplay::new();
        display.update(75.0);
        assert!(display.display_text().contains("75"));
    }

    #[test]
    fn test_balance_meter_display_status() {
        let mut display = BalanceMeterDisplay::new();

        display.update(90.0);
        assert_eq!(display.status(), "STABLE");

        display.update(70.0);
        assert_eq!(display.status(), "SHAKY");

        display.update(30.0);
        assert_eq!(display.status(), "UNSTABLE");

        display.update(10.0);
        assert_eq!(display.status(), "CRITICAL");

        display.update(0.0);
        assert_eq!(display.status(), "FALLING");
    }

    #[test]
    fn test_balance_meter_display_tick() {
        let mut display = BalanceMeterDisplay::new();
        display.update(10.0); // Critical
        display.tick(0.1);
        // Flash timer should be active
        assert!(display.flash_timer > 0.0 || display.is_flashing());
    }

    #[test]
    fn test_balance_meter_display_visibility() {
        let mut display = BalanceMeterDisplay::new();
        assert!(display.is_visible());

        display.set_visible(false);
        assert!(!display.is_visible());
    }

    #[test]
    fn test_balance_meter_display_bar_width() {
        let mut display = BalanceMeterDisplay::new();
        display.update(50.0);
        assert!((display.bar_width(200.0) - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_balance_meter_display_default() {
        let display = BalanceMeterDisplay::default();
        assert!((display.balance() - 100.0).abs() < f32::EPSILON);
    }
}
