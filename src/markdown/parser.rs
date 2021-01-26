// use crate::nom::markdown::MarkdownInline;
// use crate::nom::markdown::MarkdownText;
use crate::ast::{Emphasis, FlowContent, Node, PhrasingContent, StaticPhrasingContent, Text, MdastContent, Code, Strong};

use crate::ast::Heading;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take, take_while1},
    character::is_digit,
    combinator::{map, not},
    multi::{many0, many1, many_m_n, many_till},
    sequence::{delimited, pair, preceded, terminated, tuple},
    AsBytes,
    IResult,
};

// TODO: I would like this to return a list of key/value yaml pairs
fn parse_front_matter(i: &str) -> IResult<&str, &str> {
    delimited(tag("---"), is_not("---"), tag("---"))(i)
}

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
//         many_m_n(0, 3, tag(" ")),
//         many_till(alt((tag("-"), tag("_"), tag("*")), tag("\n")))
//     )(i)
// }

fn parse_bold(i: &str) -> IResult<&str, &str> {
    delimited(tag("**"), is_not("**"), tag("**"))(i)
}

fn parse_italics(i: &str) -> IResult<&str, &str> {
    delimited(tag("*"), is_not("*"), tag("*"))(i)
}

fn parse_inline_code(i: &str) -> IResult<&str, &str> {
    delimited(tag("`"), is_not("`"), tag("`"))(i)
}

fn parse_link(i: &str) -> IResult<&str, (&str, &str)> {
    pair(
        delimited(tag("["), is_not("]"), tag("]")),
        delimited(tag("("), is_not(")"), tag(")")),
    )(i)
}

fn parse_image(i: &str) -> IResult<&str, (&str, &str)> {
    pair(
        delimited(tag("!["), is_not("]"), tag("]")),
        delimited(tag("("), is_not(")"), tag(")")),
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

fn parse_markdown_inline(i: &str) -> IResult<&str, PhrasingContent> {
    alt((
        map(parse_italics, |s: &str| {
            // StaticPhrasingContent::Emphasis(s.to_string())
            PhrasingContent::StaticPhrasingContent(StaticPhrasingContent::Emphasis(Emphasis {
                children: vec![Node {
                    node_type: "text".to_string(),
                    value: Some(s.as_bytes().to_vec()),
                    position: None,
                }],
            }))
        }),
        // map(parse_inline_code, |s: &str| {
        //     // StaticPhrasingContent::InlineCode(s.to_string())
        // }),
        map(parse_bold, |s: &str| {
            PhrasingContent::StaticPhrasingContent(StaticPhrasingContent::Strong(Strong {
                children: vec![Node {
                    node_type: "text".to_string(),
                    value: Some(s.as_bytes().to_vec()),
                    position: None,
                }],
            }))
        }),
        // map(parse_image, |(tag, url): (&str, &str)| {
        //     // StaticPhrasingContent::Image(tag.to_string(), url.to_string())
        // }),
        // map(parse_link, |(tag, url): (&str, &str)| {
        //     // StaticPhrasingContent::Link(tag.to_string(), url.to_string())
        // }),
        map(parse_plaintext, |s| {
            PhrasingContent::StaticPhrasingContent(StaticPhrasingContent::Text(Text {
                // node_type: "text".to_string(),
                value: Some(s.as_bytes().to_vec()),
                position: None,
            }))
        }),
    ))(i)
}

fn parse_markdown_text(i: &str) -> IResult<&str, Vec<PhrasingContent>> {
    terminated(many0(parse_markdown_inline), tag("\n"))(i)
}

// this guy matches the literal character #
fn parse_header_tag(i: &str) -> IResult<&str, usize> {
    map(
        terminated(take_while1(|c| c == '#'), tag(" ")),
        |s: &str| s.len(),
    )(i)
}

// this combines a tuple of the header tag and the rest of the line
fn parse_header(i: &str) -> IResult<&str, (usize, Vec<PhrasingContent>)> {
    tuple((parse_header_tag, parse_markdown_text))(i)
}

fn parse_unordered_list_tag(i: &str) -> IResult<&str, &str> {
    terminated(tag("-"), tag(" "))(i)
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

// pub fn parse_markdown(i: &str) -> IResult<&str, Vec<Markdown>> {
//     many1(alt((
//         map(parse_header, |e| Markdown::Heading(e.0, e.1)),
//         map(parse_unordered_list, |e| Markdown::UnorderedList(e)),
//         map(parse_ordered_list, |e| Markdown::OrderedList(e)),
//         map(parse_code_block, |e| Markdown::Codeblock(e.to_string())),
//         map(parse_markdown_text, |e| Markdown::Line(e)),
//     )))(i)
// }

pub fn parse_markdown(i: &str) -> IResult<&str, Vec<MdastContent>> {
    many1(alt((
        map(parse_header, |e| {
            MdastContent::FlowContent(FlowContent::Heading(Heading {
                depth: e.0,
                children: e.1,
                setext: false,
            }))
        }),
        map(parse_code_block, |e| {
            MdastContent::FlowContent(FlowContent::Code(Code {
                lang: None,
                meta: None,
                value: vec![]
            }))
        }),
    )))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    // #[test]
    // fn test_parse_front_matter() {
    //     let string = "---\nAuthor: Sean\n---";
    //     assert_eq!(parse_front_matter(string), Ok(("", "\nAuthor: Sean\n")));
    // }

    // #[test]
    // fn header() {
    //     let string = "# Header";
    //     assert_eq!(
    //         parse_header(string),
    //         Ok(("", ""))
    //     );
    // }

    #[test]
    fn emphasis() {
        let string = "*alpha*";
        assert_eq!(parse_italics(string), Ok(("", "alpha")));

        let md = parse_markdown_inline(string);

        let content = md.ok().unwrap().1;

        println!("{:?}", &content);

        let serialized = serde_json::to_string(&content).unwrap();
        println!("serialized = {}", serialized);

        // assert_eq!(
        //     parse_markdown_inline(string),
        //     Ok(("", "alpha"))
        // );
    }

    #[test]
    fn strong() {
        let string = "**alpha**";
        assert_eq!(parse_bold(string), Ok(("", "alpha")));

        let md = parse_markdown_inline(string);

        let content = md.ok().unwrap().1;

        println!("{:?}", &content);

        let serialized = serde_json::to_string(&content).unwrap();
        println!("serialized = {}", serialized);

        // assert_eq!(
        //     parse_markdown_inline(string),
        //     Ok(("", "alpha"))
        // );
    }

    #[test]
    fn header() {
        let string = "# Header\n";
        // assert_eq!(
        //     parse_header(string),
        //     Ok(("",
        //         (
        //             1,
        //             Vec![PhrasingContent::StaticPhrasingContent(Heading {
        //                 depth: 1,
        //                 children: vec![],
        //                 setext: false
        //             })]
        //         )
        //     ))
        // );

        let md = parse_markdown(string);
        assert!(md.is_ok());
        let content = md.ok().unwrap().1;

        let serialized = serde_json::to_string(&content).unwrap();
        println!("serialized = {}", serialized);
    }
}
