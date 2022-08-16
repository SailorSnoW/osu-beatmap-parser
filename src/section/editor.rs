use crate::error::BeatmapParseError;
use crate::error::BeatmapParseError::InvalidFormat;
use crate::section::{Section, SectionKeyValue};
use std::str::FromStr;

#[derive(Default, Debug)]
pub struct EditorSection {
    pub bookmarks: Vec<i32>,
    pub distance_spacing: f32,
    pub beat_divisor: f32,
    pub grid_size: i32,
    pub timeline_zoom: f32,
}

impl Section for EditorSection {}

impl SectionKeyValue for EditorSection {}

impl FromStr for EditorSection {
    type Err = BeatmapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.trim().split("\n").map(|x| x.trim()).collect();
        let mut editor = Self::default();

        let bookmarks: String = Self::get_field_name_value(&s, "Bookmarks")?;

        editor.bookmarks = bookmarks
            .split(',')
            .map(|x| {
                i32::from_str(x)
                    .map_err(|_| InvalidFormat {
                        field: "Bookmarks".to_string(),
                    })
                    .unwrap()
            })
            .collect();
        editor.distance_spacing = Self::get_field_name_value(&s, "DistanceSpacing")?;
        editor.beat_divisor = Self::get_field_name_value(&s, "BeatDivisor")?;
        editor.grid_size = Self::get_field_name_value(&s, "GridSize")?;
        editor.timeline_zoom = Self::get_field_name_value(&s, "TimelineZoom")?;

        Ok(editor)
    }
}

impl From<EditorSection> for String {
    fn from(section: EditorSection) -> Self {
        let mut buf = String::new();
        let mut bookmarks = String::new();

        for bookmark in section.bookmarks.iter() {
            bookmarks.push_str(&bookmark.to_string());
            bookmarks.push(',');
        }

        if bookmarks.chars().count() > 0 {
            bookmarks.pop();
        }

        EditorSection::write_field_in(&mut buf, "Bookmarks", &bookmarks, true);
        EditorSection::write_field_in(&mut buf, "DistanceSpacing", &section.distance_spacing, true);
        EditorSection::write_field_in(&mut buf, "BeatDivisor", &section.beat_divisor, true);
        EditorSection::write_field_in(&mut buf, "GridSize", &section.grid_size, true);
        EditorSection::write_field_in(&mut buf, "TimelineZoom", &section.timeline_zoom, true);

        buf
    }
}

#[cfg(test)]
mod tests {
    use super::Section;
    use crate::section::editor::EditorSection;

    const TEST_SECTION: &'static str = "Bookmarks: 121309
DistanceSpacing: 0.5
BeatDivisor: 4
GridSize: 32
TimelineZoom: 1.6
";

    #[test]
    fn parse_editor() {
        let editor = EditorSection::parse(TEST_SECTION).unwrap();

        assert_eq!(editor.bookmarks.len(), 1);
        assert_eq!(editor.bookmarks[0], 121309);
        assert_eq!(editor.distance_spacing, 0.5);
        assert_eq!(editor.beat_divisor, 4.0);
        assert_eq!(editor.grid_size, 32);
        assert_eq!(editor.timeline_zoom, 1.6);
    }

    #[test]
    fn serialize_editor() {
        let mut editor = EditorSection::new();
        editor.bookmarks.push(121309);
        editor.distance_spacing = 0.5;
        editor.beat_divisor = 4.0;
        editor.grid_size = 32;
        editor.timeline_zoom = 1.6;

        let serialized_editor = editor.serialize();

        assert_eq!(serialized_editor, TEST_SECTION)
    }
}
