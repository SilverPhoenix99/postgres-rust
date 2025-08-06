/// Alias: `opt_nulls_order`
pub(super) fn nulls_order(stream: &mut TokenStream) -> scan::Result<SortNulls> {

    /*
          NULLS FIRST
        | NULLS LAST
    */

    let (_, order) = seq!(
        Nulls,
        alt!(
            First.map(|_| NullsFirst),
            Last.map(|_| NullsLast),
        )
    ).parse(stream)?;

    Ok(order)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("nulls first", NullsFirst)]
    #[test_case("nulls last", NullsLast)]
    fn test_nulls_order(source: &str, expected: SortNulls) {
        test_parser!(source, nulls_order, expected)
    }
}

use crate::combinators::foundation::alt;
use crate::combinators::foundation::seq;
use pg_ast::SortNulls;
use pg_ast::SortNulls::NullsFirst;
use pg_ast::SortNulls::NullsLast;
use pg_combinators::Combinator;
use pg_lexer::Keyword::First;
use pg_lexer::Keyword::Last;
use pg_lexer::Keyword::Nulls;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
