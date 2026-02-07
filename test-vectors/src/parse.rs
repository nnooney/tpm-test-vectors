use crate::{CommandResponsePair, TpmTestVector};

use core::error::Error;
use core::fmt;
use ron::{Options, error::SpannedError, extensions::Extensions};

/// Returns the configured ron parser
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
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Ron(ref _err) => write!(f, "ron parse error"),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::Ron(ref err) => Some(err),
        }
    }
}

impl From<SpannedError> for ParseError {
    fn from(err: SpannedError) -> Self {
        Self::Ron(err)
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
