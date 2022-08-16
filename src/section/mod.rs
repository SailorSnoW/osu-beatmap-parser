pub mod difficulty;
pub mod editor;
pub mod general;
pub mod metadata;

use crate::error::BeatmapParseError;
use std::fmt::{Debug, Display};
use std::str::FromStr;

trait Section: Debug + Default + FromStr<Err = BeatmapParseError> + Into<String> {
    fn new() -> Self {
        Self::default()
    }

    fn parse(str: &str) -> Result<Self, BeatmapParseError> {
        Ok(Self::from_str(str)?)
    }
    fn serialize(self) -> String {
        self.into()
    }
}

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
