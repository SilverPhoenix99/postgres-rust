/// Inlined: `position_list`
pub(super) fn position(stream: &mut TokenStream) -> scan::Result<PositionFunc> {

    /*
        POSITION '(' b_expr IN b_expr ')'

        A "plain syntax" option is deliberately not offered
        for position(), because the reversal of the arguments
        creates too much risk of confusion.
    */

    if ! matches!(stream.peek2(), Ok((K(Position), Op(OpenParenthesis)))) {
        return no_match(stream)
    }

    let (_, (needle, _, haystack)) = seq!(
        skip(1),
        paren!(seq!(b_expr, In, b_expr))
    ).parse(stream)?;

    let expr = PositionFunc::new(needle, haystack);
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_parser;
    use test_case::test_case;
    #[allow(unused_imports)]
    use {
        pg_ast::ExprNode::StringConst,
        scan::Error::NoMatch,
    };

    #[test_case("position('f' in 'foo')" => Ok(
        PositionFunc::new(
            StringConst("f".into()),
            StringConst("foo".into())
        )
    ))]
    #[test_case("position" => matches Err(NoMatch(_)))]
    #[test_case("position 1" => matches Err(NoMatch(_)))]
    fn test_position(source: &str) -> scan::Result<PositionFunc> {
        test_parser!(source, position)
    }
}

use crate::combinators::expr::b_expr;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::seq;
use crate::combinators::foundation::skip;
use crate::combinators::foundation::Combinator;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword as K;
use crate::stream::TokenValue::Operator as Op;
use pg_ast::PositionFunc;
use pg_lexer::Keyword::In;
use pg_lexer::Keyword::Position;
use pg_lexer::OperatorKind::OpenParenthesis;
