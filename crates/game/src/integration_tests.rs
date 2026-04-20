//! Integration tests for Titan game systems.
//!
//! These tests verify that multiple game systems work together correctly,
//! covering movement cycles, balance mechanics, terrain deformation,
//! resource harvesting, structures, Titan mood, and neural guidance.

use std::collections::HashMap;

use glam::IVec3;

use crate::balance::{BalanceMeter, Falling};
use crate::crafting::{
    NeuralInterface, OrganicLab, ShellForge, ThermalConverter, RECIPE_COAGULANT,
    RECIPE_SHELL_INGOT,
};
use crate::creatures::{HostileCreature, HostileType, PassiveCreature, PassiveType};
use crate::networking::{
    deserialize_positions, deserialize_titan_state, serialize_positions, serialize_titan_state,
    PositionSync, TitanSync,
};
use crate::titan::{
    TitanBehavior, TitanMood, TitanMovement, TitanPhase, TitanZone, ZoneProperties,
    AGITATED_THRESHOLD, ENRAGED_THRESHOLD, MAX_TITAN_HP,
};

/// Test 1: Full movement cycle - resting -> walking -> running -> scratching -> resting
#[test]
fn test_full_movement_cycle() {
    let mut movement = TitanMovement::new();
    assert_eq!(movement.current_phase(), TitanPhase::Resting);

    // Track phases encountered
    let mut phases_seen = vec![TitanPhase::Resting];

    // Run for many ticks to cycle through phases
    for _ in 0..1000 {
        if let Some(new_phase) = movement.tick(1.0) {
            if !phases_seen.contains(&new_phase) {
                phases_seen.push(new_phase);
            }
        }
    }

    // Should have seen all phases
    assert!(
        phases_seen.contains(&TitanPhase::Walking),
        "Should see Walking phase"
    );
    assert!(
        phases_seen.contains(&TitanPhase::Running) || phases_seen.contains(&TitanPhase::Scratching),
        "Should see Running or Scratching phase"
    );
}

/// Test 2: Balance recovery during resting phase
#[test]
fn test_balance_recovery_during_phases() {
    let mut balance = BalanceMeter::new();

    // Deplete balance
    balance.modify(-50.0);
    assert!((balance.balance() - 50.0).abs() < f32::EPSILON);

    // Tick during resting - should recover
    balance.tick(1.0, TitanPhase::Resting);
    assert!(balance.balance() > 50.0, "Balance should recover during resting");

    // Reset and test other phases
    balance.modify(100.0 - balance.balance()); // Reset to 100
    balance.tick(1.0, TitanPhase::Walking);
    assert!(
        balance.balance() < 100.0,
        "Balance should decrease during walking"
    );

    balance.modify(100.0 - balance.balance());
    balance.tick(1.0, TitanPhase::Running);
    assert!(
        balance.balance() < 95.0,
        "Balance should decrease more during running"
    );
}

/// Test 3: Fall detection when balance depleted
#[test]
fn test_fall_detection() {
    let mut balance = BalanceMeter::new();

    // Not falling with full balance
    assert!(!balance.is_falling());

    // Deplete balance completely
    balance.modify(-100.0);
    assert!(balance.is_falling(), "Should be falling with zero balance");

    // Reset and verify
    balance.reset();
    assert!(!balance.is_falling(), "Should not be falling after reset");
}

/// Test 4: Terrain deformation affects stability
#[test]
fn test_terrain_deformation_stability() {
    // Test zone properties for stability
    let shell_ridge = ZoneProperties::for_zone(TitanZone::ShellRidge);
    let breathing_vent = ZoneProperties::for_zone(TitanZone::BreathingVent);

    assert!(
        shell_ridge.stability > breathing_vent.stability,
        "Shell Ridge should be more stable than Breathing Vent"
    );

    assert!(
        shell_ridge.is_buildable(),
        "Shell Ridge should be buildable"
    );
    assert!(
        !breathing_vent.is_buildable(),
        "Breathing Vent should not be buildable"
    );
}

