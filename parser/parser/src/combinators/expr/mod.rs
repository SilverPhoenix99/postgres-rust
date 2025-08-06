mod associativity;

pg_basics::reexport! { pub(super)
    expr_primary,
    func_expr_common_subexpr,
}

pg_basics::reexport! {
    expr_const,
    indirection,
    unicode_normal_form,
}

pub(super) fn a_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {
    // TODO
    expr_primary(stream)
}

pub(super) fn b_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {
    // TODO
    expr_primary(stream)
}

use crate::stream::TokenStream;
use pg_ast::ExprNode;
use pg_parser_core::scan;
