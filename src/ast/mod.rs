use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use std::assert;


// TODO: anything from comrak that might be worth including? 
// - NodeValue aka NodeType?
// - DescriptionList
// - DescriptionItem
// - DescriptionTerm
// - DescriptionDetails
// - FootnoteDefinition
// - Table
// - TableRow
// - TableCell
// - TaskItem
// - Strikethrough -- we have delete
// - Superscript

// look at https://doc.rust-lang.org/edition-guide/rust-2018/trait-system/associated-constants.html
// regarding constants like type = "Root"

trait NType {
    const TYPE: String;
}

// I dont like NoteType because its more then the type
// I'm also not a fan of NodeValue
// Maybe NodeData. NodeInfo
// #[derive(Serialize, Deserialize)]
// #[serde(tag = "type", rename_all = "camelCase")] // BlockQuote doesnt follow this as the type is "blockquote"
pub enum NodeType {
    Root(Root),

    Paragraph(Paragraph),

    Heading(Heading),

    ThematicBreak,

    BlockQuote(BlockQuote),

    List(List),

    ListItem(ListItem),

    HTML(HTML),

    Code(Code),

    Definition(Definition),

    Text(Text),

    Emphasis(Emphasis),

    Strong(Strong),

    InlineCode(InlineCode),

    Break(Break),

    Link(Link),

    Image(Image),

    LinkReference(LinkReference),

    ImageReference(ImageReference),

    // TODO: add extensions
}

impl NodeType {

    // /// Indicates whether this node is a block node or inline node.
    // pub fn block(&self) -> bool {
    //     matches!(*self, NodeValue::Document
    //         | NodeValue::BlockQuote
    //         | NodeValue::FootnoteDefinition(_)
    //         | NodeValue::List(..)
    //         | NodeValue::DescriptionList
    //         | NodeValue::DescriptionItem(_)
    //         | NodeValue::DescriptionTerm
    //         | NodeValue::DescriptionDetails
    //         | NodeValue::Item(..)
    //         | NodeValue::CodeBlock(..)
    //         | NodeValue::HtmlBlock(..)
    //         | NodeValue::Paragraph
    //         | NodeValue::Heading(..)
    //         | NodeValue::ThematicBreak
    //         | NodeValue::Table(..)
    //         | NodeValue::TableRow(..)
    //         | NodeValue::TableCell)
    // }


    // pub(crate) fn accepts_lines(&self) -> bool {
    //     matches!(
    //         *self,
    //         NodeValue::Paragraph | NodeValue::Heading(..) | NodeValue::CodeBlock(..)
    //     )
    // }

    // TODO: is_parent

    // TOOD: contains_inlines

}


// Data represents information associated by the ecosystem with the node.
// TODO: should this just be a byte[]?
pub struct Data<T> {

    value: T,

}

/// Represents one place in a source file.
pub struct Point {
    /// The line field (1-indexed integer) represents a line in a source file.
    line: u64,

    /// The column field (1-indexed integer) represents a column in a source file. 
    column: u64,

    /// The offset field (0-indexed integer) represents a character in a source file.
    offset: Option<u64>
}

impl Point {
    pub fn new(line: u64, column: u64, offset: Option<u64>) -> Point {
        assert!(line >= 1);
        assert!(column >= 1);
        
        Point {
            line: line,
            column: column,
            offset: offset
        }
    }
}

/// Represents the location of a node in a source file.
/// If the syntactic unit represented by a node is not present in the source file at the time of parsing, 
/// the node is said to be generated and it must not have positional information.
pub struct Position {
    /// The start field represents the place of the first character of the parsed source region.
    start: Point,

    /// The end field represents the place of the first character after the parsed source region, whether it exists or not. 
    end: Point,

    /// The indent field (1-indexed integer) represents the start column at each index (plus start line) in the source region, for elements that span multiple lines.
    indent: Option<u32>
}

impl Position {
    pub fn new(start: Point, end: Point, indent: Option<u32>) -> Position {
        if indent.is_some() {
            assert!(indent.unwrap() >= 1);
        }
        
        Position {
            start: start,
            end: end,
            indent: indent
        }
    }
}


