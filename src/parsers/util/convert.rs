// TODO: add vimwiki accreditation
use crate::ast::location::{Locatable, Position};
use crate::parsers::{Captured, IResult, Span};
use nom::error::context;
use std::borrow::Cow;
use std::path::Path;

/// Parser that transforms a `Captured<T>` to a `Located<T>`, which involves
/// calculating the line and column information; so, this is expensive!
pub fn locate<'a, T>(
    parser: impl FnMut(Span<'a>) -> IResult<Captured<T>>,
) -> impl FnMut(Span<'a>) -> IResult<Locatable<T>> {
    context("Locate", move |input: Span| {
        let (input, c) = parser(input)?;
        let position = Position::from_span(input);

        Ok((input, Locatable::new(c.into_inner(), position)))
    })
}

/// Parser that captures the input used to create the output of provided the parser
pub fn capture<'a, T>(
    parser: impl FnMut(Span<'a>) -> IResult<T>,
) -> impl FnMut(Span<'a>) -> IResult<Captured<T>> {
    context("Capture", move |input: Span| {
        let start = input;
        let (input, x) = parser(input)?;
        let start = start.with_length(input.start_offset() - start.start_offset());

        Ok((input, Captured::new(x, start)))
    })
}

/// Parser that transforms the input to that of `Cow<'a, str>`
/// where the lifetime is bound to the resulting `Span<'a>`
pub fn cow_str<'a>(input: Span<'a>) -> IResult<Cow<'a, str>> {
    Ok((input, input.into()))
}

pub fn span_as_string(input: Span) -> IResult<String> {
    Ok((input, input.into()))
}

/// Parser that transforms the input to that of `Cow<'a, Path>`
/// where the lifetime is bound to the resulting `Span<'a>`
pub fn cow_path<'a>(input: Span<'a>) -> IResult<Cow<'a, Path>> {
    Ok((input, input.into()))
}



// /// Parser that transforms the result of one parser to that of `Cow<'a, str>`
// /// where the lifetime is bound to the resulting `Span<'a>`
// pub fn cow_str<'a>(
//     parser: impl Fn(Span<'a>) -> IResult<Span<'a>>,
// ) -> impl Fn(Span<'a>) -> IResult<Cow<'a, str>> {
//     context("Cow Str", map(parser, |s: Span<'a>| s.into()))
// }
//
// /// Parser that transforms the result of one parser to that of `Cow<'a, Path>`
// /// where the lifetime is bound to the resulting `Span<'a>`
// pub fn cow_path<'a>(
//     parser: impl Fn(Span<'a>) -> IResult<Span<'a>>,
// ) -> impl Fn(Span<'a>) -> IResult<Cow<'a, Path>> {
//     context("Cow Path", map(parser, |s: Span<'a>| s.into()))
// }

#[cfg(test)]
mod tests {
    use super::*;
    use nom::bytes::complete::tag;
    use nom::combinator::map_res;
    use std::path::PathBuf;

    #[test]
    fn locate_should_return_parser_result_with_consumed_input_location() {
        let input = Span::from("123abc");
        let (input, located) = locate(capture(map_res(tag("123"), |s: Span| {
            s.as_unsafe_remaining_str().parse::<u32>()
        })))(input)
        .unwrap();
        assert_eq!(input, "abc");
        assert_eq!(located.region().offset(), 0);
        assert_eq!(located.region().len(), 3);
        assert_eq!(located.into_inner(), 123);
    }

    #[test]
    fn capture_should_return_parser_result_with_consumed_input() {
        let input = Span::from("123abc");
        let (input, captured) = capture(map_res(tag("123"), |s: Span| {
            s.as_unsafe_remaining_str().parse::<u32>()
        }))(input)
        .unwrap();
        assert_eq!(input, "abc");
        assert_eq!(captured.input(), "123");
        assert_eq!(captured.into_inner(), 123);
    }

    #[test]
    fn cow_str_should_return_input_as_cow_str() {
        let input = Span::from("abc");
        let (input, result) = cow_str(input).unwrap();
        assert_eq!(input, "abc");
        assert_eq!(result, Cow::from("abc"));
    }

    #[test]
    fn cow_path_should_return_input_as_cow_path() {
        let input = Span::from("abc");
        let (input, result) = cow_path(input).unwrap();
        assert_eq!(input, "abc");
        assert_eq!(result, Cow::from(PathBuf::from("abc")));
    }
}
