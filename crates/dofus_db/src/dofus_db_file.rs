use anyhow::Result;
use serde::Deserialize;
use serde_json::{Value as JsonValue};
use std::fs;
use std::path::Path;
use core::model::{Gear, GearType};

use serde::Serialize;
use anyhow::Context;
use crate::model::DofusDbObject;

pub fn save_gears<P: AsRef<Path>, F>(
    base_path: P,
    gear_type: &GearType,
    gears: &Vec<Gear>,
    file_name_field: F,
) -> Result<()>
where
    F: Fn(&JsonValue, usize) -> String,
{
    // Convert gears to JsonValues
    let gear_json: Vec<JsonValue> = gears
        .iter()
        .filter_map(|g| serde_json::to_value(g).ok())
        .collect();

    write_objects(base_path, gear_type.to_string().to_lowercase(), &gear_json, file_name_field)
}

pub fn filename_safe_string(s: String) -> String {
    s.to_lowercase()
        .replace(" ", "_")
        .replace("-", "_")
        .replace("'s", "")
}

pub fn write_objects<P, A, F>(
    base_path: P,
    folder_name: String,
    objects: &Vec<A>,
    get_file_name: F,
) -> Result<()>
where
    P: AsRef<Path>,
    A: Serialize,
    F: Fn(&A, usize) -> String,
{
    let out_dir = base_path.as_ref().join(folder_name);
    fs::create_dir_all(&out_dir).context("Failed to create output dir")?;

    for (i, object) in objects.iter().enumerate() {
        let file_name = get_file_name(&object, i);
        let file_path = out_dir.join(filename_safe_string(file_name));
        let json_str = serde_json::to_string_pretty(object)?;
        fs::write(file_path, json_str).context("Failed to write json file")?;
    }

    println!(
        "✅ Written {} entry/ies to directory {}",
        objects.len(),
        out_dir.to_str().unwrap_or_default()
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


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir_in;
    use crate::model::{DofusDbCharacteristicTypeId, Effect, TranslatedString};
    // This function returns a closure type fn(&Value, usize) -> String, which is a function pointer.
    // The Rust compiler can infer that the closure |o, i| ... matches this signature.
    // fn file_name_orig() -> fn(&Value, usize) -> String {
    //     |o, i| (&o["name"]).as_str().map(String::from).unwrap_or(format!("unknown_{}", i))
    // }

    // In this case, the closure |o, i| get_name(o)... captures the get_name parameter from the outer
    // function scope. A closure that captures variables cannot be coerced into a function pointer.
    fn file_name(get_name: fn (&JsonValue) -> &JsonValue) -> impl Fn(&JsonValue, usize) -> String {
        move |o, i| get_name(o).as_str().map(String::from).unwrap_or(format!("unknown_{}", i))
    }

    #[test]
    fn get_object_name_with_english_name() -> Result<()> {
        let data = r#"{ "name": { "en": "Great Amulet", "fr": "Grande Amulette" } }"#;
        let json_value: JsonValue = serde_json::from_str(data)?;
        let dummy_index = 0;

        assert_eq!(
            file_name(|o| &o["name"]["en"])(&json_value, dummy_index),
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
            file_name(|o| &o["name"]["en"])(&json_value, dummy_index),
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

        let binding = tempdir_in("../../..")?;
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
