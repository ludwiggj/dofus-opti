// Part of the library. When the crate is compiled, the first step is to compile the library.

// use crate::... refers to the library's module hierarchy.
// use packagename::... will not work inside the library part of the crate because only the
// names of dependencies are available at the top level.

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Gear {
    pub name: String,
    pub gear_type: GearType,
    pub level: u32,
    pub characteristics: Vec<CharacteristicRange>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GearType {
    Amulet,
    Axe,
    Belt,
    Boots,
    Bow,
    Ring,
    Sword,
}

pub static ALL_GEAR_TYPES: &[GearType] = &[
    GearType::Amulet,
    GearType::Axe,
    GearType::Belt,
    GearType::Boots,
    GearType::Bow,
    GearType::Ring,
    GearType::Sword,
];

// As per https://users.rust-lang.org/t/how-can-i-implement-fmt-display-for-enum/24111
impl fmt::Display for GearType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // defers to the debug formatter
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
pub struct CharacteristicRange {
    pub kind: CharacteristicType,
    pub min: i32,
    pub max: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CharacteristicType {
    AbilityPoint,
    AbilityPointParry,
    AbilityPointReduction,
    AirDamage,
    AirResistance,
    AirResistancePercent,
    Agility,
    Chance,
    Critical,
    CriticalDamage,
    CriticalResistance,
    Damage,
    Dodge,
    EarthDamage,
    EarthResistance,
    EarthResistancePercent,
    Heals,
    Initiative,
    Intelligence,
    FireDamage,
    FireResistance,
    FireResistancePercent,
    Lock,
    MeleeDamage,
    MeleeResistance,
    MovementPoint,
    MovementPointParry,
    MovementPointReduction,
    NegativeOne,
    NeutralDamage,
    NeutralResistancePercent,
    NeutralResistance,
    Pods,
    Power,
    Prospecting,
    PushBackDamage,
    PushBackResistance,
    Range,
    RangeDamage,
    RangeResistance,
    ReflectedDamage,
    Strength,
    Summon,
    SpellDamage,
    TrapDamage,
    TrapPower,
    Vitality,
    WaterDamage,
    WaterResistance,
    WaterResistancePercent,
    WeaponDamage,
    Wisdom,
    Zero,
}

pub static ALL_CHARACTERISTIC_TYPES: &[CharacteristicType] = &[
    CharacteristicType::AbilityPoint,
    CharacteristicType::AbilityPointParry,
    CharacteristicType::AbilityPointReduction,
    CharacteristicType::AirDamage,
    CharacteristicType::AirResistance,
    CharacteristicType::AirResistancePercent,
    CharacteristicType::Agility,
    CharacteristicType::Chance,
    CharacteristicType::Critical,
    CharacteristicType::CriticalDamage,
    CharacteristicType::CriticalResistance,
    CharacteristicType::Damage,
    CharacteristicType::Dodge,
    CharacteristicType::EarthDamage,
    CharacteristicType::EarthResistance,
    CharacteristicType::EarthResistancePercent,
    CharacteristicType::Heals,
    CharacteristicType::Initiative,
    CharacteristicType::Intelligence,
    CharacteristicType::FireDamage,
    CharacteristicType::FireResistance,
    CharacteristicType::FireResistancePercent,
    CharacteristicType::Lock,
    CharacteristicType::MeleeDamage,
    CharacteristicType::MeleeResistance,
    CharacteristicType::MovementPoint,
    CharacteristicType::MovementPointParry,
    CharacteristicType::MovementPointReduction,
    CharacteristicType::NegativeOne,
    CharacteristicType::NeutralDamage,
    CharacteristicType::NeutralResistancePercent,
    CharacteristicType::NeutralResistance,
    CharacteristicType::Pods,
    CharacteristicType::Power,
    CharacteristicType::Prospecting,
    CharacteristicType::PushBackDamage,
    CharacteristicType::PushBackResistance,
    CharacteristicType::Range,
    CharacteristicType::RangeDamage,
    CharacteristicType::RangeResistance,
    CharacteristicType::ReflectedDamage,
    CharacteristicType::Strength,
    CharacteristicType::Summon,
    CharacteristicType::SpellDamage,
    CharacteristicType::TrapDamage,
    CharacteristicType::TrapPower,
    CharacteristicType::Vitality,
    CharacteristicType::WaterDamage,
    CharacteristicType::WaterResistance,
    CharacteristicType::WaterResistancePercent,
    CharacteristicType::WeaponDamage,
    CharacteristicType::Wisdom,
    CharacteristicType::Zero,
];