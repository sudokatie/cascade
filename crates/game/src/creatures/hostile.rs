//! Hostile creature types for Titan survival.
//!
//! Hostile creatures that infest the Titan's body and attack players.

use serde::{Deserialize, Serialize};

use crate::titan::{TitanMood, TitanZone};

/// Time of day preference for creature spawning.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimeOfDay {
    /// Spawns during day.
    Day,
    /// Spawns during night.
    Night,
    /// Spawns at any time.
    Any,
}

/// Mood requirement for hostile creature spawning.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MoodRequirement {
    /// Spawns in any mood.
    Any,
    /// Spawns only when Agitated or Enraged.
    AgitatedOrEnraged,
    /// Spawns only when Enraged.
    EnragedOnly,
}

impl MoodRequirement {
    /// Check if the given mood satisfies this requirement.
    #[must_use]
    pub fn is_satisfied(&self, mood: TitanMood) -> bool {
        match self {
            MoodRequirement::Any => true,
            MoodRequirement::AgitatedOrEnraged => {
                matches!(mood, TitanMood::Agitated | TitanMood::Enraged)
            }
            MoodRequirement::EnragedOnly => matches!(mood, TitanMood::Enraged),
        }
    }
}

/// Spawn condition for hostile creatures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HostileSpawnCondition {
    /// Zone where the creature spawns.
    pub zone: TitanZone,
    /// Mood requirement for spawning.
    pub mood: MoodRequirement,
}

/// Special ability info with name and damage multiplier.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpecialAbilityInfo {
    /// Name of the ability.
    pub name: &'static str,
    /// Damage multiplier for the ability.
    pub damage_multiplier: f32,
}

/// Types of hostile creatures on the Titan.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HostileType {
    /// Small tick that feeds on scales and drains Titan HP.
    ScaleTick,
    /// Borer that tunnels through shell, creating unstable ground.
    ShellBorer,
    /// Leech that drains blood and causes DOT to nearby players.
    BloodLeech,
    /// Wasp that affects neural tissue, swarms near nodes.
    NeuralWasp,
    /// Large crawler in the mouth that can devour low-balance players.
    MouthCrawler,
}

impl HostileType {
    /// Get base HP for this creature type.
    #[must_use]
    pub fn base_hp(&self) -> u32 {
        match self {
            HostileType::ScaleTick => 40,
            HostileType::ShellBorer => 80,
            HostileType::BloodLeech => 30,
            HostileType::NeuralWasp => 50,
            HostileType::MouthCrawler => 200,
        }
    }

    /// Get base damage for this creature type.
    #[must_use]
    pub fn base_damage(&self) -> u32 {
        match self {
            HostileType::ScaleTick => 5,
            HostileType::ShellBorer => 10,
            HostileType::BloodLeech => 8,
            HostileType::NeuralWasp => 15,
            HostileType::MouthCrawler => 25,
        }
    }

    /// Get base movement speed for this creature type.
    #[must_use]
    pub fn base_speed(&self) -> f32 {
        match self {
            HostileType::ScaleTick => 0.5,
            HostileType::ShellBorer => 0.8,
            HostileType::BloodLeech => 1.2,
            HostileType::NeuralWasp => 1.5,
            HostileType::MouthCrawler => 1.0,
        }
    }

