pub(super) fn opt_interval(stream: &mut TokenStream) -> scan::Result<IntervalRange> {

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
        | /* EMPTY */
    */

    let interval = or((
        year,
        MonthKw.map(|_| Month),
        day,
        hour,
        minute,
        (SecondKw, opt_precision)
            .map(|(_, precision)| Second { precision }),
    )).parse(stream);

    let interval = interval.optional()?;

    Ok(interval.unwrap_or_default())
}

fn year(stream: &mut TokenStream) -> scan::Result<IntervalRange> {

    /*
          YEAR
        | YEAR TO MONTH
    */

    let (_, interval) = (
        YearKw,
        (To, MonthKw).optional()
    ).parse(stream)?;

    let interval = if interval.is_some() { YearToMonth } else { Year };
    Ok(interval)
}

fn day(stream: &mut TokenStream) -> scan::Result<IntervalRange> {

    /*
          DAY
        | DAY TO HOUR
        | DAY TO MINUTE
        | DAY TO SECOND ( '(' ICONST ')' )?
    */

    let (_, interval) = (
        DayKw,
        (
            To,
            or((
                HourKw.map(|_| DayToHour),
                MinuteKw.map(|_| DayToMinute),
                (SecondKw, opt_precision)
                    .map(|(_, precision)| DayToSecond { precision })
            ))
        )
            .map(|(_, interval)| interval)
            .optional()
    ).parse(stream)?;

    let interval = interval.unwrap_or(Day);
    Ok(interval)
}

fn hour(stream: &mut TokenStream) -> scan::Result<IntervalRange> {

    /*
          HOUR
        | HOUR TO MINUTE
        | HOUR TO SECOND ( '(' ICONST ')' )?
    */

    let (_, interval) = (
        HourKw,
        (
            To,
            or((
                MinuteKw.map(|_| HourToMinute),
                (SecondKw, opt_precision)
                    .map(|(_, precision)| HourToSecond { precision })
            ))
        )
            .map(|(_, interval)| interval)
            .optional()
    ).parse(stream)?;

    let interval = interval.unwrap_or(Hour);
    Ok(interval)
}

fn minute(stream: &mut TokenStream) -> scan::Result<IntervalRange> {

    /*
          MINUTE
        | MINUTE TO SECOND ( '(' ICONST ')' )?
    */

    let (_, precision) = (
        MinuteKw,
        (To, SecondKw, opt_precision)
            .map(|(.., precision)| precision)
            .optional()
    ).parse(stream)?;

    let precision = match precision {
        None => Minute,
        Some(precision) => MinuteToSecond { precision }
    };

    Ok(precision)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("",                  IntervalRange::default())]
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
    fn test_opt_interval(source: &str, expected: IntervalRange) {
        test_parser!(source, opt_interval, expected)
    }
}

use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::opt_precision;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
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
use pg_lexer::Keyword::Day as DayKw;
use pg_lexer::Keyword::Hour as HourKw;
use pg_lexer::Keyword::Minute as MinuteKw;
use pg_lexer::Keyword::Month as MonthKw;
use pg_lexer::Keyword::Second as SecondKw;
use pg_lexer::Keyword::To;
use pg_lexer::Keyword::Year as YearKw;
