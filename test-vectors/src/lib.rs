use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

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

/// check_command_response_pair ensures the `command` is well-formed. Errors
/// returned from this function indicate issues in the authoring of the
/// CommandResponsePair.
pub fn check_command_response_pair(command: &CommandResponsePair) -> anyhow::Result<()> {
    // Ensure input/response are at least the minimum length
    if command.input.len() < 10 {
        return Err(anyhow::anyhow!("step \"{}\" input too short", command.step));
    }
    if command.response.len() < 10 {
        return Err(anyhow::anyhow!(
            "step \"{}\" response too short",
            command.step
        ));
    }

    // Ensure encoded length matches the length of the provided value
    let input_len = u32::from_be_bytes(command.input[2..6].try_into()?);
    if input_len as usize != command.input.len() {
        return Err(anyhow::anyhow!(
            "step \"{}\" encoded input length does not match input length
want: {}
 got: {}",
            command.step,
            input_len,
            command.input.len(),
        ));
    }
    let response_len = u32::from_be_bytes(command.response[2..6].try_into()?);
    if response_len as usize != command.response.len() {
        return Err(anyhow::anyhow!(
            "step \"{}\" encoded response length does not match response length
want: {}
 got: {}",
            command.step,
            response_len,
            command.response.len(),
        ));
    }

    // If a response mask is provided, it should have the same length as the
    // response.
    if !command.response_mask.is_empty() && command.response_mask.len() != command.response.len() {
        return Err(anyhow::anyhow!(
            "step \"{}\" response mask length does not match response length
want: {}
 got: {}",
            command.step,
            command.response.len(),
            command.response_mask.len(),
        ));
    }

    Ok(())
}
