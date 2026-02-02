use std::error::Error;

use tpm2_test_vectors::{CommandResponsePair, Harness, TestRequirement, TpmTestVector};

/// run_test_vector applies the `input` test vector to the TPM using the
/// `harness` that implements the [`Harness`] trait.
pub fn run_test_vector<E, H>(input: &str, harness: &mut H) -> anyhow::Result<()>
where
    E: Error + Send + Sync + 'static,
    H: Harness<E>,
{
    let test_case: TpmTestVector = ron::from_str(input)?;

    if let Some(requirements) = test_case.requirements {
        handle_requirements(&requirements, harness)?;
    }

    for command in test_case.test_sequence {
        command.check()?;

        let mut resp = [0u8; tpm2_rs_client::RESP_BUFFER_SIZE];
        harness.transact(&command.input, &mut resp)?;
        // https://github.com/tpm-rs/tpm-rs/issues/208 will simplify this code
        // by returning the size of the response; until then, check the size
        // from the response buffer.
        let resp_size = u32::from_be_bytes(resp[2..6].try_into()?);

        evaluate_command_response_pair(&command, &resp[..resp_size as usize])?;
    }

    Ok(())
}

/// handle_requirements modifies the TPM so it is in the correct state required
/// by the test vector.
fn handle_requirements<E, H>(
    requirements: &[TestRequirement],
    harness: &mut H,
) -> anyhow::Result<()>
where
    E: Error + Send + Sync + 'static,
    H: Harness<E>,
{
    for requirement in requirements {
        match requirement {
            TestRequirement::FailureMode => {
                harness.set_failure_mode()?;
            }
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
        return Err(anyhow::anyhow!(
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
    for (i, byte) in command.response.iter().enumerate() {
        let mask = command.response_mask.as_ref().map_or(0xff, |m| m[i]);
        if *byte & mask != resp[i] & mask {
            return Err(anyhow::anyhow!(
                r#"
step "{step}" response mismatch,
  mask: {mask}
  want: {want}
   got: {got}
        {pad:width$}^^ mismatch begins at byte {byte}"#,
                step = command.step,
                mask = hex::encode(command.response_mask.as_ref().unwrap_or(&vec![])),
                want = hex::encode(&command.response),
                got = hex::encode(resp),
                pad = ' ',
                width = i * 2, // two hex chars per byte
                byte = i,
            ));
        }
    }

    Ok(())
}
