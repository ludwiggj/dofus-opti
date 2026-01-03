use anyhow::Result;
use serde_json::{Value as JsonValue, Value};
use std::collections::HashSet;
use serde::Deserialize;
use core::model::{Gear, GearType};
use dofus_db::client::fetch_all_gears;
use dofus_db::dofus_db_file::{filename_safe_string, read_gears, write_objects};
use dofus_db::model::DofusDbObject;
use dofus_db::parser::parse_gears;

pub const IMPORT_PATH: &str = "data/import";

fn extract_gear_file_name(gear: &JsonValue) -> Option<String> {
    gear.pointer("/name/en")
        .and_then(|n| n.as_str())
        .map(|s| filename_safe_string(String::from(s)))
}

fn remove_duplicates(values: &mut Vec<JsonValue>) {
    let original_count = values.len();
    let mut seen = HashSet::new();
    values.retain(|v| {
        let key = extract_gear_file_name(v);
        if let Some(k) = key {
            if seen.contains(&k) {
                println!("❌ Discarded duplicate gear: {}", k);
                false
            } else {
                seen.insert(k);
                true
            }
        } else {
            // If no name, discard the item
            false
        }
    });

    let deduplicated_count = values.len();

    println!(
        "✅ Kept {} / {} gears ({} duplicates)",
        deduplicated_count, original_count, original_count - deduplicated_count
    );
}

pub fn deserialise(gears: &Vec<JsonValue>) -> Vec<DofusDbObject> {
    let dofus_db_objects: Vec<_> = gears.iter()
        .filter_map(|x| DofusDbObject::deserialize(x).ok())
        .collect();
    
    println!(
        "✅ Successfully deserialised: {} / {} into DofusDbObject",
        dofus_db_objects.len(), gears.len()
    );
    
    dofus_db_objects
}

fn display_parsed_gears(gears: &mut Vec<Gear>, gear_type: &GearType) {
    gears.sort_by_key(|g| g.name.clone());

    println!("Parsed {}s (alphabetical order):", gear_type);
    gears.iter().for_each(|g| println!("  > {}", g.name));
}

pub async fn fetch_and_save_all_gears(dofus_db_export_path: &str, gear_type: &GearType) -> Result<()> {
    let mut gears = fetch_all_gears(gear_type).await?;

    remove_duplicates(&mut gears);

    let dofus_db_objects: Vec<DofusDbObject> = deserialise(& gears);
    
    let mut parsed_gears: Vec<Gear> = parse_gears(gear_type, dofus_db_objects);

    display_parsed_gears(&mut parsed_gears, gear_type);

    let folder_name = gear_type.to_string().to_lowercase();

    fn file_name() -> fn(&Value, usize) -> String {
        |o, i| (&o["name"]["en"]).as_str().map(String::from).unwrap_or(format!("unknown_{}", i))
    }

    write_objects(
        dofus_db_export_path,
        folder_name,
        &gears,
        file_name(),
    )?;

    read_gears(dofus_db_export_path, gear_type)?;
    
    Ok(())
}

// Old version: prints everything in a double iteration...

// let dofus_db_objects: Vec<DofusDbObject> = result.iter()
//     .map(|x| DofusDbObject::deserialize(x).unwrap()).collect();
//
// for dofus_db_object in dofus_db_objects {
//     println!("{:?}", parse_gear(dofus_db_object));
// }