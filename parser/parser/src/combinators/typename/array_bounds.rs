/// Alias: `opt_array_bounds`
pub(super) fn array_bounds(ctx: &mut ParserContext) -> scan::Result<Vec<Option<i32>>> {

    /*
          ARRAY ( '[' ICONST ']' )?
        | ( '[' ( ICONST )? ']' )[1..]
    */

    alt!(
        explicit_array,
        implicit_array
    ).parse(ctx)
}

fn explicit_array(ctx: &mut ParserContext) -> scan::Result<Vec<Option<i32>>> {

    /*
        ARRAY ( '[' ICONST ']' )?
    */

    let (_, dim) = seq!(
        Array,
        brackets!(i32_literal).optional()
    ).parse(ctx)?;

    Ok(vec![dim])
}

fn implicit_array(ctx: &mut ParserContext) -> scan::Result<Vec<Option<i32>>> {

    /*
        ( '[' ( ICONST )? ']' )[1..]
    */

    many!(
        brackets!(
            i32_literal.optional()
        )
    ).parse(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use test_case::test_matrix;

    #[test_matrix("array" => Ok(vec![None]))]
    #[test_matrix("array[7]" => Ok(vec![Some(7)]))]
    #[test_matrix("[]" => Ok(vec![None]))]
    #[test_matrix("[9]" => Ok(vec![Some(9)]))]
    #[test_matrix("[5][]" => Ok(vec![Some(5), None]))]
    #[test_matrix("[3][4]" => Ok(vec![Some(3), Some(4)]))]
    fn test_array_bounds(source: &str) -> scan::Result<Vec<Option<i32>>> {
        test_parser!(source, array_bounds)
    }
}

use crate::alt;
use crate::brackets;
use crate::combinators::core::Combinator;
use crate::combinators::i32_literal;
use crate::many;
use crate::seq;
use crate::ParserContext;
use pg_lexer::Keyword::Array;
use pg_parser_core::scan;
