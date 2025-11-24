// Part of the library. When the crate is compiled, the first step is to compile the library.

// use crate::... refers to the library's module hierarchy.
// use packagename::... will not work inside the library part of the crate because only the
// names of dependencies are available at the top level.

use crate::dofus_db_models::{DofusDbTypeId, GetObjectsResponse};
use crate::models::GearType;

pub async fn fetch_gear(gear_type: &GearType, skip: u32) -> reqwest::Result<GetObjectsResponse> {
    let type_id = DofusDbTypeId::from(gear_type);

    let url = format!(
        "https://api.dofusdb.fr/items?typeId[$in][]={}&$sort=-id&$skip={}", type_id.0, skip
    );

    let rsp = reqwest::get(url).await?;

    let data: GetObjectsResponse = rsp.json().await?;

    Ok(data)
}

pub async fn fetch_all_gears(gear_type: &GearType) -> reqwest::Result<Vec<serde_json::Value>> {
    let mut gears: Vec<serde_json::Value> = vec![];

    loop {
        let mut response = fetch_gear(gear_type, gears.len() as u32).await?;
        if response.data.is_empty() {
            break;
        } else {
            gears.append(&mut response.data);
        }
    }

    println!("{} gear count: {}", gear_type, gears.len());

    Ok(gears)
}