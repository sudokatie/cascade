//! Temporal visual effects for time-loop survival.
//!
//! Provides visual effects for loop transitions, paradox, ghost items, and chests.

use glam::{Vec3, Vec4};
use serde::{Deserialize, Serialize};

/// Color representation for effects.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct EffectColor {
    /// Red component (0.0 - 1.0).
    pub r: f32,
    /// Green component (0.0 - 1.0).
    pub g: f32,
    /// Blue component (0.0 - 1.0).
    pub b: f32,
    /// Alpha component (0.0 - 1.0).
    pub a: f32,
}

impl EffectColor {
    /// Create a new color.
    #[must_use]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// White color.
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);

    /// Transparent color.
    pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    /// Temporal blue (for time effects).
    pub const TEMPORAL_BLUE: Self = Self::new(0.2, 0.4, 0.9, 0.8);

    /// Paradox red (for paradox effects).
    pub const PARADOX_RED: Self = Self::new(0.9, 0.2, 0.2, 0.7);

    /// Ghost white (for ghost items).
    pub const GHOST_WHITE: Self = Self::new(0.8, 0.8, 1.0, 0.4);

    /// Chest gold (for temporal chests).
    pub const CHEST_GOLD: Self = Self::new(1.0, 0.8, 0.2, 0.6);

    /// Convert to Vec4.
    #[must_use]
    pub fn to_vec4(self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.a)
    }

    /// Interpolate between two colors.
    #[must_use]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    /// Apply alpha multiplier.
    #[must_use]
    pub fn with_alpha(self, alpha: f32) -> Self {
        Self {
            a: self.a * alpha,
            ..self
        }
    }
}

/// Loop transition effect state.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LoopTransitionEffect {
    /// Whether the transition is active.
    active: bool,
    /// Progress (0.0 - 1.0).
    progress: f32,
    /// Duration in seconds.
    duration: f32,
    /// Effect intensity.
    intensity: f32,
    /// Source loop number.
    from_loop: u32,
    /// Destination loop number.
    to_loop: u32,
}

impl LoopTransitionEffect {
    /// Create a new loop transition effect.
    #[must_use]
    pub fn new() -> Self {
        Self {
            active: false,
            progress: 0.0,
            duration: 1.5,
            intensity: 1.0,
            from_loop: 0,
            to_loop: 0,
        }
    }

    /// Start a loop transition.
    pub fn start(&mut self, from_loop: u32, to_loop: u32) {
        self.active = true;
        self.progress = 0.0;
        self.from_loop = from_loop;
        self.to_loop = to_loop;
    }

    /// Update the effect.
    pub fn update(&mut self, delta_time: f32) {
        if !self.active {
            return;
        }
        self.progress += delta_time / self.duration;
        if self.progress >= 1.0 {
            self.progress = 1.0;
            self.active = false;
        }
    }

    /// Check if the effect is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get the current progress.
    #[must_use]
    pub fn progress(&self) -> f32 {
        self.progress
    }

    /// Get the current color based on progress.
    #[must_use]
    pub fn current_color(&self) -> EffectColor {
        if !self.active {
            return EffectColor::TRANSPARENT;
        }
        // Fade to white at midpoint, then back
        let white_intensity = if self.progress < 0.5 {
            self.progress * 2.0
        } else {
            (1.0 - self.progress) * 2.0
        };
        EffectColor::WHITE.with_alpha(white_intensity * self.intensity)
    }

    /// Get screen shake intensity.
    #[must_use]
    pub fn shake_intensity(&self) -> f32 {
        if !self.active {
            return 0.0;
        }
        // Peak shake at midpoint
        let base = if self.progress < 0.5 {
            self.progress * 2.0
        } else {
            (1.0 - self.progress) * 2.0
        };
        base * self.intensity * 0.02
    }

    /// Set effect intensity.
    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity.clamp(0.0, 2.0);
    }

    /// Set duration.
    pub fn set_duration(&mut self, duration: f32) {
        self.duration = duration.max(0.1);
    }
}

