///! Leafpress
///!
///! Generate portable SVG images with Tree-sitter syntax highlighting.

/// Syntax highlighting definitions and utilities.
pub mod highlights;

/// Parsing source code using Tree-sitter grammars.
pub mod parser;

/// Rendering highlighted source code as SVG.
pub mod render;

/// Colour themes and theme management.
pub mod theme;

/// Parsing and handling of Tree-sitter configuration files.
pub mod tree_sitter_config;

use std::{error::Error, path::Path};

/// Generates an SVG image from source code.
///
/// The source is parsed using the provided Tree-sitter language and query,
/// then rendered using the supplied colour palette and highlight mapping.
///
/// The SVG image is written to `output`.
///
/// # Default values
///
/// If `mapping`, `font_family`, or `font_size` is `None`, the default highlight
/// mapping, `monospace` font family, or 14px font size is used, respectively.
///
/// # Errors
///
/// Returns an error if the source cannot be parsed or the SVG cannot be
/// rendered or written to `output`.
pub fn generate_svg<P: theme::Palette>(
    output: &Path,
    source: &[u8],
    language: &tree_sitter::Language,
    query: &tree_sitter::Query,
    palette: &P,
    format: &render::Format,
    mapping: Option<&highlights::HighlightMap>,
) -> Result<(), Box<dyn Error>> {
    let mapping = mapping.unwrap_or(&highlights::DEFAULT_MAPPING);

    let captures = parser::collect_captures(&source, language, &query)?;
    let tokens = render::make_tokens(source, &captures, palette, mapping);

    let output_path = Path::new(&output);

    render::render_to_file(source, &tokens, &format, palette.background(), &output_path)
}
