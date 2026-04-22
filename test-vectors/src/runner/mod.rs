//! The runner module provides common logic to run a test vector against a
//! harness implementation.

use crate::parse::ParseError;
use crate::{
    Harness, HarnessError, Input, InputEvaluationError, Response, ResponseEvaluationError,
    TestStep, TestStepError, parse,
};

use core::error::Error;
use core::fmt;

/// Types of errors that can be returned by running a test vector.
#[derive(Debug)]
pub enum TestErrorKind {
    #[non_exhaustive]
    ParseError(ParseError),
    #[non_exhaustive]
    TestStepError(TestStepError),
    #[non_exhaustive]
    InputEvaluationError(InputEvaluationError),
    #[non_exhaustive]
    HarnessError(HarnessError),
    #[non_exhaustive]
    ResponseEvaluationError(ResponseEvaluationError),
}

impl fmt::Display for TestErrorKind {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl Error for TestErrorKind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::ParseError(ref e) => Some(e),
            Self::TestStepError(ref e) => Some(e),
            Self::InputEvaluationError(ref e) => Some(e),
            Self::HarnessError(ref e) => Some(e),
            Self::ResponseEvaluationError(ref e) => Some(e),
        }
    }
}

impl From<ParseError> for TestErrorKind {
    fn from(err: ParseError) -> Self {
        Self::ParseError(err)
    }
}

impl From<TestStepError> for TestErrorKind {
    fn from(err: TestStepError) -> Self {
        Self::TestStepError(err)
    }
}

impl From<InputEvaluationError> for TestErrorKind {
    fn from(err: InputEvaluationError) -> Self {
        Self::InputEvaluationError(err)
    }
}

impl From<HarnessError> for TestErrorKind {
    fn from(err: HarnessError) -> Self {
        Self::HarnessError(err)
    }
}

impl From<ResponseEvaluationError> for TestErrorKind {
    fn from(err: ResponseEvaluationError) -> Self {
        Self::ResponseEvaluationError(err)
    }
}

/// The error type returned by the [`run_test_vector`] function.
#[derive(Debug)]
pub struct TestError {
    // Name of the test that failed.
    name: String,
    // Step within the test that failed. May be empty.
    step: Option<String>,
    // Details about the failure that occurred.
    kind: TestErrorKind,
}

impl TestError {
    pub fn new<T>(name: String, step: Option<String>, err: T) -> Self
    where
        TestErrorKind: From<T>,
    {
        Self {
            name,
            step,
            kind: err.into(),
        }
    }
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = if self.name.is_empty() {
            "unknown"
        } else {
            &self.name
        };
        write!(f, "\nError in test \"{name}\"")?;
        if let Some(step) = &self.step {
            write!(f, "\n      in step \"{step}\"")?;
        }
        Ok(())
    }
}

impl Error for TestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.kind)
    }
}

/// run_test_vector applies the `input` test vector to the TPM using the
/// `harness` that implements the [`Harness`] trait.
pub fn run_test_vector<H: Harness>(input: &str, harness: &mut H) -> Result<(), Box<TestError>> {
    let test_case =
        parse::tpm_test_vector(input).map_err(|e| TestError::new(String::new(), None, e))?;

    for step in test_case.test_sequence {
        step.check()
            .map_err(|e| TestError::new(test_case.name.clone(), Some(step.name()), e))?;

        match step {
            TestStep::SendCommand(ref command) => {
                // 4096 matches [`tpm2_rs_client::RESP_BUFFER_SIZE`], but we
                // inline the constant here to avoid taking a runtime dependency
                // on the client, prefering to keep it a dev-dependency.
                let mut buf = [0u8; 4096];
                let resp = harness
                    .transact(
                        &Input::to_tpm_bytes(&command.input).map_err(|e| {
                            TestError::new(test_case.name.clone(), Some(step.name()), e)
                        })?,
                        &mut buf,
                    )
                    .map_err(|e| TestError::new(test_case.name.clone(), Some(step.name()), e))?;

                Response::evaluate(
                    &command.response,
                    resp,
                    harness.store_mut().map_err(|e| {
                        TestError::new(test_case.name.clone(), Some(step.name()), e)
                    })?,
                )
                .map_err(|e| TestError::new(test_case.name.clone(), Some(step.name()), e))?;
            }
            TestStep::EnterFailureMode => {
                harness
                    .set_failure_mode()
                    .map_err(|e| TestError::new(test_case.name.clone(), Some(step.name()), e))?;
            }
            TestStep::SetLocality(ref locality) => {
                harness
                    .set_locality(locality.0)
                    .map_err(|e| TestError::new(test_case.name.clone(), Some(step.name()), e))?;
            }
        }
    }

    Ok(())
}
