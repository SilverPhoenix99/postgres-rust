pub(super) fn opt_timezone(stream: &mut TokenStream) -> Result<bool> {

    /*
        ( (WITH | WITHOUT) TIME ZONE )?
    */

    let tz = seq!(=>
        choice!(parsed stream =>
            With.map(|_| true),
            Without.map(|_| false)
        ),
        Time.parse(stream),
        Zone.parse(stream)
    );

    let tz = match tz.optional()? {
        Some((tz, ..)) => tz,
        None => false
    };

    Ok(tz)
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
        test_parser!(source, opt_timezone, expected);
    }
}

use crate::combinators::foundation::choice;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::result::Optional;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_lexer::Keyword::Time;
use pg_lexer::Keyword::With;
use pg_lexer::Keyword::Without;
use pg_lexer::Keyword::Zone;
