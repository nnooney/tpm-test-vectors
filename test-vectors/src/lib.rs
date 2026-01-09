use serde::{Deserialize, Serialize};

/// A CommandResponsePair represents a single round-trip of bytes sent between
/// the client and the TPM.
#[derive(Debug, Deserialize, Serialize)]
pub struct CommandResponsePair {
    /// A descriptive name of the step to perform.
    pub step: String,
    /// The input bytes to send to the TPM.
    #[serde(with = "hex")]
    pub input: Vec<u8>,
    /// The expected response bytes received from the TPM.
    #[serde(with = "hex")]
    pub response: Vec<u8>,
    /// A mask to apply to the response.
    #[serde(with = "hex")]
    pub response_mask: Vec<u8>,
}

/// A TpmTestVector is a single test case run against a TPM.
#[derive(Debug, Deserialize, Serialize)]
pub struct TpmTestVector {
    /// The name of the test vector.
    pub name: String,
    /// The TPM specification versions that the vector applies to.
    pub spec_versions: Vec<u32>,
    /// The sequence of commands to check in the test case.
    pub test_sequence: Vec<CommandResponsePair>,
}
