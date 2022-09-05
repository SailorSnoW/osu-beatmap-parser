use crate::error::BeatmapParseError;
use crate::error::BeatmapParseError::InvalidFormat;
use crate::section::CommaListElement;
use crate::types::SampleSet;
use bitflags::bitflags;
use std::str::FromStr;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SliderPoint {
    pub x: i32,
    pub y: i32,
}

impl FromStr for SliderPoint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.trim().split(":").map(|x| x.trim()).collect();

        Ok(SliderPoint {
            x: i32::from_str(s[0]).map_err(|_| ())?,
            y: i32::from_str(s[1]).map_err(|_| ())?,
        })
    }
}

impl ToString for SliderPoint {
    fn to_string(&self) -> String {
        format!("{}:{}", self.x, self.y)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq, Eq)]
pub struct EdgeSounds {
    pub sounds: Vec<u32>,
    pub sets: Vec<(u32, u32)>,
}

impl FromStr for EdgeSounds {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut edge_sounds = EdgeSounds::default();
        let s = s.trim().split_once(",").ok_or_else(|| ())?;

        let sounds: Vec<&str> = s.0.split("|").collect();
        let sets: Vec<&str> = s.1.split("|").collect();

        for sound in sounds {
            edge_sounds
                .sounds
                .push(u32::from_str(sound).map_err(|_| ())?);
        }
        for set in sets {
            let set_values = set.split_once(":").ok_or_else(|| ())?;
            let set_values_uint = (
                u32::from_str(set_values.0).map_err(|_| ())?,
                u32::from_str(set_values.1).map_err(|_| ())?,
            );
            edge_sounds.sets.push(set_values_uint)
        }

        Ok(edge_sounds)
    }
}

impl ToString for EdgeSounds {
    fn to_string(&self) -> String {
        let mut buf = String::new();

        self.sounds.iter().for_each(|sound| {
            buf.push_str(&sound.to_string());
            buf.push('|')
        });
        buf.pop();

        buf.push(',');

        self.sets.iter().for_each(|set| {
            buf.push_str(&set.0.to_string());
            buf.push(':');
            buf.push_str(&set.1.to_string());
            buf.push('|');
        });
        buf.pop();

        buf
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq, Eq)]
pub enum SliderType {
    Bezier,
    CentripetalCatmullRom,
    #[default]
    Linear,
    PerfectCircle,
}

impl TryFrom<char> for SliderType {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'B' => Ok(SliderType::Bezier),
            'C' => Ok(SliderType::CentripetalCatmullRom),
            'L' => Ok(SliderType::Linear),
            'P' => Ok(SliderType::PerfectCircle),
            _ => Err(()),
        }
    }
}

impl From<&SliderType> for char {
    fn from(slider_type: &SliderType) -> Self {
        match slider_type {
            SliderType::Bezier => 'B',
            SliderType::CentripetalCatmullRom => 'C',
            SliderType::Linear => 'L',
            SliderType::PerfectCircle => 'P',
        }
    }
}

impl FromStr for SliderType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = char::from_str(s).map_err(|_| ())?;
        Ok(c.try_into()?)
    }
}

impl ToString for SliderType {
    fn to_string(&self) -> String {
        String::from(char::from(self))
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq, Eq)]
pub struct HitSample {
    pub normal_set: SampleSet,
    pub additional_set: SampleSet,
    pub index: u32,
    pub volume: u8,
    pub filename: String,
}

impl FromStr for HitSample {
    type Err = BeatmapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.trim().split(":").map(|x| x.trim()).collect();

        Ok(Self {
            normal_set: SampleSet::from_str(s[0]).map_err(|_| InvalidFormat {
                field: "normal_set".to_string(),
            })?,
            additional_set: SampleSet::from_str(s[1]).map_err(|_| InvalidFormat {
                field: "additional_set".to_string(),
            })?,
            index: u32::from_str(s[2]).map_err(|_| InvalidFormat {
                field: "index".to_string(),
            })?,
            volume: u8::from_str(s[3]).map_err(|_| InvalidFormat {
                field: "volume".to_string(),
            })?,
            filename: String::from(s[4]),
        })
    }
}

impl ToString for HitSample {
    fn to_string(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.normal_set.to_string(),
            self.additional_set.to_string(),
            self.index,
            self.volume,
            self.filename
        )
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq)]
pub enum HitObjectType {
    #[default]
    HitCircle,
    Slider(SliderParams),
    Spinner(SpinnerParams),
    ManiaHold(ManiaHoldParams),
}

