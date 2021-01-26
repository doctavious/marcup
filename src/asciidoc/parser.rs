// use crate::nom::markdown::MarkdownInline;
// use crate::nom::markdown::MarkdownText;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take, take_while1},
    character::is_digit,
    combinator::{map, not},
    multi::{many1},
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};

// TODO: I would like this to return a list of key/value yaml pairs
fn parse_front_matter(i: &str) -> IResult<&str, &str> {
    delimited(tag("---"), is_not("---"), tag("---"))(i)
}

// TODO: Thematic break
// '''
// or markdown
// ---
// - - -
// ***
// * * *

// TODO: page break
// <<<

// TODO: highlight
// When text is enclosed in a pair of single or double hash symbols (#), and no style is
// assigned to it, the text will be rendered as highlighted text (text wrapped in <mark>).
// #

// TODO: Description Lists
// A description list (often abbreviated as dlist) is useful when you need to include a description, definition, or supporting text for one or more terms. Each item in a description list consists of a term or phrase followed by:
// one or more terms
// a separator following each term (typically a double colon, ::)
// at least one space or endline
// the supporting content (i.e., description) (can be text, attached blocks, or both)

// TODO: Admonitions
// The label must be uppercase and immediately followed by a colon (:).
// Separate the first line of the paragraph from the label by a single space.

// TODO: sidebar
// If the sidebar content is contiguous, the block style sidebar can be placed directly on top of the text in an attribute list ([]).

// TODO: single line comment
// // A single-line comment.

// TODO: comment block
////
// A block comment.
//
// Notice it's a delimited block.
////

// from nom json example
// fn key_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
//     i: &'a str,
//   ) -> IResult<&'a str, (&'a str, JsonValue), E> {
//     separated_pair(
//       preceded(sp, string),
//       cut(preceded(sp, char(':'))),
//       json_value,
//     )(i)
//   }

// // A line consisting of 0-3 spaces of indentation, followed by a sequence of three or more matching -, _, or * characters,
// // each followed optionally by any number of spaces or tabs, forms a thematic break.
// fn parse_thematic_break(i: &str) -> IResult<&str, &str> {
//     terminated(
//         many_m_n(0, 3, tag(" ")), many_til(alt((tag("-"), tag("_"), tag("*")), tag("\n")))
//     )(i)
// }

// Constrained means "around a word". Unconstrained means "anywhere".

// constrained bold
fn parse_constrained_bold(i: &str) -> IResult<&str, &str> {
    delimited(tag("*"), is_not("*"), tag("*"))(i)
}

// TODO: might have to improve this
fn parse_unconstrained_bold(i: &str) -> IResult<&str, &str> {
    delimited(tag("**"), is_not("**"), tag("**"))(i)
}

fn parse_constrained_italics(i: &str) -> IResult<&str, &str> {
    delimited(tag("_"), is_not("_"), tag("_"))(i)
}

// TODO: might have to improve this
fn parse_unconstrained_italics(i: &str) -> IResult<&str, &str> {
    delimited(tag("__"), is_not("__"), tag("__"))(i)
}

fn parse_constrained_monospace(i: &str) -> IResult<&str, &str> {
    delimited(tag("`"), is_not("1"), tag("1"))(i)
}

// TODO: might have to improve this
fn parse_unconstrained_monospace(i: &str) -> IResult<&str, &str> {
    delimited(tag("``"), is_not("``"), tag("``"))(i)
}

// TODO: might have to improve this.
fn parse_inline_code(i: &str) -> IResult<&str, &str> {
    delimited(tag("`+"), is_not("`+"), tag("`+"))(i)
}

// fn parse_link(i: &str) -> IResult<&str, (&str, &str)> {
//     delimited(tag("["), is_not("]"), tag("]"))(i)
// }

fn parse_image(i: &str) -> IResult<&str, (&str, &str)> {
    pair(
        delimited(tag("image::"), is_not("["), tag("[")),
        delimited(tag("["), is_not("]"), tag("]")),
    )(i)
}

// we want to match many things that are not any of our special tags
// but since we have no tools available to match and consume in the negative case (without regex)
// we need to match against our tags, then consume one char
// we repeat this until we run into one of our special characters
// then we join our array of characters into a String
fn parse_plaintext(i: &str) -> IResult<&str, String> {
    map(
        many1(preceded(
            not(alt((tag("*"), tag("`"), tag("["), tag("!["), tag("\n")))),
            take(1u8),
        )),
        |vec| vec.join(""),
    )(i)
}

// fn parse_markdown_inline(i: &str) -> IResult<&str, MarkdownInline> {
//     alt((
//         map(parse_italics, |s: &str| {
//             MarkdownInline::Italic(s.to_string())
//         }),
//         map(parse_inline_code, |s: &str| {
//             MarkdownInline::InlineCode(s.to_string())
//         }),
//         map(parse_boldtext, |s: &str| {
//             MarkdownInline::Bold(s.to_string())
//         }),
//         map(parse_image, |(tag, url): (&str, &str)| {
//             MarkdownInline::Image(tag.to_string(), url.to_string())
//         }),
//         map(parse_link, |(tag, url): (&str, &str)| {
//             MarkdownInline::Link(tag.to_string(), url.to_string())
//         }),
//         map(parse_plaintext, |s| MarkdownInline::Plaintext(s)),
//     ))(i)
// }
//
// fn parse_markdown_text(i: &str) -> IResult<&str, MarkdownText> {
//     terminated(many0(parse_markdown_inline), tag("\n"))(i)
// }

// this guy matches the literal character #
fn parse_header_tag(i: &str) -> IResult<&str, usize> {
    map(
        terminated(take_while1(|c| c == '='), tag(" ")),
        |s: &str| s.len(),
    )(i)
}

// // this combines a tuple of the header tag and the rest of the line
// fn parse_header(i: &str) -> IResult<&str, (usize, MarkdownText)> {
//     tuple((parse_header_tag, parse_markdown_text))(i)
// }

fn parse_unordered_list_tag(i: &str) -> IResult<&str, &str> {
    terminated(tag("*"), tag(" "))(i)
}

// fn parse_unordered_list_element(i: &str) -> IResult<&str, MarkdownText> {
//     preceded(parse_unordered_list_tag, parse_markdown_text)(i)
// }
//
// fn parse_unordered_list(i: &str) -> IResult<&str, Vec<MarkdownText>> {
//     many1(parse_unordered_list_element)(i)
// }

fn parse_ordered_list_tag(i: &str) -> IResult<&str, &str> {
    terminated(
        terminated(take_while1(|d| is_digit(d as u8)), tag(".")),
        tag(" "),
    )(i)
}

// fn parse_ordered_list_element(i: &str) -> IResult<&str, MarkdownText> {
//     preceded(parse_ordered_list_tag, parse_markdown_text)(i)
// }
//
// fn parse_ordered_list(i: &str) -> IResult<&str, Vec<MarkdownText>> {
//     many1(parse_ordered_list_element)(i)
// }

fn parse_code_block(i: &str) -> IResult<&str, &str> {
    delimited(tag("```"), is_not("```"), tag("```"))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_front_matter() {
        let string = "---\nAuthor: Sean\n---";
        assert_eq!(parse_front_matter(string), Ok(("", "\nAuthor: Sean\n")));
    }

    // #[test]
    // fn header() {
    //     let string = "= Header";
    //     assert_eq!(
    //         parse_header(string),
    //         Ok(("", ""))
    //     );
    // }
}
