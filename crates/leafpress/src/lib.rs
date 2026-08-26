// main.rs
pub mod highlights;
pub mod parser;
pub mod render;
pub mod theme;

use std::{error::Error, path::Path};

use highlights::Mapping;
use theme::Rgb;
use tree_sitter::{Language, Query};

pub use highlights::DEFAULT_MAPPINGS;

pub fn generate_svg(
    output: &Path,
    source: &[u8],
    language: &Language,
    query: &Query,
    palette: &[Rgb; 16],
    mapping: Option<&[Mapping]>,
    font_family: Option<&str>,
    font_size: Option<u8>,
) -> Result<(), Box<dyn Error>> {
    let mapping = mapping.unwrap_or(DEFAULT_MAPPINGS);
    let font_family = font_family.unwrap_or("monospace");
    let font_size = font_size.unwrap_or(14);

    let captures = parser::collect_captures(&source, language, &query)?;
    let tokens = render::make_tokens(source, &captures, palette, mapping);

    let output_path = Path::new(&output);

    render::render(
        source,
        &tokens,
        &output_path,
        palette,
        font_family,
        font_size,
    )
}
