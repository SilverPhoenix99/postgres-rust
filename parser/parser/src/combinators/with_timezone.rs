/// Alias: `opt_timezone`
pub(super) fn with_timezone(stream: &mut TokenStream) -> scan::Result<bool> {

    /*
        (WITH | WITHOUT) TIME ZONE
    */

    let (with_tz, ..) = (
        or((
            With.map(|_| true),
            Without.map(|_| false)
        )),
        Time,
        Zone
    ).parse(stream)?;

    Ok(with_tz)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("with time zone", true)]
    #[test_case("without time zone", false)]
    fn test_with_timezone(source: &str, expected: bool) {
        test_parser!(source, with_timezone, expected);
    }
}

use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_lexer::Keyword::Time;
use pg_lexer::Keyword::With;
use pg_lexer::Keyword::Without;
use pg_lexer::Keyword::Zone;
