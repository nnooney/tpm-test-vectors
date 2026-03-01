//! A response is an encoded string describing the expected response from a
//! TPM command.

use core::error::Error;
use core::fmt;
use serde::{Deserialize, Serialize};

use crate::parse::consts::*;
use crate::parse::{self, ParseError};

/// Errors returned from evaluating a response.
#[derive(Debug)]
pub enum ResponseEvaluationError {
    /// Parse Error
    #[non_exhaustive]
    Parse(ParseError),
    /// Response is too short (`want`, `got`)
    #[non_exhaustive]
    ResponseTooShort(usize, usize),
    /// Invalid Part (`part.expected`, `part.count`)
    #[non_exhaustive]
    InvalidPart(String, usize),
    /// Part extends beyond end of response (`want`, `got`)
    #[non_exhaustive]
    PartExtendsBeyondResponse(usize, usize),
    /// Part mismatches actual data (`want`, `got`, `index`, `chars_per_byte`,
    /// `matched`)
    #[non_exhaustive]
    PartMismatch(String, String, usize, usize, usize),
    /// Part compare failure (`want`, `got`, `matched`)
    #[non_exhaustive]
    PartCompareFailure(String, String, usize),
}

impl fmt::Display for ResponseEvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Parse(ref _err) => write!(f, "error parsing encoded response"),
            Self::ResponseTooShort(ref want, ref got) => write!(
                f,
                r#"
TPM response too short
  want: >= {want}
   got: {got}"#
            ),
            Self::InvalidPart(ref expected, ref count) => {
                write!(f, "invalid response part: \"{expected}\"")?;
                if *count > 0 {
                    write!(f, ": {count} bytes to match")?;
                }
                Ok(())
            }
            Self::PartExtendsBeyondResponse(ref want, ref got) => write!(
                f,
                r#"
TPM response part extends beyond end of response
  want: >= {want}
   got: {got}"#
            ),
            Self::PartMismatch(ref want, ref got, ref index, ref chars_per_byte, ref matched) => {
                let prelude = if *matched == 0 { "" } else { "..." };
                let prelude_match = if *matched == 0 {
                    ""
                } else {
                    &format!("^^^ omitted {matched} matched bytes")
                };
                let pad = ' ';
                let width = *index + prelude.len();
                let total = index / chars_per_byte + 1 + matched;
                write!(
                    f,
                    r#"
TPM response part mismatch
  want: {prelude}{want}
   got: {prelude}{got}
        {pad:width$}^ mismatch begins in byte {total}
        {prelude_match}"#,
                )
            }
            Self::PartCompareFailure(ref want, ref got, ref matched) => {
                let prelude = if *matched == 0 { "" } else { "..." };
                write!(
                    f,
                    r#"
TPM response part compare failure
  want: {prelude}{want}
   got: {prelude}{got}
"#
                )
            }
        }
    }
}

impl Error for ResponseEvaluationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::Parse(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<ParseError> for ResponseEvaluationError {
    fn from(err: ParseError) -> Self {
        Self::Parse(err)
    }
}

/// An EncodedResponse represents expected data returned from the TPM in a
/// compact format useful for writing test vectors. Typically, this gets used
/// with the [`Response`] struct to evaluate it against actual TPM data.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct EncodedResponse(String);

impl fmt::Display for EncodedResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for EncodedResponse {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<&str> for EncodedResponse {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for EncodedResponse {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Internal struct to keep track of how much of an actual response is checked.
/// It deirectly references the actual TPM response it was constructed from.
struct PartialMatch<'a> {
    /// Number of bytes in the actual response matched so far.
    matched: usize,
    /// The remaining string to match.
    remaining: &'a str,
}

/// Helper to split the data part to be checked from the rest of the data.
fn split_data_part<'a>(
    data: &PartialMatch<'a>,
    num_hex_chars: usize,
) -> Result<(&'a str, &'a str), ResponseEvaluationError> {
    if data.remaining.len() < num_hex_chars {
        return Err(ResponseEvaluationError::PartExtendsBeyondResponse(
            num_hex_chars,
            data.remaining.len(),
        ));
    }
    Ok(data.remaining.split_at(num_hex_chars))
}

/// Helper function to convert a hex string to a binary string, ignoring
/// non-hexadecimal characters.
fn hex_string_to_binary(hex_string: &str) -> String {
    let mut result = String::new();
    for char in hex_string.chars() {
        match char {
            '0' => result.push_str("0000"),
            '1' => result.push_str("0001"),
            '2' => result.push_str("0010"),
            '3' => result.push_str("0011"),
            '4' => result.push_str("0100"),
            '5' => result.push_str("0101"),
            '6' => result.push_str("0110"),
            '7' => result.push_str("0111"),
            '8' => result.push_str("1000"),
            '9' => result.push_str("1001"),
            'a' | 'A' => result.push_str("1010"),
            'b' | 'B' => result.push_str("1011"),
            'c' | 'C' => result.push_str("1100"),
            'd' | 'D' => result.push_str("1101"),
            'e' | 'E' => result.push_str("1110"),
            'f' | 'F' => result.push_str("1111"),
            _ => {}
        }
    }
    result
}

/// Helper function to get a u16 from a big-endian hex string, ignoring
/// non-hexadecimal characters.
fn hex_string_to_u16(hex_string: &str) -> u16 {
    let mut result: u16 = 0;
    for (i, char) in hex_string.chars().rev().enumerate() {
        match char {
            '0' => result |= 0 << (i * 4),
            '1' => result |= 1 << (i * 4),
            '2' => result |= 2 << (i * 4),
            '3' => result |= 3 << (i * 4),
            '4' => result |= 4 << (i * 4),
            '5' => result |= 5 << (i * 4),
            '6' => result |= 6 << (i * 4),
            '7' => result |= 7 << (i * 4),
            '8' => result |= 8 << (i * 4),
            '9' => result |= 9 << (i * 4),
            'a' | 'A' => result |= 10 << (i * 4),
            'b' | 'B' => result |= 11 << (i * 4),
            'c' | 'C' => result |= 12 << (i * 4),
            'd' | 'D' => result |= 13 << (i * 4),
            'e' | 'E' => result |= 14 << (i * 4),
            'f' | 'F' => result |= 15 << (i * 4),
            _ => {}
        }
    }
    result
}

/// A type of comparison to perform.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompareType {
    Not,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

impl fmt::Display for CompareType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Not => write!(f, "{NOT}"),
            Self::LessThan => write!(f, "{LESS_THAN}"),
            Self::LessThanOrEqual => write!(f, "{LESS_THAN_OR_EQUAL}"),
            Self::GreaterThan => write!(f, "{GREATER_THAN}"),
            Self::GreaterThanOrEqual => write!(f, "{GREATER_THAN_OR_EQUAL}"),
        }
    }
}

