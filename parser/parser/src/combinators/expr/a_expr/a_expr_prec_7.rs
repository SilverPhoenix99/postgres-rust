pub(super) fn a_expr_prec_7(ctx: &mut ParserContext, lhs: ExprNode) -> PrecResult {

    /*
        a_expr multiplicative_op a_expr  -- %left(7)
    */

    // must Not be followed by `ALL(`/`ANY(`/`SOME(`
    if matches!(ctx.stream_mut().peek_n::<3>(), Ok([
        Operator(Mul | Div | Percent),
        Keyword(All | Any | SomeKw),
        Operator(OpenParenthesis)
    ])) {
        return Err(Ok(lhs))
    }

    let (op, rhs) = prec_wrap!(ctx, lhs,
        seq!(multiplicative_op, a_expr_prec(8))
    );

    let expr = BinaryExpr::new(op, lhs, rhs);
    Ok(expr.into())
}

use super::a_expr_prec;
use super::prec_wrap;
use super::PrecResult;
use crate::combinators::multiplicative_op;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::BinaryExpr;
use pg_ast::ExprNode;
use pg_lexer::Keyword::All;
use pg_lexer::Keyword::Any;
use pg_lexer::Keyword::SomeKw;
use pg_lexer::OperatorKind::Div;
use pg_lexer::OperatorKind::Mul;
use pg_lexer::OperatorKind::OpenParenthesis;
use pg_lexer::OperatorKind::Percent;
use pg_parser_core::stream::TokenValue::Keyword;
use pg_parser_core::stream::TokenValue::Operator;
