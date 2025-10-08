#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate)  struct GetObjectsResponse {
    total: u32,
    limit: u32,
    skip: u32,
    data: Vec<DofusDbObject>
}

#[derive(Debug, Deserialize)]
struct DofusDbObject {
    name: TranslatedString,
    typeId: i32,
    level: u32,
    img: String,
    effects: Vec<Effect>
}

#[derive(Debug, Deserialize)]
struct TranslatedString {
    en: String,
    fr: String
}

#[derive(Debug, Deserialize)]
struct Effect {
    from: i32,
    to: i32,
    characteristic: i32
}