// theme.rs

use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

pub trait Palette {
    fn colour(&self, index: usize) -> &Rgb;
    fn name(&self) -> Option<&str>;
    fn author(&self) -> Option<&str>;
    fn background(&self) -> &Rgb;
    fn iter(&self) -> Box<dyn Iterator<Item = Rgb> + '_>;
}

#[derive(Clone, Copy)]
pub struct StaticPalette {
    rgb: [Rgb; 16],
    name: Option<&'static str>,
    author: Option<&'static str>,
}

#[derive(Clone)]
pub struct CustomPalette {
    rgb: [Rgb; 16],
    name: Option<String>,
    author: Option<String>,
}

impl StaticPalette {
    pub const fn new(
        rgb: [Rgb; 16],
        name: Option<&'static str>,
        author: Option<&'static str>,
    ) -> Self {
        Self { rgb, name, author }
    }
}

impl CustomPalette {
    pub fn new(rgb: [Rgb; 16], name: Option<String>, author: Option<String>) -> Self {
        Self { rgb, name, author }
    }
}

impl Palette for StaticPalette {
    fn colour(&self, index: usize) -> &Rgb {
        assert!(index < 16, "palette index out of range: {index}");
        &self.rgb[index]
    }

    fn name(&self) -> Option<&str> {
        self.name
    }

    fn author(&self) -> Option<&str> {
        self.author
    }

    fn background(&self) -> &Rgb {
        &self.rgb[0]
    }

    fn iter(&self) -> Box<dyn Iterator<Item = Rgb> + '_> {
        Box::new(self.rgb.iter().copied())
    }
}

impl Palette for CustomPalette {
    fn colour(&self, index: usize) -> &Rgb {
        assert!(index < 16, "palette index out of range: {index}");
        &self.rgb[index]
    }

    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }

    fn background(&self) -> &Rgb {
        &self.rgb[0]
    }

    fn iter(&self) -> Box<dyn Iterator<Item = Rgb> + '_> {
        Box::new(self.rgb.iter().copied())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Deserialize)]
pub struct Scheme {
    pub system: String,
    pub name: String,
    pub author: String,
    pub variant: String,
    pub palette: SchemePalette,
}

#[derive(Debug, Deserialize)]
pub struct SchemePalette {
    pub base00: String,
    pub base01: String,
    pub base02: String,
    pub base03: String,
    pub base04: String,
    pub base05: String,
    pub base06: String,
    pub base07: String,
    pub base08: String,
    pub base09: String,
    #[serde(rename = "base0A")]
    pub base0a: String,
    #[serde(rename = "base0B")]
    pub base0b: String,
    #[serde(rename = "base0C")]
    pub base0c: String,
    #[serde(rename = "base0D")]
    pub base0d: String,
    #[serde(rename = "base0E")]
    pub base0e: String,
    #[serde(rename = "base0F")]
    pub base0f: String,

    // Base24 additions
    pub base10: String,
    pub base11: String,
    pub base12: String,
    pub base13: String,
    pub base14: String,
    pub base15: String,
    pub base16: String,
    pub base17: String,
}

fn parse_hex(s: &str) -> Result<Rgb, Box<dyn Error>> {
    let s = s.trim().trim_start_matches('#');
    if s.len() != 6 {
        return Err(format!("invalid hexadecimal colour: {s}").into());
    }
    Ok(Rgb {
        r: u8::from_str_radix(&s[0..2], 16)?,
        g: u8::from_str_radix(&s[2..4], 16)?,
        b: u8::from_str_radix(&s[4..6], 16)?,
    })
}

pub fn iter_themes() -> impl Iterator<Item = (&'static str, &'static dyn Palette)> {
    THEMES
        .iter()
        .map(|(name, palette)| (*name, *palette as &'static dyn Palette))
}

pub fn get_theme(key: &str) -> Option<&'static StaticPalette> {
    THEMES
        .iter()
        .find(|(name, _)| *name == key)
        .map(|(_, palette)| *palette)
}

pub fn load_file(path: &Path) -> Result<CustomPalette, Box<dyn Error>> {
    if !path.exists() {
        return Err(format!("failed to load theme: '{path:?}' does not exist.").into());
    }
    if !path.is_file() {
        return Err(format!("failed to load theme: '{path:?}' is not a file.").into());
    }

    let file = fs::File::open(path)?;
    let scheme: Scheme = serde_yaml::from_reader(file)?;
    let theme = scheme.palette;

    Ok(CustomPalette::new(
        [
            parse_hex(&theme.base00)?,
            parse_hex(&theme.base01)?,
            parse_hex(&theme.base02)?,
            parse_hex(&theme.base03)?,
            parse_hex(&theme.base04)?,
            parse_hex(&theme.base05)?,
            parse_hex(&theme.base06)?,
            parse_hex(&theme.base07)?,
            parse_hex(&theme.base08)?,
            parse_hex(&theme.base09)?,
            parse_hex(&theme.base0a)?,
            parse_hex(&theme.base0b)?,
            parse_hex(&theme.base0c)?,
            parse_hex(&theme.base0d)?,
            parse_hex(&theme.base0e)?,
            parse_hex(&theme.base0f)?,
        ],
        Some(scheme.name),
        Some(scheme.author),
    ))
}

include!(concat!(env!("OUT_DIR"), "/themes.rs"));