    /// Get the special ability name for this creature type.
    #[must_use]
    pub fn special_ability_name(&self) -> &'static str {
        match self {
            HostileType::ScaleTick => "drain",
            HostileType::ShellBorer => "tunnel",
            HostileType::BloodLeech => "bleed",
            HostileType::NeuralWasp => "swarm",
            HostileType::MouthCrawler => "devour",
        }
    }

    /// Get the special ability info including name and damage multiplier.
    #[must_use]
    pub fn special_ability(&self) -> SpecialAbilityInfo {
        match self {
            HostileType::ScaleTick => SpecialAbilityInfo {
                name: "drain",
                damage_multiplier: 1.0,
            },
            HostileType::ShellBorer => SpecialAbilityInfo {
                name: "tunnel",
                damage_multiplier: 1.5,
            },
            HostileType::BloodLeech => SpecialAbilityInfo {
                name: "bleed",
                damage_multiplier: 0.5,
            },
            HostileType::NeuralWasp => SpecialAbilityInfo {
                name: "swarm",
                damage_multiplier: 3.0,
            },
            HostileType::MouthCrawler => SpecialAbilityInfo {
                name: "devour",
                damage_multiplier: 40.0,
            },
        }
    }

    /// Get the spawn condition for this creature type.
    #[must_use]
    pub fn spawn_condition(&self) -> HostileSpawnCondition {
        match self {
            HostileType::ScaleTick => HostileSpawnCondition {
                zone: TitanZone::ScaleValley,
                mood: MoodRequirement::Any,
            },
            HostileType::ShellBorer => HostileSpawnCondition {
                zone: TitanZone::ShellRidge,
                mood: MoodRequirement::AgitatedOrEnraged,
            },
            HostileType::BloodLeech => HostileSpawnCondition {
                zone: TitanZone::WoundSite,
                mood: MoodRequirement::Any,
            },
            HostileType::NeuralWasp => HostileSpawnCondition {
                zone: TitanZone::NeuralNode,
                mood: MoodRequirement::EnragedOnly,
            },
            HostileType::MouthCrawler => HostileSpawnCondition {
                zone: TitanZone::BreathingVent,
                mood: MoodRequirement::EnragedOnly,
            },
        }
    }

    /// Get display name for this creature type.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            HostileType::ScaleTick => "Scale Tick",
            HostileType::ShellBorer => "Shell Borer",
            HostileType::BloodLeech => "Blood Leech",
            HostileType::NeuralWasp => "Neural Wasp",
            HostileType::MouthCrawler => "Mouth Crawler",
        }
    }
}

/// Result of using a special ability.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AbilityResult {
    /// Whether the ability was successfully used.
    pub success: bool,
    /// Damage dealt by the ability.
    pub damage: u32,
    /// Area of effect radius (0 for single target).
    pub area_radius: f32,
    /// Description of the effect.
    pub effect: String,
}

impl AbilityResult {
    /// Create a new ability result.
    #[must_use]
    pub fn new(success: bool, damage: u32, area_radius: f32, effect: String) -> Self {
        Self {
            success,
            damage,
            area_radius,
            effect,
        }
    }

    /// Create a failed ability result.
    #[must_use]
    pub fn failed() -> Self {
        Self {
            success: false,
            damage: 0,
            area_radius: 0.0,
            effect: String::new(),
        }
    }
}

/// A hostile creature instance.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HostileCreature {
    /// Type of this creature.
    creature_type: HostileType,
    /// Current HP.
    hp: u32,
    /// Maximum HP.
    max_hp: u32,
    /// Attack damage.
    damage: u32,
    /// Movement speed.
    speed: f32,
    /// Special ability name.
    special_ability: String,
    /// Whether the creature is active (not stunned/disabled).
    active: bool,
}

impl HostileCreature {
    /// Create a new hostile creature of the given type.
    #[must_use]
    pub fn new(creature_type: HostileType) -> Self {
        let max_hp = creature_type.base_hp();
        Self {
            creature_type,
            hp: max_hp,
            max_hp,
            damage: creature_type.base_damage(),
            speed: creature_type.base_speed(),
            special_ability: creature_type.special_ability_name().to_string(),
            active: true,
        }
    }

    /// Get the creature type.
    #[must_use]
    pub fn creature_type(&self) -> HostileType {
        self.creature_type
    }

    /// Get current HP.
    #[must_use]
    pub fn hp(&self) -> u32 {
        self.hp
    }

    /// Get maximum HP.
    #[must_use]
    pub fn max_hp(&self) -> u32 {
        self.max_hp
    }

