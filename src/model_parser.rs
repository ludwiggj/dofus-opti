use crate::dofus_db_models::{DofusDbObject, Effect};
use crate::models::{CharacteristicRange, CharacteristicType, Gear, GearType};

pub fn parse_gear(object: DofusDbObject) -> Result<Gear, String> {
   Ok(Gear {
      name: object.name.en,
      gear_type: parse_object_type(object.typeId)?,
      level: object.level,
      characteristics: parse_characteristics(object.effects)
   })
}

fn parse_object_type(id: i32) -> Result<GearType, String> {
   match id {
      1 => Ok(GearType::Amulet),
      2 => Ok(GearType::Bow),
      6 => Ok(GearType::Sword),
      9 => Ok(GearType::Ring),
      11 => Ok(GearType::Boots),
      19 => Ok(GearType::Axe),
      30 => Ok(GearType::Belt),
      _ => Err(format!("Unknown gear type {}", id))
   }
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