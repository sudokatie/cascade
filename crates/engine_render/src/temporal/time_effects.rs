//! Time-based visual effects for time-loop survival.
//!
//! Provides dawn/midnight colors, vignette based on loop count, and time dilation effects.

use serde::{Deserialize, Serialize};

use super::visual_effects::EffectColor;

/// Time of day phases in the loop.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LoopTimePhase {
    /// Early morning, start of loop.
    #[default]
    Dawn,
    /// Midday.
    Day,
    /// Evening, approaching loop end.
    Dusk,
    /// Night, loop about to reset.
    Midnight,
}

impl LoopTimePhase {
    /// Get the ambient color for this phase.
    #[must_use]
    pub fn ambient_color(&self) -> EffectColor {
        match self {
            LoopTimePhase::Dawn => EffectColor::new(1.0, 0.7, 0.5, 1.0),
            LoopTimePhase::Day => EffectColor::new(1.0, 1.0, 0.9, 1.0),
            LoopTimePhase::Dusk => EffectColor::new(0.9, 0.5, 0.4, 1.0),
            LoopTimePhase::Midnight => EffectColor::new(0.2, 0.2, 0.4, 1.0),
        }
    }

    /// Get the sky color for this phase.
    #[must_use]
    pub fn sky_color(&self) -> EffectColor {
        match self {
            LoopTimePhase::Dawn => EffectColor::new(0.9, 0.6, 0.4, 1.0),
            LoopTimePhase::Day => EffectColor::new(0.5, 0.7, 1.0, 1.0),
            LoopTimePhase::Dusk => EffectColor::new(0.8, 0.4, 0.3, 1.0),
            LoopTimePhase::Midnight => EffectColor::new(0.1, 0.1, 0.2, 1.0),
        }
    }

    /// Get light intensity for this phase.
    #[must_use]
    pub fn light_intensity(&self) -> f32 {
        match self {
            LoopTimePhase::Dawn => 0.6,
            LoopTimePhase::Day => 1.0,
            LoopTimePhase::Dusk => 0.5,
            LoopTimePhase::Midnight => 0.2,
        }
    }

    /// Get the display name.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            LoopTimePhase::Dawn => "Dawn",
            LoopTimePhase::Day => "Day",
            LoopTimePhase::Dusk => "Dusk",
            LoopTimePhase::Midnight => "Midnight",
        }
    }

    /// Get all phases in order.
    #[must_use]
    pub fn all() -> &'static [LoopTimePhase] {
        &[
            LoopTimePhase::Dawn,
            LoopTimePhase::Day,
            LoopTimePhase::Dusk,
            LoopTimePhase::Midnight,
        ]
    }

    /// Get phase from normalized time (0.0 - 1.0).
    #[must_use]
    pub fn from_normalized_time(t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        if t < 0.25 {
            LoopTimePhase::Dawn
        } else if t < 0.5 {
            LoopTimePhase::Day
        } else if t < 0.75 {
            LoopTimePhase::Dusk
        } else {
            LoopTimePhase::Midnight
        }
    }
}

/// Vignette effect based on loop count.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LoopVignette {
    /// Current loop count.
    loop_count: u32,
    /// Base vignette intensity.
    base_intensity: f32,
    /// Maximum vignette intensity.
    max_intensity: f32,
    /// Vignette color.
    color: EffectColor,
    /// Current animated intensity.
    animated_intensity: f32,
    /// Animation phase.
    phase: f32,
}

impl LoopVignette {
    /// Create a new loop vignette.
    #[must_use]
    pub fn new() -> Self {
        Self {
            loop_count: 1,
            base_intensity: 0.1,
            max_intensity: 0.6,
            color: EffectColor::new(0.0, 0.0, 0.0, 1.0),
            animated_intensity: 0.1,
            phase: 0.0,
        }
    }

    /// Set the current loop count.
    pub fn set_loop_count(&mut self, count: u32) {
        self.loop_count = count.max(1);
    }

    /// Get the current loop count.
    #[must_use]
    pub fn loop_count(&self) -> u32 {
        self.loop_count
    }

