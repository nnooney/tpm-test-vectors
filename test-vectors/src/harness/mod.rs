//! The harness module provides type definitions and default implementations for
//! running the test vectors.

pub mod store;
pub use store::*;

use core::error::Error;
use core::fmt;

/// Types of errors the [`Harness`] can return. Some harness errors occur from
/// the associated error type of [`tpm2-client::connection::Connection`], which
/// is implemented here via concrete types.
#[derive(Debug)]
pub enum HarnessErrorKind {
    #[non_exhaustive]
    TransactUnsupported,
    #[non_exhaustive]
    FailureModeUnsupported,
    #[non_exhaustive]
    LocalityUnsupported,
    #[non_exhaustive]
    StoreUnsupported,
    #[non_exhaustive]
    Io(std::io::Error), // Used by TcpConnection
}

impl fmt::Display for HarnessErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TransactUnsupported => write!(f, "transact fn unsupported"),
            Self::FailureModeUnsupported => write!(f, "failure mode fn unsupported"),
            Self::LocalityUnsupported => write!(f, "locality fn unsupported"),
            Self::StoreUnsupported => write!(f, "store fn unsupported"),
            Self::Io(ref _err) => write!(f, "I/O error"),
        }
    }
}

impl Error for HarnessErrorKind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::Io(ref err) => Some(err),
            _ => None,
        }
    }
}

/// The error type returned by functions in the [`Harness`].
#[derive(Debug)]
pub struct HarnessError {
    kind: HarnessErrorKind,
}

impl HarnessError {
    pub fn new(kind: HarnessErrorKind) -> Self {
        Self { kind }
    }
}

impl fmt::Display for HarnessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error in the test harness")
    }
}

impl Error for HarnessError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.kind)
    }
}

impl From<std::io::Error> for HarnessError {
    fn from(err: std::io::Error) -> Self {
        Self::new(HarnessErrorKind::Io(err))
    }
}

/// The Harness trait describes the implementation necessary to run all of the
/// test vectors.
pub trait Harness {
    /// Perform a command/response transaction with the TPM.
    ///
    /// Note that even if the response contains a `TPM_RC` error, this method
    /// still returns `Ok(())`. `Err` is only returned when we are unable to get
    /// a response at all.
    ///
    /// This function mirrors the same function from the
    /// [`tpm2-client::connection::Connection`] trait.
    fn transact<'a>(
        &mut self,
        _cmd: &[u8],
        _rsp: &'a mut [u8],
    ) -> Result<&'a mut [u8], HarnessError> {
        Err(HarnessError::new(HarnessErrorKind::TransactUnsupported))
    }

    /// Set the TPM in failure mode.
    fn set_failure_mode(&mut self) -> Result<(), HarnessError> {
        Err(HarnessError::new(HarnessErrorKind::FailureModeUnsupported))
    }

    /// Set the locality for subsequent commands.
    fn set_locality(&mut self, _locality: u8) -> Result<(), HarnessError> {
        Err(HarnessError::new(HarnessErrorKind::LocalityUnsupported))
    }

    /// Provides mutable access to the test's value store.
    fn store_mut(&mut self) -> Result<&mut store::Store, HarnessError> {
        Err(HarnessError::new(HarnessErrorKind::StoreUnsupported))
    }
}
