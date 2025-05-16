pub(super) fn opt_interval() -> impl Combinator<Output = IntervalRange> {

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

    match_first! (
        year(),
        MonthKw.map(|_| Month),
        day(),
        hour(),
        minute(),
        SecondKw
            .and_right(opt_precision())
            .map(|precision| Second { precision }),
    )
        .optional()
        .map(Option::unwrap_or_default)
}

fn year() -> impl Combinator<Output = IntervalRange> {

    /*
          YEAR
        | YEAR TO MONTH
    */

    YearKw.and_right(
        To.and(MonthKw)
            .optional()
            .map(|y|
                if y.is_some() { YearToMonth } else { Year }
            )
    )
}

fn day() -> impl Combinator<Output = IntervalRange> {

    /*
          DAY
        | DAY TO HOUR
        | DAY TO MINUTE
        | DAY TO SECOND ( '(' ICONST ')' )?
    */

    DayKw
        .and_right(
            To.and_right(match_first! {
                HourKw.map(|_| DayToHour),
                MinuteKw.map(|_| DayToMinute),
                SecondKw.and_right(
                    opt_precision().map(|precision| DayToSecond { precision })
                )
            })
            .optional()
        )
        .map(|d| d.unwrap_or(Day))
}

fn hour() -> impl Combinator<Output = IntervalRange> {

    /*
          HOUR
        | HOUR TO MINUTE
        | HOUR TO SECOND ( '(' ICONST ')' )?
    */

    HourKw
        .and_right(
            To.and_right(match_first! {
                MinuteKw.map(|_| HourToMinute),
                SecondKw.and_right(
                    opt_precision().map(|precision| HourToSecond { precision })
                )
            })
            .optional()
        )
        .map(|h| h.unwrap_or(Hour))
}

fn minute() -> impl Combinator<Output = IntervalRange> {

    /*
          MINUTE
        | MINUTE TO SECOND ( '(' ICONST ')' )?
    */

    MinuteKw
        .and_right(
            sequence!(
                To.and(SecondKw).skip(),
                opt_precision()
            )
            .map(|(_, precision)| precision)
            .optional()
        ).map(|precision| match precision {
            None => Minute,
            Some(precision) => MinuteToSecond { precision }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
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

        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = opt_interval().parse(&mut stream);

        assert_eq!(
            Ok(expected),
            actual,
            r"expected {expected:?} for source {source:?} but actually got {actual:?}"
        );
    }
}

use crate::combinators::foundation::match_first;
use crate::combinators::foundation::sequence;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::opt_precision;
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
