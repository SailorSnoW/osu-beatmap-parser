use crate::error::BeatmapParseError;
use crate::error::BeatmapParseError::InvalidFormat;
use crate::section::{CommaListElement, Section};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum ColourType {
    Combo(u8),
    SliderTrackOverride,
    SliderBorder,
}

impl Default for ColourType {
    fn default() -> Self {
        ColourType::Combo(1)
    }
}

impl FromStr for ColourType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SliderTrackOverride" => Ok(ColourType::SliderTrackOverride),
            "SliderBorder" => Ok(ColourType::SliderBorder),
            _ if s.starts_with("Combo") => {
                let id = s.strip_prefix("Combo").unwrap();
                Ok(ColourType::Combo(u8::from_str(id).map_err(|_| ())?))
            }
            _ => Err(()),
        }
    }
}

impl ToString for ColourType {
    fn to_string(&self) -> String {
        match self {
            ColourType::Combo(id) => format!("Combo{}", id.to_string()),
            ColourType::SliderTrackOverride => String::from("SliderTrackOverride"),
            ColourType::SliderBorder => String::from("SliderBorder"),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// Representation of the red, green, and blue components of the colours.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl FromStr for Rgb {
    type Err = BeatmapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.trim().split(",").map(|x| x.trim()).collect();

        Ok(Rgb {
            red: u8::from_str(s[0]).map_err(|_| InvalidFormat {
                field: "red".to_string(),
            })?,
            green: u8::from_str(s[1]).map_err(|_| InvalidFormat {
                field: "green".to_string(),
            })?,
            blue: u8::from_str(s[2]).map_err(|_| InvalidFormat {
                field: "blue".to_string(),
            })?,
        })
    }
}

impl ToString for Rgb {
    fn to_string(&self) -> String {
        format!("{},{},{}", self.red, self.green, self.blue)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Colour {
    pub colour_of: ColourType,
    pub colour: Rgb,
}

impl FromStr for Colour {
    type Err = BeatmapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.trim().split(":").map(|x| x.trim()).collect();

        Ok(Colour {
            colour_of: ColourType::from_str(s[0]).map_err(|_| InvalidFormat {
                field: "colour".to_string(),
            })?,
            colour: Rgb::from_str(s[1]).map_err(|_| InvalidFormat {
                field: "colour".to_string(),
            })?,
        })
    }
}

impl ToString for Colour {
    fn to_string(&self) -> String {
        format!(
            "{} : {}",
            self.colour_of.to_string(),
            self.colour.to_string()
        )
    }
}

impl CommaListElement for Colour {}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// Combo and skin colours.
#[derive(Debug, Default)]
pub struct Colours {
    /// Additive combo colours
    pub combos: [Option<Colour>; 8],
    /// Additive slider track colour
    pub slider_track_override: Option<Colour>,
    /// Slider border colour
    pub slider_border: Option<Colour>,
}

impl FromStr for Colours {
    type Err = BeatmapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut colours = Colours::new();
        let s: Vec<&str> = s.trim().split("\n").map(|x| x.trim()).collect();

        for x in s.iter() {
            let colour = Colour::from_str(x)?;

            match colour.colour_of {
                ColourType::Combo(x) => colours.combos[x as usize - 1] = Some(colour),
                ColourType::SliderTrackOverride => colours.slider_track_override = Some(colour),
                ColourType::SliderBorder => colours.slider_border = Some(colour),
            }
        }

        Ok(colours)
    }
}

impl ToString for Colours {
    fn to_string(&self) -> String {
        let mut buf = String::new();

        for combo in &self.combos {
            match combo {
                Some(c) => {
                    buf.push_str(&c.to_string());
                    buf.push_str("\n");
                }
                None => (),
            }
        }

        match &self.slider_track_override {
            Some(s) => {
                buf.push_str(&s.to_string());
                buf.push_str("\n");
            }
            None => (),
        }

        match &self.slider_border {
            Some(s) => {
                buf.push_str(&s.to_string());
                buf.push_str("\n");
            }
            None => (),
        }

        buf
    }
}

impl Section for Colours {}

#[cfg(test)]
mod tests {
    use crate::section::colours::{Colour, ColourType, Colours, Rgb};
    use crate::section::Section;

