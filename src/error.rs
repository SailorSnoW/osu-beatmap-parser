use thiserror::Error;

#[derive(Error, Debug)]
pub enum BeatmapParseError {
    #[error("Tried to read a data which isn't a 'key:value' pair")]
    NotValidPair,
    #[error("Invalid data format for the following field: {field}")]
    InvalidFormat { field: String },
}

#[derive(Error, Debug)]
pub enum GeneralError {
    #[error("Received unexpected value to parse to a Countdown value, got {value}, expected value in range [0 - 4]")]
    UnexpectedCountdownValue { value: i32 },
}
