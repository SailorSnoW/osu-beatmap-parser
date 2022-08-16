use crate::error::BeatmapParseError;
use crate::section::{Section, SectionKeyValue};
use crate::types::general::*;
use std::i32;
use std::str::FromStr;

#[derive(Default, Debug)]
pub struct GeneralSection {
    /// Location of the audio file relative to the current folder
    pub audio_filename: String,
    /// Milliseconds of silence before the audio starts playing
    pub audio_lead_in: i32, // Default: 0
    #[deprecated]
    pub audio_hash: String,
    /// Time in milliseconds when the audio preview should start
    pub preview_time: i32, // Default: -1
    /// Speed of the countdown before the first hit object
    /// (0 = no countdown, 1 = normal, 2 = half, 3 = double)
    pub countdown: Countdown,
    /// Sample set that will be used if timing points do not override it
    /// (Normal, Soft, Drum)
    pub sample_set: SampleSet,
    /// Multiplier for the threshold in time where hit objects placed close together stack (0–1)
    pub stack_leniency: f32,
    /// Game mode
    /// (0 = osu!, 1 = osu!taiko, 2 = osu!catch, 3 = osu!mania)
    pub mode: i32,
    /// Whether or not breaks have a letterboxing effect
    pub lb_in_breaks: bool,
    #[deprecated]
    pub story_fire_in_front: bool,
    /// Whether or not the storyboard can use the user's skin images
    pub use_skin_sprites: bool,
    #[deprecated]
    pub show_playfield: bool,
    /// Draw order of hit circle overlays compared to hit numbers
    pub overlay_pos: OverlayPosition,
    /// Preferred skin to use during gameplay
    pub skin_preference: String,
    /// Whether or not a warning about flashing colours should be shown at the beginning of the map
    pub epilepsy_warn: bool,
    /// Time in beats that the countdown starts before the first hit object
    pub countdown_offset: i32,
    /// Whether or not the "N+1" style key layout is used for osu!mania
    pub special_style: bool,
    /// Whether or not the storyboard allows widescreen viewing
    pub widescreen_sb: bool,
    /// Whether or not sound samples will change rate when playing with speed-changing mods
    pub sample_match_pb_rate: bool,
}

impl Section for GeneralSection {}
impl SectionKeyValue for GeneralSection {}

impl FromStr for GeneralSection {
    type Err = BeatmapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.trim().split("\n").map(|x| x.trim()).collect();
        let mut s_iter = s.iter();
        let mut general = Self::default();

        general.audio_filename =
            <Self as SectionKeyValue>::get_field_name_value(&mut s_iter, "AudioFilename")?;
        general.audio_lead_in =
            <Self as SectionKeyValue>::get_field_name_value(&mut s_iter, "AudioLeadIn")?;

        Ok(general)
    }
}

#[cfg(test)]
mod tests {
    use crate::section::general::GeneralSection;
    use std::str::FromStr;

    const section_test: &'static str = r#"
        AudioFilename: marb.mp3
        AudioLeadIn: 0
        PreviewTime: 126478
        Countdown: 0
        SampleSet: Soft
        StackLeniency: 0.7
        Mode: 0
        LetterboxInBreaks: 1
        EpilepsyWarning: 1
        WidescreenStoryboard: 1
    "#;

    #[test]
    fn parse_general() {
        GeneralSection::from_str(section_test);
    }
}
