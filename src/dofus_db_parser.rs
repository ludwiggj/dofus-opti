// Part of the library. When the crate is compiled, the first step is to compile the library.

// use crate::... refers to the library's module hierarchy.
// use packagename::... will not work inside the library part of the crate because only the
// names of dependencies are available at the top level.

use crate::dofus_db_models::*;
use crate::models::*;

pub fn parse_gears(
    gear_type: &GearType,
    dofus_db_objects: Vec<DofusDbObject>
) -> Vec<Gear> {
    let mut gears: Vec<Gear> = Vec::new();
    let input_object_count = dofus_db_objects.len();

    for object in dofus_db_objects {
        let object_name = object.name.en.clone();
        match parse_gear(object) {
            Ok(gear) => gears.push(gear),
            Err(e) => eprintln!("❌ Failed to parse gear ({object_name}): {e}"),
        }
    }
    println!(
        "✅ Successfully parsed {}/{} from DofusDbObject into {gear_type}",
        gears.len(),
        input_object_count
    );
    gears
}

pub fn parse_gear(object: DofusDbObject) -> Result<Gear, String> {
    Ok(Gear {
        name: object.name.en,
        gear_type: parse_gear_type(DofusDbTypeId::new(object.typeId))?,
        level: object.level,
        characteristics: parse_characteristics(object.effects)?,
    })
}

pub fn parse_gear_type(id: DofusDbTypeId) -> Result<GearType, String> {
    ALL_GEAR_TYPES
        .iter()
        .find(|g| DofusDbTypeId::from(*g) == id)
        .ok_or(format!("Unknown gear id {}", id.0))
        .map(|g| g.to_owned())
}

fn parse_characteristics(effects: Vec<Effect>) -> Result<Vec<CharacteristicRange>, String> {
    effects
        .into_iter()
        .map(parse_characteristic_range)
        .collect()
}

fn parse_characteristic_range(effect: Effect) -> Result<CharacteristicRange, String> {
    Ok(CharacteristicRange {
        kind: parse_characteristic_type(effect.characteristic)?,
        min: effect.from,
        max: effect.to,
    })
}

