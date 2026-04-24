#[derive(Debug, PartialEq)]
pub(super) enum FrameBound {
    UnboundedPreceding,
    UnboundedFollowing,
    CurrentRow,
    OffsetPreceding(ExprNode),
    OffsetFollowing(ExprNode),
}

pub(super) fn frame_bound(ctx: &mut ParserContext) -> scan::Result<FrameBound> {

    /*
        UNBOUNDED PRECEDING
      | UNBOUNDED FOLLOWING
      | CURRENT ROW
      | a_expr PRECEDING
      | a_expr FOLLOWING
    */

    // A single keyword is ambiguous with a_expr, so we need to check 2.
    if let Ok(toks) = ctx.stream_mut().peek_n::<2>() {

        let res = match toks {
            [Kw(Unbounded), Kw(Preceding)] => Some(UnboundedPreceding),
            [Kw(Unbounded), Kw(Following)] => Some(UnboundedFollowing),
            [Kw(Current), Kw(Row)] => Some(CurrentRow),
            _ => None
        };

        if let Some(bound) = res {
            ctx.stream_mut().skip(2);
            return Ok(bound);
        }
    }

    let (expr, bound) = seq!(
        a_expr,
        alt!(Preceding, Following)
    ).parse(ctx)?;

    let bound = if bound == Preceding {
        OffsetPreceding(expr)
    }
    else {
        OffsetFollowing(expr)
    };

    Ok(bound)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parser;
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_matrix;

    #[test_matrix("unbounded preceding" => Ok(UnboundedPreceding))]
    #[test_matrix("unbounded following" => Ok(UnboundedFollowing))]
    #[test_matrix("current row" => Ok(CurrentRow))]
    #[test_matrix("1 preceding" => Ok(OffsetPreceding(IntegerConst(1))))]
    #[test_matrix("1 following" => Ok(OffsetFollowing(IntegerConst(1))))]
    fn test_frame_bound(source: &str) -> scan::Result<FrameBound> {
        test_parser!(source, frame_bound)
    }
}

use self::FrameBound::CurrentRow;
use self::FrameBound::OffsetFollowing;
use self::FrameBound::OffsetPreceding;
use self::FrameBound::UnboundedFollowing;
use self::FrameBound::UnboundedPreceding;
use crate::alt;
use crate::combinators::core::Combinator;
use crate::combinators::expr::a_expr;
use crate::seq;
use crate::ParserContext;
use pg_ast::ExprNode;
use pg_lexer::Keyword::Current;
use pg_lexer::Keyword::Following;
use pg_lexer::Keyword::Preceding;
use pg_lexer::Keyword::Row;
use pg_lexer::Keyword::Unbounded;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenValue::Keyword as Kw;
