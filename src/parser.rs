// parser.rs
use std::error::Error;
use tree_sitter::{Language, Parser, Query, QueryCursor, StreamingIterator};

pub struct Capture {
    pub start: usize,
    pub end: usize,
    pub index: u32,
}

pub fn collect_captures(
    source: &[u8],
    language: &Language,
    query: &Query,
) -> Result<Vec<Capture>, Box<dyn Error>> {
    let mut parser = Parser::new();
    parser.set_language(language)?;
    let tree = parser.parse(source, None).ok_or("failed to parse source")?;
    let mut cursor = QueryCursor::new();
    let mut captures = Vec::new();
    let mut iter = cursor.captures(&query, tree.root_node(), source);

    while let Some((m, index)) = iter.next() {
        let capture = m.captures[*index];
        let start = capture.node.start_byte();
        let end = capture.node.end_byte();
        if start != end {
            captures.push(Capture {
                start: start,
                end: end,
                index: capture.index,
            });
        }
    }
    Ok(captures)
}