pub struct Node {
    pub node: NodeType,

    // aka Data
    pub value: Option<Vec<u8>>,

    pub position: Option<Position>
}

// // TODO: not sure if this should be a struct or trait
// // Syntactic units in unist syntax trees are called nodes, and implement the Node interface.
// // Specifications implementing unist are encouraged to define more fields. 
// // Ecosystems can define fields on Data.
// // Any value in unist must be expressible in JSON values: string, number, object, array, true, false, or null. 
// // This means that the syntax tree should be able to be converted to and from JSON and produce the same tree. 
// // For example, in JavaScript, a tree can be passed through JSON.parse(JSON.stringify(tree)) and result in the same tree.
// pub struct Node {
//     // The type field is a non-empty string representing the variant of a node. 
//     // This field can be used to determine the type a node implements.
//     type: String,

//     // The data field represents information from the ecosystem. 
//     // The value of the data field implements the Data interface.
//     data: Option<Data>,

//     // The position field represents the location of a node in a source document. 
//     // The value of the position field implements the Position interface. 
//     // The position field must not be present if a node is generated.
//     position: Option<Position>

// }

// TOOD: given Rust doesnt have strutural inheritance we cant extend other structs and this doesnt really feel like a trait
// so Rust pushes you towards structural composition. As a result I think we just want to include a 
// children: Node[] field on structs that are parents and have a utility fn to determine if a node is a parent or not
// TODO: implement Node
// Nodes containing other nodes (said to be children) extend the abstract interface Parent (Node).
// pub struct Parent {
//     // The children field is a list representing the children of a node.
//     // mdas specifies this as MdastContent
//     children: Node[]
// }

// Data vs Literal???
// Do we get rid of this? Is it needed? We can probably just make data/value either a string/&str or byte[]/vec
// TODO: implement Node
// Nodes containing a value extend the abstract interface Literal (Node).
// pub struct Literal<T> {
//     // The value field can contain any value.
//     // TODO: does that mean its generic? 
//     // mdas specifies this as a String
//     value: T
// }

// Root (Parent) represents a document.
// implements Parent
// Root can be used as the root of a tree, never as a child. 
// Its content model is not limited to flow content, but can contain any mdast content with the restriction that all content must be of the same category.
pub struct Root {
    // type: "root"
}

// implements Parent
// Paragraph (Parent) represents a unit of discourse dealing with a particular point or idea.
// Paragraph can be used where content is expected. Its content model is phrasing content.
pub struct Paragraph {
    // type: "paragraph"
    // children: [PhrasingContent]
    children: Vec<Node>,
}

// implements Parent
// Heading (Parent) represents a heading of a section.
// Heading can be used where flow content is expected. Its content model is phrasing content.
// A depth field must be present. A value of 1 is said to be the highest rank and 6 the lowest.
pub struct Heading {
    // type: "heading"

    /// The depth of the header; from 1 to 6 for ATX headings, 1 or 2 for setext headings.
    depth: u8,

    // children: [PhrasingContent],
    children: Vec<Node>,

    /// Whether the heading is setext (if not, ATX).
    pub setext: bool,
}

impl Heading {
    pub fn new(depth: u8, setext: bool) -> Heading {
        if setext {
            assert!(depth == 1 || depth == 2);
        } else {
            assert!(depth >= 1 && depth <= 6);
        }
        
        Heading {
            depth: depth,
            setext: setext,
            children: Vec::new()
        }
    }
}

// implements Node
// ThematicBreak (Node) represents a thematic break, such as a scene change in a story, a transition to another topic, or a new document.
// ThematicBreak can be used where flow content is expected. It has no content model.
pub struct ThematicBreak {
    // type: "thematicBreak"
}

// implements Parent
// Blockquote (Parent) represents a section quoted from somewhere else.
// Blockquote can be used where flow content is expected. Its content model is also flow content.
pub struct BlockQuote {
    // type: "blockquote"

