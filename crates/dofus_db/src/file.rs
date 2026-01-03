// File utilities for Dofus DB

use anyhow::Result;
use serde_json::Value as JsonValue;
use std::fs;
use std::path::Path;

pub fn read_json<P: AsRef<Path>>(path: P) -> Result<JsonValue> {
    let file_content = fs::read_to_string(path)?;
    let json_value: JsonValue = serde_json::from_str(&file_content)?;
    Ok(json_value)
}