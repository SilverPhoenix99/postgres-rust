mod associativity;
mod expr_const;
mod expr_primary;
mod indirection;
mod sort_clause;

pub(super) fn a_expr() -> impl Combinator<Output = ExprNode> {
    // TODO
    expr_primary()
}

pub(super) fn b_expr() -> impl Combinator<Output = ExprNode> {
    // TODO
    expr_primary()
}

use self::{
    expr_const::expr_const,
    expr_primary::expr_primary,
    indirection::{check_indirection, indirection},
};
use crate::parser::ast_node::ExprNode;
use crate::parser::combinators::foundation::Combinator;
