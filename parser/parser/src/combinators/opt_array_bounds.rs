pub(super) fn opt_array_bounds() -> impl Combinator<Output = Option<Vec<Option<i32>>>> {

    /*
          ARRAY ( '[' ICONST ']' )?
        | ( '[' ( ICONST )? ']' )*
    */

    match_first!{
        Array
            .and_right(
                between(
                    OpenBracket,
                    i32_literal(),
                    CloseBracket
                )
                .optional()
            )
            .map(|dim| Some(vec![dim])),
        many(
            between(
                OpenBracket,
                i32_literal().optional(),
                CloseBracket
            )
        )
        .optional()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;

    #[test_case("array", Some(vec![None]))]
    #[test_case("array[7]", Some(vec![Some(7)]))]
    #[test_case("[]", Some(vec![None]))]
    #[test_case("[9]", Some(vec![Some(9)]))]
    #[test_case("[5][]", Some(vec![Some(5), None]))]
    #[test_case("[3][4]", Some(vec![Some(3), Some(4)]))]
    #[test_case("", None)]
    #[test_case("something else", None)]
    fn test_opt_array_bounds(source: &str, expected: Option<Vec<Option<i32>>>) {
        test_parser!(source, opt_array_bounds(), expected);
    }
}

use crate::combinators::foundation::between;
use crate::combinators::foundation::many;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::combinators::i32_literal;
use pg_lexer::Keyword::Array;
use pg_lexer::OperatorKind::CloseBracket;
use pg_lexer::OperatorKind::OpenBracket;
