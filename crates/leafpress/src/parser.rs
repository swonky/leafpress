// parser.rs
use crate::fops;
use crate::tree_sitter_config::TreeSitterConfig;
use libloading::{Library, Symbol};
use object::{Object, ObjectSymbol};
use std::{
    error::Error,
    fs, io,
    mem::ManuallyDrop,
    path::{Path, PathBuf},
};
use tree_sitter::{Language, Parser, Query, QueryCursor, StreamingIterator};

/// A Tree-sitter language loaded from a dynamic library.
pub struct DynamicLanguage {
    _library: Library,
    language: ManuallyDrop<Language>,
}

impl DynamicLanguage {
    /// Returns the underlying Tree-sitter language.
    pub fn language(&self) -> &Language {
        &self.language
    }
}

impl std::ops::Deref for DynamicLanguage {
    type Target = Language;

    /// Returns the underlying Tree-sitter language.
    fn deref(&self) -> &Self::Target {
        &self.language
    }
}

/// A captured range from a Tree-sitter query.
pub struct Capture<'a> {
    /// Start byte offset of the captured range.
    pub start: usize,

    /// End byte offset of the captured range.
    pub end: usize,

    /// Name of the capture group.
    pub group: &'a str,
}

/// An error encountered while loading a resource.
#[derive(Debug)]
pub enum LoadError {
    /// An I/O error occurred while accessing the resource.
    Io(io::Error),

    /// The resource was accessible but contained invalid input.
    InvalidInput(Box<dyn std::error::Error + Send + Sync>),
}

/// Infers the Tree-sitter language symbol exported by a dynamic library.
fn get_symbol(path: &Path) -> Result<Option<String>, LoadError> {
    let data = fs::read(path).map_err(LoadError::Io)?;

    let file = object::File::parse(&*data).map_err(|err| LoadError::InvalidInput(Box::new(err)))?;

    let symbol = file
        .symbols()
        .filter_map(|symbol| {
            let name = symbol.name().ok()?;
            name.strip_prefix("tree_sitter_")
                .map(|suffix| (name, suffix.len()))
        })
        .min_by_key(|(_, len)| *len)
        .map(|(name, _)| name.to_owned());

    Ok(symbol)
}

/// Loads a Tree-sitter query from a scheme file.
pub fn load_query(path: &Path, language: &Language) -> Result<Query, LoadError> {
    fops::validate_file(path).map_err(LoadError::Io)?;

    let source = fs::read_to_string(path).map_err(LoadError::Io)?;

    Query::new(language, &source).map_err(|err| LoadError::InvalidInput(Box::new(err)))
}

/// Loads a Tree-sitter language from a dynamic library.
pub fn load_dynamic(path: &Path) -> Result<DynamicLanguage, LoadError> {
    fops::validate_file(path).map_err(LoadError::Io)?;

    let symbol = get_symbol(path)?.ok_or_else(|| {
        LoadError::InvalidInput(
            format!("failed to find language symbol in {}", path.display()).into(),
        )
    })?;

    let library =
        unsafe { Library::new(path) }.map_err(|err| LoadError::InvalidInput(Box::new(err)))?;

    let language_fn: Symbol<unsafe extern "C" fn() -> *const tree_sitter::ffi::TSLanguage> =
        unsafe { library.get(symbol.as_bytes()) }
            .map_err(|err| LoadError::InvalidInput(Box::new(err)))?;

    let raw = unsafe { language_fn() };

    if raw.is_null() {
        return Err(LoadError::InvalidInput(
            format!("{symbol} returned a null language").into(),
        ));
    }

    let language = unsafe { ManuallyDrop::new(Language::from_raw(raw)) };

    Ok(DynamicLanguage {
        _library: library,
        language,
    })
}

// Searches for all grammar repositories within `dirpath`.
pub fn iter_grammars(
    dirpath: &Path,
) -> Result<impl Iterator<Item = Result<(PathBuf, TreeSitterConfig), LoadError>>, LoadError> {
    fops::validate_directory(dirpath).map_err(LoadError::Io)?;

    let subdirs = fs::read_dir(dirpath).map_err(LoadError::Io)?;

    Ok(subdirs.filter_map(|entry| {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => return Some(Err(LoadError::Io(err))),
        };

        if !entry
            .file_name()
            .to_string_lossy()
            .starts_with("tree-sitter-")
        {
            return None;
        }

        let grammar_path = entry.path();

        if let Err(err) = fops::validate_directory(&grammar_path) {
            return Some(Err(LoadError::Io(err)));
        }

        let config_path = grammar_path.join("tree-sitter.json");

        Some(load_grammar_config(&config_path).map(|config| (grammar_path, config)))
    }))
}

// Deserialises a tree-sitter.json config file.
pub fn load_grammar_config(path: &Path) -> Result<TreeSitterConfig, LoadError> {
    fops::validate_file(path).map_err(LoadError::Io)?;

    let file = fs::File::open(path).map_err(LoadError::Io)?;

    serde_json::from_reader(file).map_err(|err| LoadError::InvalidInput(Box::new(err)))
}

// Searches for a grammar within `dirpath` that matches `name`.
pub fn find_in_dir(dirpath: &Path, name: &str) -> Result<Option<PathBuf>, LoadError> {
    fops::validate_directory(dirpath).map_err(LoadError::Io)?;

    for result in iter_grammars(dirpath)? {
        let (grammar_path, config) = result?;

        for grammar in config.grammars {
            if grammar.name == name {
                return Ok(match grammar.path {
                    Some(path) => Some(grammar_path.join(path)),
                    None => Some(grammar_path),
                });
            }
        }
    }

    Ok(None)
}

// Searches for a grammar within a sequence of paths within `dirpath` that matches `name`.
pub fn find_in_dirs(dirpaths: Vec<PathBuf>, name: &str) -> Result<Option<PathBuf>, LoadError> {
    for path in dirpaths {
        if let Some(grammar_path) = find_in_dir(&path, name)? {
            return Ok(Some(grammar_path));
        }
    }

    Err(LoadError::InvalidInput(Box::new(io::Error::new(
        io::ErrorKind::NotFound,
        format!("grammar '{name}' not found"),
    ))))
}

// Executes a highlighting query on the source text and returns all capture groups.
pub fn collect_captures<'a>(
    source: &[u8],
    language: &Language,
    query: &'a Query,
) -> Result<Vec<Capture<'a>>, Box<dyn Error>> {
    let mut parser = Parser::new();
    parser.set_language(language)?;
    let tree = parser.parse(source, None).ok_or("failed to parse source")?;
    let mut cursor = QueryCursor::new();
    let mut captures = Vec::new();
    let mut iter = cursor.captures(&query, tree.root_node(), source);
    let all_groups = query.capture_names();

    while let Some((m, index)) = iter.next() {
        let capture = m.captures[*index];
        let start = capture.node.start_byte();
        let end = capture.node.end_byte();
        if start != end {
            captures.push(Capture {
                start: start,
                end: end,
                group: all_groups[capture.index as usize],
            });
        }
    }
    Ok(captures)
}
