mod models;

use models::{CharacteristicRange, CharacteristicType, Gear, GearType};

fn main() {
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
