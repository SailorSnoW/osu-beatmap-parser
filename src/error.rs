use thiserror::Error;

#[derive(Error, Debug)]
pub enum MapTypeError {
    #[error("Tried to read a data which isn't a boolean value")]
    UnexpectedBoolValue,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum BeatmapParseError {
    #[error("Tried to read a data which isn't a 'key:value' pair")]
    NotValidPair,
    #[error("Invalid data format for the following field: {field}")]
    InvalidFormat { field: String },
    #[error("Commentary line")]
    CommentaryEntry,
    #[error("Storyboard related line, not supported for the moment")]
    StoryboardEntry,
    #[error("The section seems to not be present in the beatmap file")]
    SectionNotFound { section: String },
}

#[derive(Error, Debug)]
pub enum GeneralError {
    #[error("Received unexpected value to parse to a Countdown value, got {value}, expected value in range [0 - 4]")]
    UnexpectedCountdownValue { value: i32 },
    #[error("Received value other than a integer")]
    UnexpectedCountdownFormat,
    #[error("Received unexpected value to parse to a Gamemode value, got {value}, expected value in range [0 - 4]")]
    UnexpectedGamemodeValue { value: i32 },
    #[error("Received value other than a integer")]
    UnexpectedGamemodeFormat,
    #[error("Received unexpected value to parse to an OverlayPosition value, got {value}, expected 'NoChange', 'Below' or 'Above'")]
    UnexpectedOverlayPosValue { value: String },
    #[error("Received unexpected value to parse to a SampleSet value, got {value}, expected 'Normal', 'Soft' or 'Drum'")]
    UnexpectedSampleSetValue { value: String },
}

#[derive(Error, Debug)]
pub enum EventsError {
    #[error("Tried to read an unknown type in the Events section, got {value}, expected 'Background', 'Video', 'Break' or 0-1-2.")]
    UnexpectedEventType { value: String },
}
