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
    irregular_shape: Option<Vec<ShapeGroup>>,
}

impl HandRestrictions {
    #[inline]
    pub fn can_contain_honors(&self) -> bool {
        !matches!(self.honor_tiles, Some(TileRequirement::Forbidden))
    }

    pub fn usable_honor_variants_count(&self) -> Option<u8> {
        if self.can_contain_honors() {
            self.honor_variants.or(Some(7))
        } else {
            None
        }
    }

    pub fn wind_directions_allowed(&self) -> Option<&Vec<WindDirection>> {
        if self.can_contain_honors() {
            self.honor_wind_directions_allowed.as_ref()
        } else {
            None
        }
    }

    pub fn dragon_colors_allowed(&self) -> Option<&Vec<DragonColor>> {
        if self.can_contain_honors() {
            self.honor_dragon_colors_allowed.as_ref()
        } else {
            None
        }
    }

    #[inline]
    pub fn can_contain_suits(&self) -> bool {
        !matches!(self.suit_tiles, Some(TileRequirement::Forbidden))
    }

    pub fn usable_suit_variants_count(&self) -> Option<u8> {
        if self.can_contain_suits() {
            self.honor_variants.or(Some(3))
        } else {
            None
        }
    }

    pub fn suit_variants_allowed(&self) -> Option<&Vec<NumberedSuit>> {
        if self.can_contain_suits() {
            self.suit_variants_allowed.as_ref()
        } else {
            None
        }
    }

    pub fn suit_numbers_allowed(&self) -> Option<&Vec<u8>> {
        if self.can_contain_suits() {
            self.suit_numbers_allowed.as_ref()
        } else {
            None
        }
    }

    pub fn shapes_allowed(&self) -> Option<&Vec<TileGroup>> {
        self.shapes_allowed.as_ref()
    }

    pub fn hand_shape(&self) -> HandShape {
        self.hand_shape
    }

    pub fn irregular_shape_set(&self) -> Option<&Vec<ShapeGroup>> {
        match self.hand_shape {
            HandShape::Regular => None,
            _ => self.irregular_shape.as_ref(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TileRequirement {
    Required,
    Optional,
    Forbidden,
}

#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum HandShape {
    Regular,
    Irregular,
    Both,
}

#[derive(Deserialize, Debug, Copy, Clone, EnumString, EnumDisplay)]
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
    #[strum(serialize = "shinguru", serialize = "1", to_string = "Shinguru")]
    Shinguru,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct ShapeGroup {
    group_type: TileGroup,
    group_count: u8,
}
