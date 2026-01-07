use std::collections::HashMap;
use std::fmt;
use core_renamed::model::{Gear, GearType};
use thiserror::Error;

#[derive(Debug, Error, Eq, Hash, PartialEq)]
enum BuildError {
    #[error("Gear cannot be put in the expected slot, gear: {0}, slot: {1}")]
    InvalidGearSlot(String, GearSlot),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum GearSlot {
    Amulet,
    Belt,
    Boots,
    Cloak,
    Hat,
    Ring1,
    Ring2,
    Shield,
    Weapon,
}

impl GearSlot {
    pub fn is_valid_for(&self, gear_type: &GearType) -> bool {
        match(self, gear_type) {
            (GearSlot::Amulet, GearType::Amulet) => true,
            (GearSlot::Belt,   GearType::Belt)   => true,
            (GearSlot::Boots,  GearType::Boots)  => true,
            (GearSlot::Cloak,  GearType::Cloak)  => true,
            (GearSlot::Hat,    GearType::Hat)    => true,
            (GearSlot::Ring1,  GearType::Ring)   => true,
            (GearSlot::Ring2,  GearType::Ring)   => true,
            (GearSlot::Shield, GearType::Shield) => true,
            (GearSlot::Weapon, GearType::Axe)    => true,
            (GearSlot::Weapon, GearType::Bow)    => true,
            (GearSlot::Weapon, GearType::Dagger) => true,
            // ... all other weapon types ...
            _ => false,
        }
    }
}

impl fmt::Display for GearSlot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // defers to the debug formatter
        write!(f, "{:?}", self)
    }
}

struct Build {
    gear_slots: HashMap<GearSlot, Gear>,
}

fn check_gear_slot(gear_slot: GearSlot, gear: &Gear) -> Result<(), BuildError> {
    if gear_slot.is_valid_for(&gear.gear_type) {
        Ok(())
    } else {
        Err(BuildError::InvalidGearSlot(gear.name.clone(), gear_slot))
    }
}

impl Build {
    fn get_gear(&self, gear_slot: &GearSlot) -> Option<&Gear> {
        self.gear_slots.get(gear_slot)
    }

    fn set_gear(&mut self, gear_slot: GearSlot, gear: Gear) -> Result<(), BuildError> {
        check_gear_slot(gear_slot, &gear)?;
      self.gear_slots.insert(gear_slot.clone(), gear);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use core_renamed::model::{Gear, GearType};
    use std::collections::HashMap;
    use crate::build::{Build, GearSlot};

    #[test]
    fn get_invalid_gear() -> anyhow::Result<()> {
        let build = Build {
            gear_slots: HashMap::new(),
        };

        assert_eq!(None, build.get_gear(&GearSlot::Amulet));

        Ok(())
    }

    #[test]
    fn insert_and_retrieve_valid_gear() -> anyhow::Result<()> {
        let mut build = Build {
            gear_slots: HashMap::new(),
        };

        let gear = Gear {
            name: "40 inch belt".to_string(),
            gear_type: GearType::Belt,
            level: 0,
            characteristics: vec![],
        };

        let _ = build.set_gear(GearSlot::Belt, gear.clone());

        assert_eq!(Some(&gear), build.get_gear(&GearSlot::Belt));
        Ok(())
    }

    #[test]
    fn insert_invalid_gear() -> anyhow::Result<()> {
        let mut build = Build {
            gear_slots: HashMap::new(),
        };

        let gear = Gear {
            name: "40 inch belt".to_string(),
            gear_type: GearType::Belt,
            level: 0,
            characteristics: vec![],
        };

        let error = build.set_gear(GearSlot::Amulet, gear)
            .expect_err("Expected an error when inserting belt into amulet slot");

        assert_eq!(
            "Gear cannot be put in the expected slot, gear: 40 inch belt, slot: Amulet".to_string(),
            error.to_string()
        );

        Ok(())
    }
}