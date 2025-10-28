#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetObjectsResponse {
    total: u32,
    limit: u32,
    skip: u32,
    pub data: Vec<serde_json::Value>
}

#[derive(Debug, Deserialize)]
pub struct DofusDbObject {
    pub name: TranslatedString,
    pub typeId: i32,
    pub level: u32,
    img: String,
    pub effects: Vec<Effect>
}

#[derive(Debug, Deserialize)]
pub struct TranslatedString {
    pub en: String,
    fr: String
}

#[derive(Debug, Deserialize)]
pub struct Effect {
    pub from: i32,
    pub to: i32,
    pub characteristic: i32
}