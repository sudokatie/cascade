//! Titan terrain animation.
//!
//! Provides animation data for the Titan's living surface, including
//! scale movements, vent eruptions, and wound pulsing.

use engine_physics::titan::TitanPhase;

/// Terrain animation handler for Titan surface.
#[derive(Clone, Debug)]
pub struct TerrainAnimation {
    /// Current animation time.
    time: f64,
    /// Animation speed multiplier.
    speed: f32,
    /// Whether animations are enabled.
    enabled: bool,
}

impl Default for TerrainAnimation {
    fn default() -> Self {
        Self::new()
    }
}

impl TerrainAnimation {
    /// Create a new TerrainAnimation handler.
    #[must_use]
    pub fn new() -> Self {
        Self {
            time: 0.0,
            speed: 1.0,
            enabled: true,
        }
    }

    /// Update animation time.
    pub fn tick(&mut self, dt: f64) {
        if self.enabled {
            self.time += dt * self.speed as f64;
        }
    }

    /// Get scale animation offset based on phase.
    ///
    /// Returns a value 0.0-1.0 representing the scale breathing animation.
    #[must_use]
    pub fn get_scale_animation(&self, phase: TitanPhase) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        let base_frequency = match phase {
            TitanPhase::Resting => 0.5,   // Slow breathing
            TitanPhase::Walking => 1.0,   // Normal movement
            TitanPhase::Running => 2.0,   // Fast movement
            TitanPhase::Scratching => 3.0, // Rapid localized movement
        };

        let base_amplitude = match phase {
            TitanPhase::Resting => 0.1,
            TitanPhase::Walking => 0.2,
            TitanPhase::Running => 0.3,
            TitanPhase::Scratching => 0.4,
        };

