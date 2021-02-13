// use crate::nom::markdown::MarkdownInline;
// use crate::nom::markdown::MarkdownText;
use crate::ast::{Emphasis, Node, Text, Strong, Inline, Block, Code, Paragraph};

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
    Finish, error::Error,
};
use nom::bytes::complete::{take_while, take_until};
use nom::character::complete::{multispace0, space1, multispace1, newline, alphanumeric0, line_ending, not_line_ending, alphanumeric1, space0, tab, alpha1, digit1};

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

#[inline(always)]
fn is_plain_text(chr: char) -> bool {
    chr != '*' && chr != '`' && chr != '[' && chr != '\n'
}

fn parse_text(i: &str) -> IResult<&str, String> {
    map(
        many1(take_while(is_plain_text)),
        |vec| vec.join(""),
    )(i)
}

fn parse_markdown_inline(i: &str) -> IResult<&str, Inline> {
    alt((
        map(parse_italics, |s: &str| {
            Inline::Emphasis(Emphasis {
                children: vec![Node {
                    node_type: "text".to_string(),
                    value: Some(s.to_string()),
                    position: None,
                }],
            })
        }),
        map(parse_bold, |s: &str| {
            Inline::Strong(Strong {
                children: vec![Node {
                    node_type: "text".to_string(),
                    value: Some(s.to_string()),
                    position: None,
                }],
            })
        }),
        map(parse_plaintext, |s| {
            Inline::Text(Text {
                node_type: "text".to_string(),
                value: Some(s.to_string()),
                position: None,
            })
        }),
    ))(i)
}

fn parse_markdown_text(i: &str) -> IResult<&str, Vec<Inline>> {
    terminated(many0(parse_markdown_inline), multispace0)(i)
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

// An indented code block is composed of one or more indented chunks separated by blank lines.
// An indented chunk is a sequence of non-blank lines, each indented four or more spaces.
// The contents of the code block are the literal contents of the lines, including trailing
// line endings, minus four spaces of indentation.
// An indented code block has no info string.
fn  parse_indented_code_block(i: &str) -> IResult<&str, String> {
    map(
        many1(delimited(
            alt((tag("    "), tag("\t"))),
            alt((alpha1, digit1, space1)),
            line_ending
            )),
        |vec| vec.join("\n"),
    )(i)
}

// TODO: indented code block
// TODO: commonmark also supports ~ as the fenced block
fn parse_fenced_code_block(i: &str) -> IResult<&str, (Option<String>, Option<String>, String)> {
    let (remaining, code_block) = delimited(tag("```"), is_not("```"), tag("```"))(i)?;
    let (_, info): (&str, &str) = not_line_ending(code_block)?;

    let content = code_block.to_string();
    if !info.is_empty() {
        let mut split: Vec<&str> = info.splitn(2," ").collect();
        let metadata = if split.len() > 1 { Some(split[1].to_string()) } else { None };
        Ok((remaining, (Some(split[0].to_string()), metadata, content)))
    } else {
        Ok((remaining, (None, None, content)))
    }

    // delimited(tag("```"), is_not("```"), tag("```"))(i)
}

// fn parse_fenced_code_info(i: &str) -> IResult<&str, (&str, &str, &str)> {
//     // let info = take_until(multispace0);
//     let info = tuple((take_until(space1), preceded(space1, take_until(multispace0))))(i);
//     let content = take_until("```");
//     (info, content)
// }

// TODO: they can also be interrupted by lists without a second newline
// not sure alt with tag("\n- ") is the appropriate way to handle
fn parse_paragraph(i: &str) -> IResult<&str, Vec<Inline>> {
    // terminated(many0(parse_markdown_inline), tag("\n\n"))(i)
    // terminated(many0(parse_markdown_inline), newline)(i)
    let result: IResult<&str, Vec<Inline>> = terminated(
        many0(parse_markdown_inline),
        alt((tag("\n\n"), tag("\n- ")))
    )(i);
    match result {
        Ok((input, para)) => Ok((input, para)),
        Err(e) => {
            if i == "" {
                Err(e)
            } else {
                Ok(("", vec![Inline::Text(Text {
                    node_type: "text".to_string(),
                    value: Some(i.to_string()),
                    position: None,
                })]))
            }
        }
    }
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
        map(parse_indented_code_block, |e| {
            Block::Code(Code {
                lang: None,
                meta: None,
                value: e.to_string()
            })
        }),
        map(parse_fenced_code_block, |e| {
            Block::Code(Code {
                lang: e.0, //None, //Some(e.0.to_owned()),
                meta: e.1, //None, //Some(e.1.to_owned()),
                value: e.2 //e.as_bytes().to_vec()
            })
        }),
        map(parse_paragraph, |e| {
            Block::Paragraph(Paragraph {
                children: e
            })
        })
    )))(i)
}

