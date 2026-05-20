pub(super) fn a_expr_prec_12(ctx: &mut ParserContext, lhs: ExprNode) -> PrecResult {

    /*
        a_expr AT ( LOCAL | TIME ZONE a_expr )  -- %left(12)
    */

    let (_, zone) = prec_wrap!(ctx, lhs,
        seq!(At, alt!(
            Local.map(|_| None),
            seq!(Time, Zone, a_expr_prec(13)).map(|(.., tz)| Some(tz))
        ))
    );

    let expr = TimezoneExpr::new(lhs, zone);
    Ok(expr.into())
}

use super::a_expr_prec;
use super::prec_wrap;
use super::PrecResult;
use crate::alt;
use crate::combinators::core::Combinator;
use crate::context::ParserContext;
use crate::seq;
use pg_ast::ExprNode;
use pg_ast::TimezoneExpr;
use pg_lexer::Keyword::At;
use pg_lexer::Keyword::Local;
use pg_lexer::Keyword::Time;
use pg_lexer::Keyword::Zone;
