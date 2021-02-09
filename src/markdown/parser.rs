// use crate::nom::markdown::MarkdownInline;
// use crate::nom::markdown::MarkdownText;
use crate::ast::{Emphasis, Node, Text, Strong, Inline, Block, Code};

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

fn parse_bold(i: &str) -> IResult<&str, &str> {
    delimited(tag("**"), is_not("**"), tag("**"))(i)
}

fn parse_italics(i: &str) -> IResult<&str, &str> {
    delimited(tag("*"), is_not("*"), tag("*"))(i)
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

fn parse_markdown_inline(i: &str) -> IResult<&str, Inline> {
    alt((
        map(parse_italics, |s: &str| {
            Inline::Emphasis(Emphasis {
                children: vec![Node {
                    node_type: "text".to_string(),
                    value: Some(s.as_bytes().to_vec()),
                    position: None,
                }],
            })
        }),
        map(parse_bold, |s: &str| {
            Inline::Strong(Strong {
                children: vec![Node {
                    node_type: "text".to_string(),
                    value: Some(s.as_bytes().to_vec()),
                    position: None,
                }],
            })
        }),
        map(parse_plaintext, |s| {
            Inline::Text(Text {
                node_type: "text".to_string(),
                value: Some(s.as_bytes().to_vec()),
                position: None,
            })
        }),
    ))(i)
}

fn parse_markdown_text(i: &str) -> IResult<&str, Vec<Inline>> {
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
fn parse_header(i: &str) -> IResult<&str, (usize, Vec<Inline>)> {
    tuple((parse_header_tag, parse_markdown_text))(i)
}

fn parse_code_block(i: &str) -> IResult<&str, &str> {
    delimited(tag("```"), is_not("```"), tag("```"))(i)
}

pub fn parse_markdown(i: &str) -> IResult<&str, Vec<Block>> {
    many1(alt((
        map(parse_header, |e| {
            Block::Heading(Heading {
                depth: e.0,
                children: e.1,
                setext: false,
            })
        }),
        map(parse_code_block, |e| {
            Block::Code(Code {
                lang: None,
                meta: None,
                value: vec![]
            })
        }),
    )))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[test]
    fn emphasis() {
        let string = "*alpha*";
        assert_eq!(parse_italics(string), Ok(("", "alpha")));

        let md = parse_markdown_inline(string);

        let content = md.ok().unwrap().1;

        println!("{:?}", &content);

        let serialized = serde_json::to_string(&content).unwrap();
        println!("serialized = {}", serialized);

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