    /// Get attack damage.
    #[must_use]
    pub fn damage(&self) -> u32 {
        self.damage
    }

    /// Get movement speed.
    #[must_use]
    pub fn speed(&self) -> f32 {
        self.speed
    }

    /// Get the special ability name.
    #[must_use]
    pub fn special_ability(&self) -> &str {
        &self.special_ability
    }

    /// Check if the creature is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Set active state.
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /// Check if the creature is alive.
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Apply damage to the creature.
    ///
    /// Returns the actual damage dealt.
    pub fn take_damage(&mut self, amount: u32) -> u32 {
        let actual = amount.min(self.hp);
        self.hp = self.hp.saturating_sub(amount);
        actual
    }

    /// Perform a basic attack.
    ///
    /// Returns the damage dealt (0 if inactive or dead).
    #[must_use]
    pub fn attack(&self) -> u32 {
        if self.is_alive() && self.active {
            self.damage
        } else {
            0
        }
    }

    /// Use the creature's special ability.
    #[must_use]
    pub fn use_ability(&self) -> AbilityResult {
        if !self.is_alive() || !self.active {
            return AbilityResult::failed();
        }

        match self.creature_type {
            HostileType::ScaleTick => AbilityResult::new(
                true,
                self.damage,
                0.0,
                "Drains HP from the Titan itself".to_string(),
            ),
            HostileType::ShellBorer => AbilityResult::new(
                true,
                self.damage + 5,
                3.0,
                "Tunnels through shell, creating unstable ground".to_string(),
            ),
            HostileType::BloodLeech => AbilityResult::new(
                true,
                self.damage / 2,
                4.0,
                "Causes bleeding DOT to nearby players".to_string(),
            ),
            HostileType::NeuralWasp => AbilityResult::new(
                true,
                self.damage * 3,
                2.0,
                "Swarms with 3x damage near neural nodes".to_string(),
            ),
            HostileType::MouthCrawler => AbilityResult::new(
                true,
                999,
                0.0,
                "Devours players with low balance instantly".to_string(),
            ),
        }
    }

