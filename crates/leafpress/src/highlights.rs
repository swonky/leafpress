#[derive(Clone, Copy, Debug, Default)]
pub struct Highlight {
    pub base: usize,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub undercurl: bool,
}

#[derive(Clone, Copy, Debug)]
struct Mapping {
    name: &'static str,
    highlight: Highlight,
}

pub struct HighlightMap {
    pub default: Highlight,
    mappings: &'static [Mapping],
}

impl Highlight {
    pub const fn new(base: usize) -> Self {
        Self {
            base,
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
            undercurl: false,
        }
    }

    pub const fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub const fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub const fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub const fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    pub const fn undercurl(mut self) -> Self {
        self.undercurl = true;
        self
    }
}

impl HighlightMap {
    pub fn get(&self, name: &str) -> Highlight {
        self.mappings
            .binary_search_by_key(&name, |mapping| mapping.name)
            .map(|i| self.mappings[i].highlight)
            .unwrap_or(self.default)
    }
}

macro_rules! mappings {
    ($($name:literal => $highlight:expr),* $(,)?) => {
        &[
            $(Mapping {
                name: $name,
                highlight: $highlight,
            }),*
        ]
    };
}

macro_rules! h {
    ($colour:expr) => {
        Highlight::new($colour)
    };
}

pub const DEFAULT_MAPPING: HighlightMap = HighlightMap {
    default: h!(0x05),

    mappings: mappings![
        "attribute" => h!(0x0f),
        "attribute.builtin" => h!(0x0f),
        "boolean" => h!(0x09),
        "character" => h!(0x0b),
        "character.special" => h!(0x0c),
        "comment" => h!(0x03),
        "comment.documentation" => h!(0x03).italic(),
        "comment.error" => h!(0x08),
        "comment.note" => h!(0x0c),
        "comment.todo" => h!(0x0f),
        "comment.warning" => h!(0x09),
        "constant" => h!(0x09),
        "constant.builtin" => h!(0x09).bold(),
        "constant.macro" => h!(0x09).bold(),
        "constructor" => h!(0x0a),
        "diff.delta" => h!(0x0a),
        "diff.minus" => h!(0x08),
        "diff.plus" => h!(0x0b),
        "function" => h!(0x0d),
        "function.builtin" => h!(0x0d),
        "function.call" => h!(0x0d),
        "function.macro" => h!(0x0d),
        "function.method" => h!(0x0d),
        "function.method.call" => h!(0x0d),
        "keyword" => h!(0x0e),
        "keyword.conditional" => h!(0x0e),
        "keyword.conditional.ternary" => h!(0x0e),
        "keyword.coroutine" => h!(0x0e),
        "keyword.debug" => h!(0x0e),
        "keyword.directive" => h!(0x0e),
        "keyword.directive.define" => h!(0x0e),
        "keyword.exception" => h!(0x08),
        "keyword.function" => h!(0x0e),
        "keyword.import" => h!(0x0e),
        "keyword.modifier" => h!(0x0e),
        "keyword.operator" => h!(0x0e),
        "keyword.repeat" => h!(0x0e),
        "keyword.return" => h!(0x0e),
        "keyword.type" => h!(0x0e),
        "label" => h!(0x0e),
        "markup" => h!(0x0f),
        "markup.heading" => h!(0x0f),
        "markup.heading.1" => h!(0x0f),
        "markup.heading.2" => h!(0x0f),
        "markup.heading.3" => h!(0x0f),
        "markup.heading.4" => h!(0x0f),
        "markup.heading.5" => h!(0x0f),
        "markup.heading.6" => h!(0x0f),
        "markup.italic" => h!(0x0f).italic(),
        "markup.link" => h!(0x0f).underline(),
        "markup.link.label" => h!(0x0f).underline(),
        "markup.link.url" => h!(0x0d).underline(),
        "markup.list" => h!(0x0f),
        "markup.list.checked" => h!(0x0b),
        "markup.list.unchecked" => h!(0x0b),
        "markup.math" => h!(0x0c),
        "markup.quote" => h!(0x0f),
        "markup.raw" => h!(0x0f),
        "markup.raw.block" => h!(0x0f),
        "markup.strikethrough" => h!(0x0f).strikethrough(),
        "markup.strong" => h!(0x0f).bold(),
        "markup.underline" => h!(0x0f).underline(),
        "module" => h!(0x0a),
        "module.builtin" => h!(0x0a),
        "number" => h!(0x09),
        "number.float" => h!(0x09),
        "operator" => h!(0x05),
        "property" => h!(0x05),
        "punctuation.bracket" => h!(0x05),
        "punctuation.delimiter" => h!(0x05),
        "punctuation.special" => h!(0x0c),
        "string" => h!(0x0b),
        "string.documentation" => h!(0x0b),
        "string.escape" => h!(0x0c),
        "string.regexp" => h!(0x0c),
        "string.special" => h!(0x0c),
        "string.special.path" => h!(0x0c),
        "string.special.symbol" => h!(0x0c),
        "string.special.url" => h!(0x0d).underline(),
        "tag" => h!(0x0a),
        "tag.attribute" => h!(0x05),
        "tag.builtin" => h!(0x0c),
        "tag.delimiter" => h!(0x05),
        "type" => h!(0x0a),
        "type.builtin" => h!(0x0a),
        "type.definition" => h!(0x0a),
        "variable" => h!(0x05),
        "variable.builtin" => h!(0x05),
        "variable.member" => h!(0x05),
        "variable.parameter" => h!(0x05),
        "variable.parameter.builtin" => h!(0x05),
    ],
};