    // children: [FlowContent]
    children: Vec<Node>,
}

// implements Parent
// List (Parent) represents a list of items.
// List can be used where flow content is expected. Its content model is list content.
pub struct List {
    // type: "list"

    // An ordered field can be present. 
    // It represents that the items have been intentionally ordered (when true), 
    // or that the order of items is not important (when false or not present).
    ordered: Option<bool>,

    // A start field can be present. It represents, when the ordered field is true, the starting number of the list.
    start: Option<u64>,

    // A spread field can be present. It represents that one or more of its children are separated with a blank line from its siblings (when true), or not (when false or not present).
    spread: Option<bool>,

    // is spread different then loose and tight
    // https://github.github.com/gfm/#tight

    // children: [ListContent]
    children: Vec<Node>,
}


// implements Parent
// ListItem (Parent) represents an item in a List.
// ListItem can be used where list content is expected. Its content model is flow content.
pub struct ListItem {
    // type: "listItem"

    // A spread field can be present. It represents that the item contains two or more children separated by a blank line (when true), or not (when false or not present).
    spread: Option<bool>,

    // children: [FlowContent]
    children: Vec<Node>,
}

// implements literal
// HTML (Literal) represents a fragment of raw HTML.
// HTML can be used where flow or phrasing content is expected. Its content is represented by its value field.
// HTML nodes do not have the restriction of being valid or complete HTML ([HTML]) constructs.
pub struct HTML {
    // type: "html"
    value: Vec<u8>,
}

// implements literal
// Code (Literal) represents a block of preformatted text, such as ASCII art or computer code.
// Code can be used where flow content is expected. Its content is represented by its value field.
// This node relates to the phrasing content concept InlineCode.
pub struct Code {
    // type: "code"

    // A lang field can be present. It represents the language of computer code being marked up.
    // https://github.github.com/gfm/#info-string
    lang: Option<String>,

    // If the lang field is present, a meta field can be present. 
    // It represents custom information relating to the node.
    meta: Option<String>,

    value: Vec<u8>,
}


// implements node
// Definition (Node) represents a resource.
// Definition can be used where content is expected. It has no content model.
// Definition includes the mixins Association and Resource.
// Definition should be associated with LinkReferences and ImageReferences.
// TODO: what the heck is a mixin and how to model that in rust?
pub struct Definition {
    // type: "definition"
}

// implements literal
// Text (Literal) represents everything that is just text.
// Text can be used where phrasing content is expected. Its content is represented by its value field.
pub struct Text {
    // type: "text"
    
    value: Vec<u8>,
}

// implements Parent
// Emphasis (Parent) represents stress emphasis of its contents.
// Emphasis can be used where phrasing content is expected. Its content model is transparent content.
pub struct Emphasis {
    // type: "emphasis"
    
    // children: [TransparentContent]
    children: Vec<Node>,
}


// implements Parent
// Strong (Parent) represents strong importance, seriousness, or urgency for its contents.
// Strong can be used where phrasing content is expected. Its content model is transparent content.
pub struct Strong {
    // type: "emphasis"

    // children: [TransparentContent]
    children: Vec<Node>,
}

// implements literal
// InlineCode (Literal) represents a fragment of computer code, such as a file name, computer program, or anything a computer could parse.
// InlineCode can be used where phrasing content is expected. Its content is represented by its value field.
// This node relates to the flow content concept Code.
// https://github.github.com/gfm/#code-spans
pub struct InlineCode {
    // type: "inlineCode"

    value: Vec<u8>,
}

// implements Node
// Break (Node) represents a line break, such as in poems or addresses.
// Break can be used where phrasing content is expected. It has no content model
pub struct Break {
    // type: "break"
}

// implements Parent
// Link includes Resource
// Link (Parent) represents a hyperlink.
// Link can be used where phrasing content is expected. Its content model is static phrasing content.
// Link includes the mixin Resource.
pub struct Link {
    // type: "link"

    // children: [StaticPhrasingContent]
    children: Vec<Node>,
}

