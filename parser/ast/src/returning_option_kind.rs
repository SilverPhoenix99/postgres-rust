#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReturningOptionKind {
    Old(Str),
    New(Str),
}

use pg_basics::Str;