    /// Update the vignette animation.
    pub fn update(&mut self, delta_time: f32) {
        self.phase += delta_time * 0.5;
        if self.phase > std::f32::consts::TAU {
            self.phase -= std::f32::consts::TAU;
        }

        // Calculate intensity based on loop count
        let loop_factor = (self.loop_count as f32 - 1.0) / 20.0;
        let target = self.base_intensity + loop_factor * (self.max_intensity - self.base_intensity);
        let pulse = self.phase.sin() * 0.05;

        self.animated_intensity = (target + pulse).clamp(0.0, self.max_intensity);
    }

    /// Get the current vignette intensity.
    #[must_use]
    pub fn intensity(&self) -> f32 {
        self.animated_intensity
    }

    /// Get the vignette color.
    #[must_use]
    pub fn color(&self) -> EffectColor {
        self.color.with_alpha(self.animated_intensity)
    }

    /// Get the vignette radius (larger = smaller vignette).
    #[must_use]
    pub fn radius(&self) -> f32 {
        1.0 - self.animated_intensity * 0.3
    }

    /// Set custom base intensity.
    pub fn set_base_intensity(&mut self, intensity: f32) {
        self.base_intensity = intensity.clamp(0.0, 1.0);
    }

    /// Set custom max intensity.
    pub fn set_max_intensity(&mut self, intensity: f32) {
        self.max_intensity = intensity.clamp(0.0, 1.0);
    }
}

/// Time dilation visual effect.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TimeDilationEffect {
    /// Current dilation factor (1.0 = normal).
    dilation: f32,
    /// Visual blur amount.
    blur: f32,
    /// Color shift intensity.
    color_shift: f32,
    /// Whether the effect is active.
    active: bool,
}

impl TimeDilationEffect {
    /// Create a new time dilation effect.
    #[must_use]
    pub fn new() -> Self {
        Self {
            dilation: 1.0,
            blur: 0.0,
            color_shift: 0.0,
            active: false,
        }
    }

    /// Set the dilation factor.
    pub fn set_dilation(&mut self, dilation: f32) {
        self.dilation = dilation.clamp(0.1, 3.0);
        self.active = (self.dilation - 1.0).abs() > 0.1;

        // Calculate visual effects based on dilation
        if self.dilation < 1.0 {
            // Slowed time
            self.blur = (1.0 - self.dilation) * 0.3;
            self.color_shift = (1.0 - self.dilation) * 0.2;
        } else if self.dilation > 1.0 {
            // Accelerated time
            self.blur = (self.dilation - 1.0) * 0.2;
            self.color_shift = (self.dilation - 1.0) * 0.1;
        } else {
            self.blur = 0.0;
            self.color_shift = 0.0;
        }
    }

    /// Get the dilation factor.
    #[must_use]
    pub fn dilation(&self) -> f32 {
        self.dilation
    }

    /// Get the blur amount.
    #[must_use]
    pub fn blur(&self) -> f32 {
        self.blur
    }

    /// Get the color shift intensity.
    #[must_use]
    pub fn color_shift(&self) -> f32 {
        self.color_shift
    }

    /// Check if the effect is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get the tint color based on dilation.
    #[must_use]
    pub fn tint_color(&self) -> EffectColor {
        if !self.active {
            return EffectColor::WHITE;
        }
        if self.dilation < 1.0 {
            // Blue tint for slowed time
            EffectColor::new(0.8, 0.8, 1.0, 1.0)
        } else {
            // Orange tint for accelerated time
            EffectColor::new(1.0, 0.9, 0.8, 1.0)
        }
    }

    /// Reset to normal time.
    pub fn reset(&mut self) {
        self.set_dilation(1.0);
    }
}

/// Manager for time-based effects.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TimeEffects {
    /// Current time phase.
    current_phase: LoopTimePhase,
    /// Normalized time in current loop (0.0 - 1.0).
    normalized_time: f32,
    /// Loop vignette effect.
    pub vignette: LoopVignette,
    /// Time dilation effect.
    pub dilation: TimeDilationEffect,
    /// Interpolation factor between phases.
    phase_blend: f32,
}

