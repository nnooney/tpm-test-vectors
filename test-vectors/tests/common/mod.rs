use std::error::Error;

use tpm2_test_vectors::{
    CommandResponsePair, Harness, TestRequirement, TpmTestVector, check_command_response_pair,
};

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
        check_command_response_pair(&command)?;

        let mut resp = vec![0; command.response.len()];
        harness.transact(&command.input, &mut resp)?;

        evaluate_command_response_pair(&command, &resp)?;
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
    // Require the responses to have the same length.
    if command.response.len() != resp.len() {
        return Err(anyhow::anyhow!(
            r#"step "{}" response length mismatch
want: {}
 got: {}"#,
            command.step,
            command.response.len(),
            resp.len(),
        ));
    }

    assert_eq!(
        resp, command.response,
        "step \"{}\" response mismatch",
        command.step
    );
    Ok(())
}
