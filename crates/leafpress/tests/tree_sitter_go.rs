use std::{error::Error, path::Path};

use leafpress::{generate_svg, render::Format, theme};

#[test]
fn render_tree_sitter_go() -> Result<(), Box<dyn Error>> {
    let source = br#"package main

import "fmt"

func main() {
    fmt.Println("Hello, world!")
}
"#;

    let language = tree_sitter_go::LANGUAGE.into();

    let query = tree_sitter::Query::new(&language, tree_sitter_go::HIGHLIGHTS_QUERY)?;

    let palette = theme::GITHUB_DARK;
    let format = Format::default();

    generate_svg(
        Path::new("../../target/tree_sitter_go.svg"),
        source,
        &language,
        &query,
        &palette,
        &format,
        None,
    )?;

    Ok(())
}
