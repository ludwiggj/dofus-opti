use serde_json::{to_string_pretty};
use serde::Deserialize;
use crate::dofus_db_client::fetch_all_gears;
use crate::dofus_db_file::{read_gears, save_gears};
use crate::dofus_db_models::DofusDbObject;
use crate::dofus_db_parser::parse_gear;
use crate::models::GearType;

pub async fn fetch_and_save_all_gears(gear_type: &GearType) -> anyhow::Result<()> {
    let dofus_db_export_path: &str = "dofus_db/data";

    let result = fetch_all_gears(gear_type).await?;

    println!("First 10 gear of type: {}", gear_type);
    result
        .iter()
        .take(10)
        .for_each(|x| println!("  {:?}", parse_gear(DofusDbObject::deserialize(x).unwrap())));

    save_gears(dofus_db_export_path, gear_type, &result)?;

    println!("First gear of type: {} read back from disk", gear_type);

    read_gears(dofus_db_export_path, gear_type)?
        .iter()
        .take(1)
        .for_each(|x| match to_string_pretty(x) {
            Ok(json) => println!("  {}", json),
            Err(e) => eprintln!("  Failed to serialize gear: {}", e),
        });

    Ok(())
}

// Old version: prints everything in a double iteration...

// let dofus_db_objects: Vec<DofusDbObject> = result.iter()
//     .map(|x| DofusDbObject::deserialize(x).unwrap()).collect();
//
// for dofus_db_object in dofus_db_objects {
//     println!("{:?}", parse_gear(dofus_db_object));
// }