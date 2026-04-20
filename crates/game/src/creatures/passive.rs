//! Passive creature types for Titan survival.
//!
//! Passive creatures that live on the Titan and provide resources.

use serde::{Deserialize, Serialize};

use super::hostile::TimeOfDay;
use crate::titan::TitanZone;

/// Spawn condition for passive creatures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PassiveSpawnCondition {
    /// Zone where the creature spawns.
    pub zone: TitanZone,
    /// Time of day preference.
    pub time: TimeOfDay,
}

/// Types of passive creatures on the Titan.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PassiveType {
    /// Moth that cleans scales, found on scale surfaces.
    ScaleMoth,
    /// Shrimp found near thermal vents.
    VentShrimp,
    /// Fish found in wound blood pools.
    BloodFish,
    /// Butterfly found near neural nodes.
    NeuralButterfly,
    /// Crab found on shell surfaces.
    ShellCrab,
}

impl PassiveType {
    /// Get base HP for this creature type.
    #[must_use]
    pub fn base_hp(&self) -> u32 {
        match self {
            PassiveType::ScaleMoth => 8,
            PassiveType::VentShrimp => 12,
            PassiveType::BloodFish => 10,
            PassiveType::NeuralButterfly => 6,
            PassiveType::ShellCrab => 20,
        }
    }

    /// Get the drop item for this creature type.
    #[must_use]
    pub fn drop_item(&self) -> &'static str {
        match self {
            PassiveType::ScaleMoth => "scale_dust",
            PassiveType::VentShrimp => "thermal_shell",
            PassiveType::BloodFish => "coagulant",
            PassiveType::NeuralButterfly => "neural_dust",
            PassiveType::ShellCrab => "shell_fragment",
        }
    }

    /// Get the special trait for this creature type.
    #[must_use]
    pub fn special_trait(&self) -> &'static str {
        match self {
            PassiveType::ScaleMoth => "cleans_scales",
            PassiveType::VentShrimp => "heat_resistant",
            PassiveType::BloodFish => "wound_dweller",
            PassiveType::NeuralButterfly => "neural_affinity",
            PassiveType::ShellCrab => "shell_climber",
        }
    }

    /// Get display name for this creature type.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            PassiveType::ScaleMoth => "Scale Moth",
            PassiveType::VentShrimp => "Vent Shrimp",
            PassiveType::BloodFish => "Blood Fish",
            PassiveType::NeuralButterfly => "Neural Butterfly",
            PassiveType::ShellCrab => "Shell Crab",
        }
    }

    /// Get the habitat location for this creature.
    #[must_use]
    pub fn habitat(&self) -> &'static str {
        match self {
            PassiveType::ScaleMoth => "scale_surface",
            PassiveType::VentShrimp => "thermal_vents",
            PassiveType::BloodFish => "wound_pools",
            PassiveType::NeuralButterfly => "neural_nodes",
            PassiveType::ShellCrab => "shell_surface",
        }
    }

    /// Get the spawn condition for this creature type.
    #[must_use]
    pub fn spawn_condition(&self) -> PassiveSpawnCondition {
        match self {
            PassiveType::ScaleMoth => PassiveSpawnCondition {
                zone: TitanZone::ShellRidge,
                time: TimeOfDay::Night,
            },
            PassiveType::VentShrimp => PassiveSpawnCondition {
                zone: TitanZone::BreathingVent,
                time: TimeOfDay::Any,
            },
            PassiveType::BloodFish => PassiveSpawnCondition {
                zone: TitanZone::WoundSite,
                time: TimeOfDay::Any,
            },
            PassiveType::NeuralButterfly => PassiveSpawnCondition {
                zone: TitanZone::NeuralNode,
                time: TimeOfDay::Day,
            },
            PassiveType::ShellCrab => PassiveSpawnCondition {
                zone: TitanZone::ShellRidge,
                time: TimeOfDay::Any,
            },
        }
    }
}

/// A passive creature instance.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PassiveCreature {
    /// Type of this creature.
    creature_type: PassiveType,
    /// Current HP.
    hp: u32,
    /// Maximum HP.
    max_hp: u32,
    /// Item dropped when caught/killed.
    drop_item: String,
    /// Special trait identifier.
    special_trait: String,
}

