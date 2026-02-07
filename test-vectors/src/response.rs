use core::fmt;
use serde::{Deserialize, Serialize};

/// An EncodedResponse represents expected data returned from the TPM in a
/// compact format useful for writing test vectors. Typically, this gets used
/// with the [`Response`] struct to evaluate it against actual TPM data.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct EncodedResponse(String);

impl EncodedResponse {
    /// Returns the length of the encoded response string.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len() / 2
    }

    /// Returns true if the response is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Decodes the hex-encoded response into bytes.
    pub fn to_bytes(&self) -> Result<Vec<u8>, hex::FromHexError> {
        hex::decode(&self.0)
    }
}

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

/// A Response represents expected data returned from the TPM.
#[derive(Debug)]
pub struct Response {
    /// The encoded format this response is constructed from.
    endcoded: EncodedResponse,
    /// The minimum length of actual data expected.
    min_len: usize,
}

impl Response {
    /// Create a new Response.
    pub fn new(encoded: EncodedResponse) -> Self {
        let min_len = encoded.len();
        Self {
            endcoded: encoded,
            min_len,
        }
    }
}

impl From<EncodedResponse> for Response {
    fn from(encoded: EncodedResponse) -> Self {
        Response::new(encoded)
    }
}
