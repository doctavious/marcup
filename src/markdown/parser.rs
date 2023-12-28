// use crate::nom::markdown::MarkdownInline;
// use crate::nom::markdown::MarkdownText;
use crate::ast::location::{Locatable, Position};
use crate::ast::Heading;
use crate::ast::{
    Block, BlockQuote, Code, Emphasis, Inline, InlineElementContainer, Paragraph, Root, Strong,
    Text,
};
use crate::parsers::util::{beginning_of_line, blank_line, capture, cow_str, end_of_line_or_input, locate, take_end, take_line_while1, take_until_end_of_line_or_input, trim_trailing_whitespace, trim_whitespace, span_as_string, not_contains, surround_in_line1};
use crate::parsers::{IResult, Span};
use nom::character::complete::{char, line_ending, not_line_ending, space0, newline};
use nom::combinator::{eof, map_parser, peek, value, verify, recognize};
use nom::error::context;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take, take_while1},
    combinator::{map, not},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated, tuple},
};
use nom::bytes::complete::take_till1;

pub fn paragraph(input: Span) -> IResult<Paragraph> {
    // Ensure that we are starting at the beginning of a line
    let (input, _) = beginning_of_line(input)?;

    // Continuously take content until we encounter another type of element
    let (input, elements) = many1(delimited(
        continue_paragraph,
        paragraph_line,
        end_of_line_or_input,
    ))(input)?;

    // Transform contents into the paragraph itself
    let paragraph = Paragraph::new(From::from(elements));

    Ok((input, paragraph))
}

fn paragraph_line(input: Span) -> IResult<InlineElementContainer> {
    let (input, _) = space0(input)?;
    map(
        inline_element_container,
        |l: Locatable<InlineElementContainer>| l.into_inner(),
    )(input)
}

/// Parses one or more inline elements and wraps it in a container; note
/// that this does NOT consume a line termination
#[inline]
pub fn inline_element_container(input: Span) -> IResult<Locatable<InlineElementContainer>> {
    context(
        "Inline Element Container",
        locate(capture(map(
            many1(parse_markdown_inline), //many1(inline_element),
            InlineElementContainer::from,
        ))),
    )(input)
}

// TODO: Optimize by adjusting paragraph parser to be a tuple that
//       includes an Option<BlockElement> so that we don't waste
//       the processing spent
fn continue_paragraph(input: Span) -> IResult<()> {
    let (input, _) = not(header)(input)?;
    // let (input, _) = not(definition_list)(input)?;
    // let (input, _) = not(list)(input)?;
    // let (input, _) = not(table)(input)?;
    // let (input, _) = not(preformatted_text)(input)?;
    // let (input, _) = not(math_block)(input)?;
    let (input, _) = not(blank_line)(input)?;
    // let (input, _) = not(blockquote)(input)?;
    // let (input, _) = not(divider)(input)?;
    // let (input, _) = not(placeholder)(input)?;
    Ok((input, ()))
}

fn parse_bold(i: Span) -> IResult<Inline> {
    let (r, content) = delimited(tag("**"), is_not("**"), tag("**"))(i)?;
    Ok((
        r,
        Inline::Strong(Strong {
            children: vec![Inline::Text(Text {
                value: content.to_string(),
                // position: None
            })],
            // position: None,
        }),
    ))
}

// A delimiter run is either a sequence of one or more * characters that is not preceded or
// followed by a non-backslash-escaped * character, or a sequence of one or more _ characters
// that is not preceded or followed by a non-backslash-escaped _ character.

// A left-flanking delimiter run is a delimiter run that is
// (1) not followed by Unicode whitespace, and either
// (2a) not followed by a punctuation character, or
// (2b) followed by a punctuation character and preceded by Unicode whitespace or a punctuation character.
// For purposes of this definition, the beginning and the end of the line count as Unicode whitespace.
fn left_flanking_delimiter_run(i: &str) {}

// A right-flanking delimiter run is a delimiter run that is
// (1) not preceded by Unicode whitespace, and either
// (2a) not preceded by a punctuation character, or
// (2b) preceded by a punctuation character and followed by Unicode whitespace or a punctuation character.
// For purposes of this definition, the beginning and the end of the line count as Unicode whitespace.
fn right_flanking_delimiter_run(i: &str) {}

fn parse_italics(i: Span) -> IResult<Inline> {
    map(delimited(tag("*"), is_not("*"), tag("*")), |s: Span| {
        Inline::Emphasis(Emphasis {
            children: vec![Inline::Text(Text {
                value: std::str::from_utf8(s.inner).unwrap().to_string(),
                // position: None
            })],
            // position: None,
        })
    })(i)
}