/// Paradox glow effect.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParadoxGlowEffect {
    /// Whether the effect is active.
    active: bool,
    /// Current paradox level (0.0 - 1.0).
    level: f32,
    /// Pulse phase.
    pulse_phase: f32,
    /// Pulse speed.
    pulse_speed: f32,
}

impl ParadoxGlowEffect {
    /// Create a new paradox glow effect.
    #[must_use]
    pub fn new() -> Self {
        Self {
            active: false,
            level: 0.0,
            pulse_phase: 0.0,
            pulse_speed: 2.0,
        }
    }

    /// Set the paradox level.
    pub fn set_level(&mut self, level: f32) {
        self.level = level.clamp(0.0, 1.0);
        self.active = self.level > 0.1;
    }

    /// Update the effect.
    pub fn update(&mut self, delta_time: f32) {
        if !self.active {
            return;
        }
        self.pulse_phase += delta_time * self.pulse_speed;
        if self.pulse_phase > std::f32::consts::TAU {
            self.pulse_phase -= std::f32::consts::TAU;
        }
    }

    /// Check if the effect is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get the current color.
    #[must_use]
    pub fn current_color(&self) -> EffectColor {
        if !self.active {
            return EffectColor::TRANSPARENT;
        }
        let pulse = (self.pulse_phase.sin() * 0.5 + 0.5) * 0.3 + 0.7;
        EffectColor::PARADOX_RED.with_alpha(self.level * pulse)
    }

    /// Get the glow radius multiplier.
    #[must_use]
    pub fn glow_radius(&self) -> f32 {
        if !self.active {
            return 0.0;
        }
        let pulse = self.pulse_phase.sin() * 0.2 + 1.0;
        self.level * pulse * 2.0
    }
}

/// Ghost item visual effect.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GhostItemEffect {
    /// Position of the ghost item.
    position: Vec3,
    /// Opacity (0.0 - 1.0).
    opacity: f32,
    /// Loop number when the item existed.
    from_loop: u32,
    /// Whether the effect is visible.
    visible: bool,
    /// Flicker phase.
    flicker_phase: f32,
}

impl GhostItemEffect {
    /// Create a new ghost item effect.
    #[must_use]
    pub fn new(position: Vec3, from_loop: u32) -> Self {
        Self {
            position,
            opacity: 0.4,
            from_loop,
            visible: true,
            flicker_phase: 0.0,
        }
    }

    /// Get the position.
    #[must_use]
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// Get the source loop number.
    #[must_use]
    pub fn from_loop(&self) -> u32 {
        self.from_loop
    }

    /// Check if visible.
    #[must_use]
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Update the effect.
    pub fn update(&mut self, delta_time: f32) {
        self.flicker_phase += delta_time * 3.0;
        if self.flicker_phase > std::f32::consts::TAU {
            self.flicker_phase -= std::f32::consts::TAU;
        }
    }

    /// Get the current color.
    #[must_use]
    pub fn current_color(&self) -> EffectColor {
        if !self.visible {
            return EffectColor::TRANSPARENT;
        }
        let flicker = self.flicker_phase.sin() * 0.1 + 0.9;
        EffectColor::GHOST_WHITE.with_alpha(self.opacity * flicker)
    }

    /// Set visibility.
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Set opacity.
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }
}

impl Default for GhostItemEffect {
    fn default() -> Self {
        Self::new(Vec3::ZERO, 0)
    }
}

/// Temporal chest aura effect.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChestAuraEffect {
    /// Position of the chest.
    position: Vec3,
    /// Aura radius.
    radius: f32,
    /// Aura intensity.
    intensity: f32,
    /// Rotation phase.
    rotation_phase: f32,
    /// Whether items are stored in the chest.
    has_items: bool,
}

