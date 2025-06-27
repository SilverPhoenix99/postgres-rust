#[derive(Debug, PartialEq)]
pub(super) enum FrameBound {
    UnboundedPreceding,
    UnboundedFollowing,
    CurrentRow,
    OffsetPreceding(ExprNode),
    OffsetFollowing(ExprNode),
}

pub(super) fn frame_bound(stream: &mut TokenStream<'_>) -> scan::Result<FrameBound> {

    /*
        UNBOUNDED PRECEDING
      | UNBOUNDED FOLLOWING
      | CURRENT ROW
      | a_expr PRECEDING
      | a_expr FOLLOWING
    */

    // A single keyword is ambiguous with a_expr, so we need to check 2.
    if let Some((first, second)) = stream.peek2_option() {

        let res = match (first, second) {
            (Kw(Unbounded), Kw(Preceding)) => Some(UnboundedPreceding),
            (Kw(Unbounded), Kw(Following)) => Some(UnboundedFollowing),
            (Kw(Current), Kw(Row)) => Some(CurrentRow),
            _ => None
        };

        if let Some(bound) = res {
            stream.next();
            stream.next();
            return Ok(bound);
        }
    }

    let (expr, bound) = seq!(=>
        a_expr().parse(stream),
        choice!(parsed stream => Preceding, Following)
    )?;

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
    use crate::tests::test_parser;
    #[allow(unused_imports)]
    use pg_ast::ExprNode::IntegerConst;
    use test_case::test_case;

    #[test_case("unbounded preceding", UnboundedPreceding)]
    #[test_case("unbounded following", UnboundedFollowing)]
    #[test_case("current row", CurrentRow)]
    #[test_case("1 preceding", OffsetPreceding(IntegerConst(1)))]
    #[test_case("1 following", OffsetFollowing(IntegerConst(1)))]
    fn test_frame_bound(source: &str, expected: FrameBound) {
        test_parser!(source, frame_bound, expected);
    }
}

use self::FrameBound::CurrentRow;
use self::FrameBound::OffsetFollowing;
use self::FrameBound::OffsetPreceding;
use self::FrameBound::UnboundedFollowing;
use self::FrameBound::UnboundedPreceding;
use crate::combinators::expr::a_expr;
use crate::combinators::foundation::choice;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::Combinator;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword as Kw;
use pg_ast::ExprNode;
use pg_lexer::Keyword::Current;
use pg_lexer::Keyword::Following;
use pg_lexer::Keyword::Preceding;
use pg_lexer::Keyword::Row;
use pg_lexer::Keyword::Unbounded;
