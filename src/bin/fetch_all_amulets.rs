use anyhow::Result;
use dofus_opti::dofus_db_client::fetch_all_amulets;

#[tokio::main]
async fn main() -> Result<()> {
    let next_result = fetch_all_amulets().await?;

    Ok(())
}