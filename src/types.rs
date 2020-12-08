use std::fmt::{Display, Formatter, Result as FormatResult};

pub type Hand = [Tile; 13];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NumberedSuit {
    Characters,
    Circles,
    Bamboos,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HonorSUit {
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
            match number {
                1..=9 => true,
                _ => false,
            }
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

impl std::fmt::Display for Tile {
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
            Tile::Wind(direction) => write!(f, "{}", direction),
            Tile::Dragon(color) => write!(f, "{}", color),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WindDirection {
    East,
    South,
    West,
    North,
}

impl std::fmt::Display for WindDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            WindDirection::East => write!(f, "Ton"),
            WindDirection::South => write!(f, "Nan"),
            WindDirection::West => write!(f, "Shaa"),
            WindDirection::North => write!(f, "Pei"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DragonColor {
    White,
    Green,
    Red,
}

impl std::fmt::Display for DragonColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            DragonColor::White => write!(f, "Haku"),
            DragonColor::Green => write!(f, "Hatsu"),
            DragonColor::Red => write!(f, "Chun"),
        }
    }
}
