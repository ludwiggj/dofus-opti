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
    pub characteristics: Vec<CharacteristicRange>
}

#[derive(Debug, PartialEq, Clone)]
pub enum GearType {
    Amulet,
    Axe,
    Belt,
    Boots,
    Bow,
    Ring,
    Sword
}

pub static ALL_GEAR_TYPES: &[GearType] = &[
    GearType::Amulet,
    GearType::Axe,
    GearType::Belt,
    GearType::Boots,
    GearType::Bow,
    GearType::Ring,
    GearType::Sword
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
    pub max: i32
}

#[derive(Debug, PartialEq, Clone)]
pub enum CharacteristicType {
    Vitality,
    Power
    // Add all other types
}

pub static ALL_CHARACTERISTIC_TYPES: &[CharacteristicType] = &[
    CharacteristicType::Vitality,
    CharacteristicType::Power,
];