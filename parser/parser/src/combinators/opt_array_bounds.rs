/// Post-condition: Vec is **Not** empty
pub(super) fn opt_array_bounds() -> impl Combinator<Output = Vec<Option<i32>>> {

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
            .map(|dim| vec![dim]),
        many(
            between(
                OpenBracket,
                i32_literal().optional(),
                CloseBracket
            )
        )
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    use test_case::test_case;

    #[test_case("array", vec![None])]
    #[test_case("array[7]", vec![Some(7)])]
    #[test_case("[]", vec![None])]
    #[test_case("[9]", vec![Some(9)])]
    #[test_case("[5][]", vec![Some(5), None])]
    #[test_case("[3][4]", vec![Some(3), Some(4)])]
    fn test_opt_array_bounds(source: &str, expected: Vec<Option<i32>>) {

        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        let actual = opt_array_bounds().parse(&mut stream);

        assert_eq!(
            Ok(expected.clone()),
            actual,
            r"expected {expected:?} for source {source:?} but actually got {actual:?}"
        );
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
