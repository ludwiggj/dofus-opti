use crate::dofus_db_models::GetObjectsResponse;

pub(crate) async fn fetch_amulets(skip: u32) ->
reqwest::Result<GetObjectsResponse> {
    let url = format!(
        "https://api.dofusdb.fr/items?typeId[$in][]=1&$sort=-id&$skip={}", skip
    );

    let rsp = reqwest::get(url).await?;

    let data: GetObjectsResponse = rsp.json().await?;

    Ok(data)
}