// fn italics(i: Span) -> IResult<String> {
//     let (i, _) = tag("*")(i)?;
//     let (i, _) = not(tag(" "))(i)?;
//     map(terminated(many0(is_not("*")), tag("*")), |vec| vec.join(""))(i)
// }
// TODO: this is wrong for markdown
fn italics(i: Span) -> IResult<Inline> {
    context(
        "Italic Decorated Text",
        map(
            map_parser(
                not_contains("%%", surround_in_line1("*", "*")),
                decorated_text_contents,
            ),
            Inline::Emphasis,
        ),
    )(i)
}

fn decorated_text_contents(
    input: Span,
) -> IResult<Vec<Locatable<Inline>>> {
    fn inner(input: Span) -> IResult<Vec<Locatable<Inline>>> {
        many1(alt((
            // map(link, |l: Located<Link>| l.map(DecoratedTextContent::from)),
            // map(keyword, |l: Located<Keyword>| {
            //     l.map(DecoratedTextContent::from)
            // }),
            map(parse_markdown_inline, |l: Locatable<Inline>| {
                l.map(Inline::from)
            }),
            map(plaintext, |l: Locatable<Text>| l.map(Inline::from)),
        )))(input)
    }

    context("Decorated Text Contents", inner)(input)
}

// pub fn decorated_text(input: Span) -> IResult<Locatable<Inline>> {
//     context(
//         "Decorated Text",
//         locate(capture(alt((
//             bold_text,
//             italic_text,
//             strikeout_text,
//             superscript_text,
//             subscript_text,
//         )))),
//     )(input)
// }

// fn parse_italics3(i: &str) -> IResult<&str, &str> {
//     tuple!(tag!("*"), alphanumeric1!, alt!(space1!, alphanumeric1!), tag!("*"))
// }

named!(parse_emphasis<&str, &str>,
    do_parse!(
        tag!("*") >>
        text: take_until!("*") >>
        (text)
      )
);

// we want to match many things that are not any of our special tags
// but since we have no tools available to match and consume in the negative case (without regex)
// we need to match against our tags, then consume one char
// we repeat this until we run into one of our special characters
// then we join our array of characters into a String
// fn parse_plaintext(i: Span) -> IResult<String> {
//     map(
//         many1(preceded(
//             not(alt((tag("*"), tag("`"), tag("["), tag("!["), tag("\n")))),
//             take(1u8),
//         )),
//         |vec| vec.join(""),
//     )(i)
// }

fn plaintext(i: Span) -> IResult<Locatable<Text>> {
    // Uses combination of short-circuiting and full checks to ensure we
    // can continue consuming text
    fn is_text(input: Span) -> IResult<()> {
        let (input, _) = not(newline)(input)?;
        // let (input, _) = not(comment)(input)?;
        // let (input, _) = not(code_inline)(input)?;
        // let (input, _) = not(math_inline)(input)?;
        // let (input, _) = not(tags)(input)?;
        // let (input, _) = not(link)(input)?;
        // let (input, _) = not(decorated_text)(input)?;
        let (input, _) = not(parse_italics)(input)?;
        let (input, _) = not(parse_bold)(input)?;
        // let (input, _) = not(keyword)(input)?;
        Ok((input, ()))
    }

    /// Checks for a byte that is the start of anything inline that would not
    /// be regular text
    #[inline]
    fn start_of_non_text(b: u8) -> bool {
        b == b'\n'
            || b == b'%'
            || b == b'`'
            || b == b'$'
            || b == b':'
            || b == b'['
            || b == b'*'
            || b == b'_'
            || b == b'~'
            || b == b'^'
            || b == b','
            || b == b'D'
            || b == b'F'
            || b == b'S'
            || b == b'T'
            || b == b'X'
    }

    fn text_line(input: Span) -> IResult<Span> {
        recognize(many1(alt((
            take_till1(start_of_non_text),
            preceded(is_text, take(1usize)),
        ))))(input)
    }

    context(
        "Text",
        locate(capture(map(map_parser(text_line, span_as_string), Text::new))),
    )(i)
}

fn parse_markdown_inline(i: Span) -> IResult<Locatable<Inline>> {
    locate(capture(alt((
        parse_italics,
        parse_bold,
        map(plaintext, |l: Locatable<Text>| l.map(Inline::from)),
        // map(parse_plaintext, |s| {
        //     println!("returning from parse_plaintext...[{}]", s);
        //     return Inline::Text(Text {
        //         value: s.to_string(),
        //         // position: None,
        //     });
        // }),
    ))))(i)
}