/// Test 5: Resource harvesting affects Titan mood
#[test]
fn test_resource_harvesting_mood() {
    let mut behavior = TitanBehavior::new();
    assert_eq!(behavior.current_mood(), TitanMood::Calm);

    // Harvest tissue multiple times
    for _ in 0..4 {
        behavior.harvest_tissue();
    }

    // Should be agitated after 4 harvests (40 agitation > 30 threshold)
    assert_eq!(
        behavior.current_mood(),
        TitanMood::Agitated,
        "Should be agitated after multiple harvests"
    );

    // Continue harvesting to enrage
    for _ in 0..4 {
        behavior.harvest_tissue();
    }

    assert_eq!(
        behavior.current_mood(),
        TitanMood::Enraged,
        "Should be enraged after many harvests"
    );
}

/// Test 6: Structure survival during movement phases
#[test]
fn test_structure_survival_during_movement() {
    // Test stability modifiers during different phases
    let resting_props = crate::titan::PhaseProperties::for_phase(TitanPhase::Resting);
    let running_props = crate::titan::PhaseProperties::for_phase(TitanPhase::Running);

    assert!(
        (resting_props.stability_modifier - 1.0).abs() < f32::EPSILON,
        "Resting should have full stability"
    );
    assert!(
        running_props.stability_modifier < 0.5,
        "Running should have reduced stability"
    );

    // Zones with low stability should be dangerous during running
    let vent_zone = ZoneProperties::for_zone(TitanZone::BreathingVent);
    let effective_stability = vent_zone.stability * running_props.stability_modifier;
    assert!(
        effective_stability < 0.2,
        "Vent zone during running should be very unstable"
    );
}

/// Test 7: Titan mood changes affect environment
#[test]
fn test_titan_mood_changes() {
    let mut behavior = TitanBehavior::new();

    // Mood starts calm
    assert_eq!(behavior.current_mood(), TitanMood::Calm);
    assert!((behavior.agitation() - 0.0).abs() < f32::EPSILON);

    // Deal damage to agitate
    behavior.deal_damage(2000.0);
    assert!(behavior.agitation() > 0.0, "Damage should increase agitation");

    // Kill parasites to calm
    let agitation_before = behavior.agitation();
    behavior.kill_parasite();
    assert!(
        behavior.agitation() < agitation_before,
        "Killing parasites should reduce agitation"
    );

    // Test mood decay over time
    behavior.harvest_tissue(); // Add some agitation
    let agitation_before = behavior.agitation();
    behavior.tick(1.0);
    assert!(
        behavior.agitation() < agitation_before,
        "Agitation should decay over time"
    );
}

/// Test 8: Neural guidance costs and cooldowns
#[test]
fn test_neural_guidance() {
    let mut interface = NeuralInterface::new();
    let mut neural_fluid = 50;

    // Should be able to guide initially
    assert!(interface.can_guide());

    // Guide the Titan
    let _result = interface.guide_titan(IVec3::new(1, 0, 0), &mut neural_fluid);

    // Fluid should be consumed
    assert!(neural_fluid < 50, "Neural fluid should be consumed");

    // Should be on cooldown
    assert!(!interface.can_guide(), "Should be on cooldown after guidance");

    // Tick past cooldown
    interface.tick(61.0);
    assert!(interface.can_guide(), "Should be able to guide after cooldown");
}

/// Test 9: Crafting station integration - Shell Forge
#[test]
fn test_shell_forge_crafting() {
    let forge = ShellForge::new();

    // Should have default recipes
    assert!(forge.get_recipe(RECIPE_SHELL_INGOT).is_some());

    // Craft with sufficient materials
    let mut materials = HashMap::new();
    materials.insert("raw_shell".to_string(), 10);
    materials.insert("titan_calcium".to_string(), 5);

    let result = forge.craft(RECIPE_SHELL_INGOT, &materials);
    assert!(result.is_some(), "Should successfully craft shell ingot");
    assert_eq!(result.unwrap().item, "shell_ingot");
}

