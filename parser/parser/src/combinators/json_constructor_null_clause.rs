/// Aliases:
/// * `json_array_constructor_null_clause_opt`
/// * `json_object_constructor_null_clause_opt`
pub(super) fn json_constructor_null_clause(stream: &mut TokenStream) -> scan::Result<bool> {

    /*
        ( ABSENT | NULL ) ON NULL
    */

    let (absent_on_null, ..) = seq!(
        alt!(
            Absent.map(|_| true),
            Null.map(|_| false),
        ),
        On,
        Null
    ).parse(stream)?;

    Ok(absent_on_null)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("null on null" => Ok(false))]
    #[test_case("absent on null" => Ok(true))]
    fn test_json_constructor_null_clause(source: &str) -> scan::Result<bool> {
        test_parser!(source, json_constructor_null_clause)
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Absent;
use pg_lexer::Keyword::Null;
use pg_lexer::Keyword::On;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
