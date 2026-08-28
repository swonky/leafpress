use std::{error::Error, path::Path};

use leafpress::{generate_svg, render::Format, theme};

#[test]
fn render_tree_sitter_go() -> Result<(), Box<dyn Error>> {
    let source = br#"package main

import (
    "fmt"
    "strings"
)

type User struct {
    Given   []string    `json:"name"`
    Family  int         `json:"age"`
}

func greet(u User) string {
    n := strings.join(u.Given, " ")
    return fmt.Sprintf("Hello, %s!!", n)
}
"#;

    let language = tree_sitter_go::LANGUAGE.into();
    let query = tree_sitter::Query::new(&language, tree_sitter_go::HIGHLIGHTS_QUERY)?;
    let palette = theme::TOKYO_NIGHT_DARK;
    let format = Format::default().font_family("Ioskeley Mono Term");

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

#[test]
fn render_tree_sitter_rust() -> Result<(), Box<dyn Error>> {
    let source = br#"use std::collections::HashMap;

#[derive(Debug)]
struct User {
    given: Vec<String>,
    family: String,
}

/// Greets the user
fn greet(user: &User) -> String {
    if !user.given.is_empty() {
        format!("Hello {}!", user.given)
    } else {
        "Hey! ...you?".into()
    }
}
"#;

    let language = tree_sitter_rust::LANGUAGE.into();
    let query = tree_sitter::Query::new(&language, tree_sitter_rust::HIGHLIGHTS_QUERY)?;
    let palette = theme::OXOCARBON_DARK;
    let format = Format::default().font_family("Ioskeley Mono Term");

    generate_svg(
        Path::new("../../target/tree_sitter_rust.svg"),
        source,
        &language,
        &query,
        &palette,
        &format,
        None,
    )?;

    Ok(())
}

#[test]
fn render_tree_sitter_python() -> Result<(), Box<dyn Error>> {
    let source = br#"from dataclasses import dataclass

@dataclass
class User:
    given: list[str]
    family: str

def greet(user: User) -> str:
    """Greets the user"""

    if len(user.given) != 0:
        end = " ".join(user.given) + "!"
    else:
        end = "...you?"

    return f"Hello {end}"
"#;

    let language = tree_sitter_python::LANGUAGE.into();
    let query = tree_sitter::Query::new(&language, tree_sitter_python::HIGHLIGHTS_QUERY)?;
    let palette = theme::DRACULA;
    let format = Format::default().font_family("Ioskeley Mono Term");

    generate_svg(
        Path::new("../../target/tree_sitter_python.svg"),
        source,
        &language,
        &query,
        &palette,
        &format,
        None,
    )?;

    Ok(())
}
