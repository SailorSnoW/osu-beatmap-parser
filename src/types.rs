use serde::Deserialize;

pub mod general {
    use super::*;
    use crate::error::GeneralError;
    use crate::error::GeneralError::UnexpectedCountdownValue;

    #[derive(Debug, Deserialize)]
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

    ///////////////////////////////////////////////////////////////////////////
    #[derive(Debug, Deserialize)]
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

    ///////////////////////////////////////////////////////////////////////////
    #[derive(Debug, Deserialize)]
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
}
