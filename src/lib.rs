use crate::error::BeatmapParseError;
use crate::section::colours::Colours;
use crate::section::difficulty::DifficultySection;
use crate::section::editor::EditorSection;
use crate::section::events::Event;
use crate::section::general::GeneralSection;
use crate::section::hit_objects::HitObject;
use crate::section::metadata::MetadataSection;
use crate::section::timing_points::TimingPoint;
use crate::section::CommaListOf;
use crate::BeatmapParseError::SectionNotFound;
use std::str::FromStr;

mod error;
mod section;
mod types;

#[derive(Debug, Default)]
pub struct BeatmapLevel {
    pub general: GeneralSection,
    pub editor: EditorSection,
    pub metadata: MetadataSection,
    pub difficulty: DifficultySection,
    pub events: CommaListOf<Event>,
    pub timing_points: CommaListOf<TimingPoint>,
    pub colours: Colours,
    pub hit_objects: CommaListOf<HitObject>,
}

impl BeatmapLevel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(str: &str) -> Result<Self, BeatmapParseError> {
        Self::from_str(str)
    }
}

impl FromStr for BeatmapLevel {
    type Err = BeatmapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let general_index = s.find("[General]").ok_or_else(|| SectionNotFound {
            section: "General".to_string(),
        })?;
        let editor_index = s.find("[Editor]").ok_or_else(|| SectionNotFound {
            section: "Editor".to_string(),
        })?;
        let metadata_index = s.find("[Metadata]").ok_or_else(|| SectionNotFound {
            section: "Metadata".to_string(),
        })?;
        let difficulty_index = s.find("[Difficulty]").ok_or_else(|| SectionNotFound {
            section: "Difficulty".to_string(),
        })?;
        let events_index = s.find("[Events]").ok_or_else(|| SectionNotFound {
            section: "Events".to_string(),
        })?;
        let timing_points_index = s.find("[TimingPoints]").ok_or_else(|| SectionNotFound {
            section: "TimingPoints".to_string(),
        })?;
        let colours_index = s.find("[Colours]").ok_or_else(|| SectionNotFound {
            section: "Colours".to_string(),
        })?;
        let hit_objects_index = s.find("[HitObjects]").ok_or_else(|| SectionNotFound {
            section: "HitObjects".to_string(),
        })?;

        let general_str = s[general_index..editor_index]
            .strip_prefix("[General]")
            .unwrap()
            .trim();
        let editor_str = s[editor_index..metadata_index]
            .strip_prefix("[Editor]")
            .unwrap()
            .trim();
        let metadata_str = s[metadata_index..difficulty_index]
            .strip_prefix("[Metadata]")
            .unwrap()
            .trim();
        let difficulty_str = s[difficulty_index..events_index]
            .strip_prefix("[Difficulty]")
            .unwrap()
            .trim();
        let events_str = s[events_index..timing_points_index]
            .strip_prefix("[Events]")
            .unwrap()
            .trim();
        let timing_points_str = s[timing_points_index..colours_index]
            .strip_prefix("[TimingPoints]")
            .unwrap()
            .trim();
        let colours_str = s[colours_index..hit_objects_index]
            .strip_prefix("[Colours]")
            .unwrap()
            .trim();
        let hit_objects_str = s[hit_objects_index..]
            .strip_prefix("[HitObjects]")
            .unwrap()
            .trim();

        Ok(BeatmapLevel {
            general: general_str.parse()?,
            editor: editor_str.parse()?,
            metadata: metadata_str.parse()?,
            difficulty: difficulty_str.parse()?,
            events: events_str.parse()?,
            timing_points: timing_points_str.parse()?,
            colours: colours_str.parse()?,
            hit_objects: hit_objects_str.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::BeatmapLevel;
    use std::fs::File;
    use std::io::Read;

    const TEST_BEATMAP_LEVEL_PATH: &'static str = "./assets/examples/test.osu";

    #[test]
    fn parse_beatmap_level() {
        let mut file = File::open(TEST_BEATMAP_LEVEL_PATH).unwrap();
        let buf = &mut String::new();
        file.read_to_string(buf).unwrap();

        let beatmap_level = BeatmapLevel::parse(buf).unwrap();

        println!("{:?}", beatmap_level)
    }
}
