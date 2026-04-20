//! Titan anatomy zones and their properties.
//!
//! Defines the different regions on the Titan's body where players
//! can build, harvest, and survive.

use std::fmt;

use serde::{Deserialize, Serialize};

/// The different anatomical zones on the Titan's body.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum TitanZone {
    /// High, exposed ridges on the shell - safe but resource-poor.
    #[default]
    ShellRidge,
    /// Valleys between scales - moderate safety, moderate resources.
    ScaleValley,
    /// Dense parasitic growths - rich resources, dangerous creatures.
    ParasiteForest,
    /// Hot vents where the Titan breathes - high temperature, unique resources.
    BreathingVent,
    /// Open wounds on the Titan - triggers immune response, valuable tissue.
    WoundSite,
    /// Neural clusters - relatively safe, mysterious properties.
    NeuralNode,
}

impl fmt::Display for TitanZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TitanZone::ShellRidge => write!(f, "Shell Ridge"),
            TitanZone::ScaleValley => write!(f, "Scale Valley"),
            TitanZone::ParasiteForest => write!(f, "Parasite Forest"),
            TitanZone::BreathingVent => write!(f, "Breathing Vent"),
            TitanZone::WoundSite => write!(f, "Wound Site"),
            TitanZone::NeuralNode => write!(f, "Neural Node"),
        }
    }
}

impl TitanZone {
    /// Get all zone variants.
    #[must_use]
    pub fn all() -> &'static [TitanZone] {
        &[
            TitanZone::ShellRidge,
            TitanZone::ScaleValley,
            TitanZone::ParasiteForest,
            TitanZone::BreathingVent,
            TitanZone::WoundSite,
            TitanZone::NeuralNode,
        ]
    }
}

/// Properties associated with each Titan zone.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZoneProperties {
    /// The zone these properties describe.
    pub zone: TitanZone,
    /// Base temperature in degrees Celsius.
    pub base_temperature: f32,
    /// Structural stability (1.0 = fully stable).
    pub stability: f32,
    /// Resource availability multiplier.
    pub resource_richness: f32,
    /// Danger level from creatures and hazards (0.0-1.0).
    pub danger_level: f32,
}

impl ZoneProperties {
    /// Get properties for a specific zone.
    #[must_use]
    pub fn for_zone(zone: TitanZone) -> Self {
        match zone {
            TitanZone::ShellRidge => Self {
                zone,
                base_temperature: 15.0,
                stability: 1.0,
                resource_richness: 0.3,
                danger_level: 0.1,
            },
            TitanZone::ScaleValley => Self {
                zone,
                base_temperature: 20.0,
                stability: 0.8,
                resource_richness: 0.5,
                danger_level: 0.3,
            },
            TitanZone::ParasiteForest => Self {
                zone,
                base_temperature: 25.0,
                stability: 0.6,
                resource_richness: 0.9,
                danger_level: 0.7,
            },
            TitanZone::BreathingVent => Self {
                zone,
                base_temperature: 40.0,
                stability: 0.3,
                resource_richness: 0.7,
                danger_level: 0.8,
            },
            TitanZone::WoundSite => Self {
                zone,
                base_temperature: 30.0,
                stability: 0.4,
                resource_richness: 0.8,
                danger_level: 0.5,
            },
            TitanZone::NeuralNode => Self {
                zone,
                base_temperature: 18.0,
                stability: 0.9,
                resource_richness: 0.4,
                danger_level: 0.2,
            },
        }
    }

    /// Check if the zone is safe for building.
    #[must_use]
    pub fn is_buildable(&self) -> bool {
        self.stability >= 0.5
    }

    /// Check if the zone is considered dangerous.
    #[must_use]
    pub fn is_dangerous(&self) -> bool {
        self.danger_level >= 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_titan_zone_display() {
        assert_eq!(format!("{}", TitanZone::ShellRidge), "Shell Ridge");
        assert_eq!(format!("{}", TitanZone::ScaleValley), "Scale Valley");
        assert_eq!(format!("{}", TitanZone::ParasiteForest), "Parasite Forest");
        assert_eq!(format!("{}", TitanZone::BreathingVent), "Breathing Vent");
        assert_eq!(format!("{}", TitanZone::WoundSite), "Wound Site");
        assert_eq!(format!("{}", TitanZone::NeuralNode), "Neural Node");
    }

    #[test]
    fn test_titan_zone_default() {
        assert_eq!(TitanZone::default(), TitanZone::ShellRidge);
    }

    #[test]
    fn test_titan_zone_all() {
        let all = TitanZone::all();
        assert_eq!(all.len(), 6);
        assert!(all.contains(&TitanZone::ShellRidge));
        assert!(all.contains(&TitanZone::ParasiteForest));
    }

    #[test]
    fn test_zone_properties_shell_ridge() {
        let props = ZoneProperties::for_zone(TitanZone::ShellRidge);
        assert_eq!(props.zone, TitanZone::ShellRidge);
        assert!((props.base_temperature - 15.0).abs() < f32::EPSILON);
        assert!((props.stability - 1.0).abs() < f32::EPSILON);
        assert!((props.resource_richness - 0.3).abs() < f32::EPSILON);
        assert!((props.danger_level - 0.1).abs() < f32::EPSILON);
    }

    #[test]
    fn test_zone_properties_breathing_vent() {
        let props = ZoneProperties::for_zone(TitanZone::BreathingVent);
        assert!((props.base_temperature - 40.0).abs() < f32::EPSILON);
        assert!((props.stability - 0.3).abs() < f32::EPSILON);
        assert!((props.danger_level - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn test_zone_is_buildable() {
        assert!(ZoneProperties::for_zone(TitanZone::ShellRidge).is_buildable());
        assert!(ZoneProperties::for_zone(TitanZone::ScaleValley).is_buildable());
        assert!(ZoneProperties::for_zone(TitanZone::ParasiteForest).is_buildable());
        assert!(!ZoneProperties::for_zone(TitanZone::BreathingVent).is_buildable());
        assert!(!ZoneProperties::for_zone(TitanZone::WoundSite).is_buildable());
        assert!(ZoneProperties::for_zone(TitanZone::NeuralNode).is_buildable());
    }

    #[test]
    fn test_zone_is_dangerous() {
        assert!(!ZoneProperties::for_zone(TitanZone::ShellRidge).is_dangerous());
        assert!(!ZoneProperties::for_zone(TitanZone::ScaleValley).is_dangerous());
        assert!(ZoneProperties::for_zone(TitanZone::ParasiteForest).is_dangerous());
        assert!(ZoneProperties::for_zone(TitanZone::BreathingVent).is_dangerous());
        assert!(ZoneProperties::for_zone(TitanZone::WoundSite).is_dangerous());
        assert!(!ZoneProperties::for_zone(TitanZone::NeuralNode).is_dangerous());
    }
}