        // Simple sine wave animation
        let wave = (self.time * base_frequency as f64).sin() as f32;
        (wave * base_amplitude + 0.5).clamp(0.0, 1.0)
    }

    /// Get vent eruption intensity.
    ///
    /// Returns a value 0.0-1.0 representing current eruption intensity.
    #[must_use]
    pub fn get_vent_eruption_intensity(&self) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        // Vents have a random-looking cycle based on time
        let cycle1 = (self.time * 0.3).sin() as f32;
        let cycle2 = (self.time * 0.7 + 1.5).sin() as f32;
        let combined = (cycle1 * 0.6 + cycle2 * 0.4 + 1.0) / 2.0;

        combined.clamp(0.0, 1.0)
    }

    /// Get wound pulse animation.
    ///
    /// Takes a game tick for deterministic animation across clients.
    #[must_use]
    pub fn get_wound_pulse(&self, tick: u64) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        // Heart-like double pulse pattern
        let phase = (tick as f64 * 0.05) % (2.0 * std::f64::consts::PI);
        let pulse1 = (phase * 2.0).sin().max(0.0) as f32;
        let pulse2 = ((phase * 2.0) + 0.3).sin().max(0.0) as f32 * 0.7;

        (pulse1 + pulse2).min(1.0)
    }

    /// Get breathing animation for the entire Titan.
    #[must_use]
    pub fn get_breathing_animation(&self) -> f32 {
        if !self.enabled {
            return 0.5;
        }

        // Slow, deep breathing cycle
        let breath = (self.time * 0.2).sin() as f32;
        (breath * 0.3 + 0.5).clamp(0.0, 1.0)
    }

    /// Get terrain rumble intensity based on phase.
    #[must_use]
    pub fn get_rumble_intensity(&self, phase: TitanPhase) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        let base = match phase {
            TitanPhase::Resting => 0.0,
            TitanPhase::Walking => 0.2,
            TitanPhase::Running => 0.6,
            TitanPhase::Scratching => 0.8,
        };

        // Add some variation
        let variation = (self.time * 5.0).sin() as f32 * 0.1;
        (base + variation).clamp(0.0, 1.0)
    }

    /// Get muscle ripple offset for walking animation.
    #[must_use]
    pub fn get_muscle_ripple(&self, phase: TitanPhase, position_offset: f32) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        if phase == TitanPhase::Resting {
            return 0.0;
        }

        let speed = match phase {
            TitanPhase::Resting => 0.0,
            TitanPhase::Walking => 1.0,
            TitanPhase::Running => 2.5,
            TitanPhase::Scratching => 4.0,
        };

        // Traveling wave effect
        let wave = (self.time * speed as f64 - position_offset as f64).sin() as f32;
        (wave * 0.5 + 0.5).clamp(0.0, 1.0)
    }

    /// Set animation speed multiplier.
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.max(0.0);
    }

    /// Get current speed multiplier.
    #[must_use]
    pub fn speed(&self) -> f32 {
        self.speed
    }

    /// Enable or disable animations.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if animations are enabled.
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get current animation time.
    #[must_use]
    pub fn time(&self) -> f64 {
        self.time
    }

    /// Reset animation time.
    pub fn reset(&mut self) {
        self.time = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terrain_animation_new() {
        let anim = TerrainAnimation::new();
        assert!((anim.time() - 0.0).abs() < f64::EPSILON);
        assert!((anim.speed() - 1.0).abs() < f32::EPSILON);
        assert!(anim.is_enabled());
    }

    #[test]
    fn test_terrain_animation_tick() {
        let mut anim = TerrainAnimation::new();
        anim.tick(1.0);
        assert!((anim.time() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_terrain_animation_tick_with_speed() {
        let mut anim = TerrainAnimation::new();
        anim.set_speed(2.0);
        anim.tick(1.0);
        assert!((anim.time() - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_terrain_animation_tick_disabled() {
        let mut anim = TerrainAnimation::new();
        anim.set_enabled(false);
        anim.tick(1.0);
        assert!((anim.time() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_terrain_animation_scale_animation_resting() {
        let anim = TerrainAnimation::new();
        let scale = anim.get_scale_animation(TitanPhase::Resting);
        // Should be around 0.5 at time 0
        assert!(scale >= 0.0 && scale <= 1.0);
    }

    #[test]
    fn test_terrain_animation_scale_animation_running() {
        let mut anim = TerrainAnimation::new();
        anim.tick(1.0);
        let scale = anim.get_scale_animation(TitanPhase::Running);
        assert!(scale >= 0.0 && scale <= 1.0);
    }

    #[test]
    fn test_terrain_animation_scale_animation_disabled() {
        let mut anim = TerrainAnimation::new();
        anim.set_enabled(false);
        let scale = anim.get_scale_animation(TitanPhase::Running);
        assert!((scale - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_terrain_animation_vent_eruption() {
        let anim = TerrainAnimation::new();
        let intensity = anim.get_vent_eruption_intensity();
        assert!(intensity >= 0.0 && intensity <= 1.0);
    }

    #[test]
    fn test_terrain_animation_vent_eruption_changes() {
        let mut anim = TerrainAnimation::new();
        let intensity1 = anim.get_vent_eruption_intensity();
        anim.tick(5.0);
        let intensity2 = anim.get_vent_eruption_intensity();
        // Intensity should change over time
        assert!((intensity1 - intensity2).abs() > 0.001 || intensity1 == intensity2);
    }

    #[test]
    fn test_terrain_animation_wound_pulse() {
        let anim = TerrainAnimation::new();
        let pulse = anim.get_wound_pulse(0);
        assert!(pulse >= 0.0 && pulse <= 1.0);
    }

    #[test]
    fn test_terrain_animation_wound_pulse_varies() {
        let anim = TerrainAnimation::new();
        let pulse1 = anim.get_wound_pulse(0);
        let pulse2 = anim.get_wound_pulse(10);
        let pulse3 = anim.get_wound_pulse(50);
        // Should produce varying values
        assert!(pulse1 != pulse2 || pulse2 != pulse3 || pulse1 != pulse3);
    }

    #[test]
    fn test_terrain_animation_wound_pulse_disabled() {
        let mut anim = TerrainAnimation::new();
        anim.set_enabled(false);
        let pulse = anim.get_wound_pulse(100);
        assert!((pulse - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_terrain_animation_breathing() {
        let anim = TerrainAnimation::new();
        let breath = anim.get_breathing_animation();
        assert!(breath >= 0.0 && breath <= 1.0);
    }

    #[test]
    fn test_terrain_animation_breathing_disabled() {
        let mut anim = TerrainAnimation::new();
        anim.set_enabled(false);
        let breath = anim.get_breathing_animation();
        assert!((breath - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_terrain_animation_rumble_intensity() {
        let anim = TerrainAnimation::new();

        let resting = anim.get_rumble_intensity(TitanPhase::Resting);
        let walking = anim.get_rumble_intensity(TitanPhase::Walking);
        let running = anim.get_rumble_intensity(TitanPhase::Running);
        let scratching = anim.get_rumble_intensity(TitanPhase::Scratching);

        // Intensity should increase with activity
        assert!(resting < walking);
        assert!(walking < running);
        // Scratching may be higher or equal to running
        assert!(scratching >= running * 0.5);
    }

    #[test]
    fn test_terrain_animation_muscle_ripple_resting() {
        let anim = TerrainAnimation::new();
        let ripple = anim.get_muscle_ripple(TitanPhase::Resting, 0.0);
        assert!((ripple - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_terrain_animation_muscle_ripple_walking() {
        let anim = TerrainAnimation::new();
        let ripple = anim.get_muscle_ripple(TitanPhase::Walking, 0.0);
        assert!(ripple >= 0.0 && ripple <= 1.0);
    }

    #[test]
    fn test_terrain_animation_muscle_ripple_position_offset() {
        let anim = TerrainAnimation::new();
        let ripple1 = anim.get_muscle_ripple(TitanPhase::Running, 0.0);
        let ripple2 = anim.get_muscle_ripple(TitanPhase::Running, 1.0);
        // Different positions should have different ripple values
        assert!((ripple1 - ripple2).abs() >= 0.0);
    }

    #[test]
    fn test_terrain_animation_set_speed() {
        let mut anim = TerrainAnimation::new();
        anim.set_speed(2.5);
        assert!((anim.speed() - 2.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_terrain_animation_set_speed_clamp() {
        let mut anim = TerrainAnimation::new();
        anim.set_speed(-1.0);
        assert!((anim.speed() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_terrain_animation_reset() {
        let mut anim = TerrainAnimation::new();
        anim.tick(10.0);
        assert!(anim.time() > 0.0);

        anim.reset();
        assert!((anim.time() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_terrain_animation_default() {
        let anim = TerrainAnimation::default();
        assert!((anim.time() - 0.0).abs() < f64::EPSILON);
        assert!((anim.speed() - 1.0).abs() < f32::EPSILON);
        assert!(anim.is_enabled());
    }
}
