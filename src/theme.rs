// theme.rs

use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Deserialize)]
struct Base16Theme {
    base00: String,
    base01: String,
    base02: String,
    base03: String,
    base04: String,
    base05: String,
    base06: String,
    base07: String,
    base08: String,
    base09: String,
    #[serde(rename = "base0A")]
    base0a: String,
    #[serde(rename = "base0B")]
    base0b: String,
    #[serde(rename = "base0C")]
    base0c: String,
    #[serde(rename = "base0D")]
    base0d: String,
    #[serde(rename = "base0E")]
    base0e: String,
    #[serde(rename = "base0F")]
    base0f: String,
}

fn parse_hex(s: &str) -> Result<Rgb, Box<dyn Error>> {
    let s = s.trim().trim_start_matches('#');
    if s.len() != 6 {
        return Err(format!("invalid colour: {s}").into());
    }
    Ok(Rgb {
        r: u8::from_str_radix(&s[0..2], 16)?,
        g: u8::from_str_radix(&s[2..4], 16)?,
        b: u8::from_str_radix(&s[4..6], 16)?,
    })
}

pub fn load_base16(path: &Path) -> Result<[Rgb; 16], Box<dyn Error>> {
    let file = fs::File::open(path)?;
    let theme: Base16Theme = serde_yaml::from_reader(file)?;

    Ok([
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
    ])
}
