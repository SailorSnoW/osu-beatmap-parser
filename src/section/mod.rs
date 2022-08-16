pub mod general;
use crate::error::BeatmapParseError;
use std::fmt::Debug;
use std::slice::Iter;
use std::str::FromStr;

trait Section: Debug + Default + FromStr {
    #[inline]
    fn new() -> Self {
        Self::default()
    }
}

trait SectionKeyValue: Section {
    #[inline]
    fn get_field_name_value<T>(
        iter: &mut Iter<&str>,
        field_name: &str,
    ) -> Result<T, BeatmapParseError>
    where
        T: FromStr + Default,
    {
        match iter.find(|x| x.contains(field_name)) {
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

    #[inline]
    fn read_value(pair: &str) -> Result<String, BeatmapParseError> {
        let value: &str = match pair.split_once(':') {
            Some(x) => x.1.trim(),
            None => Err(BeatmapParseError::NotValidPair)?,
        };

        Ok(String::from(value))
    }
}
