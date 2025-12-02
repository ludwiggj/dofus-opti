use crate::dofus_db_models::*;
use crate::models::{Gear, GearType};
use anyhow::Result;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use std::fs;
use std::path::Path;

pub fn get_object_name<F>(object: &JsonValue, file_name_field: &F, index: usize) -> String
where
    F: Fn(&JsonValue) -> &JsonValue,
{
    file_name_field(object)
        .as_str()
        .map(String::from)
        .unwrap_or(format!("unknown_{}", index))
}

pub fn filename_safe_string(s: &str) -> String {
    s.to_lowercase()
        .replace(" ", "_")
        .replace("-", "_")
        .replace("'s", "")
}

pub fn create_filename(gear_type: &GearType, object_name: &str) -> String {
    filename_safe_string(&format!("{gear_type}_{object_name}.json"))
}

pub fn save_gears<P: AsRef<Path>, F>(
    base_path: P,
    gear_type: &GearType,
    gears: &Vec<Gear>,
    file_name_field: F,
) -> Result<()>
where
    F: Fn(&JsonValue) -> &JsonValue,
{
    // Convert gears to JsonValues
    let gear_json: Vec<JsonValue> = gears
        .iter()
        .filter_map(|g| serde_json::to_value(g).ok())
        .collect();

    save_json_gears(base_path, gear_type, &gear_json, file_name_field)
}

pub fn save_json_gears<P: AsRef<Path>, F>(
    base_path: P,
    gear_type: &GearType,
    gears: &Vec<JsonValue>,
    file_name_field: F,
) -> Result<()>
where
    F: Fn(&JsonValue) -> &JsonValue,
{
    let out_dir_path = base_path
        .as_ref()
        .join(gear_type.to_string().to_lowercase());
    fs::create_dir_all(out_dir_path.clone())?;
    for (i, object) in gears.iter().enumerate() {
        let object_name = get_object_name(object, &file_name_field, i);
        let file_name = create_filename(gear_type, &object_name);
        let file_path = out_dir_path.join(file_name);
        let json_str = serde_json::to_string_pretty(object)?;
        fs::write(file_path, json_str)?;
    }
    println!(
        "✅ Written {} entry/ies of gear type {} to directory {}",
        gears.len(),
        gear_type,
        out_dir_path.to_str().unwrap_or_default()
    );
    Ok(())
}

pub fn read_gears<P: AsRef<Path>>(
    base_path: P,
    gear_type: &GearType,
) -> Result<Vec<DofusDbObject>> {
    let in_dir_path = base_path.as_ref().join(gear_type.to_string());
    let in_dir_path_name = in_dir_path.as_path().to_str().unwrap_or("unknown_path");
    let mut gears: Vec<DofusDbObject> = vec![];
    for entry in fs::read_dir(in_dir_path.clone())? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_content = fs::read_to_string(&path);
            match file_content {
                Ok(content) => {
                    let json_value: JsonValue = serde_json::from_str(&content)?;
                    match DofusDbObject::deserialize(json_value) {
                        Err(e) => println!(
                            "❌ Failed to deserialize contents of file {}, {}",
                            path.display(),
                            e
                        ),
                        Ok(dd) => gears.push(dd),
                    }
                }
                Err(e) => {
                    println!("❌ Failed to read file {}: {}", path.display(), e);
                }
            }
        }
    }

    println!(
        "✅ Successfully read {} entries from {} into json",
        gears.len(),
        in_dir_path_name
    );

    Ok(gears)
}

pub fn read_json<P: AsRef<Path>>(path: P) -> Result<JsonValue> {
    let file_content = fs::read_to_string(path)?;
    let json_value: JsonValue = serde_json::from_str(&file_content)?;
    Ok(json_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dofus_db_models::{DofusDbCharacteristicTypeId, Effect};
    use tempfile::TempDir;

    #[test]
    fn get_object_name_with_english_name() -> Result<()> {
        let data = r#"{ "name": { "en": "Great Amulet", "fr": "Grande Amulette" } }"#;
        let json_value: JsonValue = serde_json::from_str(data)?;
        let dummy_index = 0;

        assert_eq!(
            get_object_name(&json_value, &|o| &o["name"]["en"], dummy_index),
            String::from("Great Amulet")
        );

        Ok(())
    }

    #[test]
    fn get_object_name_without_english_name() -> Result<()> {
        let data = r#"{ "name": { "fr": "Grande Amulette" } }"#;
        let json_value: JsonValue = serde_json::from_str(data)?;
        let dummy_index = 0;

        assert_eq!(
            get_object_name(&json_value, &|o| &o["name"]["en"], dummy_index),
            format!("unknown_{dummy_index}")
        );

        Ok(())
    }

    #[test]
    fn write_read_gears() -> Result<()> {
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
            .collect::<Result<Vec<JsonValue>, _>>()?;

        let temp_dir = TempDir::new()?;
        let base_dir = temp_dir.path();
        let gear_type = GearType::Amulet;

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

        save_json_gears(&base_dir, &gear_type, &json_values, |o| &o["name"]["en"])?;
        let read_json_values = read_gears(&base_dir, &gear_type)?;

        assert_eq!(expected_dofus_db_objects, read_json_values);
        Ok(())
    }
}
