pub(super) fn opt_nulls_order() -> impl Combinator<Output = Option<SortNulls>> {

    /*
          NULLS FIRST
        | NULLS LAST
        | // empty
    */

    Nulls
        .and_right(or(
            First.map(|_| NullsFirst),
            Last.map(|_| NullsLast),
        ))
        .optional()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;
    use test_case::test_case;

    #[test_case("nulls first", Some(NullsFirst))]
    #[test_case("nulls last", Some(NullsLast))]
    #[test_case("", None)]
    #[test_case("foo", None)]
    fn test_opt_nulls_order(source: &str, expected: Option<SortNulls>) {
        test_parser!(source, opt_nulls_order(), expected)
    }
}

use crate::parser::ast_node::SortNulls;
use crate::parser::ast_node::SortNulls::NullsFirst;
use crate::parser::ast_node::SortNulls::NullsLast;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use postgres_parser_lexer::Keyword::First;
use postgres_parser_lexer::Keyword::Last;
use postgres_parser_lexer::Keyword::Nulls;
