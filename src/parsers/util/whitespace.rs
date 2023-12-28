// TODO: add vimwiki accreditation
use crate::parsers::{IResult, Span};
use nom::character::complete::space0;
use nom::combinator::rest_len;
use nom::error::context;
use nom::AsBytes;

/// Counts the spaces & tabs that are trailing in our input
pub fn count_trailing_whitespace(input: Span) -> IResult<usize> {
    fn inner(input: Span) -> IResult<usize> {
        let mut cnt = 0;

        // Count whitespace in reverse so we know how many are trailing
        for b in input.as_bytes().iter().rev() {
            if !nom::character::is_space(*b) {
                break;
            }
            cnt += 1;
        }

        Ok((input, cnt))
    }

    context("Count Trailing Whitespace", inner)(input)
}

/// Trims the trailing whitespace from input, essentially working backwards
/// to cut off part of the input
pub fn trim_trailing_whitespace(input: Span) -> IResult<()> {
    fn inner(input: Span) -> IResult<()> {
        use nom::Slice;
        let (input, len) = rest_len(input)?;
        let (input, cnt) = count_trailing_whitespace(input)?;
        Ok((input.slice(..(len - cnt)), ()))
    }

    context("Trim Trailing Whitespace", inner)(input)
}

/// Trims the leading and trailing whitespace from input
pub fn trim_whitespace(input: Span) -> IResult<()> {
    fn inner(input: Span) -> IResult<()> {
        let (input, _) = space0(input)?;
        let (input, _) = trim_trailing_whitespace(input)?;
        Ok((input, ()))
    }

    context("Trim Whitespace", inner)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_trailing_whitespace_should_return_zero_if_no_spaces_or_tabs_at_end() {
        let input = Span::from("abc");
        let (input, cnt) = count_trailing_whitespace(input).unwrap();
        assert_eq!(input.as_unsafe_remaining_str(), "abc");
        assert_eq!(cnt, 0);
    }

    #[test]
    fn count_trailing_whitespace_should_return_total_spaces_at_end() {
        let input = Span::from("abc   ");
        let (input, cnt) = count_trailing_whitespace(input).unwrap();
        assert_eq!(input.as_unsafe_remaining_str(), "abc   ");
        assert_eq!(cnt, 3);
    }

    #[test]
    fn count_trailing_whitespace_should_return_total_tabs_at_end() {
        let input = Span::from("abc\t\t\t");
        let (input, cnt) = count_trailing_whitespace(input).unwrap();
        assert_eq!(input.as_unsafe_remaining_str(), "abc\t\t\t");
        assert_eq!(cnt, 3);
    }

    #[test]
    fn count_trailing_whitespace_should_return_total_spaces_and_tabs_at_end() {
        let input = Span::from("abc \t ");
        let (input, cnt) = count_trailing_whitespace(input).unwrap();
        assert_eq!(input.as_unsafe_remaining_str(), "abc \t ");
        assert_eq!(cnt, 3);
    }

    #[test]
    fn trim_trailing_whitespace_should_return_input_if_no_trailing_whitespace() {
        let input = Span::from(" abc");
        let (input, _) = trim_trailing_whitespace(input).unwrap();
        assert_eq!(input.as_unsafe_remaining_str(), " abc");
    }

    #[test]
    fn trim_trailing_whitespace_should_return_input_with_trailing_whitespace_removed() {
        let input = Span::from(" abc \t ");
        let (input, _) = trim_trailing_whitespace(input).unwrap();
        assert_eq!(input.as_unsafe_remaining_str(), " abc");
    }

    #[test]
    fn trim_whitespace_should_return_input_with_leading_whitespace_removed() {
        let input = Span::from("\t \tabc");
        let (input, _) = trim_whitespace(input).unwrap();
        assert_eq!(input.as_unsafe_remaining_str(), "abc");
    }

    #[test]
    fn trim_whitespace_should_return_input_with_trailing_whitespace_removed() {
        let input = Span::from("abc\t \t");
        let (input, _) = trim_whitespace(input).unwrap();
        assert_eq!(input.as_unsafe_remaining_str(), "abc");
    }

    #[test]
    fn trim_whitespace_should_return_input_with_leading_and_trailing_whitespace_removed() {
        let input = Span::from("\t \tabc \t ");
        let (input, _) = trim_whitespace(input).unwrap();
        assert_eq!(input.as_unsafe_remaining_str(), "abc");
    }
}
