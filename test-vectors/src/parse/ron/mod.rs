//! The ron module provides parsing logic using the `ron` crate.

use ron::{Options, extensions::Extensions};

pub type ParseError = ron::error::SpannedError;

/// Returns the configured ron parser with extensions enabled.
pub fn parser() -> Options {
    Options::default().with_default_extension(
        Extensions::IMPLICIT_SOME
            | Extensions::UNWRAP_NEWTYPES
            | Extensions::UNWRAP_VARIANT_NEWTYPES,
    )
}
