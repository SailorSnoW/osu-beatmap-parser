use crate::error::BeatmapParseError;
use crate::error::BeatmapParseError::InvalidFormat;
use crate::section::CommaListElement;
use crate::types::timing_points::*;
use crate::types::OsuBool;
use std::str::FromStr;

//////////////////////////////////////////////////////////////////////////////////////////////////

/// Representation of a timing point.
/// Each timing point influences a specified portion of the map, commonly called a "timing section"
#[derive(Debug, Default)]
pub struct TimingPoint {
    /// Start time of the timing section, in milliseconds from the beginning of the beatmap's audio.
    /// The end of the timing section is the next timing point's time (or never, if this is the last timing point).
    pub time: u32,
    /// This property has two meanings:
    /// - For uninherited timing points, the duration of a beat, in milliseconds.
    /// - For inherited timing points, a negative inverse slider velocity multiplier, as a percentage.
    pub beat_length: f32,
    /// Amount of beats in a measure. Inherited timing points ignore this property.
    pub meter: u32,
    /// Default sample set for hit objects
    pub sample_set: SampleSet,
    /// Custom sample index for hit objects.
    /// `0` indicates osu!'s default hitsounds.
    pub sample_index: u32,
    /// Volume percentage for hit objects.
    pub volume: u8,
    /// Whether or not the timing point is uninherited.
    pub is_uninherited: OsuBool,
    /// Bit flags that give the timing point extra effects. See the [effects section](crate::types::timing_points::Effects).
    pub effects: Effects,
}

impl FromStr for TimingPoint {
    type Err = BeatmapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.trim().split(",").map(|x| x.trim()).collect();

        Ok(TimingPoint {
            time: u32::from_str(s[0]).map_err(|_| InvalidFormat {
                field: "time".to_string(),
            })?,
            beat_length: f32::from_str(s[1]).map_err(|_| InvalidFormat {
                field: "beat_length".to_string(),
            })?,
            meter: u32::from_str(s[2]).map_err(|_| InvalidFormat {
                field: "meter".to_string(),
            })?,
            sample_set: SampleSet::from_str(s[3]).map_err(|_| InvalidFormat {
                field: "sample_set".to_string(),
            })?,
            sample_index: u32::from_str(s[4]).map_err(|_| InvalidFormat {
                field: "sample_index".to_string(),
            })?,
            volume: u8::from_str(s[5]).map_err(|_| InvalidFormat {
                field: "volume".to_string(),
            })?,
            is_uninherited: OsuBool::from_str(s[6]).map_err(|_| InvalidFormat {
                field: "is_uninherited".to_string(),
            })?,
            effects: Effects::from_bits_truncate(u8::from_str(s[7]).unwrap()),
        })
    }
}

impl ToString for TimingPoint {
    fn to_string(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{}",
            self.time.to_string(),
            self.beat_length.to_string(),
            self.meter.to_string(),
            self.sample_set.to_string(),
            self.sample_index.to_string(),
            self.volume.to_string(),
            self.is_uninherited.to_string(),
            self.effects.bits().to_string()
        )
    }
}

impl CommaListElement for TimingPoint {}

#[cfg(test)]
mod tests {
    use crate::section::timing_points::{Effects, SampleSet, TimingPoint};
    use crate::section::CommaListOf;
    use crate::section::Section;

    const TEST_SECTION: &'static str = "10000,333.33,4,0,0,100,1,1
12000,-25,4,3,0,100,0,1
";

    #[test]
    fn parse_timing_points() {
        let timing_points: CommaListOf<TimingPoint> = CommaListOf::parse(TEST_SECTION).unwrap();

        assert_eq!(timing_points.len(), 2);

        assert_eq!(timing_points[0].time, 10000);
        assert_eq!(timing_points[0].beat_length, 333.33);
        assert_eq!(timing_points[0].meter, 4);
        assert_eq!(timing_points[0].sample_set, SampleSet::Default);
        assert_eq!(timing_points[0].sample_index, 0);
        assert_eq!(timing_points[0].volume, 100);
        assert_eq!(timing_points[0].is_uninherited, true.into());
        assert_eq!(timing_points[0].effects, Effects::KIAI);

        assert_eq!(timing_points[1].time, 12000);
        assert_eq!(timing_points[1].beat_length, -25.0);
        assert_eq!(timing_points[1].meter, 4);
        assert_eq!(timing_points[1].sample_set, SampleSet::Drum);
        assert_eq!(timing_points[1].sample_index, 0);
        assert_eq!(timing_points[1].volume, 100);
        assert_eq!(timing_points[1].is_uninherited, false.into());
        assert_eq!(timing_points[1].effects, Effects::KIAI);
    }

    #[test]
    fn serialize_timing_points() {
        let mut timing_points: CommaListOf<TimingPoint> = CommaListOf::new();
        timing_points.push(TimingPoint {
            time: 10000,
            beat_length: 333.33,
            meter: 4,
            sample_set: SampleSet::Default,
            sample_index: 0,
            volume: 100,
            is_uninherited: true.into(),
            effects: Effects::KIAI,
        });
        timing_points.push(TimingPoint {
            time: 12000,
            beat_length: -25.0,
            meter: 4,
            sample_set: SampleSet::Drum,
            sample_index: 0,
            volume: 100,
            is_uninherited: false.into(),
            effects: Effects::KIAI,
        });

        assert_eq!(timing_points.serialize(), TEST_SECTION)
    }

    mod timing_point {
        use super::*;
        use crate::section::CommaListElement;

        const TEST_TIMING_POINT: &'static str = "10000,333.33,4,0,0,100,1,1";

        #[test]
        fn parse_timing_point() {
            let timing_point = TimingPoint::parse(TEST_TIMING_POINT).unwrap();

            assert_eq!(timing_point.time, 10000);
            assert_eq!(timing_point.beat_length, 333.33);
            assert_eq!(timing_point.meter, 4);
            assert_eq!(timing_point.sample_set, SampleSet::Default);
            assert_eq!(timing_point.sample_index, 0);
            assert_eq!(timing_point.volume, 100);
            assert_eq!(timing_point.is_uninherited, true.into());
            assert_eq!(timing_point.effects, Effects::KIAI);
        }

        #[test]
        fn serialize_timing_point() {
            let timing_point = TimingPoint {
                time: 10000,
                beat_length: 333.33,
                meter: 4,
                sample_set: SampleSet::Default,
                sample_index: 0,
                volume: 100,
                is_uninherited: true.into(),
                effects: Effects::KIAI,
            };

            assert_eq!(timing_point.serialize(), TEST_TIMING_POINT)
        }
    }
}