impl PassiveCreature {
    /// Create a new passive creature of the given type.
    #[must_use]
    pub fn new(creature_type: PassiveType) -> Self {
        let max_hp = creature_type.base_hp();
        Self {
            creature_type,
            hp: max_hp,
            max_hp,
            drop_item: creature_type.drop_item().to_string(),
            special_trait: creature_type.special_trait().to_string(),
        }
    }

    /// Get the creature type.
    #[must_use]
    pub fn creature_type(&self) -> PassiveType {
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

    /// Get the drop item name.
    #[must_use]
    pub fn drop_item(&self) -> &str {
        &self.drop_item
    }

    /// Get the special trait name.
    #[must_use]
    pub fn special_trait(&self) -> &str {
        &self.special_trait
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

    /// Attempt to catch the creature.
    ///
    /// Returns the drop item if successful (creature dies), None otherwise.
    pub fn on_catch(&mut self) -> Option<String> {
        if self.is_alive() {
            self.hp = 0;
            Some(self.drop_item.clone())
        } else {
            None
        }
    }

    /// Check if this creature has a specific trait.
    #[must_use]
    pub fn has_trait(&self, trait_name: &str) -> bool {
        self.special_trait == trait_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passive_type_scale_moth_stats() {
        assert_eq!(PassiveType::ScaleMoth.base_hp(), 8);
        assert_eq!(PassiveType::ScaleMoth.drop_item(), "scale_dust");
        assert_eq!(PassiveType::ScaleMoth.special_trait(), "cleans_scales");
    }

    #[test]
    fn test_passive_type_vent_shrimp_stats() {
        assert_eq!(PassiveType::VentShrimp.base_hp(), 12);
        assert_eq!(PassiveType::VentShrimp.drop_item(), "thermal_shell");
        assert_eq!(PassiveType::VentShrimp.special_trait(), "heat_resistant");
    }

    #[test]
    fn test_passive_type_blood_fish_stats() {
        assert_eq!(PassiveType::BloodFish.base_hp(), 10);
        assert_eq!(PassiveType::BloodFish.drop_item(), "coagulant");
        assert_eq!(PassiveType::BloodFish.special_trait(), "wound_dweller");
    }

    #[test]
    fn test_passive_type_neural_butterfly_stats() {
        assert_eq!(PassiveType::NeuralButterfly.base_hp(), 6);
        assert_eq!(PassiveType::NeuralButterfly.drop_item(), "neural_dust");
        assert_eq!(PassiveType::NeuralButterfly.special_trait(), "neural_affinity");
    }

    #[test]
    fn test_passive_type_shell_crab_stats() {
        assert_eq!(PassiveType::ShellCrab.base_hp(), 20);
        assert_eq!(PassiveType::ShellCrab.drop_item(), "shell_fragment");
        assert_eq!(PassiveType::ShellCrab.special_trait(), "shell_climber");
    }

    #[test]
    fn test_passive_type_habitats() {
        assert_eq!(PassiveType::ScaleMoth.habitat(), "scale_surface");
        assert_eq!(PassiveType::VentShrimp.habitat(), "thermal_vents");
        assert_eq!(PassiveType::BloodFish.habitat(), "wound_pools");
        assert_eq!(PassiveType::NeuralButterfly.habitat(), "neural_nodes");
        assert_eq!(PassiveType::ShellCrab.habitat(), "shell_surface");
    }

    #[test]
    fn test_passive_type_display_names() {
        assert_eq!(PassiveType::ScaleMoth.display_name(), "Scale Moth");
        assert_eq!(PassiveType::VentShrimp.display_name(), "Vent Shrimp");
        assert_eq!(PassiveType::BloodFish.display_name(), "Blood Fish");
        assert_eq!(PassiveType::NeuralButterfly.display_name(), "Neural Butterfly");
        assert_eq!(PassiveType::ShellCrab.display_name(), "Shell Crab");
    }

    #[test]
    fn test_passive_creature_new() {
        let creature = PassiveCreature::new(PassiveType::ShellCrab);

        assert_eq!(creature.creature_type(), PassiveType::ShellCrab);
        assert_eq!(creature.hp(), 20);
        assert_eq!(creature.max_hp(), 20);
        assert_eq!(creature.drop_item(), "shell_fragment");
        assert_eq!(creature.special_trait(), "shell_climber");
        assert!(creature.is_alive());
    }

    #[test]
    fn test_passive_creature_take_damage() {
        let mut creature = PassiveCreature::new(PassiveType::VentShrimp);
        assert_eq!(creature.hp(), 12);

        let dealt = creature.take_damage(5);
        assert_eq!(dealt, 5);
        assert_eq!(creature.hp(), 7);
        assert!(creature.is_alive());
    }

    #[test]
    fn test_passive_creature_death() {
        let mut creature = PassiveCreature::new(PassiveType::NeuralButterfly);
        creature.take_damage(10);

        assert_eq!(creature.hp(), 0);
        assert!(!creature.is_alive());
    }

    #[test]
    fn test_passive_creature_overkill() {
        let mut creature = PassiveCreature::new(PassiveType::ScaleMoth);
        let dealt = creature.take_damage(100);

        assert_eq!(dealt, 8);
        assert_eq!(creature.hp(), 0);
    }

    #[test]
    fn test_passive_creature_on_catch() {
        let mut creature = PassiveCreature::new(PassiveType::BloodFish);
        assert!(creature.is_alive());

        let drop = creature.on_catch();
        assert!(drop.is_some());
        assert_eq!(drop.unwrap(), "coagulant");
        assert!(!creature.is_alive());
    }

    #[test]
    fn test_passive_creature_on_catch_when_dead() {
        let mut creature = PassiveCreature::new(PassiveType::BloodFish);
        creature.take_damage(100);

        let drop = creature.on_catch();
        assert!(drop.is_none());
    }

    #[test]
    fn test_passive_creature_has_trait() {
        let creature = PassiveCreature::new(PassiveType::VentShrimp);

        assert!(creature.has_trait("heat_resistant"));
        assert!(!creature.has_trait("cleans_scales"));
    }

    #[test]
    fn test_all_passive_types_have_unique_drops() {
        let types = [
            PassiveType::ScaleMoth,
            PassiveType::VentShrimp,
            PassiveType::BloodFish,
            PassiveType::NeuralButterfly,
            PassiveType::ShellCrab,
        ];

        let drops: Vec<_> = types.iter().map(|t| t.drop_item()).collect();
        for (i, drop) in drops.iter().enumerate() {
            for (j, other) in drops.iter().enumerate() {
                if i != j {
                    assert_ne!(drop, other, "Duplicate drop items found");
                }
            }
        }
    }

    #[test]
    fn test_passive_spawn_condition_scale_moth() {
        let cond = PassiveType::ScaleMoth.spawn_condition();
        assert_eq!(cond.zone, TitanZone::ShellRidge);
        assert_eq!(cond.time, TimeOfDay::Night);
    }

    #[test]
    fn test_passive_spawn_condition_vent_shrimp() {
        let cond = PassiveType::VentShrimp.spawn_condition();
        assert_eq!(cond.zone, TitanZone::BreathingVent);
        assert_eq!(cond.time, TimeOfDay::Any);
    }

    #[test]
    fn test_passive_spawn_condition_blood_fish() {
        let cond = PassiveType::BloodFish.spawn_condition();
        assert_eq!(cond.zone, TitanZone::WoundSite);
        assert_eq!(cond.time, TimeOfDay::Any);
    }

    #[test]
    fn test_passive_spawn_condition_neural_butterfly() {
        let cond = PassiveType::NeuralButterfly.spawn_condition();
        assert_eq!(cond.zone, TitanZone::NeuralNode);
        assert_eq!(cond.time, TimeOfDay::Day);
    }

    #[test]
    fn test_passive_spawn_condition_shell_crab() {
        let cond = PassiveType::ShellCrab.spawn_condition();
        assert_eq!(cond.zone, TitanZone::ShellRidge);
        assert_eq!(cond.time, TimeOfDay::Any);
    }
}
