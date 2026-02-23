use super::*;

#[test]
fn test_parse_hexadecimal() {
    let input = "01_ab_cd_ef";
    let (remaining, parsed) = parse_hexadecimal(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(parsed, "01_ab_cd_ef");
}

#[test]
fn test_parse_encoded_input() {
    let input = "01_ab_cd_ef";
    let parsed = parse_encoded_input(input).unwrap();
    assert_eq!(parsed, "01_ab_cd_ef");
}

#[test]
fn test_parse_encoded_input_with_trailing_space() {
    let input = "01_ab_cd_ef_";
    let parsed = parse_encoded_input(input).unwrap();
    assert_eq!(parsed, "01_ab_cd_ef_");
}

#[test]
fn test_parse_encoded_input_invalid() {
    let input = "01_ab_gh_ij";
    let result = parse_encoded_input(input);
    assert!(result.is_err());
}
