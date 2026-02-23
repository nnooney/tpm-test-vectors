//! The parse module provides routines for parsing input data. It makes use of
//! two crates:
//!
//!   1. ron, for parsing test vectors and components within
//!   2. nom, for parsing encoded responses
use crate::input::{EncodedInput, Input};
use crate::parse::input::parse_encoded_input;
use crate::parse::response::parse_encoded_response;
use crate::response::{EncodedResponse, Response};
use crate::{CommandResponsePair, TpmTestVector};

use core::fmt;
use ron::{Options, error::SpannedError, extensions::Extensions};

mod input;
mod response;

/// Returns the configured ron parser with extensions enabled.
fn ron() -> Options {
    Options::default().with_default_extension(
        Extensions::IMPLICIT_SOME
            | Extensions::UNWRAP_NEWTYPES
            | Extensions::UNWRAP_VARIANT_NEWTYPES,
    )
}

/// Types of errors that can occur from parsing inputs.
#[derive(Debug)]
pub enum ParseError {
    #[non_exhaustive]
    Ron(SpannedError),
    #[non_exhaustive]
    Nom(nom::Err<nom::error::Error<String>>),
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

impl From<SpannedError> for ParseError {
    fn from(err: SpannedError) -> Self {
        Self::Ron(err)
    }
}

impl From<nom::Err<nom::error::Error<String>>> for ParseError {
    fn from(err: nom::Err<nom::error::Error<String>>) -> Self {
        Self::Nom(err)
    }
}

impl<'a> From<nom::Err<nom::error::Error<&'a str>>> for ParseError {
    fn from(err: nom::Err<nom::error::Error<&'a str>>) -> Self {
        Self::Nom(err.map(|e| nom::error::Error::new(e.input.to_string(), e.code)))
    }
}

/// Parses a TpmTestVector
pub fn tpm_test_vector(input: &str) -> Result<TpmTestVector, ParseError> {
    let test_vector: TpmTestVector = ron().from_str(input)?;
    Ok(test_vector)
}

/// Parses a CommandResponsePair
pub fn command_response_pair(input: &str) -> Result<CommandResponsePair, ParseError> {
    let command: CommandResponsePair = ron().from_str(input)?;
    Ok(command)
}

/// Parses an Input
pub fn input(input: &EncodedInput) -> Result<Input<'_>, ParseError> {
    let data = parse_encoded_input(input.as_ref())?;
    Ok(Input::new(input, data))
}

/// Parses a Response
pub fn response(input: &EncodedResponse) -> Result<Response<'_>, ParseError> {
    let parts = parse_encoded_response(input.as_ref())?;
    Ok(Response::new(input, parts))
}
