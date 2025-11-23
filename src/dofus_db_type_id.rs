// Part of the library. When the crate is compiled, the first step is to compile the library.

// use crate::... refers to the library's module hierarchy.
// use packagename::... will not work inside the library part of the crate because only the
// names of dependencies are available at the top level.

use crate::models::GearType;

#[derive(Debug, PartialEq)]
pub struct DofusDbTypeId(i32);

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

pub fn parse_gear_type(id: DofusDbTypeId) -> Result<GearType, String> {
   match id {
       DofusDbTypeId(1) => Ok(GearType::Amulet),
       DofusDbTypeId(2) => Ok(GearType::Bow),
       DofusDbTypeId(6) => Ok(GearType::Sword),
       DofusDbTypeId(9) => Ok(GearType::Ring),
       DofusDbTypeId(11) => Ok(GearType::Boots),
       DofusDbTypeId(19) => Ok(GearType::Axe),
       DofusDbTypeId(30) => Ok(GearType::Belt),
       DofusDbTypeId(i) => Err(format!("Unknown gear id {}", i))
   }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ALL_GEAR_TYPES;

    #[test]
    fn parse_valid_gear_types() {
        for gear_type in ALL_GEAR_TYPES {
            let result = parse_gear_type(DofusDbTypeId::from(gear_type));
            let expected = Ok(*gear_type);
            assert_eq!(result, expected);
        }
    }
}