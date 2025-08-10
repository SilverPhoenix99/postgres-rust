/// Alias: `opt_interval`
pub fn interval(ctx: &mut ParserContext) -> scan::Result<IntervalRange> {

    /*
          YEAR
        | YEAR TO MONTH
        | MONTH
        | DAY
        | DAY TO HOUR
        | DAY TO MINUTE
        | DAY TO SECOND ( '(' ICONST ')' )?
        | HOUR
        | HOUR TO MINUTE
        | HOUR TO SECOND ( '(' ICONST ')' )?
        | MINUTE
        | MINUTE TO SECOND ( '(' ICONST ')' )?
        | SECOND ( '(' ICONST ')' )?
    */

    alt!(
        year,
        Kw::Month.map(|_| Month),
        day,
        hour,
        minute,
        interval_second
            .map(|precision| Second { precision }),
    ).parse(ctx)
}

fn year(ctx: &mut ParserContext) -> scan::Result<IntervalRange> {

    /*
          YEAR
        | YEAR TO MONTH
    */

    let (_, interval) = seq!(
        Kw::Year,
        seq!(To, Kw::Month).optional()
    ).parse(ctx)?;

    let interval = if interval.is_some() { YearToMonth } else { Year };
    Ok(interval)
}

fn day(ctx: &mut ParserContext) -> scan::Result<IntervalRange> {

    /*
          DAY
        | DAY TO HOUR
        | DAY TO MINUTE
        | DAY TO SECOND ( '(' ICONST ')' )?
    */

    let (_, interval) = seq!(
        Kw::Day,
        seq!(
            To,
            alt!(
                Kw::Hour.map(|_| DayToHour),
                Kw::Minute.map(|_| DayToMinute),
                interval_second
                    .map(|precision| DayToSecond { precision })
            )
        )
            .map(|(_, interval)| interval)
            .optional()
    ).parse(ctx)?;

    let interval = interval.unwrap_or(Day);
    Ok(interval)
}

fn hour(ctx: &mut ParserContext) -> scan::Result<IntervalRange> {

    /*
          HOUR
        | HOUR TO MINUTE
        | HOUR TO SECOND ( '(' ICONST ')' )?
    */

    let (_, interval) = seq!(
        Kw::Hour,
        seq!(
            To,
            alt!(
                Kw::Minute.map(|_| HourToMinute),
                interval_second
                    .map(|precision| HourToSecond { precision })
            )
        )
            .map(|(_, interval)| interval)
            .optional()
    ).parse(ctx)?;

    let interval = interval.unwrap_or(Hour);
    Ok(interval)
}

fn minute(ctx: &mut ParserContext) -> scan::Result<IntervalRange> {

    /*
          MINUTE
        | MINUTE TO SECOND ( '(' ICONST ')' )?
    */

    let (_, precision) = seq!(
        Kw::Minute,
        seq!(To, interval_second)
            .map(|(_, precision)| precision)
            .optional()
    ).parse(ctx)?;

    let precision = match precision {
        None => Minute,
        Some(precision) => MinuteToSecond { precision }
    };

    Ok(precision)
}

/// The `Option` result does not come from not matching the production rule.
///
/// It returns `None` when there's no precision after the `SECOND` keyword.
fn interval_second(ctx: &mut ParserContext) -> scan::Result<Option<i32>> {

    /*
        SECOND ( '(' ICONST ')' )?
    */

    let (_, precision) = seq!(Kw::Second, precision.optional())
        .parse(ctx)?;

    Ok(precision)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use pg_interval_ast::IntervalRange;
    use test_case::test_case;

    #[test_case("year",              IntervalRange::Year)]
    #[test_case("year to month",     IntervalRange::YearToMonth)]
    #[test_case("month",             IntervalRange::Month)]
    #[test_case("day",               IntervalRange::Day)]
    #[test_case("day to hour",       IntervalRange::DayToHour)]
    #[test_case("day to second",     IntervalRange::DayToSecond { precision: None })]
    #[test_case("day to second(7)",  IntervalRange::DayToSecond { precision: Some(7) })]
    #[test_case("hour",              IntervalRange::Hour)]
    #[test_case("hour to minute",    IntervalRange::HourToMinute)]
    #[test_case("hour to second",    IntervalRange::HourToSecond { precision: None })]
    #[test_case("hour to second(5)", IntervalRange::HourToSecond { precision: Some(5) })]
    #[test_case("second",            IntervalRange::Second { precision: None })]
    #[test_case("second(3)",         IntervalRange::Second { precision: Some(3) })]
    fn test_interval(source: &str, expected: IntervalRange) {
        test_parser!(source, interval, expected)
    }
}

use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_interval_ast::IntervalRange;
use pg_interval_ast::IntervalRange::Day;
use pg_interval_ast::IntervalRange::DayToHour;
use pg_interval_ast::IntervalRange::DayToMinute;
use pg_interval_ast::IntervalRange::DayToSecond;
use pg_interval_ast::IntervalRange::Hour;
use pg_interval_ast::IntervalRange::HourToMinute;
use pg_interval_ast::IntervalRange::HourToSecond;
use pg_interval_ast::IntervalRange::Minute;
use pg_interval_ast::IntervalRange::MinuteToSecond;
use pg_interval_ast::IntervalRange::Month;
use pg_interval_ast::IntervalRange::Second;
use pg_interval_ast::IntervalRange::Year;
use pg_interval_ast::IntervalRange::YearToMonth;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::To;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
use pg_sink_combinators::precision;
