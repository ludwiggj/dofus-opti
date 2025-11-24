// Part of the library. When the crate is compiled, the first step is to compile the library.

// use crate::... refers to the library's module hierarchy.
// use packagename::... will not work inside the library part of the crate because only the
// names of dependencies are available at the top level.

use crate::dofus_db_models::GetObjectsResponse;

pub async fn fetch_amulets(skip: u32) -> reqwest::Result<GetObjectsResponse> {
    let url = format!(
        "https://api.dofusdb.fr/items?typeId[$in][]=1&$sort=-id&$skip={}", skip
    );

    let rsp = reqwest::get(url).await?;

    let data: GetObjectsResponse = rsp.json().await?;

    Ok(data)
}