mod ambiguous_prefix_expr;
mod attr_tail;
mod identifier_prefixed_expr;
mod tailed_expr;

pub(super) fn prefixed_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    or((
        // This need to be first, due to conflicts with some keywords.
        ambiguous_prefix_expr,

        identifier_prefixed_expr,
    )).parse(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ColumnRef,
        pg_ast::ExprNode::{IntegerConst, StringConst},
        pg_ast::FuncArgExpr,
        pg_ast::FuncArgsKind,
        pg_ast::FuncArgsOrder,
        pg_ast::FuncCall,
        pg_ast::OverClause,
        pg_ast::SortBy,
        pg_ast::TypeName,
        pg_ast::TypecastExpr,
        pg_basics::Location,
    };

    #[test_case("foo",
        ColumnRef::SingleName("foo".into()).into()
    )]
    #[test_case("foo.bar",
        ColumnRef::Name(vec!["foo".into(), "bar".into()]).into()
    )]
    #[test_case("foo.*",
        ColumnRef::WildcardName(vec!["foo".into()]).into()
    )]
    #[test_case("double.*",
        ColumnRef::WildcardName(vec!["double".into()]).into()
    )]
    #[test_case("double precision '123'",
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Float8
        ).into()
    )]
    #[test_case("foo.bar 'baz'",
        TypecastExpr::new(
            StringConst("baz".into()),
            TypeName::Generic {
                name: vec!["foo".into(), "bar".into()],
                type_modifiers: None
            }
        ).into()
    )]
    #[test_case("foo()",
        FuncCall::new(
            vec!["foo".into()],
            FuncArgsKind::Empty { order_within_group: None },
            None,
            None
        ).into()
    )]
    #[test_case("foo() within group (order by 1)",
        FuncCall::new(
            vec!["foo".into()],
            FuncArgsKind::Empty {
                order_within_group: Some(vec![
                    SortBy::new(IntegerConst(1), None, None)
                ])
            },
            None,
            None
        ).into()
    )]
    #[test_case("foo(*) within group (order by 1)",
        FuncCall::new(
            vec!["foo".into()],
            FuncArgsKind::Wildcard {
                order_within_group: Some(vec![
                    SortBy::new(IntegerConst(1), None, None)
                ])
            },
            None,
            None
        ).into()
    )]
    #[test_case("foo(1) within group (order by 2) filter (where 3) over bar",
        FuncCall::new(
            vec!["foo".into()],
            FuncArgsKind::All {
                args: vec![
                    (
                        FuncArgExpr::Unnamed(IntegerConst(1)),
                        Location::new(4..5, 1, 5)
                    )
                ],
                order: Some((
                    FuncArgsOrder::WithinGroup(vec![
                        SortBy::new(IntegerConst(2), None, None)
                    ]),
                    Location::new(7..13, 1, 8)
                ))
            },
            Some(IntegerConst(3)),
            Some(OverClause::WindowName("bar".into()))
        ).into()
    )]
    #[test_case("foo(1 order by 2) filter (where 3) over bar",
        FuncCall::new(
            vec!["foo".into()],
            FuncArgsKind::All {
                args: vec![
                    (
                        FuncArgExpr::Unnamed(IntegerConst(1)),
                        Location::new(4..5, 1, 5)
                    )
                ],
                order: Some((
                    FuncArgsOrder::OrderBy(vec![
                        SortBy::new(IntegerConst(2), None, None)
                    ]),
                    Location::new(6..11, 1, 7)
                ))
            },
            Some(IntegerConst(3)),
            Some(OverClause::WindowName("bar".into()))
        ).into()
    )]
    #[test_case("foo(1)",
        FuncCall::new(
            vec!["foo".into()],
            FuncArgsKind::All {
                args: vec![
                    (
                        FuncArgExpr::Unnamed(IntegerConst(1)),
                        Location::new(4..5, 1, 5)
                    )
                ],
                order: None
            },
            None,
            None
        ).into()
    )]
    #[test_case("double(7) '123'",
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["double".into()],
                type_modifiers: Some(vec![
                    IntegerConst(7)
                ])
            }
        ).into()
    )]
    #[test_case("foo(7) '123'",
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                name: vec!["foo".into()],
                type_modifiers: Some(vec![
                    IntegerConst(7)
                ])
            }
        ).into()
    )]
    fn test_prefixed_expr(source: &str, expected: ExprNode) {
        test_parser!(source, prefixed_expr, expected)
    }
}

use self::{
    ambiguous_prefix_expr::*,
    attr_tail::*,
    identifier_prefixed_expr::*,
    tailed_expr::*,
};
use crate::combinators::foundation::or;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
