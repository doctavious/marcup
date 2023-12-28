use crate::parsers::Span;
use serde::{Deserialize, Serialize};

/// Represents the location of a node in a source file.
/// If the syntactic unit represented by a node is not present in the source file at the time of parsing,
/// the node is said to be generated and it must not have positional information.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Serialize, Deserialize)]
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
    pub fn new(start: Point, end: Point) -> Position {
        Position {
            start,
            end,
            indent: None,
        }
    }

    pub fn new_with_indent(start: Point, end: Point, indent: u32) -> Position {
        assert!(indent >= 1);
        Self {
            start,
            end,
            indent: Some(indent),
        }
    }

    /// Consumes the given region and returns a new one with its position
    /// set to the provided position
    pub fn with_indent(self, indent: u32) -> Self {
        Self::new_with_indent(self.start, self.end, indent)
    }

    // TODO: I might put this on span instead and call it to_position/as_position
    // as I rather not have this depend on parser package
    /// Constructs a new position based on the offset and length of the given span.
    pub fn from_span(span: Span) -> Self {
        let start = Point::new(span.line(), span.column(), span.start_offset());
        let end = Point::new(span.end_line(), span.end_column(), span.end_offset());
        Self::new(start, end)
    }

    /// Checks if a offset is contained within this position
    #[inline]
    pub fn contains(&self, offset: usize) -> bool {
        offset >= self.starting_offset() && offset < self.ending_offset()
    }

    /// The starting offset of the position relative to some span of input
    #[inline]
    pub fn starting_offset(&self) -> usize {
        self.start.offset()
    }

    /// The ending offset of the position relative to some span of input
    #[inline]
    pub fn ending_offset(&self) -> usize {
        self.end.offset()
    }

    /// The length of the position
    #[inline]
    pub fn len(&self) -> usize {
        self.end.offset() - self.start.offset()
    }

    /// Returns true if the length of the position is zero
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Represents one place in a source file.
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Point {
    /// The line field (1-indexed integer) represents a line in a source file.
    line: usize,

    /// The column field (1-indexed integer) represents a column in a source file.
    column: usize,

    /// The offset field (0-indexed integer) represents a character in a source file.
    offset: usize,
}

impl Point {
    pub fn new(line: usize, column: usize, offset: usize) -> Point {
        assert!(line >= 1);
        assert!(column >= 1);
        assert!(offset >= 0);
        Self {
            line,
            column,
            offset,
        }
    }

    #[inline]
    pub fn line(&self) -> usize {
        self.line
    }

    #[inline]
    pub fn column(&self) -> usize {
        self.column
    }

    #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_should_successfully_return_whether_or_not_offset_within_position() {
        let position = Position::new(Point::new(1, 3, 2), Point::new(1, 5, 4));
        assert!(!position.contains(0));
        assert!(!position.contains(1));
        assert!(!position.contains(2));
        assert!(position.contains(3));
        assert!(position.contains(4));
        assert!(!position.contains(5));
        assert!(!position.contains(6));
        assert!(!position.contains(7));
    }
}
