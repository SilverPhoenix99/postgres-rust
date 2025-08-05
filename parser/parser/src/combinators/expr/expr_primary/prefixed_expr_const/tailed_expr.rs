pub(super) fn tailed_expr(name: Vec<Str>, tail: AttrTail) -> ExprNode {
    match tail {
        AttrTail::Typecast { type_modifiers, value } => {
            // AexprConst

            let type_name = TypeName::Generic {
                name,
                type_modifiers
            };

            let typecast = StringTypecastExpr::new(value, type_name);

            typecast.into()
        },
        AttrTail::FuncTail { args, filter, over } => {
            // func_expr
            let mut func_call = FuncCallExpr::from(FuncCall::new(name, args));
            func_call.set_agg_filter(filter)
                .set_over(over);
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
        }
        => ExprNode::from(
            StringTypecastExpr::new(
                "123",
                TypeName::Generic {
                    name: vec!["foo".into()],
                    type_modifiers: Some(vec![IntegerConst(234)]),
                }
            )
        )
    )]
    #[test_case(
        AttrTail::FuncTail {
            args: FuncArgsKind::Wildcard { order_within_group: None },
            filter: Some(IntegerConst(123)),
            over: Some(WindowName("bar".into()))
        }
        => ExprNode::from(
            FuncCallExpr::from(
                FuncCall::new(
                    vec!["foo".into()],
                    FuncArgsKind::Wildcard { order_within_group: None }
                )
            )
            .with_agg_filter(IntegerConst(123))
            .with_over(WindowName("bar".into()))
        )
    )]
    fn test_tailed_expr(tail: AttrTail) -> ExprNode {
        tailed_expr(vec!["foo".into()], tail)
    }
}

use super::attr_tail::AttrTail;
use pg_ast::ExprNode;
use pg_ast::FuncCall;
use pg_ast::FuncCallExpr;
use pg_ast::StringTypecastExpr;
use pg_ast::TypeName;
use pg_basics::Str;
