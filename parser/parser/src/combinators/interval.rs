/// Alias: `opt_interval`
pub(super) fn interval(ctx: &mut ParserContext) -> scan::Result<IntervalRange> {

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
    use crate::test_parser;
    use test_case::test_matrix;

    #[test_matrix("year"              => Ok(Year))]
    #[test_matrix("year to month"     => Ok(YearToMonth))]
    #[test_matrix("month"             => Ok(Month))]
    #[test_matrix("day"               => Ok(Day))]
    #[test_matrix("day to hour"       => Ok(DayToHour))]
    #[test_matrix("day to second"     => Ok(DayToSecond { precision: None }))]
    #[test_matrix("day to second(7)"  => Ok(DayToSecond { precision: Some(7) }))]
    #[test_matrix("hour"              => Ok(Hour))]
    #[test_matrix("hour to minute"    => Ok(HourToMinute))]
    #[test_matrix("hour to second"    => Ok(HourToSecond { precision: None }))]
    #[test_matrix("hour to second(5)" => Ok(HourToSecond { precision: Some(5) }))]
    #[test_matrix("second"            => Ok(Second { precision: None }))]
    #[test_matrix("second(3)"         => Ok(Second { precision: Some(3) }))]
    fn test_interval(source: &str) -> scan::Result<IntervalRange> {
        test_parser!(source, interval)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::precision;
use crate::seq;
use crate::ParserContext;
use pg_ast::IntervalRange;
use pg_ast::IntervalRange::Day;
use pg_ast::IntervalRange::DayToHour;
use pg_ast::IntervalRange::DayToMinute;
use pg_ast::IntervalRange::DayToSecond;
use pg_ast::IntervalRange::Hour;
use pg_ast::IntervalRange::HourToMinute;
use pg_ast::IntervalRange::HourToSecond;
use pg_ast::IntervalRange::Minute;
use pg_ast::IntervalRange::MinuteToSecond;
use pg_ast::IntervalRange::Month;
use pg_ast::IntervalRange::Second;
use pg_ast::IntervalRange::Year;
use pg_ast::IntervalRange::YearToMonth;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::To;
use pg_parser_core::scan;
