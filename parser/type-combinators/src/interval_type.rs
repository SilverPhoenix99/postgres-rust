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
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("interval"     => Ok(IntervalRange::default()))]
    #[test_case("interval day" => Ok(IntervalRange::Day))]
    #[test_case("interval(5)"  => Ok(IntervalRange::Full { precision: Some(5) }))]
    fn test_interval_type(source: &str) -> scan::Result<IntervalRange> {
        test_parser!(source, interval_type)
    }
}

use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_interval_ast::IntervalRange;
use pg_interval_ast::IntervalRange::Full;
use pg_interval_combinators::interval;
use pg_lexer::Keyword as Kw;
use pg_parser_core::scan;
use pg_sink_combinators::precision;
