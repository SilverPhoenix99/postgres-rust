mod associativity;
mod expr_const;
mod expr_primary;
mod indirection;
mod unicode_normal_form;

pub(super) fn a_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {
    // TODO
    expr_primary(stream)
}

pub(super) fn b_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {
    // TODO
    expr_primary(stream)
}

use self::{
    expr_const::*,
    expr_primary::*,
    indirection::*,
    unicode_normal_form::*,
};
use crate::scan;
use crate::stream::TokenStream;
use pg_ast::ExprNode;
