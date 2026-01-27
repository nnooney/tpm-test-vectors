//! The harness module provides type definitions and default implementations for
//! running the test vectors.

use core::error::Error;
use core::fmt;

/// Types of errors the [`Harness`] can return.
#[derive(Debug)]
pub enum HarnessErrorKind<T> {
    #[non_exhaustive]
    TransactUnsupported,
    #[non_exhaustive]
    FailureModeUnsupported,
    #[non_exhaustive]
    Connection(T),
}

impl<T> fmt::Display for HarnessErrorKind<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TransactUnsupported => write!(f, "transact fn unsupported"),
            Self::FailureModeUnsupported => write!(f, "failure mode fn unsupported"),
            Self::Connection(ref _err) => write!(f, "connection error"),
        }
    }
}

impl<T: fmt::Debug + Error + 'static> Error for HarnessErrorKind<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::Connection(ref err) => Some(err),
            _ => None,
        }
    }
}

/// The error type returned by functions in the [`Harness`].
#[derive(Debug)]
pub struct HarnessError<T> {
    kind: HarnessErrorKind<T>,
}

impl<T> HarnessError<T> {
    pub fn new(kind: HarnessErrorKind<T>) -> Self {
        Self { kind }
    }
}

impl<T> fmt::Display for HarnessError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error in the test harness")
    }
}

impl<T: fmt::Debug + Error + 'static> Error for HarnessError<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.kind)
    }
}

// Until https://github.com/rust-lang/rust/issues/29205 is addressed, it's not
// currently possible to constrain this From impl to the associated type on the
// upstream tpm2-client crate. Therefore, this is a blanket implementation. The
// ideal version is:
// impl<T: Connection> From<T::Error> for HarnessError<T::Error> {
//     fn from(err: T::Error) -> Self {
impl<T> From<T> for HarnessError<T> {
    fn from(err: T) -> Self {
        Self::new(HarnessErrorKind::Connection(err))
    }
}

/// The Harness trait describes the implementation necessary to run all of the
/// test vectors.
pub trait Harness<T> {
    /// Perform a command/response transaction with the TPM.
    ///
    /// Note that even if the response contains a `TPM_RC` error, this method
    /// still returns `Ok(())`. `Err` is only returned when we are unable to get
    /// a response at all.
    ///
    /// This function mirrors the same function from the
    /// [`tpm2-client::connection::Connection`] trait.
    fn transact(&mut self, _cmd: &[u8], _rsp: &mut [u8]) -> Result<(), HarnessError<T>> {
        Err(HarnessError::new(HarnessErrorKind::TransactUnsupported))
    }

    /// Set the TPM in failure mode.
    ///
    /// This function is required to run test vectors with the
    /// [`TestRequirement::FailureMode`] requirement.
    fn set_failure_mode(&mut self) -> Result<(), HarnessError<T>> {
        Err(HarnessError::new(HarnessErrorKind::FailureModeUnsupported))
    }
}
