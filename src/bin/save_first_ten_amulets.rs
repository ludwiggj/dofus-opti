use anyhow::Result;
use dofus_opti::dofus_db_client::fetch_amulets;
use dofus_opti::dofus_db_models::DofusDbObject;
use dofus_opti::model_parser::parse_gear;
use dofus_opti::models::GearType;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

fn save_dofus_db_data(
    objects: &Vec<serde_json::Value>,
    gear_type: GearType
) -> Result<(), Box<dyn Error>> {
    let out_dir = Path::new("dofus-db/data");
    fs::create_dir_all(out_dir)?;
    for (i, object) in objects.iter().enumerate() {
        // This line requires GearType to implement fmt::Display,
        // see https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
        let file_name = format!("{gear_type}_{i}.json");
        let file_path = out_dir.join(file_name);
        let json_str = serde_json::to_string_pretty(object)?;
        fs::write(file_path, json_str)?;
    }
    Ok(())
}

fn get_object_name(object: &serde_json::Value, index: usize) -> String {
    object["name"]["en"]
        .as_str()
        .map(String::from)
        .unwrap_or(format!("unknown_{}", index))
}

fn create_filename(gear_type: &GearType, object_name: &str) -> String {
    format!("{gear_type}_{object_name}.json")
        .to_lowercase()
        .replace(" ", "_")
        .replace("-", "_")
        .replace("'s", "")
}

fn save_dofus_db_data_2(
    objects: &Vec<serde_json::Value>,
    gear_type: GearType
) -> Result<()> {
    let out_dir = Path::new("dofus-db/data2");
    fs::create_dir_all(out_dir)?;
    for (i, object) in objects.iter().enumerate() {
        let object_name = get_object_name(object, i);
        let file_name = create_filename(&gear_type, &object_name);
        let file_path = out_dir.join(file_name);
        let json_str = serde_json::to_string_pretty(object)?;
        fs::write(file_path, json_str)?;
    }
    Ok(())
}

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
    save_dofus_db_data(&result.data, GearType::Amulet).unwrap();
    save_dofus_db_data_2(&result.data, GearType::Amulet)?;

    // There are other ways to handle the above e.g. redefine main signature as:
    // fn main() -> Result<(), Box<dyn Error>>

    Ok(())
}