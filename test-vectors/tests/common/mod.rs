use anyhow::{Context, anyhow};
use tpm2_test_vectors::input::Input;
use tpm2_test_vectors::parse;
use tpm2_test_vectors::response::Response;
use tpm2_test_vectors::{Harness, TestStep};

/// run_test_vector applies the `input` test vector to the TPM using the
/// `harness` that implements the [`Harness`] trait.
pub fn run_test_vector<H: Harness>(input: &str, harness: &mut H) -> anyhow::Result<()> {
    let test_case = parse::tpm_test_vector(input)?;

    for step in test_case.test_sequence {
        step.check()?;

        match step {
            TestStep::SendCommand(command) => {
                let mut buf = [0u8; tpm2_rs_client::RESP_BUFFER_SIZE];
                let resp = harness.transact(&Input::to_tpm_bytes(&command.input)?, &mut buf)?;

                Response::evaluate(&command.response, resp)
                    .context(format!("\nFailure in step \"{step}\"", step = command.step))?;
            }
            TestStep::EnterFailureMode => {
                harness.set_failure_mode()?;
            }
            TestStep::SetLocality(locality) => {
                harness.set_locality(locality.0)?;
            }
            _ => return Err(anyhow!("\nUnhandled step type {:?}", step)),
        }
    }

    Ok(())
}
