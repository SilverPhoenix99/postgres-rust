pub(super) fn interval_type(ctx: &mut ParserContext) -> scan::Result<IntervalRange> {

    /*
          INTERVAL '(' ICONST ')'
        | INTERVAL ( interval )?
    */

    let (_, interval) = seq!(
        Kw::Interval,
        alt!(
            precision
                .map(|precision| Full { precision: Some(precision) }),
            interval.optional()
                .map(Option::unwrap_or_default)
        )
    ).parse(ctx)?;

    Ok(interval)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_case;

    #[test_case("interval"     => Ok(IntervalRange::default()))]
    #[test_case("interval day" => Ok(IntervalRange::Day))]
    #[test_case("interval(5)"  => Ok(IntervalRange::Full { precision: Some(5) }))]
    fn test_interval_type(source: &str) -> scan::Result<IntervalRange> {
        test_parser!(source, interval_type)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::interval;
use crate::combinators::precision;
use crate::seq;
use crate::ParserContext;
use pg_ast::IntervalRange;
use pg_ast::IntervalRange::Full;
use pg_lexer::Keyword as Kw;
use pg_parser_core::scan;
