pub(super) fn a_expr_prec_10(ctx: &mut ParserContext, lhs: ExprNode) -> PrecResult {

    /*
        a_expr COLLATE any_name  -- %left(10)
    */

    let collation = prec_wrap!(ctx, lhs, collate_clause);

    let expr = CollationExpr::new(lhs, collation);
    Ok(expr.into())
}

use super::prec_wrap;
use super::PrecResult;
use crate::combinators::collate_clause;
use crate::context::ParserContext;
use pg_ast::CollationExpr;
use pg_ast::ExprNode;
