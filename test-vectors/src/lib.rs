use serde::{Deserialize, Serialize};

mod step;
pub use step::*;

mod harness;
pub use harness::*;

#[cfg(test)]
mod tests;

/// A TpmTestVector is a single test case run against a TPM.
#[derive(Debug, Deserialize, Serialize)]
pub struct TpmTestVector {
    /// The name of the test vector, effectively the name of the test case.
    pub name: String,
    /// The TPM specification versions that the vector applies to.
    pub spec_versions: Vec<u32>,
    /// The sequence of commands to check in the test case.
    pub test_sequence: Vec<TestStep>,
}
