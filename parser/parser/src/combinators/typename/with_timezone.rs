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
    use crate::test_parser;
    use test_case::test_matrix;

    #[test_matrix("with time zone" => Ok(true))]
    #[test_matrix("without time zone" => Ok(false))]
    fn test_with_timezone(source: &str) -> scan::Result<bool> {
        test_parser!(source, with_timezone)
    }
}

use crate::alt;
use crate::combinators::core::Combinator;
use crate::seq;
use crate::ParserContext;
use pg_lexer::Keyword::Time;
use pg_lexer::Keyword::With;
use pg_lexer::Keyword::Without;
use pg_lexer::Keyword::Zone;
use pg_parser_core::scan;
