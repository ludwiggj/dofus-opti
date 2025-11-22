use crate::models::GearType;

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