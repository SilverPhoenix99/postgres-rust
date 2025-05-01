pub(super) fn opt_nulls_order() -> impl Combinator<Output = Option<SortNulls>> {

    /*
          NULLS FIRST
        | NULLS LAST
        | // empty
    */

    Nulls
        .and_right(or(
            Kw::First.map(|_| First),
            Kw::Last.map(|_| Last),
        ))
        .optional()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tests::test_parser;
    use test_case::test_case;

    #[test_case("nulls first", Some(First))]
    #[test_case("nulls last", Some(Last))]
    #[test_case("", None)]
    #[test_case("foo", None)]
    fn test_opt_nulls_order(source: &str, expected: Option<SortNulls>) {
        test_parser!(source, opt_nulls_order(), expected)
    }
}

use crate::lexer::Keyword as Kw;
use crate::lexer::Keyword::Nulls;
use crate::parser::ast_node::SortNulls;
use crate::parser::ast_node::SortNulls::First;
use crate::parser::ast_node::SortNulls::Last;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