impl ChestAuraEffect {
    /// Create a new chest aura effect.
    #[must_use]
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            radius: 1.5,
            intensity: 1.0,
            rotation_phase: 0.0,
            has_items: false,
        }
    }

    /// Get the position.
    #[must_use]
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// Get the radius.
    #[must_use]
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// Check if the chest has items.
    #[must_use]
    pub fn has_items(&self) -> bool {
        self.has_items
    }

    /// Set whether the chest has items.
    pub fn set_has_items(&mut self, has_items: bool) {
        self.has_items = has_items;
        self.intensity = if has_items { 1.5 } else { 1.0 };
    }

    /// Update the effect.
    pub fn update(&mut self, delta_time: f32) {
        self.rotation_phase += delta_time * 1.5;
        if self.rotation_phase > std::f32::consts::TAU {
            self.rotation_phase -= std::f32::consts::TAU;
        }
    }

    /// Get the current color.
    #[must_use]
    pub fn current_color(&self) -> EffectColor {
        let pulse = self.rotation_phase.sin() * 0.2 + 0.8;
        EffectColor::CHEST_GOLD.with_alpha(self.intensity * pulse * 0.6)
    }

    /// Get particle positions around the chest.
    #[must_use]
    pub fn particle_positions(&self) -> Vec<Vec3> {
        let count = if self.has_items { 8 } else { 4 };
        let mut positions = Vec::with_capacity(count);
        for i in 0..count {
            let angle = self.rotation_phase + (i as f32 * std::f32::consts::TAU / count as f32);
            let x = self.position.x + angle.cos() * self.radius;
            let z = self.position.z + angle.sin() * self.radius;
            let y = self.position.y + (angle * 2.0).sin() * 0.3;
            positions.push(Vec3::new(x, y, z));
        }
        positions
    }
}

impl Default for ChestAuraEffect {
    fn default() -> Self {
        Self::new(Vec3::ZERO)
    }
}

/// Manager for all temporal visual effects.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TemporalVisuals {
    /// Loop transition effect.
    pub loop_transition: LoopTransitionEffect,
    /// Paradox glow effect.
    pub paradox_glow: ParadoxGlowEffect,
    /// Ghost item effects.
    pub ghost_items: Vec<GhostItemEffect>,
    /// Chest aura effects.
    pub chest_auras: Vec<ChestAuraEffect>,
}

impl TemporalVisuals {
    /// Create a new temporal visuals manager.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Update all effects.
    pub fn update(&mut self, delta_time: f32) {
        self.loop_transition.update(delta_time);
        self.paradox_glow.update(delta_time);
        for ghost in &mut self.ghost_items {
            ghost.update(delta_time);
        }
        for chest in &mut self.chest_auras {
            chest.update(delta_time);
        }
    }

    /// Add a ghost item effect.
    pub fn add_ghost_item(&mut self, position: Vec3, from_loop: u32) {
        self.ghost_items.push(GhostItemEffect::new(position, from_loop));
    }

    /// Add a chest aura effect.
    pub fn add_chest_aura(&mut self, position: Vec3) {
        self.chest_auras.push(ChestAuraEffect::new(position));
    }

    /// Clear all effects.
    pub fn clear(&mut self) {
        self.ghost_items.clear();
        self.chest_auras.clear();
    }

    /// Start a loop transition.
    pub fn start_loop_transition(&mut self, from_loop: u32, to_loop: u32) {
        self.loop_transition.start(from_loop, to_loop);
    }

