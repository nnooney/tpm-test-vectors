/// Character that represents spaces in numeric sequences, used for making long
/// sequences more readable.
pub const SPACE: char = '_';

/// Character that represents a wildcard value, which will automatically match
/// the corresponding element of the response.
pub const WILDCARD: char = '*';

/// Character that represents the opening of an expansion control sequence.
pub const EXPANSION_START: char = '{';

/// Character that represents the closing of an expansion control sequence.
pub const EXPANSION_END: char = '}';

/// Tag representing a TPM2B.
pub const TPM2B: &str = "TPM2B";

/// Tag representing a Not comparison.
pub const NOT: &str = "!";

/// Tag representing a Less Than comparison.
pub const LESS_THAN: &str = "<";

/// Tag representing a Less Than or Equal to comparison.
pub const LESS_THAN_OR_EQUAL: &str = "<=";

/// Tag representing a Greater Than comparison.
pub const GREATER_THAN: &str = ">";

/// Tag representing a Greater Than or Equal to comparison.
pub const GREATER_THAN_OR_EQUAL: &str = ">=";
