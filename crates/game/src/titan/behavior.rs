//! Titan behavior and mood system.
//!
//! Manages the Titan's health, mood, and reactions to player actions.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Maximum Titan health points.
pub const MAX_TITAN_HP: f32 = 10000.0;

/// Agitation threshold for becoming Agitated.
pub const AGITATED_THRESHOLD: f32 = 30.0;

/// Agitation threshold for becoming Enraged.
pub const ENRAGED_THRESHOLD: f32 = 70.0;

/// Agitation increase from harvesting tissue.
pub const HARVEST_AGITATION: f32 = 10.0;

/// Agitation decrease from killing a parasite.
pub const PARASITE_KILL_RELIEF: f32 = 5.0;

/// The Titan's current mood state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TitanMood {
    /// Titan is calm, minimal threats.
    #[default]
    Calm,
    /// Titan is agitated, increased hazards.
    Agitated,
    /// Titan is enraged, maximum danger.
    Enraged,
}

impl fmt::Display for TitanMood {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TitanMood::Calm => write!(f, "Calm"),
            TitanMood::Agitated => write!(f, "Agitated"),
            TitanMood::Enraged => write!(f, "Enraged"),
        }
    }
}

/// Manages the Titan's behavioral state.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TitanBehavior {
    /// Current health points.
    hp: f32,
    /// Current mood state.
    mood: TitanMood,
    /// Agitation level (0-100).
    mood_agitation: f32,
    /// Target agitation for smooth transitions.
    target_agitation: f32,
    /// How fast mood changes (default 1.0).
    mood_transition_rate: f32,
}

impl TitanBehavior {
    /// Create a new Titan behavior manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            hp: MAX_TITAN_HP,
            mood: TitanMood::Calm,
            mood_agitation: 0.0,
            target_agitation: 0.0,
            mood_transition_rate: 1.0,
        }
    }

    /// Create a new Titan behavior manager with custom transition rate.
    #[must_use]
    pub fn with_transition_rate(mut self, rate: f32) -> Self {
        self.mood_transition_rate = rate;
        self
    }

    /// Deal damage to the Titan.
    ///
    /// Reduces HP and increases agitation. Returns the new mood.
    pub fn deal_damage(&mut self, amount: f32) -> TitanMood {
        self.hp = (self.hp - amount).max(0.0);
        // Damage increases agitation proportionally
        let agitation_increase = (amount / 100.0).min(20.0);
        self.add_agitation(agitation_increase);
        self.mood
    }

    /// Called when the player kills a parasite.
    ///
    /// Decreases agitation as the Titan appreciates the help.
    pub fn kill_parasite(&mut self) {
        self.add_agitation(-PARASITE_KILL_RELIEF);
    }

    /// Called when the player harvests Titan tissue.
    ///
    /// Increases agitation as the Titan feels the damage.
    pub fn harvest_tissue(&mut self) {
        self.add_agitation(HARVEST_AGITATION);
    }

    /// Update the behavior system.
    ///
    /// Natural mood decay occurs over time. Returns `Some(new_mood)` if mood changed.
    pub fn tick(&mut self, _dt: f32) -> Option<TitanMood> {
        let old_mood = self.mood;

        // Natural agitation decay based on current mood
        let decay = match self.mood {
            TitanMood::Calm => 0.5,
            TitanMood::Agitated => 0.2,
            TitanMood::Enraged => 0.1,
        };
        self.add_agitation(-decay);

        if self.mood != old_mood {
            Some(self.mood)
        } else {
            None
        }
    }

    /// Get the current mood.
    #[must_use]
    pub fn current_mood(&self) -> TitanMood {
        self.mood
    }

    /// Get the current HP as a percentage (0.0-1.0).
    #[must_use]
    pub fn hp_percentage(&self) -> f32 {
        self.hp / MAX_TITAN_HP
    }

    /// Check if the Titan is still alive.
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.hp > 0.0
    }

    /// Get current agitation level.
    #[must_use]
    pub fn agitation(&self) -> f32 {
        self.mood_agitation
    }

    /// Get current HP.
    #[must_use]
    pub fn hp(&self) -> f32 {
        self.hp
    }

    /// Get the mood transition rate.
    #[must_use]
    pub fn mood_transition_rate(&self) -> f32 {
        self.mood_transition_rate
    }

    /// Set the mood transition rate.
    pub fn set_mood_transition_rate(&mut self, rate: f32) {
        self.mood_transition_rate = rate.max(0.0);
    }

    /// Smoothly transition mood agitation toward the target.
    ///
    /// Instead of instant mood changes, this gradually shifts `mood_agitation`
    /// toward `target_agitation` based on `mood_transition_rate`.
    pub fn smooth_mood_transition(&mut self, dt: f32) {
        let diff = self.target_agitation - self.mood_agitation;
        let change = diff.signum() * (diff.abs().min(self.mood_transition_rate * dt * 10.0));
        self.mood_agitation = (self.mood_agitation + change).clamp(0.0, 100.0);
        self.update_mood();
    }

    /// Apply neural guidance effect from neural interface.
    ///
    /// When the neural interface guides the Titan, reduce agitation by 2.0 per tick.
    pub fn neural_guidance_effect(&mut self) {
        self.add_agitation(-2.0);
    }

    /// Recover HP during natural regeneration (e.g., Resting phase).
    ///
    /// Returns the actual amount healed.
    pub fn recover_hp(&mut self, amount: f32) -> f32 {
        let old_hp = self.hp;
        self.hp = (self.hp + amount).min(MAX_TITAN_HP);
        self.hp - old_hp
    }

    fn add_agitation(&mut self, amount: f32) {
        self.mood_agitation = (self.mood_agitation + amount).clamp(0.0, 100.0);
        self.target_agitation = self.mood_agitation;
        self.update_mood();
    }

    fn update_mood(&mut self) {
        // HP thresholds override agitation-based mood
        let hp_percent = self.hp_percentage();
        self.mood = if hp_percent <= 0.25 {
            // Below 25% HP = auto-Enraged
            TitanMood::Enraged
        } else if hp_percent <= 0.5 {
            // Below 50% HP = auto-Agitated (at minimum)
            if self.mood_agitation >= ENRAGED_THRESHOLD {
                TitanMood::Enraged
            } else {
                TitanMood::Agitated
            }
        } else if self.mood_agitation >= ENRAGED_THRESHOLD {
            TitanMood::Enraged
        } else if self.mood_agitation >= AGITATED_THRESHOLD {
            TitanMood::Agitated
        } else {
            TitanMood::Calm
        };
    }
}

