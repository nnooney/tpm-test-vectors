use super::*;
use crate::response::Part;

#[test]
fn test_parse_hex() {
    assert_eq!(parse_hex("01ab_cdef"), Ok(("", "01ab_cdef")));
    assert_eq!(parse_hex("0123"), Ok(("", "0123")));
    assert_eq!(parse_hex("aBcDeF"), Ok(("", "aBcDeF")));
    assert_eq!(parse_hex("1_2_3"), Ok(("", "1_2_3")));
    assert_eq!(parse_hex("1__2"), Ok(("", "1__2")));
    assert_eq!(parse_hex("12__"), Ok(("", "12__")));
    assert!(parse_hex("g").is_err());
    assert!(parse_hex("").is_err());
    assert!(parse_hex("_12").is_err());
    assert!(parse_hex("*").is_err());
    assert!(parse_hex("{").is_err());
    assert!(parse_hex("}").is_err());
}

#[test]
fn test_parse_hex_with_wildcards() {
    assert_eq!(parse_hex_with_wildcards("0123"), Ok(("", "0123")));
    assert_eq!(parse_hex_with_wildcards("abcdef"), Ok(("", "abcdef")));
    assert_eq!(parse_hex_with_wildcards("ABCDEF"), Ok(("", "ABCDEF")));
    assert_eq!(parse_hex_with_wildcards("0_1_2_3"), Ok(("", "0_1_2_3")));
    assert_eq!(parse_hex_with_wildcards("**"), Ok(("", "**")));
    assert_eq!(parse_hex_with_wildcards("A*"), Ok(("", "A*")));
    assert_eq!(parse_hex_with_wildcards("1"), Ok(("", "1")));
    assert_eq!(parse_hex_with_wildcards("123"), Ok(("", "123")));
    assert_eq!(parse_hex_with_wildcards("1_2_3"), Ok(("", "1_2_3")));
    assert_eq!(parse_hex_with_wildcards("aBcD"), Ok(("", "aBcD")));
    assert_eq!(parse_hex_with_wildcards("1*2*"), Ok(("", "1*2*")));
    assert_eq!(parse_hex_with_wildcards("*1*2"), Ok(("", "*1*2")));
    assert_eq!(parse_hex_with_wildcards("1__2"), Ok(("", "1__2")));
    assert_eq!(parse_hex_with_wildcards("12__"), Ok(("", "12__")));
    assert!(parse_hex_with_wildcards("g").is_err());
    assert!(parse_hex_with_wildcards("").is_err());
    assert!(parse_hex_with_wildcards("_12").is_err());
    assert!(parse_hex_with_wildcards("{").is_err());
    assert!(parse_hex_with_wildcards("}").is_err());
}

#[test]
fn test_parse_binary_with_wildcards() {
    assert_eq!(
        parse_binary_with_wildcards("0b00000000"),
        Ok(("", "00000000"))
    );
    assert_eq!(
        parse_binary_with_wildcards("0B00000001"),
        Ok(("", "00000001"))
    );
    assert_eq!(
        parse_binary_with_wildcards("0b00000101"),
        Ok(("", "00000101"))
    );
    assert_eq!(
        parse_binary_with_wildcards("0b1_0_1_0_0_0_0_0"),
        Ok(("", "1_0_1_0_0_0_0_0"))
    );
    assert_eq!(
        parse_binary_with_wildcards("0b********"),
        Ok(("", "********"))
    );
    assert_eq!(
        parse_binary_with_wildcards("0b1*******"),
        Ok(("", "1*******"))
    );
    assert_eq!(parse_binary_with_wildcards("0b1"), Ok(("", "1")));
    assert_eq!(
        parse_binary_with_wildcards("0b1111111"),
        Ok(("", "1111111"))
    );
    assert_eq!(parse_binary_with_wildcards("0b1_1"), Ok(("", "1_1")));
    assert_eq!(
        parse_binary_with_wildcards("0b1__1__1__1__1__1__1__1"),
        Ok(("", "1__1__1__1__1__1__1__1"))
    );
    assert_eq!(
        parse_binary_with_wildcards("0b11111111__"),
        Ok(("", "11111111__"))
    );
    assert!(parse_binary_with_wildcards("0b").is_err());
    assert!(parse_binary_with_wildcards("0b2").is_err());
    assert!(parse_binary_with_wildcards("0b_11111111").is_err());
    assert!(parse_binary_with_wildcards("0b{").is_err());
    assert!(parse_binary_with_wildcards("0b}").is_err());
}

#[test]
fn test_parse_binary_part() {
    assert_eq!(
        parse_binary_part("0b00000101"),
        Ok(("", Part::Binary("00000101", 8)))
    );
    assert_eq!(
        parse_binary_part("0b********"),
        Ok(("", Part::Binary("********", 8)))
    );
    assert_eq!(
        parse_binary_part("0b0000_0101"),
        Ok(("", Part::Binary("0000_0101", 8)))
    );
    assert!(parse_binary_part("0b1111111").is_err());
    assert!(parse_binary_part("0b1").is_err());
    assert!(parse_binary_part("0b1_1").is_err());
}