type Err = Error<String>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::error::Error;

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
        let string = "**alpha** ";
        assert_eq!(parse_bold(string), Ok((" ", "alpha")));

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
        let string = "# Header";
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

        // let md = parse_header(string);
        // let z = match md.finish() {
        //     Ok((_remaining, name)) => {
        //         println!("remaining [{}] name[{:?}]", _remaining, name);
        //     },
        //     Err(_) => {
        //         println!("an error occurred");
        //     }
        // };

        let md = parse_markdown(string);
        assert!(md.is_ok());
        let content = md.ok().unwrap().1;

        let serialized = serde_json::to_string(&content).unwrap();
        println!("serialized = {}", serialized);
    }

    #[test]
    fn header_italicized() {
        let string = "# *Hello* World";

        let md = parse_markdown(string);
        assert!(md.is_ok());
        let content = md.ok().unwrap().1;

        let serialized = serde_json::to_string(&content).unwrap();
        println!("serialized = {}", serialized);
    }

    #[test]
    fn indented_code_block() {
        let string = "    ls\n     foo\n";

        let md = parse_markdown(string);

        // let md = parse_markdown(string);
        assert!(md.is_ok());
        let content = md.ok().unwrap().1;

        let serialized = serde_json::to_string(&content).unwrap();
        println!("serialized = {}", serialized);
    }

    #[test]
    fn indented_tab_code_block() {
        let string = "\tls\n\tfoo\n";

        let md = parse_markdown(string);

        // let md = parse_markdown(string);
        assert!(md.is_ok());
        let content = md.ok().unwrap().1;

        let serialized = serde_json::to_string(&content).unwrap();
        println!("serialized = {}", serialized);
    }

    #[test]
    fn fenced_code_block() {
        let string = "```\nls\n```";

        let md = parse_markdown(string);

        // let md = parse_markdown(string);
        assert!(md.is_ok());
        let content = md.ok().unwrap().1;

        let serialized = serde_json::to_string(&content).unwrap();
        println!("serialized = {}", serialized);
    }

    #[test]
    fn fenced_code_block_empty() {
        let string = "```\n```";

        let md = parse_markdown(string);

        // let md = parse_markdown(string);
        assert!(md.is_ok());
        let content = md.ok().unwrap().1;

        let serialized = serde_json::to_string(&content).unwrap();
        println!("serialized = {}", serialized);
    }

    // TODO: this is wrong. for some reason there is a paragraph in the output
    // content in code block should be considered literal
    // from spec: The contents of a code block are literal text, and do not get parsed as Markdown:
    // TODO: The closing code fence must use the same character as the opening fence:
//     ```
//     aaa
//     ~~~
//     ```
    // TODO: Closing fences may be indented by 0-3 spaces, and their indentation need not match that of the opening fence:
    #[test]
    fn fenced_code_block_empty_content() {
        let string = "```\n\n```";

        let md = parse_markdown(string);

        // let md = parse_markdown(string);
        assert!(md.is_ok());
        let content = md.ok().unwrap().1;

        let serialized = serde_json::to_string(&content).unwrap();
        println!("serialized = {}", serialized);
    }

    #[test]
    fn fenced_code_block_literal_content() {
        let string = "```\n*hi*\n```";

        let md = parse_markdown(string);

        // let md = parse_markdown(string);
        assert!(md.is_ok());
        let content = md.ok().unwrap().1;

        let serialized = serde_json::to_string(&content).unwrap();
        println!("serialized = {}", serialized);
    }

    #[test]
    fn fenced_code_block_with_info() {
        let string = "```shell some metadata\nls\n```";

        let md = parse_markdown(string);

        // let md = parse_markdown(string);
        assert!(md.is_ok());
        let content = md.ok().unwrap().1;

        let serialized = serde_json::to_string(&content).unwrap();
        println!("serialized = {}", serialized);
    }

    #[test]
    fn paragraph() {
        let string = "Hello world";

        let md = parse_paragraph(string);

        // let md = parse_markdown(string);
        assert!(md.is_ok());
        let content = md.ok().unwrap().1;

        let serialized = serde_json::to_string(&content).unwrap();
        println!("serialized = {}", serialized);
    }

    #[test]
    fn multiline_paragraph() {
        let string = "Hello.\nWorld.";

        let md = parse_markdown(string);
        assert!(md.is_ok());
        let content = md.ok().unwrap().1;

        let serialized = serde_json::to_string(&content).unwrap();
        println!("serialized = {}", serialized);
    }

    #[test]
    fn paragraph_terminated_by_list() {
        let string = "Hello.\n- list item";

        let md = parse_markdown(string);
        assert!(md.is_ok());
        let content = md.ok().unwrap().1;

        let serialized = serde_json::to_string(&content).unwrap();
        println!("serialized = {}", serialized);
    }

}
