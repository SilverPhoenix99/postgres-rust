#[derive(Debug, Clone, Eq, PartialEq, From)]
pub enum VarValue {
    #[from]
    Boolean(bool),
    #[from(SignedNumber, i32)]
    Number(SignedNumber),
    #[from(Str, String, &'static str, Box<str>)]
    String(Str),
}

impl From<BooleanOrString> for VarValue {
    fn from(value: BooleanOrString) -> Self {
        match value {
            BooleanOrString::Boolean(value) => Self::Boolean(value),
            BooleanOrString::String(value) => Self::String(value),
        }
    }
}

use crate::BooleanOrString;
use crate::SignedNumber;
use derive_more::From;
use pg_basics::Str;
