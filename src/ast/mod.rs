use nom::lib::std::fmt::Formatter;
use serde_derive::{Deserialize, Serialize};
use std::{assert, fmt};


// I dont like NoteType because its more then the type
// I'm also not a fan of NodeValue
// Maybe NodeData. NodeInfo
// #[derive(Serialize, Deserialize)]
// #[serde(tag = "type", rename_all = "camelCase")] // BlockQuote doesnt follow this as the type is "blockquote"
pub enum NodeType {
    // Root(Root),

    Paragraph(Paragraph),

    Heading(Heading),

    ThematicBreak,

    Text(Text),

    Emphasis(Emphasis),

    Strong(Strong),
}

/// Represents one place in a source file.
#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    /// The line field (1-indexed integer) represents a line in a source file.
    line: u64,

    /// The column field (1-indexed integer) represents a column in a source file.
    column: u64,

    /// The offset field (0-indexed integer) represents a character in a source file.
    offset: Option<u64>,
}

impl Point {
    pub fn new(line: u64, column: u64, offset: Option<u64>) -> Point {
        assert!(line >= 1);
        assert!(column >= 1);

        Point {
            line,
            column,
            offset,
        }
    }
}

/// Represents the location of a node in a source file.
/// If the syntactic unit represented by a node is not present in the source file at the time of parsing,
/// the node is said to be generated and it must not have positional information.
#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    /// The start field represents the place of the first character of the parsed source region.
    start: Point,

    /// The end field represents the place of the first character after the parsed source region,
    /// whether it exists or not.
    end: Point,

    // TODO: remark/unify doesnt appear to include this in the json output
    /// The indent field (1-indexed integer) represents the start column at each index
    /// (plus start line) in the source region, for elements that span multiple lines.
    indent: Option<u32>,
}

impl Position {
    pub fn new(start: Point, end: Point, indent: Option<u32>) -> Position {
        if indent.is_some() {
            assert!(indent.unwrap() >= 1);
        }

        Position { start, end, indent }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Node {
    // pub node: NodeType,
    #[serde(rename = "type")]
    pub node_type: String,

    // aka Data
    pub value: Option<Vec<u8>>,

    pub position: Option<Position>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "type: [{}], value: [{:?}]",
            self.node_type,
            std::str::from_utf8(self.value.as_ref().unwrap())
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Paragraph {
    // type: "paragraph"
    // children: [PhrasingContent]
    pub children: Vec<Inline>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Heading {
    // type: "heading"

    // TODO: should this be u8 or usize?
    /// The depth of the header; from 1 to 6 for ATX headings, 1 or 2 for setext headings.
    pub depth: usize,

    // children: [PhrasingContent],
    pub children: Vec<Inline>,

    /// Whether the heading is setext (if not, ATX).
    pub setext: bool,
}

impl Heading {
    pub fn new(depth: usize, setext: bool) -> Heading {
        if setext {
            assert!(depth == 1 || depth == 2);
        } else {
            assert!(depth >= 1 && depth <= 6);
        }

        Heading {
            depth,
            setext,
            children: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub struct Text {
    // type: "text"
    // #[serde(rename = "type")]
    pub node_type: String,
    pub value: Option<Vec<u8>>,
    pub position: Option<Position>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Emphasis {
    // type: "emphasis"

    // children: [TransparentContent]
    pub children: Vec<Node>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Strong {
    // type: "strong"

    // children: [TransparentContent]
    pub children: Vec<Node>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Content {
    Paragraph(Paragraph),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Code {
    // type: "code"

    // A lang field can be present. It represents the language of computer code being marked up.
    // https://github.github.com/gfm/#info-string
    pub lang: Option<String>,

    // If the lang field is present, a meta field can be present.
    // It represents custom information relating to the node.
    pub meta: Option<String>,

    pub value: Vec<u8>,
}

// TODO: alias for Vec<StaticPhrasingContent>
#[derive(Serialize, Deserialize)]
pub enum Inline {
    /// aka italic
    Emphasis(Emphasis),
    /// aka bold
    Strong(Strong),
    /// aka plaintext
    Text(Text),
}

impl fmt::Debug for Inline {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Inline::Emphasis(e) => {
                write!(f, "{:?}", e.children)
            }
            Inline::Text(t) => {
                write!(f, "{:?}", t.value)
            }
            _ => write!(f, ""),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Block {
    Heading(Heading),
    Paragraph(Paragraph),
    Code(Code)
}