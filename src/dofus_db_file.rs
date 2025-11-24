use crate::models::GearType;
use anyhow::Result;
use std::path::Path;
use std::fs;
use serde_json::Value;
use tempfile::TempDir;

pub fn get_object_name(object: &Value, index: usize) -> String {
    object["name"]["en"]
        .as_str()
        .map(String::from)
        .unwrap_or(format!("unknown_{}", index))
}

pub fn create_filename(gear_type: &GearType, object_name: &str) -> String {
    format!("{gear_type}_{object_name}.json")
        .to_lowercase()
        .replace(" ", "_")
        .replace("-", "_")
        .replace("'s", "")
}

pub fn save_gears<P: AsRef<Path>>(
    base_path: P,
    gear_type: &GearType,
    gears: &Vec<Value>
) -> Result<()> {
    let out_dir_path = base_path.as_ref().join(gear_type.to_string());
    fs::create_dir_all(out_dir_path.clone())?;
    for (i, object) in gears.iter().enumerate() {
        let object_name = get_object_name(object, i);
        let file_name = create_filename(gear_type, &object_name);
        let file_path = out_dir_path.join(file_name);
        let json_str = serde_json::to_string_pretty(object)?;
        fs::write(file_path, json_str)?;
    }
    println!(
        "Written {} entry/ies of gear type {} to directory {}",
        gears.len(),
        gear_type,
        out_dir_path.to_str().unwrap_or_default()
    );
    Ok(())
}

pub fn read_gears<P: AsRef<Path>>(
    base_path: P,
    gear_type: &GearType
) -> Result<Vec<Value>> {
    let in_dir_path = base_path.as_ref().join(gear_type.to_string());
    let mut gears: Vec<Value> = vec![];
    for entry in fs::read_dir(in_dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_content = fs::read_to_string(&path)?;
            let json_value: Value = serde_json::from_str(&file_content)?;
            gears.push(json_value);
        }
    }
    Ok(gears)
}

#[test]
fn write_read_gears() -> Result<()> {
    // r# negates need to escape quotes in JSON strings
    let json_1 = r#"{ "name": { "en": "Great Amulet", "fr": "Grande Amulette" } }"#;
    let json_2 = r#"{ "foo": "bar" }"#;

    let json_values: Vec<Value> = vec![json_1, json_2]
        .into_iter()
        .map(serde_json::from_str)
        .collect::<Result<Vec<Value>, _>>()?;

    let temp_dir = TempDir::new()?;
    let base_dir = temp_dir.path();
    let gear_type = GearType::Amulet;

    save_gears(&base_dir, &gear_type, &json_values)?;
    let read_json_values = read_gears(&base_dir, &gear_type)?;

    assert_eq!(json_values, read_json_values);

    Ok(())
}