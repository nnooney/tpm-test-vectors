use super::*;
use crate::response::Part;

use nom::{Err, error::Error, error::ErrorKind};

#[test]
fn test_parse_tpm2b() {
    assert_eq!(parse_tpm2b("TPM2B"), Ok(("", Part::TPM2B)));
    assert_eq!(
        parse_tpm2b("XXXXX"),
        Err(Err::Error(Error::new("XXXXX", ErrorKind::Tag)))
    );
}

#[test]
fn test_parse_binary() {
    assert_eq!(
        parse_binary("0b00000000"),
        Ok(("", Part::Binary("00000000", 8)))
    );
    assert_eq!(
        parse_binary("0B00000001"),
        Ok(("", Part::Binary("00000001", 8)))
    );
    assert_eq!(
        parse_binary("0b00000101"),
        Ok(("", Part::Binary("00000101", 8)))
    );
    assert_eq!(
        parse_binary("0b1_0_1_0_0_0_0_0"),
        Ok(("", Part::Binary("1_0_1_0_0_0_0_0", 8)))
    );
    assert_eq!(
        parse_binary("0b********"),
        Ok(("", Part::Binary("********", 8)))
    );
    assert_eq!(
        parse_binary("0b1*******"),
        Ok(("", Part::Binary("1*******", 8)))
    );
    assert_eq!(
        parse_binary("0b"),
        Err(Err::Error(Error::new("", ErrorKind::OneOf)))
    );
    assert_eq!(
        parse_binary("0b2"),
        Err(Err::Error(Error::new("2", ErrorKind::OneOf)))
    );
    assert_eq!(
        parse_binary("0b1"),
        Err(Err::Error(Error::new("1", ErrorKind::Verify)))
    );
    assert_eq!(
        parse_binary("0b1111111"),
        Err(Err::Error(Error::new("1111111", ErrorKind::Verify)))
    );
    assert_eq!(
        parse_binary("0b1_1"),
        Err(Err::Error(Error::new("1_1", ErrorKind::Verify)))
    );
    assert_eq!(
        parse_binary("0b1__1__1__1__1__1__1__1"),
        Ok(("", Part::Binary("1__1__1__1__1__1__1__1", 8)))
    );
    assert_eq!(
        parse_binary("0b11111111__"),
        Ok(("", Part::Binary("11111111__", 8)))
    );
    assert_eq!(
        parse_binary("0b_11111111"),
        Err(Err::Error(Error::new("_11111111", ErrorKind::OneOf)))
    );
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
    assert_eq!(
        parse_expansion_control_sequence("{TPM2B"),
        Err(Err::Error(Error::new("", ErrorKind::Char)))
    );
    assert_eq!(
        parse_expansion_control_sequence("TPM2B}"),
        Err(Err::Error(Error::new("TPM2B}", ErrorKind::Char)))
    );
    assert_eq!(
        parse_expansion_control_sequence("{0b}"),
        Err(Err::Error(Error::new("}", ErrorKind::OneOf)))
    );
    assert_eq!(
        parse_expansion_control_sequence("{}"),
        Err(Err::Error(Error::new("}", ErrorKind::Tag)))
    );
    assert_eq!(
        parse_expansion_control_sequence("{invalid}"),
        Err(Err::Error(Error::new("invalid}", ErrorKind::Tag)))
    );
    assert_eq!(
        parse_expansion_control_sequence("{tpm2b}"),
        Err(Err::Error(Error::new("tpm2b}", ErrorKind::Tag)))
    );
}

#[test]
fn test_parse_hexadecimal() {
    assert_eq!(parse_hexadecimal("0123"), Ok(("", Part::Hex("0123", 4))));
    assert_eq!(
        parse_hexadecimal("abcdef"),
        Ok(("", Part::Hex("abcdef", 6)))
    );
    assert_eq!(
        parse_hexadecimal("ABCDEF"),
        Ok(("", Part::Hex("ABCDEF", 6)))
    );
    assert_eq!(
        parse_hexadecimal("0_1_2_3"),
        Ok(("", Part::Hex("0_1_2_3", 4)))
    );
    assert_eq!(parse_hexadecimal("**"), Ok(("", Part::Hex("**", 2))));
    assert_eq!(parse_hexadecimal("A*"), Ok(("", Part::Hex("A*", 2))));
    assert_eq!(
        parse_hexadecimal("g"),
        Err(Err::Error(Error::new("g", ErrorKind::OneOf)))
    );
    assert_eq!(
        parse_hexadecimal(""),
        Err(Err::Error(Error::new("", ErrorKind::OneOf)))
    );
    assert_eq!(
        parse_hexadecimal("1"),
        Err(Err::Error(Error::new("1", ErrorKind::Verify)))
    );
    assert_eq!(
        parse_hexadecimal("123"),
        Err(Err::Error(Error::new("123", ErrorKind::Verify)))
    );
    assert_eq!(
        parse_hexadecimal("1_2_3"),
        Err(Err::Error(Error::new("1_2_3", ErrorKind::Verify)))
    );
    assert_eq!(parse_hexadecimal("aBcD"), Ok(("", Part::Hex("aBcD", 4))));
    assert_eq!(parse_hexadecimal("1*2*"), Ok(("", Part::Hex("1*2*", 4))));
    assert_eq!(parse_hexadecimal("*1*2"), Ok(("", Part::Hex("*1*2", 4))));
    assert_eq!(parse_hexadecimal("1__2"), Ok(("", Part::Hex("1__2", 2))));
    assert_eq!(parse_hexadecimal("12__"), Ok(("", Part::Hex("12__", 2))));
    assert_eq!(
        parse_hexadecimal("_12"),
        Err(Err::Error(Error::new("_12", ErrorKind::OneOf)))
    );
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
        parse_encoded_response("01g"),
        Err(Err::Error(Error::new("g", ErrorKind::Eof)))
    );
    assert_eq!(
        parse_encoded_response(""),
        Err(Err::Error(Error::new("", ErrorKind::Char)))
    );
    assert_eq!(
        parse_encoded_response("g"),
        Err(Err::Error(Error::new("g", ErrorKind::Char)))
    );
    assert_eq!(
        parse_encoded_response("{0b}"),
        Err(Err::Error(Error::new("}", ErrorKind::OneOf)))
    );
}
