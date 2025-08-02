pub(super) fn coalesce_expr(stream: &mut TokenStream) -> scan::Result<ExprNode> {

    /*
        COALESCE '(' expr_list ')'
    */

    if ! matches!(stream.peek2(), Ok((K(Kw::Coalesce), Op(OpenParenthesis)))) {
        return no_match(stream)
    };

    let args = skip_prefix(1, paren(expr_list))
        .parse(stream)?;

    Ok(Coalesce(args))
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

    #[test_case("coalesce('foo', 'bar')" => Ok(
        Coalesce(vec![
            StringConst("foo".into()),
            StringConst("bar".into())
        ])
    ))]
    #[test_case("coalesce" => matches Err(NoMatch(_)))]
    #[test_case("coalesce 1" => matches Err(NoMatch(_)))]
    fn test_coalesce_expr(source: &str) -> scan::Result<ExprNode> {
        test_parser!(source, coalesce_expr)
    }
}

use crate::combinators::expr_list::expr_list;
use crate::combinators::foundation::paren;
use crate::combinators::foundation::skip_prefix;
use crate::combinators::foundation::Combinator;
use crate::no_match;
use crate::scan;
use crate::stream::TokenStream;
use crate::stream::TokenValue::Keyword as K;
use crate::stream::TokenValue::Operator as Op;
use pg_ast::ExprNode;
use pg_ast::ExprNode::Coalesce;
use pg_lexer::Keyword as Kw;
use pg_lexer::OperatorKind::OpenParenthesis;
