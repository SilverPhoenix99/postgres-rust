#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QualifiedOperator(pub Vec<Str>, pub Operator);

impl From<Operator> for QualifiedOperator {
    fn from(value: Operator) -> Self {
        Self(vec![], value)
    }
}

use crate::Operator;
use postgres_basics::Str;
