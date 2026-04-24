// `indirection` has different rules depending on context.
// See:
// * [`makeColumnRef(..., List *indirection, ...)`](https://github.com/postgres/postgres/blob/ae4569161a27823793ca24825bbabce2a91a0bc9/src/backend/parser/gram.y#L18696-L18727)
// * [`check_qualified_name()`](https://github.com/postgres/postgres/blob/ae4569161a27823793ca24825bbabce2a91a0bc9/src/backend/parser/gram.y#L18849-L18864)
// * [`check_func_name()`](https://github.com/postgres/postgres/blob/ae4569161a27823793ca24825bbabce2a91a0bc9/src/backend/parser/gram.y#L18866-L18882)
// * [`check_indirection()`](https://github.com/postgres/postgres/blob/ae4569161a27823793ca24825bbabce2a91a0bc9/src/backend/parser/gram.y#L18884-L18903)
// * [`makeRangeVarFromQualifiedName(..., List *namelist, ...)`](https://github.com/postgres/postgres/blob/ae4569161a27823793ca24825bbabce2a91a0bc9/src/backend/parser/gram.y#L19335)
pub(super) fn indirection(ctx: &mut ParserContext) -> scan::Result<Vec<Indirection>> {

    /*
        ( indirection_el )+
    */

    many!(indirection_el).parse(ctx)
}

fn indirection_el(ctx: &mut ParserContext) -> scan::Result<Indirection> {

    /*
          '.' '*'
        | '.' ColLabel
        | '[' ':' ']'
        | '[' ':' a_expr ']'
        | '[' a_expr ']'
        | '[' a_expr ':' ']'
        | '[' a_expr ':' a_expr ']'
    */

    alt!(
        dot_indirection_el,
        brackets!(index_indirection_el)
    ).parse(ctx)
}

fn dot_indirection_el(ctx: &mut ParserContext) -> scan::Result<Indirection> {

    /*
          '.' '*'
        | '.' ColLabel
    */

    let (_, indirection) = seq!(
        Dot,
        alt!(
            Mul.map(|_| Wildcard),
            col_label.map(Property),
        )
    ).parse(ctx)?;

    Ok(indirection)
}

fn index_indirection_el(ctx: &mut ParserContext) -> scan::Result<Indirection> {

    /*
          '[' ':' ']'
        | '[' ':' a_expr ']'
        | '[' a_expr ']'
        | '[' a_expr ':' ']'
        | '[' a_expr ':' a_expr ']'
    */

    alt!(
        seq!(
            Colon,
            a_expr.map(|index| Slice(None, Some(index)))
                .optional()
        )
            .map(|(_, expr)| expr.unwrap_or(Slice(None, None))),

        seq!(
            a_expr,
            seq!(Colon, a_expr.optional())
                .map(|(_, expr)| expr)
                .optional()
        )
            .map(|(left, right)| match right {
                None => Index(left),
                Some(None) => Slice(Some(left), None),
                Some(Some(right)) => Slice(Some(left), Some(right)),
            })
    ).parse(ctx)
}

pub(super) fn check_indirection(indirection: Located<Vec<Indirection>>) -> scan::Result<Vec<Indirection>> {

    // If present, '.*' must be the last element

    let Located(indirection, location) = indirection;

    let valid = indirection.iter()
        .position(|ind| matches!(ind, Wildcard))
        .is_none_or(|index| index == indirection.len() - 1);

    if valid {
        Ok(indirection)
    }
    else {
        Err(ImproperUseOfStar.at_location(location).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::IntegerConst;
    use pg_ast::Indirection;
    use pg_ast::Indirection::Property;
    use pg_ast::Indirection::Slice;
    use pg_basics::Location;
    use test_case::test_matrix;

    #[test_matrix(".*" => Ok(Wildcard))]
    #[test_matrix(".some_property" => Ok(Property("some_property".into())))]
    #[test_matrix("[:]" => Ok(Slice(None, None)))]
    #[test_matrix("[:1]" => Ok(Slice(None, Some(IntegerConst(1)))))]
    #[test_matrix("[2]" => Ok(Index(IntegerConst(2))))]
    #[test_matrix("[3:]" => Ok(Slice(Some(IntegerConst(3)), None)))]
    #[test_matrix("[4:5]" => Ok(Slice(Some(IntegerConst(4)), Some(IntegerConst(5)))))]
    fn test_indirection_el(source: &str) -> scan::Result<Indirection> {
        test_parser!(source, indirection_el)
    }

    #[test_matrix(".some_property[:].*" => Ok(
        vec![
            Property("some_property".into()),
            Slice(None, None),
            Wildcard,
        ]
    ))]
    fn test_indirection(source: &str) -> scan::Result<Vec<Indirection>> {
        test_parser!(source, indirection)
    }

    #[test_matrix(
        [
            vec![Property("some_property".into()), Wildcard],
            vec![Property("some_property".into())]
        ]
        => matches Ok(_)
    )]
    #[test_matrix(vec![Wildcard, Property("some_property".into())] => matches Err(_))]
    fn test_check_indirection(indirection: Vec<Indirection>) -> scan::Result<Vec<Indirection>> {
        check_indirection(Located(
            indirection,
            Location::new(0..0, 0, 0)
        ))
    }
}

use crate::alt;
use crate::brackets;
use crate::combinators::col_label;
use crate::combinators::core::Combinator;
use crate::combinators::expr::a_expr;
use crate::many;
use crate::seq;
use crate::ParserContext;
use pg_ast::Indirection;
use pg_ast::Indirection::Index;
use pg_ast::Indirection::Property;
use pg_ast::Indirection::Slice;
use pg_ast::Indirection::Wildcard;
use pg_basics::IntoLocated;
use pg_basics::Located;
use pg_elog::parser::Error::ImproperUseOfStar;
use pg_lexer::OperatorKind::Colon;
use pg_lexer::OperatorKind::Dot;
use pg_lexer::OperatorKind::Mul;
use pg_parser_core::scan;
