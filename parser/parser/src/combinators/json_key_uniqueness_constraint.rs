/// Alias: `json_key_uniqueness_constraint_opt`
pub(super) fn json_key_uniqueness_constraint(stream: &mut TokenStream) -> scan::Result<bool> {

    /*
        ( WITH | WITHOUT ) UNIQUE ( KEYS )?
    */

    let (unique, ..) = seq!(
        alt!(
            With.map(|_| true),
            Without.map(|_| false)
        ),
        Unique,
        Keys.optional()
    ).parse(stream)?;

    Ok(unique)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("with unique keys" => Ok(true))]
    #[test_case("with unique" => Ok(true))]
    #[test_case("without unique keys" => Ok(false))]
    #[test_case("without unique" => Ok(false))]
    fn test_json_key_uniqueness_constraint(source: &str) -> scan::Result<bool> {
        test_parser!(source, json_key_uniqueness_constraint)
    }
}

use crate::combinators::foundation::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Keys;
use pg_lexer::Keyword::Unique;
use pg_lexer::Keyword::With;
use pg_lexer::Keyword::Without;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
