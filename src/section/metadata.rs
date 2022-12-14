use crate::error::BeatmapParseError;
use crate::section::{Section, SectionKeyValue};
use std::str::FromStr;

/// [Information](https://osu.ppy.sh/wiki/en/Client/Beatmap_editor/Song_Setup#song-and-map-metadata)
/// used to identify the beatmap
#[derive(Default, Debug)]
pub struct MetadataSection {
    /// Romanised song title
    pub title: String,
    /// Song title
    pub title_unicode: String,
    /// Romanised song artist
    pub artist: String,
    /// Song artist
    pub artist_unicode: String,
    /// Beatmap creator
    pub creator: String,
    /// Difficulty name
    pub version: String,
    /// 	Original media the song was produced for
    pub source: String,
    /// Search terms
    pub tags: Vec<String>,
    /// Difficulty ID
    pub beatmap_id: i32,
    /// Beatmap ID
    pub beatmap_set_id: i32,
}

impl Section for MetadataSection {}

impl SectionKeyValue for MetadataSection {}

impl FromStr for MetadataSection {
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

impl ToString for MetadataSection {
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
    use crate::section::metadata::MetadataSection;
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
        let metadata = MetadataSection::from_str(TEST_SECTION).unwrap();

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
        let mut metadata = MetadataSection::new();
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
