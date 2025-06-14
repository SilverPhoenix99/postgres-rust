// TODO: `indirection` has different rules depending on context.
// See:
// * [`makeColumnRef(..., List *indirection, ...)`](https://github.com/postgres/postgres/blob/ae4569161a27823793ca24825bbabce2a91a0bc9/src/backend/parser/gram.y#L18696-L18727)
// * [`check_qualified_name()`](https://github.com/postgres/postgres/blob/ae4569161a27823793ca24825bbabce2a91a0bc9/src/backend/parser/gram.y#L18849-L18864)
// * [`check_func_name()`](https://github.com/postgres/postgres/blob/ae4569161a27823793ca24825bbabce2a91a0bc9/src/backend/parser/gram.y#L18866-L18882)
// * [`check_indirection()`](https://github.com/postgres/postgres/blob/ae4569161a27823793ca24825bbabce2a91a0bc9/src/backend/parser/gram.y#L18884-L18903)
// * [`makeRangeVarFromQualifiedName(..., List *namelist, ...)`](https://github.com/postgres/postgres/blob/ae4569161a27823793ca24825bbabce2a91a0bc9/src/backend/parser/gram.y#L19335)
pub(super) fn indirection() -> impl Combinator<Output = Vec<Indirection>> {

    /*
        ( indirection_el )+
    */

    many(indirection_el())
}

fn indirection_el() -> impl Combinator<Output = Indirection> {

    /*
          '.' '*'
        | '.' ColLabel
        | '[' ':' ']'
        | '[' ':' a_expr ']'
        | '[' a_expr ']'
        | '[' a_expr ':' ']'
        | '[' a_expr ':' a_expr ']'
    */

    match_first!(

        Dot.and_right(or(
            Mul.map(|_| All),
            col_label().map(Property),
        )),

        between_brackets(match_first!(
            Colon
                .and_right(
                    a_expr()
                        .map(|index| Slice(None, Some(index)))
                        .optional()
                )
                .map(|expr| expr.unwrap_or(Slice(None, None))),

            sequence!(
                a_expr(),
                optional(
                    Colon.and_right(
                        a_expr().optional()
                    ),
                ))
                .map(|(left, right)| match right {
                    None => Index(left),
                    Some(None) => Slice(Some(left), None),
                    Some(Some(right)) => Slice(Some(left), Some(right)),
                })
        ))
    )
}

pub(super) fn check_indirection(indirection: Located<Vec<Indirection>>) -> Result<Vec<Indirection>> {

    // If present, '.*' must be the last element

    let (indirection, location) = indirection;

    let valid = indirection.iter()
        .position(|ind| matches!(ind, All))
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
    use crate::stream::TokenStream;
    use crate::tests::DEFAULT_CONFIG;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use pg_ast::Indirection;
    use pg_ast::Indirection::Property;
    use pg_ast::Indirection::Slice;
    use pg_basics::Location;
    use test_case::test_case;

    #[test_case(".*", All)]
    #[test_case(".some_property", Property("some_property".into()))]
    #[test_case("[:]", Slice(None, None))]
    #[test_case("[:1]", Slice(None, Some(IntegerConst(1))))]
    #[test_case("[2]", Index(IntegerConst(2)))]
    #[test_case("[3:]", Slice(Some(IntegerConst(3)), None))]
    #[test_case("[4:5]", Slice(Some(IntegerConst(4)), Some(IntegerConst(5))))]
    fn test_indirection_el(source: &str, expected: Indirection) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), indirection_el().parse(&mut stream));
    }

    #[test]
    fn test_indirection() {
        let mut stream = TokenStream::new(".some_property[:].*", DEFAULT_CONFIG);

        let expected = vec![
            Property("some_property".into()),
            Slice(None, None),
            All,
        ];

        assert_eq!(Ok(expected), indirection().parse(&mut stream));
    }

    #[test]
    fn test_check_indirection() {
        assert_matches!(
            check_indirection((
                vec![Property("some_property".into()), All],
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
                vec![All, Property("some_property".into())],
                Location::new(0..0, 0, 0)
            )),
            Err(_)
        );
    }
}

use crate::combinators::between_brackets;
use crate::combinators::col_label;
use crate::combinators::expr::a_expr;
use crate::combinators::foundation::many;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::optional;
use crate::combinators::foundation::or;
use crate::combinators::foundation::sequence;
use crate::combinators::foundation::Combinator;
use crate::combinators::foundation::CombinatorHelpers;
use crate::scan::Result;
use pg_ast::Indirection;
use pg_ast::Indirection::All;
use pg_ast::Indirection::Index;
use pg_ast::Indirection::Property;
use pg_ast::Indirection::Slice;
use pg_basics::Located;
use pg_elog::parser::Error::ImproperUseOfStar;
use pg_lexer::OperatorKind::Colon;
use pg_lexer::OperatorKind::Dot;
use pg_lexer::OperatorKind::Mul;
