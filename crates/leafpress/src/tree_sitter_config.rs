use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeSitterConfig {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,

    /// The language bindings that will be generated.
    pub bindings: Option<Bindings>,

    pub grammars: Vec<Grammar>,

    pub metadata: Metadata,
}

/// The language bindings that will be generated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bindings {
    pub c: Option<bool>,

    pub go: Option<bool>,

    pub java: Option<bool>,

    pub kotlin: Option<bool>,

    pub node: Option<bool>,

    pub python: Option<bool>,

    pub rust: Option<bool>,

    pub swift: Option<bool>,

    pub zig: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Grammar {
    /// The name converted to CamelCase.
    pub camelcase: Option<String>,

    /// The class name for the Swift, Java & Kotlin bindings
    pub class_name: Option<String>,

    /// A regex pattern that will be tested against the contents of the file in order to break
    /// ties in cases where multiple grammars matched the file.
    pub content_regex: Option<String>,

    /// The relative paths to files that should be checked for modifications during recompilation.
    pub external_files: Option<Vec<String>>,

    /// An array of filename suffix strings.
    pub file_types: Option<Vec<String>>,

    /// A regex pattern that will be tested against the first line of a file in order to
    /// determine whether this language applies to the file.
    pub first_line_regex: Option<String>,

    /// The path(s) to the grammar's highlight queries.
    pub highlights: Option<Highlights>,

    /// A regex pattern that will be tested against a language name in order to determine whether
    /// this language should be used for a potential language injection site.
    pub injection_regex: Option<String>,

    /// The path(s) to the grammar's injection queries.
    pub injections: Option<Highlights>,

    /// The path(s) to the grammar's local variable queries.
    pub locals: Option<Highlights>,

    /// The name of the grammar.
    pub name: String,

    /// The relative path to the directory containing the grammar.
    pub path: Option<String>,

    /// The TextMate scope that represents this language.
    pub scope: String,

    /// The path(s) to the grammar's code navigation queries.
    pub tags: Option<Highlights>,

    /// The title of the language.
    pub title: Option<String>,
}

/// The path(s) to the grammar's highlight queries.
///
/// The path(s) to the grammar's injection queries.
///
/// The path(s) to the grammar's local variable queries.
///
/// The path(s) to the grammar's code navigation queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Highlights {
    PurpleString(String),

    StringArray(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub authors: Option<Vec<Author>>,

    /// The project's description.
    pub description: Option<String>,

    /// The project's license.
    pub license: Option<String>,

    pub links: Links,

    /// The namespace for the Java & Kotlin packages.
    pub namespace: Option<String>,

    /// The current version of the project.
    pub version: String,
}

/// The project's author(s).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub email: Option<String>,

    pub name: String,

    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Links {
    /// The project's funding link.
    pub funding: Option<String>,

    /// The project's repository.
    pub repository: String,
}
