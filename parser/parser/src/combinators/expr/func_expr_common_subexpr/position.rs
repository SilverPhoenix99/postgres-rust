/// Inlined: `position_list`
pub(super) fn position(stream: &mut TokenStream) -> scan::Result<PositionFunc> {

    /*
        POSITION '(' b_expr IN b_expr ')'

        A "plain syntax" option is deliberately not offered
        for position(), because the reversal of the arguments
        creates too much risk of confusion.
    */

    // ❗ Don't call directly. Prefix is checked by `func_expr_common_subexpr`.

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
    #[allow(unused_imports)]
    use pg_ast::ExprNode::StringConst;
    use pg_combinators::test_parser;
    use test_case::test_case;

    #[test_case("position('f' in 'foo')" => Ok(
        PositionFunc::new(
            StringConst("f".into()),
            StringConst("foo".into())
        )
    ))]
    fn test_position(source: &str) -> scan::Result<PositionFunc> {
        test_parser!(source, position)
    }
}

use crate::combinators::expr::b_expr;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::skip;
use pg_ast::PositionFunc;
use pg_combinators::seq;
use pg_combinators::Combinator;
use pg_lexer::Keyword::In;
use pg_parser_core::scan;
use pg_parser_core::stream::TokenStream;