    /// Set paradox level.
    pub fn set_paradox_level(&mut self, level: f32) {
        self.paradox_glow.set_level(level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effect_color_new() {
        let color = EffectColor::new(0.5, 0.6, 0.7, 0.8);
        assert!((color.r - 0.5).abs() < f32::EPSILON);
        assert!((color.g - 0.6).abs() < f32::EPSILON);
        assert!((color.b - 0.7).abs() < f32::EPSILON);
        assert!((color.a - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_effect_color_lerp() {
        let a = EffectColor::new(0.0, 0.0, 0.0, 1.0);
        let b = EffectColor::new(1.0, 1.0, 1.0, 1.0);
        let mid = a.lerp(b, 0.5);

        assert!((mid.r - 0.5).abs() < f32::EPSILON);
        assert!((mid.g - 0.5).abs() < f32::EPSILON);
        assert!((mid.b - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_effect_color_with_alpha() {
        let color = EffectColor::WHITE.with_alpha(0.5);
        assert!((color.a - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_loop_transition_effect() {
        let mut effect = LoopTransitionEffect::new();
        assert!(!effect.is_active());

        effect.start(1, 2);
        assert!(effect.is_active());
        assert!((effect.progress() - 0.0).abs() < f32::EPSILON);

        effect.update(0.75);
        assert!(effect.is_active());
        assert!(effect.progress() > 0.0);

        effect.update(1.0);
        assert!(!effect.is_active());
    }

    #[test]
    fn test_loop_transition_color() {
        let mut effect = LoopTransitionEffect::new();
        effect.start(1, 2);

        let color = effect.current_color();
        assert!(color.a > 0.0);
    }

    #[test]
    fn test_loop_transition_shake() {
        let mut effect = LoopTransitionEffect::new();
        effect.start(1, 2);
        effect.update(0.75);

        let shake = effect.shake_intensity();
        assert!(shake > 0.0);
    }

    #[test]
    fn test_paradox_glow_effect() {
        let mut effect = ParadoxGlowEffect::new();
        assert!(!effect.is_active());

        effect.set_level(0.5);
        assert!(effect.is_active());

        effect.update(0.5);
        let color = effect.current_color();
        assert!(color.a > 0.0);
    }

    #[test]
    fn test_paradox_glow_radius() {
        let mut effect = ParadoxGlowEffect::new();
        effect.set_level(0.8);

        let radius = effect.glow_radius();
        assert!(radius > 0.0);
    }

    #[test]
    fn test_ghost_item_effect() {
        let mut effect = GhostItemEffect::new(Vec3::new(1.0, 2.0, 3.0), 5);

        assert!(effect.is_visible());
        assert_eq!(effect.from_loop(), 5);
        assert_eq!(effect.position(), Vec3::new(1.0, 2.0, 3.0));

        effect.update(0.5);
        let color = effect.current_color();
        assert!(color.a > 0.0);
    }

    #[test]
    fn test_ghost_item_visibility() {
        let mut effect = GhostItemEffect::new(Vec3::ZERO, 1);
        effect.set_visible(false);

        let color = effect.current_color();
        assert!((color.a - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_chest_aura_effect() {
        let mut effect = ChestAuraEffect::new(Vec3::new(5.0, 0.0, 5.0));

        assert!(!effect.has_items());
        effect.set_has_items(true);
        assert!(effect.has_items());

        effect.update(0.5);
        let color = effect.current_color();
        assert!(color.a > 0.0);
    }

    #[test]
    fn test_chest_aura_particles() {
        let effect = ChestAuraEffect::new(Vec3::ZERO);
        let particles = effect.particle_positions();

        assert_eq!(particles.len(), 4);
    }

    #[test]
    fn test_chest_aura_particles_with_items() {
        let mut effect = ChestAuraEffect::new(Vec3::ZERO);
        effect.set_has_items(true);
        let particles = effect.particle_positions();

        assert_eq!(particles.len(), 8);
    }

    #[test]
    fn test_temporal_visuals_new() {
        let visuals = TemporalVisuals::new();
        assert!(visuals.ghost_items.is_empty());
        assert!(visuals.chest_auras.is_empty());
    }

    #[test]
    fn test_temporal_visuals_add_effects() {
        let mut visuals = TemporalVisuals::new();

        visuals.add_ghost_item(Vec3::ZERO, 1);
        visuals.add_chest_aura(Vec3::new(10.0, 0.0, 10.0));

        assert_eq!(visuals.ghost_items.len(), 1);
        assert_eq!(visuals.chest_auras.len(), 1);
    }

    #[test]
    fn test_temporal_visuals_clear() {
        let mut visuals = TemporalVisuals::new();
        visuals.add_ghost_item(Vec3::ZERO, 1);
        visuals.add_chest_aura(Vec3::ZERO);

        visuals.clear();
        assert!(visuals.ghost_items.is_empty());
        assert!(visuals.chest_auras.is_empty());
    }

    #[test]
    fn test_temporal_visuals_start_transition() {
        let mut visuals = TemporalVisuals::new();
        visuals.start_loop_transition(1, 2);

        assert!(visuals.loop_transition.is_active());
    }

    #[test]
    fn test_temporal_visuals_set_paradox() {
        let mut visuals = TemporalVisuals::new();
        visuals.set_paradox_level(0.8);

        assert!(visuals.paradox_glow.is_active());
    }
}
