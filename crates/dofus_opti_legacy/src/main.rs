// The main binary.

// The second step is to compile the binary. The compiler will take main.rs and other 
// files corresponding to modules declared in main.rs and try to compile the binary.
// The library is already built at this point, and the library is technically a
// dependency of the binary.

mod dofus_db_export;
mod dofus_db_import;

// use packagename::... refers to the library's module hierarchy
// use crate::...       refers to the binary's own module hierarchy
use anyhow::Result;
use clap::Parser;
use futures::{stream, StreamExt};
use std::time::Instant;
use crate::dofus_db_export::{export_parsed_data, EXPORT_PATH};
use crate::dofus_db_import::{fetch_and_save_all_gears, IMPORT_PATH};
use core::model::ALL_GEAR_TYPES;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Import data from Dofus DB site and save locally
    #[arg(short = 'i', long = "import")]
    import: bool,

    // Parse local DofusDB json files into Rust models and export them
    #[arg(short = 'e', long = "export")]
    export: bool,
}

// cargo run --bin dofus-opti -- -i
// cargo run --bin dofus-opti -- -e

// See https://www.sheshbabu.com/posts/rust-module-system/ for description of module system
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

        // TODO Cases of paths
        // ✅ Written 383 entry/ies to directory data/import/ring
        // ✅ Successfully read 383 entries from data/import/Ring into json
        // ✅ Successfully processed gear_type Ring
    }

    // read, parse, write out our model representation
    // Example layout:
    // TODO - Adjust paths as needed
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

        // TODO Cases of paths
        // ✅ Successfully read 328 entries from data/import/Amulet into json
        // ✅ Successfully parsed 328/328 from DofusDbObject into Amulet
        // ✅ Written 328 entry/ies to directory data/export/amulet
        // ✅ Successfully processed gear_type Amulet
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