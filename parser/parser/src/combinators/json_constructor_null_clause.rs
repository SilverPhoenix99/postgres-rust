/// Aliases:
/// * `json_array_constructor_null_clause_opt`
/// * `json_object_constructor_null_clause_opt`
pub(super) fn json_constructor_null_clause(stream: &mut TokenStream) -> scan::Result<bool> {

    /*
        ( NULL | ABSENT ) ON NULL
    */

    let (remove_nulls, ..) = (
        or((
            Null.map(|_| false),
            Absent.map(|_| true)
        )),
        On,
        Null
    ).parse(stream)?;

    Ok(remove_nulls)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("null on null" => Ok(false))]
    #[test_case("absent on null" => Ok(true))]
    fn test_json_constructor_null_clause(source: &str) -> scan::Result<bool> {
        test_parser!(source, json_constructor_null_clause)
    }
}

use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_lexer::Keyword::Absent;
use pg_lexer::Keyword::Null;
use pg_lexer::Keyword::On;
