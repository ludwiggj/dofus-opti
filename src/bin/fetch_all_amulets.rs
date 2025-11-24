// A separate binary

// The second step is to compile the binary. The compiler will take main.rs and other
// files corresponding to modules declared in main.rs and try to compile the binary.
// The library is already built at this point, and the library is technically a
// dependency of the binary.

// use packagename::... refers to the library's module hierarchy
// use crate::...       refers to the binary's own module hierarchy

use anyhow::Result;
use dofus_opti::superceded::old_dofus_db_client::fetch_amulets;

async fn fetch_all_amulets() -> reqwest::Result<Vec<serde_json::Value>> {
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

#[tokio::main]
async fn main() -> Result<()> {
    fetch_all_amulets().await?;

    Ok(())
}