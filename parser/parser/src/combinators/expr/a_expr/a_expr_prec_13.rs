pub(super) fn a_expr_prec_13(ctx: &mut ParserContext, lhs: ExprNode) -> PrecResult {

    /*
          a_expr IN '(' expr_list ')'      -- %left(13)
        | a_expr NOT IN '(' expr_list ')'  -- %left(13)
    */

    if
        let Ok(toks) = ctx.stream_mut().peek_n::<5>()
        && matches!(toks, [
            Keyword(Kw::Not),
            Keyword(In),
            Operator(OpenParenthesis),
            ..
        ])
        // must Not be select_stmt
        && ! matches!(toks,
              [.., Keyword(With | Select | Table), _]
            | [.., Keyword(Values), Operator(OpenParenthesis)]
        )
    {
        let (_, expr_list) = prec_wrap!(ctx, lhs,
            seq!(skip(2), paren!(expr_list))
        );

        let expr = InArray(lhs.into(), expr_list);
        let expr = Not(expr.into());
        return Ok(expr.into())
    }

    if
        let Ok(toks) = ctx.stream_mut().peek_n::<4>()
        && matches!(toks, [
            Keyword(In),
            Operator(OpenParenthesis),
            ..
        ])
        // must Not be select_stmt
        && ! matches!(toks,
              [.., Keyword(With | Select | Table), _]
            | [.., Keyword(Values), Operator(OpenParenthesis)]
        )
    {
        let (_, expr_list) = prec_wrap!(ctx, lhs,
            seq!(skip(1), paren!(expr_list))
        );

        let expr = InArray(lhs.into(), expr_list);
        return Ok(expr)
    }

    Err(Ok(lhs))
}

use super::prec_wrap;
use super::PrecResult;
use crate::combinators::core::skip;
use crate::combinators::expr_list;
use crate::context::ParserContext;
use crate::paren;
use crate::seq;
use pg_ast::BoolExpr::Not;
use pg_ast::ExprNode;
use pg_ast::ExprNode::InArray;
use pg_lexer::Keyword as Kw;
use pg_lexer::Keyword::In;
use pg_lexer::Keyword::Select;
use pg_lexer::Keyword::Table;
use pg_lexer::Keyword::Values;
use pg_lexer::Keyword::With;
use pg_lexer::OperatorKind::OpenParenthesis;
use pg_parser_core::stream::TokenValue::Keyword;
use pg_parser_core::stream::TokenValue::Operator;
