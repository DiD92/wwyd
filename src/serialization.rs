use crate::hand::TileGroup;
use crate::types::{DragonColor, NumberedSuit, WindDirection};
use serde::de::{SeqAccess, Visitor};
use serde::Deserializer;
use std::fmt::{Formatter, Result as FormatResult};
use std::marker::PhantomData;
use std::str::FromStr;

trait EnumExpander {
    fn get_message() -> String;
    fn parse_single(item: &str) -> Self
    where
        Self: Sized;

    fn get_all_variants() -> Vec<Self>
    where
        Self: Sized;
}

impl EnumExpander for WindDirection {
    fn get_message() -> String {
        "Message".to_string()
    }

    fn parse_single(item: &str) -> WindDirection {
        WindDirection::from_str(item).unwrap()
    }

    fn get_all_variants() -> Vec<WindDirection> {
        vec![
            WindDirection::East,
            WindDirection::South,
            WindDirection::West,
            WindDirection::North,
        ]
    }
}

impl EnumExpander for DragonColor {
    fn get_message() -> String {
        "Message".to_string()
    }

    fn parse_single(item: &str) -> DragonColor {
        DragonColor::from_str(item).unwrap()
    }

    fn get_all_variants() -> Vec<DragonColor> {
        vec![DragonColor::White, DragonColor::Green, DragonColor::Red]
    }
}

impl EnumExpander for NumberedSuit {
    fn get_message() -> String {
        "Message".to_string()
    }

    fn parse_single(item: &str) -> NumberedSuit {
        NumberedSuit::from_str(item).unwrap()
    }

    fn get_all_variants() -> Vec<NumberedSuit> {
        vec![
            NumberedSuit::Characters,
            NumberedSuit::Circles,
            NumberedSuit::Bamboos,
        ]
    }
}

impl EnumExpander for TileGroup {
    fn get_message() -> String {
        "Message".to_string()
    }

    fn parse_single(item: &str) -> TileGroup {
        TileGroup::from_str(item).unwrap()
    }

    fn get_all_variants() -> Vec<TileGroup> {
        vec![
            TileGroup::Jantou,
            TileGroup::Shuntsu,
            TileGroup::Koutsu,
            TileGroup::Kantsu,
        ]
    }
}

struct TileParser<T>
where
    T: EnumExpander,
{
    _marker: PhantomData<T>,
}

impl<T> TileParser<T>
where
    T: EnumExpander,
{
    pub fn new() -> Self {
        TileParser {
            _marker: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for TileParser<T>
where
    T: EnumExpander,
{
    type Value = Option<Vec<T>>;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        write!(formatter, "{}", T::get_message())
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, <A as SeqAccess<'de>>::Error>
    where
        A: SeqAccess<'de>,
    {
        let size_hint = seq.size_hint().unwrap();

        if size_hint == 0 {
            Ok(Some(Vec::new()))
        } else {
            let mut vec: Vec<T> = Vec::with_capacity(size_hint);

            loop {
                let tmp = seq.next_element::<String>()?;

                if let Some(value) = tmp {
                    if value == "*" {
                        return Ok(Some(T::get_all_variants()));
                    }

                    vec.push(T::parse_single(&value));
                } else {
                    return Ok(Some(vec));
                }
            }
        }
    }
}

pub fn deserialize_wind_direction<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<WindDirection>>, D::Error>
where
    D: Deserializer<'de>,
{
    let parser: TileParser<WindDirection> = TileParser::new();
    deserializer.deserialize_seq(parser)
}

pub fn deserialize_dragon_color<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<DragonColor>>, D::Error>
where
    D: Deserializer<'de>,
{
    let parser: TileParser<DragonColor> = TileParser::new();
    deserializer.deserialize_seq(parser)
}

pub fn deserialize_suit_variant<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<NumberedSuit>>, D::Error>
where
    D: Deserializer<'de>,
{
    let parser: TileParser<NumberedSuit> = TileParser::new();
    deserializer.deserialize_seq(parser)
}

pub fn deserialize_tile_group<'de, D>(deserializer: D) -> Result<Option<Vec<TileGroup>>, D::Error>
where
    D: Deserializer<'de>,
{
    let parser: TileParser<TileGroup> = TileParser::new();
    deserializer.deserialize_seq(parser)
}
