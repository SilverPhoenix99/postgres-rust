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

use crate::IntervalRange;
use crate::SignedNumber;
use postgres_basics::Str;
