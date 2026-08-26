// parser.rs
use libloading::{Library, Symbol};
use object::{Object, ObjectSymbol};
use std::{error::Error, fs, mem::ManuallyDrop, path::Path};
use tree_sitter::{Language, Parser, Query, QueryCursor, StreamingIterator};

pub struct LoadedLanguage {
    _library: Library,
    pub language: ManuallyDrop<Language>,
}

pub struct Capture<'a> {
    pub start: usize,
    pub end: usize,
    pub group: &'a str,
}

pub fn load_query(path: &Path, language: &Language) -> Result<Query, Box<dyn Error>> {
    let source = String::from_utf8(fs::read(path)?)?;
    let query = Query::new(language, &source)?;
    Ok(query)
}

pub fn load_dynamic(path: &Path) -> Result<LoadedLanguage, Box<dyn Error>> {
    let filename = path
        .file_name()
        .and_then(|x| x.to_str())
        .ok_or("invalid language library path")?;
    let stem = filename
        .strip_suffix(".so")
        .ok_or_else(|| format!("invalid language library: {filename}"))?;
    if stem.is_empty() {
        return Err("invalid language library name".into());
    }
    let symbol = match get_symbol(path)? {
        Some(v) => v,
        None => {
            return Err(format!("failed to read symbol from {path:?}").into());
        }
    };
    let library = unsafe { Library::new(path) }?;
    let language_fn: Symbol<unsafe extern "C" fn() -> *const tree_sitter::ffi::TSLanguage> =
        unsafe { library.get(symbol.as_bytes())? };
    let raw = unsafe { language_fn() };
    if raw.is_null() {
        return Err(format!("{symbol} returned a null language").into());
    }
    let language = unsafe { ManuallyDrop::new(Language::from_raw(raw)) };
    Ok(LoadedLanguage {
        _library: library,
        language,
    })
}

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

fn get_symbol(path: &Path) -> Result<Option<String>, Box<dyn Error>> {
    let data = fs::read(path)?;
    let file = object::File::parse(&*data)?;

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