/// Test 10: Crafting station integration - Organic Lab
#[test]
fn test_organic_lab_crafting() {
    let lab = OrganicLab::new();

    // Should have default recipes
    assert!(lab.get_recipe(RECIPE_COAGULANT).is_some());

    // Craft with sufficient materials
    let mut materials = HashMap::new();
    materials.insert("titan_blood".to_string(), 10);
    materials.insert("clotting_enzyme".to_string(), 5);
    materials.insert("binding_agent".to_string(), 5);

    let result = lab.craft(RECIPE_COAGULANT, &materials);
    assert!(result.is_some(), "Should successfully craft coagulant");
    assert_eq!(result.unwrap().item, "coagulant");
}

/// Test 11: Thermal converter power generation
#[test]
fn test_thermal_converter_power() {
    let mut converter = ThermalConverter::new();
    assert!(converter.is_operational());

    // Generate power
    let generated = converter.generate(60.0); // 1 minute
    assert!(generated > 0.0, "Should generate power");

    // Power should be stored
    assert!(converter.stored_power() > 0.0, "Power should be stored");

    // Consume power
    let consumed = converter.consume(5.0);
    assert!((consumed - 5.0).abs() < f32::EPSILON, "Should consume power");
}

/// Test 12: Hostile creature abilities
#[test]
fn test_creature_abilities() {
    // Scale Tick - drain ability
    let tick = HostileCreature::new(HostileType::ScaleTick);
    let ability = tick.use_ability();
    assert!(ability.success);
    assert!(ability.effect.contains("Drains"));

    // Shell Borer - tunnel ability with area effect
    let borer = HostileCreature::new(HostileType::ShellBorer);
    let ability = borer.use_ability();
    assert!(ability.success);
    assert!(ability.area_radius > 0.0, "Tunnel should have area effect");

    // Mouth Crawler - instant kill for low balance players
    let crawler = HostileCreature::new(HostileType::MouthCrawler);
    let ability = crawler.use_ability();
    assert!(ability.success);
    assert_eq!(ability.damage, 999, "Devour should be lethal");
}

/// Test 13: Passive creature behaviors
#[test]
fn test_passive_creatures() {
    // Scale Moth - found on scales, provides cleaning
    let moth = PassiveCreature::new(PassiveType::ScaleMoth);
    assert!(moth.is_alive());
    assert!(moth.special_trait().contains("cleans"));

    // Vent Shrimp - found in thermal vents
    let shrimp = PassiveCreature::new(PassiveType::VentShrimp);
    assert!(shrimp.special_trait().contains("heat"));

    // Neural Butterfly - found at neural nodes
    let butterfly = PassiveCreature::new(PassiveType::NeuralButterfly);
    assert!(butterfly.special_trait().contains("neural"));
}

/// Test 14: Zone properties and dangers
#[test]
fn test_zone_dangers() {
    let shell_ridge = ZoneProperties::for_zone(TitanZone::ShellRidge);
    let parasite_forest = ZoneProperties::for_zone(TitanZone::ParasiteForest);
    let wound_site = ZoneProperties::for_zone(TitanZone::WoundSite);

    // Shell Ridge should be safest
    assert!(
        !shell_ridge.is_dangerous(),
        "Shell Ridge should not be dangerous"
    );

    // Parasite Forest should be dangerous
    assert!(
        parasite_forest.is_dangerous(),
        "Parasite Forest should be dangerous"
    );

    // Resource richness should vary
    assert!(
        shell_ridge.resource_richness < parasite_forest.resource_richness,
        "Parasite Forest should have more resources than Shell Ridge"
    );

    // Temperature varies by zone
    assert!(
        ZoneProperties::for_zone(TitanZone::BreathingVent).base_temperature > 35.0,
        "Breathing Vent should be hot"
    );
}

