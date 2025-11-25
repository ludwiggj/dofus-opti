// A separate binary

// The second step is to compile the binary. The compiler will take main.rs and other
// files corresponding to modules declared in main.rs and try to compile the binary.
// The library is already built at this point, and the library is technically a
// dependency of the binary.

// use packagename::... refers to the library's module hierarchy
// use crate::...       refers to the binary's own module hierarchy
use anyhow::Result;
use dofus_opti::dofus_db_import::fetch_and_save_all_gears;
use dofus_opti::models::ALL_GEAR_TYPES;
use std::time::Instant;

// See https://www.sheshbabu.com/posts/rust-module-system/ for description of module system
#[tokio::main]
async fn main() -> Result<()> {
    let now = Instant::now();

    // Calling it sequentially
    for gear_type in ALL_GEAR_TYPES {
        if let Err(e) = fetch_and_save_all_gears(gear_type).await {
            eprintln!("❌ Failed to process gear_type {}: {}", gear_type, e);
        } else {
            println!("✅ Successfully processed gear_type {}", gear_type);
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}