pub(super) fn a_expr_prec_8(ctx: &mut ParserContext, lhs: ExprNode) -> PrecResult {

    /*
        a_expr additive_op a_expr  -- %left(8)
    */

    // must Not be followed by `ALL(`/`ANY(`/`SOME(`
    if matches!(ctx.stream_mut().peek_n::<3>(), Ok([
        Operator(Minus | Plus),
        Keyword(All | Any | SomeKw),
        Operator(OpenParenthesis)
    ])) {
        return Err(Ok(lhs))
    }

    let (op, rhs) = prec_wrap!(ctx, lhs,
        seq!(additive_op, a_expr_prec(9))
    );

    let expr = BinaryExpr::new(op, lhs, rhs);
    Ok(expr.into())
}

use super::a_expr_prec;
use super::prec_wrap;
use super::PrecResult;
use crate::combinators::additive_op;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::BinaryExpr;
use pg_ast::ExprNode;
use pg_lexer::Keyword::All;
use pg_lexer::Keyword::Any;
use pg_lexer::Keyword::SomeKw;
use pg_lexer::OperatorKind::Minus;
use pg_lexer::OperatorKind::OpenParenthesis;
use pg_lexer::OperatorKind::Plus;
use pg_parser_core::stream::TokenValue::Keyword;
use pg_parser_core::stream::TokenValue::Operator;
