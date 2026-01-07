// Part of the library. When the crate is compiled, the first step is to compile the library.

// use crate::... refers to the library's module hierarchy.
// use packagename::... will not work inside the library part of the crate because only the
// names of dependencies are available at the top level.

#![allow(non_snake_case)]

use serde::Deserialize;
use core::model::{CharacteristicType, GearType};

#[derive(Debug, Deserialize)]
pub struct GetObjectsResponse {
    pub total: u32,
    pub limit: u32,
    pub skip: u32,
    pub data: Vec<serde_json::Value>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DofusDbObject {
    pub name: TranslatedString,
    pub typeId: i32,
    pub level: u32,
    pub img: String,
    pub effects: Vec<Effect>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TranslatedString {
    pub en: String,
    pub fr: String
}

#[derive(Debug, Deserialize, Copy, Clone, PartialEq)]
pub struct Effect {
    pub from: i32,
    pub to: i32,
    pub characteristic: DofusDbCharacteristicTypeId
}

#[derive(Debug, PartialEq)]
pub struct DofusDbTypeId(pub i32);

impl DofusDbTypeId {
    pub fn new(id: i32) -> DofusDbTypeId {
        DofusDbTypeId(id)
    }
}

impl From<&GearType> for DofusDbTypeId {
    fn from(gear_type: &GearType) -> Self {
        let id = match gear_type {
            GearType::Amulet => 1,
            GearType::Axe    => 19,
            GearType::Belt   => 10,
            GearType::Boots  => 11,
            GearType::Bow    => 2,
            GearType::Cloak  => 17,
            GearType::Dagger => 5,
            GearType::Hat    => 16,
            GearType::Ring   => 9,
            GearType::Shield => 82,
            GearType::Sword  => 6,
        };
        DofusDbTypeId(id)
    }
}

#[derive(Debug, Deserialize, PartialEq, Copy, Clone)]
pub struct DofusDbCharacteristicTypeId(pub i32);

impl From<&CharacteristicType> for DofusDbCharacteristicTypeId {
    fn from(characteristic_type: &CharacteristicType) -> Self {
        let id = match characteristic_type {
            CharacteristicType::AbilityPoint             => 1,
            CharacteristicType::AbilityPointParry        => 27,
            CharacteristicType::AbilityPointReduction    => 82,
            CharacteristicType::AirDamage                => 91,
            CharacteristicType::AirResistance            => 57,
            CharacteristicType::AirResistancePercent     => 36,
            CharacteristicType::Agility                  => 14,
            CharacteristicType::Chance                   => 13,
            CharacteristicType::Critical                 => 18,
            CharacteristicType::CriticalDamage           => 86,
            CharacteristicType::CriticalResistance       => 87,
            CharacteristicType::Damage                   => 16,
            CharacteristicType::Dodge                    => 78,
            CharacteristicType::EarthDamage              => 88,
            CharacteristicType::EarthResistance          => 54,
            CharacteristicType::EarthResistancePercent   => 33,
            CharacteristicType::Heals                    => 49,
            CharacteristicType::Initiative               => 44,
            CharacteristicType::Intelligence             => 15,
            CharacteristicType::FireDamage               => 89,
            CharacteristicType::FireResistance           => 55,
            CharacteristicType::FireResistancePercent    => 34,
            CharacteristicType::Lock                     => 79,
            CharacteristicType::MeleeDamage              => 125,
            CharacteristicType::MeleeResistance          => 124,
            CharacteristicType::MovementPoint            => 23,
            CharacteristicType::MovementPointParry       => 28,
            CharacteristicType::MovementPointReduction   => 83,
            CharacteristicType::NegativeOne              => -1,
            CharacteristicType::NeutralDamage            => 92,
            CharacteristicType::NeutralResistancePercent => 37,
            CharacteristicType::NeutralResistance        => 58,
            CharacteristicType::Pods                     => 40,
            CharacteristicType::Power                    => 25,
            CharacteristicType::Prospecting              => 48,
            CharacteristicType::PushBackDamage           => 84,
            CharacteristicType::PushBackResistance       => 85,
            CharacteristicType::Range                    => 19,
            CharacteristicType::RangeDamage              => 120,
            CharacteristicType::RangeResistance          => 121,
            CharacteristicType::ReflectedDamage          => 50,
            CharacteristicType::Strength                 => 10,
            CharacteristicType::Summon                   => 26,
            CharacteristicType::SpellDamage              => 123,
            CharacteristicType::TrapDamage               => 70,
            CharacteristicType::TrapPower                => 69,
            CharacteristicType::Vitality                 => 11,
            CharacteristicType::WaterDamage              => 90,
            CharacteristicType::WaterResistance          => 56,
            CharacteristicType::WaterResistancePercent   => 35,
            CharacteristicType::WeaponDamage             => 122,
            CharacteristicType::Wisdom                   => 12,
            CharacteristicType::Zero                     => 0,
        };
        DofusDbCharacteristicTypeId(id)
    }
}