fn parse_markdown_text(i: Span) -> IResult<Vec<Locatable<Inline>>> {
    terminated(many0(parse_markdown_inline), alt((line_ending, eof)))(i)
}

// this guy matches the literal character #
fn parse_header_tag(i: Span) -> IResult<usize> {
    map(
        terminated(take_while1(|c| char::from(c) == '#'), tag(" ")),
        |s: Span| s.inner.len(),
    )(i)
}

// this combines a tuple of the header tag and the rest of the line
fn parse_header(i: Span) -> IResult<(usize, Vec<Locatable<Inline>>)> {
    println!("parse_header with [{}]", i);
    tuple((parse_header_tag, parse_markdown_text))(i)
}

/// Parses a vimwiki header, returning the associated header if successful
#[inline]
pub fn header(input: Span) -> IResult<Locatable<Heading>> {
    fn inner(input: Span) -> IResult<Heading> {
        // Header must start at the beginning of a line
        let (input, _) = beginning_of_line(input)?;

        // Second, determine the potential level of the header (the number of =)
        let (input, level) = verify(
            map(take_line_while1(char('#')), |s: Span| s.remaining_len()),
            |level| *level >= Heading::MIN_LEVEL && *level <= Heading::MAX_LEVEL,
        )(input)?;

        // Third, get the content of the header by collecting all text until we
        // find a closing set of = matching our expected level
        let (input, header) = map(header_tail(level), |content| {
            Heading::new_with_children(level, false, content)
        })(input)?;

        // Fourth, consume the end of line/input to indicate header complete
        let (input, _) = end_of_line_or_input(input)?;

        Ok((input, header))
    }

    context("Header", locate(capture(inner)))(input)
}

fn header_tail(level: usize) -> impl Fn(Span) -> IResult<InlineElementContainer> {
    use nom::{AsBytes, InputIter};
    move |input: Span| {
        // Get remainder of line and remove any excess whitespace
        let (input, rest_of_line) = take_until_end_of_line_or_input(input)?;
        let (rest_of_line, _) = trim_trailing_whitespace(rest_of_line)?;

        // Verify that the end of the line (minus whitespace) has the same
        // number of equals signs, and chop them off
        let (rest_of_line, _) = context(
            "Header Tail Equal Levels",
            verify(take_end(level), |end| {
                end.iter_elements().all(|b| b == b'=')
            }),
        )(rest_of_line)?;

        // Verify that there is no equals sign at the beginning or end of the
        // header content, which would imply that we have unbalanced levels
        let (rest_of_line, _) = peek(verify(take(1usize), |start: &Span| {
            start.as_bytes()[0] != b'='
        }))(rest_of_line)?;
        let (rest_of_line, _) = peek(verify(take_end(1usize), |end: &Span| {
            end.as_bytes()[0] != b'='
        }))(rest_of_line)?;

        // Remove leading and trailing whitespace within header content
        let (rest_of_line, _) = trim_whitespace(rest_of_line)?;

        // Parse our container of inline elements
        let (_, container) = map(
            inline_element_container,
            |l: Locatable<InlineElementContainer>| l.into_inner(),
        )(rest_of_line)?;

        Ok((input, container))
    }
}

fn indent(i: Span) -> IResult<Span> {
    alt((tag("    "), tag("\t")))(i)
}

// An indented code block is composed of one or more indented chunks separated by blank lines.
// An indented chunk is a sequence of non-blank lines, each indented four or more spaces.
// The contents of the code block are the literal contents of the lines, including trailing
// line endings, minus four spaces of indentation.
// An indented code block has no info string.
// fn parse_indented_code_block(i: Span) -> IResult<Locatable<Code>> {
//
//     fn inner(input: Span) -> IResult<Code> {
//         let (input, c) = map(
//             map_parser(
//                 many1(delimited(
//                     alt((tag("    "), tag("\t"))),
//                     not_line_ending,
//                     line_ending,
//                 )),
//                 cow_str,
//             ),
//             |vec| vec.join("\n"),
//         )(input)?;
//
//         Ok((input, Code {
//             lang: None,
//             meta: None,
//             value: c,
//             // position: None
//         }))
//     }
//
//     context("Indented Code Block", locate(capture(inner)))(i)
// }

