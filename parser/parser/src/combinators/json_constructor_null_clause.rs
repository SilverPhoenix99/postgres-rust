/// Aliases:
/// * `json_array_constructor_null_clause_opt`
/// * `json_object_constructor_null_clause_opt`
pub(super) fn json_constructor_null_clause(ctx: &mut ParserContext) -> scan::Result<bool> {

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
    ).parse(ctx)?;

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

use pg_combinators::alt;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_combinators::ParserContext;
use pg_lexer::Keyword::Absent;
use pg_lexer::Keyword::Null;
use pg_lexer::Keyword::On;
use pg_parser_core::scan;
