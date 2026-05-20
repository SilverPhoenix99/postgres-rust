pub(super) fn a_expr_prec_4(ctx: &mut ParserContext, lhs: ExprNode) -> PrecResult {

    /*
        a_expr boolean_op a_expr  -- %nonassoc(4)
    */

    if
        ! matches!(ctx.stream_mut().peek(), Ok(
            Operator(Less | Equals | Greater | LessEquals | GreaterEquals | NotEquals)
        ))
        || matches!(ctx.stream_mut().peek_n::<3>(), Ok(
            [
                _,
                Keyword(All | Any | SomeKw),
                Operator(OpenParenthesis)
            ]
        ))
    {
        return Err(Ok(lhs))
    }

    let (op, rhs) = prec_wrap!(ctx, lhs, seq!(boolean_op, a_expr_prec(5)));

    let expr = BinaryExpr::new(op, lhs, rhs);
    Ok(expr.into())
}

use super::a_expr_prec;
use super::prec_wrap;
use super::PrecResult;
use crate::combinators::boolean_op;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::BinaryExpr;
use pg_ast::ExprNode;
use pg_lexer::Keyword::All;
use pg_lexer::Keyword::Any;
use pg_lexer::Keyword::SomeKw;
use pg_lexer::OperatorKind::Equals;
use pg_lexer::OperatorKind::Greater;
use pg_lexer::OperatorKind::GreaterEquals;
use pg_lexer::OperatorKind::Less;
use pg_lexer::OperatorKind::LessEquals;
use pg_lexer::OperatorKind::NotEquals;
use pg_lexer::OperatorKind::OpenParenthesis;
use pg_parser_core::stream::TokenValue::Keyword;
use pg_parser_core::stream::TokenValue::Operator;
