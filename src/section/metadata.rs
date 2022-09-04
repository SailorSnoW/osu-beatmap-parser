use crate::error::BeatmapParseError;
use crate::section::{Section, SectionKeyValue};
use std::str::FromStr;

#[derive(Default, Debug)]
pub struct Metadata {
    pub title: String,
    pub title_unicode: String,
    pub artist: String,
    pub artist_unicode: String,
    pub creator: String,
    pub version: String,
    pub source: String,
    pub tags: Vec<String>,
    pub beatmap_id: i32,
    pub beatmap_set_id: i32,
}

impl Section for Metadata {}

impl SectionKeyValue for Metadata {}

impl FromStr for Metadata {
    type Err = BeatmapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.trim().split("\n").map(|x| x.trim()).collect();
        let mut metadata = Self::new();

        metadata.title = Self::get_field_name_value(&s, "Title")?;
        metadata.title_unicode = Self::get_field_name_value(&s, "TitleUnicode")?;
        metadata.artist = Self::get_field_name_value(&s, "Artist")?;
        metadata.artist_unicode = Self::get_field_name_value(&s, "ArtistUnicode")?;
        metadata.creator = Self::get_field_name_value(&s, "Creator")?;
        metadata.version = Self::get_field_name_value(&s, "Version")?;
        metadata.source = Self::get_field_name_value(&s, "Source")?;

        let tags: String = Self::get_field_name_value(&s, "Tags")?;
        metadata.tags = tags.split(' ').map(|x| x.to_string()).collect();

        metadata.beatmap_id = Self::get_field_name_value(&s, "BeatmapID")?;
        metadata.beatmap_set_id = Self::get_field_name_value(&s, "BeatmapSetID")?;

        Ok(metadata)
    }
}

impl ToString for Metadata {
    fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut tags = String::new();

        for tag in self.tags.iter() {
            tags.push_str(tag);
            tags.push(' ')
        }

        if tags.chars().count() > 0 {
            tags.pop();
        }

        Self::write_field_in(&mut buf, "Title", &self.title, false);
        Self::write_field_in(&mut buf, "TitleUnicode", &self.title_unicode, false);
        Self::write_field_in(&mut buf, "Artist", &self.artist, false);
        Self::write_field_in(&mut buf, "ArtistUnicode", &self.artist_unicode, false);
        Self::write_field_in(&mut buf, "Creator", &self.creator, false);
        Self::write_field_in(&mut buf, "Version", &self.version, false);
        Self::write_field_in(&mut buf, "Source", &self.source, false);
        Self::write_field_in(&mut buf, "Tags", &tags, false);
        Self::write_field_in(&mut buf, "BeatmapID", &self.beatmap_id, false);
        Self::write_field_in(&mut buf, "BeatmapSetID", &self.beatmap_set_id, false);

        buf
    }
}

#[cfg(test)]
mod tests {
    use crate::section::metadata::Metadata;
    use crate::section::Section;
    use std::str::FromStr;

    const TEST_SECTION: &'static str = "Title:Marble Soda
TitleUnicode:Marble Soda
Artist:Shawn Wasabi
ArtistUnicode:Shawn Wasabi
Creator:Len
Version:Crier's Hyper
Tags:Narcissu launchpad midi fighter Crier BetaStar mashup Fast
BeatmapID:846260
BeatmapSetID:387784
";
    #[test]
    fn parse_metadata() {
        let metadata = Metadata::from_str(TEST_SECTION).unwrap();

        assert_eq!(metadata.title, "Marble Soda");
        assert_eq!(metadata.title_unicode, "Marble Soda");
        assert_eq!(metadata.artist, "Shawn Wasabi");
        assert_eq!(metadata.artist_unicode, "Shawn Wasabi");
        assert_eq!(metadata.creator, "Len");
        assert_eq!(metadata.version, "Crier's Hyper");
        assert_eq!(metadata.source, "");
        assert_eq!(metadata.tags.len(), 8);
        assert_eq!(metadata.beatmap_id, 846260);
        assert_eq!(metadata.beatmap_set_id, 387784)
    }

    #[test]
    fn serialize_metadata() {
        let mut metadata = Metadata::new();
        metadata.title = String::from("Marble Soda");
        metadata.title_unicode = String::from("Marble Soda");
        metadata.artist = String::from("Shawn Wasabi");
        metadata.artist_unicode = String::from("Shawn Wasabi");
        metadata.creator = String::from("Len");
        metadata.version = String::from("Crier's Hyper");
        metadata.source = String::from("");
        metadata.tags.push(String::from("Narcissu"));
        metadata.tags.push(String::from("launchpad"));
        metadata.tags.push(String::from("midi"));
        metadata.tags.push(String::from("fighter"));
        metadata.tags.push(String::from("Crier"));
        metadata.tags.push(String::from("BetaStar"));
        metadata.tags.push(String::from("mashup"));
        metadata.tags.push(String::from("Fast"));
        metadata.beatmap_id = 846260;
        metadata.beatmap_set_id = 387784;

        let serialized_metadata = metadata.serialize();

        assert_eq!(serialized_metadata, TEST_SECTION);
    }
}
