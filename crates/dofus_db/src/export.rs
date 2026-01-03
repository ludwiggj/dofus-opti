use anyhow::Result;
use serde_json::Value;
use core::model::{Gear, GearType};
use core::file::{read_gears, save_gears};
use crate::model::DofusDbObject;
use crate::parser::parse_gears;

pub const EXPORT_PATH: &str = "data/export";

fn file_name() -> fn(&Value, usize) -> String {
    |o, i| (&o["name"]).as_str().map(String::from).unwrap_or(format!("unknown_{}", i))
}

pub fn export_parsed_data(import_path: &str, export_path: &str, gear_type: &GearType) -> Result<()> {
    let dofus_db_objects: Vec<DofusDbObject> = read_gears(import_path, gear_type)?;
    let parsed_gears: Vec<Gear> = parse_gears(gear_type, dofus_db_objects);
    save_gears(export_path, gear_type, &parsed_gears, file_name())
}