impl Default for TitanBehavior {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_titan_mood_display() {
        assert_eq!(format!("{}", TitanMood::Calm), "Calm");
        assert_eq!(format!("{}", TitanMood::Agitated), "Agitated");
        assert_eq!(format!("{}", TitanMood::Enraged), "Enraged");
    }

    #[test]
    fn test_titan_mood_default() {
        assert_eq!(TitanMood::default(), TitanMood::Calm);
    }

    #[test]
    fn test_titan_behavior_new() {
        let behavior = TitanBehavior::new();
        assert!((behavior.hp() - MAX_TITAN_HP).abs() < f32::EPSILON);
        assert_eq!(behavior.current_mood(), TitanMood::Calm);
        assert!((behavior.agitation() - 0.0).abs() < f32::EPSILON);
        assert!(behavior.is_alive());
    }

    #[test]
    fn test_titan_behavior_deal_damage() {
        let mut behavior = TitanBehavior::new();
        behavior.deal_damage(500.0);
        assert!((behavior.hp() - 9500.0).abs() < f32::EPSILON);
        assert!(behavior.agitation() > 0.0);
    }

    #[test]
    fn test_titan_behavior_kill_parasite() {
        let mut behavior = TitanBehavior::new();
        // First agitate the titan
        behavior.harvest_tissue();
        behavior.harvest_tissue();
        behavior.harvest_tissue();
        let agitation_before = behavior.agitation();
        behavior.kill_parasite();
        assert!(behavior.agitation() < agitation_before);
    }

