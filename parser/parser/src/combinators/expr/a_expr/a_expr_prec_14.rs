pub(super) fn a_expr_prec_14(ctx: &mut ParserContext, lhs: ExprNode) -> PrecResult {

    /*
        a_expr TYPECAST Typename  -- %left(14)
    */

    let (_, rhs) = super::prec_wrap!(ctx, lhs,
        seq!(Typecast, typename)
    );

    let expr = TypecastExpr::new(lhs, rhs);
    Ok(expr.into())
}

use super::PrecResult;
use crate::combinators::typename;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::ExprNode;
use pg_ast::TypecastExpr;
use pg_lexer::OperatorKind::Typecast;
