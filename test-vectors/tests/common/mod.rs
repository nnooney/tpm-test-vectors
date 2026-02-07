use anyhow::anyhow;
use tpm2_test_vectors::parse;
use tpm2_test_vectors::{CommandResponsePair, Harness, TestStep};

/// run_test_vector applies the `input` test vector to the TPM using the
/// `harness` that implements the [`Harness`] trait.
pub fn run_test_vector<H: Harness>(input: &str, harness: &mut H) -> anyhow::Result<()> {
    let test_case = parse::tpm_test_vector(input)?;

    for step in test_case.test_sequence {
        step.check()?;

        match step {
            TestStep::SendCommand(command) => {
                let mut buf = [0u8; tpm2_rs_client::RESP_BUFFER_SIZE];
                let resp = harness.transact(&command.input, &mut buf)?;

                evaluate_command_response_pair(&command, resp)?;
            }
            TestStep::EnterFailureMode => {
                harness.set_failure_mode()?;
            }
            _ => return Err(anyhow!("Unhandled step type {:?}", step)),
        }
    }

    Ok(())
}

/// evaluate_command_response_pair compares the `resp` returned from the TPM
/// against the expectations of the `command`.
fn evaluate_command_response_pair(
    command: &CommandResponsePair,
    resp: &[u8],
) -> anyhow::Result<()> {
    // The response buffer must be at least as long as the expected response.
    if resp.len() < command.response.len() {
        return Err(anyhow!(
            r#"
step "{step}" TPM response too short
  want: >= {want}
   got: {got}"#,
            step = command.step,
            want = command.response.len(),
            got = resp.len(),
        ));
    }

    // Each byte in the expected response must match the response.
    for (i, byte) in command.response.to_bytes()?.iter().enumerate() {
        let mask = command.response_mask.as_ref().map_or(0xff, |m| m[i]);
        if *byte & mask != resp[i] & mask {
            return Err(anyhow!(
                r#"
step "{step}" response mismatch,
  mask: {mask}
  want: {want}
   got: {got}
        {pad:width$}^^ mismatch begins at byte {byte}"#,
                step = command.step,
                mask = hex::encode(command.response_mask.as_ref().unwrap_or(&vec![])),
                want = command.response.as_ref(),
                got = hex::encode(resp),
                pad = ' ',
                width = i * 2, // two hex chars per byte
                byte = i,
            ));
        }
    }

    Ok(())
}
