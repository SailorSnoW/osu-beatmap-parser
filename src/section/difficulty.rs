use crate::error::BeatmapParseError;
use crate::section::{Section, SectionKeyValue};
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct Difficulty {
    hp_drain_rate: f32,
    circle_size: f32,
    overall_difficulty: f32,
    approach_rate: f32,
    slider_multiplier: f32,
    slider_tick_rate: f32,
}

impl FromStr for Difficulty {
    type Err = BeatmapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.trim().split("\n").map(|x| x.trim()).collect();
        let mut difficulty = Self::new();

        difficulty.hp_drain_rate = Self::get_field_name_value(&s, "HPDrainRate")?;
        difficulty.circle_size = Self::get_field_name_value(&s, "CircleSize")?;
        difficulty.overall_difficulty = Self::get_field_name_value(&s, "OverallDifficulty")?;
        difficulty.approach_rate = Self::get_field_name_value(&s, "ApproachRate")?;
        difficulty.slider_multiplier = Self::get_field_name_value(&s, "SliderMultiplier")?;
        difficulty.slider_tick_rate = Self::get_field_name_value(&s, "SliderTickRate")?;

        Ok(difficulty)
    }
}

impl From<Difficulty> for String {
    fn from(section: Difficulty) -> Self {
        let mut buf = String::new();

        Difficulty::write_field_in(&mut buf, "HPDrainRate", &section.hp_drain_rate, false);
        Difficulty::write_field_in(&mut buf, "CircleSize", &section.circle_size, false);
        Difficulty::write_field_in(
            &mut buf,
            "OverallDifficulty",
            &section.overall_difficulty,
            false,
        );
        Difficulty::write_field_in(&mut buf, "ApproachRate", &section.approach_rate, false);
        Difficulty::write_field_in(
            &mut buf,
            "SliderMultiplier",
            &section.slider_multiplier,
            false,
        );
        Difficulty::write_field_in(&mut buf, "SliderTickRate", &section.slider_tick_rate, false);

        buf
    }
}

impl Section for Difficulty {}
impl SectionKeyValue for Difficulty {}

#[cfg(test)]
mod tests {
    use crate::section::difficulty::Difficulty;
    use crate::section::Section;

    const TEST_SECTION: &'static str = "HPDrainRate:5
CircleSize:4
OverallDifficulty:6
ApproachRate:8
SliderMultiplier:1.5
SliderTickRate:1
";

    #[test]
    fn parse_difficulty() {
        let difficulty = Difficulty::parse(TEST_SECTION).unwrap();

        assert_eq!(difficulty.hp_drain_rate, 5.0);
        assert_eq!(difficulty.circle_size, 4.0);
        assert_eq!(difficulty.overall_difficulty, 6.0);
        assert_eq!(difficulty.approach_rate, 8.0);
        assert_eq!(difficulty.slider_multiplier, 1.5);
        assert_eq!(difficulty.slider_tick_rate, 1.0);
    }

    #[test]
    fn serialize_difficulty() {
        let mut difficulty = Difficulty::new();
        difficulty.hp_drain_rate = 5.0;
        difficulty.circle_size = 4.0;
        difficulty.overall_difficulty = 6.0;
        difficulty.approach_rate = 8.0;
        difficulty.slider_multiplier = 1.5;
        difficulty.slider_tick_rate = 1.0;

        let serialized_difficulty = difficulty.serialize();

        assert_eq!(serialized_difficulty, TEST_SECTION)
    }
}
