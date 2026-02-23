use crate::input::SPACE;

use nom::character::complete::{char, one_of};
use nom::combinator::{all_consuming, recognize};
use nom::error::Error;
use nom::multi::{many0, many1};
use nom::sequence::terminated;
use nom::{IResult, Parser};

#[cfg(test)]
mod tests;

/// Parse the hexadecimal portion of the response, including wildcards.
fn parse_hexadecimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(
        one_of("0123456789abcdefABCDEF"),
        many0(char(SPACE)),
    )))
    .parse(input)
}

/// Parse an encoded input
pub fn parse_encoded_input(input: &str) -> Result<&str, nom::Err<Error<&str>>> {
    all_consuming(parse_hexadecimal)
        .parse(input)
        .map(|(_, consumed)| consumed)
}
