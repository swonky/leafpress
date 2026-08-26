#[derive(Clone, Copy, Debug, Default)]
pub struct Highlight {
    pub colour: usize,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub undercurl: bool,
}

struct Mapping {
    name: &'static str,
    hl: Highlight,
}

const fn hl(colour: usize) -> Highlight {
    Highlight {
        colour,
        bold: false,
        italic: false,
        underline: false,
        strikethrough: false,
        undercurl: false,
    }
}

const MAPPINGS: &[Mapping] = &[
    Mapping {
        name: "attribute",
        hl: hl(0x0f),
    },
    Mapping {
        name: "attribute.builtin",
        hl: hl(0x0f),
    },
    Mapping {
        name: "boolean",
        hl: hl(0x09),
    },
    Mapping {
        name: "character",
        hl: hl(0x0b),
    },
    Mapping {
        name: "character.special",
        hl: hl(0x0c),
    },
    Mapping {
        name: "comment",
        hl: hl(0x03),
    },
    Mapping {
        name: "comment.documentation",
        hl: hl(0x03),
    },
    Mapping {
        name: "comment.error",
        hl: hl(0x08),
    },
    Mapping {
        name: "comment.note",
        hl: hl(0x0c),
    },
    Mapping {
        name: "comment.todo",
        hl: hl(0x0f),
    },
    Mapping {
        name: "comment.warning",
        hl: hl(0x09),
    },
    Mapping {
        name: "constant",
        hl: hl(0x09),
    },
    Mapping {
        name: "constant.builtin",
        hl: hl(0x09),
    },
    Mapping {
        name: "constant.macro",
        hl: hl(0x09),
    },
    Mapping {
        name: "constructor",
        hl: hl(0x0a),
    },
    Mapping {
        name: "diff.delta",
        hl: hl(0x0a),
    },
    Mapping {
        name: "diff.minus",
        hl: hl(0x08),
    },
    Mapping {
        name: "diff.plus",
        hl: hl(0x0b),
    },
    Mapping {
        name: "function",
        hl: hl(0x0d),
    },
    Mapping {
        name: "function.builtin",
        hl: hl(0x0d),
    },
    Mapping {
        name: "function.call",
        hl: hl(0x0d),
    },
    Mapping {
        name: "function.macro",
        hl: hl(0x0d),
    },
    Mapping {
        name: "function.method",
        hl: hl(0x0d),
    },
    Mapping {
        name: "function.method.call",
        hl: hl(0x0d),
    },
    Mapping {
        name: "keyword",
        hl: hl(0x0e),
    },
    Mapping {
        name: "keyword.conditional",
        hl: hl(0x0e),
    },
    Mapping {
        name: "keyword.conditional.ternary",
        hl: hl(0x0e),
    },
    Mapping {
        name: "keyword.coroutine",
        hl: hl(0x0e),
    },
    Mapping {
        name: "keyword.debug",
        hl: hl(0x0e),
    },
    Mapping {
        name: "keyword.directive",
        hl: hl(0x0e),
    },
    Mapping {
        name: "keyword.directive.define",
        hl: hl(0x0e),
    },
    Mapping {
        name: "keyword.exception",
        hl: hl(0x08),
    },
    Mapping {
        name: "keyword.function",
        hl: hl(0x0e),
    },
    Mapping {
        name: "keyword.import",
        hl: hl(0x0e),
    },
    Mapping {
        name: "keyword.modifier",
        hl: hl(0x0e),
    },
    Mapping {
        name: "keyword.operator",
        hl: hl(0x0e),
    },
    Mapping {
        name: "keyword.repeat",
        hl: hl(0x0e),
    },
    Mapping {
        name: "keyword.return",
        hl: hl(0x0e),
    },
    Mapping {
        name: "keyword.type",
        hl: hl(0x0e),
    },
    Mapping {
        name: "label",
        hl: hl(0x0e),
    },
    Mapping {
        name: "markup",
        hl: hl(0x0f),
    },
    Mapping {
        name: "markup.heading",
        hl: hl(0x0f),
    },
    Mapping {
        name: "markup.heading.1",
        hl: hl(0x0f),
    },
    Mapping {
        name: "markup.heading.2",
        hl: hl(0x0f),
    },
    Mapping {
        name: "markup.heading.3",
        hl: hl(0x0f),
    },
    Mapping {
        name: "markup.heading.4",
        hl: hl(0x0f),
    },
    Mapping {
        name: "markup.heading.5",
        hl: hl(0x0f),
    },
    Mapping {
        name: "markup.heading.6",
        hl: hl(0x0f),
    },
    Mapping {
        name: "markup.italic",
        hl: Highlight {
            colour: 0x0f,
            italic: true,
            ..hl(0x0f)
        },
    },
    Mapping {
        name: "markup.link",
        hl: Highlight {
            colour: 0x0f,
            underline: true,
            ..hl(0x0f)
        },
    },
    Mapping {
        name: "markup.link.label",
        hl: Highlight {
            colour: 0x0f,
            underline: true,
            ..hl(0x0f)
        },
    },
    Mapping {
        name: "markup.link.url",
        hl: Highlight {
            colour: 0x0d,
            underline: true,
            ..hl(0x0d)
        },
    },
    Mapping {
        name: "markup.list",
        hl: hl(0x0f),
    },
    Mapping {
        name: "markup.list.checked",
        hl: hl(0x0b),
    },
    Mapping {
        name: "markup.list.unchecked",
        hl: hl(0x0b),
    },
    Mapping {
        name: "markup.math",
        hl: hl(0x0c),
    },
    Mapping {
        name: "markup.quote",
        hl: hl(0x0f),
    },
    Mapping {
        name: "markup.raw",
        hl: hl(0x0f),
    },
    Mapping {
        name: "markup.raw.block",
        hl: hl(0x0f),
    },
    Mapping {
        name: "markup.strikethrough",
        hl: Highlight {
            colour: 0x0f,
            strikethrough: true,
            ..hl(0x0f)
        },
    },
    Mapping {
        name: "markup.strong",
        hl: Highlight {
            colour: 0x0f,
            bold: true,
            ..hl(0x0f)
        },
    },
    Mapping {
        name: "markup.underline",
        hl: Highlight {
            colour: 0x0f,
            underline: true,
            ..hl(0x0f)
        },
    },
    Mapping {
        name: "module",
        hl: hl(0x0a),
    },
    Mapping {
        name: "module.builtin",
        hl: hl(0x0a),
    },
    Mapping {
        name: "number",
        hl: hl(0x09),
    },
    Mapping {
        name: "number.float",
        hl: hl(0x09),
    },
    Mapping {
        name: "operator",
        hl: hl(0x05),
    },
    Mapping {
        name: "property",
        hl: hl(0x05),
    },
    Mapping {
        name: "punctuation.bracket",
        hl: hl(0x05),
    },
    Mapping {
        name: "punctuation.delimiter",
        hl: hl(0x05),
    },
    Mapping {
        name: "punctuation.special",
        hl: hl(0x0c),
    },
    Mapping {
        name: "string",
        hl: hl(0x0b),
    },
    Mapping {
        name: "string.documentation",
        hl: hl(0x0b),
    },
    Mapping {
        name: "string.escape",
        hl: hl(0x0c),
    },
    Mapping {
        name: "string.regexp",
        hl: hl(0x0c),
    },
    Mapping {
        name: "string.special",
        hl: hl(0x0c),
    },
    Mapping {
        name: "string.special.path",
        hl: hl(0x0c),
    },
    Mapping {
        name: "string.special.symbol",
        hl: hl(0x0c),
    },
    Mapping {
        name: "string.special.url",
        hl: Highlight {
            colour: 0x0d,
            underline: true,
            ..hl(0x0d)
        },
    },
    Mapping {
        name: "tag",
        hl: hl(0x0a),
    },
    Mapping {
        name: "tag.attribute",
        hl: hl(0x05),
    },
    Mapping {
        name: "tag.builtin",
        hl: hl(0x0c),
    },
    Mapping {
        name: "tag.delimiter",
        hl: hl(0x05),
    },
    Mapping {
        name: "type",
        hl: hl(0x0a),
    },
    Mapping {
        name: "type.builtin",
        hl: hl(0x0a),
    },
    Mapping {
        name: "type.definition",
        hl: hl(0x0a),
    },
    Mapping {
        name: "variable",
        hl: hl(0x05),
    },
    Mapping {
        name: "variable.builtin",
        hl: hl(0x05),
    },
    Mapping {
        name: "variable.member",
        hl: hl(0x05),
    },
    Mapping {
        name: "variable.parameter",
        hl: hl(0x05),
    },
    Mapping {
        name: "variable.parameter.builtin",
        hl: hl(0x05),
    },
];

pub fn get_colour(name: &str) -> Highlight {
    MAPPINGS
        .binary_search_by(|m| m.name.cmp(name))
        .map(|i| MAPPINGS[i].hl)
        .unwrap_or_else(|_| hl(0x05))
}
