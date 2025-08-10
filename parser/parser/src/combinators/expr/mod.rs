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

pub(super) fn a_expr(ctx: &mut ParserContext) -> scan::Result<ExprNode> {
    // TODO
    expr_primary(ctx)
}

pub(super) fn b_expr(ctx: &mut ParserContext) -> scan::Result<ExprNode> {
    // TODO
    expr_primary(ctx)
}

use pg_ast::ExprNode;
use pg_parser_core::scan;
use pg_parser_core::ParserContext;