    #[test]
    fn test_titan_behavior_harvest_tissue() {
        let mut behavior = TitanBehavior::new();
        behavior.harvest_tissue();
        assert!((behavior.agitation() - HARVEST_AGITATION).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_behavior_mood_transitions() {
        let mut behavior = TitanBehavior::new();
        assert_eq!(behavior.current_mood(), TitanMood::Calm);

        // Push to Agitated
        for _ in 0..4 {
            behavior.harvest_tissue();
        }
        assert_eq!(behavior.current_mood(), TitanMood::Agitated);

        // Push to Enraged
        for _ in 0..4 {
            behavior.harvest_tissue();
        }
        assert_eq!(behavior.current_mood(), TitanMood::Enraged);
    }

    #[test]
    fn test_titan_behavior_tick_decay() {
        let mut behavior = TitanBehavior::new();
        // Set some agitation
        behavior.harvest_tissue();
        let initial = behavior.agitation();
        behavior.tick(1.0);
        assert!(behavior.agitation() < initial);
    }

    #[test]
    fn test_titan_behavior_hp_percentage() {
        let mut behavior = TitanBehavior::new();
        assert!((behavior.hp_percentage() - 1.0).abs() < f32::EPSILON);
        behavior.deal_damage(5000.0);
        assert!((behavior.hp_percentage() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_titan_behavior_is_alive() {
        let mut behavior = TitanBehavior::new();
        assert!(behavior.is_alive());
        behavior.deal_damage(MAX_TITAN_HP);
        assert!(!behavior.is_alive());
    }

    #[test]
    fn test_titan_behavior_mood_transition_rate() {
        let behavior = TitanBehavior::new();
        assert!((behavior.mood_transition_rate() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_behavior_with_transition_rate() {
        let behavior = TitanBehavior::new().with_transition_rate(2.0);
        assert!((behavior.mood_transition_rate() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_behavior_set_transition_rate() {
        let mut behavior = TitanBehavior::new();
        behavior.set_mood_transition_rate(0.5);
        assert!((behavior.mood_transition_rate() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_behavior_smooth_mood_transition_increases() {
        let mut behavior = TitanBehavior::new();
        behavior.target_agitation = 50.0;
        behavior.smooth_mood_transition(1.0);
        assert!(behavior.agitation() > 0.0);
        assert!(behavior.agitation() <= 50.0);
    }

    #[test]
    fn test_titan_behavior_smooth_mood_transition_decreases() {
        let mut behavior = TitanBehavior::new();
        behavior.mood_agitation = 50.0;
        behavior.target_agitation = 0.0;
        behavior.smooth_mood_transition(1.0);
        assert!(behavior.agitation() < 50.0);
    }

    #[test]
    fn test_titan_behavior_neural_guidance_effect() {
        let mut behavior = TitanBehavior::new();
        behavior.harvest_tissue(); // Add agitation
        behavior.harvest_tissue();
        let initial = behavior.agitation();
        behavior.neural_guidance_effect();
        assert!((behavior.agitation() - (initial - 2.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_behavior_neural_guidance_clamps_at_zero() {
        let mut behavior = TitanBehavior::new();
        behavior.neural_guidance_effect();
        assert!((behavior.agitation() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_behavior_recover_hp() {
        let mut behavior = TitanBehavior::new();
        behavior.deal_damage(1000.0);
        let healed = behavior.recover_hp(500.0);
        assert!((healed - 500.0).abs() < f32::EPSILON);
        assert!((behavior.hp() - 9500.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_behavior_recover_hp_capped() {
        let mut behavior = TitanBehavior::new();
        behavior.deal_damage(100.0);
        let healed = behavior.recover_hp(500.0);
        assert!((healed - 100.0).abs() < f32::EPSILON);
        assert!((behavior.hp() - MAX_TITAN_HP).abs() < f32::EPSILON);
    }

    #[test]
    fn test_titan_behavior_hp_threshold_auto_agitated() {
        let mut behavior = TitanBehavior::new();
        // Deal enough damage to get below 50% HP
        behavior.deal_damage(5001.0);
        assert!(behavior.hp_percentage() < 0.5);
        assert_eq!(behavior.current_mood(), TitanMood::Agitated);
    }

    #[test]
    fn test_titan_behavior_hp_threshold_auto_enraged() {
        let mut behavior = TitanBehavior::new();
        // Deal enough damage to get below 25% HP
        behavior.deal_damage(7501.0);
        assert!(behavior.hp_percentage() < 0.25);
        assert_eq!(behavior.current_mood(), TitanMood::Enraged);
    }

    #[test]
    fn test_titan_behavior_hp_threshold_overrides_low_agitation() {
        let mut behavior = TitanBehavior::new();
        // HP below 25% should force Enraged even with 0 agitation
        behavior.hp = MAX_TITAN_HP * 0.2;
        behavior.mood_agitation = 0.0;
        behavior.update_mood();
        assert_eq!(behavior.current_mood(), TitanMood::Enraged);
    }

    #[test]
    fn test_titan_behavior_high_agitation_at_low_hp() {
        let mut behavior = TitanBehavior::new();
        // HP below 50% but high agitation should still be Enraged
        behavior.hp = MAX_TITAN_HP * 0.4;
        behavior.mood_agitation = ENRAGED_THRESHOLD;
        behavior.update_mood();
        assert_eq!(behavior.current_mood(), TitanMood::Enraged);
    }
}
