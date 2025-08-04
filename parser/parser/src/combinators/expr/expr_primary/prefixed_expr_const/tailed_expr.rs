pub(super) fn tailed_expr(name: Vec<Str>, tail: AttrTail) -> ExprNode {
    match tail {
        AttrTail::Typecast { type_modifiers, value } => {
            // AexprConst

            let type_name = TypeName::Generic {
                name,
                type_modifiers
            };

            let typecast = TypecastExpr::new(
                StringConst(value),
                type_name
            );

            typecast.into()
        },
        AttrTail::FuncTail { args, filter, over } => {
            // func_expr
            let func_call = FuncCallExpr::new(name, args, filter, over);
            func_call.into()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pg_ast::{
        ExprNode::IntegerConst,
        FuncArgsKind,
        FuncCallExpr,
        OverClause::WindowName,
    };
    use test_case::test_case;

    #[test_case(
        AttrTail::Typecast {
            value: "123".into(),
            type_modifiers: Some(vec![IntegerConst(234)]),
        },
        TypecastExpr::new(
            StringConst("123".into()),
            TypeName::Generic {
                    name: vec!["foo".into()],
                    type_modifiers: Some(vec![IntegerConst(234)]),
                }
        ).into()
    )]
    #[test_case(
        AttrTail::FuncTail {
            args: FuncArgsKind::Wildcard { order_within_group: None },
            filter: Some(IntegerConst(123)),
            over: Some(WindowName("bar".into()))
        },
        FuncCallExpr::new(
            vec!["foo".into()],
            FuncArgsKind::Wildcard { order_within_group: None },
            Some(IntegerConst(123)),
            Some(WindowName("bar".into()))
        ).into()
    )]
    fn test_tailed_expr(tail: AttrTail, expected: ExprNode) {
        let actual = tailed_expr(vec!["foo".into()], tail);
        assert_eq!(expected, actual)
    }
}

use super::attr_tail::AttrTail;
use pg_ast::ExprNode;
use pg_ast::ExprNode::StringConst;
use pg_ast::FuncCallExpr;
use pg_ast::TypeName;
use pg_ast::TypecastExpr;
use pg_basics::Str;
