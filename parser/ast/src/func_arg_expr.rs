#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FuncArgExpr {
    Unnamed(ExprNode),
    NamedValue {
        name: Str,
        value: ExprNode,
    }
}

use pg_basics::Str;
use crate::ExprNode;
