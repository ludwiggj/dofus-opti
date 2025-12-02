use anyhow::Result;
use crate::dofus_db_file::{read_gears, save_gears};
use crate::dofus_db_models::DofusDbObject;
use crate::dofus_db_parser::parse_gears;
use crate::models::{Gear, GearType};

pub const EXPORT_PATH: &str = "core/data";

pub fn export_parsed_data(import_path: &str, export_path: &str, gear_type: &GearType) -> Result<()> {
    let dofus_db_objects: Vec<DofusDbObject> = read_gears(import_path, gear_type)?;
    let parsed_gears: Vec<Gear> = parse_gears(gear_type, dofus_db_objects);
    save_gears(export_path, gear_type, &parsed_gears, |o| &o["name"])
}