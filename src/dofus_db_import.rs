use anyhow::Result;
use serde_json::Value as JsonValue;
use serde::Deserialize;
use std::collections::HashSet;
use crate::dofus_db_client::fetch_all_gears;
use crate::dofus_db_file::{filename_safe_string, read_gears, save_gears};
use crate::dofus_db_models::DofusDbObject;
use crate::dofus_db_parser::parse_gears;
use crate::models::{Gear, GearType};


fn extract_gear_file_name(gear: &JsonValue) -> Option<String> {
    gear.pointer("/name/en")
        .and_then(|n| n.as_str())
        .map(filename_safe_string)
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

fn deserialise(gears: &Vec<JsonValue>) -> Vec<DofusDbObject> {
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

pub async fn fetch_and_save_all_gears(gear_type: &GearType) -> Result<()> {
    let dofus_db_export_path: &str = "dofus_db/data";

    let mut gears = fetch_all_gears(gear_type).await?;

    remove_duplicates(&mut gears);

    let dofus_db_objects: Vec<DofusDbObject> = deserialise(& gears);
    
    let mut parsed_gears: Vec<Gear> = parse_gears(gear_type, dofus_db_objects);

    display_parsed_gears(&mut parsed_gears, gear_type);

    save_gears(dofus_db_export_path, gear_type, &gears)?;

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