//! Warning signs for cross-loop communication.
//!
//! Players can place signs with messages that persist across
//! time loops for a limited number of iterations.

use glam::IVec3;
use serde::{Deserialize, Serialize};

/// Maximum number of loops a sign can persist.
pub const MAX_SIGN_LOOPS: u32 = 50;

/// Maximum number of active signs at once.
pub const MAX_ACTIVE_SIGNS: usize = 10;

/// A warning sign placed by the player.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WarningSign {
    /// World position of the sign.
    position: IVec3,
    /// Message text on the sign.
    text: String,
    /// Number of loops until the sign fades.
    loops_remaining: u32,
}

impl WarningSign {
    /// Create a new warning sign.
    #[must_use]
    pub fn new(position: IVec3, text: impl Into<String>) -> Self {
        Self {
            position,
            text: text.into(),
            loops_remaining: MAX_SIGN_LOOPS,
        }
    }

    /// Age the sign by one loop.
    ///
    /// Returns true if the sign has expired (no loops remaining).
    pub fn age_one_loop(&mut self) -> bool {
        self.loops_remaining = self.loops_remaining.saturating_sub(1);
        self.loops_remaining == 0
    }

    /// Get the sign's position.
    #[must_use]
    pub fn position(&self) -> IVec3 {
        self.position
    }

    /// Get the sign's text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get remaining loops.
    #[must_use]
    pub fn loops_remaining(&self) -> u32 {
        self.loops_remaining
    }

    /// Check if the sign is about to expire (5 or fewer loops).
    #[must_use]
    pub fn is_fading(&self) -> bool {
        self.loops_remaining <= 5
    }
}

/// Manages all warning signs in the world.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MessageManager {
    /// Active warning signs.
    signs: Vec<WarningSign>,
    /// Maximum number of active signs.
    max_active: usize,
}

impl MessageManager {
    /// Create a new message manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            signs: Vec::new(),
            max_active: MAX_ACTIVE_SIGNS,
        }
    }

    /// Add a new warning sign.
    ///
    /// Returns false if at maximum capacity.
    pub fn add_sign(&mut self, sign: WarningSign) -> bool {
        if self.signs.len() >= self.max_active {
            return false;
        }
        self.signs.push(sign);
        true
    }

    /// Remove a sign by index.
    ///
    /// Returns the removed sign, or None if index is invalid.
    pub fn remove_sign(&mut self, index: usize) -> Option<WarningSign> {
        if index >= self.signs.len() {
            return None;
        }
        Some(self.signs.remove(index))
    }

    /// Age all signs by one loop and remove expired ones.
    pub fn age_all(&mut self) {
        self.signs.retain_mut(|sign| !sign.age_one_loop());
    }

    /// Get all signs within a radius of a position.
    #[must_use]
    pub fn signs_near(&self, pos: IVec3, radius: i32) -> Vec<&WarningSign> {
        let radius_sq = radius * radius;
        self.signs
            .iter()
            .filter(|sign| {
                let diff = sign.position - pos;
                diff.x * diff.x + diff.y * diff.y + diff.z * diff.z <= radius_sq
            })
            .collect()
    }

    /// Get total number of active signs.
    #[must_use]
    pub fn sign_count(&self) -> usize {
        self.signs.len()
    }

    /// Get all signs.
    #[must_use]
    pub fn all_signs(&self) -> &[WarningSign] {
        &self.signs
    }

    /// Check if at maximum capacity.
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.signs.len() >= self.max_active
    }

    /// Get count of fading signs.
    #[must_use]
    pub fn fading_count(&self) -> usize {
        self.signs.iter().filter(|s| s.is_fading()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warning_sign_new() {
        let sign = WarningSign::new(IVec3::new(10, 20, 30), "Danger ahead!");
        assert_eq!(sign.position(), IVec3::new(10, 20, 30));
        assert_eq!(sign.text(), "Danger ahead!");
        assert_eq!(sign.loops_remaining(), MAX_SIGN_LOOPS);
    }

    #[test]
    fn test_warning_sign_age() {
        let mut sign = WarningSign::new(IVec3::ZERO, "Test");
        assert!(!sign.age_one_loop());
        assert_eq!(sign.loops_remaining(), MAX_SIGN_LOOPS - 1);
    }

    #[test]
    fn test_warning_sign_expires() {
        let mut sign = WarningSign::new(IVec3::ZERO, "Test");
        sign.loops_remaining = 1;

        let expired = sign.age_one_loop();
        assert!(expired);
        assert_eq!(sign.loops_remaining(), 0);
    }

    #[test]
    fn test_warning_sign_is_fading() {
        let mut sign = WarningSign::new(IVec3::ZERO, "Test");
        assert!(!sign.is_fading());

        sign.loops_remaining = 5;
        assert!(sign.is_fading());

        sign.loops_remaining = 3;
        assert!(sign.is_fading());
    }

    #[test]
    fn test_manager_new() {
        let manager = MessageManager::new();
        assert_eq!(manager.sign_count(), 0);
        assert!(!manager.is_full());
    }

    #[test]
    fn test_manager_add_sign() {
        let mut manager = MessageManager::new();
        let sign = WarningSign::new(IVec3::ZERO, "Warning");

        let added = manager.add_sign(sign);
        assert!(added);
        assert_eq!(manager.sign_count(), 1);
    }

    #[test]
    fn test_manager_add_at_capacity() {
        let mut manager = MessageManager::new();
        manager.max_active = 2;

        manager.add_sign(WarningSign::new(IVec3::ZERO, "1"));
        manager.add_sign(WarningSign::new(IVec3::ZERO, "2"));
        let added = manager.add_sign(WarningSign::new(IVec3::ZERO, "3"));

        assert!(!added);
        assert_eq!(manager.sign_count(), 2);
    }

    #[test]
    fn test_manager_remove_sign() {
        let mut manager = MessageManager::new();
        manager.add_sign(WarningSign::new(IVec3::ZERO, "Test"));

        let removed = manager.remove_sign(0);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().text(), "Test");
        assert_eq!(manager.sign_count(), 0);
    }

    #[test]
    fn test_manager_age_all() {
        let mut manager = MessageManager::new();
        let mut sign1 = WarningSign::new(IVec3::ZERO, "Lasting");
        let mut sign2 = WarningSign::new(IVec3::ZERO, "Expiring");
        sign2.loops_remaining = 1;

        manager.add_sign(sign1);
        manager.add_sign(sign2);
        assert_eq!(manager.sign_count(), 2);

        manager.age_all();
        assert_eq!(manager.sign_count(), 1);
        assert_eq!(manager.all_signs()[0].text(), "Lasting");
    }

    #[test]
    fn test_manager_signs_near() {
        let mut manager = MessageManager::new();
        manager.add_sign(WarningSign::new(IVec3::new(0, 0, 0), "Close"));
        manager.add_sign(WarningSign::new(IVec3::new(5, 0, 0), "Near"));
        manager.add_sign(WarningSign::new(IVec3::new(100, 0, 0), "Far"));

        let nearby = manager.signs_near(IVec3::ZERO, 10);
        assert_eq!(nearby.len(), 2);
    }
}
