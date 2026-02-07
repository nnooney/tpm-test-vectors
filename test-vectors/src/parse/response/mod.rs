use crate::response::Part;
use crate::response::{EXPANSION_END, EXPANSION_START, SPACE, WILDCARD};

use const_format::concatcp;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, one_of};
use nom::combinator::{all_consuming, map, recognize};
use nom::error::{Error, ErrorKind};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded, terminated};
use nom::{IResult, Parser};

#[cfg(test)]
mod tests;

/// Parse a TPM2B expansion control sequence. This "expands" to the contents of
/// a TPM2B in the response, which is a big-endian u16 length followed by that
/// many bytes.
fn parse_tpm2b(input: &str) -> IResult<&str, Part<'_>> {
    map(tag("TPM2B"), |_| Part::TPM2B).parse(input)
}

/// Parse a binary expansion control sequence. This allows for precision in
/// matching specific bits in the response.
fn parse_binary(input: &str) -> IResult<&str, Part<'_>> {
    preceded(
        alt((tag("0b"), tag("0B"))),
        parse_with_count(
            recognize(many1(terminated(
                one_of(concatcp!("01", WILDCARD)),
                many0(char(SPACE)),
            ))),
            8,
            Part::Binary,
        ),
    )
    .parse(input)
}

/// Parse one of the expansion control sequences.
fn parse_expansion_control_sequence(input: &str) -> IResult<&str, Part<'_>> {
    delimited(
        char(EXPANSION_START),
        alt((parse_tpm2b, parse_binary)),
        char(EXPANSION_END),
    )
    .parse(input)
}

/// Parse the hexadecimal portion of the response, including wildcards.
fn parse_hexadecimal(input: &str) -> IResult<&str, Part<'_>> {
    parse_with_count(
        recognize(many1(terminated(
            one_of(concatcp!("0123456789abcdefABCDEF", WILDCARD)),
            many0(char(SPACE)),
        ))),
        2,
        Part::Hex,
    )
    .parse(input)
}

/// Parse an encoded response
pub fn parse_encoded_response(input: &str) -> Result<Vec<Part<'_>>, nom::Err<Error<&str>>> {
    all_consuming(many1(alt((
        parse_hexadecimal,
        parse_expansion_control_sequence,
    ))))
    .parse(input)
    .map(|(_, parts)| parts)
}

/// Custom parser for numeric literals (hex and binary) which accepts an input
/// `parser`, ensures it parses a `multiple_of` characters excluding [`SPACE`],
/// and maps to the resulting [`Part`] using `part_ctor`.
fn parse_with_count<'a, P, F>(
    mut parser: P,
    multiple_of: usize,
    part_ctor: F,
) -> impl Parser<&'a str, Output = Part<'a>, Error = Error<&'a str>>
where
    P: Parser<&'a str, Output = &'a str, Error = Error<&'a str>>,
    F: Fn(&'a str, usize) -> Part<'a> + Clone,
{
    move |input: &'a str| {
        let (remaining, s) = parser.parse(input)?;
        let count = s.chars().filter(|&c| c != SPACE).count();
        if count % multiple_of != 0 {
            return Err(nom::Err::Error(Error::new(input, ErrorKind::Verify)));
        }
        Ok((remaining, part_ctor(s, count)))
    }
}
