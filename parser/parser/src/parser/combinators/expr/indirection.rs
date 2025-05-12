/// Post-condition: Vec is **Not** empty
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

        between(
            OpenBracket.skip(),
            match_first!(

                Colon
                    .and_right(
                        a_expr().map(SliceTo)
                            .optional()
                    )
                    .map(|expr| expr.unwrap_or(FullSlice)),

                sequence!(
                    a_expr(),
                    optional(
                        Colon.and_right(
                            a_expr().optional()
                        ),
                    ))
                    .map(|(left, right)| match right {
                        None => Index(left),
                        Some(None) => SliceFrom(left),
                        Some(Some(right)) => Slice(left, right),
                    })
            ),
            CloseBracket.skip()
        )
    )
}

pub(super) fn check_indirection(indirection: Located<Vec<Indirection>>) -> ScanResult<Vec<Indirection>> {

    // If present, '.*' must be the last element

    let (indirection, location) = indirection;

    let valid = indirection.iter()
        .position(|ind| matches!(ind, All))
        .is_none_or(|index| index == indirection.len() - 1);

    if valid {
        Ok(indirection)
    }
    else {
        let err = ParserError::new(ImproperUseOfStar, location);
        Err(err.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast_node::ExprNode;
    use crate::parser::tests::DEFAULT_CONFIG;
    use crate::parser::token_stream::TokenStream;
    use postgres_basics::Location;
    use test_case::test_case;

    #[test_case(".*", All)]
    #[test_case(".some_property", Property("some_property".into()))]
    #[test_case("[:]", FullSlice)]
    #[test_case("[:1]", SliceTo(ExprNode::IntegerConst(1)))]
    #[test_case("[2]", Index(ExprNode::IntegerConst(2)))]
    #[test_case("[3:]", SliceFrom(ExprNode::IntegerConst(3)))]
    #[test_case("[4:5]", Slice(ExprNode::IntegerConst(4), ExprNode::IntegerConst(5)))]
    fn test_indirection_el(source: &str, expected: Indirection) {
        let mut stream = TokenStream::new(source, DEFAULT_CONFIG);
        assert_eq!(Ok(expected), indirection_el().parse(&mut stream));
    }

    #[test]
    fn test_indirection() {
        let mut stream = TokenStream::new(".some_property[:].*", DEFAULT_CONFIG);

        let expected = vec![
            Property("some_property".into()),
            FullSlice,
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

use crate::parser::ast_node::Indirection;
use crate::parser::ast_node::Indirection::All;
use crate::parser::ast_node::Indirection::FullSlice;
use crate::parser::ast_node::Indirection::Index;
use crate::parser::ast_node::Indirection::Property;
use crate::parser::ast_node::Indirection::Slice;
use crate::parser::ast_node::Indirection::SliceFrom;
use crate::parser::ast_node::Indirection::SliceTo;
use crate::parser::combinators::col_label;
use crate::parser::combinators::expr::a_expr;
use crate::parser::combinators::foundation::between;
use crate::parser::combinators::foundation::many;
use crate::parser::combinators::foundation::match_first;
use crate::parser::combinators::foundation::optional;
use crate::parser::combinators::foundation::or;
use crate::parser::combinators::foundation::sequence;
use crate::parser::combinators::foundation::Combinator;
use crate::parser::combinators::foundation::CombinatorHelpers;
use crate::parser::result::ScanResult;
use crate::parser::ParserError;
use crate::parser::ParserErrorKind::ImproperUseOfStar;
use postgres_basics::Located;
use postgres_parser_lexer::OperatorKind::CloseBracket;
use postgres_parser_lexer::OperatorKind::Colon;
use postgres_parser_lexer::OperatorKind::Dot;
use postgres_parser_lexer::OperatorKind::Mul;
use postgres_parser_lexer::OperatorKind::OpenBracket;
