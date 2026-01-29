use core::error::Error;
use core::fmt;

use crate::CommandResponsePair;

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

/// check_command_response_pair ensures the `command` is well-formed. Errors
/// returned from this function indicate issues in the authoring of the
/// CommandResponsePair.
pub fn check_command_response_pair(
    command: &CommandResponsePair,
) -> Result<(), CommandResponseError> {
    // Ensure input/response are at least the minimum length
    if command.input.len() < 10 {
        return Err(CommandResponseError::new(
            &command.step,
            CommandResponseErrorKind::InputTooShort,
        ));
    }
    if command.response.len() < 10 {
        return Err(CommandResponseError::new(
            &command.step,
            CommandResponseErrorKind::ResponseTooShort,
        ));
    }

    // If a response mask is provided, it should have the same length as the
    // response.
    if !command.response_mask.is_empty() && command.response_mask.len() != command.response.len() {
        return Err(CommandResponseError::new(
            &command.step,
            CommandResponseErrorKind::ResponseMaskLengthDoesNotMatchResponseLength,
        ));
    }

    Ok(())
}