/// Test 15: Titan state network sync
#[test]
fn test_titan_state_sync() {
    let mut sync = TitanSync::new();

    // Create state packet
    let packet = sync.create_packet(8000.0, 1, 2, 10);

    // Update from packet
    let updated = sync.update_from_network(&packet);
    assert!(updated);
    assert!((sync.hp() - 8000.0).abs() < f32::EPSILON);
    assert_eq!(sync.mood(), 1);
    assert_eq!(sync.phase(), 2);
    assert_eq!(sync.day(), 10);
}

/// Test 16: Position network sync
#[test]
fn test_position_sync() {
    let mut sync = PositionSync::new();

    let positions = vec![
        (1u64, IVec3::new(100, 50, 200)),
        (2u64, IVec3::new(-50, 100, -100)),
    ];

    let packet = serialize_positions(&positions);
    sync.update_from_network(&packet);

    assert_eq!(sync.count(), 2);
    assert_eq!(sync.get_position(1), Some(IVec3::new(100, 50, 200)));
    assert_eq!(sync.get_position(2), Some(IVec3::new(-50, 100, -100)));
}

/// Test 17: Balance with equipment bonus
#[test]
fn test_balance_equipment_bonus() {
    let mut balance_no_bonus = BalanceMeter::new();
    let mut balance_with_bonus = BalanceMeter::with_equipment(0.5);

    // Both tick during running
    balance_no_bonus.tick(1.0, TitanPhase::Running);
    balance_with_bonus.tick(1.0, TitanPhase::Running);

    // Equipment bonus should reduce balance loss
    assert!(
        balance_with_bonus.balance() > balance_no_bonus.balance(),
        "Equipment bonus should reduce balance loss"
    );
}

/// Test 18: Titan HP and mortality
#[test]
fn test_titan_hp_mortality() {
    let mut behavior = TitanBehavior::new();
    assert!(behavior.is_alive());
    assert!((behavior.hp() - MAX_TITAN_HP).abs() < f32::EPSILON);

    // Deal massive damage
    behavior.deal_damage(MAX_TITAN_HP);

    assert!(!behavior.is_alive(), "Titan should die at 0 HP");
    assert!((behavior.hp() - 0.0).abs() < f32::EPSILON);
}

/// Test 19: Movement phase properties
#[test]
fn test_movement_phase_properties() {
    let resting = crate::titan::PhaseProperties::for_phase(TitanPhase::Resting);
    let walking = crate::titan::PhaseProperties::for_phase(TitanPhase::Walking);
    let running = crate::titan::PhaseProperties::for_phase(TitanPhase::Running);
    let scratching = crate::titan::PhaseProperties::for_phase(TitanPhase::Scratching);

    // Resting should have no shift
    assert_eq!(resting.shift_rate, 0);

    // Running should have highest shift
    assert!(running.shift_rate > walking.shift_rate);
    assert!(running.shift_rate > scratching.shift_rate);

    // Wind force should increase with activity
    assert!(running.wind_force > walking.wind_force);
    assert!(walking.wind_force > resting.wind_force);
}

/// Test 20: Titan behavior agitation thresholds
#[test]
fn test_agitation_thresholds() {
    assert!(
        AGITATED_THRESHOLD < ENRAGED_THRESHOLD,
        "Agitated threshold should be lower than enraged"
    );

    let mut behavior = TitanBehavior::new();

    // Just below agitated threshold
    for _ in 0..2 {
        behavior.harvest_tissue(); // 20 agitation
    }
    assert_eq!(behavior.current_mood(), TitanMood::Calm);

    // Cross agitated threshold
    behavior.harvest_tissue(); // 30 agitation
    assert_eq!(behavior.current_mood(), TitanMood::Agitated);

    // Cross enraged threshold
    for _ in 0..4 {
        behavior.harvest_tissue(); // 70+ agitation
    }
    assert_eq!(behavior.current_mood(), TitanMood::Enraged);
}

