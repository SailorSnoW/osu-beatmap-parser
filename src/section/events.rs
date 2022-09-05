use crate::error::BeatmapParseError::InvalidFormat;
use crate::error::{BeatmapParseError, EventsError};
use crate::section::CommaListElement;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// Type of an event with the wrapped event params.
/// Some events may be referred to by either a name or a number.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventType {
    Background(BackgroundParams),
    Video(VideoParams),
    Break(BreakParams),
}

impl FromStr for EventType {
    type Err = EventsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s == "Background" || s == "0" => Ok(EventType::Background(Default::default())),
            _ if s == "Video" || s == "1" => Ok(EventType::Video(Default::default())),
            _ if s == "Break" || s == "2" => Ok(EventType::Break(Default::default())),
            _ => Err(EventsError::UnexpectedEventType {
                value: s.to_string(),
            }),
        }
    }
}

impl Default for EventType {
    fn default() -> Self {
        EventType::Background(Default::default())
    }
}

impl EventType {
    pub fn serialize_inner(&self) -> String {
        match self {
            EventType::Background(x) => x.to_string(),
            EventType::Video(x) => x.to_string(),
            EventType::Break(x) => x.to_string(),
        }
    }

    pub fn try_into_inner<T: TryFrom<Self>>(self) -> Result<T, T::Error> {
        self.try_into()
    }
}

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Background(_) => write!(f, "0"),
            EventType::Video(_) => write!(f, "1"),
            EventType::Break(_) => write!(f, "2"),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BackgroundParams {
    /// Location of the background image relative to the beatmap directory.
    pub filename: String,
    /// X offset in [osu! pixels](https://osu.ppy.sh/wiki/en/osupixel) from the centre of the screen.
    pub x_offset: i32,
    /// Y offset in [osu! pixels](https://osu.ppy.sh/wiki/en/osupixel) from the centre of the screen.
    pub y_offset: i32,
}

impl From<BackgroundParams> for EventType {
    fn from(background_params: BackgroundParams) -> Self {
        EventType::Background(background_params)
    }
}

impl TryFrom<EventType> for BackgroundParams {
    type Error = ();

    fn try_from(value: EventType) -> Result<Self, Self::Error> {
        match value {
            EventType::Background(x) => Ok(x),
            _ => Err(()),
        }
    }
}

impl Display for BackgroundParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"{}\",{},{}",
            self.filename, self.x_offset, self.y_offset
        )
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VideoParams {
    /// Location of the background image relative to the beatmap directory.
    pub filename: String,
    /// X offset in [osu! pixels](https://osu.ppy.sh/wiki/en/osupixel) from the centre of the screen.
    pub x_offset: i32,
    /// Y offset in [osu! pixels](https://osu.ppy.sh/wiki/en/osupixel) from the centre of the screen.
    pub y_offset: i32,
}

impl From<VideoParams> for EventType {
    fn from(video_params: VideoParams) -> Self {
        EventType::Video(video_params)
    }
}

impl TryFrom<EventType> for VideoParams {
    type Error = ();

    fn try_from(value: EventType) -> Result<Self, Self::Error> {
        match value {
            EventType::Video(x) => Ok(x),
            _ => Err(()),
        }
    }
}

impl Display for VideoParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"{}\",{},{}",
            self.filename, self.x_offset, self.y_offset
        )
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BreakParams {
    /// End time of the break, in milliseconds from the beginning of the beatmap's audio.
    pub end_time: u32,
}

impl From<BreakParams> for EventType {
    fn from(break_params: BreakParams) -> Self {
        EventType::Break(break_params)
    }
}

impl TryFrom<EventType> for BreakParams {
    type Error = ();

    fn try_from(value: EventType) -> Result<Self, Self::Error> {
        match value {
            EventType::Break(x) => Ok(x),
            _ => Err(()),
        }
    }
}

impl Display for BreakParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.end_time)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// Beatmap graphic event | TODO and storyboard
#[derive(Debug, Default)]
pub struct Event {
    /// Start time of the event, in milliseconds from the beginning of the beatmap's audio.
    /// For events that do not use a start time, the default is `0`.
    pub start_time: u32,
    /// Type of the event with these params.
    pub event_params: EventType,
}

impl FromStr for Event {
    type Err = BeatmapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("//") {
            return Err(BeatmapParseError::CommentaryEntry);
        }