#[test]
fn test_parse_hex_part() {
    assert_eq!(parse_hex_part("01"), Ok(("", Part::Hex("01", 2))));
    assert_eq!(parse_hex_part("0123"), Ok(("", Part::Hex("0123", 4))));
    assert_eq!(parse_hex_part("01_23"), Ok(("", Part::Hex("01_23", 4))));
    assert!(parse_hex_part("1").is_err());
    assert!(parse_hex_part("123").is_err());
    assert!(parse_hex_part("1_2_3").is_err());
}

#[test]
fn test_parse_tpm2b_part() {
    assert_eq!(parse_tpm2b_part("TPM2B"), Ok(("", Part::TPM2B)));
    assert!(parse_tpm2b_part("XXXXX").is_err());
    assert!(parse_tpm2b_part("tpm2b").is_err());
}

#[test]
fn test_parse_expansion_control_sequence() {
    assert_eq!(
        parse_expansion_control_sequence("{TPM2B}"),
        Ok(("", Part::TPM2B))
    );
    assert_eq!(
        parse_expansion_control_sequence("{0b00000101}"),
        Ok(("", Part::Binary("00000101", 8)))
    );
    assert_eq!(
        parse_expansion_control_sequence("{0B********}"),
        Ok(("", Part::Binary("********", 8)))
    );
    assert!(parse_expansion_control_sequence("{TPM2B").is_err());
    assert!(parse_expansion_control_sequence("TPM2B}").is_err());
    assert!(parse_expansion_control_sequence("{0b}").is_err());
    assert!(parse_expansion_control_sequence("{}").is_err());
    assert!(parse_expansion_control_sequence("{invalid}").is_err());
    assert!(parse_expansion_control_sequence("{tpm2b}").is_err());
}

#[test]
fn test_parse_encoded_response() {
    assert_eq!(
        parse_encoded_response("0123"),
        Ok(vec![Part::Hex("0123", 4)])
    );
    assert_eq!(parse_encoded_response("{TPM2B}"), Ok(vec![Part::TPM2B]));
    assert_eq!(
        parse_encoded_response("{0b00000101}"),
        Ok(vec![Part::Binary("00000101", 8)])
    );
    assert_eq!(
        parse_encoded_response("01{TPM2B}23"),
        Ok(vec![Part::Hex("01", 2), Part::TPM2B, Part::Hex("23", 2)])
    );
    assert_eq!(
        parse_encoded_response("{TPM2B}0123"),
        Ok(vec![Part::TPM2B, Part::Hex("0123", 4)])
    );
    assert_eq!(
        parse_encoded_response("0123{TPM2B}"),
        Ok(vec![Part::Hex("0123", 4), Part::TPM2B])
    );
    assert_eq!(
        parse_encoded_response("01{0b00000001}23"),
        Ok(vec![
            Part::Hex("01", 2),
            Part::Binary("00000001", 8),
            Part::Hex("23", 2)
        ])
    );
    assert_eq!(
        parse_encoded_response("01**23"),
        Ok(vec![Part::Hex("01**23", 6)])
    );
    assert_eq!(
        parse_encoded_response("{0b1*******}"),
        Ok(vec![Part::Binary("1*******", 8)])
    );
    assert_eq!(
        parse_encoded_response("01_23"),
        Ok(vec![Part::Hex("01_23", 4)])
    );
    assert_eq!(
        parse_encoded_response("{0b0000_0101}"),
        Ok(vec![Part::Binary("0000_0101", 8)])
    );
    assert_eq!(
        parse_encoded_response("01_23{0b0000_0001}"),
        Ok(vec![Part::Hex("01_23", 4), Part::Binary("0000_0001", 8)])
    );
    assert_eq!(
        parse_encoded_response("01{TPM2B}{0b00000001}23"),
        Ok(vec![
            Part::Hex("01", 2),
            Part::TPM2B,
            Part::Binary("00000001", 8),
            Part::Hex("23", 2)
        ])
    );
    assert!(parse_encoded_response("01g").is_err());
    assert!(parse_encoded_response("").is_err());
    assert!(parse_encoded_response("g").is_err());
    assert!(parse_encoded_response("{0b}").is_err());
    assert!(parse_encoded_response("{").is_err());
    assert!(parse_encoded_response("}").is_err());
    assert!(parse_encoded_response("01{02").is_err());
    assert!(parse_encoded_response("01}02").is_err());
}

#[test]
fn test_parse_encoded_input() {
    assert_eq!(parse_encoded_input("01_ab_cd_ef").unwrap(), "01_ab_cd_ef");
    assert_eq!(parse_encoded_input("01_ab_cd_ef_").unwrap(), "01_ab_cd_ef_");
    assert_eq!(parse_encoded_input("01abcdef").unwrap(), "01abcdef");
    assert!(parse_encoded_input("01_ab_gh_ij").is_err());
    assert!(parse_encoded_input("g").is_err());
    assert!(parse_encoded_input("").is_err());
    assert!(parse_encoded_input("*").is_err());
    assert!(parse_encoded_input("{").is_err());
    assert!(parse_encoded_input("}").is_err());
}
