// The main binary.

// The second step is to compile the binary. The compiler will take main.rs and other 
// files corresponding to modules declared in main.rs and try to compile the binary.
// The library is already built at this point, and the library is technically a
// dependency of the binary.

// use packagename::... refers to the library's module hierarchy
// use crate::...       refers to the binary's own module hierarchy
use anyhow::Result;
use clap::Parser;
use dofus_opti::dofus_db_import::fetch_and_save_all_gears;
use dofus_opti::models::ALL_GEAR_TYPES;
use futures::{stream, StreamExt};
use std::time::Instant;

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

// cargo run --bin dofus-opti -- -e
// cargo run --bin dofus-opti -- -i

// See https://www.sheshbabu.com/posts/rust-module-system/ for description of module system
#[tokio::main]
async fn main() -> Result<()> {
    let now = Instant::now();

    let args = Args::parse();

    if args.import {
        println!("Importing data from Dofus DB site...");

        // fetch and save
        const MAX_CONCURRENCY: usize = 5;
        stream::iter(ALL_GEAR_TYPES)
            .for_each_concurrent(MAX_CONCURRENCY, |gear_type| async move {
                if let Err(e) = fetch_and_save_all_gears(gear_type).await {
                    eprintln!("❌ Failed to process gear_type {}: {}", gear_type, e);
                } else {
                    println!("✅ Successfully processed gear_type {}", gear_type);
                }
            }).await;
    }

    if args.export {
        println!("Exporting Dofus DB data to our model...");

        // read, parse, write
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}