/// Test 21: Network serialization roundtrip
#[test]
fn test_network_serialization_roundtrip() {
    // Titan state
    let hp = 7500.5;
    let mood = 2u8;
    let phase = 1u8;
    let day = 42u32;

    let data = serialize_titan_state(hp, mood, phase, day);
    let (hp2, mood2, phase2, day2) = deserialize_titan_state(&data).unwrap();

    assert!((hp - hp2).abs() < f32::EPSILON);
    assert_eq!(mood, mood2);
    assert_eq!(phase, phase2);
    assert_eq!(day, day2);

    // Positions
    let positions = vec![
        (100u64, IVec3::new(i32::MIN, 0, i32::MAX)),
        (u64::MAX, IVec3::new(0, -1000, 1000)),
    ];

    let data = serialize_positions(&positions);
    let positions2 = deserialize_positions(&data);

    assert_eq!(positions.len(), positions2.len());
    assert_eq!(positions[0], positions2[0]);
    assert_eq!(positions[1], positions2[1]);
}

/// Test 22: Falling system damage calculation
#[test]
fn test_falling_damage() {
    let mut falling = Falling::new();

    // Add an anchor for respawn
    falling.add_anchor(IVec3::new(0, 100, 0));

    // Check fall from below titan height
    let result = falling.check_fall(IVec3::new(0, 50, 0), 100);
    assert!(result.is_some(), "Should detect fall below titan height");

    let fall_result = result.unwrap();
    assert!(fall_result.damage > 0.0, "Should take fall damage");
    assert!(fall_result.fall_distance > 0.0, "Should have fall distance");
}

/// Test 23: Neural interface mood influence
#[test]
fn test_neural_mood_influence() {
    let mut interface = NeuralInterface::new();
    let mut fluid = 50;

    // Should be able to influence mood
    assert!(interface.can_influence_mood());

    // Influence mood
    let result = interface.influence_mood(&mut fluid);

    // Fluid consumed regardless of success
    assert!(fluid < 50);

    // Should be on cooldown
    assert!(!interface.can_influence_mood());

    // Tick past cooldown (mood cooldown is 120s)
    interface.tick(121.0);
    assert!(interface.can_influence_mood());
}

/// Test 24: Zone temperature effects
#[test]
fn test_zone_temperatures() {
    let zones = TitanZone::all();

    for zone in zones {
        let props = ZoneProperties::for_zone(*zone);

        // All zones should have reasonable temperatures
        assert!(
            props.base_temperature >= 0.0 && props.base_temperature <= 50.0,
            "Temperature should be in reasonable range for {:?}",
            zone
        );
    }

    // Breathing Vent should be hottest
    let vent = ZoneProperties::for_zone(TitanZone::BreathingVent);
    let ridge = ZoneProperties::for_zone(TitanZone::ShellRidge);
    assert!(vent.base_temperature > ridge.base_temperature);
}

/// Test 25: Complete gameplay scenario
#[test]
fn test_complete_gameplay_scenario() {
    // Initialize systems
    let mut movement = TitanMovement::new();
    let mut behavior = TitanBehavior::new();
    let mut balance = BalanceMeter::new();

    // Simulate 100 ticks of gameplay
    for tick in 0..100u32 {
        let dt = 1.0;

        // Update movement
        if let Some(new_phase) = movement.tick(dt) {
            // Phase changed - log it
            let _ = new_phase;
        }

        // Update balance based on phase
        let phase = movement.current_phase();
        balance.tick(dt, phase);

        // Update behavior
        behavior.tick(dt);

        // Occasionally harvest (every 20 ticks)
        if tick % 20 == 0 {
            behavior.harvest_tissue();
        }

        // Check game state
        assert!(behavior.is_alive(), "Titan should still be alive");
    }

    // After gameplay, state should be valid
    assert!(balance.balance() >= 0.0 && balance.balance() <= 100.0);
    assert!(behavior.agitation() >= 0.0 && behavior.agitation() <= 100.0);
}
