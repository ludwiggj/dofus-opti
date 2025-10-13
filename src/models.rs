#[derive(Debug)]
pub(crate) struct Gear {
    pub(crate) name: String,
    pub(crate) gear_type: GearType,
    pub(crate) level: u32,
    pub(crate) characteristics: Vec<CharacteristicRange>
}

#[derive(Debug)]
pub(crate) enum GearType {
    Amulet,
    Hat,
    Ring,
    Shield
    // Add all other types
}

#[derive(Debug)]
pub(crate) struct CharacteristicRange {
    pub(crate) kind: CharacteristicType,
    pub(crate) min: i32,
    pub(crate) max: i32
}

#[derive(Debug)]
pub(crate) enum CharacteristicType {
    Vitality,
    Power
    // Add all other types
}