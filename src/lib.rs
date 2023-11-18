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

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::ops::Range;
use std::path::Path;
use std::str::FromStr;
use std::{fs, io};

mod error;
pub mod section;
pub mod types;

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
    pub fn open(path: &Path) -> Result<Self, Box<dyn Error>> {
        Ok(path.try_into()?)
    }
    pub fn save(&self, path: &Path) -> io::Result<()> {
        Ok(fs::write(path, self.to_string())?)
    }
}

impl TryFrom<File> for BeatmapLevel {
    type Error = Box<dyn Error>;

    fn try_from(mut value: File) -> Result<Self, Self::Error> {
        let buf = &mut String::new();
        value.read_to_string(buf)?;
        Ok(BeatmapLevel::from_str(buf)?)
    }
}

impl TryFrom<&Path> for BeatmapLevel {
    type Error = Box<dyn Error>;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        Ok(File::open(value)?.try_into()?)
    }
}

struct Section {
    name: String,
    range: Option<Range<usize>>,
}

impl FromStr for BeatmapLevel {
    type Err = BeatmapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Section names and whether they are mandatory
        // Has to be in order
        let sections = vec![
            ("[General]", true),
            ("[Editor]", true),
            ("[Metadata]", true),
            ("[Difficulty]", true),
            ("[Events]", true),
            ("[TimingPoints]", false),
            ("[Colours]", false),
            ("[HitObjects]", true),
        ];

        // This next section checks if all mandatory sections are present
        // and it maps the range based on if the section is
        let mut processed_sections = Vec::new();

        let mut previous_index_option = None;
        let mut previous_section_name = "";
        for (section_name, mandatory) in sections {
            match s.find(section_name) {
                None => {
                    if mandatory {
                        return Err(SectionNotFound {
                            section: section_name.to_string(),
                        });
                    } else {
                        processed_sections.push(Section {
                            name: section_name.to_string(),
                            range: None,
                        });
                    }
                }
                Some(index) => {
                    if let Some(previous_index) = previous_index_option {
                        processed_sections.push(Section {
                            name: previous_section_name.to_string(),
                            range: Some(previous_index..index),
                        });
                    }
                    previous_index_option = Some(index);
                    previous_section_name = section_name;
                }
            }
        }
        // Adding last section
        processed_sections.push(Section {
            name: previous_section_name.to_string(),
            range: Some(previous_index_option.unwrap()..s.len()),
        });

        // Stripping sections and moving them to a hashmap
        let mut section_strings = HashMap::new();
        processed_sections.into_iter().for_each(|section| {
            let section_str = section
                .range
                .map(|range| s[range].strip_prefix(&section.name).unwrap().trim());
            section_strings.insert(section.name, section_str);
        });

        Ok(BeatmapLevel {
            general: section_strings.get("[General]").unwrap().unwrap().parse()?,
            editor: section_strings.get("[Editor]").unwrap().unwrap().parse()?,
            metadata: section_strings
                .get("[Metadata]")
                .unwrap()
                .unwrap()
                .parse()?,
            difficulty: section_strings
                .get("[Difficulty]")
                .unwrap()
                .unwrap()
                .parse()?,
            events: section_strings.get("[Events]").unwrap().unwrap().parse()?,
            timing_points: match section_strings.get("[TimingPoints]").unwrap() {
                Some(timing_str) => timing_str.parse()?,
                None => CommaListOf::from(Vec::new()),
            },
            colours: match section_strings.get("[Colours]").unwrap() {
                Some(colour_str) => colour_str.parse()?,
                None => Colours::default(),
            },
            hit_objects: section_strings
                .get("[HitObjects]")
                .unwrap()
                .unwrap()
                .parse()?,
        })
    }
}

impl ToString for BeatmapLevel {
    fn to_string(&self) -> String {
        format! {"osu file format v14\n\
        \n\
        [General]\n\
        {}\n\
        [Editor]\n\
        {}\n\
        [Metadata]\n\
        {}\n\
        [Difficulty]\n\
        {}\n\
        [Events]\n\
        {}\n\
        [TimingPoints]\n\
        {}\n\
        [Colours]\n\
        {}\n\
        [HitObjects]\n\
        {}", self.general.to_string(), self.editor.to_string(), self.metadata.to_string(),
        self.difficulty.to_string(), self.events.to_string(), self.timing_points.to_string(),
        self.colours.to_string(), self.hit_objects.to_string()}
    }
}

#[cfg(test)]
mod tests {
    use crate::BeatmapLevel;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    const TEST_BEATMAP_LEVEL_PATH: &'static str = "./assets/examples/test.osu";
    const OUTPUT_BEATMAP_LEVEL_PATH: &'static str = "./assets/examples/test_output.osu";

    #[test]
    fn parse_save_beatmap_level() {
        let mut file = File::open(TEST_BEATMAP_LEVEL_PATH).unwrap();
        let buf = &mut String::new();
        file.read_to_string(buf).unwrap();

        let beatmap_level = BeatmapLevel::parse(buf).unwrap();
        beatmap_level
            .save(&Path::new(OUTPUT_BEATMAP_LEVEL_PATH))
            .unwrap();
    }
}
