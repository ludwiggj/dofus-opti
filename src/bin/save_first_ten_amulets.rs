// A separate binary

// The second step is to compile the binary. The compiler will take main.rs and other
// files corresponding to modules declared in main.rs and try to compile the binary.
// The library is already built at this point, and the library is technically a
// dependency of the binary.

// use packagename::... refers to the library's module hierarchy
// use crate::...       refers to the binary's own module hierarchy

use anyhow::Result;
use dofus_opti::dofus_db_models::DofusDbObject;
use dofus_opti::model_parser::parse_gear;
use dofus_opti::models::GearType;
use dofus_opti::superceded::dofus_db_client:: fetch_amulets;
use dofus_opti::superceded::dofus_db_file::{save_dofus_db_data_1, save_dofus_db_data_2};
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<()> {
    let result = fetch_amulets(0).await?;

    // Debug format
    // https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html
    // println!("{:#?}", result);

    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
    let dofus_db_objects: Vec<DofusDbObject> = result.data.iter()
        .map(|x| DofusDbObject::deserialize(x).unwrap()).collect();

    // idiomatic - see https://doc.rust-lang.org/rust-by-example/flow_control/for.html
    for dofus_db_object in dofus_db_objects {
        println!("{:#?}", parse_gear(dofus_db_object));
    }

    // now let's write the files out (twice!)

    // If you want to read more about the ? in Rust, Alex Garella wrote a helpful article,
    // https://rustjobs.dev/blog/the-question-mark-operator-in-rust/
    save_dofus_db_data_1(&result.data, GearType::Amulet).unwrap();
    save_dofus_db_data_2(&result.data, GearType::Amulet)?;

    // There are other ways to handle the above e.g. redefine main signature as:
    // fn main() -> Result<(), Box<dyn Error>>

    Ok(())
}