// // TODO: indented code block
// // TODO: at least three consecutive backtick characters (`) or tildes (~). (Tildes and backticks cannot be mixed.)
// // TODO: The closing code fence must be at least as long as the opening fence
// fn parse_fenced_code_block(i: Span) -> IResult<(Option<String>, Option<String>, String)> {
//     let (remaining, block) = alt((
//         delimited(tag("```"), is_not("```"), tag("```")),
//         delimited(tag("~~~"), is_not("~~~"), tag("~~~")),
//     ))(i)?;
//
//     let (code, info): (&str, &str) = terminated(not_line_ending, line_ending)(block)?;
//
//     let mut lang: Option<String> = None;
//     let mut metadata: Option<String> = None;
//     if !info.is_empty() {
//         let mut split: Vec<&str> = info.splitn(2, " ").collect();
//         metadata = if split.len() > 1 {
//             Some(split[1].to_string())
//         } else {
//             None
//         };
//         lang = Some(split[0].to_string());
//     }
//
//     // I dislike this but works for not.
//     // eat trailing new line in code if it exists
//     let code_block = if code.is_empty() {
//         String::from("")
//     } else {
//         let (_, code) = terminated(not_line_ending, line_ending)(code)?;
//         code.to_string()
//     };
//
//     println!("parse_fenced_code_block remaining [{}]", remaining);
//     Ok((remaining, (lang, metadata, code_block)))
// }

// TODO: This is very wrong
// TODO: this needs to handle hanging lines (with >) which are paragraph continuations
// TODO: this needs to support markdown (e.g. italics) within the quote
fn parse_blockquote(i: Span) -> IResult<Locatable<BlockQuote>> {
    // preceded(tag(">"), parse_markdown)(i)
    fn inner(input: Span) -> IResult<BlockQuote> {
        let (input, lines) = alt((
            // NOTE: Indented blockquotes do not allow blank lines
            many1(blockquote_line_1),
            // NOTE: > blockquotes allow blank lines in between
            map(
                pair(
                    many1(blockquote_line_2),
                    map(
                        many0(pair(
                            many0(value(String::from(""), blank_line)),
                            blockquote_line_2,
                        )),
                        |pairs| {
                            pairs
                                .into_iter()
                                .flat_map(|(mut blanks, bq)| {
                                    blanks.push(bq.to_string());
                                    blanks
                                })
                                .collect()
                        },
                    ),
                ),
                |(head, rest)| vec![head, rest].concat(),
            ),
        ))(input)?;
        //Ok((input, Blockquote::new(lines)))
        Ok((input, BlockQuote::new(lines)))
    }

    context("Blockquote", locate(capture(inner)))(i)
}

/// Parses a blockquote line that begins with four or more spaces
#[inline]
fn blockquote_line_1(input: Span) -> IResult<String> {
    let (input, _) = beginning_of_line(input)?;
    let (input, _) = verify(space0, |s: &Span| s.remaining_len() >= 4)(input)?;
    let (input, text) = map_parser(
        verify(not_line_ending, |s: &Span| !s.is_only_whitespace()),
        cow_str,
    )(input)?;
    let (input, _) = end_of_line_or_input(input)?;

    Ok((input, text.to_string()))
}

/// Parses a blockquote line that begins with >
#[inline]
fn blockquote_line_2(input: Span) -> IResult<String> {
    let (input, _) = beginning_of_line(input)?;
    let (input, _) = tag("> ")(input)?;
    let (input, text) = map_parser(not_line_ending, cow_str)(input)?;
    let (input, _) = end_of_line_or_input(input)?;

    Ok((input, text.to_string()))
}

// TODO: they can also be interrupted by lists without a second newline
// not sure alt with tag("\n- ") is the appropriate way to handle
fn parse_paragraph(i: Span) -> IResult<Locatable<Paragraph>> {
    println!("inside parse_paragraph for [{}]", i);
    // terminated(many0(parse_markdown_inline), tag("\n\n"))(i)
    // terminated(many0(parse_markdown_inline), tag("\n\n"))(i);
    // terminated(many0(parse_markdown_inline), alt((tag("\n"), tag("\n\n"), eof)))(i)
    // terminated(many0(parse_markdown_inline), tag("\n\n"))(i)
    // terminated(many0(parse_markdown_inline), tag("\n\n"))(i)
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

    fn inner(input: Span) -> IResult<Paragraph> {
        // Ensure that we are starting at the beginning of a line
        let (input, _) = beginning_of_line(input)?;

        // Continuously take content until we encounter another type of
        // element
        let (input, elements) = context(
            "Paragraph",
            many1(delimited(
                continue_paragraph,
                paragraph_line,
                end_of_line_or_input,
            )),
        )(input)?;

        // Transform contents into the paragraph itself
        let paragraph = Paragraph::new(From::from(elements));

        Ok((input, paragraph))
    }

    context("Paragraph", locate(capture(inner)))(i)
}

