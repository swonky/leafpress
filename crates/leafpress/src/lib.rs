// main.rs
pub mod highlights;
pub mod parser;
pub mod render;
pub mod theme;
pub mod tree_sitter_config;

use std::{error::Error, path::Path};

use highlights::HighlightMap;
use theme::Palette;
use tree_sitter::{Language, Query};

pub use highlights::DEFAULT_MAPPING;

pub fn generate_svg<P: Palette>(
    output: &Path,
    source: &[u8],
    language: &Language,
    query: &Query,
    palette: &P,
    mapping: Option<&HighlightMap>,
    font_family: Option<&str>,
    font_size: Option<u8>,
) -> Result<(), Box<dyn Error>> {
    let mapping = mapping.unwrap_or(&DEFAULT_MAPPING);
    let font_family = font_family.unwrap_or("monospace");
    let font_size = font_size.unwrap_or(14);

    let captures = parser::collect_captures(&source, language, &query)?;
    let background = palette.colour(0);
    let tokens = render::make_tokens(source, &captures, palette, mapping);

    let output_path = Path::new(&output);

    render::render(
        source,
        &tokens,
        &output_path,
        background,
        font_family,
        font_size,
    )
}
