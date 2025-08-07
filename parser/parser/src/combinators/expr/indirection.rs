// `indirection` has different rules depending on context.
// See:
// * [`makeColumnRef(..., List *indirection, ...)`](https://github.com/postgres/postgres/blob/ae4569161a27823793ca24825bbabce2a91a0bc9/src/backend/parser/gram.y#L18696-L18727)
// * [`check_qualified_name()`](https://github.com/postgres/postgres/blob/ae4569161a27823793ca24825bbabce2a91a0bc9/src/backend/parser/gram.y#L18849-L18864)
// * [`check_func_name()`](https://github.com/postgres/postgres/blob/ae4569161a27823793ca24825bbabce2a91a0bc9/src/backend/parser/gram.y#L18866-L18882)
// * [`check_indirection()`](https://github.com/postgres/postgres/blob/ae4569161a27823793ca24825bbabce2a91a0bc9/src/backend/parser/gram.y#L18884-L18903)
// * [`makeRangeVarFromQualifiedName(..., List *namelist, ...)`](https://github.com/postgres/postgres/blob/ae4569161a27823793ca24825bbabce2a91a0bc9/src/backend/parser/gram.y#L19335)
pub(super) fn indirection(stream: &mut TokenStream) -> scan::Result<Vec<Indirection>> {

    /*
        ( indirection_el )+
    */

    many!(indirection_el).parse(stream)
}

fn indirection_el(stream: &mut TokenStream) -> scan::Result<Indirection> {

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
    ).parse(stream)
}

fn dot_indirection_el(stream: &mut TokenStream) -> scan::Result<Indirection> {

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
    ).parse(stream)?;

    Ok(indirection)
}

fn index_indirection_el(stream: &mut TokenStream) -> scan::Result<Indirection> {

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
    ).parse(stream)
}

pub(super) fn check_indirection(indirection: Located<Vec<Indirection>>) -> scan::Result<Vec<Indirection>> {

    // If present, '.*' must be the last element

    let (indirection, location) = indirection;

    let valid = indirection.iter()
        .position(|ind| matches!(ind, Wildcard))
        .is_none_or(|index| index == indirection.len() - 1);

    if valid {
        Ok(indirection)
    }
    else {
        let err = ImproperUseOfStar.at(location);
        Err(err.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use pg_ast::Indirection;
    use pg_ast::Indirection::Property;
    use pg_ast::Indirection::Slice;
    use pg_basics::Location;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case(".*", Wildcard)]
    #[test_case(".some_property", Property("some_property".into()))]
    #[test_case("[:]", Slice(None, None))]
    #[test_case("[:1]", Slice(None, Some(IntegerConst(1))))]
    #[test_case("[2]", Index(IntegerConst(2)))]
    #[test_case("[3:]", Slice(Some(IntegerConst(3)), None))]
    #[test_case("[4:5]", Slice(Some(IntegerConst(4)), Some(IntegerConst(5))))]
    fn test_indirection_el(source: &str, expected: Indirection) {
        test_parser!(source, indirection_el, expected)
    }

    #[test]
    fn test_indirection() {
        test_parser!(
            source = ".some_property[:].*",
            parser = indirection,
            expected = vec![
                Property("some_property".into()),
                Slice(None, None),
                Wildcard,
            ]
        )
    }

    #[test]
    fn test_check_indirection() {
        assert_matches!(
            check_indirection((
                vec![Property("some_property".into()), Wildcard],
                Location::new(0..0, 0, 0)
            )),
            Ok(_)
        );

        assert_matches!(
            check_indirection((
                vec![Property("some_property".into())],
                Location::new(0..0, 0, 0)
            )),
            Ok(_)
        );

        assert_matches!(
            check_indirection((
                vec![Wildcard, Property("some_property".into())],
                Location::new(0..0, 0, 0)
            )),
            Err(_)
        );
    }
}

use crate::combinators::expr::a_expr;
use pg_ast::Indirection;
use pg_ast::Indirection::Index;
use pg_ast::Indirection::Property;
use pg_ast::Indirection::Slice;
use pg_ast::Indirection::Wildcard;
use pg_basics::Located;
use pg_combinators::alt;
use pg_combinators::brackets;
use pg_combinators::many;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_elog::parser::Error::ImproperUseOfStar;
use pg_lexer::OperatorKind::Colon;
use pg_lexer::OperatorKind::Dot;
use pg_lexer::OperatorKind::Mul;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
use pg_sink_combinators::col_label;
