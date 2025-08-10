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

use pg_basics::Str;
use pg_interval_ast::IntervalRange;
use pg_sink_ast::SignedNumber;
