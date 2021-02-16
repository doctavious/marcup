// use crate::nom::markdown::MarkdownInline;
// use crate::nom::markdown::MarkdownText;
use crate::ast::{Emphasis, Node, Text, Strong, Inline, Block, Code, Paragraph, BlockQuote, Root};

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
    Finish, error::Error
};
use nom::bytes::complete::{take_while, take_until, take_till};
use nom::character::complete::{multispace0, space1, multispace1, newline, alphanumeric0, line_ending, not_line_ending, alphanumeric1, space0, tab, alpha1, digit1};

fn parse_bold(i: &str) -> IResult<&str, &str> {
    delimited(tag("**"), is_not("**"), tag("**"))(i)
}

fn parse_italics(i: &str) -> IResult<&str, &str> {
    let r = delimited(tag("*"), is_not("*"), tag("*"))(i);
    match r {
        Ok((a, b)) => {
            println!("found italics. [{}], [{}]", a, b);
            Ok((a,b))
        }
        Err(e) => Err(e)
    }

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
            println!("wth...[{}]", s);
            Inline::Emphasis(Emphasis {
                children: vec![Node {
                    node_type: "text".to_string(),
                    value: Some(s.to_string()),
                    position: None,
                }],
                position: None,
            })
        }),
        map(parse_bold, |s: &str| {
            Inline::Strong(Strong {
                children: vec![Node {
                    node_type: "text".to_string(),
                    value: Some(s.to_string()),
                    position: None,
                }],
                position: None,
            })
        }),
        map(parse_plaintext, |s| {
            Inline::Text(Text {
                //node_type: "text".to_string(),
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

fn indent(i: &str) -> nom::IResult<&str, &str> {
    alt((tag("    "), tag("\t")))(i)
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
            not_line_ending,
            line_ending
            )),
        |vec| vec.join("\n"),
    )(i)
}

// TODO: indented code block
// TODO: at least three consecutive backtick characters (`) or tildes (~). (Tildes and backticks cannot be mixed.)
// TODO: The closing code fence must be at least as long as the opening fence
// TODO: commonmark also supports ~ as the fenced block
fn parse_fenced_code_block(i: &str) -> IResult<&str, (Option<String>, Option<String>, String)> {
    let (remaining, block) = alt((delimited(
        tag("```"),
        is_not("```"),
        tag("```")
            ),
                                 delimited(
                                     tag("~~~"),
                                     is_not("~~~"),
                                     tag("~~~")))
    )(i)?;

    let (code, info) = terminated(not_line_ending, line_ending)(block)?;

    let mut lang: Option<String> = None;
    let mut metadata: Option<String> = None;
    if !info.is_empty() {
        let mut split: Vec<&str> = info.splitn(2," ").collect();
        metadata = if split.len() > 1 { Some(split[1].to_string()) } else { None };
        lang = Some(split[0].to_string());
    }

    // I dislike this but works for not.
    // eat trailing new line in code if it exists
    let code_block = if code.is_empty() {
        String::from("")
    } else {
        let (_, code) = terminated(not_line_ending, line_ending)(code)?;
        code.to_string()
    };

    Ok((remaining, (lang, metadata, code_block)))
}

// TODO: This is very wrong
// TODO: this needs to handle hanging lines (with >) which are paragraph continuations
fn parse_blockquote(i: &str) -> IResult<&str, Vec<Block>> {
    preceded(tag(">"), parse_markdown)(i)
    // map(
    //     many1(delimited(
    //         tag(">"),
    //         not_line_ending,
    //         line_ending
    //     )),
    //     parse_markdown_text,
    // )(i)
}

// TODO: they can also be interrupted by lists without a second newline
// not sure alt with tag("\n- ") is the appropriate way to handle
fn parse_paragraph(i: &str) -> IResult<&str, Vec<Inline>> {
    terminated(many0(parse_markdown_inline), tag("\n\n"))(i)
    // let result = terminated(many0(parse_markdown_inline), multispace0)(i);
    // // let result = terminated(many0(parse_markdown_inline), newline)(i);
    // // let result = parse_markdown_text(i);
    // // let result: IResult<&str, Vec<Inline>> = terminated(
    // //     many0(parse_markdown_inline),
    // //     alt((newline, tag("\n- ")))
    // // )(i);
    // match result {
    //     Ok((input, para)) => {
    //         println!("parse_paragraph input [{}], paragraph [{:?}]", input, para);
    //         Ok((input, para))
    //     },
    //     Err(e) => {
    //         println!("parse_paragraph...we got an error. i [{}]", i);
    //         if i == "" {
    //             Err(e)
    //         } else {
    //             Ok(("", vec![Inline::Text(Text {
    //                 value: Some(i.to_string()),
    //                 position: None,
    //             })]))
    //         }
    //     }
    // }
}


pub fn parse_markdown(i: &str) -> IResult<&str, Vec<Block>> {
    many1(alt((
        map(parse_header, |e| {
            Block::Heading(Heading {
                depth: e.0,
                children: e.1,
                setext: false,
                position: None,
            })
        }),
        map(parse_indented_code_block, |e| {
            Block::Code(Code {
                lang: None,
                meta: None,
                value: e.to_string(),
                position: None
            })
        }),
        map(parse_fenced_code_block, |e| {
            Block::Code(Code {
                lang: e.0,
                meta: e.1,
                value: e.2,
                position: None,
            })
        }),
        map(parse_blockquote, |e| {
           Block:: BlockQuote(BlockQuote {
               children: e,
               position: None
           })
        }),
        map(parse_paragraph, |e| {
            Block::Paragraph(Paragraph {
                children: e,
                position: None,
            })
        })
    )))(i)
}

pub fn parse(i: &str) -> IResult<&str, Root> {
    let ast = parse_markdown(i)?;
    Ok((ast.0, Root {
        children: ast.1,
        position: None
    }))
}

type Err = Error<String>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::error::Error;
    use insta::assert_json_snapshot;

    // #[test]
    // fn emphasis() {
    //     let string = "*alpha*";
    //     let md = parse(string);
    //
    //     let content = md.ok().unwrap().1;
    //
    //     println!("{:?}", &content);
    //
    //     let serialized = serde_json::to_string(&content).unwrap();
    //     println!("serialized = {}", serialized);
    //
    //     assert_json_snapshot!(content);
    // }
    //
    // #[test]
    // fn strong() {
    //     let string = "**alpha** ";
    //     assert_eq!(parse_bold(string), Ok((" ", "alpha")));
    //
    //     let md = parse(string);
    //
    //     let content = md.ok().unwrap().1;
    //
    //     println!("{:?}", &content);
    //
    //     let serialized = serde_json::to_string(&content).unwrap();
    //     println!("serialized = {}", serialized);
    //
    //     // assert_json_snapshot!(content);
    // }

    #[test]
    fn header() {
        let string = "# Header";

        let md = parse(string);

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    #[test]
    fn header_italicized() {
        let string = "# *Hello* World";

        let md = parse(string);

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    #[test]
    fn indented_code_block() {
        let string = "    ls\n    foo\n";

        let md = parse(string);

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    #[test]
    fn indented_code_block_with_indented_line() {
        let string = "    ls\n        foo\n";

        let md = parse(string);

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    // TODO: this is incorrect based on spec
    // trailing and preceding blank lines should not be included
    #[test]
    fn indented_code_block_trailing_and_preceding_blank_lines() {
        let string = "    \n    ls\n    \n";

        let md = parse(string);

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    // TODO: this is incorrect based on spec
    // interior blank lines, even when not fully indented, should be included
    #[test]
    fn indented_code_block_interior_blank_lines() {
        let string = "    ls\n \n  \n    hi";

        let md = parse(string);

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    #[test]
    fn indented_tab_code_block() {
        let string = "\tls\n\tfoo\n";

        let md = parse(string);

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    #[test]
    fn fenced_code_block() {
        let string = "```\nls\n```";

        let md = parse(string);

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    #[test]
    fn fenced_code_block_tilde() {
        let string = "~~~\nls\n~~~";

        let md = parse(string);

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    #[test]
    fn fenced_code_block_empty() {
        let string = "```\n```";

        let md = parse(string);

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
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

        let md = parse(string);

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    #[test]
    fn fenced_code_block_literal_content() {
        let string = "```\n*hi*\n```";

        let md = parse(string);

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    #[test]
    fn fenced_code_block_with_info() {
        let string = "```shell some metadata\nls\n```";

        let md = parse(string);

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    // #[test]
    // fn blockquote() {
    //     let string = ">this is a block quote.\n> and this too";
    //
    //     let md = parse_markdown(string);
    //
    //     // let md = parse_markdown(string);
    //     assert!(md.is_ok());
    //     let content = md.ok().unwrap().1;
    //
    //     let serialized = serde_json::to_string(&content).unwrap();
    //     println!("serialized = {}", serialized);
    // }
    //
    // #[test]
    // fn blockquote_with_hanging_line() {
    //     let string = ">this is a block quote.\nand this too\n\nhi";
    //
    //     let md = parse_markdown(string);
    //
    //     // let md = parse_markdown(string);
    //     assert!(md.is_ok());
    //     let content = md.ok().unwrap().1;
    //
    //     let serialized = serde_json::to_string(&content).unwrap();
    //     println!("serialized = {}", serialized);
    // }

    // #[test]
    // fn paragraph() {
    //     let string = "Hello world";
    //
    //     let md = parse(string);
    //     assert!(md.is_ok());
    //     let content = md.ok().unwrap().1;
    //
    //     let serialized = serde_json::to_string(&content).unwrap();
    //     println!("serialized = {}", serialized);
    //
    //     assert_json_snapshot!(content);
    // }

    // #[test]
    // fn multiline_paragraph() {
    //     let string = "Hello.\nWorld.";
    //
    //     let md = parse_markdown(string);
    //     assert!(md.is_ok());
    //     let content = md.ok().unwrap().1;
    //
    //     let serialized = serde_json::to_string(&content).unwrap();
    //     println!("serialized = {}", serialized);
    // }

    // #[test]
    // fn paragraph_terminated_by_list() {
    //     let string = "Hello.\n- list item";
    //
    //     let md = parse_markdown(string);
    //     assert!(md.is_ok());
    //     let content = md.ok().unwrap().1;
    //
    //     let serialized = serde_json::to_string(&content).unwrap();
    //     println!("serialized = {}", serialized);
    // }

}
