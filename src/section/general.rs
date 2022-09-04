use crate::error::BeatmapParseError;
use crate::section::{Section, SectionKeyValue};
use crate::types::general::*;
use crate::types::OsuBool;
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
    pub mode: Gamemode,
    /// Whether or not breaks have a letterboxing effect
    pub lb_in_breaks: OsuBool,
    #[deprecated]
    pub story_fire_in_front: OsuBool,
    /// Whether or not the storyboard can use the user'&s skin images
    pub use_skin_sprites: OsuBool,
    #[deprecated]
    pub show_playfield: OsuBool,
    /// Draw order of hit circle overlays compared to hit numbers
    pub overlay_pos: OverlayPosition,
    /// Preferred skin to use during gameplay
    pub skin_preference: String,
    /// Whether or not a warning about flashing colours should be shown at the beginning of the map
    pub epilepsy_warn: OsuBool,
    /// Time in beats that the countdown starts before the first hit object
    pub countdown_offset: i32,
    /// Whether or not the "N+1" style key layout is used for osu!mania
    pub special_style: OsuBool,
    /// Whether or not the storyboard allows widescreen viewing
    pub widescreen_sb: OsuBool,
    /// Whether or not sound samples will change rate when playing with speed-changing mods
    pub sample_match_pb_rate: OsuBool,
}

impl Section for GeneralSection {}
impl SectionKeyValue for GeneralSection {}

impl FromStr for GeneralSection {
    type Err = BeatmapParseError;

    #[allow(deprecated)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.trim().split("\n").map(|x| x.trim()).collect();
        let mut general = Self::default();

        general.audio_filename = Self::get_field_name_value(&s, "AudioFilename")?;
        general.audio_lead_in = Self::get_field_name_value(&s, "AudioLeadIn")?;
        general.audio_hash = Self::get_field_name_value(&s, "AudioHash")?;
        general.preview_time = Self::get_field_name_value(&s, "PreviewTime")?;
        general.countdown = Self::get_field_name_value(&s, "Countdown")?;
        general.sample_set = Self::get_field_name_value(&s, "SampleSet")?;
        general.stack_leniency = Self::get_field_name_value(&s, "StackLeniency")?;
        general.mode = Self::get_field_name_value(&s, "Mode")?;
        general.lb_in_breaks = Self::get_field_name_value(&s, "LetterboxInBreaks")?;
        general.story_fire_in_front = Self::get_field_name_value(&s, "StoryFireInFront")?;
        general.use_skin_sprites = Self::get_field_name_value(&s, "UseSkinSprites")?;
        general.show_playfield = Self::get_field_name_value(&s, "AlwaysShowPlayfield")?;
        general.overlay_pos = Self::get_field_name_value(&s, "OverlayPosition")?;
        general.skin_preference = Self::get_field_name_value(&s, "SkinPreference")?;
        general.epilepsy_warn = Self::get_field_name_value(&s, "EpilepsyWarning")?;
        general.countdown_offset = Self::get_field_name_value(&s, "CountdownOffset")?;
        general.special_style = Self::get_field_name_value(&s, "SpecialStyle")?;
        general.widescreen_sb = Self::get_field_name_value(&s, "WidescreenStoryboard")?;
        general.sample_match_pb_rate = Self::get_field_name_value(&s, "SamplesMatchPlaybackRate")?;

        Ok(general)
    }
}

impl ToString for GeneralSection {
    #[allow(deprecated)]
    fn to_string(&self) -> String {
        let mut buf = String::new();

        Self::write_field_in(&mut buf, "AudioFilename", &self.audio_filename, true);
        Self::write_field_in(&mut buf, "AudioLeadIn", &self.audio_lead_in, true);
        Self::write_field_in(&mut buf, "AudioHash", &self.audio_hash, true);
        Self::write_field_in(&mut buf, "PreviewTime", &self.preview_time, true);
        Self::write_field_in(&mut buf, "Countdown", &self.countdown, true);
        Self::write_field_in(&mut buf, "SampleSet", &self.sample_set, true);
        Self::write_field_in(&mut buf, "StackLeniency", &self.stack_leniency, true);
        Self::write_field_in(&mut buf, "Mode", &self.mode, true);
        Self::write_field_in(&mut buf, "LetterboxInBreaks", &self.lb_in_breaks, true);
        Self::write_field_in(
            &mut buf,
            "StoryFireInFront",
            &self.story_fire_in_front,
            true,
        );
        Self::write_field_in(&mut buf, "UseSkinSprites", &self.use_skin_sprites, true);
        Self::write_field_in(&mut buf, "AlwaysShowPlayfield", &self.show_playfield, true);
        Self::write_field_in(&mut buf, "OverlayPosition", &self.overlay_pos, true);
        Self::write_field_in(&mut buf, "SkinPreference", &self.skin_preference, true);
        Self::write_field_in(&mut buf, "EpilepsyWarning", &self.epilepsy_warn, true);
        Self::write_field_in(&mut buf, "CountdownOffset", &self.countdown_offset, true);
        Self::write_field_in(&mut buf, "SpecialStyle", &self.special_style, true);
        Self::write_field_in(&mut buf, "WidescreenStoryboard", &self.widescreen_sb, true);
        Self::write_field_in(
            &mut buf,
            "SamplesMatchPlaybackRate",
            &self.sample_match_pb_rate,
            true,
        );

        buf
    }
}

#[cfg(test)]
mod tests {
    use super::Section;
    use crate::section::general::GeneralSection;
    use crate::types::general::Countdown::NONE;
    use crate::types::general::Gamemode::STD;
    use crate::types::general::SampleSet::NORMAL;

    const SECTION_TEST: &'static str = "AudioFilename: marb.mp3
PreviewTime: 126478
Countdown: 0
StackLeniency: 0.7
LetterboxInBreaks: 1
EpilepsyWarning: 1
WidescreenStoryboard: 1
";

    #[test]
    fn parse_general() {
        let general = GeneralSection::parse(SECTION_TEST).unwrap();

        assert_eq!(general.audio_filename, "marb.mp3");
        assert_eq!(general.preview_time, 126478);
        assert_eq!(general.countdown, NONE);
        assert_eq!(general.stack_leniency, 0.7);
        assert_eq!(general.lb_in_breaks, true.into());
        assert_eq!(general.epilepsy_warn, true.into());
        assert_eq!(general.widescreen_sb, true.into());
    }

    #[test]
    fn serialize_general() {
        let mut general = GeneralSection::default();
        general.audio_filename = "marb.mp3".to_string();
        general.audio_lead_in = 0;
        general.preview_time = 126478;
        general.countdown = NONE;
        general.sample_set = NORMAL;
        general.stack_leniency = 0.7;
        general.mode = STD;
        general.lb_in_breaks = true.into();
        general.epilepsy_warn = true.into();
        general.widescreen_sb = true.into();

        let serialized_general = general.serialize();

        assert_eq!(serialized_general, SECTION_TEST);
    }
}
