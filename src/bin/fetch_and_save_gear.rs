use anyhow::Result;
use dofus_opti::dofus_db_client::fetch_all_gears;
use dofus_opti::dofus_db_models::DofusDbObject;
use dofus_opti::model_parser::parse_gear;
use dofus_opti::models::{ALL_GEAR_TYPES, GearType};
use futures::{StreamExt, stream};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::time::Instant;

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

fn create_dir_name(gear_type: &GearType) -> String {
    format!("dofus-db/data/{gear_type}")
        .to_lowercase()
        .replace(" ", "_")
        .replace("-", "_")
        .replace("'s", "")
}

fn save_dofus_db_data(objects: &Vec<serde_json::Value>, gear_type: &GearType) -> Result<()> {
    let dir_name = create_dir_name(gear_type);
    let out_dir_path = Path::new(&dir_name);
    fs::create_dir_all(out_dir_path)?;
    for (i, object) in objects.iter().enumerate() {
        let object_name = get_object_name(object, i);
        let file_name = create_filename(gear_type, &object_name);
        let file_path = out_dir_path.join(file_name);
        let json_str = serde_json::to_string_pretty(object)?;
        fs::write(file_path, json_str)?;
    }
    println!(
        "Written {} entry/ies of gear type {} to directory {}",
        objects.len(),
        gear_type,
        dir_name
    );
    Ok(())
}

// This version prints everything in a double iteration:

// let dofus_db_objects: Vec<DofusDbObject> = result.iter()
//     .map(|x| DofusDbObject::deserialize(x).unwrap()).collect();
//
// for dofus_db_object in dofus_db_objects {
//     println!("{:?}", parse_gear(dofus_db_object));
// }
pub async fn fetch_and_save_all_gears(gear_type: &GearType) -> Result<()> {
    let result = fetch_all_gears(gear_type).await?;

    println!("First 10 gear of type: {}", gear_type);
    result
        .iter()
        .take(10)
        .for_each(|x| println!("  {:?}", parse_gear(DofusDbObject::deserialize(x).unwrap())));

    save_dofus_db_data(&result, gear_type)
}

#[tokio::main]
async fn main() -> Result<()> {
    let now = Instant::now();

    // Calling it sequentially
    // for gear_type in ALL_GEAR_TYPES {
    //     if let Err(e) = fetch_and_save_all_gears(gear_type).await {
    //         eprintln!("❌ Failed to process gear_type {}: {}", gear_type, e);
    //     } else {
    //         println!("✅ Successfully processed gear_type {}", gear_type);
    //     }
    // }

    // Calling it concurrently
    const MAX_CONCURRENCY: usize = 5;

    stream::iter(ALL_GEAR_TYPES)
        .for_each_concurrent(MAX_CONCURRENCY, |gear_type| async move {
            if let Err(e) = fetch_and_save_all_gears(gear_type).await {
                eprintln!("❌ Failed to process gear_type {}: {}", gear_type, e);
            } else {
                println!("✅ Successfully processed gear_type {}", gear_type);
            }
        }).await;

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
