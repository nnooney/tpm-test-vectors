//! The nom module provides parsing logic using the `nom` crate.

#[cfg(test)]
mod tests;

use crate::parse::{EXPANSION_END, EXPANSION_START, SPACE, WILDCARD};
use crate::response::Part;
use const_format::concatcp;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, one_of};
use nom::combinator::{all_consuming, map, recognize};
use nom::error::{Error, ErrorKind};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded, terminated};
use nom::{Finish, IResult, Parser};

pub type ParseError = nom::Err<nom::error::Error<String>>;

/// Parses hexadecimal from `input`, allowing for [`SPACE`] characters.
fn parse_hex(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(
        one_of("0123456789abcdefABCDEF"),
        many0(char(SPACE)),
    )))
    .parse(input)
}

/// Parses hexadecimal from `input`, allowing for [`SPACE`] and [`WILDCARD`]
/// characters.
fn parse_hex_with_wildcards(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(
        one_of(concatcp!("0123456789abcdefABCDEF", WILDCARD)),
        many0(char(SPACE)),
    )))
    .parse(input)
}

/// Parses binary from `input`, allowing for [`SPACE`] and [`WILDCARD`]
/// characters.
fn parse_binary_with_wildcards(input: &str) -> IResult<&str, &str> {
    preceded(
        alt((tag("0b"), tag("0B"))),
        recognize(many1(terminated(
            one_of(concatcp!("01", WILDCARD)),
            many0(char(SPACE)),
        ))),
    )
    .parse(input)
}

/// Custom parser for numeric literals (hex and binary) which accepts an input
/// `parser`, ensures it parses a `multiple_of` characters excluding [`SPACE`],
/// and maps to the resulting [`Part`] using `part_ctor`.
fn parse_numeric_part<'a, P, F>(
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

/// Parses a binary part from an encoded response.
fn parse_binary_part(input: &str) -> IResult<&str, Part<'_>> {
    parse_numeric_part(parse_binary_with_wildcards, 8, Part::Binary).parse(input)
}

/// Parses a hex part from an encoded response.
fn parse_hex_part(input: &str) -> IResult<&str, Part<'_>> {
    parse_numeric_part(parse_hex_with_wildcards, 2, Part::Hex).parse(input)
}

/// Parses a TPM2B part from an encoded response. This "expands" to the contents
/// of a TPM2B in the response, which is a big-endian u16 length followed by
/// that many bytes.
fn parse_tpm2b_part(input: &str) -> IResult<&str, Part<'_>> {
    map(tag("TPM2B"), |_| Part::TPM2B).parse(input)
}

/// Parses expansion control sequences.
fn parse_expansion_control_sequence(input: &str) -> IResult<&str, Part<'_>> {
    terminated(
        delimited(
            char(EXPANSION_START),
            alt((parse_tpm2b_part, parse_binary_part)),
            char(EXPANSION_END),
        ),
        many0(char(SPACE)),
    )
    .parse(input)
}

/// Parse an encoded response.
pub fn parse_encoded_response(input: &str) -> Result<Vec<Part<'_>>, ParseError> {
    match all_consuming(many1(alt((
        parse_hex_part,
        parse_expansion_control_sequence,
    ))))
    .parse(input)
    .finish()
    {
        Ok((_, parts)) => Ok(parts),
        Err(Error { input, code }) => Err(nom::Err::Error(Error {
            input: input.to_string(),
            code,
        })),
    }
}

/// Parse an encoded input
pub fn parse_encoded_input(input: &str) -> Result<&str, ParseError> {
    match all_consuming(parse_hex).parse(input).finish() {
        Ok((_, remaining)) => Ok(remaining),
        Err(Error { input, code }) => Err(nom::Err::Error(Error {
            input: input.to_string(),
            code,
        })),
    }
}
