use tpm2_rs_client::connection::Connection;

/// run_test_vector applies the `input` test vector to the TPM using the `conn`
/// [`Connection`].
pub fn run_test_vector<T: Connection>(input: &str, conn: &mut T) -> anyhow::Result<()> {
    let test_case: tpm2_test_vectors::TpmTestVector = ron::from_str(input)?;

    for command in test_case.test_sequence {
        let mut resp = vec![0; command.response.len()];
        conn.transact(&command.input, &mut resp)?;
        assert_eq!(
            resp, command.response,
            "step \"{}\" response mismatch",
            command.step
        );
    }

    Ok(())
}
