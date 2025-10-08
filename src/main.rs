mod models;
mod dofus_db_models;
mod dofus_db_client;

use models::{CharacteristicRange, CharacteristicType, Gear, GearType};
use anyhow::Result;
use dofus_db_client::fetch_amulets;

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

    Ok(())
}
