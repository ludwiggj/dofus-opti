use std::error::Error;
use crate::models::GearType;
use anyhow::Result;
use std::path::Path;
use std::fs;
use serde_json::Value;
use tempfile:: TempDir;

fn create_dir_name(base_path: &str, gear_type: &GearType) -> String {
    format!("{base_path}/{gear_type}")
        .to_lowercase()
        .replace(" ", "_")
        .replace("-", "_")
        .replace("'s", "")
}

fn get_object_name(object: &serde_json::Value, index: usize) -> String {
    object["name"]["en"]
        .as_str()
        .map(String::from)
        .unwrap_or(format!("unknown_{}", index))
}

fn create_filename(gear_type: &GearType, object_name: &str) -> String {
    format!("{gear_type}_{object_name}.json")
        .to_lowercase()
        .replace(" ", "_")
        .replace("-", "_")
        .replace("'s", "")
}

pub fn save_gears(
    base_path: &str,
    gear_type: &GearType,
    gears: &Vec<serde_json::Value>
) -> Result<()> {
    let dir_name = create_dir_name(base_path, gear_type);
    let out_dir_path = Path::new(&dir_name);
    fs::create_dir_all(out_dir_path)?;
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
        dir_name
    );
    Ok(())
}

pub fn read_gears(
    base_path: &str,
    gear_type: &GearType
) -> Result<Vec<serde_json::Value>> {
    let dir_name = create_dir_name(base_path, gear_type);
    let in_dir_path = Path::new(&dir_name);
    let mut gears: Vec<serde_json::Value> = vec![];
    for entry in fs::read_dir(in_dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_content = fs::read_to_string(&path)?;
            let json_value: serde_json::Value = serde_json::from_str(&file_content)?;
            gears.push(json_value);
        }
    }
    Ok(gears)
}

// Other variations
pub fn save_dofus_db_data_1(
    objects: &Vec<serde_json::Value>,
    gear_type: GearType
) -> Result<(), Box<dyn Error>> {
    let out_dir = Path::new("dofus_db/data1");
    fs::create_dir_all(out_dir)?;
    for (i, object) in objects.iter().enumerate() {
        // This line requires GearType to implement fmt::Display,
        // see https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
        let file_name = format!("{gear_type}_{i}.json");
        let file_path = out_dir.join(file_name);
        let json_str = serde_json::to_string_pretty(object)?;
        fs::write(file_path, json_str)?;
    }
    Ok(())
}

pub fn save_dofus_db_data_2(
    objects: &Vec<serde_json::Value>,
    gear_type: GearType
) -> Result<()> {
    let out_dir = Path::new("dofus_db/data2");
    fs::create_dir_all(out_dir)?;
    for (i, object) in objects.iter().enumerate() {
        let object_name = get_object_name(object, i);
        let file_name = create_filename(&gear_type, &object_name);
        let file_path = out_dir.join(file_name);
        let json_str = serde_json::to_string_pretty(object)?;
        fs::write(file_path, json_str)?;
    }
    Ok(())
}

#[test]
fn write_read_gears() -> Result<()> {
    // r# negates need to escape quotes in JSON strings
    let json_1 = r#"{ "name": { "en": "Great Amulet", "fr": "Grande Amulette" } }"#;
    let json_2 = r#"{ "foo": "bar" }"#;

    let json_values: Vec<Value> = vec![json_1, json_2]
        .into_iter()
        .map(serde_json::from_str)
        .collect::<Result<Vec<serde_json::Value>, _>>()?;

    let base_dir = TempDir::new()?.path().to_str().unwrap().to_string();
    let gear_type = GearType::Amulet;

    save_gears(&base_dir, &gear_type, &json_values)?;
    let read_json_values = read_gears(&base_dir, &gear_type)?;

    assert_eq!(json_values, read_json_values);

    Ok(())
}