impl HitObjectType {
    #[allow(dead_code)]
    pub fn try_into_inner<T: TryFrom<Self>>(self) -> Result<T, T::Error> {
        self.try_into()
    }
}

impl FromStr for HitObjectType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            HitObjectTypeFlag::from_bits_truncate(u8::from_str(s).map_err(|_| ())?)
                .try_into()
                .map_err(|_| ())?,
        )
    }
}
impl TryFrom<HitObjectTypeFlag> for HitObjectType {
    type Error = ();

    fn try_from(value: HitObjectTypeFlag) -> Result<Self, Self::Error> {
        match value {
            _ if value.contains(HitObjectTypeFlag::HIT_CIRCLE) => Ok(HitObjectType::HitCircle),
            _ if value.contains(HitObjectTypeFlag::SLIDER) => {
                Ok(HitObjectType::Slider(SliderParams::default()))
            }
            _ if value.contains(HitObjectTypeFlag::SPINNER) => {
                Ok(HitObjectType::Spinner(SpinnerParams::default()))
            }
            _ if value.contains(HitObjectTypeFlag::MANIA_HOLD) => {
                Ok(HitObjectType::ManiaHold(ManiaHoldParams::default()))
            }
            _ => Err(()),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

bitflags! {
    pub struct HitObjectTypeFlag: u8 {
        const HIT_CIRCLE = 0b00000001;
        const SLIDER = 0b00000010;
        const SPINNER = 0b00001000;
        const MANIA_HOLD = 0b10000000;

        const NEW_COMBO = 0b00000100;
        const SKIP_ONE = 0b00010000 | Self::NEW_COMBO.bits;
        const SKIP_TWO = 0b00100000 | Self::NEW_COMBO.bits;
        const SKIP_FOUR = 0b01000000 | Self::NEW_COMBO.bits;
    }
}

impl From<&HitObjectType> for HitObjectTypeFlag {
    fn from(hit_object_type: &HitObjectType) -> Self {
        match hit_object_type {
            HitObjectType::HitCircle => Self::HIT_CIRCLE,
            HitObjectType::Slider(_) => Self::SLIDER,
            HitObjectType::Spinner(_) => Self::SPINNER,
            HitObjectType::ManiaHold(_) => Self::MANIA_HOLD,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

bitflags! {
    pub struct HitSoundFlag: u8 {
        const NORMAL = 0b00000001;
        const WHISTLE = 0b00000010;
        const FINISH = 0b00000100;
        const CLAP = 0b00001000;
    }
}

impl Default for HitSoundFlag {
    /// If no bits are set, the normal hitsound is used by default.
    fn default() -> Self {
        Self::NORMAL
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq)]
pub struct SliderParams {
    pub slider_type: SliderType,
    pub curve_points: Vec<SliderPoint>,
    pub slides: u32,
    pub length: f32,
    pub edge_sounds: EdgeSounds,
}

impl SliderParams {
    pub fn serialize_curve_points(&self) -> String {
        let mut buf = String::new();

        self.curve_points.iter().for_each(|p| {
            buf.push('|');
            buf.push_str(&p.to_string());
        });

        buf
    }
}

impl TryFrom<HitObjectType> for SliderParams {
    type Error = ();

    fn try_from(value: HitObjectType) -> Result<Self, Self::Error> {
        match value {
            HitObjectType::Slider(x) => Ok(x),
            _ => Err(()),
        }
    }
}

impl FromStr for SliderParams {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.trim().splitn(4, ",").map(|x| x.trim()).collect();
        let type_and_points: Vec<&str> = s[0].split("|").collect();

        Ok(SliderParams {
            slider_type: SliderType::from_str(type_and_points[0]).map_err(|_| ())?,
            curve_points: {
                let mut x: Vec<SliderPoint> = Vec::default();

                for p in type_and_points.iter().skip(1) {
                    x.push(SliderPoint::from_str(p).map_err(|_| ())?)
                }

                x
            },
            slides: u32::from_str(s[1]).map_err(|_| ())?,
            length: f32::from_str(s[2]).map_err(|_| ())?,
            edge_sounds: EdgeSounds::from_str(s[3]).map_err(|_| ())?,
        })
    }
}

impl ToString for SliderParams {
    fn to_string(&self) -> String {
        format!(
            "{}{},{},{},{}",
            self.slider_type.to_string(),
            self.serialize_curve_points(),
            self.slides,
            self.length,
            self.edge_sounds.to_string()
        )
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SpinnerParams {
    pub end_time: u32,
}

impl TryFrom<HitObjectType> for SpinnerParams {
    type Error = ();

    fn try_from(value: HitObjectType) -> Result<Self, Self::Error> {
        match value {
            HitObjectType::Spinner(x) => Ok(x),
            _ => Err(()),
        }
    }
}

impl FromStr for SpinnerParams {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            end_time: u32::from_str(s).map_err(|_| ())?,
        })
    }
}

impl ToString for SpinnerParams {
    fn to_string(&self) -> String {
        self.end_time.to_string()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ManiaHoldParams {
    pub end_time: u32,
}

impl TryFrom<HitObjectType> for ManiaHoldParams {
    type Error = ();

    fn try_from(value: HitObjectType) -> Result<Self, Self::Error> {
        match value {
            HitObjectType::ManiaHold(x) => Ok(x),
            _ => Err(()),
        }
    }
}

impl FromStr for ManiaHoldParams {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            end_time: u32::from_str(s).map_err(|_| ())?,
        })
    }
}

impl ToString for ManiaHoldParams {
    fn to_string(&self) -> String {
        self.end_time.to_string()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq)]
pub struct HitObject {
    pub x: i32,
    pub y: i32,
    pub time: u32,
    pub object_params: HitObjectType,
    pub new_combo: bool,
    pub combo_skip: u8,
    pub hit_sound: HitSoundFlag,
    pub hit_sample: HitSample,
}

impl FromStr for HitObject {
    type Err = BeatmapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.trim().splitn(6, ",").map(|x| x.trim()).collect();
        let mut hit_object = HitObject::new();

        let object_type =
            HitObjectTypeFlag::from_bits_truncate(u8::from_str(split[3]).map_err(|_| {
                InvalidFormat {
                    field: "object_type".to_string(),
                }
            })?);

        if object_type.contains(HitObjectTypeFlag::NEW_COMBO) {
            hit_object.new_combo = true
        }

        let mut combo_skip_count = 0u8;
        if object_type.contains(HitObjectTypeFlag::SKIP_ONE) {
            combo_skip_count += 1;
        }
        if object_type.contains(HitObjectTypeFlag::SKIP_TWO) {
            combo_skip_count += 2;
        }
        if object_type.contains(HitObjectTypeFlag::SKIP_FOUR) {
            combo_skip_count += 4;
        }
        hit_object.combo_skip = combo_skip_count;

        hit_object.x = i32::from_str(split[0]).map_err(|_| InvalidFormat {
            field: "x".to_string(),
        })?;
        hit_object.y = i32::from_str(split[1]).map_err(|_| InvalidFormat {
            field: "y".to_string(),
        })?;
        hit_object.time = u32::from_str(split[2]).map_err(|_| InvalidFormat {
            field: "time".to_string(),
        })?;
        hit_object.object_params =
            HitObjectType::try_from(object_type).map_err(|_| InvalidFormat {
                field: "object_params".to_string(),
            })?;
        hit_object.hit_sound =
            HitSoundFlag::from_bits_truncate(u8::from_str(split[4]).map_err(|_| {
                InvalidFormat {
                    field: "hit_sound".to_string(),
                }
            })?);

        match hit_object.object_params {
            HitObjectType::HitCircle => {
                let hit_sample = split.get(5);
                match hit_sample {
                    Some(hit_sample) => {
                        hit_object.hit_sample =
                            HitSample::from_str(hit_sample).map_err(|_| InvalidFormat {
                                field: "hit_sample".to_string(),
                            })?;
                        Ok(hit_object)
                    }
                    None => {
                        hit_object.hit_sample = HitSample::default();
                        Ok(hit_object)
                    }
                }
            }
            HitObjectType::Slider(ref mut _params) => {
                let mut vec_splitted_params: Vec<&str> = split[5].split_inclusive(",").collect();
                vec_splitted_params.pop();
                let mut string_params: String = vec_splitted_params.drain(0..).collect();
                string_params.pop();

                let hit_sample: &str = split[5].split(",").last().ok_or_else(|| InvalidFormat {
                    field: "hit_sample".to_string(),
                })?;

                *_params = SliderParams::from_str(&string_params).map_err(|_| InvalidFormat {
                    field: "object_params".to_string(),
                })?;
                hit_object.hit_sample =
                    HitSample::from_str(hit_sample).map_err(|_| InvalidFormat {
                        field: "hit_sample".to_string(),
                    })?;

                Ok(hit_object)
            }
            HitObjectType::Spinner(ref mut _params) => {
                let splitted = split[5].split_once(",").ok_or_else(|| InvalidFormat {
                    field: "object_params/hit_sample".to_string(),
                })?;

                *_params = SpinnerParams::from_str(splitted.0).map_err(|_| InvalidFormat {
                    field: "object_params".to_string(),
                })?;
                hit_object.hit_sample =
                    HitSample::from_str(splitted.1).map_err(|_| InvalidFormat {
                        field: "hit_sample".to_string(),
                    })?;

                Ok(hit_object)
            }
            HitObjectType::ManiaHold(ref mut _params) => {
                let splitted = split[5].split_once(":").ok_or_else(|| InvalidFormat {
                    field: "object_params/hit_sample".to_string(),
                })?;

                *_params = ManiaHoldParams::from_str(splitted.0).map_err(|_| InvalidFormat {
                    field: "object_params".to_string(),
                })?;
                hit_object.hit_sample =
                    HitSample::from_str(splitted.1).map_err(|_| InvalidFormat {
                        field: "hit_sample".to_string(),
                    })?;

                Ok(hit_object)
            }
        }
    }
}

impl ToString for HitObject {
    fn to_string(&self) -> String {
        let mut type_infos = HitObjectTypeFlag::from(&self.object_params);

        if self.new_combo {
            type_infos.insert(HitObjectTypeFlag::NEW_COMBO);
        }
        if self.combo_skip & (1 << 0) == 1 {
            type_infos.insert(HitObjectTypeFlag::SKIP_ONE);
        }
        if self.combo_skip & (1 << 1) == 1 {
            type_infos.insert(HitObjectTypeFlag::SKIP_TWO);
        }
        if self.combo_skip & (1 << 2) == 1 {
            type_infos.insert(HitObjectTypeFlag::SKIP_FOUR);
        }

        let mut buf = format!(
            "{},{},{},{},{},",
            self.x, self.y, self.time, type_infos.bits, self.hit_sound.bits
        );

        match &self.object_params {
            HitObjectType::Slider(x) => {
                buf.push_str(&x.to_string());
                buf.push(',');
            }
            HitObjectType::Spinner(x) => {
                buf.push_str(&x.to_string());
                buf.push(',');
            }
            HitObjectType::ManiaHold(x) => {
                buf.push_str(&x.to_string());
                buf.push(',');
            }
            _ => (),
        }

        buf.push_str(&self.hit_sample.to_string());

        buf
    }
}

impl CommaListElement for HitObject {}

#[cfg(test)]
mod tests {
    use crate::section::hit_objects::*;
    use crate::section::{CommaListElement, CommaListOf, Section};
    use crate::types::SampleSet;

    const TEST_SECTION: &'static str = "256,192,11000,21,2,0:0:0:0:
256,192,11200,8,12,12000,3:0:0:80:
100,100,12600,6,1,B|200:200|250:200|250:200|300:150,2,310.123,2|1|2,0:0|0:0|0:2,0:0:0:0:
";

    fn test_slider_object() -> HitObject {
        let object_type = HitObjectType::Slider(SliderParams {
            curve_points: vec![
                SliderPoint { x: 200, y: 200 },
                SliderPoint { x: 250, y: 200 },
                SliderPoint { x: 250, y: 200 },
                SliderPoint { x: 300, y: 150 },
            ],
            slider_type: SliderType::Bezier,
            length: 310.123,
            slides: 2,
            edge_sounds: EdgeSounds {
                sounds: vec![2, 1, 2],
                sets: vec![(0, 0), (0, 0), (0, 2)],
            },
        });
        HitObject {
            x: 100,
            y: 100,
            time: 12600,
            object_params: object_type,
            new_combo: true,
            combo_skip: 0,
            hit_sound: HitSoundFlag::default(),
            hit_sample: HitSample::default(),
        }
    }
    fn test_spinner_object() -> HitObject {
        let mut spinner = HitObject {
            x: 256,
            y: 192,
            time: 11200,
            object_params: HitObjectType::Spinner(SpinnerParams { end_time: 12000 }),
            new_combo: false,
            combo_skip: 0,
            hit_sound: HitSoundFlag::FINISH | HitSoundFlag::CLAP,
            hit_sample: HitSample::default(),
        };
        spinner.hit_sample.volume = 80;
        spinner.hit_sample.normal_set = SampleSet::Drum;
        spinner
    }
    fn test_circle_object() -> HitObject {
        HitObject {
            x: 256,
            y: 192,
            time: 11000,
            object_params: HitObjectType::HitCircle,
            new_combo: true,
            combo_skip: 1,
            hit_sound: HitSoundFlag::WHISTLE,
            hit_sample: HitSample::default(),
        }
    }

    #[test]
    fn parse_hit_objects() {
        let hit_objects: CommaListOf<HitObject> = CommaListOf::parse(TEST_SECTION).unwrap();

        assert_eq!(hit_objects.len(), 3);
    }

    #[test]
    fn serialize_hit_objects() {
        let mut hit_objects: CommaListOf<HitObject> = CommaListOf::new();

        hit_objects.push(test_circle_object());
        hit_objects.push(test_spinner_object());
        hit_objects.push(test_slider_object());

        assert_eq!(hit_objects.serialize(), TEST_SECTION)
    }

    mod hit_object {
        use super::*;

        const TEST_HIT_CIRCLE: &'static str = "256,192,11000,21,2,0:0:0:0:";
        const TEST_SPINNER: &'static str = "256,192,11200,8,12,12000,3:0:0:80:";
        const TEST_SLIDER: &'static str = "100,100,12600,6,1,B|200:200|250:200|250:200|300:150,2,310.123,2|1|2,0:0|0:0|0:2,0:0:0:0:";

        #[test]
        fn parse_hit_circle() {
            let hit_circle = HitObject::parse(TEST_HIT_CIRCLE).unwrap();

            assert_eq!(hit_circle.x, 256);
            assert_eq!(hit_circle.y, 192);
            assert_eq!(hit_circle.time, 11000);
            assert_eq!(hit_circle.object_params, HitObjectType::HitCircle);
            assert_eq!(hit_circle.new_combo, true);
            assert_eq!(hit_circle.combo_skip, 1);
            assert_eq!(hit_circle.hit_sound, HitSoundFlag::WHISTLE);
            assert_eq!(hit_circle.hit_sample, HitSample::default());
        }

        #[test]
        fn parse_spinner() {
            let spinner = HitObject::parse(TEST_SPINNER).unwrap();

            assert_eq!(spinner.x, 256);
            assert_eq!(spinner.y, 192);
            assert_eq!(spinner.time, 11200);
            assert_eq!(
                spinner.object_params,
                HitObjectType::Spinner(SpinnerParams { end_time: 12000 })
            );
            assert_eq!(spinner.new_combo, false);
            assert_eq!(spinner.combo_skip, 0);
            assert_eq!(spinner.hit_sound, HitSoundFlag::FINISH | HitSoundFlag::CLAP);
            assert_eq!(spinner.hit_sample.normal_set, SampleSet::Drum);
            assert_eq!(spinner.hit_sample.volume, 80);
        }

        #[test]
        fn parse_slider() {
            let slider = HitObject::parse(TEST_SLIDER).unwrap();
            let slider_params: SliderParams = slider.object_params.try_into_inner().unwrap();

            assert_eq!(slider.x, 100);
            assert_eq!(slider.y, 100);
            assert_eq!(slider.time, 12600);
            assert_eq!(slider.new_combo, true);
            assert_eq!(slider.combo_skip, 0);
            assert_eq!(slider.hit_sound, HitSoundFlag::default());
            assert_eq!(slider.hit_sample, HitSample::default());
            assert_eq!(slider_params.curve_points.len(), 4);
            assert_eq!(slider_params.slider_type, SliderType::Bezier);
            assert_eq!(slider_params.length, 310.123);
            assert_eq!(slider_params.slides, 2);
            assert_eq!(slider_params.edge_sounds.sounds.len(), 3);
            assert_eq!(slider_params.edge_sounds.sets.len(), 3);
        }

        #[test]
        fn serialize_hit_circle() {
            assert_eq!(test_circle_object().serialize(), TEST_HIT_CIRCLE)
        }

        #[test]
        fn serialize_spinner() {
            assert_eq!(test_spinner_object().serialize(), TEST_SPINNER)
        }

        #[test]
        fn serialize_slider() {
            assert_eq!(test_slider_object().serialize(), TEST_SLIDER)
        }
    }
}
