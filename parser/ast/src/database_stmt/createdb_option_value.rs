#[derive(Debug, Clone, Eq, PartialEq, From)]
pub enum CreatedbOptionValue {
    Default,
    #[from]
    Boolean(bool),
    #[from(SignedNumber, i32)]
    Number(SignedNumber),
    #[from(Str, String, &'static str, Box<str>)]
    String(Str),
}

impl From<VarValue> for CreatedbOptionValue {
    fn from(value: VarValue) -> Self {
        match value {
            VarValue::Boolean(value) => Self::Boolean(value),
            VarValue::String(value) => Self::String(value),
            VarValue::Number(value) => Self::Number(value),
        }
    }
}

use crate::SignedNumber;
use crate::VarValue;
use derive_more::From;
use pg_basics::Str;
