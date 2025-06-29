pub(super) fn opt_array_bounds(stream: &mut TokenStream) -> scan::Result<Option<Vec<Option<i32>>>> {

    /*
          ARRAY ( '[' ICONST ']' )?
        | ( '[' ( ICONST )? ']' )*
    */

    let bounds = or((
        (
            Array,
            between_square(i32_literal).optional()
        )
            .map(|(_, dim)| vec![dim]),
        many(
            between_square(i32_literal.optional())
        )
    )).parse(stream);

    let bounds = bounds.optional()?;
    Ok(bounds)
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

use crate::combinators::foundation::between_square;
use crate::combinators::foundation::many;
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::combinators::i32_literal;
use crate::result::Optional;
use crate::scan;
use crate::stream::TokenStream;
use pg_lexer::Keyword::Array;