// implements Node
// Image (Node) represents an image.
// Image can be used where phrasing content is expected. It has no content model, but is described by its alt field.
// Image includes the mixins Resource and Alternative.
pub struct Image {
    // type: "image"
}

// implements Parent
// LinkReference (Parent) represents a hyperlink through association, or its original source if there is no association.
// LinkReference can be used where phrasing content is expected. Its content model is static phrasing content.
// LinkReference includes the mixin Reference.
// LinkReferences should be associated with a Definition.
pub struct LinkReference {
    // type: "linkReference"
    
    // children: [StaticPhrasingContent]
    children: Vec<Node>,
}

// implements Node
// ImageReference (Node) represents an image through association, or its original source if there is no association.
// ImageReference can be used where phrasing content is expected. It has no content model, but is described by its alt field
// ImageReference includes the mixins Reference and Alternative.
// ImageReference should be associated with a Definition.
pub struct ImageReference {
    // type: "imageReference"
}


// ------ MIXINS ------

/// Resource represents a reference to resource.
pub trait Resource {
    // A url field must be present. It represents a URL to the referenced resource.
    fn url(&self) -> String;

    // A title field can be present. It represents advisory information for the resource, such as would be appropriate for a tooltip.
    fn title(&self) -> Option<String>;
}

// Association represents an internal relation from one node to another.
// To normalize a value, collapse markdown whitespace ([\t\n\r ]+) to a space, trim the optional initial and/or final space, and perform case-folding.
// Whether the value of identifier (or normalized label if there is no identifier) is expected to be a unique identifier or not depends on the type of node including the Association. 
// An example of this is that they should be unique on Definition, whereas multiple LinkReferences can be non-unique to be associated with one definition.
pub struct Association {
    // An identifier field must be present. It can match another node. 
    // identifier is a source value: character escapes and character references are not parsed. Its value must be normalized.
    identifier: String,

    // A label field can be present. 
    // label is a string value: it works just like title on a link or a lang on code: character escapes and character references are parsed.
    label: Option<String>
}

// Reference represents a marker that is associated to another node.
pub struct Reference {
    // A referenceType field must be present. 
    // Its value must be a referenceType. It represents the explicitness of the reference.
    referenceType: String,
}

// Alternative represents a node with a fallback
pub struct Alternative {
    // An alt field should be present. 
    // It represents equivalent content for environments that cannot represent the node as intended.
    alt: Option<String>,
}


// ------ ENUMERATION ------

/// Represents the explicitness of a reference.
pub enum ReferenceType {
    /// shortcut: the reference is implicit, its identifier inferred from its content
    Shortcut,

    /// collapsed: the reference is explicit, its identifier inferred from its content
    Collapsed,

    /// full: the reference is explicit, its identifier explicitly set
    Full
}


// ------ CONTENT MODEL ------

// type MdastContent = FlowContent | ListContent | PhrasingContent
// Each node in mdast falls into one or more categories of Content that group nodes with similar characteristics together.
// union MdasContent {
//     flow_content: FlowContent,
//     list_content: ListContent,
//     phrasing_content: PhrasingContent
// }

enum MdasContent {
    FlowContent(FlowContent),
    ListContent(ListContent),
    PhrasingContent(PhrasingContent),
}

// another possibility is to use trait
// pub trait MdasContent {}
// impl MdasContent for FlowContent {}

// type FlowContent = Blockquote | Code | Heading | HTML | List | ThematicBreak | Content
// Flow content represent the sections of document.
// union FlowContent {
//     blockquote: BlockQuote,
//     code: Code,
//     heading: Heading,
//     html: HTML,
//     list: List,
//     thematic_break: ThematicBreak,
//     content: Content,
// }

// another possibility is to use trait
// pub trait FlowContent {}
// impl FlowContent for BlockQuote {}
// impl FlowContent for Code {}
// impl FlowContent for Heading {}
// impl FlowContent for HTML {}
// impl FlowContent for List {}
// impl FlowContent for ThematicBreak {}
// impl FlowContent for Content {}

