pub(super) fn opt_nulls_order(stream: &mut TokenStream) -> Result<Option<SortNulls>> {

    /*
          NULLS FIRST
        | NULLS LAST
        | // empty
    */

    let order = seq!(=>
        Nulls.parse(stream),
        choice!(parsed stream =>
            First.map(|_| NullsFirst),
            Last.map(|_| NullsLast),
        )
    );

    let order = order.optional()?
        .map(|(_, order)| order);

    Ok(order)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("nulls first", Some(NullsFirst))]
    #[test_case("nulls last", Some(NullsLast))]
    #[test_case("", None)]
    #[test_case("foo", None)]
    fn test_opt_nulls_order(source: &str, expected: Option<SortNulls>) {
        test_parser!(source, opt_nulls_order, expected)
    }
}

use crate::combinators::foundation::choice;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::result::Optional;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_ast::SortNulls;
use pg_ast::SortNulls::NullsFirst;
use pg_ast::SortNulls::NullsLast;
use pg_lexer::Keyword::First;
use pg_lexer::Keyword::Last;
use pg_lexer::Keyword::Nulls;