fn parse_characteristic_type(
    characteristic: DofusDbCharacteristicTypeId,
) -> Result<CharacteristicType, String> {
    ALL_CHARACTERISTIC_TYPES
        .iter()
        .find(|c| DofusDbCharacteristicTypeId::from(*c) == characteristic)
        .ok_or(format!(
            "Unknown characteristic type id {}",
            characteristic.0
        ))
        .map(|c| c.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dofus_db_file::read_json;
    use anyhow::Result;
    use std::path::Path;

    static VITALITY: Effect = Effect {
        characteristic: DofusDbCharacteristicTypeId(11),
        from: 10,
        to: 20,
    };

    static POWER: Effect = Effect {
        characteristic: DofusDbCharacteristicTypeId(25),
        from: -5,
        to: 15,
    };

    static UNKNOWN: Effect = Effect {
        characteristic: DofusDbCharacteristicTypeId(99),
        from: 0,
        to: 100,
    };

    #[test]
    fn parse_valid_gear_types() {
        for gear_type in ALL_GEAR_TYPES {
            let type_id = DofusDbTypeId::from(gear_type);
            assert_eq!(parse_gear_type(type_id), Ok(gear_type.clone()));
        }
    }

    #[test]
    fn parse_invalid_gear_type() {
        let invalid_type_id = DofusDbTypeId(-2);
        assert_eq!(
            parse_gear_type(invalid_type_id),
            Err(String::from("Unknown gear id -2"))
        );
    }

    #[test]
    fn parse_valid_characteristic_types() {
        for characteristic_type in ALL_CHARACTERISTIC_TYPES {
            let type_id = DofusDbCharacteristicTypeId::from(characteristic_type);
            assert_eq!(
                parse_characteristic_type(type_id),
                Ok(characteristic_type.clone())
            );
        }
    }

    #[test]
    fn parse_invalid_characteristic_type() {
        let invalid_type_id = DofusDbCharacteristicTypeId(-2);
        assert_eq!(
            parse_characteristic_type(invalid_type_id),
            Err(String::from("Unknown characteristic type id -2"))
        );
    }

    #[test]
    fn parse_valid_characteristics() {
        assert_eq!(
            parse_characteristics(vec![VITALITY, POWER]),
            Ok(vec![
                CharacteristicRange {
                    kind: CharacteristicType::Vitality,
                    min: 10,
                    max: 20,
                },
                CharacteristicRange {
                    kind: CharacteristicType::Power,
                    min: -5,
                    max: 15,
                },
            ])
        )
    }

    #[test]
    fn parse_characteristics_do_not_discard_invalid() {
        assert_eq!(
            parse_characteristics(vec![VITALITY, UNKNOWN]),
            Err(String::from("Unknown characteristic type id 99"))
        )
    }

    #[test]
    fn parse_golden_gear() -> Result<()> {
        let file_path = Path::new("golden").join("amulet_gargandyas_necklace.json");
        let json = read_json(file_path)?;

        let dofus_db_object: DofusDbObject = serde_json::from_value(json)?;

        let gear = parse_gear(dofus_db_object);

        let expected_gear = Gear {
            name: String::from("Gargandyas's Necklace"),
            gear_type: GearType::Amulet,
            level: 200,
            characteristics: vec![
                CharacteristicRange {
                    kind: CharacteristicType::Vitality,
                    min: 451,
                    max: 500,
                },
                CharacteristicRange {
                    kind: CharacteristicType::Wisdom,
                    min: 41,
                    max: 60,
                },
                CharacteristicRange {
                    kind: CharacteristicType::Power,
                    min: 41,
                    max: 60,
                },
                CharacteristicRange {
                    kind: CharacteristicType::Critical,
                    min: 3,
                    max: 4,
                },
                CharacteristicRange {
                    kind: CharacteristicType::AbilityPoint,
                    min: 2,
                    max: 0,
                },
                CharacteristicRange {
                    kind: CharacteristicType::MovementPoint,
                    min: -1,
                    max: 0,
                },
                CharacteristicRange {
                    kind: CharacteristicType::Range,
                    min: 1,
                    max: 0,
                },
                CharacteristicRange {
                    kind: CharacteristicType::Summon,
                    min: 2,
                    max: 0,
                },
                CharacteristicRange {
                    kind: CharacteristicType::Dodge,
                    min: -20,
                    max: 0,
                },
                CharacteristicRange {
                    kind: CharacteristicType::MovementPointParry,
                    min: -20,
                    max: 0,
                },
                CharacteristicRange {
                    kind: CharacteristicType::PushBackDamage,
                    min: 16,
                    max: 20,
                },
                CharacteristicRange {
                    kind: CharacteristicType::PushBackResistance,
                    min: 31,
                    max: 40,
                },
                CharacteristicRange {
                    kind: CharacteristicType::MeleeResistance,
                    min: 3,
                    max: 5,
                },
                CharacteristicRange {
                    kind: CharacteristicType::RangeResistance,
                    min: 3,
                    max: 5,
                },
            ],
        };

        assert_eq!(gear, Ok(expected_gear));

        Ok(())
    }

    #[test]
    fn parse_gear_fails_if_invalid_type_id() -> Result<()> {
        let invalid_type_id = -999;

        assert_eq!(
            parse_gear(DofusDbObject {
                name: TranslatedString {
                    en: String::from("Invalid Gear"),
                    fr: String::from("Objet Invalide"),
                },
                typeId: invalid_type_id,
                level: 1,
                img: String::from("invalid_gear.png"),
                effects: vec![VITALITY],
            }),
            Err(format!("Unknown gear id {}", invalid_type_id))
        );

        Ok(())
    }

    #[test]
    fn parse_gear_fails_if_invalid_effect() -> Result<()> {
        assert_eq!(
            parse_gear(DofusDbObject {
                name: TranslatedString {
                    en: String::from("Studded Belt of Death"),
                    fr: String::from("Ceinture Cloutée de la Mort"),
                },
                typeId: 30,
                level: 50,
                img: String::from("studded_belt_of_death.png"),
                effects: vec![VITALITY, POWER, UNKNOWN],
            }),
            Err(String::from("Unknown characteristic type id 99"))
        );

        Ok(())
    }

    // #[test]
    // parse_gears test with multiple gears, reusing test gear structures....
}