/// A Part represents a subsequence of the total response with specific
/// semantics for matching data against the TPM. It directly references the
/// [`EncodedResponse`] it was parsed from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Part<'a> {
    /// Hexadecimal bytes with wildcards
    Hex(&'a str, usize),
    /// Binary bits with wildcards
    Binary(&'a str, usize),
    /// A TPM2B in the response (big-endian u16 length followed by length bytes)
    TPM2B,
    /// Compare against a known hexadecimal value
    Compare(CompareType, &'a str, usize),
}

impl<'a> Part<'a> {
    /// Helper constructor for making a Not Compare part.
    pub fn not(expected: &'a str, count: usize) -> Self {
        Self::Compare(CompareType::Not, expected, count)
    }

    /// Helper constructor for making a Less Than Compare part.
    pub fn lt(expected: &'a str, count: usize) -> Self {
        Self::Compare(CompareType::LessThan, expected, count)
    }

    /// Helper constructor for making a Less Than Or Equal Compare part.
    pub fn le(expected: &'a str, count: usize) -> Self {
        Self::Compare(CompareType::LessThanOrEqual, expected, count)
    }

    /// Helper constructor for making a Greater Than Compare part.
    pub fn gt(expected: &'a str, count: usize) -> Self {
        Self::Compare(CompareType::GreaterThan, expected, count)
    }

    /// Helper constructor for making a Greater Than Or Equal Compare part.
    pub fn ge(expected: &'a str, count: usize) -> Self {
        Self::Compare(CompareType::GreaterThanOrEqual, expected, count)
    }

    /// Returns the minimum length required in the response to match the part.
    #[must_use]
    pub fn min_len(&self) -> usize {
        match self {
            Self::Hex(_, count) => count / 2,
            Self::Binary(_, count) => count / 8,
            Self::TPM2B => 2, // must encode at least a u16 size
            Self::Compare(_, _, count) => count / 2,
        }
    }

    /// Checks the part against actual `data` (formated as a hexadecimal
    /// string)
    fn check(&self, data: PartialMatch<'a>) -> Result<PartialMatch<'a>, ResponseEvaluationError> {
        match self {
            Self::Hex(expected, count) => {
                let num_hex_chars = *count;
                let (data_part, data_rest) = split_data_part(&data, num_hex_chars)?;

                let expected_chars = expected.chars().filter(|&c| c != SPACE);

                for (i, (want, got)) in expected_chars.clone().zip(data_part.chars()).enumerate() {
                    if want != WILDCARD && !want.eq_ignore_ascii_case(&got) {
                        return Err(ResponseEvaluationError::PartMismatch(
                            expected.replace(SPACE, ""),
                            data_part.to_string(),
                            i,
                            2, // chars_per_byte
                            data.matched,
                        ));
                    }
                }

                Ok(PartialMatch {
                    matched: data.matched + num_hex_chars / 2,
                    remaining: data_rest,
                })
            }
            Self::Binary(expected, count) => {
                // count is number of binary characters (bits).
                let num_hex_chars = count / 4;
                let (data_part, data_rest) = split_data_part(&data, num_hex_chars)?;
                let binary_part = hex_string_to_binary(data_part);

                let expected_chars = expected.chars().filter(|&c| c != SPACE);

                for (i, (want, got)) in expected_chars.zip(binary_part.chars()).enumerate() {
                    if want != WILDCARD && want != got {
                        return Err(ResponseEvaluationError::PartMismatch(
                            expected.replace(SPACE, ""),
                            binary_part,
                            i,
                            8, // chars_per_byte
                            data.matched,
                        ));
                    }
                }

                Ok(PartialMatch {
                    matched: data.matched + num_hex_chars / 2,
                    remaining: data_rest,
                })
            }
            Self::TPM2B => {
                // A TPM2B is a u16 size followed by size bytes.
                const U16_HEX_CHARS: usize = 4;
                if data.remaining.len() < U16_HEX_CHARS {
                    return Err(ResponseEvaluationError::PartExtendsBeyondResponse(
                        U16_HEX_CHARS,
                        data.remaining.len(),
                    ));
                }

                let (size_hex, rest) = data.remaining.split_at(U16_HEX_CHARS);
                let num_hex_chars = (hex_string_to_u16(size_hex) as usize) * 2;

                if rest.len() < num_hex_chars {
                    return Err(ResponseEvaluationError::PartExtendsBeyondResponse(
                        num_hex_chars,
                        rest.len(),
                    ));
                }
                let (_data_part, data_rest) =
                    data.remaining.split_at(U16_HEX_CHARS + num_hex_chars);

                Ok(PartialMatch {
                    matched: data.matched + (U16_HEX_CHARS + num_hex_chars) / 2,
                    remaining: data_rest,
                })
            }
            Self::Compare(compare_type, expected, count) => {
                let num_hex_chars = *count;
                let (data_part, data_rest) = split_data_part(&data, num_hex_chars)?;

                let expected_chars = expected
                    .chars()
                    .filter(|&c| c != SPACE)
                    .map(|c| c.to_ascii_lowercase());
                let data_iter = data_part.chars().map(|c| c.to_ascii_lowercase());

                let matched = match compare_type {
                    CompareType::Not => data_iter.ne(expected_chars),
                    CompareType::LessThan => data_iter.lt(expected_chars),
                    CompareType::LessThanOrEqual => data_iter.le(expected_chars),
                    CompareType::GreaterThan => data_iter.gt(expected_chars),
                    CompareType::GreaterThanOrEqual => data_iter.ge(expected_chars),
                };

                if !matched {
                    return Err(ResponseEvaluationError::PartCompareFailure(
                        expected.replace(SPACE, ""),
                        data_part.to_string(),
                        data.matched,
                    ));
                }

                Ok(PartialMatch {
                    matched: data.matched + num_hex_chars / 2,
                    remaining: data_rest,
                })
            }
        }
    }
}

impl<'a> fmt::Display for Part<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hex(s, c) => write!(f, "Part \"{s}\": {c} bytes to match"),
            Self::Binary(s, c) => write!(f, "Part \"{s}\": {c} bytes to match"),
            Self::TPM2B => write!(f, "TPM2B"),
            Self::Compare(t, s, c) => write!(f, "Compare {t} \"{s}\": {c} bytes to match"),
        }
    }
}