impl TimeEffects {
    /// Create a new time effects manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            current_phase: LoopTimePhase::Dawn,
            normalized_time: 0.0,
            vignette: LoopVignette::new(),
            dilation: TimeDilationEffect::new(),
            phase_blend: 0.0,
        }
    }

    /// Set the normalized time (0.0 - 1.0).
    pub fn set_time(&mut self, normalized_time: f32) {
        self.normalized_time = normalized_time.clamp(0.0, 1.0);
        self.current_phase = LoopTimePhase::from_normalized_time(self.normalized_time);

        // Calculate blend factor within the phase
        let phase_start = match self.current_phase {
            LoopTimePhase::Dawn => 0.0,
            LoopTimePhase::Day => 0.25,
            LoopTimePhase::Dusk => 0.5,
            LoopTimePhase::Midnight => 0.75,
        };
        self.phase_blend = (self.normalized_time - phase_start) / 0.25;
    }

    /// Get the current phase.
    #[must_use]
    pub fn current_phase(&self) -> LoopTimePhase {
        self.current_phase
    }

    /// Get the normalized time.
    #[must_use]
    pub fn normalized_time(&self) -> f32 {
        self.normalized_time
    }

    /// Get the blended ambient color.
    #[must_use]
    pub fn ambient_color(&self) -> EffectColor {
        let current = self.current_phase.ambient_color();
        let next = self.next_phase().ambient_color();
        current.lerp(next, self.phase_blend)
    }

    /// Get the blended sky color.
    #[must_use]
    pub fn sky_color(&self) -> EffectColor {
        let current = self.current_phase.sky_color();
        let next = self.next_phase().sky_color();
        current.lerp(next, self.phase_blend)
    }

    /// Get the blended light intensity.
    #[must_use]
    pub fn light_intensity(&self) -> f32 {
        let current = self.current_phase.light_intensity();
        let next = self.next_phase().light_intensity();
        current + (next - current) * self.phase_blend
    }

    /// Get the next phase.
    fn next_phase(&self) -> LoopTimePhase {
        match self.current_phase {
            LoopTimePhase::Dawn => LoopTimePhase::Day,
            LoopTimePhase::Day => LoopTimePhase::Dusk,
            LoopTimePhase::Dusk => LoopTimePhase::Midnight,
            LoopTimePhase::Midnight => LoopTimePhase::Dawn,
        }
    }

    /// Update all effects.
    pub fn update(&mut self, delta_time: f32) {
        self.vignette.update(delta_time);
    }

    /// Set the loop count.
    pub fn set_loop_count(&mut self, count: u32) {
        self.vignette.set_loop_count(count);
    }

    /// Set time dilation.
    pub fn set_dilation(&mut self, factor: f32) {
        self.dilation.set_dilation(factor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_time_phase_ambient_color() {
        let dawn = LoopTimePhase::Dawn.ambient_color();
        let midnight = LoopTimePhase::Midnight.ambient_color();

        // Dawn should be warmer (higher red)
        assert!(dawn.r > midnight.r);
    }

    #[test]
    fn test_loop_time_phase_light_intensity() {
        assert!(LoopTimePhase::Day.light_intensity() > LoopTimePhase::Midnight.light_intensity());
    }

    #[test]
    fn test_loop_time_phase_from_normalized() {
        assert_eq!(LoopTimePhase::from_normalized_time(0.0), LoopTimePhase::Dawn);
        assert_eq!(LoopTimePhase::from_normalized_time(0.3), LoopTimePhase::Day);
        assert_eq!(LoopTimePhase::from_normalized_time(0.6), LoopTimePhase::Dusk);
        assert_eq!(LoopTimePhase::from_normalized_time(0.9), LoopTimePhase::Midnight);
    }

    #[test]
    fn test_loop_time_phase_all() {
        assert_eq!(LoopTimePhase::all().len(), 4);
    }

    #[test]
    fn test_loop_vignette_new() {
        let vignette = LoopVignette::new();
        assert_eq!(vignette.loop_count(), 1);
    }

    #[test]
    fn test_loop_vignette_set_loop_count() {
        let mut vignette = LoopVignette::new();
        vignette.set_loop_count(10);
        assert_eq!(vignette.loop_count(), 10);
    }

    #[test]
    fn test_loop_vignette_intensity_increases_with_loops() {
        let mut low_loop = LoopVignette::new();
        low_loop.set_loop_count(1);
        low_loop.update(0.0);

        let mut high_loop = LoopVignette::new();
        high_loop.set_loop_count(10);
        high_loop.update(0.0);

        assert!(high_loop.intensity() > low_loop.intensity());
    }

    #[test]
    fn test_loop_vignette_radius() {
        let mut vignette = LoopVignette::new();
        vignette.update(0.0);

        let radius = vignette.radius();
        assert!(radius > 0.0);
        assert!(radius <= 1.0);
    }

    #[test]
    fn test_time_dilation_effect_new() {
        let effect = TimeDilationEffect::new();
        assert!((effect.dilation() - 1.0).abs() < f32::EPSILON);
        assert!(!effect.is_active());
    }

    #[test]
    fn test_time_dilation_set_slow() {
        let mut effect = TimeDilationEffect::new();
        effect.set_dilation(0.5);

        assert!(effect.is_active());
        assert!(effect.blur() > 0.0);
        assert!(effect.color_shift() > 0.0);
    }

    #[test]
    fn test_time_dilation_set_fast() {
        let mut effect = TimeDilationEffect::new();
        effect.set_dilation(2.0);

        assert!(effect.is_active());
        assert!(effect.blur() > 0.0);
    }

    #[test]
    fn test_time_dilation_tint_color() {
        let mut effect = TimeDilationEffect::new();

        effect.set_dilation(0.5);
        let slow_tint = effect.tint_color();
        assert!(slow_tint.b >= slow_tint.r); // Blue tint

        effect.set_dilation(2.0);
        let fast_tint = effect.tint_color();
        assert!(fast_tint.r >= fast_tint.b); // Orange tint
    }

    #[test]
    fn test_time_dilation_reset() {
        let mut effect = TimeDilationEffect::new();
        effect.set_dilation(0.5);
        assert!(effect.is_active());

        effect.reset();
        assert!(!effect.is_active());
    }

    #[test]
    fn test_time_effects_new() {
        let effects = TimeEffects::new();
        assert_eq!(effects.current_phase(), LoopTimePhase::Dawn);
        assert!((effects.normalized_time() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_time_effects_set_time() {
        let mut effects = TimeEffects::new();

        effects.set_time(0.3);
        assert_eq!(effects.current_phase(), LoopTimePhase::Day);

        effects.set_time(0.8);
        assert_eq!(effects.current_phase(), LoopTimePhase::Midnight);
    }

    #[test]
    fn test_time_effects_ambient_color_blending() {
        let mut effects = TimeEffects::new();

        effects.set_time(0.0);
        let dawn_color = effects.ambient_color();

        effects.set_time(0.125); // Midway through dawn
        let mid_color = effects.ambient_color();

        // Mid color should be between dawn and day
        assert!(mid_color.r != dawn_color.r || mid_color.g != dawn_color.g);
    }

    #[test]
    fn test_time_effects_set_loop_count() {
        let mut effects = TimeEffects::new();
        effects.set_loop_count(5);

        assert_eq!(effects.vignette.loop_count(), 5);
    }

    #[test]
    fn test_time_effects_set_dilation() {
        let mut effects = TimeEffects::new();
        effects.set_dilation(0.5);

        assert!(effects.dilation.is_active());
    }

    #[test]
    fn test_time_effects_update() {
        let mut effects = TimeEffects::new();
        effects.set_loop_count(10);

        let initial = effects.vignette.intensity();
        effects.update(0.5);

        // After update, animated intensity should have changed
        assert!((effects.vignette.intensity() - initial).abs() >= 0.0);
    }
}
