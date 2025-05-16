#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BooleanOrString {
    Boolean(bool),
    String(Str)
}

impl_from!(bool for BooleanOrString::Boolean);
impl_from!(Str for BooleanOrString::String);
impl_from!(String for BooleanOrString);

impl From<&'static str> for BooleanOrString {
    fn from(value: &'static str) -> Self {
        Self::String(value.into())
    }
}

impl From<Box<str>> for BooleanOrString {
    fn from(value: Box<str>) -> Self {
        Self::String(value.into())
    }
}

use pg_basics::impl_from;
use pg_basics::Str;
