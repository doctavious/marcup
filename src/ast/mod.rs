use nom::lib::std::fmt::Formatter;
use serde_derive::{Deserialize, Serialize};
use std::{assert, fmt};
pub mod location;
use location::Locatable;
use derive_more::{Display, From};
use std::borrow::Cow;

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
    // pub position: Option<Position>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "type: [{}], value: [{:?}]", self.node_type, self.value)
    }
}

/// Root represents a document.
/// Top level type used as the root of the tree and can never be a child
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "root")]
pub struct Root {
    pub children: Vec<Locatable<Block>>,
    // pub position: Option<Position>,
}

impl Root {
    pub fn new(children: Vec<Locatable<Block>>) -> Self {
        Self { children }
    }
}

impl NodeType for Root {
    const TYPE: &'static str = "root";
}

#[derive(Clone, Display, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "paragraph")]
pub struct Paragraph {
    // children: [PhrasingContent]
    pub children: InlineElementContainer, //Locatable<Vec<Inline>>,
                                          // pub position: Option<Position>,
}

impl Paragraph {
    pub fn new(children: InlineElementContainer) -> Paragraph {
        Self { children }
    }
    // pub fn new(children: Locatable<Vec<Inline>>) -> Paragraph {
    //     Self { children }
    // }
    // pub fn new(children: Vec<Inline>, position: Option<Position>) -> Paragraph {
    //     Self {
    //         children,
    //         position,
    //     }
    // }
}

impl<'a> From<Vec<Locatable<Inline>>> for Paragraph {
    /// Wraps multiple located inline elements in a container that is then
    /// placed inside a paragraph
    fn from(elements: Vec<Locatable<Inline>>) -> Self {
        Self::new(elements.into())
    }
}

impl<'a> From<Locatable<Inline>> for Paragraph {
    /// Wraps single, located inline element in a container that is then
    /// placed inside a paragraph
    fn from(element: Locatable<Inline>) -> Self {
        Self::new(element.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", rename = "heading")]
pub struct Heading {
    // TODO: should this be u8 or usize?
    /// The depth of the header; from 1 to 6 for ATX headings, 1 or 2 for setext headings.
    pub depth: usize,

    // children: [PhrasingContent],
    pub children: InlineElementContainer, //Locatable<Vec<Inline>>,

    /// Whether the heading is setext (if not, ATX).
    pub setext: bool,
    // pub position: Option<Position>,
}

impl Heading {
    /// Represents the smallest a header's level can be
    pub const MIN_LEVEL: usize = 1;

    /// Represents teh largest a header's level can be
    pub const MAX_LEVEL: usize = 6;

    pub fn new(depth: usize, setext: bool) -> Heading {
        if setext {
            assert!(depth == 1 || depth == 2);
        } else {
            assert!(depth >= Heading::MIN_LEVEL && depth <= Heading::MAX_LEVEL);
        }

        Heading {
            depth,
            setext,
            children: InlineElementContainer::default(), // Vec::new(),
                                                         // position: None,
        }
    }

    pub fn new_with_children(
        depth: usize,
        setext: bool,
        children: InlineElementContainer,
    ) -> Heading {
        if setext {
            assert!(depth == 1 || depth == 2);
        } else {
            assert!(depth >= Heading::MIN_LEVEL && depth <= Heading::MAX_LEVEL);
        }

        Heading {
            depth,
            setext,
            children,
            // position: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize,)]
#[serde(tag = "type", rename = "blockquote")]
pub struct BlockQuote {
    // children: [FlowContent]
    // TODO: go back to Vec<Block>
    // pub children: Vec<Block>,
    pub children: Vec<String>,
    // pub position: Option<Position>,
}

impl BlockQuote {
    pub fn new(children: Vec<String>) -> Self {
        Self { children }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "text")]
pub struct Text {
    pub value: String,
    // pub position: Option<Position>,
}

impl Text {
    pub fn new(value: String) -> Self {
        Self {
            value
        }
    }
}

impl Text {
    pub fn as_borrowed(&self) -> Text {
        // use self::Cow::*;
        //
        // let inner = Cow::Borrowed(match &self.value {
        //     Borrowed(x) => *x,
        //     Owned(x) => x.as_str(),
        // });
        //
        // Text(inner)
        Text::new(self.value.to_owned())
    }

    pub fn into_owned(self) -> Text {
        // let inner = Cow::from(self.value.into_owned());
        //
        // Text(inner)
        Text::new(self.value.to_owned())
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Text {
            value: value.to_string()
        }
    }
}

impl NodeType for Text {
    const TYPE: &'static str = "text";
}

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "emphasis")]
pub struct Emphasis {
    // children: [TransparentContent]
    pub children: Vec<Inline>,
    // pub position: Option<Position>,
}

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename = "strong")]
pub struct Strong {
    // children: [TransparentContent]
    pub children: Vec<Inline>,
    // pub position: Option<Position>,
}

impl fmt::Display for Strong {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for content in self.children.as_slice().iter() {
            write!(f, "{}", content.to_string())?;
        }
        Ok(())
    }
}

