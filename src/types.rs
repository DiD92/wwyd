use serde::Deserialize;
use std::fmt::{Display, Formatter, Result as FormatResult};
use strum_macros::{Display as EnumDisplay, EnumString};

pub type Hand = [Tile; 13];

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, EnumString, EnumDisplay)]
pub enum NumberedSuit {
    #[strum(serialize = "man", serialize = "M", to_string = "Manzu")]
    Characters,
    #[strum(serialize = "pin", serialize = "P", to_string = "Pinzu")]
    Circles,
    #[strum(serialize = "sou", serialize = "S", to_string = "Souzu")]
    Bamboos,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize)]
pub enum HonorSuit {
    Winds,
    Dragons,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Tile {
    Character { number: u8, is_red: bool },
    Circle { number: u8, is_red: bool },
    Bamboo { number: u8, is_red: bool },
    Dragon(DragonColor),
    Wind(WindDirection),
}

impl Tile {
    pub fn new_numbered(suit: NumberedSuit, number: u8, is_red: bool) -> Self {
        match number {
            1..=9 => match suit {
                NumberedSuit::Characters => Self::Character { number, is_red },
                NumberedSuit::Circles => Self::Circle { number, is_red },
                NumberedSuit::Bamboos => Self::Bamboo { number, is_red },
            },
            _ => panic!("Invalid tile number!"),
        }
    }

    pub fn new_wind(direction: WindDirection) -> Self {
        Self::Wind(direction)
    }

    pub fn new_dragon(color: DragonColor) -> Self {
        Self::Dragon(color)
    }

    pub fn is_valid(&self) -> bool {
        fn valid_number(number: &u8) -> bool {
            matches!(number, 1..=9)
        };

        match self {
            Tile::Character { number, .. } => valid_number(number),
            Tile::Circle { number, .. } => valid_number(number),
            Tile::Bamboo { number, .. } => valid_number(number),
            _ => true,
        }
    }
}

#[cfg(test)]
mod tile_tests {

    use super::*;

    #[test]
    #[should_panic]
    fn panics_with_invalid_tile_number() {
        let tile = Tile::new_numbered(NumberedSuit::Bamboos, 13, false);
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            Tile::Character {
                number,
                is_red: red,
            } => {
                write!(f, "Man{}{}", number, if *red { "r" } else { "" })
            }
            Tile::Circle {
                number,
                is_red: red,
            } => {
                write!(f, "Pin{}{}", number, if *red { "r" } else { "" })
            }
            Tile::Bamboo {
                number,
                is_red: red,
            } => {
                write!(f, "Sou{}{}", number, if *red { "r" } else { "" })
            }
            Tile::Wind(direction) => write!(f, "{}", direction.to_string()),
            Tile::Dragon(color) => write!(f, "{}", color.to_string()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, EnumString, EnumDisplay)]
pub enum WindDirection {
    #[strum(serialize = "ton", serialize = "E", to_string = "Ton")]
    East,
    #[strum(serialize = "nan", serialize = "S", to_string = "Nan")]
    South,
    #[strum(serialize = "shaa", serialize = "W", to_string = "Shaa")]
    West,
    #[strum(serialize = "pei", serialize = "N", to_string = "Pei")]
    North,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, EnumString, EnumDisplay)]
pub enum DragonColor {
    #[strum(serialize = "haku", serialize = "W", to_string = "Haku")]
    White,
    #[strum(serialize = "hatsu", serialize = "G", to_string = "Hatsu")]
    Green,
    #[strum(serialize = "chun", serialize = "R", to_string = "Chun")]
    Red,
}
