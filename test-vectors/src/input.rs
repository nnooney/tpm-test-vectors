//! An input is an encoded string describing the bytes to send to a TPM.

use core::error::Error;
use core::fmt;
use serde::{Deserialize, Serialize};

use crate::parse::{self, ParseError, SPACE};

/// Errors returned from evaluating an input.
#[derive(Debug)]
pub enum InputEvaluationError {
    /// Parse Error
    #[non_exhaustive]
    Parse(ParseError),
    /// From Hex Error
    #[non_exhaustive]
    FromHex(hex::FromHexError),
}

impl fmt::Display for InputEvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Parse(_) => write!(f, "error parsing encoded input"),
            Self::FromHex(_) => write!(f, "error converting input to bytes"),
        }
    }
}

impl Error for InputEvaluationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::Parse(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<ParseError> for InputEvaluationError {
    fn from(err: ParseError) -> Self {
        Self::Parse(err)
    }
}

impl From<hex::FromHexError> for InputEvaluationError {
    fn from(err: hex::FromHexError) -> Self {
        Self::FromHex(err)
    }
}

/// An EncodedInput represents the data to send to the TPM in a compact format
/// useful for writing test vectors. Typically, this gets used with the
/// [`Input`] struct to send the actual data to the TPM.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct EncodedInput(String);

impl fmt::Display for EncodedInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for EncodedInput {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<&str> for EncodedInput {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for EncodedInput {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// An Input represents the data to send to the TPM.
#[derive(Debug)]
pub struct Input<'a> {
    /// The encoded format this input is constructed from, owned for lifetime
    /// referencing the data.
    _encoded: &'a EncodedInput,
    /// The data to send to the TPM with spaces.
    data: &'a str,
}

impl<'a> Input<'a> {
    /// Create a new Input.
    pub fn new(encoded: &'a EncodedInput, data: &'a str) -> Self {
        Self {
            _encoded: encoded,
            data,
        }
    }

    /// Returns the length of the input in bytes, excluding spaces.
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.chars().filter(|&c| c != SPACE).count() / 2
    }

    /// Returns whether the input is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the bytes to send to the TPM.
    pub fn to_bytes(&self) -> Result<Vec<u8>, InputEvaluationError> {
        Ok(hex::decode(self.data.replace(SPACE, ""))?)
    }

    /// Convenience function to construct an Input and get the bytes to send to
    /// the TPM.
    pub fn to_tpm_bytes(encoded: &EncodedInput) -> Result<Vec<u8>, InputEvaluationError> {
        let input = parse::input(encoded)?;
        input.to_bytes()
    }
}
