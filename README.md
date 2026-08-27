# leafpress

A small library and command-line utility for rendering source code as SVG images using Tree-sitter grammars. 

## Features
* Portable SVG output (text rendered as paths).
* Compile-time parser integration.
* Dynamic parser loading at runtime.
* Large collection of built-in colour schemes.
* Supports custom colour schemes.

## Installation
### From crates
TBD

### From source
```sh
cargo build --release leafpress-cli
cargo install --path crate/leafpress-cli
```

## Usage

### Command-line utility

```sh
leafpress render -l rust main.rs
```

```
leafpress render --help
Usage: leafpress render [OPTIONS] --lang <LANG> <INPUT>

Arguments:
  <INPUT>  Input file

Options:
  -l, --lang <LANG>                Language name
  -t, --theme <THEME>              Theme name [default: onedark/onedark]
  -f, --font <FONT>                [default: monospace]
  -s, --size <SIZE>                [default: 14]
  -o, --output <OUTPUT>            Output file [default: ./output.svg]
      --config-path <CONFIG_PATH>  Parser directory [env: TSRENDER_CONFIG_PATH=]
  -p, --grammar-dir <GRAMMAR_DIR>  Parser directory [env: TSRENDER_GRAMMAR_DIR=]
      --theme-dir <THEME_DIR>      Parser directory [env: TSRENDER_THEME_DIR=/home/tcs/src/tree-sitter/leafpress/themes] [default: ./themes]
  -h, --help                       Print help
```

### Rust library

#### Dynamically loading a compiled parser
```rust
let lang_path = "./javascript.so";
let query_path = "./highlights.scm";
let theme_path = "./theme.yaml";

let input_path = Path::new("./main.js"),
let output_path = Path::new("./output.svg"),

let source = fs::read(&input_path),
let loaded = parser::load_dynamic(&lang_path)?;
let query = parser::load_query(&scheme_path, &loaded.language)?;
let theme = theme::load_base16(&theme_path);

leafpress::generate_svg(
    &output_path,
    &source,
    &loaded.language,
    &query,
    &palette,
    Some(highlights::DEFAULT_MAPPINGS),
    Some(font_family),
    Some(font_size),
)
```
