use crate::error::MapTypeError::UnexpectedBoolValue;
use crate::error::{BeatmapParseError, MapTypeError};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct OsuBool(bool);

impl From<bool> for OsuBool {
    fn from(boolean: bool) -> Self {
        Self { 0: boolean }
    }
}

impl From<OsuBool> for bool {
    fn from(osu_boolean: OsuBool) -> Self {
        osu_boolean.0
    }
}

impl FromStr for OsuBool {
    type Err = MapTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "true" | "1" => Ok(Self { 0: true }),
            "false" | "0" => Ok(Self { 0: false }),
            _ => Err(UnexpectedBoolValue),
        }
    }
}

impl Default for OsuBool {
    fn default() -> Self {
        Self { 0: false }
    }
}

impl Display for OsuBool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", i8::from(self.0))
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq, Eq)]
pub enum SampleSet {
    #[default]
    Default,
    Normal,
    Soft,
    Drum,
}

impl TryFrom<u8> for SampleSet {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SampleSet::Default),
            1 => Ok(SampleSet::Normal),
            2 => Ok(SampleSet::Soft),
            3 => Ok(SampleSet::Drum),
            _ => Err(()),
        }
    }
}

impl FromStr for SampleSet {
    type Err = BeatmapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let to_u8 = u8::from_str(s).unwrap();
        Ok(to_u8.try_into().unwrap())
    }
}

impl ToString for SampleSet {
    fn to_string(&self) -> String {
        match self {
            SampleSet::Default => String::from("0"),
            SampleSet::Normal => String::from("1"),
            SampleSet::Soft => String::from("2"),
            SampleSet::Drum => String::from("3"),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

pub mod general {
    use crate::error::GeneralError;
    use crate::error::GeneralError::{
        UnexpectedCountdownFormat, UnexpectedCountdownValue, UnexpectedGamemodeFormat,
        UnexpectedGamemodeValue, UnexpectedOverlayPosValue, UnexpectedSampleSetValue,
    };
    use std::fmt::{Display, Formatter};
    use std::str::FromStr;

    #[derive(Debug, PartialEq, Eq)]
    pub enum Gamemode {
        STD,
        TAIKO,
        CTB,
        MANIA,
    }

    impl Default for Gamemode {
        fn default() -> Self {
            Gamemode::STD
        }
    }

    impl TryFrom<i32> for Gamemode {
        type Error = GeneralError;

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Gamemode::STD),
                1 => Ok(Gamemode::TAIKO),
                2 => Ok(Gamemode::CTB),
                3 => Ok(Gamemode::MANIA),
                _ => Err(UnexpectedGamemodeValue { value }),
            }
        }
    }

    impl FromStr for Gamemode {
        type Err = GeneralError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            i32::from_str(s)
                .map_err(|_| UnexpectedGamemodeFormat)?
                .try_into()
        }
    }

    impl From<&Gamemode> for i32 {
        fn from(mode: &Gamemode) -> Self {
            match mode {
                Gamemode::STD => 0,
                Gamemode::TAIKO => 1,
                Gamemode::CTB => 2,
                Gamemode::MANIA => 3,
            }
        }
    }

    impl Display for Gamemode {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", i32::from(self))
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////

    #[derive(Debug, PartialEq, Eq)]
    pub enum OverlayPosition {
        /// use skin setting
        NOCHANGE,
        /// draw overlays under numbers
        BELOW,
        /// draw overlays on top of numbers
        ABOVE,
    }

    impl Default for OverlayPosition {
        fn default() -> Self {
            OverlayPosition::NOCHANGE
        }
    }

    impl FromStr for OverlayPosition {
        type Err = GeneralError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "NoChange" => Ok(OverlayPosition::NOCHANGE),
                "Below" => Ok(OverlayPosition::BELOW),
                "Above" => Ok(OverlayPosition::ABOVE),
                _ => Err(UnexpectedOverlayPosValue {
                    value: s.to_string(),
                }),
            }
        }
    }

    impl From<&OverlayPosition> for String {
        fn from(pos: &OverlayPosition) -> Self {
            match pos {
                OverlayPosition::NOCHANGE => String::from("NoChange"),
                OverlayPosition::BELOW => String::from("Below"),
                OverlayPosition::ABOVE => String::from("Above"),
            }
        }
    }

    impl Display for OverlayPosition {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", String::from(self))
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    #[derive(Debug, PartialEq, Eq)]
    pub enum SampleSet {
        NORMAL,
        SOFT,
        DRUM,
    }

    impl Default for SampleSet {
        fn default() -> Self {
            SampleSet::NORMAL
        }
    }

    impl FromStr for SampleSet {
        type Err = GeneralError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "Normal" => Ok(SampleSet::NORMAL),
                "Soft" => Ok(SampleSet::SOFT),
                "Drum" => Ok(SampleSet::DRUM),
                _ => Err(UnexpectedSampleSetValue {
                    value: s.to_string(),
                }),
            }
        }
    }

    impl From<&SampleSet> for String {
        fn from(pos: &SampleSet) -> Self {
            match pos {
                SampleSet::NORMAL => String::from("Normal"),
                SampleSet::SOFT => String::from("Soft"),
                SampleSet::DRUM => String::from("Drum"),
            }
        }
    }

    impl Display for SampleSet {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", String::from(self))
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    #[derive(Debug, PartialEq, Eq)]
    pub enum Countdown {
        NONE,
        NORMAL,
        HALF,
        DOUBLE,
    }

    impl Default for Countdown {
        fn default() -> Self {
            Countdown::NORMAL
        }
    }

    impl TryFrom<i32> for Countdown {
        type Error = GeneralError;

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Countdown::NONE),
                1 => Ok(Countdown::NORMAL),
                2 => Ok(Countdown::HALF),
                3 => Ok(Countdown::DOUBLE),
                _ => Err(UnexpectedCountdownValue { value }),
            }
        }
    }

    impl FromStr for Countdown {
        type Err = GeneralError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            i32::from_str(s)
                .map_err(|_| UnexpectedCountdownFormat)?
                .try_into()
        }
    }

    impl From<&Countdown> for i32 {
        fn from(mode: &Countdown) -> Self {
            match mode {
                Countdown::NONE => 0,
                Countdown::NORMAL => 1,
                Countdown::HALF => 2,
                Countdown::DOUBLE => 3,
            }
        }
    }

    impl Display for Countdown {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", i32::from(self))
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

pub mod timing_points {
    use bitflags::bitflags;

    bitflags! {
        /// Timing points have two extra effects that can be toggled using bits 0 and 3
        /// (from least to most significant) in the effects integer.
        pub struct Effects: u8 {
            /// Whether or not [kiai time](https://osu.ppy.sh/wiki/en/Gameplay/Kiai_time) is enabled
            const KIAI = 0b00000001;
            /// Whether or not the first barline is omitted in osu!taiko and osu!mania
            const OMIT_BARLINE = 0b00000100;
        }
    }

    impl Default for Effects {
        fn default() -> Self {
            Self { bits: 0 }
        }
    }
}