    /// Heal the creature.
    pub fn heal(&mut self, amount: u32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hostile_type_scale_tick_stats() {
        assert_eq!(HostileType::ScaleTick.base_hp(), 40);
        assert_eq!(HostileType::ScaleTick.base_damage(), 5);
        assert!((HostileType::ScaleTick.base_speed() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hostile_type_shell_borer_stats() {
        assert_eq!(HostileType::ShellBorer.base_hp(), 80);
        assert_eq!(HostileType::ShellBorer.base_damage(), 10);
        assert!((HostileType::ShellBorer.base_speed() - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hostile_type_blood_leech_stats() {
        assert_eq!(HostileType::BloodLeech.base_hp(), 30);
        assert_eq!(HostileType::BloodLeech.base_damage(), 8);
        assert!((HostileType::BloodLeech.base_speed() - 1.2).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hostile_type_neural_wasp_stats() {
        assert_eq!(HostileType::NeuralWasp.base_hp(), 50);
        assert_eq!(HostileType::NeuralWasp.base_damage(), 15);
        assert!((HostileType::NeuralWasp.base_speed() - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hostile_type_mouth_crawler_stats() {
        assert_eq!(HostileType::MouthCrawler.base_hp(), 200);
        assert_eq!(HostileType::MouthCrawler.base_damage(), 25);
        assert!((HostileType::MouthCrawler.base_speed() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hostile_type_special_ability_names() {
        assert_eq!(HostileType::ScaleTick.special_ability_name(), "drain");
        assert_eq!(HostileType::ShellBorer.special_ability_name(), "tunnel");
        assert_eq!(HostileType::BloodLeech.special_ability_name(), "bleed");
        assert_eq!(HostileType::NeuralWasp.special_ability_name(), "swarm");
        assert_eq!(HostileType::MouthCrawler.special_ability_name(), "devour");
    }

    #[test]
    fn test_hostile_type_special_ability_info() {
        let ability = HostileType::ScaleTick.special_ability();
        assert_eq!(ability.name, "drain");
        assert!((ability.damage_multiplier - 1.0).abs() < f32::EPSILON);

        let ability = HostileType::NeuralWasp.special_ability();
        assert_eq!(ability.name, "swarm");
        assert!((ability.damage_multiplier - 3.0).abs() < f32::EPSILON);

        let ability = HostileType::MouthCrawler.special_ability();
        assert_eq!(ability.name, "devour");
        assert!((ability.damage_multiplier - 40.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hostile_type_spawn_condition_scale_tick() {
        let cond = HostileType::ScaleTick.spawn_condition();
        assert_eq!(cond.zone, TitanZone::ScaleValley);
        assert_eq!(cond.mood, MoodRequirement::Any);
    }

    #[test]
    fn test_hostile_type_spawn_condition_shell_borer() {
        let cond = HostileType::ShellBorer.spawn_condition();
        assert_eq!(cond.zone, TitanZone::ShellRidge);
        assert_eq!(cond.mood, MoodRequirement::AgitatedOrEnraged);
    }

    #[test]
    fn test_hostile_type_spawn_condition_blood_leech() {
        let cond = HostileType::BloodLeech.spawn_condition();
        assert_eq!(cond.zone, TitanZone::WoundSite);
        assert_eq!(cond.mood, MoodRequirement::Any);
    }

    #[test]
    fn test_hostile_type_spawn_condition_neural_wasp() {
        let cond = HostileType::NeuralWasp.spawn_condition();
        assert_eq!(cond.zone, TitanZone::NeuralNode);
        assert_eq!(cond.mood, MoodRequirement::EnragedOnly);
    }

    #[test]
    fn test_hostile_type_spawn_condition_mouth_crawler() {
        let cond = HostileType::MouthCrawler.spawn_condition();
        assert_eq!(cond.zone, TitanZone::BreathingVent);
        assert_eq!(cond.mood, MoodRequirement::EnragedOnly);
    }

    #[test]
    fn test_mood_requirement_any() {
        let req = MoodRequirement::Any;
        assert!(req.is_satisfied(TitanMood::Calm));
        assert!(req.is_satisfied(TitanMood::Agitated));
        assert!(req.is_satisfied(TitanMood::Enraged));
    }

    #[test]
    fn test_mood_requirement_agitated_or_enraged() {
        let req = MoodRequirement::AgitatedOrEnraged;
        assert!(!req.is_satisfied(TitanMood::Calm));
        assert!(req.is_satisfied(TitanMood::Agitated));
        assert!(req.is_satisfied(TitanMood::Enraged));
    }

    #[test]
    fn test_mood_requirement_enraged_only() {
        let req = MoodRequirement::EnragedOnly;
        assert!(!req.is_satisfied(TitanMood::Calm));
        assert!(!req.is_satisfied(TitanMood::Agitated));
        assert!(req.is_satisfied(TitanMood::Enraged));
    }

    #[test]
    fn test_hostile_type_display_names() {
        assert_eq!(HostileType::ScaleTick.display_name(), "Scale Tick");
        assert_eq!(HostileType::ShellBorer.display_name(), "Shell Borer");
        assert_eq!(HostileType::BloodLeech.display_name(), "Blood Leech");
        assert_eq!(HostileType::NeuralWasp.display_name(), "Neural Wasp");
        assert_eq!(HostileType::MouthCrawler.display_name(), "Mouth Crawler");
    }

    #[test]
    fn test_hostile_creature_new() {
        let creature = HostileCreature::new(HostileType::ShellBorer);

        assert_eq!(creature.creature_type(), HostileType::ShellBorer);
        assert_eq!(creature.hp(), 80);
        assert_eq!(creature.max_hp(), 80);
        assert_eq!(creature.damage(), 10);
        assert!((creature.speed() - 0.8).abs() < f32::EPSILON);
        assert!(creature.is_alive());
        assert!(creature.is_active());
    }

    #[test]
    fn test_hostile_creature_take_damage() {
        let mut creature = HostileCreature::new(HostileType::BloodLeech);
        assert_eq!(creature.hp(), 30);

        let dealt = creature.take_damage(10);
        assert_eq!(dealt, 10);
        assert_eq!(creature.hp(), 20);
        assert!(creature.is_alive());
    }

    #[test]
    fn test_hostile_creature_death() {
        let mut creature = HostileCreature::new(HostileType::ScaleTick);
        creature.take_damage(50);

        assert_eq!(creature.hp(), 0);
        assert!(!creature.is_alive());
    }

    #[test]
    fn test_hostile_creature_overkill() {
        let mut creature = HostileCreature::new(HostileType::ScaleTick);
        let dealt = creature.take_damage(1000);

        assert_eq!(dealt, 40);
        assert_eq!(creature.hp(), 0);
    }

    #[test]
    fn test_hostile_creature_attack() {
        let creature = HostileCreature::new(HostileType::NeuralWasp);
        assert_eq!(creature.attack(), 15);
    }

    #[test]
    fn test_hostile_creature_attack_when_dead() {
        let mut creature = HostileCreature::new(HostileType::NeuralWasp);
        creature.take_damage(100);
        assert_eq!(creature.attack(), 0);
    }

    #[test]
    fn test_hostile_creature_attack_when_inactive() {
        let mut creature = HostileCreature::new(HostileType::NeuralWasp);
        creature.set_active(false);
        assert_eq!(creature.attack(), 0);
    }

    #[test]
    fn test_hostile_creature_use_ability_drain() {
        let creature = HostileCreature::new(HostileType::ScaleTick);
        let result = creature.use_ability();

        assert!(result.success);
        assert_eq!(result.damage, 5);
        assert!(result.effect.contains("Drains"));
    }

    #[test]
    fn test_hostile_creature_use_ability_tunnel() {
        let creature = HostileCreature::new(HostileType::ShellBorer);
        let result = creature.use_ability();

        assert!(result.success);
        assert_eq!(result.damage, 15);
        assert!(result.area_radius > 0.0);
    }

    #[test]
    fn test_hostile_creature_use_ability_swarm() {
        let creature = HostileCreature::new(HostileType::NeuralWasp);
        let result = creature.use_ability();

        assert!(result.success);
        assert_eq!(result.damage, 45); // 15 * 3
    }

    #[test]
    fn test_hostile_creature_use_ability_devour() {
        let creature = HostileCreature::new(HostileType::MouthCrawler);
        let result = creature.use_ability();

        assert!(result.success);
        assert_eq!(result.damage, 999);
        assert!(result.effect.contains("Devours"));
    }

    #[test]
    fn test_hostile_creature_ability_when_dead() {
        let mut creature = HostileCreature::new(HostileType::MouthCrawler);
        creature.take_damage(300);
        let result = creature.use_ability();

        assert!(!result.success);
    }

    #[test]
    fn test_hostile_creature_heal() {
        let mut creature = HostileCreature::new(HostileType::MouthCrawler);
        creature.take_damage(100);
        assert_eq!(creature.hp(), 100);

        creature.heal(50);
        assert_eq!(creature.hp(), 150);

        creature.heal(100);
        assert_eq!(creature.hp(), 200);
    }

    #[test]
    fn test_ability_result_failed() {
        let result = AbilityResult::failed();
        assert!(!result.success);
        assert_eq!(result.damage, 0);
    }

    #[test]
    fn test_blood_leech_bleed_ability() {
        let creature = HostileCreature::new(HostileType::BloodLeech);
        let result = creature.use_ability();

        assert!(result.success);
        assert_eq!(result.damage, 4); // 8 / 2
        assert!(result.area_radius > 0.0);
        assert!(result.effect.contains("bleeding"));
    }
}
