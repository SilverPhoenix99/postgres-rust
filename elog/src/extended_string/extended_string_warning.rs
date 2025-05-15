#[derive(Debug, Copy, Clone, Eq, PartialEq, thiserror::Error)]
pub enum ExtendedStringWarning {

    #[error("nonstandard use of escape in a string literal")]
    NonstandardEscape,

    #[error(r"nonstandard use of \' in a string literal")]
    NonstandardQuoteEscape,

    #[error(r"nonstandard use of \\ in a string literal")]
    NonstandardBackslashEscape,
}

impl ErrorReport for ExtendedStringWarning {
    fn hint(&self) -> Option<Cow<'static, str>> {
        match self {
            Self::NonstandardEscape => Some(r"Use the escape string syntax for escapes, e.g., E'\r\n'.".into()),
            Self::NonstandardQuoteEscape => {
                Some("Use '' to write quotes in strings, or use the escape string syntax (E'...').".into())
            },
            Self::NonstandardBackslashEscape => {
                Some(r"Use the escape string syntax for backslashes, e.g., E'\\'.".into())
            },
        }
    }
}

use std::borrow::Cow;
use crate::ErrorReport;
