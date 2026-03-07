/// Alias: `opt_timezone`
pub(super) fn with_timezone(ctx: &mut ParserContext) -> scan::Result<bool> {

    /*
        (WITH | WITHOUT) TIME ZONE
    */

    let (with_tz, ..) = seq!(
        alt!(
            With.map(|_| true),
            Without.map(|_| false)
        ),
        Time,
        Zone
    ).parse(ctx)?;

    Ok(with_tz)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("with time zone", true)]
    #[test_case("without time zone", false)]
    fn test_with_timezone(source: &str, expected: bool) {
        test_parser!(source, with_timezone, expected);
    }
}

use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Time;
use pg_lexer::Keyword::With;
use pg_lexer::Keyword::Without;
use pg_lexer::Keyword::Zone;
use pg_parser_core::scan;
