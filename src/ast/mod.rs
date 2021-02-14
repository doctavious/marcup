use nom::lib::std::fmt::Formatter;
use serde_derive::{Deserialize, Serialize};
use std::{assert, fmt};

pub enum Nodes {
    Root(Root),

    Paragraph(Paragraph),

    BlockQuote(BlockQuote),

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

trait NodeType {
    const TYPE: &'static str;

    fn node_type() -> &'static str {
        Self::TYPE
    }
}

#[derive(Serialize, Deserialize)]
pub struct Node {
    // pub node: NodeType,
    #[serde(rename = "type")]
    pub node_type: String,

    // aka Data
    pub value: Option<String>,

    pub position: Option<Position>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "type: [{}], value: [{:?}]",
            self.node_type,
            self.value
        )
    }
}

/// Root represents a document.
/// Top level type used as the root of the tree and can never be a child
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "root")]
pub struct Root {
    pub children: Vec<Block>,
    pub position: Option<Position>,
}

impl NodeType for Root {
    const TYPE: &'static str = "root";
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "paragraph")]
pub struct Paragraph {
    // children: [PhrasingContent]
    pub children: Vec<Inline>,
    pub position: Option<Position>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "heading")]
pub struct Heading {

    // TODO: should this be u8 or usize?
    /// The depth of the header; from 1 to 6 for ATX headings, 1 or 2 for setext headings.
    pub depth: usize,

    // children: [PhrasingContent],
    pub children: Vec<Inline>,

    /// Whether the heading is setext (if not, ATX).
    pub setext: bool,

    pub position: Option<Position>,
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
            position: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "blockquote")]
pub struct BlockQuote {
    // children: [FlowContent]
    pub children: Vec<Block>,
    pub position: Option<Position>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "text")]
pub struct Text {
    pub value: Option<String>,
    pub position: Option<Position>,
}

impl NodeType for Text {
    const TYPE: &'static str = "text";
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "emphasis")]
pub struct Emphasis {
    // children: [TransparentContent]
    pub children: Vec<Node>,
    pub position: Option<Position>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "strong")]
pub struct Strong {
    // children: [TransparentContent]
    pub children: Vec<Node>,
    pub position: Option<Position>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Content {
    Paragraph(Paragraph),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "code")]
pub struct Code {
    // A lang field can be present. It represents the language of computer code being marked up.
    // https://github.github.com/gfm/#info-string
    pub lang: Option<String>,

    // If the lang field is present, a meta field can be present.
    // It represents custom information relating to the node.
    pub meta: Option<String>,

    pub value: String,

    pub position: Option<Position>,
}

// TODO: alias for Vec<StaticPhrasingContent>
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Block {
    Heading(Heading),
    Paragraph(Paragraph),
    Code(Code),
    BlockQuote(BlockQuote),
}