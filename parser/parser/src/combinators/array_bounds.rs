/// Alias: `opt_array_bounds`
pub(super) fn array_bounds(stream: &mut TokenStream) -> scan::Result<Vec<Option<i32>>> {

    /*
          ARRAY ( '[' ICONST ']' )?
        | ( '[' ( ICONST )? ']' )[1..]
    */

    alt!(
        explicit_array,
        implicit_array
    ).parse(stream)
}

fn explicit_array(stream: &mut TokenStream) -> scan::Result<Vec<Option<i32>>> {

    /*
        ARRAY ( '[' ICONST ']' )?
    */

    let (_, dim) = seq!(
        Array,
        brackets!(i32_literal).optional()
    ).parse(stream)?;

    Ok(vec![dim])
}

fn implicit_array(stream: &mut TokenStream) -> scan::Result<Vec<Option<i32>>> {

    /*
        ( '[' ( ICONST )? ']' )[1..]
    */

    many!(
        brackets!(
            i32_literal.optional()
        )
    ).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("array", vec![None])]
    #[test_case("array[7]", vec![Some(7)])]
    #[test_case("[]", vec![None])]
    #[test_case("[9]", vec![Some(9)])]
    #[test_case("[5][]", vec![Some(5), None])]
    #[test_case("[3][4]", vec![Some(3), Some(4)])]
    fn test_array_bounds(source: &str, expected: Vec<Option<i32>>) {
        test_parser!(source, array_bounds, expected);
    }
}

use crate::combinators::i32_literal;
use pg_combinators::alt;
use pg_combinators::brackets;
use pg_combinators::many;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::Array;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