// pub struct Foo<T: FlowContent> {}

// or maybe yet just use another enum?
enum FlowContent {
    BlockQuote(BlockQuote),
    Code(Code),
    Heading(Heading),
    HTML(HTML),
    List(List),
    ThematicBreak(ThematicBreak),
    Content(Content),
}


// type Content = Definition | Paragraph
// Content represents runs of text that form definitions and paragraphs.
// union Content {
//     definition: Definition,
//     paragraph: Paragraph,
// }

enum Content {
    Definition(Definition),
    Paragraph(Paragraph)
}

// type ListContent = ListItem
// List content represent the items in a list.
// union ListContent {
//     list_item: ListItem;
// }

enum ListContent {
    ListItem(ListItem),
}

// type PhrasingContent = Link | LinkReference | StaticPhrasingContent
// Phrasing content represent the text in a document, and its markup.
// union PhrasingContent {
//     link: Link,
//     link_reference: LinkReference,
//     static_phrasing_content: StaticPhrasingContent
// }

enum PhrasingContent {
    Link(Link),
    LinkReference(LinkReference),
    StaticPhrasingContent(StaticPhrasingContent)
}

// type StaticPhrasingContent = Break | Emphasis | HTML | Image | ImageReference | InlineCode | Strong | Text
// StaticPhrasing content represent the text in a document, and its markup, that is not intended for user interaction.
// union StaticPhrasingContent {
//     break: Break,
//     emphasis: Emphasis,
//     html: HTML,
//     image: Image,
//     image_reference: ImageReference,
//     inline_code: InlineCode,
//     strong: Strong,
//     text: Text,
// }

enum StaticPhrasingContent {
    Break(Break),
    Emphasis(Emphasis),
    HTML(HTML),
    Image(Image),
    ImageReference(ImageReference),
    InlineCode(InlineCode),
    Strong(Strong),
    Text(Text),
}


// TODO: not sure how to handle as enums cant be used as a bound generic, only traits can
// we could go the trait route but that seems funky to me
// From what I can see TranparentContent parents are currently PhrasingContent so perhaps we use that
// and thats good enough
// The transparent content model is derived from the content model of its parent. 
// Effectively, this is used to prohibit nested links (and link references).
// struct TransparentContent {

// }

enum TransparentContent {
    PhrasingContent(PhrasingContent)
}


// ------ EXTENSIONS ------

// ### GFM ###

// implements Parent
// Table (Parent) represents two-dimensional data.
// Table can be used where flow content is expected. Its content model is table content.
// The head of the node represents the labels of the columns.
pub struct Table {
    // type: "table"

    // Represents how cells in columns are aligned.
    align: Option<Vec<AlignType>>,

    // children: [TableContent]
}

// implements Parent
// TableRow (Parent) represents a row of cells in a table.
// TableRow can be used where table content is expected. Its content model is row content.
// If the node is a head, it represents the labels of the columns for its parent Table.
pub struct TableRow {
    // type: "tableRow"
    // children: [RowContent]
}

// implements Parent
// TableCell (Parent) represents a header cell in a Table, if its parent is a head, or a data cell otherwise.
// TableCell can be used where row content is expected. Its content model is phrasing content excluding Break nodes.

pub struct TableCell {
    // type: "tableCell"
    // children: [PhrasingContent]
}

// implements ListItem
pub struct ListItemGfm {
    // In GFM, a checked field can be present. 
    // It represents whether the item is done (when true), not done (when false), or indeterminate or not applicable (when null or not present).
    checked: Option<bool>,
}

// implements Parent
// Delete (Parent) represents contents that are no longer accurate or no longer relevant.
// Delete can be used where phrasing content is expected. Its content model is transparent content.
// aka strikethrough
pub struct Delete {
    // type: "delete"
    // children: [TransparentContent]
}

/// Represents how phrasing content is aligned ([CSSTEXT]).
enum AlignType {
    // See the left value of the text-align CSS property
    Left,
    // See the right value of the text-align CSS property
    Right,
    // See the center value of the text-align CSS property
    Center,
    // phrasing content is aligned as defined by the host environment
    None,
}

