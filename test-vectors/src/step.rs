use core::error::Error;
use core::fmt;
use serde::{Deserialize, Serialize};
use serde_with::hex::Hex;
use serde_with::serde_as;

use crate::EncodedResponse;

/// Types of errors for a [`CommandResponseError`].
#[derive(Debug)]
pub enum CommandResponseErrorKind {
    #[non_exhaustive]
    InputTooShort,
    #[non_exhaustive]
    ResponseTooShort,
    #[non_exhaustive]
    ResponseMaskLengthDoesNotMatchResponseLength,
}

impl fmt::Display for CommandResponseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::InputTooShort => write!(f, "input too short"),
            Self::ResponseTooShort => write!(f, "response too short"),
            Self::ResponseMaskLengthDoesNotMatchResponseLength => {
                write!(f, "response mask length does not match response length")
            }
        }
    }
}

impl Error for CommandResponseErrorKind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// The error type returned by functions using [`CommandResponsePair`].
#[derive(Debug)]
pub struct CommandResponseError {
    step: String,
    kind: CommandResponseErrorKind,
}

impl CommandResponseError {
    pub fn new(step: &str, kind: CommandResponseErrorKind) -> Self {
        Self {
            step: step.to_string(),
            kind,
        }
    }
}

impl fmt::Display for CommandResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error in step \"{}\"", self.step)
    }
}

impl Error for CommandResponseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.kind)
    }
}

/// A CommandResponsePair represents a single round-trip of bytes sent between
/// the client and the TPM.
#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct CommandResponsePair {
    /// A descriptive name of the step to perform. This ends up in error
    /// messages.
    pub step: String,
    /// The input bytes to send to the TPM.
    #[serde_as(as = "Hex")]
    pub input: Vec<u8>,
    /// The expected response bytes received from the TPM.
    pub response: EncodedResponse,
    /// A mask to apply to the response. If provided, it must have the same
    /// length as the response. When not provided, the mask is treated as all
    /// 0xff bytes.
    #[serde_as(as = "Option<Hex>")]
    pub response_mask: Option<Vec<u8>>,
}

impl CommandResponsePair {
    /// check ensures the CommandResponsePair is well-formed. Errors returned
    /// from this function indicate issues in the authoring of the
    /// CommandResponsePair.
    pub fn check(&self) -> Result<(), CommandResponseError> {
        // Ensure input/response are at least the minimum length
        if self.input.len() < 10 {
            return Err(CommandResponseError::new(
                &self.step,
                CommandResponseErrorKind::InputTooShort,
            ));
        }
        if self.response.len() < 10 {
            return Err(CommandResponseError::new(
                &self.step,
                CommandResponseErrorKind::ResponseTooShort,
            ));
        }

        // If a response mask is provided, it should have the same length as the
        // response.
        if let Some(response_mask) = &self.response_mask
            && response_mask.len() != self.response.len()
        {
            return Err(CommandResponseError::new(
                &self.step,
                CommandResponseErrorKind::ResponseMaskLengthDoesNotMatchResponseLength,
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
}

impl TestStep {
    /// check ensures the TestStep is well-formed. Errors returned from this
    /// function indicate issues in the authoring of the TestStep.
    pub fn check(&self) -> Result<(), CommandResponseError> {
        match self {
            Self::SendCommand(pair) => pair.check(),
            Self::EnterFailureMode => Ok(()),
        }
    }
}