pub fn parse_markdown(i: Span) -> IResult<Vec<Locatable<Block>>> {
    many1(alt((
        map(header, |e| e.map(Block::from)),
        // map(parse_indented_code_block, |e| {
        //     Block::Code(Code {
        //         lang: None,
        //         meta: None,
        //         value: e.to_string(),
        //         // position: None
        //     })
        // }),
        // map(parse_fenced_code_block, |e| {
        //     Block::Code(Code {
        //         lang: e.0,
        //         meta: e.1,
        //         value: e.2,
        //         // position: None,
        //     })
        // }),
        map(parse_blockquote, |e| e.map(Block::from)),
        map(parse_paragraph, |e| e.map(Block::from)),
    )))(i)
}

pub fn parse(markup: &str) -> IResult<Locatable<Root>> {
    let ast = parse_markdown(Span::from(markup))?;
    println!("parse ast has remaining [{}]", &ast.0);
    Ok((
        ast.0,
        // Root {
        //     children: ast.1,
        //     // position: None
        // },
        Locatable::new(Root::new(ast.1), Position::from_span(ast.0)),
    ))
}

macro_rules! le_mapping {
    ($type:ty) => {
        impl<'a> From<Locatable<$type>> for Locatable<Block> {
            fn from(element: Locatable<$type>) -> Self {
                element.map(Block::from)
            }
        }
    };
}

le_mapping!(Heading);
le_mapping!(Paragraph);
le_mapping!(BlockQuote);



#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_json_snapshot;
    use serde::{Deserialize, Serialize};
    use std::error::Error;

    #[test]
    fn emphasis() {
        let string = "*alpha*";
        let md = parse_paragraph(Span {
            inner: string.as_bytes(),
            start: 0,
            end: 0,
        });

        let content = md.ok().unwrap();

        println!("{:?}", &content.1);
        println!("{:?}", &content.0);

        let serialized = serde_json::to_string(&content).unwrap();
        println!("serialized = {}", serialized);

        // assert_json_snapshot!(content);
    }

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
        let md = parse("# Header World");

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    #[test]
    fn header_italicized() {
        let md = parse("# *Hello* World");

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    #[test]
    fn indented_code_block() {
        let md = parse("    ls\n    foo\n");

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    #[test]
    fn indented_code_block_with_indented_line() {
        let md = parse("    ls\n        foo\n");

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    // TODO: this is incorrect based on spec
    // trailing and preceding blank lines should not be included
    #[test]
    fn indented_code_block_trailing_and_preceding_blank_lines() {
        let md = parse("    \n    ls\n    \n");

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    // TODO: this is incorrect based on spec
    // interior blank lines, even when not fully indented, should be included
    #[test]
    fn indented_code_block_interior_blank_lines() {
        let md = parse("    ls\n \n  \n    hi");

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    #[test]
    fn indented_tab_code_block() {
        let md = parse("\tls\n\tfoo\n");

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    #[test]
    fn fenced_code_block() {
        let md = parse("```\nls\n```");

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    #[test]
    fn fenced_code_block_tilde() {
        let md = parse("~~~\nls\n~~~");

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    #[test]
    fn fenced_code_block_empty() {
        let md = parse("```\n```");

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
        let md = parse("```\n\n```");

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    #[test]
    fn fenced_code_block_literal_content() {
        let md = parse("```\n*hi*\n```");

        assert!(md.is_ok());
        assert_json_snapshot!(md.ok().unwrap().1);
    }

    #[test]
    fn fenced_code_block_with_info() {
        let md = parse("```shell some metadata\nls\n```");

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

    #[test]
    fn multiline_paragraph() {
        let string = "Hello.\nWorld.";

        let md = parse_markdown(Span::from(string));
        assert!(md.is_ok());
        let content = md.ok().unwrap().1;

        let serialized = serde_json::to_string(&content).unwrap();
        println!("serialized = {}", serialized);
    }

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

    #[test]
    fn t() {
        let result = parse_emphasis("a *foo bar*").ok().unwrap();
        println!("remainder [{}] and value [{}]", result.0, result.1);
    }

    #[test]
    fn test_italics() {
        let result = italics(Span::from("*foo bar*"));
        assert_eq!(result, Ok(("", String::from("foo bar"))));

        let result = italics(Span::from("* foo bar*"));
        //assert_eq!(result, Err(nom::Err::Error(("* foo bar*", nom::error::ErrorKind::Tag))));
    }
}
