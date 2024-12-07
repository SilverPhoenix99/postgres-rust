#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VarValue {
    Boolean(bool),
    Number(SignedNumber),
    String(Str),
}

impl_from!(SignedNumber for VarValue::Number);
impl_from!(Str for VarValue::String);
impl_from!(String for VarValue::String);
impl_from!(bool for VarValue::Boolean);
impl_from!(i32 for VarValue::Number);

impl From<&'static str> for VarValue {
    fn from(value: &'static str) -> Self {
        Self::String(value.into())
    }
}

impl From<Box<str>> for VarValue {
    fn from(value: Box<str>) -> Self {
        Self::String(value.into())
    }
}

impl From<BooleanOrString> for VarValue {
    fn from(value: BooleanOrString) -> Self {
        match value {
            BooleanOrString::Boolean(value) => Self::Boolean(value),
            BooleanOrString::String(value) => Self::String(value),
        }
    }
}

use crate::parser::ast_node::BooleanOrString;
use crate::parser::ast_node::SignedNumber;
use postgres_basics::impl_from;
use postgres_basics::Str;