    const TEST_COLOURS: &'static str = "Combo1 : 255,0,0
Combo2 : 202,202,202
";

    #[test]
    fn parse_colours() {
        let colours = Colours::parse(TEST_COLOURS).unwrap();

        let colours_combo_one = colours.combos[0].as_ref().unwrap();
        let colours_combo_two = colours.combos[1].as_ref().unwrap();

        assert_eq!(colours_combo_one.colour_of, ColourType::Combo(1));
        assert_eq!(colours_combo_one.colour.red, 255);
        assert_eq!(colours_combo_one.colour.green, 0);
        assert_eq!(colours_combo_one.colour.blue, 0);
        assert_eq!(colours_combo_two.colour_of, ColourType::Combo(2));
        assert_eq!(colours_combo_two.colour.red, 202);
        assert_eq!(colours_combo_two.colour.green, 202);
        assert_eq!(colours_combo_two.colour.blue, 202);

        for i in 2..8 as usize {
            assert_eq!(colours.combos[i], None);
        }

        assert_eq!(colours.slider_track_override, None);
        assert_eq!(colours.slider_border, None);
    }

    #[test]
    fn serialize_colours() {
        let mut colours = Colours::new();
        let colours_combo_one = Colour {
            colour_of: ColourType::Combo(1),
            colour: Rgb {
                red: 255,
                green: 0,
                blue: 0,
            },
        };
        let colours_combo_two = Colour {
            colour_of: ColourType::Combo(2),
            colour: Rgb {
                red: 202,
                green: 202,
                blue: 202,
            },
        };
        colours.combos[0] = Some(colours_combo_one);
        colours.combos[1] = Some(colours_combo_two);

        assert_eq!(colours.serialize(), TEST_COLOURS);
    }

    mod colour_type {
        use crate::section::colours::ColourType;
        use core::str::FromStr;

        const TEST_COMBO: &'static str = "Combo2";
        const TEST_SLIDER_TRACK_OVERRIDE: &'static str = "SliderTrackOverride";
        const TEST_SLIDER_BORDER: &'static str = "SliderBorder";

        #[test]
        fn parse_colour_type() {
            let combo = ColourType::from_str(TEST_COMBO).unwrap();
            let slider_track_override = ColourType::from_str(TEST_SLIDER_TRACK_OVERRIDE).unwrap();
            let slider_border = ColourType::from_str(TEST_SLIDER_BORDER).unwrap();

            assert_eq!(combo, ColourType::Combo(2));
            assert_eq!(slider_track_override, ColourType::SliderTrackOverride);
            assert_eq!(slider_border, ColourType::SliderBorder);
        }

        #[test]
        fn serialize_colour_type() {
            let combo = ColourType::Combo(2);
            let slider_track_override = ColourType::SliderTrackOverride;
            let slider_border = ColourType::SliderBorder;

            assert_eq!(combo.to_string(), TEST_COMBO);
            assert_eq!(
                slider_track_override.to_string(),
                TEST_SLIDER_TRACK_OVERRIDE
            );
            assert_eq!(slider_border.to_string(), TEST_SLIDER_BORDER);
        }
    }

    mod rgb {
        use crate::section::colours::Rgb;
        use core::str::FromStr;

        const TEST_RGB: &'static str = "255,202,202";

        #[test]
        fn parse_rgb() {
            let rgb = Rgb::from_str(TEST_RGB).unwrap();

            assert_eq!(rgb.red, 255);
            assert_eq!(rgb.green, 202);
            assert_eq!(rgb.blue, 202);
        }

        #[test]
        fn serialize_rgb() {
            let rgb = Rgb {
                red: 255,
                green: 202,
                blue: 202,
            };

            assert_eq!(rgb.to_string(), TEST_RGB)
        }
    }
}
