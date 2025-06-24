pub(super) fn opt_array_bounds(stream: &mut TokenStream) -> Result<Option<Vec<Option<i32>>>> {

    /*
          ARRAY ( '[' ICONST ']' )?
        | ( '[' ( ICONST )? ']' )*
    */

    choice!(stream =>
        seq!(=>
            Array.parse(stream),
            between!(square : stream => i32_literal.parse(stream))
                .optional()
        )
            .map(|(_, dim)| vec![dim]),
        many!(=>
            between!(square : stream =>
                i32_literal.parse(stream)
                    .optional()
            )
        )
    )
        .optional()
        .map_err(From::from)
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
        test_parser!(source, opt_array_bounds, expected);
    }
}

use crate::combinators::foundation::between;
use crate::combinators::foundation::choice;
use crate::combinators::foundation::many;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::combinators::i32_literal;
use crate::result::Optional;
use crate::scan::Result;
use crate::stream::TokenStream;
use pg_lexer::Keyword::Array;
