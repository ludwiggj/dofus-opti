mod models;
mod dofus_db_models;
mod dofus_db_client;
mod model_parser;

use anyhow::Result;
use serde::Deserialize;
use crate::dofus_db_client::fetch_amulets;
use crate::dofus_db_models::DofusDbObject;
use crate::model_parser::parse_gear;
use crate::models::{CharacteristicRange, CharacteristicType, Gear, GearType};

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

    println!("Example of a gear: {:?}", crocoshield);
}

#[tokio::main]
async fn main() -> Result<()> {
    show_croco_shield();

    let result = fetch_amulets(0).await?;

    println!("{:#?}", result);

    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
    let blah: Vec<DofusDbObject> = result.data.iter()
        // .map(|x| (serde_json::from_str(x.to_string().as_str())).unwrap()).collect();
        .map(|x| DofusDbObject::deserialize(x).unwrap()).collect();

    //let r2: Vec<DofusDbObject> = serde_json::from_str(result.data);

    // idiomatic - see https://doc.rust-lang.org/rust-by-example/flow_control/for.html
    for data in blah {
        println!("{:?}", parse_gear(data));
    }

    // Can we just process one item?

    // Nope - see https://stackoverflow.com/questions/27904864/what-does-cannot-move-out-of-index-of-mean

    // let bob = result.data[0];

    // Type mismatch [E0308]
    // Expected:
    //     dofus_opti::dofus_db_models::DofusDbObject
    // Found:
    //     Option<dofus_opti::dofus_db_models::DofusDbObject>

    // let result2 = parse_gear(result.data.into_iter().nth(1));

    // This "works" if change signature of parse_gear to:
    //   fn parse_gear(object: &DofusDbObject) -> Result<Gear, String>
    // but that has implications - other parts of code do not compile

    // let bob = &result.data[0];
    // let result2 = parse_gear(bob);

    // This works - remove item from the array
    // let mut data = result.data;
    // println!("{:?}", parse_gear(data.remove(1)));

    Ok(())
}
