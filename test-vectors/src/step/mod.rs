use core::error::Error;
use core::fmt;
use serde::{Deserialize, Serialize};

use crate::input::EncodedInput;
use crate::parse::{self, ParseError};
use crate::response::EncodedResponse;

/// Types of errors for a [`TestStepError`].
#[derive(Debug)]
pub enum TestStepErrorKind {
    #[non_exhaustive]
    ParseInput(Box<ParseError>),
    #[non_exhaustive]
    ParseResponse(Box<ParseError>),
    #[non_exhaustive]
    InputTooShort,
    #[non_exhaustive]
    ResponseTooShort,
    #[non_exhaustive]
    InvalidLocality,
}

impl fmt::Display for TestStepErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ParseInput(_) => write!(f, "parse input error"),
            Self::ParseResponse(_) => write!(f, "parse response error"),
            Self::InputTooShort => write!(f, "input too short"),
            Self::ResponseTooShort => write!(f, "response too short"),
            Self::InvalidLocality => write!(f, "invalid locality"),
        }
    }
}

impl Error for TestStepErrorKind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::ParseInput(ref err) => Some(err.as_ref()),
            Self::ParseResponse(ref err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

/// The error type returned by functions using [`CommandResponsePair`].
#[derive(Debug)]
pub struct TestStepError {
    step: String,
    kind: TestStepErrorKind,
}

impl TestStepError {
    pub fn new(step: &str, kind: TestStepErrorKind) -> Self {
        Self {
            step: step.to_string(),
            kind,
        }
    }
}

impl fmt::Display for TestStepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error in step \"{}\"", self.step)
    }
}

impl Error for TestStepError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.kind)
    }
}

/// A CommandResponsePair represents a single round-trip of bytes sent between
/// the client and the TPM.
#[derive(Debug, Deserialize, Serialize)]
pub struct CommandResponsePair {
    /// A descriptive name of the step to perform. This ends up in error
    /// messages.
    pub step: String,
    /// The input bytes to send to the TPM. This value supports the encoded
    /// format described in TODO.
    pub input: EncodedInput,
    /// The expected response bytes received from the TPM. This value supports
    /// the encoded format described in TODO.
    pub response: EncodedResponse,
}

impl CommandResponsePair {
    /// check ensures the CommandResponsePair is well-formed. Errors returned
    /// from this function indicate issues in the authoring of the
    /// CommandResponsePair.
    pub fn check(&self) -> Result<(), TestStepError> {
        // Ensure encoded fields parse correctly.
        let input = parse::input(&self.input).map_err(|e| {
            TestStepError::new(&self.step, TestStepErrorKind::ParseInput(Box::new(e)))
        })?;
        let response = parse::response(&self.response).map_err(|e| {
            TestStepError::new(&self.step, TestStepErrorKind::ParseResponse(Box::new(e)))
        })?;

        // Minimum size of an input is a command header with no params.
        const MIN_INPUT_LEN: usize = 10;
        // Minimum size of a response is a response header with no params.
        const MIN_RESPONSE_LEN: usize = 10;

        // Ensure input/response are at least the minimum length
        if input.len() < MIN_INPUT_LEN {
            return Err(TestStepError::new(
                &self.step,
                TestStepErrorKind::InputTooShort,
            ));
        }
        if response.min_len() < MIN_RESPONSE_LEN {
            return Err(TestStepError::new(
                &self.step,
                TestStepErrorKind::ResponseTooShort,
            ));
        }

        Ok(())
    }
}

/// A locality represents the locality to issue TPM commands at.
#[derive(Debug, Deserialize, Serialize)]
pub struct Locality(pub u8);

impl Locality {
    /// check ensures the Locality is well-formed. Errors returned from this
    /// function indicate issues in the authoring of the Locality.
    pub fn check(&self) -> Result<(), TestStepError> {
        if self.0 > 4 && self.0 < 32 {
            return Err(TestStepError::new(
                &format!("SetLocality {}", self.0),
                TestStepErrorKind::InvalidLocality,
            ));
        }

        Ok(())
    }
}

/// Types of steps in a test sequence.
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum TestStep {
    /// Send a command to the TPM and check its response.
    SendCommand(CommandResponsePair),
    /// Cause the TPM to enter failure mode.
    EnterFailureMode,
    /// Set the locality for subsequent commands.
    SetLocality(Locality),
}

impl TestStep {
    /// check ensures the TestStep is well-formed. Errors returned from this
    /// function indicate issues in the authoring of the TestStep.
    pub fn check(&self) -> Result<(), TestStepError> {
        match self {
            Self::SendCommand(cmd) => cmd.check(),
            Self::EnterFailureMode => Ok(()),
            Self::SetLocality(locality) => locality.check(),
        }
    }

    /// name returns the name of the TestStep, useful for diagnostics.
    pub fn name(&self) -> String {
        match self {
            Self::SendCommand(cmd) => cmd.step.clone(),
            Self::EnterFailureMode => "EnterFailureMode".to_owned(),
            Self::SetLocality(locality) => format!("SetLocality {}", locality.0),
        }
    }
}
