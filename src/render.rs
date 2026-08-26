// render.rs

use cairo::{Context as CairoContext, SvgSurface};
use pango::prelude::FontMapExt;
use pango::{AttrColor, AttrList, FontDescription, Style, Underline, Weight};

use crate::highlights::{Highlight, Mapping, get_colour};
use crate::parser::Capture;
use crate::theme::Rgb;
use tree_sitter::Query;

use std::{error::Error, path::Path};

#[derive(Debug)]
pub struct Token {
    start: usize,
    end: usize,
    hl: Highlight,
    colour: Rgb,
}

pub fn render(
    source: &[u8],
    tokens: &[Token],
    output: &Path,
    colours: &[Rgb; 16],
    font_family: &str,
    font_size: u8,
) -> Result<(), Box<dyn Error>> {
    let text = std::str::from_utf8(source)?;

    let font_map = pangocairo::FontMap::default();
    let context = font_map.create_context();
    let layout = pango::Layout::new(&context);
    layout.set_text(&text);

    let attrs = AttrList::new();
    let mut output_position = 0usize;
    for token in tokens {
        let length = token.end - token.start;
        let start = output_position as i32;
        let end = (output_position + length) as i32;

        let rgb = token.colour;
        let mut attr: pango::Attribute =
            AttrColor::new_foreground(rgb.r as u16 * 257, rgb.g as u16 * 257, rgb.b as u16 * 257)
                .into();
        attr.set_start_index(start as u32);
        attr.set_end_index(end as u32);
        attrs.insert(attr);

        if token.hl.bold {
            let mut attr: pango::Attribute = pango::AttrInt::new_weight(Weight::Bold).into();
            attr.set_start_index(start as u32);
            attr.set_end_index(end as u32);
            attrs.insert(attr);
        }
        if token.hl.italic {
            let mut attr: pango::Attribute = pango::AttrInt::new_style(Style::Italic).into();
            attr.set_start_index(start as u32);
            attr.set_end_index(end as u32);
            attrs.insert(attr);
        }
        if token.hl.underline {
            let mut attr: pango::Attribute =
                pango::AttrInt::new_underline(Underline::Single).into();
            attr.set_start_index(start as u32);
            attr.set_end_index(end as u32);
            attrs.insert(attr);
        }
        if token.hl.undercurl {
            let mut attr: pango::Attribute = pango::AttrInt::new_underline(Underline::Error).into();
            attr.set_start_index(start as u32);
            attr.set_end_index(end as u32);
            attrs.insert(attr);
        }
        if token.hl.strikethrough {
            let mut attr: pango::Attribute = pango::AttrInt::new_strikethrough(true).into();
            attr.set_start_index(start as u32);
            attr.set_end_index(end as u32);
            attrs.insert(attr);
        }
        output_position += length;
    }
    layout.set_attributes(Some(&attrs));

    let font_string = format!("{font_family} {font_size}");
    let font = FontDescription::from_string(&font_string);
    layout.set_font_description(Some(&font));
    let (ink, _) = layout.pixel_extents();
    let width = ink.width();
    let height = ink.height();
    let padding = 20.0;
    let surface = SvgSurface::new(
        width as f64 + padding * 2.0,
        height as f64 + padding * 2.0,
        Some(output),
    )?;
    let cr = CairoContext::new(&surface)?;
    let background = colours[0];
    cr.set_source_rgb(
        background.r as f64 / 255.0,
        background.g as f64 / 255.0,
        background.b as f64 / 255.0,
    );
    cr.paint()?;
    cr.move_to(padding, padding);
    pangocairo::functions::update_layout(&cr, &layout);
    pangocairo::functions::show_layout(&cr, &layout);
    cr.status()?;
    surface.finish();
    println!("written {}", output.display());
    Ok(())
}

pub fn make_tokens(
    source: &[u8],
    captures: &[Capture],
    query: &Query,
    colours: &[Rgb; 16],
    mapping: &[Mapping],
) -> Vec<Token> {
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

            if next.map(|capture| capture.index) != active.map(|capture| capture.index) {
                break;
            }

            end += 1;
        }

        let (hl, colour) = match active {
            Some(capture) => {
                let name = query.capture_names()[capture.index as usize];
                let hl = get_colour(name, mapping);
                (hl, colours[hl.colour])
            }
            None => (Highlight::default(), colours[5]),
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
