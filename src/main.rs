// The main binary.

// The second step is to compile the binary. The compiler will take main.rs and other 
// files corresponding to modules declared in main.rs and try to compile the binary.
// The library is already built at this point, and the library is technically a
// dependency of the binary.

// use packagename::... refers to the library's module hierarchy
// use crate::...       refers to the binary's own module hierarchy
use anyhow::Result;
use dofus_opti::dofus_db_export::{export_parsed_data, EXPORT_PATH};
use dofus_opti::dofus_db_import::{fetch_and_save_all_gears, IMPORT_PATH};
use dofus_opti::models::ALL_GEAR_TYPES;
use futures::{stream, StreamExt};
use std::time::Instant;
use clap::Parser;

// The following modules are part of the binary only
// They cannot be used by the library itself, or rust code that imports this library
// Although pub can be used here, it's best practice not to, since no external code
// can import from the binary crate
mod args;

use args::Args;

// cargo run --bin dofus-opti -- -e
// cargo run --bin dofus-opti -- -i
#[tokio::main]
async fn main() -> Result<()> {
    let now = Instant::now();

    let args = Args::parse();

    async fn fetch_and_save() {
        const MAX_CONCURRENCY: usize = 5;
        stream::iter(ALL_GEAR_TYPES)
            .for_each_concurrent(MAX_CONCURRENCY, |gear_type| async move {
                if let Err(e) = fetch_and_save_all_gears(IMPORT_PATH, gear_type).await {
                    eprintln!("❌ Failed to process gear_type {}: {}", gear_type, e);
                } else {
                    println!("✅ Successfully processed gear_type {}", gear_type);
                };
                println!("=================================================");
            }).await;
    }

    if args.import {
        println!("Importing data from Dofus DB site...");
        fetch_and_save().await;
    }

    // read, parse, write out our model representation
    // Example layout:
    //
    // core/
    // └── data/
    //     ├── Amulet/
    //     │   ├── aerdala_amulet.json
    //     │   ├── helsephine_love.json
    //     │   └── ...
    //     ├── Belt/
    //     │   ├── minotoror.json
    //     │   ├── ogivol.json
    //     │   └── ...

    async fn export_data() {
        const MAX_CONCURRENCY: usize = 5;
        stream::iter(ALL_GEAR_TYPES)
            .for_each_concurrent(MAX_CONCURRENCY, |gear_type| async move {
                if let Err(e) = export_parsed_data(IMPORT_PATH, EXPORT_PATH, gear_type) {
                    eprintln!("❌ Failed to process gear_type {}: {}", gear_type, e);
                } else {
                    println!("✅ Successfully processed gear_type {}", gear_type);
                };
                println!("=================================================");
            }).await;
    }

    if args.export {
        println!("Exporting Dofus DB data to our model...");
        export_data().await;
    }

    if !args.import && !args.export {
        println!("No action specified. Use -i to import or -e to export.");
        println!("Example: cargo run --bin dofus-opti -- -i");
        println!("Importing as default action...\n");

        // fetch_and_save_all_gears(IMPORT_PATH, &GearType::Belt).await.unwrap_or_default();
        // export_parsed_data(EXPORT_PATH, &GearType::Belt).unwrap_or_default();

        // fetch_and_save().await;
        // export_data().await;
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}