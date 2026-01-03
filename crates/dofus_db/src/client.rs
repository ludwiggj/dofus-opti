// Dofus DB client implementation

use crate::model::{DofusDbTypeId, GetObjectsResponse};
use core::model::GearType;

use serde_json::{Value as JsonValue};

pub async fn fetch_gear(gear_type: &GearType, skip: u32) -> reqwest::Result<GetObjectsResponse> {
    let type_id = DofusDbTypeId::from(gear_type);

    let url = format!(
        "https://api.dofusdb.fr/items?typeId[$in][]={}&$sort=-id&$skip={}", type_id.0, skip
    );

    let rsp = reqwest::get(url).await?;

    let data: GetObjectsResponse = rsp.json().await?;

    Ok(data)
}

pub async fn fetch_all_gears(gear_type: &GearType) -> reqwest::Result<Vec<JsonValue>> {
    let mut gears: Vec<JsonValue> = vec![];

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
