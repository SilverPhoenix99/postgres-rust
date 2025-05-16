pub(super) fn opt_timezone() -> impl Combinator<Output = bool> {

    /*
        ( (WITH | WITHOUT) TIME ZONE )?
    */

    match_first!(
        With.map(|_| true),
        Without.map(|_| false)
    )
        .and_left(sequence!(Time, Zone).skip())
        .optional()
        .map(|tz| tz.unwrap_or(false))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("with time zone", true)]
    #[test_case("without time zone", false)]
    #[test_case("something else", false)]
    #[test_case("", false)]
    fn test_opt_timezone(source: &str, expected: bool) {
        test_parser!(source, opt_timezone(), expected);
    }
}

use crate::combinators::foundation::match_first;
use crate::combinators::foundation::sequence;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use pg_lexer::Keyword::Time;
use pg_lexer::Keyword::With;
use pg_lexer::Keyword::Without;
use pg_lexer::Keyword::Zone;
