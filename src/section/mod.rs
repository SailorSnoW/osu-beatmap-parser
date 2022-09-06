pub mod colours;
pub mod difficulty;
pub mod editor;
pub mod events;
pub mod general;
pub mod hit_objects;
pub mod metadata;
pub mod timing_points;

use crate::error::BeatmapParseError;
use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// Trait representing a section in an osu file format.
trait Section: Debug + Default + FromStr<Err = BeatmapParseError> + ToString {
    fn new() -> Self {
        Self::default()
    }

    fn parse(str: &str) -> Result<Self, BeatmapParseError> {
        Ok(Self::from_str(str)?)
    }
    fn serialize(&self) -> String {
        self.to_string()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// Trait representing a section in the format of `key:value` or `key: value` pairs.
trait SectionKeyValue: Section {
    fn get_field_name_value<T>(str: &Vec<&str>, field_name: &str) -> Result<T, BeatmapParseError>
    where
        T: FromStr + Default,
    {
        match str.iter().find(|x| x.contains(field_name)) {
            Some(pair) => Ok(Self::read_value(*pair)
                .map_err(|_| BeatmapParseError::InvalidFormat {
                    field: field_name.into(),
                })?
                .parse()
                .map_err(|_| BeatmapParseError::InvalidFormat {
                    field: field_name.into(),
                })?),
            None => Ok(T::default()),
        }
    }

    fn serialize_field<T>(field_name: &str, value: &T, with_space: bool) -> Option<String>
    where
        T: Display + Default + PartialEq,
    {
        if value == &T::default() {
            return None;
        } else {
            match with_space {
                true => return Some(format!("{}: {}\n", field_name, value)),
                false => return Some(format!("{}:{}\n", field_name, value)),
            }
        }
    }

    fn write_field_in<T>(buf: &mut String, field_name: &str, value: &T, with_space: bool)
    where
        T: Display + Default + PartialEq,
    {
        match Self::serialize_field(field_name, value, with_space) {
            Some(str) => buf.push_str(&str),
            None => (),
        }
    }

    fn read_value(pair: &str) -> Result<String, BeatmapParseError> {
        let value: &str = match pair.split_once(':') {
            Some(x) => x.1.trim(),
            None => Err(BeatmapParseError::NotValidPair)?,
        };

        Ok(String::from(value))
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// Trait representing an element of a section stored as a comma-separated list.
pub trait CommaListElement: Debug + Default + FromStr<Err = BeatmapParseError> + ToString {
    fn new() -> Self {
        Self::default()
    }

    fn parse(str: &str) -> Result<Self, BeatmapParseError> {
        Ok(Self::from_str(str)?)
    }
    fn serialize(&self) -> String {
        self.to_string()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// Represent a Section under the format of a comma-separated list.
#[derive(Debug, Default)]
pub struct CommaListOf<T: CommaListElement>(Vec<T>);

impl<T: CommaListElement> Deref for CommaListOf<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: CommaListElement> DerefMut for CommaListOf<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: CommaListElement> From<Vec<T>> for CommaListOf<T> {
    fn from(vec: Vec<T>) -> Self {
        Self { 0: vec }
    }
}

impl<T: CommaListElement> FromStr for CommaListOf<T> {
    type Err = BeatmapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut list: Vec<T> = Vec::new();

        let s: Vec<&str> = s.trim().split("\n").map(|x| x.trim()).collect();
        for element in s {
            let res = T::parse(element);
            match res {
                Ok(x) => list.push(x),
                Err(BeatmapParseError::CommentaryEntry) => (),
                Err(BeatmapParseError::StoryboardEntry) => (),
                Err(x) => return Err(x),
            }
        }

        Ok(list.into())
    }
}

impl<T: CommaListElement> ToString for CommaListOf<T> {
    fn to_string(&self) -> String {
        let mut buf = String::new();

        self.0.iter().for_each(|x| {
            buf.push_str(&x.serialize());
            buf.push_str("\n")
        });

        buf
    }
}

impl<T: CommaListElement> Section for CommaListOf<T> {}
