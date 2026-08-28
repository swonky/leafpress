// main.rs

use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use clap::{Parser as ClapParser, Subcommand};
use serde_json::Value;

use leafpress::{generate_svg, highlights, parser, theme};

#[derive(ClapParser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    ListThemes {
        #[arg(long, default_value_t = false)]
        json: bool,
    },
    ListGrammars {
        #[arg(long, default_value_t = false)]
        json: bool,
        /// Parser directory
        #[arg(long, env = "TSRENDER_CONFIG_PATH")]
        config_path: Option<PathBuf>,

        /// Parser directory
        #[arg(short = 'p', long, env = "TSRENDER_GRAMMAR_DIR")]
        grammar_dir: Option<PathBuf>,
    },
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
    },
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

// fn find_grammar(
//     grammar_path: Option<PathBuf>,
//     parser_directories: &[PathBuf],
//     lang: &str,
// ) -> Result<PathBuf, Box<dyn Error>> {
//     if let Some(path) = grammar_path {
//         if !path.is_dir() {
//             return Err(format!("grammar directory does not exist: {path:?}").into());
//         }
//
//         return Ok(path);
//     }
//
//     let name = format!("tree-sitter-{lang}");
//
//     parser_directories
//         .iter()
//         .map(|directory| directory.join(&name))
//         .find(|path| path.is_dir())
//         .ok_or_else(|| {
//             format!("could not find grammar directory '{name}' in any configured parser directory")
//                 .into()
//         })
// }

fn run(
    input: &str,
    output: &str,
    lang: &str,
    theme: &str,
    font_family: &str,
    font_size: u8,
    config_path: Option<PathBuf>,
    grammar_dir: Option<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    let grammar_path = match grammar_dir {
        Some(path) => parser::find_in_dir(&path, lang)?,
        None => {
            let config = load_config(config_path)?;
            let directories = parser_directories(&config)?;
            parser::find_in_dirs(directories, lang)?
        }
    };

    let grammar = grammar_path.ok_or_else(|| format!("grammar '{lang}' not found"))?;

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

    let palette = match theme::get_theme(&theme) {
        Some(v) => v,
        None => {
            return Err("bad theme".into());
        }
    };
    let source = fs::read(input)?;
    let loaded = parser::load_dynamic(&lang_path)?;
    let query = parser::load_query(&scheme_path, &loaded.language)?;
    let output_path = Path::new(&output);

    generate_svg(
        output_path,
        &source,
        &loaded.language,
        &query,
        palette,
        Some(&highlights::DEFAULT_MAPPING),
        Some(font_family),
        Some(font_size),
    )
}

fn list_grammars(
    json: bool,
    config_path: Option<PathBuf>,
    grammar_dir: Option<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    match grammar_dir {
        Some(path) => {
            for result in parser::iter_grammars(&path)? {
                let (_grammar_path, grammar_config) = result?;

                for grammar in grammar_config.grammars {
                    if json {
                        println!("{}", serde_json::to_string_pretty(&grammar)?);
                    } else {
                        println!("{}", grammar.name);
                    }
                }
            }
        }
        None => {
            let config = load_config(config_path)?;
            let directories = parser_directories(&config)?;

            for directory in directories {
                for result in parser::iter_grammars(&directory)? {
                    let (_grammar_path, grammar_config) = result?;

                    for grammar in grammar_config.grammars {
                        if json {
                            println!("{}", serde_json::to_string_pretty(&grammar)?);
                        } else {
                            println!("{}", grammar.name);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::ListGrammars {
            json,
            config_path,
            grammar_dir,
        } => match list_grammars(json, config_path, grammar_dir) {
            Ok(()) => std::process::exit(0),
            Err(err) => {
                eprintln!("ERROR: {err}");
                std::process::exit(1);
            }
        },
        Command::ListThemes { json } => {
            let themes = theme::list_themes();
            if json {
                let values: Vec<_> = themes.collect();
                let json_string = match serde_json::to_string(&values) {
                    Ok(v) => v,
                    Err(err) => {
                        eprintln!("ERROR: {err}");
                        std::process::exit(1);
                    }
                };
                println!("{}", json_string);
            } else {
                for name in themes {
                    println!("{name}");
                }
            }
        }
        Command::Render {
            input,
            output,
            theme,
            font,
            size,
            lang,
            config_path,
            grammar_dir,
        } => match run(
            &input,
            &output,
            &lang,
            &theme,
            &font,
            size,
            config_path,
            grammar_dir,
        ) {
            Ok(()) => std::process::exit(0),
            Err(err) => {
                eprintln!("ERROR: {err}");
                std::process::exit(1);
            }
        },
    }
}
