use crate::dofus_db_models::GetObjectsResponse;
use crate::models::{GearType, gear_type_to_type_id};

pub async fn fetch_amulets(skip: u32) -> reqwest::Result<GetObjectsResponse> {
    let url = format!(
        "https://api.dofusdb.fr/items?typeId[$in][]=1&$sort=-id&$skip={}", skip
    );

    let rsp = reqwest::get(url).await?;

    let data: GetObjectsResponse = rsp.json().await?;

    Ok(data)
}

pub async fn fetch_all_amulets() -> reqwest::Result<Vec<serde_json::Value>> {
    let mut gears: Vec<serde_json::Value> = vec![];

    loop {
        let mut response = fetch_amulets(gears.len() as u32).await?;
        if response.data.is_empty() {
            break;
        } else {
            gears.append(&mut response.data);
        }
    }

    println!("Amulet gear count: {}", gears.len());

    Ok(gears)
}

pub async fn fetch_gear(gear_type: &GearType, skip: u32) -> reqwest::Result<GetObjectsResponse> {
    let type_id = gear_type_to_type_id(gear_type);

    let url = format!(
        "https://api.dofusdb.fr/items?typeId[$in][]={}&$sort=-id&$skip={}", type_id, skip
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