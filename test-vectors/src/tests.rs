use anyhow::Context;
use rstest::rstest;

use crate::CommandResponsePair;
use crate::check_command_response_pair;

#[rstest]
#[case::input_too_short("src/testdata/01-input-too-short.ron", "input too short")]
#[case::response_too_short("src/testdata/02-response-too-short.ron", "response too short")]
#[case::encoded_input_length_does_not_match_input_length(
    "src/testdata/03-input-length-encoding.ron",
    "encoded input length does not match input length"
)]
#[case::encoded_response_length_does_not_match_response_length(
    "src/testdata/04-response-length-encoding.ron",
    "encoded response length does not match response length"
)]
#[case::response_mask_length_does_not_match_response_length(
    "src/testdata/05-response-mask-length.ron",
    "response mask length does not match response length"
)]
fn test_check_command_response_pair_errors(
    #[case] input: &str,
    #[case] expected: &str,
) -> anyhow::Result<()> {
    let contents = std::fs::read_to_string(input)
        .with_context(|| format!("Failed to read testdata file from {}", input))?;
    let command: CommandResponsePair = ron::from_str(&contents).with_context(|| {
        format!(
            "Failed to parse contents as CommandResponsePair from {}",
            input
        )
    })?;

    let result = check_command_response_pair(&command);

    match result {
        Ok(()) => {
            return Err(anyhow::anyhow!(
                "Expected failure but succeeded for input {}",
                input
            ));
        }
        Err(err) => assert!(
            err.to_string().contains(expected),
            r#"err does not contain expected substring
        err: {}
  substring: {}"#,
            err,
            expected
        ),
    }

    Ok(())
}
