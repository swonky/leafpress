// render.rs

use cairo::{Context as CairoContext, SvgSurface};
use pango::prelude::FontMapExt;
use pango::{AttrList, FontDescription, Style, Underline, Weight};
use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

use pangocairo::FontMap;
use pangocairo::functions;

use crate::fops;
use crate::highlights::{Highlight, HighlightMap};
use crate::parser::Capture;
use crate::theme::{Palette, Rgb};

use std::{error::Error, path::Path};

#[derive(Debug, Clone)]
pub struct Format {
    font_family: String,
    font_size: u8,
    padding: f64,
}

impl Default for Format {
    fn default() -> Self {
        Self {
            font_family: "monospace".to_string(),
            font_size: 14,
            padding: 20.0,
        }
    }
}

impl Format {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn font_family(mut self, font_family: impl Into<String>) -> Self {
        self.font_family = font_family.into();
        self
    }

    pub fn font_size(mut self, font_size: u8) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn padding(mut self, padding: f64) -> Self {
        self.padding = padding;
        self
    }

    pub fn get_font_family(&self) -> &str {
        &self.font_family
    }

    pub fn get_font_size(&self) -> u8 {
        self.font_size
    }

    pub fn get_padding(&self) -> f64 {
        self.padding
    }

    pub fn width(&self, ink_width: i32) -> f64 {
        ink_width as f64 + self.padding * 2.0
    }

    pub fn height(&self, ink_height: i32) -> f64 {
        ink_height as f64 + self.padding * 2.0
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    start: usize,
    end: usize,
    hl: Highlight,
    colour: &'a Rgb,
}

struct VecWriter {
    buffer: Rc<RefCell<Vec<u8>>>,
}

impl Write for VecWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.borrow_mut().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn render_context(
    cr: &CairoContext,
    layout: &pango::Layout,
    background: &Rgb,
    padding: f64,
) -> Result<(), Box<dyn Error>> {
    cr.set_source_rgb(
        background.r as f64 / 255.0,
        background.g as f64 / 255.0,
        background.b as f64 / 255.0,
    );
    cr.paint()?;

    cr.move_to(padding, padding);
    functions::update_layout(cr, layout);
    functions::show_layout(cr, layout);

    cr.status()?;

    Ok(())
}

fn make_layout(
    source: &[u8],
    tokens: &[Token],
    font_family: &str,
    font_size: u8,
) -> Result<(pango::Layout, i32, i32), Box<dyn Error>> {
    let text = std::str::from_utf8(source)?;

    let font_map = FontMap::default();
    let context = font_map.create_context();
    let layout = pango::Layout::new(&context);
    layout.set_text(text);

    let attrs = AttrList::new();

    // possibly unnecessary - should remove if ligatures work without it
    let mut features: pango::Attribute = pango::AttrFontFeatures::new("'calt' 1").into();
    features.set_start_index(0);
    features.set_end_index(text.len() as u32);
    attrs.insert(features);

    let mut output_position = 0usize;

    for token in tokens {
        let length = token.end - token.start;
        let start = output_position as u32;
        let end = (output_position + length) as u32;

        // println!("{}", &text[start as usize..end as usize]);

        let rgb = token.colour;
        let mut attr: pango::Attribute = pango::AttrColor::new_foreground(
            rgb.r as u16 * 257,
            rgb.g as u16 * 257,
            rgb.b as u16 * 257,
        )
        .into();
        attr.set_start_index(start);
        attr.set_end_index(end);
        attrs.insert(attr);

        if token.hl.bold {
            let mut attr: pango::Attribute = pango::AttrInt::new_weight(Weight::Bold).into();
            attr.set_start_index(start);
            attr.set_end_index(end);
            attrs.insert(attr);
        }

        if token.hl.italic {
            let mut attr: pango::Attribute = pango::AttrInt::new_style(Style::Italic).into();
            attr.set_start_index(start);
            attr.set_end_index(end);
            attrs.insert(attr);
        }

        if token.hl.underline {
            let mut attr: pango::Attribute =
                pango::AttrInt::new_underline(Underline::Single).into();
            attr.set_start_index(start);
            attr.set_end_index(end);
            attrs.insert(attr);
        }

        if token.hl.undercurl {
            let mut attr: pango::Attribute = pango::AttrInt::new_underline(Underline::Error).into();
            attr.set_start_index(start);
            attr.set_end_index(end);
            attrs.insert(attr);
        }

        if token.hl.strikethrough {
            let mut attr: pango::Attribute = pango::AttrInt::new_strikethrough(true).into();
            attr.set_start_index(start);
            attr.set_end_index(end);
            attrs.insert(attr);
        }

        output_position += length;
    }

    layout.set_attributes(Some(&attrs));

    let font_string = format!("{font_family} {font_size}");
    let font = FontDescription::from_string(&font_string);
    layout.set_font_description(Some(&font));
    let (ink, _) = layout.pixel_extents();

    Ok((layout, ink.width(), ink.height()))
}

pub fn render(
    source: &[u8],
    tokens: &[Token],
    format: &Format,
    background: &Rgb,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let (layout, ink_width, ink_height) = make_layout(
        source,
        tokens,
        &format.get_font_family(),
        format.get_font_size(),
    )?;

    let buffer = Rc::new(RefCell::new(Vec::new()));
    let writer = VecWriter {
        buffer: Rc::clone(&buffer),
    };

    {
        let surface =
            SvgSurface::for_stream(format.width(ink_width), format.height(ink_height), writer)?;
        let cr = CairoContext::new(&surface)?;

        render_context(&cr, &layout, background, format.get_padding())?;

        surface.finish();
    }

    Ok(buffer.borrow().clone())
}

pub fn render_to_file(
    source: &[u8],
    tokens: &[Token],
    format: &Format,
    background: &Rgb,
    output: &Path,
) -> Result<(), Box<dyn Error>> {
    fops::validate_output_path(output)?;

    let (layout, ink_width, ink_height) = make_layout(
        source,
        tokens,
        format.get_font_family(),
        format.get_font_size(),
    )?;

    let surface = SvgSurface::new(
        format.width(ink_width),
        format.height(ink_height),
        Some(output),
    )?;
    let cr = CairoContext::new(&surface)?;

    render_context(&cr, &layout, background, format.get_padding())?;

    surface.finish();

    Ok(())
}

pub fn make_tokens<'a, P: Palette>(
    source: &[u8],
    captures: &[Capture],
    palette: &'a P,
    mapping: &HighlightMap,
) -> Vec<Token<'a>> {
    let mut tokens = Vec::new();
    let mut position = 0usize;

    while position < source.len() {
        let active = captures
            .iter()
            .rev()
            .find(|capture| capture.start <= position && position < capture.end);

        let mut end = position + 1;
        while end < source.len() {
            let next = captures
                .iter()
                .rev()
                .find(|capture| capture.start <= end && end < capture.end);

            if next.map(|capture| capture.group) != active.map(|capture| capture.group) {
                break;
            }

            end += 1;
        }

        let (hl, colour) = match active {
            Some(capture) => {
                let hl = mapping.get(capture.group);
                (hl, palette.colour(hl.base))
            }
            None => (mapping.default, palette.colour(mapping.default.base)),
        };

        tokens.push(Token {
            start: position,
            end,
            hl,
            colour,
        });

        position = end;
    }

    tokens
}