/// A Response represents expected data returned from the TPM.
#[derive(Debug)]
pub struct Response<'a> {
    /// The encoded format this response is constructed from, owned for lifetime
    /// referencing the parts.
    _encoded: &'a EncodedResponse,
    /// The minimum length of actual data expected, computed based on
    /// `_encoded`.
    min_len: usize,
    /// The parts of the response.
    parts: Vec<Part<'a>>,
}

impl<'a> Response<'a> {
    /// Create a new Response.
    pub fn new(encoded: &'a EncodedResponse, parts: Vec<Part<'a>>) -> Self {
        let min_len = parts.iter().map(|p| p.min_len()).sum();
        Self {
            _encoded: encoded,
            min_len,
            parts,
        }
    }

    /// Returns the minimum length of TPM data needed for this response to
    /// evaluate.
    #[must_use]
    pub fn min_len(&self) -> usize {
        self.min_len
    }

    /// Check the response against actual `data` (formated as a hexadecimal
    /// string) returned from the TPM.
    pub fn check(&self, data: &str) -> Result<(), ResponseEvaluationError> {
        if data.len() < self.min_len {
            return Err(ResponseEvaluationError::ResponseTooShort(
                self.min_len,
                data.len(),
            ));
        }

        let mut data = PartialMatch {
            matched: 0,
            remaining: data,
        };
        for part in &self.parts {
            data = part.check(data)?;
        }

        Ok(())
    }

    /// Convenience function to construct a Response and evaluate it.
    pub fn evaluate(encoded: &EncodedResponse, data: &[u8]) -> Result<(), ResponseEvaluationError> {
        let response = parse::response(encoded)?;
        response.check(&hex::encode(data))
    }
}
