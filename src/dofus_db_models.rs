#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate)  struct GetObjectsResponse {
    total: u32,
    limit: u32,
    skip: u32,
    pub(crate) data: Vec<DofusDbObject>
}

#[derive(Debug, Deserialize)]
pub(crate) struct DofusDbObject {
    pub(crate) name: TranslatedString,
    pub(crate) typeId: i32,
    pub(crate) level: u32,
    img: String,
    pub(crate) effects: Vec<Effect>
}

#[derive(Debug, Deserialize)]
pub(crate) struct TranslatedString {
    pub(crate) en: String,
    fr: String
}

#[derive(Debug, Deserialize)]
pub(crate) struct Effect {
    pub(crate) from: i32,
    pub(crate) to: i32,
    pub(crate) characteristic: i32
}