        let s: Vec<&str> = s.trim().split(",").map(|x| x.trim()).collect();

        let mut event_type = EventType::from_str(s[0]).map_err(|_| InvalidFormat {
            field: String::from("0"),
        })?;

        match event_type {
            EventType::Background(ref mut x) => {
                x.filename = String::from(s[2].trim_matches('\"'));
                x.x_offset = i32::from_str(s[3]).map_err(|_| InvalidFormat {
                    field: String::from("3"),
                })?;
                x.y_offset = i32::from_str(s[4]).map_err(|_| InvalidFormat {
                    field: String::from("4"),
                })?;
            }
            EventType::Video(ref mut x) => {
                x.filename = String::from(s[2].trim_matches('\"'));
                x.x_offset = i32::from_str(s[3]).map_err(|_| InvalidFormat {
                    field: String::from("3"),
                })?;
                x.y_offset = i32::from_str(s[4]).map_err(|_| InvalidFormat {
                    field: String::from("4"),
                })?;
            }
            EventType::Break(ref mut x) => {
                x.end_time = u32::from_str(s[2]).map_err(|_| InvalidFormat {
                    field: String::from("2"),
                })?;
            }
        }

        Ok(Event {
            start_time: u32::from_str(s[1]).map_err(|_| InvalidFormat {
                field: String::from("1"),
            })?,
            event_params: event_type,
        })
    }
}

impl ToString for Event {
    fn to_string(&self) -> String {
        format!(
            "{},{},{}",
            self.event_params.to_string(),
            self.start_time.to_string(),
            self.event_params.serialize_inner()
        )
    }
}

impl CommaListElement for Event {}

///////////////////////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::section::events::{BackgroundParams, BreakParams, Event, EventType};
    use crate::section::CommaListOf;
    use crate::section::Section;

    const TEST_SECTION: &'static str = "0,0,\"bg.jpg\",0,0
2,104177,114656
";

    #[test]
    fn parse_events() {
        let events: CommaListOf<Event> = CommaListOf::parse(TEST_SECTION).unwrap();

        let first_event = events.get(0).unwrap();
        let first_event_params: BackgroundParams =
            first_event.event_params.clone().try_into_inner().unwrap();

        let second_event = events.get(1).unwrap();
        let second_event_params: BreakParams =
            second_event.event_params.clone().try_into_inner().unwrap();

        assert_eq!(events.len(), 2);

        assert_eq!(first_event.start_time, 0);
        assert_eq!(first_event_params.filename, "bg.jpg");
        assert_eq!(first_event_params.x_offset, 0);
        assert_eq!(first_event_params.y_offset, 0);

        assert_eq!(second_event.start_time, 104177);
        assert_eq!(second_event_params.end_time, 114656);
    }

    #[test]
    fn serialize_events() {
        let mut events: CommaListOf<Event> = CommaListOf::new();
        let first_event = Event {
            start_time: 0,
            event_params: EventType::Background(BackgroundParams {
                filename: String::from("bg.jpg"),
                x_offset: 0,
                y_offset: 0,
            }),
        };
        let second_event = Event {
            start_time: 104177,
            event_params: EventType::Break(BreakParams { end_time: 114656 }),
        };

        events.push(first_event);
        events.push(second_event);

        assert_eq!(events.serialize(), TEST_SECTION)
    }

    mod event {
        use crate::section::events::{BackgroundParams, Event};
        use crate::section::CommaListElement;

        const TEST_BACKGROUND_EVENT: &'static str = "0,0,\"bg.jpg\",0,0";

        #[test]
        fn parse_background_event() {
            let event = Event::parse(TEST_BACKGROUND_EVENT).unwrap();
            let event_params: BackgroundParams = event.event_params.try_into_inner().unwrap();

            assert_eq!(event.start_time, 0);
            assert_eq!(event_params.filename, "bg.jpg");
            assert_eq!(event_params.x_offset, 0);
            assert_eq!(event_params.y_offset, 0);
        }

        #[test]
        fn serialize_background_event() {
            let mut event = Event::new();
            let mut event_params: BackgroundParams = event.event_params.try_into_inner().unwrap();
            event.start_time = 0;
            event_params.filename = String::from("bg.jpg");
            event.event_params = event_params.into();

            assert_eq!(event.serialize(), TEST_BACKGROUND_EVENT)
        }
    }
}
