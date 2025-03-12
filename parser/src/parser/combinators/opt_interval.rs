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
    use crate::parser::ast_node::IntervalRange;
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
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

use crate::lexer::Keyword::Day as DayKw;
use crate::lexer::Keyword::Hour as HourKw;
use crate::lexer::Keyword::Minute as MinuteKw;
use crate::lexer::Keyword::Month as MonthKw;
use crate::lexer::Keyword::Second as SecondKw;
use crate::lexer::Keyword::To;
use crate::lexer::Keyword::Year as YearKw;
use crate::parser::ast_node::IntervalRange;
use crate::parser::ast_node::IntervalRange::DayToHour;
use crate::parser::ast_node::IntervalRange::DayToMinute;
use crate::parser::ast_node::IntervalRange::DayToSecond;
use crate::parser::ast_node::IntervalRange::HourToSecond;
use crate::parser::ast_node::IntervalRange::Minute;
use crate::parser::ast_node::IntervalRange::MinuteToSecond;
use crate::parser::ast_node::IntervalRange::Month;
use crate::parser::ast_node::IntervalRange::Second;
use crate::parser::ast_node::IntervalRange::Year;
use crate::parser::ast_node::IntervalRange::YearToMonth;
use crate::parser::ast_node::IntervalRange::{Day, Hour, HourToMinute};
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::opt_precision;
