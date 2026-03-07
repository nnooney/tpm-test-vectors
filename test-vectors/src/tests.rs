use anyhow::Context;
use core::error::Error;
use core::fmt::Write;
use rstest::rstest;

use crate::parse;

// Each step in the real test vectors should pass the check function.
#[rstest]
fn test_check_step_success(
    #[files("*.ron")]
    #[base_dir = "src/vectors/"]
    #[mode = str]
    tv: &str,
) -> anyhow::Result<()> {
    let test_vector = parse::tpm_test_vector(tv)?;

    for command in test_vector.test_sequence {
        command.check()?;
    }

    Ok(())
}

#[rstest]
#[case::input_too_short("src/testdata/01-input-too-short.ron", "input too short")]
#[case::response_too_short("src/testdata/02-response-too-short.ron", "response too short")]
#[case::parse_input_error("src/testdata/03-parse-input-error.ron", "parse input error")]
#[case::parse_response_error("src/testdata/04-parse-response-error.ron", "parse response error")]
#[case::invalid_locality("src/testdata/05-invalid-locality.ron", "invalid locality")]
fn test_check_step_errors(#[case] input: &str, #[case] expected: &str) -> anyhow::Result<()> {
    let contents = std::fs::read_to_string(input)
        .with_context(|| format!("Failed to read testdata file from {}", input))?;
    let step = parse::test_step(&contents)
        .with_context(|| format!("Failed to parse contents as TestStep from {}", input))?;

    let result = step.check();

    match result {
        Ok(()) => Err(anyhow::anyhow!(
            "Expected failure but succeeded for input {}",
            input
        ))?,
        Err(err) => assert!(
            format_error(&err)?.contains(expected),
            r#"err does not contain expected substring
        err: {}
  substring: {}"#,
            format_error(&err)?,
            expected
        ),
    }

    Ok(())
}

// Helper function to unroll an error and display it and all its sources, useful
// for checking certain error messages appear in results.
//
// This could be simplified once the following features are stabilized:
//  - https://doc.rust-lang.org/stable/core/error/trait.Error.html#method.sources
//  - https://doc.rust-lang.org/std/error/struct.Report.html
fn format_error<T: Error>(err: T) -> Result<String, anyhow::Error> {
    let mut s = String::new();

    write!(s, "{err}")?;
    let mut next_source = err.source();
    while let Some(source) = next_source {
        write!(s, ": {source}")?;
        next_source = source.source();
    }

    Ok(s)
}