#[derive(Clone, Display, Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub enum Content {
    Paragraph(Paragraph),
}

#[derive(Clone, Display, Debug, Eq, PartialEq, Hash, Serialize, Deserialize,)]
#[display(fmt = "{:?} {:?} {}", lang, meta, value)]
#[serde(tag = "type", rename = "code")]
pub struct Code {
    // A lang field can be present. It represents the language of computer code being marked up.
    // https://github.github.com/gfm/#info-string
    pub lang: Option<String>,

    // If the lang field is present, a meta field can be present.
    // It represents custom information relating to the node.
    pub meta: Option<String>,

    pub value: String,
    // pub position: Option<Position>,
}

// TODO: alias for Vec<StaticPhrasingContent>
#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
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
            Inline::Strong(s) => {
                write!(f, "{:?}", s.children)
            }
            Inline::Text(t) => {
                write!(f, "{:?}", t.value)
            }
            _ => write!(f, "")
        }
    }
}

impl fmt::Display for Inline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Inline::Emphasis(e) => {
                for content in e.children.as_slice().iter() {
                    write!(f, "{}", content.to_string())?;
                }
            }
            Inline::Strong(e) => {
                for content in e.children.as_slice().iter() {
                    write!(f, "{}", content.to_string())?;
                }
            }
            Inline::Text(t) => {
                write!(f, "{:?}", t.value)
            }
            _ => write!(f, "")
        }
        Ok(())
    }
}

impl Inline {
    pub fn to_borrowed(&self) -> Inline {
        match self {
            Self::Emphasis(x) => Inline::from(x.to_borrowed()),
            Self::Text(x) => Inline::from(x.as_borrowed()),
            Self::Strong(x) => Inline::from(x.as_borrowed()),
        }
    }

    pub fn into_owned(self) -> Inline {
        match self {
            Self::Emphasis(x) => Inline::from(x.into_owned()),
            Self::Text(x) => Inline::from(x.into_owned()),
            Self::Strong(x) => Inline::from(x.into_owned()),
        }
    }
}

impl Inline {
    pub fn into_children(self) -> Vec<Locatable<Inline>> {
        match self {
            Self::Text(x) => vec![],
            Self::Strong(x) => x.into_children(),
            Self::Emphasis(x) => x.into_children(),
        }
    }
}

#[derive(Clone, Debug, From, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Block {
    Heading(Heading),
    Paragraph(Paragraph),
    Code(Code),
    BlockQuote(BlockQuote),
}

// Represents a convenience wrapper around a series of inline elements
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InlineElementContainer {
    pub elements: Vec<Locatable<Inline>>,
}

impl InlineElementContainer {
    pub fn new(elements: Vec<Locatable<Inline>>) -> Self {
        Self { elements }
    }

    pub fn to_borrowed(&self) -> InlineElementContainer {
        let elements = self
            .elements
            .iter()
            .map(|x| x.as_ref().map(Inline::to_borrowed))
            .collect();

        InlineElementContainer { elements }
    }

    pub fn into_owned(self) -> InlineElementContainer {
        let elements = self
            .elements
            .into_iter()
            .map(|x| x.map(Inline::into_owned))
            .collect();

        InlineElementContainer { elements }
    }
}

impl<'a> InlineElementContainer {
    pub fn into_children(self) -> Vec<Locatable<Inline>> {
        self.elements
    }
}

impl<'a> fmt::Display for InlineElementContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for le in self.elements.iter() {
            write!(f, "{}", le.as_inner().to_string())?;
        }
        Ok(())
    }
}

impl<'a> From<Vec<Locatable<Inline>>> for InlineElementContainer {
    fn from(mut elements: Vec<Locatable<Inline>>) -> Self {
        Self::new(elements)
    }
}

impl<'a> From<Vec<InlineElementContainer>> for InlineElementContainer {
    fn from(mut containers: Vec<Self>) -> Self {
        Self::new(containers.drain(..).flat_map(|c| c.elements).collect())
    }
}

impl<'a> From<Locatable<Inline>> for InlineElementContainer {
    fn from(element: Locatable<Inline>) -> Self {
        Self::new(vec![element])
    }
}

impl<'a> From<Locatable<&'a str>> for InlineElementContainer {
    fn from(element: Locatable<&'a str>) -> Self {
        Self::from(element.map(Text::from))
    }
}

macro_rules! container_mapping {
    ($type:ty) => {
        impl<'a> From<$type> for InlineElementContainer {
            fn from(element: $type) -> Self {
                Self::from(element.map(Inline::from))
            }
        }
    };
}

container_mapping!(Locatable<Text>);
