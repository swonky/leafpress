mod highlights;

use std::{
    env,
    error::Error,
    fs,
    mem::ManuallyDrop,
    path::{Path, PathBuf},
};

use cairo::{Context as CairoContext, SvgSurface};
use clap::{Parser as ClapParser, Subcommand};
use libloading::{Library, Symbol};
use pango::prelude::FontMapExt;
use pango::{AttrColor, AttrList, FontDescription, Style, Underline, Weight};
use serde_json::Value;
use tree_sitter::{Language, Parser as TSParser, Query, QueryCursor, StreamingIterator};

use highlights::{Highlight, get_colour};

#[derive(Clone, Copy, Debug)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug)]
struct Token {
    start: usize,
    end: usize,
    hl: Highlight,
    colour: Rgb,
}

#[derive(ClapParser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Render {
        /// Input file
        input: String,

        /// Language name
        #[arg(short, long)]
        lang: String,

        /// Theme name
        #[arg(short, long, default_value = "onedark/onedark")]
        theme: String,

        // Font family
        #[arg(short, long, default_value = "monospace")]
        font: String,

        // Font family
        #[arg(short, long, default_value_t = 14)]
        size: u8,

        /// Output file
        #[arg(short, long, default_value = "./output.svg")]
        output: String,

        /// Parser directory
        #[arg(long, env = "TSRENDER_CONFIG_PATH")]
        config_path: Option<PathBuf>,

        /// Parser directory
        #[arg(short = 'p', long, env = "TSRENDER_GRAMMAR_DIR")]
        grammar_dir: Option<PathBuf>,

        /// Parser directory
        #[arg(long, env = "TSRENDER_THEME_DIR", default_value = "./themes")]
        theme_dir: PathBuf,
    },
}

use serde::Deserialize;

#[derive(Deserialize)]
struct Base16Theme {
    base00: String,
    base01: String,
    base02: String,
    base03: String,
    base04: String,
    base05: String,
    base06: String,
    base07: String,
    base08: String,
    base09: String,
    #[serde(rename = "base0A")]
    base0a: String,
    #[serde(rename = "base0B")]
    base0b: String,
    #[serde(rename = "base0C")]
    base0c: String,
    #[serde(rename = "base0D")]
    base0d: String,
    #[serde(rename = "base0E")]
    base0e: String,
    #[serde(rename = "base0F")]
    base0f: String,
}

fn load_base16(path: PathBuf) -> Result<[Rgb; 16], Box<dyn Error>> {
    let file = fs::File::open(path)?;
    let theme: Base16Theme = serde_yaml::from_reader(file)?;

    Ok([
        parse_hex(&theme.base00)?,
        parse_hex(&theme.base01)?,
        parse_hex(&theme.base02)?,
        parse_hex(&theme.base03)?,
        parse_hex(&theme.base04)?,
        parse_hex(&theme.base05)?,
        parse_hex(&theme.base06)?,
        parse_hex(&theme.base07)?,
        parse_hex(&theme.base08)?,
        parse_hex(&theme.base09)?,
        parse_hex(&theme.base0a)?,
        parse_hex(&theme.base0b)?,
        parse_hex(&theme.base0c)?,
        parse_hex(&theme.base0d)?,
        parse_hex(&theme.base0e)?,
        parse_hex(&theme.base0f)?,
    ])
}

fn config_path(path: Option<PathBuf>) -> Result<PathBuf, Box<dyn Error>> {
    path.or_else(|| {
        env::var_os("XDG_CONFIG_HOME").map(|p| PathBuf::from(p).join("tree-sitter/config.json"))
    })
    .or_else(|| {
        env::var_os("HOME").map(|p| PathBuf::from(p).join(".config/tree-sitter/config.json"))
    })
    .ok_or_else(|| "failed to infer config file location.".into())
}

fn load_config(path: Option<PathBuf>) -> Result<Value, Box<dyn Error>> {
    let path = config_path(path)?;
    let contents = fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&contents)?)
}

fn parse_hex(s: &str) -> Result<Rgb, Box<dyn Error>> {
    let s = s.trim().trim_start_matches('#');
    if s.len() != 6 {
        return Err(format!("invalid colour: {s}").into());
    }
    Ok(Rgb {
        r: u8::from_str_radix(&s[0..2], 16)?,
        g: u8::from_str_radix(&s[2..4], 16)?,
        b: u8::from_str_radix(&s[4..6], 16)?,
    })
}

struct LoadedLanguage {
    _library: Library,
    language: ManuallyDrop<Language>,
}

