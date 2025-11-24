// Part of the library. When the crate is compiled, the first step is to compile the library.

// use crate::... refers to the library's module hierarchy.
// use packagename::... will not work inside the library part of the crate because only the
// names of dependencies are available at the top level.

#![allow(non_snake_case)]

use serde::Deserialize;
use crate::models::{CharacteristicType, GearType};

#[derive(Debug, Deserialize)]
pub struct GetObjectsResponse {
    total: u32,
    limit: u32,
    skip: u32,
    pub data: Vec<serde_json::Value>
}

#[derive(Debug, Deserialize)]
pub struct DofusDbObject {
    pub name: TranslatedString,
    pub typeId: i32,
    pub level: u32,
    img: String,
    pub effects: Vec<Effect>
}

#[derive(Debug, Deserialize)]
pub struct TranslatedString {
    pub en: String,
    fr: String
}

#[derive(Debug, Deserialize)]
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
            GearType::Bow    => 2,
            GearType::Sword  => 6,
            GearType::Ring   => 9,
            GearType::Boots  => 11,
            GearType::Axe    => 19,
            GearType::Belt   => 30,
        };
        DofusDbTypeId(id)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DofusDbCharacteristicTypeId(pub i32);

impl From<&CharacteristicType> for DofusDbCharacteristicTypeId {
    fn from(characteristic_type: &CharacteristicType) -> Self {
        let id = match characteristic_type {
            CharacteristicType::Vitality => 11,
            CharacteristicType::Power    => 25,
        };
        DofusDbCharacteristicTypeId(id)
    }
}