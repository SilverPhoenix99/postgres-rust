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
    use crate::parser::combinators::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
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

use crate::lexer::Keyword::Array;
use crate::lexer::OperatorKind::CloseBracket;
use crate::lexer::OperatorKind::OpenBracket;
use crate::parser::combinators::foundation::between;
use crate::parser::combinators::foundation::many;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::combinators::i32_literal;
