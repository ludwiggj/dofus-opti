// A separate binary

// The second step is to compile the binary. The compiler will take main.rs and other
// files corresponding to modules declared in main.rs and try to compile the binary.
// The library is already built at this point, and the library is technically a
// dependency of the binary.

// use packagename::... refers to the library's module hierarchy
// use crate::...       refers to the binary's own module hierarchy

use anyhow::Result;
use dofus_opti::dofus_db_client::fetch_all_gears;
use dofus_opti::dofus_db_file::{read_gears, save_gears};
use dofus_opti::dofus_db_models::DofusDbObject;
use dofus_opti::model_parser::parse_gear;
use dofus_opti::models::{ALL_GEAR_TYPES, GearType};
use futures::{StreamExt, stream};
use serde::Deserialize;
use serde_json::to_string_pretty;
use std::time::Instant;

const DOFUS_DB_EXPORT_PATH: &str = "dofus_db/data";

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

    save_gears(DOFUS_DB_EXPORT_PATH, gear_type, &result)?;

    println!("First gear of type: {} read back from disk", gear_type);

    // read_gears(gear_type)?.iter().take(1).for_each(|x| println!("  {:?}", x));

    read_gears(DOFUS_DB_EXPORT_PATH, gear_type)?
        .iter()
        .take(1)
        .for_each(|x| match to_string_pretty(x) {
            Ok(json) => println!("  {}", json),
            Err(e) => eprintln!("  Failed to serialize gear: {}", e),
        });

    Ok(())
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