// type FlowContentGfm = Table | FlowContent
// union FlowContentGfm {
//     table: Table,
//     flow_content: FlowContent
// }

enum FlowContentGfm {
    Table(Table),
    FlowContent(FlowContent),
}

// type TableContent = TableRow
// Table content represent the rows in a table.
// union TableContent {
//     table_row: TableRow
// }

enum TableContent {
    TableRow(TableRow),
}


// type RowContent = TableCell
// Row content represent the cells in a row.
// union RowContent {
//     table_cell: TableCell
// }

enum RowContent {
    TableCell(TableCell),
}

// type ListContentGfm = ListItemGfm
// union ListContentGfm {
//     list_item_gfm: ListItemGfm
// }

enum ListContentGfm {
    ListItemGfm(ListItemGfm),
}

// type StaticPhrasingContentGfm = Delete | StaticPhrasingContent
// union StaticPhrasingContentGfm {
//     delete: Delete,
//     static_phrasing_content: StaticPhrasingContent
// }

enum StaticPhrasingContentGfm {
    Delete(Delete),
    StaticPhrasingContent(StaticPhrasingContent),
}


// ### Frontmatter ###


// implements Literal
// The following interfaces are found with YAML.
// YAML (Literal) represents a collection of metadata for the document in the YAML ([YAML]) data serialisation language.
// YAML can be used where frontmatter content is expected. Its content is represented by its value field.
pub struct YAML {
    // type: "yaml"

    // TODO: I would like this to be a list of key/value pairs
    value: Vec<u8>,
}


// type FrontmatterContent = YAML
// Frontmatter content represent out-of-band information about the document.
// If frontmatter is present, it must be limited to one node in the tree, and can only exist as a head.
// union FrontmatterContent {
//     yaml: YAML,
// }

enum FrontmatterContent {
    YAML(YAML),
}

// type FlowContentFrontmatter = FrontmatterContent | FlowContent
// union FlowContentFrontmatter {
//     frontmatter_content: FrontmatterContent,
//     flow_content: FlowContent
// }

enum FlowContentFrontmatter {
    FrontmatterContent(FrontmatterContent),
    FlowContent(FlowContent),
}

// implements Parent
// FootnoteDefinition (Parent) represents content relating to the document that is outside its flow.
// FootnoteDefinition can be used where flow content is expected. Its content model is also flow content.
// FootnoteDefinition includes the mixin Association.
// FootnoteDefinition should be associated with FootnoteReferences.
pub struct FootnoteDefinition {
    // type: "footnoteDefinition"
    // children: [FlowContent]
}

// implements Parent
// Footnote (Parent) represents content relating to the document that is outside its flow.
// Footnote can be used where phrasing content is expected. Its content model is also phrasing content.
pub struct Footnote {
    // type: "footnote"
    // children: [PhrasingContent]
}


// implement Node
// FootnoteReference (Node) represents a marker through association.
// FootnoteReference can be used where phrasing content is expected. It has no content model.
// FootnoteReference includes the mixin Association.
// FootnoteReference should be associated with a FootnoteDefinition.
pub struct FootnoteReference {
    // type: "footnoteReference"
}



// type FlowContentFootnotes = FootnoteDefinition | FlowContent
// union FlowContentFootnotes {
//     footnote_definition: FootnoteDefinition,
//     flow_content: FlowContent
// }

enum FlowContentFootnotes {
    FootnoteDefinition(FootnoteDefinition),
    FlowContent(FlowContent),
}


// type StaticPhrasingContentFootnotes = Footnote | FootnoteReference | StaticPhrasingContent
// union StaticPhrasingContentFootnotes {
//     footnote: Footnote,
//     footnote_reference: FootnoteReference,
//     static_phrasing_content: StaticPhrasingContent
// }

enum StaticPhrasingContentFootnotes {
    Footnote(Footnote),
    FootnoteReference(FootnoteReference),
    StaticPhrasingContent(StaticPhrasingContent),
}