// Part of the library. When the crate is compiled, the first step is to compile the library.

// use crate::... refers to the library's module hierarchy.
// use packagename::... will not work inside the library part of the crate because only the
// names of dependencies are available at the top level.

use crate::dofus_db_models::{DofusDbObject, Effect};
use crate::models::{CharacteristicRange, CharacteristicType, Gear};
use crate::dofus_db_type_id::{DofusDbTypeId, parse_gear_type};

pub fn parse_gear(object: DofusDbObject) -> Result<Gear, String> {
   Ok(Gear {
      name: object.name.en,
      gear_type: parse_gear_type(DofusDbTypeId::new(object.typeId))?,
      level: object.level,
      characteristics: parse_characteristics(object.effects)
   })
}

fn parse_characteristics(effects: Vec<Effect>) -> Vec<CharacteristicRange> {
   effects.into_iter()
       .filter_map(|e| parse_characteristic(e).ok())
       .collect()
}

fn parse_characteristic(effect: Effect) -> Result<CharacteristicRange, String> {
   Ok(
      CharacteristicRange {
         kind: parse_characteristic_type(effect.characteristic)?,
         min: effect.from,
         max: effect.to,
      }
   )
}

fn parse_characteristic_type(characteristic: i32) -> Result<CharacteristicType, String> {
   match characteristic {
      11 => Ok(CharacteristicType::Vitality),
      25 => Ok(CharacteristicType::Power),
      _ => Err(format!("Unknown characteristic type {}", characteristic))
   }
}