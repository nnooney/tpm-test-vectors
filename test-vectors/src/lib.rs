use serde::{Deserialize, Serialize};

mod cmd_rsp;
pub use cmd_rsp::*;

mod harness;
pub use harness::*;

#[cfg(test)]
mod tests;

/// A TestRequirement provides restrictions on the environment needed to run
/// the test vector. See each value for details. If a particular requirement
/// is not satisfied, then the test is failed.
///
/// The test harness satisfies a requirement by providing an implementation of
/// a function on the [`Harness`] trait.
#[derive(Debug, Deserialize, Serialize)]
pub enum TestRequirement {
    /// Require the ability to put the TPM into failure mode. This is useful for
    /// test vectors which evaluate funcitonality of a TPM while in failure
    /// mode.
    FailureMode,
}

/// A TpmTestVector is a single test case run against a TPM.
#[derive(Debug, Deserialize, Serialize)]
pub struct TpmTestVector {
    /// The name of the test vector, effectively the name of the test case.
    pub name: String,
    /// The TPM specification versions that the vector applies to.
    pub spec_versions: Vec<u32>,
    /// Requirements for running the test vector.
    pub requirements: Option<Vec<TestRequirement>>,
    /// The sequence of commands to check in the test case.
    pub test_sequence: Vec<CommandResponsePair>,
}
