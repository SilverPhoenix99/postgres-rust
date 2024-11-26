mod associativity;
mod case_expr;
mod expr_const;
mod expr_primary;
mod indirection;

pub(super) use indirection::indirection;

pub(in crate::parser) fn a_expr() -> impl Combinator<Output = ExprNode> {
    // TODO
    expr_primary()
}

pub(in crate::parser) fn b_expr() -> impl Combinator<Output = ExprNode> {
    // TODO
    expr_primary()
}

use crate::parser::ast_node::ExprNode;
use crate::parser::combinators::Combinator;
use expr_primary::expr_primary;
