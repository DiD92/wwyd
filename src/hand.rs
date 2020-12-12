use crate::types::{DragonColor, Hand, NumberedSuit, WindDirection};
use serde::Deserialize;
use strum_macros::{Display as EnumDisplay, EnumString};

trait HandBuilder {
    fn build(shanten: u8) -> Hand;
}

#[derive(Deserialize)]
pub struct HandList {
    #[serde(rename = "hand")]
    pub hands: Vec<HandDefinition>,
}

#[derive(Deserialize, Debug)]
pub struct HandDefinition {
    name: String,
    kanji: Option<String>,
    description: Option<String>,
    restrictions: HandRestrictions,
}

#[derive(Deserialize, Debug)]
pub struct HandRestrictions {
    honor_tiles: Option<TileRequirement>,
    honor_variants: Option<u8>,
    #[serde(deserialize_with = "crate::serialization::deserialize_wind_direction")]
    honor_wind_directions_allowed: Option<Vec<WindDirection>>,
    #[serde(deserialize_with = "crate::serialization::deserialize_dragon_color")]
    honor_dragon_colors_allowed: Option<Vec<DragonColor>>,
    suit_tiles: Option<TileRequirement>,
    suit_variants: Option<u8>,
    #[serde(deserialize_with = "crate::serialization::deserialize_suit_variant")]
    suit_variants_allowed: Option<Vec<NumberedSuit>>,
    suit_numbers_allowed: Option<Vec<u8>>,
    #[serde(deserialize_with = "crate::serialization::deserialize_tile_group")]
    shapes_allowed: Option<Vec<TileGroup>>,
    hand_shape: HandShape,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TileRequirement {
    Required,
    Optional,
    Forbidden,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum HandShape {
    Regular,
    Irregular,
    Both,
}

#[derive(Deserialize, Debug, EnumString, EnumDisplay)]
#[serde(rename_all = "lowercase")]
pub enum TileGroup {
    #[strum(serialize = "shuntsu", serialize = "123", to_string = "Shuntsu")]
    Shuntsu,
    #[strum(serialize = "koutsu", serialize = "111", to_string = "Koutsu")]
    Koutsu,
    #[strum(serialize = "kantsu", serialize = "1111", to_string = "Kantsu")]
    Kantsu,
    #[strum(serialize = "jantou", serialize = "11", to_string = "Jantou")]
    Jantou,
}
