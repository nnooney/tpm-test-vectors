//! The parse module provides routines for parsing input data. It makes use of
//! two crates:
//!
//!   1. ron, for parsing test vectors and components within.
//!   2. nom, for parsing encoded fields.
use crate::input::{EncodedInput, Input};
use crate::response::{EncodedResponse, Response};
use crate::{CommandResponsePair, TpmTestVector};

use core::fmt;

mod nom;
mod ron;

/// Character that represents spaces in numeric sequences, used for making long
/// sequences more readable.
pub const SPACE: char = '_';

/// Character that represents a wildcard value, which will automatically match
/// the corresponding element of the response.
pub const WILDCARD: char = '*';

/// Character that represents the opening of an expansion control sequence.
pub const EXPANSION_START: char = '{';

/// Character that represents the closing of an expansion control sequence.
pub const EXPANSION_END: char = '}';

/// Types of errors that can occur from parsing inputs.
#[derive(Debug)]
pub enum ParseError {
    #[non_exhaustive]
    Ron(ron::ParseError),
    #[non_exhaustive]
    Nom(nom::ParseError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Ron(_) => write!(f, "ron parse error"),
            Self::Nom(_) => write!(f, "nom parse error"),
        }
    }
}

impl core::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match *self {
            Self::Ron(ref err) => Some(err),
            Self::Nom(ref err) => Some(err),
        }
    }
}

impl From<ron::ParseError> for ParseError {
    fn from(err: ron::ParseError) -> Self {
        Self::Ron(err)
    }
}

impl From<nom::ParseError> for ParseError {
    fn from(err: nom::ParseError) -> Self {
        Self::Nom(err)
    }
}

/// Parses a [`TpmTestVector`].
pub fn tpm_test_vector(input: &str) -> Result<TpmTestVector, ParseError> {
    let test_vector: TpmTestVector = ron::parser().from_str(input)?;
    Ok(test_vector)
}

/// Parses a [`CommandResponsePair`].
pub fn command_response_pair(input: &str) -> Result<CommandResponsePair, ParseError> {
    let command: CommandResponsePair = ron::parser().from_str(input)?;
    Ok(command)
}

/// Parses an [`Input`] from the [`EncodedInput`] of a [`CommandResponsePair`].
pub fn input(input: &EncodedInput) -> Result<Input<'_>, ParseError> {
    let data = nom::parse_encoded_input(input.as_ref())?;
    Ok(Input::new(input, data))
}

/// Parses a [`Response`] from the [`EncodedResponse`] of a
/// [`CommandResponsePair`].
pub fn response(input: &EncodedResponse) -> Result<Response<'_>, ParseError> {
    let parts = nom::parse_encoded_response(input.as_ref())?;
    Ok(Response::new(input, parts))
}
