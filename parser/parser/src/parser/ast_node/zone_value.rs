#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ZoneValue {
    /// Alias: `Default`
    Local,
    String(Str),
    Numeric(SignedNumber),
    Interval {
        value: Box<str>,
        range: IntervalRange
    },
}

use crate::parser::ast_node::IntervalRange;
use crate::parser::ast_node::SignedNumber;
use postgres_basics::Str;
