pub(super) fn tailed_expr(name: Vec<Str>, tail: AttrTail) -> ExprNode {
    match tail {
        AttrTail::Typecast { type_modifiers, value } => {
            // AexprConst

            // TODO: Check if it's a known type name, like "numeric", etc.
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
            let func_call = FuncCall::new(name, args, filter, over);
            func_call.into()
        },
    }
}

use super::attr_tail::AttrTail;
use pg_ast::ExprNode;
use pg_ast::ExprNode::StringConst;
use pg_ast::FuncCall;
use pg_ast::TypeName;
use pg_ast::TypecastExpr;
use pg_basics::Str;
