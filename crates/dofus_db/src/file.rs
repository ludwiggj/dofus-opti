// File utilities for Dofus DB

use anyhow::Result;
use serde_json::Value as JsonValue;
use std::fs;
use std::path::Path;

pub fn read_json<P: AsRef<Path>>(path: P) -> Result<JsonValue> {
    let file_content = fs::read_to_string(path)?;
    let json_value: JsonValue = serde_json::from_str(&file_content)?;
    Ok(json_value)
}

#[cfg(test)]
mod tests {
    use core::file::{file_name, read_gears, write_objects};
    use core::model::GearType;
    use serde_json::{Value as JsonValue};
    use tempfile::tempdir_in;
    use crate::model::{DofusDbCharacteristicTypeId, DofusDbObject, Effect, TranslatedString};

    #[test]
    fn write_read_gears() -> anyhow::Result<()> {
        // r# negates need to escape quotes in JSON strings
        let json_1 = r#"{
  "name": {
    "en": "Great Amulet",
    "fr": "Grande Amulette"
  },
  "typeId": 1,
  "level": 60,
  "img": "https://api.dofusdb.fr/img/items/img.png",
  "effects": [{
    "category": 0,
    "characteristic": 11,
    "effectId": 125,
    "elementId": -1,
    "from": 31,
    "to": 50
  }]
}"#;

        let json_values: Vec<JsonValue> = vec![json_1]
            .into_iter()
            .map(serde_json::from_str)
            .collect::<anyhow::Result<Vec<JsonValue>, _>>()?;

        let binding = tempdir_in(".")?;
        let base_dir = binding.path();
        let gear_type = GearType::Amulet;
        let folder_name = gear_type.to_string().to_lowercase();

        let expected_dofus_db_objects: Vec<DofusDbObject> = vec![DofusDbObject {
            name: TranslatedString {
                en: "Great Amulet".to_string(),
                fr: "Grande Amulette".to_string(),
            },
            typeId: 1,
            level: 60,
            img: "https://api.dofusdb.fr/img/items/img.png".to_string(),
            effects: vec![Effect {
                characteristic: DofusDbCharacteristicTypeId(11),
                from: 31,
                to: 50,
            }],
        }];

        write_objects(&base_dir, folder_name, &json_values, file_name(|o| &o["name"]["en"]))?;

        let read_json_values = read_gears(&base_dir, &gear_type)?;

        assert_eq!(expected_dofus_db_objects, read_json_values);
        Ok(())
    }
}