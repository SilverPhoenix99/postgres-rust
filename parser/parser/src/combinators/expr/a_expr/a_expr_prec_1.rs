pub(super) fn a_expr_prec_1(ctx: &mut ParserContext, mut lhs: ExprNode) -> PrecResult {

    /*
        a_expr AND a_expr  -- %left(1)
    */

    let (_, rhs) = prec_wrap!(ctx, lhs,
        seq!(And, a_expr_prec(2))
    );

    if let ExprNode::BoolExpr(BoolExpr::And(args)) = &mut lhs {
        // Flatten "a OR b OR c ..." to a single BoolExpr on sight
        args.push(rhs);
        return Ok(lhs)
    }

    let expr = BoolExpr::And(vec![lhs, rhs]);
    Ok(expr.into())
}

use super::a_expr_prec;
use super::prec_wrap;
use super::PrecResult;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::BoolExpr;
use pg_ast::ExprNode;
use pg_lexer::Keyword::And;
