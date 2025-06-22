#[derive(Debug, PartialEq)]
pub(super) enum FrameBound {
    UnboundedPreceding,
    UnboundedFollowing,
    CurrentRow,
    OffsetPreceding(ExprNode),
    OffsetFollowing(ExprNode),
}

pub(super) fn frame_bound() -> impl Combinator<Output = FrameBound> {

    /*
        UNBOUNDED PRECEDING
      | UNBOUNDED FOLLOWING
      | CURRENT ROW
      | a_expr PRECEDING
      | a_expr FOLLOWING
    */

    match_first! {
        parser(|stream| {
            // A single keyword is ambiguous with a_expr, so we need to check 2.
            let res = match stream.peek2_option() {
                Some((Kw(Unbounded), Kw(Preceding))) => UnboundedPreceding,
                Some((Kw(Unbounded), Kw(Following))) => UnboundedFollowing,
                Some((Kw(Current), Kw(Row))) => CurrentRow,
                _ => return Err(NoMatch(stream.current_location()))
            };

            stream.next();
            stream.next();
            Ok(res)
        }),
        a_expr().chain(match_first_with_state!(|expr, stream| {
            Preceding => (_) OffsetPreceding(expr),
            Following => (_) OffsetFollowing(expr),
        }))
    }
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
        test_parser!(source, frame_bound(), expected);
    }
}

use self::FrameBound::CurrentRow;
use self::FrameBound::OffsetFollowing;
use self::FrameBound::OffsetPreceding;
use self::FrameBound::UnboundedFollowing;
use self::FrameBound::UnboundedPreceding;
use crate::combinators::expr::a_expr;
use crate::combinators::foundation::match_first;
use crate::combinators::foundation::match_first_with_state;
use crate::combinators::foundation::parser;
use crate::combinators::foundation::Combinator;
use crate::scan::Error::NoMatch;
use crate::stream::TokenValue::Keyword as Kw;
use pg_ast::ExprNode;
use pg_lexer::Keyword::Current;
use pg_lexer::Keyword::Following;
use pg_lexer::Keyword::Preceding;
use pg_lexer::Keyword::Row;
use pg_lexer::Keyword::Unbounded;
