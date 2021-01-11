use crate::hand::TileGroup;
use crate::types::{DragonColor, NumberedSuit, WindDirection};
use serde::de::{SeqAccess, Visitor};
use serde::Deserializer;
use std::collections::HashSet;
use std::fmt::{Formatter, Result as FormatResult};
use std::marker::PhantomData;
use std::str::FromStr;

macro_rules! set {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_set = HashSet::new();
            $(
                temp_set.insert($x);
            )*
            temp_set
        }
    };
}

pub trait EnumExpander {
    fn get_message() -> String;
    fn parse_single(item: &str) -> Self
    where
        Self: Sized;

    fn get_all_variants() -> HashSet<Self>
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

    fn get_all_variants() -> HashSet<WindDirection> {
        set![
            WindDirection::East,
            WindDirection::South,
            WindDirection::West,
            WindDirection::North
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

    fn get_all_variants() -> HashSet<DragonColor> {
        set![DragonColor::White, DragonColor::Green, DragonColor::Red]
    }
}

impl EnumExpander for NumberedSuit {
    fn get_message() -> String {
        "Message".to_string()
    }

    fn parse_single(item: &str) -> NumberedSuit {
        NumberedSuit::from_str(item).unwrap()
    }

    fn get_all_variants() -> HashSet<NumberedSuit> {
        set![
            NumberedSuit::Characters,
            NumberedSuit::Circles,
            NumberedSuit::Bamboos
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

    fn get_all_variants() -> HashSet<TileGroup> {
        set![
            TileGroup::Jantou,
            TileGroup::Shuntsu,
            TileGroup::Koutsu,
            TileGroup::Kantsu
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
    T: EnumExpander + Eq + std::hash::Hash,
{
    type Value = Option<HashSet<T>>;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        write!(formatter, "{}", T::get_message())
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, <A as SeqAccess<'de>>::Error>
    where
        A: SeqAccess<'de>,
    {
        let size_hint = seq.size_hint().unwrap();

        if size_hint == 0 {
            Ok(Some(HashSet::new()))
        } else {
            let mut item_set: HashSet<T> = HashSet::with_capacity(size_hint);

            loop {
                let tmp = seq.next_element::<String>()?;

                if let Some(value) = tmp {
                    if value == "*" {
                        return Ok(Some(T::get_all_variants()));
                    }

                    item_set.insert(T::parse_single(&value));
                } else {
                    return Ok(Some(item_set));
                }
            }
        }
    }
}

pub fn deserialize_wind_direction<'de, D>(
    deserializer: D,
) -> Result<Option<HashSet<WindDirection>>, D::Error>
where
    D: Deserializer<'de>,
{
    let parser: TileParser<WindDirection> = TileParser::new();
    deserializer.deserialize_seq(parser)
}

pub fn deserialize_dragon_color<'de, D>(
    deserializer: D,
) -> Result<Option<HashSet<DragonColor>>, D::Error>
where
    D: Deserializer<'de>,
{
    let parser: TileParser<DragonColor> = TileParser::new();
    deserializer.deserialize_seq(parser)
}

pub fn deserialize_suit_variant<'de, D>(
    deserializer: D,
) -> Result<Option<HashSet<NumberedSuit>>, D::Error>
where
    D: Deserializer<'de>,
{
    let parser: TileParser<NumberedSuit> = TileParser::new();
    deserializer.deserialize_seq(parser)
}

pub fn deserialize_tile_group<'de, D>(
    deserializer: D,
) -> Result<Option<HashSet<TileGroup>>, D::Error>
where
    D: Deserializer<'de>,
{
    let parser: TileParser<TileGroup> = TileParser::new();
    deserializer.deserialize_seq(parser)
}
