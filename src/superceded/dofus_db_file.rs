use anyhow::Result;
use crate::dofus_db_file::{create_filename, get_object_name};
use crate::models::GearType;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn save_dofus_db_data_1(
    objects: &Vec<Value>,
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
    objects: &Vec<Value>,
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