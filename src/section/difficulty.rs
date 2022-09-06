use crate::error::BeatmapParseError;
use crate::section::{Section, SectionKeyValue};
use std::str::FromStr;

/// [Difficulty settings](https://osu.ppy.sh/wiki/en/Client/Beatmap_editor/Song_Setup#difficulty)
#[derive(Debug, Default)]
pub struct DifficultySection {
    /// HP setting (0–10)
    hp_drain_rate: f32,
    /// CS setting (0–10)
    circle_size: f32,
    /// OD setting (0–10)
    overall_difficulty: f32,
    /// AR setting (0–10)
    approach_rate: f32,
    /// Base slider velocity in hundreds of
    /// [osu! pixels](https://osu.ppy.sh/wiki/en/osupixel) per beat
    slider_multiplier: f32,
    /// Amount of slider ticks per beat
    slider_tick_rate: f32,
}

impl FromStr for DifficultySection {
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

impl ToString for DifficultySection {
    fn to_string(&self) -> String {
        let mut buf = String::new();

        Self::write_field_in(&mut buf, "HPDrainRate", &self.hp_drain_rate, false);
        Self::write_field_in(&mut buf, "CircleSize", &self.circle_size, false);
        Self::write_field_in(
            &mut buf,
            "OverallDifficulty",
            &self.overall_difficulty,
            false,
        );
        Self::write_field_in(&mut buf, "ApproachRate", &self.approach_rate, false);
        Self::write_field_in(&mut buf, "SliderMultiplier", &self.slider_multiplier, false);
        Self::write_field_in(&mut buf, "SliderTickRate", &self.slider_tick_rate, false);

        buf
    }
}

impl Section for DifficultySection {}
impl SectionKeyValue for DifficultySection {}

#[cfg(test)]
mod tests {
    use crate::section::difficulty::DifficultySection;
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
        let difficulty = DifficultySection::parse(TEST_SECTION).unwrap();

        assert_eq!(difficulty.hp_drain_rate, 5.0);
        assert_eq!(difficulty.circle_size, 4.0);
        assert_eq!(difficulty.overall_difficulty, 6.0);
        assert_eq!(difficulty.approach_rate, 8.0);
        assert_eq!(difficulty.slider_multiplier, 1.5);
        assert_eq!(difficulty.slider_tick_rate, 1.0);
    }

    #[test]
    fn serialize_difficulty() {
        let mut difficulty = DifficultySection::new();
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
