pub(super) fn a_expr_prec_0(ctx: &mut ParserContext, mut lhs: ExprNode) -> PrecResult {

    /*
        a_expr OR a_expr  -- %left(0)
    */

    let (_, rhs) = prec_wrap!(ctx, lhs,
        seq!(Or, a_expr_prec(1))
    );

    if let ExprNode::BoolExpr(BoolExpr::Or(args)) = &mut lhs {
        // Flatten "a OR b OR c ..." to a single BoolExpr on sight
        args.push(rhs);
        return Ok(lhs)
    }

    let expr = BoolExpr::Or(vec![lhs, rhs]);
    Ok(expr.into())
}

use super::a_expr_prec;
use super::prec_wrap;
use super::PrecResult;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::BoolExpr;
use pg_ast::ExprNode;
use pg_lexer::Keyword::Or;
