// A separate binary

// The second step is to compile the binary. The compiler will take main.rs and other 
// files corresponding to modules declared in main.rs and try to compile the binary.
// The library is already built at this point, and the library is technically a
// dependency of the binary.

// use packagename::... refers to the library's module hierarchy
// use crate::...       refers to the binary's own module hierarchy

use anyhow::Result;
use dofus_opti::models::{CharacteristicRange, CharacteristicType, Gear, GearType};

fn show_croco_shield() {
    let crocoshield = Gear {
        name: String::from("Crocoshield"),
        gear_type: GearType::Amulet,
        level: 200,
        characteristics: vec![
            CharacteristicRange {
                kind: CharacteristicType::Vitality,
                min: 201,
                max: 250,
            },
            CharacteristicRange {
                kind: CharacteristicType::Power,
                min: 41,
                max: 50,
            }
        ],
    };

    println!("Example of a gear: {:#?}", crocoshield);
}

#[tokio::main]
async fn main() -> Result<()> {
    show_croco_shield();
    Ok(())
}