fn load_parser_library(path: &Path) -> Result<LoadedLanguage, Box<dyn Error>> {
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
    let symbol = format!("tree_sitter_{stem}");

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

fn collect_captures(
    source: &[u8],
    language: &Language,
    query_source: &str,
) -> Result<Vec<(usize, usize, u32)>, Box<dyn Error>> {
    let mut parser = TSParser::new();
    parser.set_language(language)?;
    let tree = parser.parse(source, None).ok_or("failed to parse source")?;
    let query = Query::new(language, query_source)?;
    let mut cursor = QueryCursor::new();
    let mut captures = Vec::new();
    let mut iter = cursor.captures(&query, tree.root_node(), source);

    while let Some((m, index)) = iter.next() {
        let capture = m.captures[*index];
        let start = capture.node.start_byte();
        let end = capture.node.end_byte();
        if start != end {
            captures.push((start, end, capture.index));
        }
    }
    Ok(captures)
}

fn make_tokens(
    source: &[u8],
    captures: &[(usize, usize, u32)],
    query: &Query,
    colours: &[Rgb; 16],
) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut position = 0usize;

    while position < source.len() {
        let active = captures
            .iter()
            .rev()
            .find(|(start, end, _)| *start <= position && position < *end);
        let mut end = position + 1;
        while end < source.len() {
            let next = captures
                .iter()
                .rev()
                .find(|(start, finish, _)| *start <= end && end < *finish);
            if next.map(|x| x.2) != active.map(|x| x.2) {
                break;
            }
            end += 1;
        }

        let (hl, colour) = match active {
            Some((_, _, capture_index)) => {
                let name = query.capture_names()[*capture_index as usize];
                let hl = get_colour(name);
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

fn render(
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

fn parser_directories(config: &Value) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let Some(value) = config.get("parser-directories") else {
        return Err("failed to retrieve 'parser-directories' value from config".into());
    };

    let directories = value
        .as_array()
        .ok_or("\"parser-directories\" must be an array")?;

    if directories.is_empty() {
        return Err("\"parser-directories\" contains no directories".into());
    }

    directories
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(PathBuf::from)
                .ok_or("\"parser-directories\" must contain only strings".into())
        })
        .collect()
}

fn find_grammar(
    grammar_path: Option<PathBuf>,
    parser_directories: &[PathBuf],
    lang: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    if let Some(path) = grammar_path {
        if !path.is_dir() {
            return Err(format!("grammar directory does not exist: {path:?}").into());
        }

        return Ok(path);
    }

    let name = format!("tree-sitter-{lang}");

    parser_directories
        .iter()
        .map(|directory| directory.join(&name))
        .find(|path| path.is_dir())
        .ok_or_else(|| {
            format!("could not find grammar directory '{name}' in any configured parser directory")
                .into()
        })
}

fn run(
    input: &str,
    output: &str,
    lang: &str,
    theme: &str,
    font_family: &str,
    font_size: u8,
    config_path: Option<PathBuf>,
    grammar_dir: Option<PathBuf>,
    theme_dir: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let config = load_config(config_path)?;
    let directories = parser_directories(&config)?;
    let grammar = find_grammar(grammar_dir, &directories, lang)?;

    let scheme_path = grammar.join("queries").join(format!("highlights.scm"));

    if !scheme_path.is_file() {
        return Err(std::io::Error::other(format!(
            "tree-sitter highlights scheme does not exist: {}",
            scheme_path.display()
        ))
        .into());
    }

    // cache directory
    let cache_dir = match env::var_os("XDG_CACHE_HOME") {
        Some(xdg) => PathBuf::from(xdg),
        None => match env::var_os("HOME") {
            Some(home) => PathBuf::from(home).join(".cache"),
            None => {
                return Err(
                    std::io::Error::other("failed to find tree-sitter cache directory").into(),
                );
            }
        },
    };

    let cache_dir = cache_dir.join("tree-sitter").join("lib");

    if !cache_dir.is_dir() {
        return Err(std::io::Error::other(format!(
            "tree-sitter cache directory does not exist: {}",
            cache_dir.display()
        ))
        .into());
    }

    let lang_path = cache_dir.join(format!("{lang}.so"));

    if !lang_path.is_file() {
        return Err(std::io::Error::other(format!(
            "tree-sitter parser does not exist: {}",
            lang_path.display()
        ))
        .into());
    }

    if !theme_dir.is_dir() {
        return Err(format!(
            "Failed to locate theme directory: {}",
            theme_dir.to_str().unwrap()
        )
        .into());
    }

    let theme_path = theme_dir.join(format!("{theme}.yaml"));

    let colours = load_base16(theme_path)?;
    let source = fs::read(input)?;
    let query_source = String::from_utf8(fs::read(&scheme_path)?)?;
    let loaded = load_parser_library(&lang_path)?;
    let captures = collect_captures(&source, &loaded.language, &query_source)?;
    let query = Query::new(&loaded.language, &query_source)?;
    let tokens = make_tokens(&source, &captures, &query, &colours);

    let output_path = Path::new(&output);

    render(
        &source,
        &tokens,
        &output_path,
        &colours,
        &font_family,
        font_size,
    )
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Render {
            input,
            output,
            theme,
            font,
            size,
            lang,
            config_path,
            grammar_dir,
            theme_dir,
        } => match run(
            &input,
            &output,
            &lang,
            &theme,
            &font,
            size,
            config_path,
            grammar_dir,
            theme_dir,
        ) {
            Ok(()) => std::process::exit(0),
            Err(err) => {
                eprintln!("ERROR: {err}");
                std::process::exit(1);
            }
        },
    }
}
